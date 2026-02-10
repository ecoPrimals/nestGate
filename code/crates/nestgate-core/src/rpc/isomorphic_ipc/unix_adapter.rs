//! # 🔌 Unix Socket RPC Handler Adapter
//!
//! **BRIDGE**: Adapts existing JsonRpcUnixServer logic to RpcHandler trait
//!
//! This module bridges the isomorphic IPC system with NestGate's existing
//! Unix socket server implementation, allowing the Try→Detect→Adapt→Succeed
//! pattern to use the battle-tested Unix socket code.
//!
//! ## Architecture
//!
//! ```text
//! IsomorphicIpcServer
//!   ├─→ Try: UnixSocketRpcHandler (this adapter)
//!   │        └─→ Delegates to existing JSON-RPC logic
//!   └─→ Adapt: TcpFallbackServer
//!            └─→ Uses same handler interface
//! ```
//!
//! ## Usage
//!
//! ```no_run
//! use nestgate_core::rpc::isomorphic_ipc::{
//!     IsomorphicIpcServer,
//!     unix_adapter::UnixSocketRpcHandler,
//! };
//! use std::sync::Arc;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Create handler from existing storage state
//! let handler = Arc::new(UnixSocketRpcHandler::new().await?);
//!
//! // Create isomorphic server
//! let server = Arc::new(IsomorphicIpcServer::new(
//!     "nestgate".to_string(),
//!     handler,
//! ));
//!
//! // Start (automatically adapts to platform)
//! server.start().await?;
//! # Ok(())
//! # }
//! ```

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{debug, warn};

use super::tcp_fallback::RpcHandler;

/// JSON-RPC 2.0 Request
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[allow(dead_code)] // Required by JSON-RPC 2.0 spec for deserialization validation
    jsonrpc: String,
    method: String,
    params: Option<Value>,
    id: Option<Value>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// Storage service state
#[derive(Clone)]
struct StorageState {
    /// Persistent storage manager
    storage_manager: Arc<crate::services::storage::StorageManagerService>,
    /// Template storage (reserved for template.* method delegation)
    #[allow(dead_code)]
    templates: crate::rpc::template_storage::TemplateStorage,
    /// Audit storage (reserved for audit.* method delegation)
    #[allow(dead_code)]
    audits: crate::rpc::audit_storage::AuditStorage,
}

impl StorageState {
    async fn new() -> Result<Self> {
        let storage_manager = Arc::new(
            crate::services::storage::StorageManagerService::new()
                .await
                .map_err(|e| {
                    warn!("Failed to initialize storage manager: {}", e);
                    e
                })?,
        );

        Ok(Self {
            storage_manager,
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
        })
    }
}

/// Unix socket RPC handler adapter
///
/// Implements `RpcHandler` trait by delegating to NestGate's existing
/// JSON-RPC 2.0 request handling logic.
pub struct UnixSocketRpcHandler {
    state: Arc<StorageState>,
}

impl UnixSocketRpcHandler {
    /// Create new Unix socket RPC handler
    ///
    /// Initializes the storage state (same as existing Unix socket server).
    pub async fn new() -> Result<Self> {
        let state = Arc::new(StorageState::new().await?);
        Ok(Self { state })
    }

    /// Handle JSON-RPC 2.0 request (internal logic)
    ///
    /// ✅ EVOLVED: Now uses actual StorageState for real operations
    /// instead of returning hardcoded stub responses.
    async fn handle_rpc_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!("📥 Processing JSON-RPC request: method={}", request.method);

