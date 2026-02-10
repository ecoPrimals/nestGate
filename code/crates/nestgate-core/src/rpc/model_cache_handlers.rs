//! Model Cache JSON-RPC Handlers
//!
//! ✅ SMART REFACTORING: Extracted from unix_socket_server.rs (1400+ lines)
//! Separated by domain responsibility: model cache operations
//!
//! ✅ DEEP DEBT PRINCIPLE #3: Smart File Refactoring
//! - Logical cohesion: All model cache methods in one module
//! - Not just split: Domain-specific extraction
//! - Shared types via super::StorageState
//!
//! ## Methods
//! - `model.register` - Register a model in the mesh
//! - `model.exists` - Check if model is cached
//! - `model.locate` - Find gates with model cached
//! - `model.metadata` - Get model registration info

use crate::error::{NestGateError, Result};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{debug, info};

/// Model key prefix for namespace isolation
const MODEL_KEY_PREFIX: &str = "model:";
/// Model metadata key prefix
const MODEL_META_PREFIX: &str = "model_meta:";

/// model.register - Register a model in the mesh with metadata
///
/// Stores model metadata for zero-download model sharing across gates.
/// Uses storage backend with model-specific key prefix for namespace isolation.
pub(crate) async fn model_register(
    params: &Option<Value>,
    storage_manager: &Arc<crate::services::storage::StorageManagerService>,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let model_id = params["model_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("model_id", "model_id (string) required")
    })?;

    let metadata = if let Some(meta) = params.get("metadata") {
        meta.clone()
    } else {
        json!({})
    };

    // Build model registration record
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("NESTGATE_NODE_ID"))
        .unwrap_or_else(|_| "unknown".to_string());

    let registration = json!({
        "model_id": model_id,
        "metadata": metadata,
        "registered_at": chrono_timestamp(),
        "gate": hostname,
        "status": "available"
    });

    // Store via storage backend with model: prefix
    let storage_key = format!("{MODEL_KEY_PREFIX}{model_id}");
    let family_id = params["family_id"].as_str().unwrap_or("models");

    let data_bytes = serde_json::to_vec(&registration).map_err(|e| {
        NestGateError::storage_error(&format!("Failed to serialize model data: {}", e))
    })?;

    let object_info = storage_manager
        .store_object(family_id, &storage_key, data_bytes)
        .await?;

    // If metadata provided, store separately for efficient metadata-only queries
    if !metadata.is_null() && metadata != json!({}) {
        let meta_key = format!("{MODEL_META_PREFIX}{model_id}");
        let meta_bytes = serde_json::to_vec(&metadata).map_err(|e| {
            NestGateError::storage_error(&format!("Failed to serialize metadata: {}", e))
        })?;
        // Best-effort metadata storage (don't fail registration if this fails)
        let _ = storage_manager
            .store_object(family_id, &meta_key, meta_bytes)
            .await;
    }

    info!(
        "✅ model.register SUCCESS: model_id='{}', gate='{}', size={} bytes",
        model_id, hostname, object_info.size_bytes
    );

    Ok(json!({
        "registered": true,
        "model_id": model_id,
        "gate": hostname,
        "size_bytes": object_info.size_bytes
    }))
}

/// model.exists - Check if a model is cached locally or on any gate
///
/// Efficient existence check without full data transfer.
pub(crate) async fn model_exists(
    params: &Option<Value>,
    storage_manager: &Arc<crate::services::storage::StorageManagerService>,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let model_id = params["model_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("model_id", "model_id (string) required")
    })?;

    let storage_key = format!("{MODEL_KEY_PREFIX}{model_id}");
    let family_id = params["family_id"].as_str().unwrap_or("models");

    let exists = match storage_manager
        .retrieve_object(family_id, &storage_key)
        .await
    {
        Ok(_) => true,
        Err(e) => {
            if e.to_string().contains("not found") || e.to_string().contains("Not found") {
                false
            } else {
                return Err(e);
            }
        }
    };

    debug!(
        "🔍 model.exists: model_id='{}', exists={}",
        model_id, exists
    );

    Ok(json!({
        "exists": exists,
        "model_id": model_id,
        "family_id": family_id
    }))
}

