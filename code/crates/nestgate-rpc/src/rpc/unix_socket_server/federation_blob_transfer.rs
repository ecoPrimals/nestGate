// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Blob transfer helpers for CAS federation (pull and replicate).
//!
//! Extracted from `content_federation_handlers` to keep the handler module
//! within the 800-line budget. These functions handle the actual byte-level
//! transfer of content blobs between `NestGate` instances, with BLAKE3
//! integrity verification and streaming support for large objects.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_types::error::{ErrorContextExt, NestGateError, Result};
use serde_json::{Value, json};
use std::path::Path;
use tokio::io::AsyncReadExt;
use tracing::debug;

use super::federation_ops;
use super::storage_paths::{content_hash_hex, ensure_parent_dirs};

/// Blobs above this threshold use chunked streaming for federation transfer.
/// Below this, the inline base64 JSON-RPC path is used (simpler, single-call).
pub(super) const FEDERATION_STREAM_THRESHOLD: u64 = 16 * 1024 * 1024;

/// Chunk size for federation streaming (4 MiB, matching `MAX_STREAM_CHUNK`).
const FEDERATION_CHUNK_SIZE: u64 = 4 * 1024 * 1024;

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
pub(super) async fn pull_blob_from_remote(
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
pub(super) async fn replicate_blob_to_remote(
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
