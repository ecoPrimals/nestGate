//! Storage JSON-RPC Handlers
//!
//! Extracted from unix_socket_server for domain-based refactoring.
//! Handles: storage.store, storage.retrieve, storage.exists, storage.delete,
//! storage.list, storage.stats, storage.store_blob, storage.retrieve_blob

use crate::config::storage_paths::get_storage_base_path;
use crate::error::{NestGateError, Result};
use serde_json::{json, Value};
use std::path::Path;
use tracing::{debug, error, info, warn};

use super::StorageState;

/// Resolve `family_id` from params, falling back to the server's socket-scoped family.
///
/// When callers connect via a family-scoped socket (`nestgate-{family}.sock`),
/// the server already knows the family context. This eliminates the #1 friction
/// point identified in primalSpring composition experiments (exp066/068).
pub(crate) fn resolve_family_id<'a>(params: &'a Value, state: &'a StorageState) -> Result<&'a str> {
    if let Some(fid) = params["family_id"].as_str() {
        return Ok(fid);
    }
    if let Some(ref fid) = state.family_id {
        return Ok(fid.as_str());
    }
    Err(NestGateError::invalid_input_with_field(
        "family_id",
        "family_id required (or connect via a family-scoped socket)",
    ))
}

/// storage.store - Store key-value data
pub(super) async fn storage_store(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;

    let data = if params.get("value").is_some() && !params["value"].is_null() {
        &params["value"]
    } else if params.get("data").is_some() && !params["data"].is_null() {
        &params["data"]
    } else {
        return Err(NestGateError::invalid_input_with_field(
            "value",
            "value or data (json) required",
        ));
    };

    let family_id = resolve_family_id(params, state)?;

    // ✅ ENHANCED LOGGING: Input validation
    let data_str = serde_json::to_string(data).unwrap_or_else(|_| "<invalid>".to_string());
    debug!(
        "📝 storage.store called: family_id='{}', key='{}', value_size={} bytes",
        family_id,
        key,
        data_str.len()
    );

    // ✅ PERSISTENT: Store via StorageManagerService (filesystem-backed)
    let dataset = family_id; // Family maps to dataset
    let object_id = key;

    // Serialize JSON data to bytes
    let data_bytes = serde_json::to_vec(data)
        .map_err(|e| NestGateError::storage_error(&format!("Failed to serialize data: {}", e)))?;

    // ✅ ENHANCED LOGGING: Before storage call
    debug!(
        "💾 Calling storage_manager.store_object: dataset='{}', key='{}', bytes={}",
        dataset,
        object_id,
        data_bytes.len()
    );

    // Store via persistent backend
    let object_info = state
        .storage_manager
        .store_object(dataset, object_id, data_bytes)
        .await?;

    // ✅ ENHANCED LOGGING: Success with details
    info!(
        "✅ storage.store SUCCESS: {}/{} ({} bytes stored)",
        family_id, key, object_info.size_bytes
    );

    Ok(json!({
        "success": true,
        "key": key,
        "family_id": family_id,
        "size_bytes": object_info.size_bytes
    }))
}

