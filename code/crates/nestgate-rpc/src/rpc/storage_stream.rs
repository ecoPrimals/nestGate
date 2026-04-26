// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Chunked JSON-RPC storage for large binary payloads (`storage.store_stream*` / `storage.retrieve_stream*`).
//!
//! Objects are persisted under `{storage_base}/datasets/{family_id}/{dataset}/_blobs/{key}` to align
//! with namespace-isolated blob layout used by the isomorphic Unix adapter.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Mutex;
use uuid::Uuid;

/// Maximum decoded bytes per `storage.store_stream_chunk` / `storage.retrieve_stream_chunk` response.
pub const MAX_STREAM_CHUNK: u64 = 4 * 1024 * 1024;

/// Upper bound on declared object size for streaming uploads (1 TiB).
const MAX_STREAM_TOTAL: u64 = 1_u64 << 40;

/// In-flight streams are dropped if idle longer than this (upload or download session).
const STREAM_TTL: Duration = Duration::from_secs(3600);

struct StoreUpload {
    temp_path: PathBuf,
    final_path: PathBuf,
    total_size: u64,
    bytes_written: u64,
    created: Instant,
    family_id: String,
    dataset: String,
    key: String,
    content_type: Option<String>,
}

struct RetrieveSession {
    path: PathBuf,
    total_size: u64,
    chunk_size: u64,
    created: Instant,
}

struct StreamMaps {
    uploads: HashMap<String, StoreUpload>,
    retrieves: HashMap<String, RetrieveSession>,
}

fn maps() -> &'static Arc<Mutex<StreamMaps>> {
    static MAPS: OnceLock<Arc<Mutex<StreamMaps>>> = OnceLock::new();
    MAPS.get_or_init(|| {
        Arc::new(Mutex::new(StreamMaps {
            uploads: HashMap::new(),
            retrieves: HashMap::new(),
        }))
    })
}

fn validate_segment(name: &str, field: &'static str) -> Result<()> {
    if name.is_empty()
        || name.contains('/')
        || name.contains('\\')
        || name.contains("..")
        || name.starts_with('.')
    {
        return Err(NestGateError::invalid_input_with_field(
            field,
            "must be a non-empty simple name without path separators",
        ));
    }
    Ok(())
}

fn staging_path(family_id: &str, stream_id: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join(".stream_staging")
        .join(format!("{stream_id}.part"))
}

fn namespaced_blob_path(family_id: &str, dataset: &str, key: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join(dataset)
        .join("_blobs")
        .join(key)
}

/// Flat blob path used by `storage.store_blob` / `storage.retrieve_blob`.
/// `retrieve_stream_begin` checks this as a fallback when the dataset-scoped
/// path does not exist, so blobs stored via `store_blob` can be streamed.
fn flat_blob_path(family_id: &str, key: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_blobs")
        .join(key)
}

fn resolve_family_id<'a>(params: &'a Value, fallback: Option<&'a str>) -> Result<&'a str> {
    if let Some(s) = params.get("family_id").and_then(Value::as_str) {
        return Ok(s);
    }
    fallback.ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "family_id",
            "family_id required (or connect via a family-scoped socket)",
        )
    })
}

fn extract_dataset(params: &Value) -> &str {
    params
        .get("dataset")
        .and_then(Value::as_str)
        .or_else(|| params.get("namespace").and_then(Value::as_str))
        .unwrap_or("shared")
}

async fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| NestGateError::io_error(format!("mkdir: {e}")))?;
    }
    Ok(())
}

fn ttl_expired(created: Instant) -> bool {
    created.elapsed() > STREAM_TTL
}

