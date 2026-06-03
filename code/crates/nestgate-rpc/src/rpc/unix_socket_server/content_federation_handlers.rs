// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content federation handlers — remote repo sync, push, and blob replication.
//!
//! These methods form the Neural API surface that replaces `cascade-pull.sh`
//! and enables the waterFall / rootPulse signal graphs to operate against
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
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::Path;

use super::storage_paths::ensure_parent_dirs;

use super::StorageState;
use super::federation_ops;
use super::storage_paths::{content_key_path, resolve_family_id};

/// `content.fetch_heads` — read-only freshness check against remote repos.
///
/// For each repo in `repos`, runs `git ls-remote` to fetch remote HEAD refs
/// and compares against the local HEAD. Returns drift status without pulling.
///
/// Serves `ecosystem.check` signal graph.
pub async fn content_fetch_heads(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let repos = params["repos"]
        .as_array()
        .ok_or_else(|| {
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

    let repos = params["repos"]
        .as_array()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "repos",
                "repos array required: [{path, remote?, branch?}]",
            )
        })?;

    let mut results = Vec::with_capacity(repos.len());
    let mut pushed_count: u64 = 0;

    for repo in repos {
        let path = repo["path"].as_str().unwrap_or("");
        let remote = repo["remote"].as_str().unwrap_or("forgejo");
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

    let cids = params["cids"]
        .as_array()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "cids",
                "cids array required: [\"<blake3_hex>\", ...]",
            )
        })?;

    let target = params["target"]
        .as_str()
        .ok_or_else(|| {
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

    let repos = params["repos"]
        .as_array()
        .ok_or_else(|| {
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

    let cids = params["cids"]
        .as_array()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "cids",
                "cids array required: [\"<blake3_hex>\", ...]",
            )
        })?;

    let source = params["source"]
        .as_str()
        .ok_or_else(|| {
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

/// Fetch a blob from a remote `NestGate` via `content.get`, verify BLAKE3
/// integrity, and write to local CAS path.
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
    let get_request = json!({
        "jsonrpc": "2.0",
        "method": "content.get",
        "params": {"hash": cid, "family_id": family_id},
        "id": 1
    });

    let get_response = federation_ops::send_jsonrpc(source, &get_request).await?;

    if let Some(err) = get_response.get("error") {
        return Err(NestGateError::internal(format!(
            "remote content.get failed: {err}"
        )));
    }

    let data_b64 = get_response["result"]["data"]
        .as_str()
        .ok_or_else(|| NestGateError::internal(String::from("remote returned no data field")))?;

    let raw = STANDARD
        .decode(data_b64)
        .map_err(|e| NestGateError::internal(format!("base64 decode failed: {e}")))?;

    let actual_hash = blake3::hash(&raw).to_hex().to_string();
    if actual_hash != cid {
        return Err(NestGateError::internal(format!(
            "BLAKE3 integrity failure: expected {cid}, got {actual_hash} \
             (remote {source} served corrupted content)"
        )));
    }

    let size = raw.len() as u64;

    ensure_parent_dirs(local_path).await?;
    tokio::fs::write(local_path, &raw)
        .await
        .map_err(|e| NestGateError::internal(format!("write blob {cid}: {e}")))?;

    Ok(size)
}

/// Transfer a single content blob to a remote `NestGate` via JSON-RPC `content.put`.
///
/// Checks remote `content.exists` first — skips transfer if already present.
/// Returns the blob size if transferred, 0 if already present on the remote.
async fn replicate_blob_to_remote(
    blob_path: &Path,
    cid: &str,
    target: &str,
    family_id: &str,
) -> Result<u64> {
    let raw = tokio::fs::read(blob_path)
        .await
        .map_err(|e| NestGateError::internal(format!("read blob {cid}: {e}")))?;
    let size = raw.len() as u64;
    let encoded = STANDARD.encode(&raw);

    let exists_request = json!({
        "jsonrpc": "2.0",
        "method": "content.exists",
        "params": {"hash": cid, "family_id": family_id},
        "id": 1
    });

    let exists_response = federation_ops::send_jsonrpc(target, &exists_request).await?;
    if exists_response["result"]["exists"].as_bool() == Some(true) {
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

    let put_request = json!({
        "jsonrpc": "2.0",
        "method": "content.put",
        "params": put_params,
        "id": 2
    });

    let put_response = federation_ops::send_jsonrpc(target, &put_request).await?;
    if put_response.get("error").is_some() {
        return Err(NestGateError::internal(format!(
            "remote content.put failed: {}",
            put_response["error"]
        )));
    }

    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn mock_state() -> StorageState {
        StorageState::new().unwrap_or_else(|_| {
            panic!("StorageState::new failed in test — check env")
        })
    }

    #[tokio::test]
    async fn fetch_heads_rejects_missing_repos() {
        let state = mock_state();
        let params = json!({});
        let err = content_fetch_heads(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn fetch_heads_empty_path_returns_error_entry() {
        let state = mock_state();
        let params = json!({"repos": [{"path": ""}]});
        let result = content_fetch_heads(Some(&params), &state).await.unwrap();
        assert_eq!(result["error_count"], 1);
        assert!(result["heads"][0]["error"].as_str().is_some());
    }

    #[tokio::test]
    async fn push_rejects_missing_repos() {
        let state = mock_state();
        let params = json!({});
        let err = content_push(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn push_empty_path_returns_error_entry() {
        let state = mock_state();
        let params = json!({"repos": [{"path": ""}]});
        let result = content_push(Some(&params), &state).await.unwrap();
        assert_eq!(result["pushed_count"], 0);
        assert!(result["results"][0]["error"].as_str().is_some());
    }

    #[tokio::test]
    async fn replicate_rejects_missing_cids() {
        let state = mock_state();
        let params = json!({"target": "/tmp/test.sock"});
        let err = content_replicate(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_rejects_missing_target() {
        let state = mock_state();
        let params = json!({"cids": ["abc"]});
        let err = content_replicate(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_rejects_invalid_cid_length() {
        let state = mock_state();
        let params = json!({
            "cids": ["tooshort"],
            "target": "/tmp/test.sock",
            "family_id": "test"
        });
        let result = content_replicate(Some(&params), &state).await.unwrap();
        assert!(result["replicated"][0]["error"].as_str().unwrap().contains("invalid CID"));
    }

    #[tokio::test]
    async fn sync_rejects_missing_repos() {
        let state = mock_state();
        let params = json!({});
        let err = content_sync(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn sync_empty_path_returns_error() {
        let state = mock_state();
        let params = json!({"repos": [{"path": ""}]});
        let result = content_sync(Some(&params), &state).await.unwrap();
        assert_eq!(result["synced_count"], 0);
    }

    #[tokio::test]
    async fn sync_missing_repo_without_clone() {
        let state = mock_state();
        let params = json!({"repos": [{"path": "/nonexistent/repo/path"}]});
        let result = content_sync(Some(&params), &state).await.unwrap();
        let repo_result = &result["results"][0];
        assert_eq!(repo_result["synced"], false);
        assert!(repo_result["error"].as_str().unwrap().contains("not found"));
    }

    #[tokio::test]
    async fn sync_respects_parallel_limit() {
        let state = mock_state();
        let params = json!({
            "repos": [{"path": "/nonexistent1"}, {"path": "/nonexistent2"}],
            "parallel": 1
        });
        let result = content_sync(Some(&params), &state).await.unwrap();
        assert_eq!(result["parallel"], 1);
        assert_eq!(result["total_count"], 2);
    }

    #[tokio::test]
    async fn all_handlers_reject_none_params() {
        let state = mock_state();
        assert!(content_fetch_heads(None, &state).await.is_err());
        assert!(content_push(None, &state).await.is_err());
        assert!(content_replicate(None, &state).await.is_err());
        assert!(content_sync(None, &state).await.is_err());
        assert!(content_replicate_pull(None, &state).await.is_err());
    }

    #[tokio::test]
    async fn replicate_pull_rejects_missing_cids() {
        let state = mock_state();
        let params = json!({"source": "/tmp/test.sock"});
        let err = content_replicate_pull(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_pull_rejects_missing_source() {
        let state = mock_state();
        let params = json!({"cids": ["abc"]});
        let err = content_replicate_pull(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_pull_rejects_invalid_cid_length() {
        let state = mock_state();
        let params = json!({
            "cids": ["tooshort"],
            "source": "/tmp/test.sock",
            "family_id": "test"
        });
        let result = content_replicate_pull(Some(&params), &state).await.unwrap();
        assert!(result["pulled"][0]["error"].as_str().unwrap().contains("invalid CID"));
    }

    #[tokio::test]
    async fn replicate_pull_skips_existing_local_blob() {
        let state = mock_state();
        let family_id = "test_pull_skip";
        let cid = "a".repeat(64);
        let blob_path = content_key_path(family_id, &cid);
        tokio::fs::create_dir_all(blob_path.parent().unwrap()).await.unwrap();
        tokio::fs::write(&blob_path, b"test data").await.unwrap();

        let params = json!({
            "cids": [cid],
            "source": "/tmp/nonexistent.sock",
            "family_id": family_id
        });
        let result = content_replicate_pull(Some(&params), &state).await.unwrap();
        assert_eq!(result["skipped_count"], 1);
        assert!(result["pulled"][0]["skipped"].as_bool().unwrap());

        let _ = tokio::fs::remove_dir_all(
            nestgate_config::config::storage_paths::get_storage_base_path()
                .join("datasets")
                .join(family_id),
        )
        .await;
    }
}
