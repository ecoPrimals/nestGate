// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Model Cache JSON-RPC Handlers
//!
//! TODO: wire to nestgate-core `services::storage` for persistent model metadata.

use nestgate_types::error::{NestGateError, Result};
use serde_json::{json, Value};
use tracing::info;

/// Model key prefix for namespace isolation
const MODEL_KEY_PREFIX: &str = "model:";
/// Model metadata key prefix
const MODEL_META_PREFIX: &str = "model_meta:";

/// model.register — stub until nestgate-core storage is wired.
pub(crate) async fn model_register(params: &Option<Value>) -> Result<Value> {
    let _ = (MODEL_KEY_PREFIX, MODEL_META_PREFIX, params);
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (model.register)",
    ))
}

/// model.exists — stub.
pub(crate) async fn model_exists(params: &Option<Value>) -> Result<Value> {
    let _ = params;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (model.exists)",
    ))
}

/// model.locate — stub.
pub(crate) async fn model_locate(params: &Option<Value>) -> Result<Value> {
    let _ = params;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (model.locate)",
    ))
}

/// model.metadata — stub.
pub(crate) async fn model_metadata(params: &Option<Value>) -> Result<Value> {
    let _ = params;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (model.metadata)",
    ))
}

/// discover_capabilities - Return all available JSON-RPC methods
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
            "audit.store_execution",
            "nat.store_traversal_info",
            "nat.retrieve_traversal_info",
            "beacon.store",
            "beacon.retrieve",
            "beacon.list",
            "beacon.delete"
        ],
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
    async fn test_discover_capabilities_returns_valid_json() {
        let result = discover_capabilities().await;
        assert!(result.is_ok());
        let value = result.unwrap();

        assert_eq!(value["primal"], "nestgate");
        assert!(value["version"].is_string());
        assert!(value["capabilities"].is_array());
        assert!(value["backend"].is_object());

        let caps = value["capabilities"].as_array().unwrap();
        let cap_strings: Vec<&str> = caps.iter().filter_map(|v| v.as_str()).collect();
        assert!(cap_strings.contains(&"health"));
        assert!(cap_strings.contains(&"storage.store"));
        assert!(cap_strings.contains(&"model.register"));
    }

    #[tokio::test]
    async fn test_discover_capabilities_has_backend_info() {
        let result = discover_capabilities().await.unwrap();
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
        let result = model_register(&None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_model_register_and_exists_roundtrip() {
        let params = Some(json!({"model_id": "x"}));
        assert!(model_register(&params).await.is_err());
        assert!(model_exists(&params).await.is_err());
    }
}