/// storage.retrieve - Retrieve data by key
pub(super) async fn storage_retrieve(
    params: &Option<Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;

    debug!(
        "📖 storage.retrieve called: family_id='{}', key='{}'",
        family_id, key
    );

    // ✅ PERSISTENT: Retrieve from StorageManagerService (filesystem-backed)
    let dataset = family_id;
    let object_id = key;

    // ✅ ENHANCED LOGGING: Before storage call
    debug!(
        "🔍 Calling storage_manager.retrieve_object: dataset='{}', key='{}'",
        dataset, object_id
    );

    // Retrieve from persistent backend (returns (Vec<u8>, ObjectInfo))
    let (data_bytes, _info) = state
        .storage_manager
        .retrieve_object(dataset, object_id)
        .await?;

    // ✅ ENHANCED LOGGING: Retrieved bytes
    debug!(
        "📦 Retrieved raw bytes: {} bytes for {}/{}",
        data_bytes.len(),
        family_id,
        key
    );

    // ✅ ENHANCED LOGGING: Before deserialization
    debug!("🔄 Deserializing {} bytes as JSON...", data_bytes.len());

    // Deserialize bytes to JSON (data_bytes is Bytes - use .as_ref() for zero-copy)
    let data: Value = serde_json::from_slice(data_bytes.as_ref()).map_err(|e| {
        error!("❌ DESERIALIZATION FAILED for {}/{}: {}", family_id, key, e);
        NestGateError::storage_error(&format!("Failed to deserialize data: {}", e))
    })?;

    // ✅ ENHANCED LOGGING: Success
    info!(
        "✅ storage.retrieve SUCCESS: {}/{} → {} bytes JSON",
        family_id,
        key,
        serde_json::to_string(&data).unwrap_or_default().len()
    );

    // ✅ BUG 2 FIX: Return both "value" (biomeOS convention) and "data" (legacy)
    // biomeOS upstream expects {"value": ...}, legacy clients expect {"data": ...}
    // Returning both ensures universal compatibility
    Ok(json!({
        "value": data,
        "data": data,
        "key": key,
        "family_id": family_id
    }))
}

/// storage.exists - Check if data exists by key
///
/// Modern idiomatic Rust: Efficient existence check without data transfer
/// Deep Debt Principle #1: Standard API pattern, no unnecessary data retrieval
pub(super) async fn storage_exists(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;
    let dataset = family_id;
    let object_id = key;

    // Check existence via retrieve (returns error if not found)
    // ✅ DEEP DEBT: Proper Result propagation, no unwraps
    let exists = match state
        .storage_manager
        .retrieve_object(dataset, object_id)
        .await
    {
        Ok(_) => true,
        Err(e) => {
            // Distinguish "not found" from actual errors
            if e.to_string().contains("not found") || e.to_string().contains("Not found") {
                false
            } else {
                // Propagate actual errors
                return Err(e);
            }
        }
    };

    debug!(
        "🔍 Existence check: key='{}', family='{}', exists={}",
        key, family_id, exists
    );

    Ok(json!({
        "exists": exists,
        "key": key,
        "family_id": family_id
    }))
}

/// storage.delete - Delete data by key
pub(super) async fn storage_delete(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;
    let dataset = family_id;
    let object_id = key;

    // Delete from persistent backend
    let result = state
        .storage_manager
        .delete_object(dataset, object_id)
        .await;

    let deleted = result.is_ok();

    if deleted {
        debug!(
            "✅ Deleted key '{}' for family '{}' (persistent)",
            key, family_id
        );
    } else {
        warn!(
            "Key '{}' not found for deletion (family: '{}')",
            key, family_id
        );
    }

    Ok(json!({
        "success": deleted
    }))
}

/// storage.list - List all keys with optional prefix
pub(super) async fn storage_list(params: &Option<Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = resolve_family_id(params, _state)?;
    let prefix = params["prefix"].as_str();

    let dataset = family_id;

    // Scan the dataset directory — aligned with store_object's write path
    // which writes to .../datasets/{dataset}/{key} (no "objects/" segment).
    let dataset_path = crate::config::storage_paths::get_storage_base_path()
        .join("datasets")
        .join(dataset);

    let keys = list_keys_recursive(&dataset_path, &dataset_path, prefix).await;

    debug!(
        "Listed {} keys for family '{}' (prefix: {:?})",
        keys.len(),
        family_id,
        prefix
    );

    Ok(json!({
        "keys": keys
    }))
}