/// `storage.store_stream` — begin a chunked upload; returns a `stream_id` for follow-up chunks.
pub async fn storage_store_stream_begin(
    params: Value,
    family_fallback: Option<&str>,
) -> Result<Value> {
    let family_id = resolve_family_id(&params, family_fallback)?;
    let key = params
        .get("key")
        .and_then(Value::as_str)
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;
    let dataset = extract_dataset(&params);
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
    validate_segment(dataset, "dataset")?;
    validate_segment(key, "key")?;

    let content_type = params
        .get("content_type")
        .and_then(|v| v.as_str())
        .map(str::to_owned);

    let stream_id = Uuid::new_v4().to_string();
    let temp_path = staging_path(family_id, &stream_id);
    let final_path = namespaced_blob_path(family_id, dataset, key);

    ensure_parent(&temp_path).await?;

    if total_size == 0 {
        ensure_parent(&final_path).await?;
        tokio::fs::write(&final_path, [])
            .await
            .map_err(|e| NestGateError::io_error(format!("write empty blob: {e}")))?;
        return Ok(json!({
            "stream_id": stream_id,
            "chunk_size": MAX_STREAM_CHUNK,
            "family_id": family_id,
            "dataset": dataset,
            "key": key,
            "total_size": total_size,
            "status": "stored",
            "size": 0,
            "content_type": content_type,
        }));
    }

    tokio::fs::File::create(&temp_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("create staging: {e}")))?;

    let upload = StoreUpload {
        temp_path,
        final_path,
        total_size,
        bytes_written: 0,
        created: Instant::now(),
        family_id: family_id.to_string(),
        dataset: dataset.to_string(),
        key: key.to_string(),
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
        "dataset": dataset,
        "key": key,
        "total_size": total_size,
    }))
}

fn prune_stale_streams(maps: &mut StreamMaps) {
    maps.uploads.retain(|_, u| !ttl_expired(u.created));
    maps.retrieves.retain(|_, r| !ttl_expired(r.created));
}

/// `storage.store_stream_chunk` — append a base64 chunk; finalize when `is_last` is true.
pub async fn storage_store_stream_chunk(params: Value) -> Result<Value> {
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
        .ok_or_else(|| NestGateError::invalid_input_with_field("is_last", "bool required"))?;

    let chunk = STANDARD.decode(data_b64.trim()).map_err(|e| {
        NestGateError::invalid_input_with_field("data", format!("Invalid base64: {e}"))
    })?;

    let chunk_len = u64::try_from(chunk.len())
        .map_err(|_| NestGateError::invalid_input_with_field("data", "chunk length overflow"))?;

    if chunk_len > MAX_STREAM_CHUNK {
        return Err(NestGateError::invalid_input_with_field(
            "data",
            format!("chunk exceeds maximum decoded size ({MAX_STREAM_CHUNK} bytes)"),
        ));
    }

    let mut guard = maps().lock().await;
    prune_stale_streams(&mut guard);

    let Some(mut upload) = guard.uploads.remove(stream_id) else {
        return Err(NestGateError::invalid_input_with_field(
            "stream_id",
            "unknown or expired stream_id",
        ));
    };

    if ttl_expired(upload.created) {
        let _ = std::fs::remove_file(&upload.temp_path);
        return Err(NestGateError::invalid_input_with_field(
            "stream_id",
            "stream expired",
        ));
    }

    if offset != upload.bytes_written {
        let expected = upload.bytes_written;
        guard.uploads.insert(stream_id.to_string(), upload);
        return Err(NestGateError::invalid_input_with_field(
            "offset",
            format!("expected {expected} for sequential upload, got {offset}"),
        ));
    }

    let next = upload
        .bytes_written
        .checked_add(chunk_len)
        .ok_or_else(|| NestGateError::invalid_input_with_field("data", "size overflow"))?;
    if next > upload.total_size {
        let _ = std::fs::remove_file(&upload.temp_path);
        return Err(NestGateError::invalid_input_with_field(
            "data",
            "chunk would exceed total_size",
        ));
    }

    {
        let mut file: File = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&upload.temp_path)
            .await
            .map_err(|e| NestGateError::io_error(format!("open staging: {e}")))?;
        file.seek(std::io::SeekFrom::Start(offset))
            .await
            .map_err(|e| NestGateError::io_error(format!("seek: {e}")))?;
        file.write_all(&chunk)
            .await
            .map_err(|e| NestGateError::io_error(format!("write staging: {e}")))?;
        file.flush()
            .await
            .map_err(|e| NestGateError::io_error(format!("flush staging: {e}")))?;
    }

    upload.bytes_written = next;

    if is_last {
        if upload.bytes_written != upload.total_size {
            let _ = std::fs::remove_file(&upload.temp_path);
            return Err(NestGateError::invalid_input_with_field(
                "is_last",
                format!(
                    "total bytes {} do not match declared total_size {}",
                    upload.bytes_written, upload.total_size
                ),
            ));
        }

        ensure_parent(&upload.final_path).await?;
        tokio::fs::rename(&upload.temp_path, &upload.final_path)
            .await
            .map_err(|e| NestGateError::io_error(format!("finalize rename: {e}")))?;

        drop(guard);

        return Ok(json!({
            "status": "stored",
            "key": upload.key,
            "family_id": upload.family_id,
            "dataset": upload.dataset,
            "size": upload.total_size,
            "content_type": upload.content_type,
            "ack": true,
            "bytes_written": upload.bytes_written,
        }));
    }

    guard.uploads.insert(stream_id.to_string(), upload);
    drop(guard);

    Ok(json!({
        "ack": true,
        "bytes_received": chunk.len(),
        "bytes_written": next,
        "is_last": false
    }))
}

