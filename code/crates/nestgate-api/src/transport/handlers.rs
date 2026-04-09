// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "RPC handlers use Result for consistent error propagation to JSON-RPC clients"
)]

//! **RPC METHOD HANDLERS**
//!
//! JSON-RPC method implementations for `NestGate` storage and system operations.

use super::jsonrpc::RpcMethodHandler;
use nestgate_core::constants::system::DEFAULT_SERVICE_NAME;
use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;
use tracing::debug;

static PROCESS_START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

fn self_primal_name() -> String {
    std::env::var("NESTGATE_SERVICE_NAME").unwrap_or_else(|_| DEFAULT_SERVICE_NAME.to_string())
}

/// **NESTGATE RPC HANDLER**
///
/// Implements JSON-RPC methods for `NestGate` storage and system operations.
///
/// ## Method Namespace
///
/// - `storage.*` - Storage operations (store, retrieve, delete)
/// - `health.*` - Health and status checks
/// - `identity.*` - Primal identity and discovery
/// - `system.*` - System information and capabilities
#[derive(Clone)]
pub struct NestGateRpcHandler {
    /// Optional storage backend
    storage: Option<Arc<dyn StorageBackend>>,
}

impl NestGateRpcHandler {
    /// Create new RPC handler
    #[must_use]
    pub fn new() -> Self {
        Self { storage: None }
    }

    /// Create handler with storage backend
    #[must_use]
    pub fn with_storage(storage: Arc<dyn StorageBackend>) -> Self {
        Self {
            storage: Some(storage),
        }
    }
}

impl Default for NestGateRpcHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl RpcMethodHandler for NestGateRpcHandler {
    async fn handle_method(&self, method: &str, params: Value) -> Result<Value> {
        debug!("Handling RPC method: {}", method);

        match method {
            // Storage methods
            "storage.store" => self.handle_store(params).await,
            "storage.retrieve" => self.handle_retrieve(params).await,
            "storage.delete" => self.handle_delete(params).await,
            "storage.list" => self.handle_list(params).await,

            // Health methods
            "health.ping" => self.handle_ping(params),
            "health.status" => self.handle_status(params),

            // Identity methods
            "identity.get" => self.handle_identity(params),
            "identity.capabilities" => self.handle_capabilities(params),

            // System methods
            "system.info" => self.handle_system_info(params),

            // Unknown method
            _ => Err(NestGateError::api_error(format!(
                "Unknown method: {method}"
            ))),
        }
    }
}

impl NestGateRpcHandler {
    /// Handle storage.store request
    async fn handle_store(&self, params: Value) -> Result<Value> {
        let request: StoreRequest = serde_json::from_value(params)
            .map_err(|e| NestGateError::api_error(format!("Invalid params: {e}")))?;

        if let Some(storage) = &self.storage {
            storage.store(&request.key, &request.value).await?;
            Ok(serde_json::json!({"success": true, "key": request.key}))
        } else {
            Err(NestGateError::api_error("Storage backend not configured"))
        }
    }

    /// Handle storage.retrieve request
    async fn handle_retrieve(&self, params: Value) -> Result<Value> {
        let request: RetrieveRequest = serde_json::from_value(params)
            .map_err(|e| NestGateError::api_error(format!("Invalid params: {e}")))?;

        if let Some(storage) = &self.storage {
            let value = storage.retrieve(&request.key).await?;
            Ok(serde_json::json!({"key": request.key, "value": value}))
        } else {
            Err(NestGateError::api_error("Storage backend not configured"))
        }
    }

    /// Handle storage.delete request
    async fn handle_delete(&self, params: Value) -> Result<Value> {
        let request: DeleteRequest = serde_json::from_value(params)
            .map_err(|e| NestGateError::api_error(format!("Invalid params: {e}")))?;

        if let Some(storage) = &self.storage {
            storage.delete(&request.key).await?;
            Ok(serde_json::json!({"success": true, "key": request.key}))
        } else {
            Err(NestGateError::api_error("Storage backend not configured"))
        }
    }

