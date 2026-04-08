// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Model Cache JSON-RPC Handlers
//!
//! Filesystem-backed model metadata persistence. Models are stored as JSON
//! files under `{storage_base}/datasets/_models/{model_id}.json` with optional
//! model-level metadata at `{storage_base}/datasets/_model_meta/{model_id}.json`.

use nestgate_config::config::storage_paths::StoragePaths;
use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use nestgate_types::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv};
use serde_json::{Value, json};
use std::path::PathBuf;
use tracing::{debug, info};

const MODELS_DATASET: &str = "_models";

fn storage_base_path_for_models(env: &dyn EnvSource) -> PathBuf {
    if let Some(p) = env.get("NESTGATE_STORAGE_BASE_PATH") {
        return PathBuf::from(p);
    }
    StoragePaths::from_env_source(env).storage_base_path()
}

fn model_path_from_env(env: &dyn EnvSource, model_id: &str) -> PathBuf {
    storage_base_path_for_models(env)
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
    model_register_from_env_source(&ProcessEnv, params).await
}

/// Like [`model_register`], but resolves storage paths from an injectable [`EnvSource`].
pub async fn model_register_from_env_source(
    env: &dyn EnvSource,
    params: Option<&Value>,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let model_id = params["model_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("model_id", "model_id (string) required")
    })?;

    let path = model_path_from_env(env, model_id);
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
    model_exists_from_env_source(&ProcessEnv, params)
}

/// Like [`model_exists`], but resolves storage paths from an injectable [`EnvSource`].
pub fn model_exists_from_env_source(env: &dyn EnvSource, params: Option<&Value>) -> Result<Value> {
    let model_id = require_model_id(params)?;
    let exists = model_path_from_env(env, model_id).exists();
    debug!(model_id, exists, "model.exists");
    Ok(json!({ "model_id": model_id, "exists": exists }))
}

/// `model.locate` — return the filesystem path of a registered model.
pub fn model_locate(params: Option<&Value>) -> Result<Value> {
    model_locate_from_env_source(&ProcessEnv, params)
}

/// Like [`model_locate`], but resolves storage paths from an injectable [`EnvSource`].
pub fn model_locate_from_env_source(env: &dyn EnvSource, params: Option<&Value>) -> Result<Value> {
    let model_id = require_model_id(params)?;
    let path = model_path_from_env(env, model_id);

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
    model_metadata_from_env_source(&ProcessEnv, params).await
}

/// Like [`model_metadata`], but resolves storage paths from an injectable [`EnvSource`].
pub async fn model_metadata_from_env_source(
    env: &dyn EnvSource,
    params: Option<&Value>,
) -> Result<Value> {
    let model_id = require_model_id(params)?;
    let path = model_path_from_env(env, model_id);

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
    // Health triad (Wire Standard L1)
    "health.liveness",
    "health.readiness",
    "health.check",
    "health",
    // Meta / self-advertisement (Wire Standard L2)
    "identity.get",
    "capabilities.list",
    "discover_capabilities",
    "discovery.capability.register",
    // Storage operations (primary domain)
    "storage.store",
    "storage.retrieve",
    "storage.exists",
    "storage.delete",
    "storage.list",
    "storage.stats",
    "storage.store_blob",
    "storage.retrieve_blob",
    "storage.fetch_external",
    // Model cache
    "model.register",
    "model.exists",
    "model.locate",
    "model.metadata",
    // Template persistence
    "templates.store",
    "templates.retrieve",
    "templates.list",
    "templates.community_top",
    // Session persistence
    "session.save",
    "session.load",
    // Audit persistence
    "audit.store_execution",
    // NAT traversal persistence
    "nat.store_traversal_info",
    "nat.retrieve_traversal_info",
    // Beacon persistence
    "beacon.store",
    "beacon.retrieve",
    "beacon.list",
    "beacon.delete",
    // External data feeds
    "data.ncbi_search",
    "data.ncbi_fetch",
    "data.noaa_ghcnd",
    "data.iris_stations",
    "data.iris_events",
    // ZFS storage management
    "zfs.pool.list",
    "zfs.pool.get",
    "zfs.pool.health",
    "zfs.dataset.list",
    "zfs.dataset.get",
    "zfs.snapshot.list",
    "zfs.health",
];