/// model.locate - Return gate(s) that have this model cached
///
/// Currently returns local gate info. Future: cross-gate mesh query via Songbird.
pub(crate) async fn model_locate(
    params: &Option<Value>,
    storage_manager: &Arc<crate::services::storage::StorageManagerService>,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let model_id = params["model_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("model_id", "model_id (string) required")
    })?;

    let storage_key = format!("{MODEL_KEY_PREFIX}{model_id}");
    let family_id = params["family_id"].as_str().unwrap_or("models");

    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("NESTGATE_NODE_ID"))
        .unwrap_or_else(|_| "unknown".to_string());

    // Check if model exists locally
    let local_exists = (storage_manager
        .retrieve_object(family_id, &storage_key)
        .await)
        .is_ok();

    let mut gates = Vec::new();
    if local_exists {
        gates.push(json!({
            "gate": hostname,
            "status": "available",
            "local": true
        }));
    }

    // TODO: Future evolution - query Songbird mesh for remote gates
    // This will use cross-gate replication when implemented
    debug!(
        "🔍 model.locate: model_id='{}', gates={}",
        model_id,
        gates.len()
    );

    Ok(json!({
        "model_id": model_id,
        "gates": gates,
        "total_gates": gates.len()
    }))
}

/// model.metadata - Return model size, format, hash, and registration info
///
/// Retrieves the full model registration record including metadata.
pub(crate) async fn model_metadata(
    params: &Option<Value>,
    storage_manager: &Arc<crate::services::storage::StorageManagerService>,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let model_id = params["model_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("model_id", "model_id (string) required")
    })?;

    let storage_key = format!("{MODEL_KEY_PREFIX}{model_id}");
    let family_id = params["family_id"].as_str().unwrap_or("models");

    // Retrieve model registration record
    let (data_bytes, info) = storage_manager
        .retrieve_object(family_id, &storage_key)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") || e.to_string().contains("Not found") {
                NestGateError::not_found(format!("model '{}'", model_id))
            } else {
                e
            }
        })?;

    // Deserialize model registration (data_bytes is Bytes - use .as_ref())
    let registration: Value = serde_json::from_slice(data_bytes.as_ref()).map_err(|e| {
        NestGateError::storage_error(&format!("Failed to deserialize model data: {}", e))
    })?;

    debug!(
        "📋 model.metadata: model_id='{}', size={} bytes",
        model_id, info.size_bytes
    );

    Ok(json!({
        "model_id": model_id,
        "size_bytes": info.size_bytes,
        "registration": registration,
        "checksum": info.checksum,
        "family_id": family_id
    }))
}

/// discover_capabilities - Return all available JSON-RPC methods
///
/// Deep Debt Principle #6: Primal Self-Knowledge
/// NestGate advertises its own capabilities for runtime discovery
pub(crate) async fn discover_capabilities() -> Result<Value> {
    info!("🔍 discover_capabilities called");

    Ok(json!({
        "primal": "nestgate",
        "version": env!("CARGO_PKG_VERSION"),
        "capabilities": [
            "health",
            "discover_capabilities",
            "storage.store",
            "storage.retrieve",
            "storage.exists",
            "storage.delete",
            "storage.list",
            "storage.stats",
            "storage.store_blob",
            "storage.retrieve_blob",
            "model.register",
            "model.exists",
            "model.locate",
            "model.metadata",
            "templates.store",
            "templates.retrieve",
            "templates.list",
            "templates.community_top",
            "audit.store_execution"
        ],
        "backend": {
            "type": if crate::services::storage::capabilities::is_zfs_available() { "zfs" } else { "filesystem" },
            "features": {
                "persistent": true,
                "blob_storage": true,
                "model_cache": true,
                "templates": true,
                "audit": true
            }
        }
    }))
}