    /// Handle storage.list request
    async fn handle_list(&self, params: Value) -> Result<Value> {
        let request: ListRequest = serde_json::from_value(params)
            .map_err(|e| NestGateError::api_error(format!("Invalid params: {e}")))?;

        if let Some(storage) = &self.storage {
            let keys = storage.list(&request.prefix).await?;
            Ok(serde_json::json!({"keys": keys}))
        } else {
            Err(NestGateError::api_error("Storage backend not configured"))
        }
    }

    /// Handle health.ping request
    fn handle_ping(&self, _params: Value) -> Result<Value> {
        Ok(serde_json::json!({"status": "pong", "timestamp": chrono::Utc::now().timestamp()}))
    }

    /// Handle health.status request
    fn handle_status(&self, _params: Value) -> Result<Value> {
        let primal = self_primal_name();
        Ok(serde_json::json!({
            "status": "healthy",
            "primal": primal,
            "transport": "unix-socket",
            "protocol": "jsonrpc-2.0",
            "timestamp": chrono::Utc::now().timestamp()
        }))
    }

    /// Handle `identity.get` request per `CAPABILITY_WIRE_STANDARD` L2.
    fn handle_identity(&self, _params: Value) -> Result<Value> {
        let family_id = std::env::var("BIOMEOS_FAMILY_ID")
            .or_else(|_| std::env::var("NESTGATE_FAMILY_ID"))
            .unwrap_or_else(|_| "default".to_string());
        let primal = self_primal_name();

        Ok(serde_json::json!({
            "primal": primal,
            "version": env!("CARGO_PKG_VERSION"),
            "domain": "storage",
            "license": "AGPL-3.0-or-later",
            "family_id": family_id
        }))
    }

    /// Handle identity.capabilities request
    fn handle_capabilities(&self, _params: Value) -> Result<Value> {
        Ok(serde_json::json!({
            "storage": self.storage.is_some(),
            "zfs": true,
            "performance_monitoring": true,
            "hardware_tuning": true,
            "transport": ["unix-socket", "http"],
            "protocol": ["jsonrpc-2.0"],
            "security": ["genetic_key_validation"]
        }))
    }

    /// Handle system.info request
    fn handle_system_info(&self, _params: Value) -> Result<Value> {
        let primal = self_primal_name();
        Ok(serde_json::json!({
            "primal": primal,
            "version": env!("CARGO_PKG_VERSION"),
            "rust_version": env!("CARGO_PKG_RUST_VERSION"),
            "uptime_seconds": get_uptime_seconds(),
            "transport": "unix-socket",
            "protocol": "jsonrpc-2.0"
        }))
    }
}

