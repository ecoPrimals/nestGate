// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content-addressed stream (CAS layout + BLAKE3 verify).
//!
//! Reuses the session infrastructure from [`super::storage_stream`] but
//! writes to `_content/{prefix}/{hash}` instead of `_blobs/{key}`.
//! On finalize, computes BLAKE3 and renames staging → final CAS path.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::PathBuf;
use std::time::Instant;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use uuid::Uuid;

use super::storage_stream::{
    MAX_STREAM_CHUNK, MAX_STREAM_TOTAL, RetrieveSession, StoreUpload, ensure_parent, maps,
    prune_stale_streams, resolve_family_id, staging_path, ttl_expired, validate_segment,
};
use super::unix_socket_server::storage_paths::{content_cas_path, content_hash_hex};

/// `content.store_stream` — begin a chunked CAS upload.
///
/// Unlike `storage.store_stream`, the caller does not supply a key.
/// The content hash is computed on finalize and becomes the CAS key.
pub async fn content_store_stream_begin(
    params: Value,
    family_fallback: Option<&str>,
) -> Result<Value> {
    let family_id = resolve_family_id(&params, family_fallback)?;
    let total_size = params
        .get("total_size")
        .and_then(Value::as_u64)
        .ok_or_else(|| NestGateError::invalid_input_with_field("total_size", "u64 required"))?;

    if total_size > MAX_STREAM_TOTAL {
        return Err(NestGateError::invalid_input_with_field(
            "total_size",
            format!("exceeds maximum ({MAX_STREAM_TOTAL} bytes)"),
        ));
    }

    validate_segment(family_id, "family_id")?;

    let content_type = params
        .get("content_type")
        .and_then(|v| v.as_str())
        .map(str::to_owned);

    let stream_id = Uuid::new_v4().to_string();
    let temp_path = staging_path(family_id, &stream_id);

    ensure_parent(&temp_path).await?;

    if total_size == 0 {
        let empty_hash = content_hash_hex(&[]);
        let final_path = content_cas_path(family_id, &empty_hash);
        ensure_parent(&final_path).await?;
        if !final_path.exists() {
            tokio::fs::write(&final_path, [])
                .await
                .map_err(|e| NestGateError::io_error(format!("write empty content: {e}")))?;
        }
        return Ok(json!({
            "stream_id": stream_id,
            "hash": empty_hash,
            "size": 0,
            "stored": true,
            "deduplicated": final_path.exists(),
            "family_id": family_id,
            "content_type": content_type,
            "chunk_size": MAX_STREAM_CHUNK,
        }));
    }

    tokio::fs::File::create(&temp_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("create content staging: {e}")))?;

    let upload = StoreUpload {
        temp_path,
        final_path: PathBuf::new(),
        total_size,
        bytes_written: 0,
        created: Instant::now(),
        family_id: family_id.to_string(),
        dataset: "_content_stream".into(),
        key: stream_id.clone(),
        content_type,
    };

    let mut guard = maps().lock().await;
    prune_stale_streams(&mut guard);
    guard.uploads.insert(stream_id.clone(), upload);
    drop(guard);

    Ok(json!({
        "stream_id": stream_id,
        "chunk_size": MAX_STREAM_CHUNK,
        "family_id": family_id,
        "total_size": total_size,
    }))
}

