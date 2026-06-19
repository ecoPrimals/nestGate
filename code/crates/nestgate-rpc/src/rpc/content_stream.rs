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
        dataset: String::from("_content_stream"),
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
mod tests {
    use super::*;
    use nestgate_config::config::storage_paths::get_storage_base_path;
    use serde_json::json;

    async fn cleanup_family(family_id: &str) {
        let base = get_storage_base_path().join("datasets").join(family_id);
        let _ = tokio::fs::remove_dir_all(&base).await;
    }

    #[tokio::test]
    async fn content_stream_store_and_retrieve_roundtrip() {
        let family_id = format!("content-stream-rt-{}", Uuid::new_v4());
        let payload = vec![0xAB_u8; 6000];

        let begin = content_store_stream_begin(
            json!({
                "family_id": family_id,
                "total_size": payload.len(),
                "content_type": "application/octet-stream"
            }),
            Some("default"),
        )
        .await
        .expect("content store stream begin");

        let sid = begin["stream_id"].as_str().unwrap();
        let chunk_size = begin["chunk_size"].as_u64().unwrap() as usize;
        assert!(chunk_size > 0);

        let mut offset = 0_u64;
        while offset < payload.len() as u64 {
            let end = (offset as usize + chunk_size).min(payload.len());
            let is_last = end == payload.len();
            let chunk = &payload[offset as usize..end];
            let result = content_store_stream_chunk(json!({
                "stream_id": sid,
                "offset": offset,
                "data": STANDARD.encode(chunk),
                "is_last": is_last
            }))
            .await
            .expect("content store stream chunk");

            if is_last {
                assert!(result["hash"].is_string());
                assert_eq!(result["stored"], true);
                let hash = result["hash"].as_str().unwrap();
                assert_eq!(hash.len(), 64);
                assert_eq!(content_hash_hex(&payload), hash, "BLAKE3 mismatch");

                let r_begin = content_retrieve_stream_begin(
                    json!({
                        "family_id": family_id,
                        "hash": hash
                    }),
                    Some("default"),
                )
                .await
                .expect("content retrieve stream begin");

                let rsid = r_begin["stream_id"].as_str().unwrap();
                let total = r_begin["total_size"].as_u64().unwrap();
                assert_eq!(total, payload.len() as u64);

                let mut ro = 0_u64;
                let mut out = Vec::new();
                loop {
                    let part = super::super::storage_stream::storage_retrieve_stream_chunk(json!({
                        "stream_id": rsid,
                        "offset": ro
                    }))
                    .await
                    .expect("content retrieve chunk");
                    let bytes = STANDARD.decode(part["data"].as_str().unwrap()).unwrap();
                    out.extend_from_slice(&bytes);
                    ro += bytes.len() as u64;
                    if part["is_last"].as_bool() == Some(true) {
                        break;
                    }
                }

                assert_eq!(out, payload);
            }

            offset = end as u64;
        }

        cleanup_family(&family_id).await;
    }

    #[tokio::test]
    async fn content_stream_dedup_on_second_upload() {
        let family_id = format!("content-stream-dedup-{}", Uuid::new_v4());
        let payload = b"dedup stream test";

        let store_once = |fam: &str, data: &[u8]| {
            let fam = fam.to_owned();
            let data = data.to_vec();
            async move {
                let begin = content_store_stream_begin(
                    json!({"family_id": fam, "total_size": data.len()}),
                    Some("default"),
                )
                .await?;
                let sid = begin["stream_id"].as_str().unwrap().to_owned();
                content_store_stream_chunk(json!({
                    "stream_id": sid,
                    "offset": 0,
                    "data": STANDARD.encode(&data),
                    "is_last": true
                }))
                .await
            }
        };

        let first = store_once(&family_id, payload).await.unwrap();
        assert_eq!(first["deduplicated"], false);

        let second = store_once(&family_id, payload).await.unwrap();
        assert_eq!(second["deduplicated"], true);
        assert_eq!(first["hash"], second["hash"]);

        cleanup_family(&family_id).await;
    }

