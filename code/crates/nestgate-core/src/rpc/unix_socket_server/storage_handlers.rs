//! Storage JSON-RPC Handlers
//!
//! Extracted from unix_socket_server for domain-based refactoring.
//! Handles: storage.store, storage.retrieve, storage.exists, storage.delete,
//! storage.list, storage.stats, storage.store_blob, storage.retrieve_blob

use crate::config::storage_paths::get_storage_base_path;
use crate::error::{NestGateError, Result};
use serde_json::{json, Value};
use tracing::{debug, error, info, warn};

use super::StorageState;

/// storage.store - Store key-value data
pub(super) async fn storage_store(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;

    // ✅ FIX: Accept both "value" (biomeOS) and "data" (legacy) parameters
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

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

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
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ ENHANCED LOGGING: Input validation
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
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ MODERN IDIOMATIC: Efficient check without full data retrieval
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
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ PERSISTENT: Delete from StorageManagerService (filesystem-backed)
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

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;
    let prefix = params["prefix"].as_str();

    // ✅ PERSISTENT: List from filesystem (StorageManagerService doesn't have list_objects yet)
    let dataset = family_id;

    // Read directly from storage filesystem
    // ✅ EVOLVED: Use XDG-compliant storage path (Phase 4)
    let base_path = crate::config::storage_paths::get_storage_base_path()
        .join("datasets")
        .join(dataset)
        .join("objects");

    let mut keys: Vec<String> = Vec::new();
    if base_path.exists() {
        if let Ok(mut entries) = tokio::fs::read_dir(&base_path).await {
            // Read all entries
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(file_name) = entry.file_name().to_str() {
                    // Filter by prefix if provided
                    if let Some(p) = prefix {
                        if file_name.starts_with(p) {
                            keys.push(file_name.to_string());
                        }
                    } else {
                        keys.push(file_name.to_string());
                    }
                }
            }
        }
    }

    debug!(
        "✅ Listed {} keys for family '{}' (prefix: {:?}) (persistent)",
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

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ PERSISTENT: Get stats from filesystem (XDG-compliant path)
    let dataset = family_id;

    // Count objects by reading directory (uses get_storage_base_path() - configurable)
    let base_path = get_storage_base_path()
        .join("datasets")
        .join(dataset)
        .join("objects");

    let key_count = if base_path.exists() {
        if let Ok(mut entries) = tokio::fs::read_dir(&base_path).await {
            let mut count = 0;
            while let Ok(Some(_)) = entries.next_entry().await {
                count += 1;
            }
            count
        } else {
            0
        }
    } else {
        0
    };

    debug!(
        "✅ Stats for family '{}': {} objects (persistent)",
        family_id, key_count
    );

    Ok(json!({
        "key_count": key_count,
        "blob_count": 0,  // No separate blob tracking in new architecture
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
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

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
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // ✅ PERSISTENT: Retrieve blob from StorageManagerService
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
