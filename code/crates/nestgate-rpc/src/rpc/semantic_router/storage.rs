// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage domain semantic methods
//!
//! Handles storage.* and storage.dataset.* semantic method routing.

use super::{MetadataBackend, SemanticRouter};
use crate::rpc::tarpc_types::DatasetParams;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use bytes::Bytes;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};

/// Route storage.put → `store_object`
pub(super) async fn storage_put(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;
    let data_b64 = params["data"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("data", "base64 string required"))?;

    // Decode base64
    let data = STANDARD.decode(data_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("data", format!("Invalid base64: {e}"))
    })?;

    // Call internal implementation
    let result = router
        .client
        .store_object(dataset, key, Bytes::from(data), None)
        .await?;

    serde_json::to_value(&result).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize object info: {e}"),
            "semantic_router",
        )
    })
}

/// Route storage.get → `retrieve_object`
pub(super) async fn storage_get(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;

    let data = router.client.retrieve_object(dataset, key).await?;

    // Encode to base64 for transport
    let data_b64 = STANDARD.encode(data.as_ref());

    Ok(json!({
        "data": data_b64,
        "size": data.len()
    }))
}

/// Route storage.delete → `delete_object`
pub(super) async fn storage_delete(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;

    let result = router.client.delete_object(dataset, key).await?;

    Ok(json!({
        "success": result.success,
        "message": result.message
    }))
}

/// Route storage.list → `list_objects`
pub(super) async fn storage_list(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let prefix = params["prefix"].as_str().map(String::from);

    let objects = router.client.list_objects(dataset, prefix, None).await?;

    Ok(json!({
        "objects": objects,
        "count": objects.len()
    }))
}

/// Route storage.exists → check if object exists
pub(super) async fn storage_exists(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;

    let exists = router
        .client
        .get_object_metadata(dataset, key)
        .await
        .is_ok();

    Ok(json!({ "exists": exists }))
}

/// Route storage.metadata → get object metadata
pub(super) async fn storage_metadata(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;

    let meta = router.client.get_object_metadata(dataset, key).await?;

    serde_json::to_value(&meta).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize object metadata: {e}"),
            "semantic_router",
        )
    })
}

// ==================== BLOB / STREAMING OPERATIONS ====================

fn resolve_family_id_from_params(params: &Value) -> &str {
    params["family_id"].as_str().unwrap_or("default")
}

fn family_dir(family_id: &str) -> std::path::PathBuf {
    nestgate_config::config::storage_paths::get_storage_base_path()
        .join("datasets")
        .join(family_id)
}

fn blob_path(family_id: &str, key: &str) -> std::path::PathBuf {
    family_dir(family_id).join("_blobs").join(key)
}

fn object_path(family_id: &str, key: &str) -> std::path::PathBuf {
    family_dir(family_id).join(key)
}

/// Route `storage.store_stream`
pub(super) async fn storage_store_stream(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    crate::rpc::storage_stream::storage_store_stream_begin(params, Some("default")).await
}

/// Route `storage.store_stream_chunk`
pub(super) async fn storage_store_stream_chunk(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    crate::rpc::storage_stream::storage_store_stream_chunk(params).await
}

/// Route `storage.retrieve_stream`
pub(super) async fn storage_retrieve_stream(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    crate::rpc::storage_stream::storage_retrieve_stream_begin(params, Some("default")).await
}

/// Route `storage.retrieve_stream_chunk`
pub(super) async fn storage_retrieve_stream_chunk(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    crate::rpc::storage_stream::storage_retrieve_stream_chunk(params).await
}

/// Route `storage.store_blob`
pub(super) async fn storage_store_blob(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;
    let blob_b64 = params["blob"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("blob", "base64 string required"))?;
    let fid = resolve_family_id_from_params(&params);

    let data = STANDARD.decode(blob_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("blob", format!("Invalid base64: {e}"))
    })?;

    let path = blob_path(fid, key);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| NestGateError::io_error(format!("mkdir: {e}")))?;
    }
    let size = data.len();
    tokio::fs::write(&path, &data)
        .await
        .map_err(|e| NestGateError::io_error(format!("write blob: {e}")))?;

    Ok(json!({"status": "stored", "key": key, "family_id": fid, "size": size}))
}

/// Route `storage.retrieve_blob`
pub(super) async fn storage_retrieve_blob(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;
    let fid = resolve_family_id_from_params(&params);

    let path = blob_path(fid, key);
    if !path.exists() {
        return Ok(json!({"blob": null, "key": key}));
    }
    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| NestGateError::io_error(format!("read blob: {e}")))?;

    Ok(json!({"blob": STANDARD.encode(&data), "key": key, "family_id": fid, "size": data.len()}))
}

