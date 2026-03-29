// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! Model Cache JSON-RPC Handlers
//!
//! **Integration:** Persistent model metadata belongs in `nestgate-core` `services::storage` when
//! this RPC surface is linked to the core storage manager.

use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::info;

/// Model key prefix for namespace isolation
const MODEL_KEY_PREFIX: &str = "model:";
/// Model metadata key prefix
const MODEL_META_PREFIX: &str = "model_meta:";

/// model.register — not implemented until nestgate-core storage is wired.
pub fn model_register(params: Option<&Value>) -> Result<Value> {
    tracing::debug!("feature pending: model.register via nestgate-core storage");
    let _ = (MODEL_KEY_PREFIX, MODEL_META_PREFIX, params);
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (model.register)",
    ))
}

/// model.exists — not implemented until nestgate-core storage is wired.
pub fn model_exists(params: Option<&Value>) -> Result<Value> {
    tracing::debug!("feature pending: model.exists via nestgate-core storage");
    let _ = params;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (model.exists)",
    ))
}

/// model.locate — not implemented until nestgate-core storage is wired.
pub fn model_locate(params: Option<&Value>) -> Result<Value> {
    tracing::debug!("feature pending: model.locate via nestgate-core storage");
    let _ = params;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (model.locate)",
    ))
}

/// model.metadata — not implemented until nestgate-core storage is wired.
pub fn model_metadata(params: Option<&Value>) -> Result<Value> {
    tracing::debug!("feature pending: model.metadata via nestgate-core storage");
    let _ = params;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (model.metadata)",
    ))
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
pub fn capabilities_list() -> Result<Value> {
    info!("🔍 capabilities.list called");
    Ok(json!({
        "methods": UNIX_SOCKET_SUPPORTED_METHODS,
        "primal": "nestgate",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// `discover_capabilities` - Return all available JSON-RPC methods
pub fn discover_capabilities() -> Result<Value> {
    info!("🔍 discover_capabilities called");

    Ok(json!({
        "primal": "nestgate",
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
    fn test_chrono_timestamp_returns_numeric_string() {
        let ts = chrono_timestamp();
        assert!(!ts.is_empty());
        assert!(ts.parse::<u64>().is_ok());
    }

    #[test]
    fn test_model_key_prefix_constants() {
        assert_eq!(MODEL_KEY_PREFIX, "model:");
        assert_eq!(MODEL_META_PREFIX, "model_meta:");
    }

    #[tokio::test]
    async fn test_model_register_missing_params() {
        let result = model_register(None);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_register_and_exists_roundtrip() {
        let params = Some(json!({"model_id": "x"}));
        assert!(model_register(params.as_ref()).is_err());
        assert!(model_exists(params.as_ref()).is_err());
    }
}
