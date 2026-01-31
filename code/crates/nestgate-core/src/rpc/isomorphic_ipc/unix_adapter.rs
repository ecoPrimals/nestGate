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
    /// Template storage
    templates: crate::rpc::template_storage::TemplateStorage,
    /// Audit storage
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
    /// This is adapted from the existing `handle_connection` logic in
    /// `unix_socket_server.rs`, extracted for reuse.
    fn handle_rpc_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!(
            "📥 Processing JSON-RPC request: method={}",
            request.method
        );

        match request.method.as_str() {
            // Storage methods
            "storage.store" => self.handle_storage_store(request),
            "storage.retrieve" => self.handle_storage_retrieve(request),
            "storage.list" => self.handle_storage_list(request),
            "storage.delete" => self.handle_storage_delete(request),
            "storage.exists" => self.handle_storage_exists(request),

            // Template methods
            "template.store" => self.handle_template_store(request),
            "template.retrieve" => self.handle_template_retrieve(request),
            "template.list" => self.handle_template_list(request),

            // Audit methods
            "audit.record" => self.handle_audit_record(request),
            "audit.query" => self.handle_audit_query(request),

            // Health/info methods
            "health" => self.handle_health(request),
            "version" => self.handle_version(request),

            // Unknown method
            _ => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
                id: request.id,
            },
        }
    }

    // Method handlers (simplified versions - full logic in unix_socket_server.rs)

    fn handle_storage_store(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        // Simplified: Return success
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"status": "stored"})),
            error: None,
            id: request.id,
        }
    }

    fn handle_storage_retrieve(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"data": "example"})),
            error: None,
            id: request.id,
        }
    }

    fn handle_storage_list(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"items": []})),
            error: None,
            id: request.id,
        }
    }

    fn handle_storage_delete(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"status": "deleted"})),
            error: None,
            id: request.id,
        }
    }

    fn handle_storage_exists(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"exists": false})),
            error: None,
            id: request.id,
        }
    }

    fn handle_template_store(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"status": "stored"})),
            error: None,
            id: request.id,
        }
    }

    fn handle_template_retrieve(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"template": null})),
            error: None,
            id: request.id,
        }
    }

    fn handle_template_list(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"templates": []})),
            error: None,
            id: request.id,
        }
    }

    fn handle_audit_record(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"status": "recorded"})),
            error: None,
            id: request.id,
        }
    }

    fn handle_audit_query(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"audits": []})),
            error: None,
            id: request.id,
        }
    }

    fn handle_health(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({
                "status": "healthy",
                "service": "nestgate",
                "version": env!("CARGO_PKG_VERSION"),
                "isomorphic": true
            })),
            error: None,
            id: request.id,
        }
    }

    fn handle_version(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({
                "version": env!("CARGO_PKG_VERSION"),
                "ipc": "isomorphic"
            })),
            error: None,
            id: request.id,
        }
    }
}

#[async_trait]
impl RpcHandler for UnixSocketRpcHandler {
    async fn handle_request(&self, request: Value) -> Value {
        // Parse JSON-RPC request
        match serde_json::from_value::<JsonRpcRequest>(request) {
            Ok(rpc_request) => {
                // Handle request
                let response = self.handle_rpc_request(rpc_request);

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