/// Route `storage.retrieve_range` — byte-range read for streaming/chunked fetch
pub(super) async fn storage_retrieve_range(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    use tokio::io::{AsyncReadExt, AsyncSeekExt};

    const MAX_CHUNK: u64 = 4 * 1024 * 1024;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;
    let fid = resolve_family_id_from_params(&params);
    let offset = params["offset"].as_u64().unwrap_or(0);
    let raw_length = params["length"]
        .as_u64()
        .ok_or_else(|| NestGateError::invalid_input_with_field("length", "u64 required"))?;
    let length = usize::try_from(raw_length.min(MAX_CHUNK)).unwrap_or(usize::MAX);

    let bp = blob_path(fid, key);
    let op = object_path(fid, key);
    let path = if bp.exists() {
        bp
    } else if op.exists() {
        op
    } else {
        return Ok(json!({"data": null, "key": key, "error": "not_found"}));
    };

    let mut file = tokio::fs::File::open(&path)
        .await
        .map_err(|e| NestGateError::io_error(format!("open: {e}")))?;
    let total_size = file
        .metadata()
        .await
        .map_err(|e| NestGateError::io_error(format!("stat: {e}")))?
        .len();
    file.seek(std::io::SeekFrom::Start(offset))
        .await
        .map_err(|e| NestGateError::io_error(format!("seek: {e}")))?;

    let mut buf = vec![0u8; length];
    let bytes_read = file
        .read(&mut buf)
        .await
        .map_err(|e| NestGateError::io_error(format!("read: {e}")))?;
    buf.truncate(bytes_read);

    Ok(json!({
        "data": STANDARD.encode(&buf),
        "offset": offset,
        "length": bytes_read,
        "total_size": total_size,
        "key": key,
        "family_id": fid,
        "encoding": "base64"
    }))
}

/// Route `storage.object.size` — blob metadata (total byte size)
pub(super) async fn storage_object_size(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;
    let fid = resolve_family_id_from_params(&params);

    let bp = blob_path(fid, key);
    let op = object_path(fid, key);
    let path = if bp.exists() {
        bp
    } else if op.exists() {
        op
    } else {
        return Ok(json!({"size": null, "key": key, "exists": false}));
    };

    let meta = tokio::fs::metadata(&path)
        .await
        .map_err(|e| NestGateError::io_error(format!("stat: {e}")))?;

    Ok(json!({"size": meta.len(), "key": key, "family_id": fid, "exists": true}))
}

/// Route `storage.namespaces.list` — enumerate namespaces under a family
pub(super) async fn storage_namespaces_list(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let fid = resolve_family_id_from_params(&params);
    let dir = family_dir(fid);
    let mut namespaces = Vec::new();
    if dir.exists() {
        let mut entries = tokio::fs::read_dir(&dir)
            .await
            .map_err(|e| NestGateError::io_error(format!("readdir: {e}")))?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with('_')
                && entry
                    .file_type()
                    .await
                    .map(|ft| ft.is_dir())
                    .unwrap_or(false)
            {
                namespaces.push(name.to_string());
            }
        }
    }
    namespaces.sort();
    Ok(json!({"namespaces": namespaces, "family_id": fid, "count": namespaces.len()}))
}

// ==================== DATASET OPERATIONS ====================

/// Route storage.dataset.create → `create_dataset`
pub(super) async fn dataset_create(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "string required"))?;
    let description = params["description"].as_str().unwrap_or("").to_string();

    let dataset_params = DatasetParams {
        description: Some(description),
        ..Default::default()
    };

    let dataset = router.client.create_dataset(name, dataset_params).await?;

    Ok(json!({
        "name": dataset.name,
        "created_at": dataset.created_at,
        "status": dataset.status
    }))
}

/// Route storage.dataset.get → `get_dataset`
pub(super) async fn dataset_get(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "string required"))?;

    let dataset = router.client.get_dataset(name).await?;

    serde_json::to_value(dataset).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize dataset: {e}"),
            "semantic_router",
        )
    })
}

/// Route storage.dataset.list → `list_datasets`
pub(super) async fn dataset_list(
    router: &SemanticRouter<impl MetadataBackend>,
    _params: Value,
) -> Result<Value> {
    let datasets = router.client.list_datasets().await?;

    Ok(json!({
        "datasets": datasets,
        "count": datasets.len()
    }))
}

/// Route storage.dataset.delete → `delete_dataset`
pub(super) async fn dataset_delete(
    router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "string required"))?;

    let result = router.client.delete_dataset(name).await?;

    Ok(json!({
        "success": result.success,
        "message": result.message
    }))
}

#[cfg(test)]
#[path = "storage_tests.rs"]
mod tests;