/// capabilities.list — Wire Standard L3 compliant response.
///
/// Returns the required `{primal, version, methods}` envelope (L2) plus
/// structured `provided_capabilities` grouping and `consumed_capabilities`
/// declaration for composition completeness validation (L3).
#[expect(clippy::unnecessary_wraps, reason = "Handler dispatch requires Result")]
pub fn capabilities_list() -> Result<Value> {
    info!("🔍 capabilities.list called");
    Ok(json!({
        "primal": DEFAULT_SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION"),
        "methods": UNIX_SOCKET_SUPPORTED_METHODS,
        "provided_capabilities": [
            {
                "type": "storage",
                "methods": [
                    "store", "retrieve", "exists", "delete", "list",
                    "stats", "store_blob", "retrieve_blob", "fetch_external"
                ],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "Filesystem-backed durable key-value and blob storage"
            },
            {
                "type": "model",
                "methods": ["register", "exists", "locate", "metadata"],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "AI model cache — registration, lookup, metadata"
            },
            {
                "type": "templates",
                "methods": ["store", "retrieve", "list", "community_top"],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "Template persistence and community ranking"
            },
            {
                "type": "session",
                "methods": ["save", "load"],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "Game session persistence (convenience over storage.*)"
            },
            {
                "type": "audit",
                "methods": ["store_execution"],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "Execution audit trail persistence"
            },
            {
                "type": "nat",
                "methods": ["store_traversal_info", "retrieve_traversal_info"],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "NAT traversal endpoint persistence"
            },
            {
                "type": "beacon",
                "methods": ["store", "retrieve", "list", "delete"],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "Known beacon persistence for mesh discovery"
            },
            {
                "type": "data",
                "methods": ["ncbi_search", "ncbi_fetch", "noaa_ghcnd", "iris_stations", "iris_events"],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "External data feed proxying (NCBI, NOAA, IRIS)"
            },
            {
                "type": "zfs",
                "methods": ["pool.list", "pool.get", "pool.health", "dataset.list", "dataset.get", "snapshot.list", "health"],
                "version": env!("CARGO_PKG_VERSION"),
                "description": "ZFS storage management — pools, datasets, snapshots"
            }
        ],
        "consumed_capabilities": [],
        "protocol": "jsonrpc-2.0",
        "transport": ["uds", "http"]
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
                "fetch_external": true,
                "model_cache": true,
                "templates": true,
                "audit": true,
                "nat_traversal": true,
                "beacon_persistence": true,
                "zfs": true
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

    #[test]
    fn test_capabilities_list_wire_standard_l2_envelope() {
        let value = capabilities_list().unwrap();
        assert!(value["primal"].is_string(), "L2: primal field required");
        assert!(value["version"].is_string(), "L2: version field required");
        assert!(
            value["methods"].is_array(),
            "L2: methods flat array required"
        );
    }

    #[test]
    fn test_capabilities_list_wire_standard_l3_composable() {
        let value = capabilities_list().unwrap();
        let provided = value["provided_capabilities"].as_array().unwrap();
        assert!(!provided.is_empty(), "L3: provided_capabilities required");
        for group in provided {
            assert!(group["type"].is_string(), "L3: group type required");
            assert!(group["methods"].is_array(), "L3: group methods required");
        }
        assert!(
            value["consumed_capabilities"].is_array(),
            "L3: consumed_capabilities required"
        );
    }

    #[test]
    fn test_capabilities_list_protocol_and_transport() {
        let value = capabilities_list().unwrap();
        assert_eq!(value["protocol"], "jsonrpc-2.0");
        let transports = value["transport"].as_array().unwrap();
        let ts: Vec<&str> = transports.iter().filter_map(|v| v.as_str()).collect();
        assert!(ts.contains(&"uds"));
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
        use nestgate_types::MapEnv;

        let tmp = tempfile::tempdir().unwrap();
        let base = tmp.path().to_str().unwrap();
        let env = MapEnv::from([("NESTGATE_STORAGE_BASE_PATH", base)]);
        let params = json!({"model_id": "test-model", "name": "Test Model"});
        let reg = model_register_from_env_source(&env, Some(&params))
            .await
            .unwrap();
        assert!(reg["registered"].as_bool().unwrap());

        let exists = model_exists_from_env_source(&env, Some(&params)).unwrap();
        assert!(exists["exists"].as_bool().unwrap());

        let meta = model_metadata_from_env_source(&env, Some(&params))
            .await
            .unwrap();
        assert_eq!(meta["model_id"], "test-model");
        assert_eq!(meta["name"], "Test Model");

        let loc = model_locate_from_env_source(&env, Some(&params)).unwrap();
        assert!(loc["path"].as_str().unwrap().contains("test-model"));
    }

    #[tokio::test]
    async fn test_model_exists_returns_false_for_missing() {
        use nestgate_types::MapEnv;

        let tmp = tempfile::tempdir().unwrap();
        let base = tmp.path().to_str().unwrap();
        let env = MapEnv::from([("NESTGATE_STORAGE_BASE_PATH", base)]);
        let params = json!({"model_id": "nonexistent"});
        let result: Result<Value> = model_exists_from_env_source(&env, Some(&params));
        assert!(!result.unwrap()["exists"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn test_model_locate_missing_returns_not_found() {
        use nestgate_types::MapEnv;

        let tmp = tempfile::tempdir().unwrap();
        let base = tmp.path().to_str().unwrap();
        let env = MapEnv::from([("NESTGATE_STORAGE_BASE_PATH", base)]);
        let params = json!({"model_id": "missing"});
        let result: Result<Value> = model_locate_from_env_source(&env, Some(&params));
        assert!(result.is_err());
    }
}