/// storage.stats - Get storage statistics
pub(super) async fn storage_stats(params: &Option<Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = resolve_family_id(params, _state)?;

    let dataset = family_id;

    let dataset_path = get_storage_base_path().join("datasets").join(dataset);

    let keys = list_keys_recursive(&dataset_path, &dataset_path, None).await;
    let key_count = keys.len();

    debug!("Stats for family '{}': {} objects", family_id, key_count);

    Ok(json!({
        "key_count": key_count,
        "blob_count": 0,
        "family_id": family_id
    }))
}

/// storage.store_blob - Store binary blob (base64 encoded)
pub(super) async fn storage_store_blob(
    params: &Option<Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let blob_base64 = params["blob"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("blob", "blob (base64 string) required")
    })?;
    let family_id = resolve_family_id(params, state)?;

    // Decode base64
    use base64::Engine;
    let blob_data = base64::engine::general_purpose::STANDARD
        .decode(blob_base64)
        .map_err(|e| {
            NestGateError::invalid_input_with_field("blob", format!("Invalid base64: {}", e))
        })?;

    // ✅ PERSISTENT: Store blob via StorageManagerService
    // No clone: store_object accepts impl AsRef<[u8]>, pass blob_data directly
    let dataset = family_id;
    let object_id = key;

    state
        .storage_manager
        .store_object(dataset, object_id, &blob_data)
        .await?;

    debug!(
        "✅ Stored blob '{}' ({} bytes) for family '{}' (persistent)",
        key,
        blob_data.len(),
        family_id
    );

    Ok(json!({
        "success": true,
        "key": key,
        "size": blob_data.len()
    }))
}

/// storage.retrieve_blob - Retrieve binary blob (base64 encoded)
pub(super) async fn storage_retrieve_blob(
    params: &Option<Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;
    let dataset = family_id;
    let object_id = key;

    // Retrieve raw bytes (returns (Vec<u8>, ObjectInfo))
    let (blob_data, _info) = state
        .storage_manager
        .retrieve_object(dataset, object_id)
        .await?;

    // Encode as base64 (blob_data is Bytes - use .as_ref() for zero-copy)
    use base64::Engine;
    let blob_base64 = base64::engine::general_purpose::STANDARD.encode(blob_data.as_ref());

    debug!(
        "✅ Retrieved blob '{}' ({} bytes) for family '{}' (persistent)",
        key,
        blob_data.len(),
        family_id
    );

    Ok(json!({
        "blob": blob_base64,
        "size": blob_data.len()
    }))
}

