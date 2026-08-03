// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content federation handlers — remote repo sync, push, and blob replication.
//!
//! These methods form the Neural API surface that replaces `cascade-pull.sh`
//! and enables the cascade / provenance signal graphs to operate against
//! live `NestGate` instances.
//!
//! ## Methods
//!
//! | Method               | Signal Graph         | Purpose                              |
//! |----------------------|----------------------|--------------------------------------|
//! | `content.fetch_heads`| `ecosystem.check`    | Read-only drift detection            |
//! | `content.push`       | `ecosystem.push`     | Push to Forgejo (periplasm)          |
//! | `content.replicate`  | `rootpulse.federate` | Cross-gate content blob transfer     |
//! | `content.sync`       | `ecosystem.pull`     | Cascade-pull from remote sources     |
//!
//! ## External tools
//!
//! Repo operations delegate to the system `git` binary via `tokio::process::Command`.
//! `NestGate` does **not** link a C git library — the ecosystem standard mandates
//! a pure Rust toolchain with no C build dependencies. Git is a runtime peer tool,
//! similar to how the installer uses system `curl`.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_types::error::{ErrorContextExt, NestGateError, Result};
use serde_json::{Value, json};
use std::path::Path;
use tokio::io::AsyncReadExt;
use tracing::debug;

use super::storage_paths::ensure_parent_dirs;

use super::StorageState;
use super::federation_ops;
use super::storage_paths::{content_hash_hex, content_key_path, resolve_family_id};

/// Blobs above this threshold use chunked streaming for federation transfer.
/// Below this, the inline base64 JSON-RPC path is used (simpler, single-call).
const FEDERATION_STREAM_THRESHOLD: u64 = 16 * 1024 * 1024;

/// Chunk size for federation streaming (4 MiB, matching `MAX_STREAM_CHUNK`).
const FEDERATION_CHUNK_SIZE: u64 = 4 * 1024 * 1024;

