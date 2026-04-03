// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Model Cache JSON-RPC Handlers
//!
//! Filesystem-backed model metadata persistence. Models are stored as JSON
//! files under `{storage_base}/datasets/_models/{model_id}.json` with optional
//! model-level metadata at `{storage_base}/datasets/_model_meta/{model_id}.json`.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::PathBuf;
use tracing::{debug, info};

const MODELS_DATASET: &str = "_models";

fn model_path(model_id: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(MODELS_DATASET)
        .join(format!("{model_id}.json"))
}

fn require_model_id(params: Option<&Value>) -> Result<&str> {
    params.and_then(|p| p["model_id"].as_str()).ok_or_else(|| {
        NestGateError::invalid_input_with_field("model_id", "model_id (string) required")
    })
}

async fn ensure_dir(path: &std::path::Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            NestGateError::io_error(format!(
                "Failed to create directory {}: {e}",
                parent.display()
            ))
        })?;
    }
    Ok(())
}

/// `model.register` — persist model registration metadata to disk.
pub async fn model_register(params: Option<&Value>) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let model_id = params["model_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("model_id", "model_id (string) required")
    })?;

    let path = model_path(model_id);
    ensure_dir(&path).await?;

    let name_default = json!(model_id);
    let version_default = json!("1.0.0");
    let meta_default = json!({});
    let record = json!({
        "model_id": model_id,
        "name": params.get("name").unwrap_or(&name_default),
        "version": params.get("version").unwrap_or(&version_default),
        "registered_at": chrono::Utc::now().to_rfc3339(),
        "metadata": params.get("metadata").unwrap_or(&meta_default),
    });

    let data = serde_json::to_vec_pretty(&record)
        .map_err(|e| NestGateError::io_error(format!("Failed to serialize model record: {e}")))?;
    tokio::fs::write(&path, &data)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to write model file: {e}")))?;

    debug!(model_id, "model.register: persisted");

    Ok(json!({
        "model_id": model_id,
        "registered": true,
        "path": path.display().to_string(),
    }))
}

/// `model.exists` — check if a model registration exists on disk.
pub fn model_exists(params: Option<&Value>) -> Result<Value> {
    let model_id = require_model_id(params)?;
    let exists = model_path(model_id).exists();
    debug!(model_id, exists, "model.exists");
    Ok(json!({ "model_id": model_id, "exists": exists }))
}

/// `model.locate` — return the filesystem path of a registered model.
pub fn model_locate(params: Option<&Value>) -> Result<Value> {
    let model_id = require_model_id(params)?;
    let path = model_path(model_id);

    if !path.exists() {
        return Err(NestGateError::not_found(format!(
            "model '{model_id}' not registered"
        )));
    }

    debug!(model_id, "model.locate: found");
    Ok(json!({
        "model_id": model_id,
        "path": path.display().to_string(),
    }))
}

/// `model.metadata` — retrieve model registration metadata from disk.
pub async fn model_metadata(params: Option<&Value>) -> Result<Value> {
    let model_id = require_model_id(params)?;
    let path = model_path(model_id);

    let data = tokio::fs::read(&path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NestGateError::not_found(format!("model '{model_id}' not registered"))
        } else {
            NestGateError::io_error(format!("Failed to read model file: {e}"))
        }
    })?;

    let record: Value = serde_json::from_slice(&data)
        .map_err(|e| NestGateError::io_error(format!("Corrupted model record: {e}")))?;

    debug!(model_id, "model.metadata: loaded");
    Ok(record)
}

/// All JSON-RPC method names supported by the Unix socket server (`unix_socket_server`).
/// Keep in sync with `handle_request` in `unix_socket_server/mod.rs`.
pub const UNIX_SOCKET_SUPPORTED_METHODS: &[&str] = &[
    "health.liveness",
    "health.readiness",
    "health.check",
    "health",
    "capabilities.list",
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
    "audit.store_execution",
    "nat.store_traversal_info",
    "nat.retrieve_traversal_info",
    "beacon.store",
    "beacon.retrieve",
    "beacon.list",
    "beacon.delete",
];

