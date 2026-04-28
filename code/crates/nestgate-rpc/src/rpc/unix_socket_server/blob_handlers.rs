// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Blob storage JSON-RPC handlers — base64 payloads and byte-range reads.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tracing::debug;

use super::StorageState;
use super::storage_handlers::{
    blob_key_path, dataset_key_path, ensure_parent_dirs, resolve_family_id,
};

/// `storage.store_blob` - Store binary blob (base64 encoded, filesystem-backed)
pub(super) async fn storage_store_blob(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let blob_base64 = params["blob"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("blob", "blob (base64 string) required")
    })?;
    let family_id = resolve_family_id(params, state)?;

    let blob_data = STANDARD.decode(blob_base64).map_err(|e| {
        NestGateError::invalid_input_with_field("blob", format!("Invalid base64: {e}"))
    })?;

    let original_size = blob_data.len();
    debug!(
        "storage.store_blob: family_id='{}', key='{}', blob_size={} bytes",
        family_id, key, original_size
    );

    let write_data = if let Some(ref enc) = state.encryption {
        enc.encrypt(&blob_data)?
    } else {
        blob_data
    };

    let blob_path = blob_key_path(family_id, key);
    ensure_parent_dirs(&blob_path).await?;
    tokio::fs::write(&blob_path, &write_data)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to write blob {family_id}/{key}: {e}"))
        })?;

    Ok(json!({
        "status": "stored",
        "key": key,
        "family_id": family_id,
        "size": original_size
    }))
}

/// `storage.retrieve_blob` - Retrieve binary blob (base64 encoded, filesystem-backed)
pub(super) async fn storage_retrieve_blob(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;

    let blob_path = blob_key_path(family_id, key);
    if !blob_path.exists() {
        return Ok(json!({"blob": null, "key": key}));
    }

    let raw_data = tokio::fs::read(&blob_path).await.map_err(|e| {
        NestGateError::io_error(format!("Failed to read blob {family_id}/{key}: {e}"))
    })?;

    let blob_data =
        if crate::rpc::storage_encryption::StorageEncryption::is_encrypted_envelope(&raw_data) {
            if let Some(ref enc) = state.encryption {
                enc.decrypt(&raw_data)?
            } else {
                raw_data
            }
        } else {
            raw_data
        };

    Ok(json!({
        "blob": STANDARD.encode(&blob_data),
        "key": key,
        "family_id": family_id,
        "size": blob_data.len()
    }))
}