/// `storage.retrieve_stream` — open a read session for chunked download.
pub async fn storage_retrieve_stream_begin(
    params: Value,
    family_fallback: Option<&str>,
) -> Result<Value> {
    let family_id = resolve_family_id(&params, family_fallback)?;
    let key = params
        .get("key")
        .and_then(Value::as_str)
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;
    let dataset = extract_dataset(&params);

    validate_segment(family_id, "family_id")?;
    validate_segment(dataset, "dataset")?;
    validate_segment(key, "key")?;

    let mut chunk_size = params
        .get("chunk_size")
        .and_then(Value::as_u64)
        .unwrap_or(MAX_STREAM_CHUNK)
        .min(MAX_STREAM_CHUNK);
    if chunk_size == 0 {
        chunk_size = MAX_STREAM_CHUNK;
    }

    let ns_path = namespaced_blob_path(family_id, dataset, key);
    let flat = flat_blob_path(family_id, key);
    let path = if ns_path.exists() {
        ns_path
    } else if flat.exists() {
        flat
    } else {
        return Err(NestGateError::not_found(format!(
            "blob not found for {family_id}/{dataset}/{key}"
        )));
    };

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
        "dataset": dataset,
        "key": key,
    }))
}

/// `storage.retrieve_stream_chunk` — read the next range of bytes for a download session.
pub async fn storage_retrieve_stream_chunk(params: Value) -> Result<Value> {
    let stream_id = params
        .get("stream_id")
        .and_then(Value::as_str)
        .ok_or_else(|| NestGateError::invalid_input_with_field("stream_id", "string required"))?;
    let offset = params
        .get("offset")
        .and_then(Value::as_u64)
        .ok_or_else(|| NestGateError::invalid_input_with_field("offset", "u64 required"))?;

    let mut guard = maps().lock().await;
    prune_stale_streams(&mut guard);

    let Some(session) = guard.retrieves.remove(stream_id) else {
        return Err(NestGateError::invalid_input_with_field(
            "stream_id",
            "unknown or expired stream_id",
        ));
    };

    if ttl_expired(session.created) {
        return Err(NestGateError::invalid_input_with_field(
            "stream_id",
            "stream expired",
        ));
    }

    if offset > session.total_size {
        guard.retrieves.insert(stream_id.to_string(), session);
        return Err(NestGateError::invalid_input_with_field(
            "offset",
            "offset past end of object",
        ));
    }

    let remaining = session.total_size - offset;
    let to_read = u64::min(session.chunk_size, remaining);
    let to_read_usize = usize::try_from(to_read).unwrap_or(usize::MAX);

    let mut file = tokio::fs::File::open(&session.path)
        .await
        .map_err(|e| NestGateError::io_error(format!("open: {e}")))?;
    file.seek(std::io::SeekFrom::Start(offset))
        .await
        .map_err(|e| NestGateError::io_error(format!("seek: {e}")))?;

    let mut buf = vec![0u8; to_read_usize];
    let n = file
        .read(&mut buf)
        .await
        .map_err(|e| NestGateError::io_error(format!("read: {e}")))?;
    buf.truncate(n);

    let is_last = offset.saturating_add(u64::try_from(n).unwrap_or(0)) >= session.total_size;

    let total_size = session.total_size;
    if is_last {
        drop(session);
    } else {
        guard.retrieves.insert(stream_id.to_string(), session);
    }
    drop(guard);

    Ok(json!({
        "data": STANDARD.encode(&buf),
        "offset": offset,
        "length": n,
        "total_size": total_size,
        "is_last": is_last,
        "encoding": "base64"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    async fn cleanup_family(family_id: &str) {
        let base = get_storage_base_path().join("datasets").join(family_id);
        let _ = tokio::fs::remove_dir_all(&base).await;
    }

    #[tokio::test]
    async fn store_and_retrieve_stream_roundtrip_multichunk() {
        let family_id = format!("stream-test-{}", Uuid::new_v4());
        let dataset = "tensors";
        let key = "weights.bin";
        let payload: Vec<u8> = (0_u8..=211).cycle().take(9000).collect();

        let begin = storage_store_stream_begin(
            json!({
                "family_id": family_id,
                "dataset": dataset,
                "key": key,
                "total_size": payload.len(),
                "content_type": "application/octet-stream"
            }),
            Some("default"),
        )
        .await
        .expect("begin");

        let stream_id = begin["stream_id"].as_str().unwrap().to_string();

        let chunk_size = 4000usize;
        let mut offset = 0_u64;
        while offset < payload.len() as u64 {
            let end = usize::min(offset as usize + chunk_size, payload.len());
            let piece = &payload[offset as usize..end];
            let is_last = end == payload.len();
            let chunk = storage_store_stream_chunk(json!({
                "stream_id": stream_id,
                "offset": offset,
                "data": STANDARD.encode(piece),
                "is_last": is_last
            }))
            .await
            .expect("chunk");
            if !is_last {
                assert_eq!(chunk["ack"], true);
            } else {
                assert_eq!(chunk["status"], "stored");
                assert_eq!(chunk["size"], payload.len());
            }
            offset = end as u64;
        }

        let r_begin = storage_retrieve_stream_begin(
            json!({
                "family_id": family_id,
                "dataset": dataset,
                "key": key,
                "chunk_size": 3000
            }),
            Some("default"),
        )
        .await
        .expect("retrieve begin");

        let rsid = r_begin["stream_id"].as_str().unwrap();
        let total_size = r_begin["total_size"].as_u64().unwrap();
        assert_eq!(total_size, payload.len() as u64);

        let mut ro = 0_u64;
        let mut out = Vec::new();
        loop {
            let part = storage_retrieve_stream_chunk(json!({
                "stream_id": rsid,
                "offset": ro
            }))
            .await
            .expect("retrieve chunk");
            let b64 = part["data"].as_str().expect("data");
            let bytes = STANDARD.decode(b64).expect("b64");
            out.extend_from_slice(&bytes);
            ro = ro.saturating_add(bytes.len() as u64);
            if part["is_last"].as_bool() == Some(true) {
                break;
            }
        }

        assert_eq!(out, payload);

        cleanup_family(&family_id).await;
    }

    #[tokio::test]
    async fn store_stream_rejects_out_of_order_chunk() {
        let family_id = format!("stream-oo-{}", Uuid::new_v4());
        let begin = storage_store_stream_begin(
            json!({
                "family_id": family_id,
                "dataset": "d",
                "key": "k",
                "total_size": 10
            }),
            Some("default"),
        )
        .await
        .unwrap();
        let sid = begin["stream_id"].as_str().unwrap();
        let err = storage_store_stream_chunk(json!({
            "stream_id": sid,
            "offset": 5,
            "data": STANDARD.encode([0_u8; 5]),
            "is_last": false
        }))
        .await
        .expect_err("out of order");
        assert!(!err.to_string().is_empty());

        let mut guard = maps().lock().await;
        let _ = guard.uploads.remove(sid);
        drop(guard);
        cleanup_family(&family_id).await;
    }

    #[tokio::test]
    async fn retrieve_stream_falls_back_to_flat_blob_path() {
        let family_id = format!("stream-flat-{}", Uuid::new_v4());
        let key = "legacy.bin";
        let payload = vec![0xCA_u8; 256];

        let flat = flat_blob_path(&family_id, key);
        if let Some(parent) = flat.parent() {
            tokio::fs::create_dir_all(parent).await.unwrap();
        }
        tokio::fs::write(&flat, &payload).await.unwrap();

        let begin = storage_retrieve_stream_begin(
            json!({
                "family_id": family_id,
                "key": key,
                "dataset": "shared"
            }),
            Some("default"),
        )
        .await
        .expect("should find via flat fallback");

        assert_eq!(begin["total_size"], 256);
        let stream_id = begin["stream_id"].as_str().unwrap();

        let chunk = storage_retrieve_stream_chunk(json!({
            "stream_id": stream_id,
            "offset": 0
        }))
        .await
        .expect("chunk");
        let bytes = STANDARD.decode(chunk["data"].as_str().unwrap()).unwrap();
        assert_eq!(bytes, payload);
        assert_eq!(chunk["is_last"], true);

        cleanup_family(&family_id).await;
    }
}