        let id = request.id.clone();
        let result = match request.method.as_str() {
            // Storage methods - delegate to real StorageManagerService
            "storage.store" => self.handle_storage_store(&request).await,
            "storage.retrieve" => self.handle_storage_retrieve(&request).await,
            "storage.list" => self.handle_storage_list(&request).await,
            "storage.delete" => self.handle_storage_delete(&request).await,
            "storage.exists" => self.handle_storage_exists(&request).await,

            // Health/info methods
            "health" => Ok(json!({
                "status": "healthy",
                "service": "nestgate",
                "version": env!("CARGO_PKG_VERSION"),
                "isomorphic": true
            })),
            "version" => Ok(json!({
                "version": env!("CARGO_PKG_VERSION"),
                "ipc": "isomorphic"
            })),
            "discover_capabilities" => Ok(json!({
                "primal": "nestgate",
                "version": env!("CARGO_PKG_VERSION"),
                "capabilities": [
                    "storage.store", "storage.retrieve", "storage.list",
                    "storage.delete", "storage.exists",
                    "health", "version", "discover_capabilities"
                ]
            })),

            // Unknown method
            _ => Err((-32601, format!("Method not found: {}", request.method))),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(value),
                error: None,
                id,
            },
            Err((code, message)) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code,
                    message,
                    data: None,
                }),
                id,
            },
        }
    }

    // ✅ EVOLVED: Real storage operations using StorageManagerService

    async fn handle_storage_store(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, String)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, "Missing params".to_string()))?;
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or((-32602, "Missing 'key' parameter".to_string()))?;
        let value = params
            .get("value")
            .ok_or((-32602, "Missing 'value' parameter".to_string()))?;

        let data = serde_json::to_vec(value)
            .map_err(|e| (-32603, format!("Serialization error: {}", e)))?;

        self.state
            .storage_manager
            .store_object("default", key, data)
            .await
            .map_err(|e| (-32603, format!("Storage error: {}", e)))?;

        Ok(json!({"status": "stored", "key": key}))
    }

    async fn handle_storage_retrieve(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, String)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, "Missing params".to_string()))?;
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or((-32602, "Missing 'key' parameter".to_string()))?;

        match self
            .state
            .storage_manager
            .retrieve_object("default", key)
            .await
        {
            Ok((data, _info)) => {
                let value: Value = serde_json::from_slice(data.as_ref())
                    .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&data).to_string()));
                // ✅ UNIVERSAL: Return both "value" (biomeOS) and "data" (legacy)
                Ok(json!({"value": value, "data": value, "key": key}))
            }
            Err(e) => {
                let err_str = e.to_string();
                if err_str.contains("not found") || err_str.contains("Not found") {
                    Ok(json!({"value": null, "data": null, "key": key}))
                } else {
                    Err((-32603, format!("Storage error: {}", e)))
                }
            }
        }
    }

    async fn handle_storage_list(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, String)> {
        let _params = request.params.as_ref();

        match self.state.storage_manager.list_datasets().await {
            Ok(datasets) => {
                let items: Vec<Value> = datasets
                    .iter()
                    .map(|d| json!({"name": d.name, "object_count": d.object_count}))
                    .collect();
                Ok(json!({"datasets": items}))
            }
            Err(e) => Err((-32603, format!("Storage error: {}", e))),
        }
    }

    async fn handle_storage_delete(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, String)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, "Missing params".to_string()))?;
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or((-32602, "Missing 'key' parameter".to_string()))?;

        self.state
            .storage_manager
            .delete_object("default", key)
            .await
            .map_err(|e| (-32603, format!("Storage error: {}", e)))?;

        Ok(json!({"status": "deleted", "key": key}))
    }

    async fn handle_storage_exists(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, String)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, "Missing params".to_string()))?;
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or((-32602, "Missing 'key' parameter".to_string()))?;

        let exists = (self
            .state
            .storage_manager
            .retrieve_object("default", key)
            .await)
            .is_ok();

        Ok(json!({"exists": exists, "key": key}))
    }
}

#[async_trait]
impl RpcHandler for UnixSocketRpcHandler {
    async fn handle_request(&self, request: Value) -> Value {
        // Parse JSON-RPC request
        match serde_json::from_value::<JsonRpcRequest>(request) {
            Ok(rpc_request) => {
                // ✅ EVOLVED: Now awaits async handler that uses real storage
                let response = self.handle_rpc_request(rpc_request).await;

                // Serialize response
                serde_json::to_value(response).unwrap_or_else(|e| {
                    json!({
                        "jsonrpc": "2.0",
                        "error": {
                            "code": -32603,
                            "message": format!("Internal error: {}", e)
                        },
                        "id": null
                    })
                })
            }
            Err(e) => {
                // Invalid request
                json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32700,
                        "message": format!("Parse error: {}", e)
                    },
                    "id": null
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_creation() {
        let handler = UnixSocketRpcHandler::new().await;
        assert!(handler.is_ok());
    }

    #[tokio::test]
    async fn test_health_request() {
        let handler = UnixSocketRpcHandler::new().await.unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "health",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["result"]["status"] == "healthy");
        assert!(response["result"]["isomorphic"] == true);
    }

    #[tokio::test]
    async fn test_version_request() {
        let handler = UnixSocketRpcHandler::new().await.unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "version",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["result"]["version"].is_string());
        assert!(response["result"]["ipc"] == "isomorphic");
    }

    #[tokio::test]
    async fn test_unknown_method() {
        let handler = UnixSocketRpcHandler::new().await.unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "unknown.method",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["error"].is_object());
        assert!(response["error"]["code"] == -32601);
    }

    #[tokio::test]
    async fn test_invalid_request() {
        let handler = UnixSocketRpcHandler::new().await.unwrap();

        let request = json!({
            "not": "valid"
        });

        let response = handler.handle_request(request).await;

        assert!(response["error"].is_object());
        assert!(response["error"]["code"] == -32700);
    }
}