/// `storage.retrieve_range` — byte-range read for large objects (streaming tensors, datasets).
///
/// Callers read large blobs in chunks: first `storage.object.size` to learn the total,
/// then N calls to `retrieve_range` with `{offset, length}`.
pub(super) async fn storage_retrieve_range(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    const MAX_CHUNK: u64 = 4 * 1024 * 1024; // 4 MiB per chunk

    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;

    let offset = params["offset"].as_u64().unwrap_or(0);
    let raw_length = params["length"].as_u64().ok_or_else(|| {
        NestGateError::invalid_input_with_field("length", "length (u64) required")
    })?;
    let length = usize::try_from(raw_length.min(MAX_CHUNK)).unwrap_or(usize::MAX);

    let blob_path = blob_key_path(family_id, key);
    let object_path = dataset_key_path(family_id, key);

    let path = if blob_path.exists() {
        blob_path
    } else if object_path.exists() {
        object_path
    } else {
        return Ok(json!({"data": null, "key": key, "error": "not_found"}));
    };

    let mut file = tokio::fs::File::open(&path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to open {family_id}/{key}: {e}")))?;

    let metadata = file
        .metadata()
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to stat {family_id}/{key}: {e}")))?;
    let total_size = metadata.len();

    file.seek(std::io::SeekFrom::Start(offset))
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to seek in {family_id}/{key}: {e}"))
        })?;

    let mut buf = vec![0u8; length];
    let bytes_read = file
        .read(&mut buf)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read {family_id}/{key}: {e}")))?;
    buf.truncate(bytes_read);

    Ok(json!({
        "data": STANDARD.encode(&buf),
        "offset": offset,
        "length": bytes_read,
        "total_size": total_size,
        "key": key,
        "family_id": family_id,
        "encoding": "base64"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_config::config::storage_paths::get_storage_base_path;
    use serde_json::json;

    async fn mock_state(family_id: Option<&str>) -> StorageState {
        StorageState {
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
            family_id: family_id.map(String::from),
            storage_initialized: true,
            encryption: None,
        }
    }

    #[tokio::test]
    async fn storage_blob_round_trip() {
        let state = mock_state(Some("test-blob")).await;
        let family_id = format!("test-blob-{}", uuid::Uuid::new_v4());
        let raw_data = b"binary payload \x00\xff\xfe";
        let encoded = base64::engine::general_purpose::STANDARD.encode(raw_data);

        let store_p = json!({"family_id": &family_id, "key": "binfile", "blob": encoded});
        let store_result = storage_store_blob(Some(&store_p), &state).await;
        assert!(store_result.is_ok(), "blob store: {store_result:?}");

        let retrieve_p = json!({"family_id": &family_id, "key": "binfile"});
        let retrieve_result = storage_retrieve_blob(Some(&retrieve_p), &state).await;
        assert!(retrieve_result.is_ok());
        let blob_b64 = retrieve_result.unwrap()["blob"]
            .as_str()
            .unwrap()
            .to_string();
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&blob_b64)
            .unwrap();
        assert_eq!(decoded, raw_data);

        // Cleanup
        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn retrieve_range_requires_params() {
        let state = mock_state(Some("test")).await;
        assert!(storage_retrieve_range(None, &state).await.is_err());
    }

    #[tokio::test]
    async fn retrieve_range_requires_length() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"key": "x", "family_id": "test"}));
        assert!(
            storage_retrieve_range(params.as_ref(), &state)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn retrieve_range_returns_null_for_missing_key() {
        let state = mock_state(Some("test")).await;
        let params =
            Some(json!({"key": "nonexistent-key-12345", "family_id": "test", "length": 1024}));
        let result = storage_retrieve_range(params.as_ref(), &state)
            .await
            .unwrap();
        assert!(result["data"].is_null());
        assert_eq!(result["error"], "not_found");
    }

    #[tokio::test]
    async fn retrieve_range_reads_blob_chunks() {
        let state = mock_state(Some("test-range")).await;
        let family_id = format!("test-range-{}", uuid::Uuid::new_v4());
        let payload = vec![0xABu8; 8192];
        let encoded = base64::engine::general_purpose::STANDARD.encode(&payload);

        let store_p = json!({"family_id": &family_id, "key": "tensor", "blob": encoded});
        assert!(storage_store_blob(Some(&store_p), &state).await.is_ok());

        let chunk1 = json!({"family_id": &family_id, "key": "tensor", "offset": 0, "length": 4096});
        let r1 = storage_retrieve_range(Some(&chunk1), &state).await.unwrap();
        assert_eq!(r1["total_size"], 8192);
        assert_eq!(r1["length"], 4096);
        assert_eq!(r1["offset"], 0);
        assert_eq!(r1["encoding"], "base64");

        let decoded = base64::engine::general_purpose::STANDARD
            .decode(r1["data"].as_str().unwrap())
            .unwrap();
        assert_eq!(decoded.len(), 4096);
        assert!(decoded.iter().all(|&b| b == 0xAB));

        let chunk2 =
            json!({"family_id": &family_id, "key": "tensor", "offset": 4096, "length": 4096});
        let r2 = storage_retrieve_range(Some(&chunk2), &state).await.unwrap();
        assert_eq!(r2["length"], 4096);
        assert_eq!(r2["offset"], 4096);

        // Cleanup
        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }
}