/// Get system uptime in seconds (monotonic, process-scoped).
fn get_uptime_seconds() -> u64 {
    PROCESS_START.get_or_init(Instant::now).elapsed().as_secs()
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct StoreRequest {
    key: String,
    value: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RetrieveRequest {
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeleteRequest {
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListRequest {
    #[serde(default)]
    prefix: Option<String>,
}

// ============================================================================
// Storage Backend Trait
// ============================================================================

/// **STORAGE BACKEND TRAIT**
///
/// Trait for storage implementations to be used by RPC handlers.
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    /// Store a key-value pair
    async fn store(&self, key: &str, value: &[u8]) -> Result<()>;

    /// Retrieve a value by key
    async fn retrieve(&self, key: &str) -> Result<Vec<u8>>;

    /// Delete a key
    async fn delete(&self, key: &str) -> Result<()>;

    /// List keys with optional prefix
    async fn list(&self, prefix: &Option<String>) -> Result<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockStorage;

    #[async_trait::async_trait]
    impl StorageBackend for MockStorage {
        async fn store(&self, _key: &str, _value: &[u8]) -> Result<()> {
            Ok(())
        }

        async fn retrieve(&self, key: &str) -> Result<Vec<u8>> {
            Ok(format!("mock_value_{key}").into_bytes())
        }

        async fn delete(&self, _key: &str) -> Result<()> {
            Ok(())
        }

        async fn list(&self, _prefix: &Option<String>) -> Result<Vec<String>> {
            Ok(vec!["key1".to_string(), "key2".to_string()])
        }
    }

    #[test]
    fn test_ping() {
        let handler = NestGateRpcHandler::new();
        let result = handler.handle_ping(Value::Null);
        assert!(result.is_ok());
    }

    #[test]
    fn test_identity() {
        let handler = NestGateRpcHandler::new();
        let result = handler.handle_identity(Value::Null);
        assert!(result.is_ok());
        let identity = result.unwrap();
        assert_eq!(identity["primal"], "nestgate");
        assert_eq!(identity["domain"], "storage");
        assert_eq!(identity["license"], "AGPL-3.0-or-later");
        assert!(identity.get("version").is_some());
        assert!(identity.get("family_id").is_some());
    }

    #[test]
    fn test_capabilities() {
        let handler = NestGateRpcHandler::new();
        let result = handler.handle_capabilities(Value::Null);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_store_without_backend() {
        let handler = NestGateRpcHandler::new();
        let params = serde_json::json!({"key": "test", "value": [1, 2, 3]});
        let result = handler.handle_store(params).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_store_with_backend() {
        let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
        let params = serde_json::json!({"key": "test", "value": [1, 2, 3]});
        let result = handler.handle_store(params).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_retrieve_invalid_params() {
        let handler = NestGateRpcHandler::new();
        let result = handler
            .handle_retrieve(serde_json::json!("not_object"))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retrieve_without_backend() {
        let handler = NestGateRpcHandler::new();
        let result = handler
            .handle_retrieve(serde_json::json!({"key": "k"}))
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_retrieve_with_backend_ok() {
        let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
        let result = handler
            .handle_retrieve(serde_json::json!({"key": "k"}))
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_invalid_params() {
        let handler = NestGateRpcHandler::new();
        assert!(handler.handle_delete(serde_json::json!([])).await.is_err());
    }

    #[tokio::test]
    async fn test_delete_without_backend() {
        let handler = NestGateRpcHandler::new();
        assert!(
            handler
                .handle_delete(serde_json::json!({"key": "k"}))
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_delete_with_backend_ok() {
        let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
        assert!(
            handler
                .handle_delete(serde_json::json!({"key": "k"}))
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_list_invalid_params_type() {
        let handler = NestGateRpcHandler::new();
        assert!(handler.handle_list(serde_json::json!("x")).await.is_err());
    }

    #[tokio::test]
    async fn test_list_without_backend() {
        let handler = NestGateRpcHandler::new();
        assert!(handler.handle_list(serde_json::json!({})).await.is_err());
    }

    #[tokio::test]
    async fn test_list_with_backend_ok() {
        let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
        assert!(
            handler
                .handle_list(serde_json::json!({"prefix": "p"}))
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_store_invalid_params() {
        let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
        assert!(
            handler
                .handle_store(serde_json::json!({"key": 1}))
                .await
                .is_err()
        );
    }

    #[test]
    fn test_system_info_ok() {
        let handler = NestGateRpcHandler::new();
        let v = handler.handle_system_info(serde_json::Value::Null);
        assert!(v.is_ok());
    }

    #[tokio::test]
    async fn test_handle_method_dispatch_branches() {
        use crate::transport::jsonrpc::RpcMethodHandler;
        let handler = NestGateRpcHandler::new();
        assert!(
            handler
                .handle_method("storage.list", serde_json::json!({}))
                .await
                .is_err()
        );
        assert!(
            handler
                .handle_method("health.ping", serde_json::json!({}))
                .await
                .is_ok()
        );
    }
}