/// capabilities.list — wateringHole semantic naming; lists all supported method names.
#[expect(clippy::unnecessary_wraps, reason = "Handler dispatch requires Result")]
pub fn capabilities_list() -> Result<Value> {
    info!("🔍 capabilities.list called");
    Ok(json!({
        "methods": UNIX_SOCKET_SUPPORTED_METHODS,
        "primal": DEFAULT_SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// `discover_capabilities` - Return all available JSON-RPC methods
#[expect(clippy::unnecessary_wraps, reason = "Handler dispatch requires Result")]
pub fn discover_capabilities() -> Result<Value> {
    info!("🔍 discover_capabilities called");

    Ok(json!({
        "primal": DEFAULT_SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION"),
        "capabilities": UNIX_SOCKET_SUPPORTED_METHODS,
        "backend": {
            "type": "filesystem",
            "features": {
                "persistent": true,
                "blob_storage": true,
                "model_cache": true,
                "templates": true,
                "audit": true,
                "nat_traversal": true,
                "beacon_persistence": true
            }
        }
    }))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_capabilities_list_matches_supported_methods() {
        let value = capabilities_list().unwrap();
        assert_eq!(value["primal"], "nestgate");
        assert_eq!(value["version"], env!("CARGO_PKG_VERSION"));
        let methods = value["methods"].as_array().unwrap();
        let names: Vec<&str> = methods.iter().filter_map(|v| v.as_str()).collect();
        assert_eq!(names, UNIX_SOCKET_SUPPORTED_METHODS);
    }

    #[tokio::test]
    async fn test_discover_capabilities_returns_valid_json() {
        let result = discover_capabilities();
        assert!(result.is_ok());
        let value = result.unwrap();

        assert_eq!(value["primal"], "nestgate");
        assert!(value["version"].is_string());
        assert!(value["capabilities"].is_array());
        assert!(value["backend"].is_object());

        let caps = value["capabilities"].as_array().unwrap();
        let cap_strings: Vec<&str> = caps.iter().filter_map(|v| v.as_str()).collect();
        assert!(cap_strings.contains(&"health.liveness"));
        assert!(cap_strings.contains(&"capabilities.list"));
        assert!(cap_strings.contains(&"health"));
        assert!(cap_strings.contains(&"storage.store"));
        assert!(cap_strings.contains(&"model.register"));
    }

    #[tokio::test]
    async fn test_discover_capabilities_has_backend_info() {
        let result = discover_capabilities().unwrap();
        let backend = &result["backend"];

        assert_eq!(backend["type"], "filesystem");
        assert!(backend["features"]["persistent"].as_bool().unwrap());
    }

    #[test]
    fn test_wall_clock_seconds_string_is_numeric() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let ts = format!("{secs}");
        assert!(!ts.is_empty());
        assert!(ts.parse::<u64>().is_ok());
    }

    #[test]
    fn test_model_dataset_constants() {
        assert_eq!(MODELS_DATASET, "_models");
    }

    #[tokio::test]
    async fn test_model_register_missing_params() {
        let result = model_register(None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_register_and_exists_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        temp_env::async_with_vars(
            [(
                "NESTGATE_STORAGE_BASE_PATH",
                Some(tmp.path().to_str().unwrap()),
            )],
            async {
                let params = json!({"model_id": "test-model", "name": "Test Model"});
                let reg = model_register(Some(&params)).await.unwrap();
                assert!(reg["registered"].as_bool().unwrap());

                let exists = model_exists(Some(&params)).unwrap();
                assert!(exists["exists"].as_bool().unwrap());

                let meta = model_metadata(Some(&params)).await.unwrap();
                assert_eq!(meta["model_id"], "test-model");
                assert_eq!(meta["name"], "Test Model");

                let loc = model_locate(Some(&params)).unwrap();
                assert!(loc["path"].as_str().unwrap().contains("test-model"));
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_model_exists_returns_false_for_missing() {
        let tmp = tempfile::tempdir().unwrap();
        temp_env::async_with_vars(
            [(
                "NESTGATE_STORAGE_BASE_PATH",
                Some(tmp.path().to_str().unwrap()),
            )],
            async {
                let params = json!({"model_id": "nonexistent"});
                let result: Result<Value> = model_exists(Some(&params));
                assert!(!result.unwrap()["exists"].as_bool().unwrap());
            },
        )
        .await;
    }

    #[tokio::test]
    async fn test_model_locate_missing_returns_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        temp_env::async_with_vars(
            [(
                "NESTGATE_STORAGE_BASE_PATH",
                Some(tmp.path().to_str().unwrap()),
            )],
            async {
                let params = json!({"model_id": "missing"});
                let result: Result<Value> = model_locate(Some(&params));
                assert!(result.is_err());
            },
        )
        .await;
    }
}