/// `content.fetch_heads` — read-only freshness check against remote repos.
///
/// For each repo in `repos`, runs `git ls-remote` to fetch remote HEAD refs
/// and compares against the local HEAD. Returns drift status without pulling.
///
/// Serves `ecosystem.check` signal graph.
pub async fn content_fetch_heads(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let repos = params["repos"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "repos",
            "repos array required: [{path, remote?, branch?}]",
        )
    })?;

    let mut heads = Vec::with_capacity(repos.len());
    let mut error_count: u64 = 0;

    for repo in repos {
        let path = repo["path"].as_str().unwrap_or("");
        let remote = repo["remote"].as_str().unwrap_or("origin");
        let branch = repo["branch"].as_str().unwrap_or("main");

        if path.is_empty() {
            heads.push(json!({
                "path": path,
                "error": "path required"
            }));
            error_count += 1;
            continue;
        }

        let result = federation_ops::fetch_head_refs(path, remote, branch).await;
        match result {
            Ok(head_info) => heads.push(head_info),
            Err(e) => {
                heads.push(json!({
                    "path": path,
                    "remote": remote,
                    "branch": branch,
                    "error": e.to_string()
                }));
                error_count += 1;
            }
        }
    }

    let checked_count = heads.len() as u64;
    Ok(json!({
        "heads": heads,
        "checked_count": checked_count,
        "error_count": error_count,
        "checked_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// `content.push` — push local content to a remote (Forgejo periplasm).
///
/// For each repo in `repos`, runs `git push` to the specified remote.
///
/// Serves `ecosystem.push` signal graph.
pub async fn content_push(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let repos = params["repos"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "repos",
            "repos array required: [{path, remote?, branch?}]",
        )
    })?;

    let mut results = Vec::with_capacity(repos.len());
    let mut pushed_count: u64 = 0;

    for repo in repos {
        let path = repo["path"].as_str().unwrap_or("");
        let default_remote =
            std::env::var("NESTGATE_PREFERRED_REMOTE").unwrap_or_else(|_| "origin".into());
        let remote = repo["remote"]
            .as_str()
            .map_or(default_remote.as_str(), |r| r);
        let branch = repo["branch"].as_str().unwrap_or("main");

        if path.is_empty() {
            results.push(json!({
                "path": path,
                "pushed": false,
                "error": "path required"
            }));
            continue;
        }

        let result = federation_ops::push_to_remote(path, remote, branch).await;
        match result {
            Ok(info) => {
                if info["pushed"].as_bool().unwrap_or(false) {
                    pushed_count += 1;
                }
                results.push(info);
            }
            Err(e) => {
                results.push(json!({
                    "path": path,
                    "remote": remote,
                    "branch": branch,
                    "pushed": false,
                    "error": e.to_string()
                }));
            }
        }
    }

    Ok(json!({
        "results": results,
        "pushed_count": pushed_count,
        "total_count": results.len() as u64,
        "pushed_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// `content.replicate` — transfer content blobs (by CID) to a remote `NestGate`.
///
/// Accepts a list of BLAKE3 content identifiers and transfers them to the
/// target `NestGate` instance. Used by `rootpulse.federate` for cross-gate
/// content synchronization.
///
/// Transfer is diff-based: only blobs the remote lacks are sent.
pub async fn content_replicate(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let cids = params["cids"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "cids",
            "cids array required: [\"<blake3_hex>\", ...]",
        )
    })?;

    let target = params["target"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "target",
            "target required: socket path or tcp://host:port of remote nestgate",
        )
    })?;

    let family_id = resolve_family_id(params, state)?;

    let mut replicated = Vec::with_capacity(cids.len());
    let mut total_bytes: u64 = 0;
    let mut transferred_count: u64 = 0;
    let mut skipped_count: u64 = 0;

    for cid_val in cids {
        let cid = match cid_val.as_str() {
            Some(c) if c.len() == 64 => c,
            Some(c) => {
                replicated.push(json!({
                    "cid": c,
                    "transferred": false,
                    "error": "invalid CID: expected 64-char BLAKE3 hex"
                }));
                continue;
            }
            None => continue,
        };

        let blob_path = content_key_path(family_id, cid);
        if !blob_path.exists() {
            replicated.push(json!({
                "cid": cid,
                "transferred": false,
                "error": "blob not found locally"
            }));
            continue;
        }

        match replicate_blob_to_remote(&blob_path, cid, target, family_id).await {
            Ok(size) => {
                if size > 0 {
                    transferred_count += 1;
                    total_bytes += size;
                    replicated.push(json!({
                        "cid": cid,
                        "transferred": true,
                        "size": size
                    }));
                } else {
                    skipped_count += 1;
                    replicated.push(json!({
                        "cid": cid,
                        "transferred": false,
                        "skipped": true,
                        "reason": "already exists on remote"
                    }));
                }
            }
            Err(e) => {
                replicated.push(json!({
                    "cid": cid,
                    "transferred": false,
                    "error": e.to_string()
                }));
            }
        }
    }

    Ok(json!({
        "replicated": replicated,
        "transferred_count": transferred_count,
        "skipped_count": skipped_count,
        "total_bytes": total_bytes,
        "target": target,
        "family_id": family_id,
        "replicated_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// `content.sync` — cascade-pull from remote sources.
///
/// Neural API equivalent of `cascade-pull.sh`. For each repo, resolves the
/// remote (forgejo-first, origin fallback) and pulls with `--ff-only`.
///
/// Serves `ecosystem.pull` signal graph.
pub async fn content_sync(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let repos = params["repos"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "repos",
            "repos array required: [{path, remote?, branch?}]",
        )
    })?;

    let parallel = params["parallel"].as_u64().unwrap_or(4).min(16);
    let clone_missing = params["clone_missing"].as_bool().unwrap_or(false);
    let source = params["source"].as_str().unwrap_or("auto");

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(parallel as usize));
    let mut handles = Vec::with_capacity(repos.len());

    for repo in repos {
        let path = repo["path"].as_str().unwrap_or("").to_owned();
        let remote = repo["remote"]
            .as_str()
            .map_or_else(|| source.to_owned(), String::from);
        let branch = repo["branch"].as_str().unwrap_or("main").to_owned();
        let clone_url = repo["clone_url"].as_str().map(String::from);
        let do_clone = clone_missing;
        let sem = semaphore.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await;
            federation_ops::sync_repo(&path, &remote, &branch, clone_url.as_deref(), do_clone).await
        }));
    }

    let mut results = Vec::with_capacity(handles.len());
    let mut synced_count: u64 = 0;

    for handle in handles {
        match handle.await {
            Ok(Ok(info)) => {
                if info["synced"].as_bool().unwrap_or(false) {
                    synced_count += 1;
                }
                results.push(info);
            }
            Ok(Err(e)) => {
                results.push(json!({
                    "synced": false,
                    "error": e.to_string()
                }));
            }
            Err(e) => {
                results.push(json!({
                    "synced": false,
                    "error": format!("task panic: {e}")
                }));
            }
        }
    }

    Ok(json!({
        "results": results,
        "synced_count": synced_count,
        "total_count": results.len() as u64,
        "parallel": parallel,
        "source": source,
        "synced_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// `content.replicate.pull` — pull CIDs from a remote `NestGate` to local storage.
///
/// The inverse of `content.replicate` (which pushes). Used for cold-from-hot
/// federation: a cold-storage gate schedules pulls from the hot gate.
///
/// Diff-based: only fetches blobs the local store lacks.
pub async fn content_replicate_pull(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let cids = params["cids"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "cids",
            "cids array required: [\"<blake3_hex>\", ...]",
        )
    })?;

    let source = params["source"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "source",
            "source required: socket path or tcp://host:port of remote nestgate",
        )
    })?;

    let family_id = resolve_family_id(params, state)?;

    let mut pulled = Vec::with_capacity(cids.len());
    let mut total_bytes: u64 = 0;
    let mut transferred_count: u64 = 0;
    let mut skipped_count: u64 = 0;

    for cid_val in cids {
        let cid = match cid_val.as_str() {
            Some(c) if c.len() == 64 => c,
            Some(c) => {
                pulled.push(json!({
                    "cid": c,
                    "pulled": false,
                    "error": "invalid CID: expected 64-char BLAKE3 hex"
                }));
                continue;
            }
            None => continue,
        };

        let local_path = content_key_path(family_id, cid);
        if local_path.exists() {
            skipped_count += 1;
            pulled.push(json!({
                "cid": cid,
                "pulled": false,
                "skipped": true,
                "reason": "already exists locally"
            }));
            continue;
        }

        match pull_blob_from_remote(cid, source, family_id, &local_path).await {
            Ok(size) => {
                transferred_count += 1;
                total_bytes += size;
                pulled.push(json!({
                    "cid": cid,
                    "pulled": true,
                    "size": size
                }));
            }
            Err(e) => {
                pulled.push(json!({
                    "cid": cid,
                    "pulled": false,
                    "error": e.to_string()
                }));
            }
        }
    }

    Ok(json!({
        "pulled": pulled,
        "transferred_count": transferred_count,
        "skipped_count": skipped_count,
        "total_bytes": total_bytes,
        "source": source,
        "family_id": family_id,
        "pulled_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// Fetch a blob from a remote `NestGate`, verify BLAKE3 integrity, and write
/// to local CAS path.
///
/// Probes the remote with `content.exists` first to get the size. For blobs
/// <= 16 MiB, uses inline `content.get`. For larger blobs, uses chunked
/// `content.retrieve_stream` / `content.retrieve_stream_chunk`.
///
/// The BLAKE3 hash of the received bytes **must** match the requested CID.
/// Content is self-certifying: the hash IS the authority, regardless of which
/// gate served it.
async fn pull_blob_from_remote(
    cid: &str,
    source: &str,
    family_id: &str,
    local_path: &Path,
) -> Result<u64> {
    let mut client = federation_ops::connect_federation(source).await?;

    let exists_result = client
        .call(
            "content.exists",
            json!({"hash": cid, "family_id": family_id}),
        )
        .await
        .map_err(|e| NestGateError::internal(format!("remote content.exists failed: {e}")))?;

    if exists_result["exists"].as_bool() != Some(true) {
        return Err(NestGateError::not_found(format!(
            "CID {cid} not found on remote {source}"
        )));
    }

    let remote_size = exists_result["size"].as_u64().unwrap_or(0);

    ensure_parent_dirs(local_path).await?;

    let size = if remote_size > FEDERATION_STREAM_THRESHOLD {
        pull_blob_streamed(&mut client, cid, source, family_id, local_path, remote_size).await?
    } else {
        pull_blob_inline(&mut client, cid, source, family_id, local_path).await?
    };

    Ok(size)
}

/// Small-blob inline pull via `content.get`.
async fn pull_blob_inline(
    client: &mut crate::rpc::JsonRpcClient,
    cid: &str,
    source: &str,
    family_id: &str,
    local_path: &Path,
) -> Result<u64> {
    let result = client
        .call("content.get", json!({"hash": cid, "family_id": family_id}))
        .await
        .map_err(|e| NestGateError::internal(format!("remote content.get failed: {e}")))?;

    if result["use_streaming"].as_bool() == Some(true) {
        let stream_size = result["size"].as_u64().unwrap_or(0);
        return pull_blob_streamed(client, cid, source, family_id, local_path, stream_size).await;
    }

    let data_b64 = result["data"]
        .as_str()
        .ok_or_else(|| NestGateError::internal("remote returned no data field"))?;

    let raw = STANDARD
        .decode(data_b64)
        .internal_ctx("base64 decode failed")?;

    verify_and_write(cid, source, &raw, local_path).await?;
    Ok(raw.len() as u64)
}

/// Large-blob chunked pull via `content.retrieve_stream` + `content.retrieve_stream_chunk`.
async fn pull_blob_streamed(
    client: &mut crate::rpc::JsonRpcClient,
    cid: &str,
    source: &str,
    family_id: &str,
    local_path: &Path,
    _remote_size: u64,
) -> Result<u64> {
    debug!(
        cid,
        _remote_size, "federation pull: using chunked streaming"
    );

    let begin_result = client
        .call(
            "content.retrieve_stream",
            json!({
                "hash": cid,
                "family_id": family_id,
                "chunk_size": FEDERATION_CHUNK_SIZE,
            }),
        )
        .await
        .map_err(|e| NestGateError::internal(format!("remote content.retrieve_stream: {e}")))?;

    let stream_id = begin_result["stream_id"]
        .as_str()
        .ok_or_else(|| NestGateError::internal("remote returned no stream_id"))?
        .to_owned();

    let total_size = begin_result["total_size"]
        .as_u64()
        .ok_or_else(|| NestGateError::internal("remote returned no total_size"))?;

    let tmp_path = local_path.with_extension("part");
    let mut file = tokio::fs::File::create(&tmp_path)
        .await
        .map_err(|e| NestGateError::internal(format!("create staging {cid}: {e}")))?;

    let mut offset: u64 = 0;
    loop {
        let chunk_result = client
            .call(
                "content.retrieve_stream_chunk",
                json!({
                    "stream_id": stream_id,
                    "offset": offset,
                }),
            )
            .await
            .map_err(|e| {
                NestGateError::internal(format!(
                    "remote content.retrieve_stream_chunk at offset {offset}: {e}"
                ))
            })?;

        let data_b64 = chunk_result["data"]
            .as_str()
            .ok_or_else(|| NestGateError::internal("chunk has no data field"))?;

        let decoded = STANDARD
            .decode(data_b64)
            .internal_ctx("base64 decode chunk")?;

        let n = decoded.len() as u64;

        tokio::io::AsyncWriteExt::write_all(&mut file, &decoded)
            .await
            .map_err(|e| NestGateError::internal(format!("write chunk at {offset}: {e}")))?;

        offset += n;

        if chunk_result["is_last"].as_bool() == Some(true) || offset >= total_size {
            break;
        }
    }

    tokio::io::AsyncWriteExt::flush(&mut file)
        .await
        .map_err(|e| NestGateError::internal(format!("flush staging {cid}: {e}")))?;
    drop(file);

    let raw = tokio::fs::read(&tmp_path)
        .await
        .map_err(|e| NestGateError::internal(format!("read staging for verify {cid}: {e}")))?;

    verify_and_write(cid, source, &raw, local_path).await?;

    let _ = tokio::fs::remove_file(&tmp_path).await;

    debug!(cid, offset, "federation pull: streamed transfer complete");

    Ok(offset)
}

/// Verify BLAKE3 integrity and write to the final CAS path.
async fn verify_and_write(
    cid: &str,
    source: &str,
    raw: &[u8],
    local_path: &Path,
) -> Result<()> {
    let actual_hash = content_hash_hex(raw);
    if actual_hash != cid {
        return Err(NestGateError::internal(format!(
            "BLAKE3 integrity failure: expected {cid}, got {actual_hash} \
             (remote {source} served corrupted content)"
        )));
    }

    let tmp_path = local_path.with_extension("part");
    tokio::fs::write(&tmp_path, raw)
        .await
        .map_err(|e| NestGateError::internal(format!("write blob {cid}: {e}")))?;
    tokio::fs::rename(&tmp_path, local_path)
        .await
        .map_err(|e| NestGateError::internal(format!("finalize blob {cid}: {e}")))?;

    Ok(())
}

/// Transfer a single content blob to a remote `NestGate`.
///
/// Checks remote `content.exists` first — skips transfer if already present.
/// For blobs <= 16 MiB, uses inline `content.put` (single call).
/// For larger blobs, uses chunked `content.store_stream` / `content.store_stream_chunk`
/// to avoid loading the entire payload into a single JSON-RPC message.
///
/// Returns the blob size if transferred, 0 if already present on the remote.
async fn replicate_blob_to_remote(
    blob_path: &Path,
    cid: &str,
    target: &str,
    family_id: &str,
) -> Result<u64> {
    let file_meta = tokio::fs::metadata(blob_path)
        .await
        .map_err(|e| NestGateError::internal(format!("stat blob {cid}: {e}")))?;
    let size = file_meta.len();

    let mut client = federation_ops::connect_federation(target).await?;

    let exists_result = client
        .call(
            "content.exists",
            json!({"hash": cid, "family_id": family_id}),
        )
        .await;

    if let Ok(ref val) = exists_result
        && val["exists"].as_bool() == Some(true)
    {
        return Ok(0);
    }

    let meta_path = blob_path.with_extension("meta.json");
    let metadata: Option<Value> = if meta_path.exists() {
        tokio::fs::read_to_string(&meta_path)
            .await
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
    } else {
        None
    };

    if size > FEDERATION_STREAM_THRESHOLD {
        replicate_blob_streamed(&mut client, blob_path, cid, family_id, size, metadata.as_ref())
            .await?;
    } else {
        replicate_blob_inline(&mut client, blob_path, cid, family_id, metadata.as_ref()).await?;
    }

    Ok(size)
}

/// Small-blob inline transfer via `content.put`.
async fn replicate_blob_inline(
    client: &mut crate::rpc::JsonRpcClient,
    blob_path: &Path,
    cid: &str,
    family_id: &str,
    metadata: Option<&Value>,
) -> Result<()> {
    let raw = tokio::fs::read(blob_path)
        .await
        .map_err(|e| NestGateError::internal(format!("read blob {cid}: {e}")))?;
    let encoded = STANDARD.encode(&raw);

    let mut put_params = json!({
        "data": encoded,
        "family_id": family_id
    });
    if let Some(meta) = metadata {
        if let Some(ct) = meta.get("content_type") {
            put_params["content_type"] = ct.clone();
        }
        if let Some(src) = meta.get("source") {
            put_params["source"] = src.clone();
        }
    }

    client
        .call("content.put", put_params)
        .await
        .map_err(|e| NestGateError::internal(format!("remote content.put failed: {e}")))?;

    Ok(())
}

/// Large-blob chunked transfer via `content.store_stream` + `content.store_stream_chunk`.
async fn replicate_blob_streamed(
    client: &mut crate::rpc::JsonRpcClient,
    blob_path: &Path,
    cid: &str,
    family_id: &str,
    total_size: u64,
    metadata: Option<&Value>,
) -> Result<()> {
    debug!(
        cid,
        total_size, "federation replicate: using chunked streaming"
    );

    let mut begin_params = json!({
        "total_size": total_size,
        "family_id": family_id,
    });
    if let Some(meta) = metadata
        && let Some(ct) = meta.get("content_type")
    {
        begin_params["content_type"] = ct.clone();
    }

    let begin_result = client
        .call("content.store_stream", begin_params)
        .await
        .map_err(|e| NestGateError::internal(format!("remote content.store_stream: {e}")))?;

    let stream_id = begin_result["stream_id"]
        .as_str()
        .ok_or_else(|| NestGateError::internal("remote returned no stream_id"))?
        .to_owned();

    let chunk_size = usize::try_from(
        begin_result["chunk_size"]
            .as_u64()
            .unwrap_or(FEDERATION_CHUNK_SIZE),
    )
    .unwrap_or(4 * 1024 * 1024);

    let mut file = tokio::fs::File::open(blob_path)
        .await
        .map_err(|e| NestGateError::internal(format!("open blob {cid}: {e}")))?;

    let mut offset: u64 = 0;
    let mut buf = vec![0u8; chunk_size];

    loop {
        let n = file.read(&mut buf).await.io_ctx("read chunk")?;
        if n == 0 {
            break;
        }

        let chunk_len = u64::try_from(n).unwrap_or(0);
        let is_last = offset + chunk_len >= total_size;

        let chunk_params = json!({
            "stream_id": stream_id,
            "offset": offset,
            "data": STANDARD.encode(&buf[..n]),
            "is_last": is_last,
        });

        client
            .call("content.store_stream_chunk", chunk_params)
            .await
            .map_err(|e| {
                NestGateError::internal(format!(
                    "remote content.store_stream_chunk at offset {offset}: {e}"
                ))
            })?;

        offset += chunk_len;

        if is_last {
            break;
        }
    }

    debug!(cid, offset, "federation replicate: streamed transfer complete");

    Ok(())
}

#[cfg(test)]
#[path = "content_federation_handlers_tests.rs"]
mod tests;