/// Get current Unix timestamp (no chrono dependency)
fn chrono_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a `StorageManagerService` backed by a temporary directory.
    /// Avoids permission issues from system-level paths.
    async fn test_storage() -> Arc<crate::services::storage::StorageManagerService> {
        let tmp = std::env::temp_dir().join(format!("nestgate-test-{}", fastrand::u64(..)));
        std::fs::create_dir_all(&tmp).expect("create temp dir");

        let mut config = crate::services::storage::config::StorageServiceConfig::development();
        config.base_path = tmp.to_string_lossy().to_string();
        config.auto_discover_pools = false;
        config.enable_quotas = false;
        config.enable_caching = false;
        config.enable_monitoring = false;

        Arc::new(
            crate::services::storage::StorageManagerService::with_config(config)
                .await
                .expect("Storage init with temp dir"),
        )
    }

    #[tokio::test]
    async fn test_discover_capabilities_returns_valid_json() {
        let result = discover_capabilities().await;
        assert!(result.is_ok());
        let value = result.unwrap();

        // Verify structure
        assert_eq!(value["primal"], "nestgate");
        assert!(value["version"].is_string());
        assert!(value["capabilities"].is_array());
        assert!(value["backend"].is_object());

        // Verify key capabilities are listed
        let caps = value["capabilities"].as_array().unwrap();
        let cap_strings: Vec<&str> = caps.iter().filter_map(|v| v.as_str()).collect();
        assert!(cap_strings.contains(&"health"));
        assert!(cap_strings.contains(&"storage.store"));
        assert!(cap_strings.contains(&"storage.retrieve"));
        assert!(cap_strings.contains(&"model.register"));
        assert!(cap_strings.contains(&"model.exists"));
    }

    #[tokio::test]
    async fn test_discover_capabilities_has_backend_info() {
        let result = discover_capabilities().await.unwrap();
        let backend = &result["backend"];

        assert!(backend["type"].is_string());
        let backend_type = backend["type"].as_str().unwrap();
        assert!(backend_type == "zfs" || backend_type == "filesystem");

        assert!(backend["features"]["persistent"].as_bool().unwrap());
        assert!(backend["features"]["model_cache"].as_bool().unwrap());
    }

    #[test]
    fn test_chrono_timestamp_returns_numeric_string() {
        let ts = chrono_timestamp();
        assert!(!ts.is_empty());
        assert!(
            ts.parse::<u64>().is_ok(),
            "Timestamp should be parseable as u64"
        );
    }

    #[test]
    fn test_model_key_prefix_constants() {
        assert_eq!(MODEL_KEY_PREFIX, "model:");
        assert_eq!(MODEL_META_PREFIX, "model_meta:");
    }

    #[tokio::test]
    async fn test_model_register_missing_params() {
        let storage = test_storage().await;
        let result = model_register(&None, &storage).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_register_missing_model_id() {
        let storage = test_storage().await;
        let params = Some(json!({"format": "onnx"}));
        let result = model_register(&params, &storage).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_exists_missing_params() {
        let storage = test_storage().await;
        let result = model_exists(&None, &storage).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_locate_missing_params() {
        let storage = test_storage().await;
        let result = model_locate(&None, &storage).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_metadata_missing_params() {
        let storage = test_storage().await;
        let result = model_metadata(&None, &storage).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_register_and_exists_roundtrip() {
        let storage = test_storage().await;

        let model_id = format!("test-model-{}", fastrand::u64(..));

        // Register a model
        let params = Some(json!({
            "model_id": model_id,
            "format": "onnx",
            "size_bytes": 1024
        }));
        let result = model_register(&params, &storage).await;
        assert!(result.is_ok(), "Registration should succeed: {:?}", result);
        let reg = result.unwrap();
        assert_eq!(reg["model_id"], model_id);
        assert!(reg["registered"].as_bool().unwrap(), "Should be registered");

        // Check exists
        let exists_params = Some(json!({"model_id": model_id}));
        let exists_result = model_exists(&exists_params, &storage).await;
        assert!(exists_result.is_ok());
        let exists_val = exists_result.unwrap();
        assert!(
            exists_val["exists"].as_bool().unwrap(),
            "Model should exist after registration"
        );

        // Get metadata
        let meta_result = model_metadata(&exists_params, &storage).await;
        assert!(meta_result.is_ok());
        let meta = meta_result.unwrap();
        assert_eq!(meta["model_id"], model_id);
    }
}