    #[tokio::test]
    async fn content_retrieve_stream_not_found() {
        let family_id = format!("content-stream-404-{}", Uuid::new_v4());
        let fake_hash = "b".repeat(64);
        let err = content_retrieve_stream_begin(
            json!({"family_id": family_id, "hash": fake_hash}),
            Some("default"),
        )
        .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn begin_rejects_missing_total_size() {
        let fam = format!("cs-no-size-{}", Uuid::new_v4());
        let err = content_store_stream_begin(json!({"family_id": fam}), None).await;
        assert!(err.is_err(), "missing total_size should be rejected");
        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn begin_rejects_oversized_total() {
        let fam = format!("cs-oversize-{}", Uuid::new_v4());
        let err = content_store_stream_begin(
            json!({"family_id": fam, "total_size": MAX_STREAM_TOTAL + 1}),
            None,
        )
        .await;
        assert!(err.is_err());
        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn chunk_rejects_unknown_stream_id() {
        let err = content_store_stream_chunk(json!({
            "stream_id": "nonexistent-uuid",
            "offset": 0,
            "data": STANDARD.encode(b"x"),
            "is_last": true
        }))
        .await;
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("unknown") || msg.contains("expired"));
    }

    #[tokio::test]
    async fn chunk_rejects_bad_offset() {
        let fam = format!("cs-offset-{}", Uuid::new_v4());
        let begin = content_store_stream_begin(json!({"family_id": fam, "total_size": 100}), None)
            .await
            .unwrap();
        let sid = begin["stream_id"].as_str().unwrap();

        let err = content_store_stream_chunk(json!({
            "stream_id": sid,
            "offset": 50,
            "data": STANDARD.encode(b"data"),
            "is_last": false
        }))
        .await;
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("expected 0"));
        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn chunk_rejects_exceeding_total_size() {
        let fam = format!("cs-exceed-{}", Uuid::new_v4());
        let begin = content_store_stream_begin(json!({"family_id": fam, "total_size": 4}), None)
            .await
            .unwrap();
        let sid = begin["stream_id"].as_str().unwrap();

        let err = content_store_stream_chunk(json!({
            "stream_id": sid,
            "offset": 0,
            "data": STANDARD.encode(b"too much data for 4 bytes"),
            "is_last": true
        }))
        .await;
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("exceed"));
        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn chunk_rejects_invalid_base64() {
        let fam = format!("cs-b64-{}", Uuid::new_v4());
        let begin = content_store_stream_begin(json!({"family_id": fam, "total_size": 100}), None)
            .await
            .unwrap();
        let sid = begin["stream_id"].as_str().unwrap();

        let err = content_store_stream_chunk(json!({
            "stream_id": sid,
            "offset": 0,
            "data": "not!valid!base64!!!",
            "is_last": false
        }))
        .await;
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("base64"));
        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn begin_missing_family_id_errors() {
        let err = content_store_stream_begin(json!({"total_size": 10}), None).await;
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("family_id") || msg.contains("family"));
    }

    #[tokio::test]
    async fn retrieve_begin_invalid_hash_format() {
        let fam = format!("cs-badhash-{}", Uuid::new_v4());
        let err =
            content_retrieve_stream_begin(json!({"family_id": fam, "hash": "tooshort"}), None)
                .await;
        assert!(err.is_err());
        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn begin_uses_family_fallback() {
        let fam = format!("cs-fallback-{}", Uuid::new_v4());
        let result = content_store_stream_begin(json!({"total_size": 16}), Some(&fam)).await;
        assert!(result.is_ok(), "should succeed with fallback family");
        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn empty_upload_fast_path_returns_hash() {
        let fam = format!("cs-empty-{}", Uuid::new_v4());
        let result = content_store_stream_begin(
            json!({"family_id": &fam, "total_size": 0, "content_type": "text/plain"}),
            None,
        )
        .await
        .expect("empty upload should succeed");

        let expected_hash = content_hash_hex(&[]);
        assert_eq!(result["hash"].as_str().unwrap(), expected_hash);
        assert_eq!(result["size"], 0);
        assert_eq!(result["stored"], true);
        assert_eq!(result["family_id"].as_str().unwrap(), fam);
        assert!(result.get("stream_id").is_some());

        let cas_path = content_cas_path(&fam, &expected_hash);
        assert!(cas_path.exists(), "CAS file should exist for empty content");
        let data = tokio::fs::read(&cas_path).await.unwrap();
        assert!(
            data.is_empty(),
            "empty content should be zero bytes on disk"
        );

        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn empty_upload_dedup_on_second_call() {
        let fam = format!("cs-empty-dedup-{}", Uuid::new_v4());
        let params = json!({"family_id": &fam, "total_size": 0});

        let first = content_store_stream_begin(params.clone(), None)
            .await
            .unwrap();
        let second = content_store_stream_begin(params, None).await.unwrap();

        assert_eq!(first["hash"], second["hash"]);
        assert_eq!(second["deduplicated"], true);

        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn chunk_rejects_oversized_decoded_data() {
        let fam = format!("cs-chunkmax-{}", Uuid::new_v4());
        let total = MAX_STREAM_CHUNK + 1024;
        let begin =
            content_store_stream_begin(json!({"family_id": &fam, "total_size": total}), None)
                .await
                .unwrap();
        let sid = begin["stream_id"].as_str().unwrap();

        let oversized = vec![0xCC_u8; (MAX_STREAM_CHUNK + 1) as usize];
        let err = content_store_stream_chunk(json!({
            "stream_id": sid,
            "offset": 0,
            "data": STANDARD.encode(&oversized),
            "is_last": false
        }))
        .await;
        assert!(err.is_err());
        let msg = format!("{}", err.unwrap_err());
        assert!(msg.contains("chunk exceeds") || msg.contains("MAX_STREAM_CHUNK"));

        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn retrieve_stream_begin_valid_hash_returns_stream() {
        let fam = format!("cs-retrieve-ok-{}", Uuid::new_v4());
        let payload = b"retrieve me via stream";

        let begin = content_store_stream_begin(
            json!({"family_id": &fam, "total_size": payload.len()}),
            None,
        )
        .await
        .unwrap();
        let sid = begin["stream_id"].as_str().unwrap();

        let fin = content_store_stream_chunk(json!({
            "stream_id": sid,
            "offset": 0,
            "data": STANDARD.encode(payload),
            "is_last": true
        }))
        .await
        .unwrap();
        let hash = fin["hash"].as_str().unwrap();

        let r = content_retrieve_stream_begin(json!({"family_id": &fam, "hash": hash}), None)
            .await
            .expect("should open retrieve stream");
        assert!(r.get("stream_id").is_some());
        assert_eq!(r["total_size"].as_u64().unwrap(), payload.len() as u64);

        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn chunk_missing_stream_id_param_errors() {
        let err = content_store_stream_chunk(json!({
            "offset": 0,
            "data": STANDARD.encode(b"x"),
            "is_last": true
        }))
        .await;
        assert!(err.is_err(), "missing stream_id should be rejected");
    }

    #[tokio::test]
    async fn chunk_missing_offset_param_errors() {
        let fam = format!("cs-no-offset-{}", Uuid::new_v4());
        let begin = content_store_stream_begin(json!({"family_id": &fam, "total_size": 100}), None)
            .await
            .unwrap();
        let sid = begin["stream_id"].as_str().unwrap();

        let err = content_store_stream_chunk(json!({
            "stream_id": sid,
            "data": STANDARD.encode(b"x"),
            "is_last": false
        }))
        .await;
        assert!(err.is_err(), "missing offset should be rejected");
        cleanup_family(&fam).await;
    }

    #[tokio::test]
    async fn chunk_missing_data_param_errors() {
        let fam = format!("cs-no-data-{}", Uuid::new_v4());
        let begin = content_store_stream_begin(json!({"family_id": &fam, "total_size": 100}), None)
            .await
            .unwrap();
        let sid = begin["stream_id"].as_str().unwrap();

        let err = content_store_stream_chunk(json!({
            "stream_id": sid,
            "offset": 0,
            "is_last": false
        }))
        .await;
        assert!(err.is_err(), "missing data should be rejected");
        cleanup_family(&fam).await;
    }
}