/// `content.store_stream_chunk` — append data and optionally finalize.
///
/// On `is_last: true`, computes BLAKE3 of the full staging file and
/// renames to the CAS path. Returns the content hash.
pub async fn content_store_stream_chunk(params: Value) -> Result<Value> {
    let stream_id = params
        .get("stream_id")
        .and_then(Value::as_str)
        .ok_or_else(|| NestGateError::invalid_input_with_field("stream_id", "string required"))?;
    let offset = params
        .get("offset")
        .and_then(Value::as_u64)
        .ok_or_else(|| NestGateError::invalid_input_with_field("offset", "u64 required"))?;
    let data_b64 = params
        .get("data")
        .and_then(Value::as_str)
        .ok_or_else(|| NestGateError::invalid_input_with_field("data", "base64 string required"))?;
    let is_last = params
        .get("is_last")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let decoded = STANDARD
        .decode(data_b64)
        .map_err(|e| NestGateError::invalid_input_with_field("data", format!("bad base64: {e}")))?;

    let decoded_len = u64::try_from(decoded.len()).unwrap_or(u64::MAX);
    if decoded_len > MAX_STREAM_CHUNK {
        return Err(NestGateError::invalid_input_with_field(
            "data",
            format!("chunk exceeds {MAX_STREAM_CHUNK} bytes decoded"),
        ));
    }

    let mut guard = maps().lock().await;
    prune_stale_streams(&mut guard);

    let Some(upload) = guard.uploads.get_mut(stream_id) else {
        return Err(NestGateError::invalid_input_with_field(
            "stream_id",
            "unknown or expired stream_id",
        ));
    };

    if ttl_expired(upload.created) {
        guard.uploads.remove(stream_id);
        return Err(NestGateError::invalid_input_with_field(
            "stream_id",
            "stream expired",
        ));
    }

    if offset != upload.bytes_written {
        return Err(NestGateError::invalid_input_with_field(
            "offset",
            format!("expected {}, got {offset}", upload.bytes_written),
        ));
    }

    let new_written = upload.bytes_written + decoded_len;
    if new_written > upload.total_size {
        return Err(NestGateError::invalid_input_with_field(
            "data",
            "chunk would exceed declared total_size",
        ));
    }

    let temp_path = upload.temp_path.clone();
    let family_id = upload.family_id.clone();

    let mut file = OpenOptions::new()
        .write(true)
        .open(&temp_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("open content staging: {e}")))?;
    file.seek(std::io::SeekFrom::Start(offset))
        .await
        .map_err(|e| NestGateError::io_error(format!("seek: {e}")))?;
    file.write_all(&decoded)
        .await
        .map_err(|e| NestGateError::io_error(format!("write chunk: {e}")))?;
    file.flush()
        .await
        .map_err(|e| NestGateError::io_error(format!("flush: {e}")))?;
    drop(file);

    upload.bytes_written = new_written;

    if !is_last {
        return Ok(json!({
            "stream_id": stream_id,
            "offset": offset,
            "bytes_written": decoded_len,
            "is_last": false,
        }));
    }

    let Some(upload) = guard.uploads.remove(stream_id) else {
        return Err(NestGateError::invalid_input_with_field(
            "stream_id",
            "session lost during finalize",
        ));
    };
    drop(guard);

    let raw = tokio::fs::read(&upload.temp_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("read staging for hash: {e}")))?;
    let blake3_hex = content_hash_hex(&raw);
    let final_path = content_cas_path(&family_id, &blake3_hex);

    let deduplicated = final_path.exists();
    if deduplicated {
        let _ = tokio::fs::remove_file(&upload.temp_path).await;
    } else {
        ensure_parent(&final_path).await?;
        tokio::fs::rename(&upload.temp_path, &final_path)
            .await
            .map_err(|e| NestGateError::io_error(format!("rename to CAS: {e}")))?;
    }

    Ok(json!({
        "stream_id": stream_id,
        "hash": blake3_hex,
        "size": upload.total_size,
        "stored": true,
        "deduplicated": deduplicated,
        "family_id": family_id,
    }))
}

/// `content.retrieve_stream` — begin a chunked CAS download by hash.
pub async fn content_retrieve_stream_begin(
    params: Value,
    family_fallback: Option<&str>,
) -> Result<Value> {
    let family_id = resolve_family_id(&params, family_fallback)?;
    let hash = params
        .get("hash")
        .and_then(Value::as_str)
        .ok_or_else(|| NestGateError::invalid_input_with_field("hash", "BLAKE3 hex required"))?;

    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(NestGateError::invalid_input_with_field(
            "hash",
            "must be 64-char lowercase hex BLAKE3 digest",
        ));
    }

    validate_segment(family_id, "family_id")?;

    let mut chunk_size = params
        .get("chunk_size")
        .and_then(Value::as_u64)
        .unwrap_or(MAX_STREAM_CHUNK)
        .min(MAX_STREAM_CHUNK);
    if chunk_size == 0 {
        chunk_size = MAX_STREAM_CHUNK;
    }

    let path = content_cas_path(family_id, hash);
    if !path.exists() {
        return Err(NestGateError::not_found(format!(
            "content not found: {hash}"
        )));
    }

    let meta = tokio::fs::metadata(&path)
        .await
        .map_err(|e| NestGateError::io_error(format!("stat: {e}")))?;
    let total_size = meta.len();

    let stream_id = Uuid::new_v4().to_string();
    let session = RetrieveSession {
        path,
        total_size,
        chunk_size,
        created: Instant::now(),
    };

    let mut guard = maps().lock().await;
    prune_stale_streams(&mut guard);
    guard.retrieves.insert(stream_id.clone(), session);
    drop(guard);

    Ok(json!({
        "stream_id": stream_id,
        "total_size": total_size,
        "chunk_size": chunk_size,
        "family_id": family_id,
        "hash": hash,
    }))
}

#[cfg(test)]
#[path = "content_stream_tests.rs"]
mod tests;