/// Recursively list all file keys under a dataset directory.
///
/// Keys are returned as relative paths from `root` (e.g. `test/primalspring/hello`
/// for a file at `{root}/test/primalspring/hello`). Directories are traversed but
/// not returned as keys themselves — only files are keys.
fn list_keys_recursive<'a>(
    dir: &'a Path,
    root: &'a Path,
    prefix: Option<&'a str>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<String>> + Send + 'a>> {
    Box::pin(async move {
        let mut keys = Vec::new();
        let Ok(mut entries) = tokio::fs::read_dir(dir).await else {
            return keys;
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.is_dir() {
                keys.extend(list_keys_recursive(&path, root, prefix).await);
            } else if let Ok(relative) = path.strip_prefix(root) {
                let key = relative.to_string_lossy().to_string();
                if let Some(p) = prefix {
                    if key.starts_with(p) {
                        keys.push(key);
                    }
                } else {
                    keys.push(key);
                }
            }
        }
        keys
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    async fn mock_state(family_id: Option<&str>) -> StorageState {
        StorageState {
            storage_manager: std::sync::Arc::new(
                crate::services::storage::StorageManagerService::new()
                    .await
                    .expect("test storage manager"),
            ),
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
            family_id: family_id.map(String::from),
        }
    }

    #[tokio::test]
    async fn resolve_family_id_from_params() {
        let state = mock_state(Some("server-family")).await;
        let params = json!({"family_id": "explicit-family"});
        let result = resolve_family_id(&params, &state).unwrap();
        assert_eq!(result, "explicit-family");
    }

    #[tokio::test]
    async fn resolve_family_id_falls_back_to_state() {
        let state = mock_state(Some("server-family")).await;
        let params = json!({"key": "some-key"});
        let result = resolve_family_id(&params, &state).unwrap();
        assert_eq!(result, "server-family");
    }

    #[tokio::test]
    async fn resolve_family_id_errors_when_missing() {
        let state = mock_state(None).await;
        let params = json!({"key": "some-key"});
        let result = resolve_family_id(&params, &state);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_family_id_param_overrides_state() {
        let state = mock_state(Some("default")).await;
        let params = json!({"family_id": "override"});
        let result = resolve_family_id(&params, &state).unwrap();
        assert_eq!(result, "override");
    }

    #[tokio::test]
    async fn storage_store_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_store(&None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_store_requires_key() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"family_id": "test", "dataset": "ds"}));
        let result = storage_store(&params, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_retrieve_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_retrieve(&None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_exists_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_exists(&None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_delete_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_delete(&None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_list_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_list(&None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_store_and_retrieve_round_trip() {
        let state = mock_state(Some("test-rt")).await;
        let dataset = format!("test-dataset-{}", uuid::Uuid::new_v4());

        let store_params = Some(json!({
            "dataset": dataset,
            "key": "hello",
            "value": "world"
        }));
        let store_result = storage_store(&store_params, &state).await;
        assert!(
            store_result.is_ok(),
            "store failed: {:?}",
            store_result.err()
        );

        let retrieve_params = Some(json!({
            "dataset": dataset,
            "key": "hello"
        }));
        let retrieve_result = storage_retrieve(&retrieve_params, &state).await;
        assert!(retrieve_result.is_ok());

        let data = retrieve_result.unwrap();
        assert_eq!(data["value"], "world");

        // Cleanup
        let _ = storage_delete(&Some(json!({"dataset": dataset, "key": "hello"})), &state).await;
    }

    #[tokio::test]
    async fn storage_list_returns_stored_keys() {
        let state = mock_state(Some("test-list")).await;
        let dataset = format!("test-list-{}", uuid::Uuid::new_v4());

        let _ = storage_store(
            &Some(json!({"dataset": &dataset, "key": "a", "value": "1"})),
            &state,
        )
        .await;
        let _ = storage_store(
            &Some(json!({"dataset": &dataset, "key": "b", "value": "2"})),
            &state,
        )
        .await;

        let list_result = storage_list(&Some(json!({"dataset": &dataset})), &state).await;
        assert!(list_result.is_ok());
        let keys = list_result.unwrap();
        let key_arr = keys["keys"].as_array().expect("keys array");
        assert!(key_arr.len() >= 2);

        // Cleanup
        let _ = storage_delete(&Some(json!({"dataset": &dataset, "key": "a"})), &state).await;
        let _ = storage_delete(&Some(json!({"dataset": &dataset, "key": "b"})), &state).await;
    }

    #[tokio::test]
    async fn storage_nested_key_paths_work() {
        let state = mock_state(Some("test-nested")).await;
        let dataset = format!("test-nested-{}", uuid::Uuid::new_v4());

        let store_result = storage_store(
            &Some(json!({"dataset": &dataset, "key": "deep/path/key", "value": "nested"})),
            &state,
        )
        .await;
        assert!(
            store_result.is_ok(),
            "nested store: {:?}",
            store_result.err()
        );

        let retrieve_result = storage_retrieve(
            &Some(json!({"dataset": &dataset, "key": "deep/path/key"})),
            &state,
        )
        .await;
        assert!(retrieve_result.is_ok());
        assert_eq!(retrieve_result.unwrap()["value"], "nested");

        // Cleanup
        let _ = storage_delete(
            &Some(json!({"dataset": &dataset, "key": "deep/path/key"})),
            &state,
        )
        .await;
    }
}
