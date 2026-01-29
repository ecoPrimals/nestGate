//! **JSON-RPC 2.0 HANDLER**
//!
//! JSON-RPC 2.0 protocol implementation for TRUE PRIMAL communication.

use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tracing::{debug, error, trace};

/// **JSON-RPC 2.0 REQUEST**
///
/// Standard JSON-RPC 2.0 request format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// Protocol version (always "2.0")
    pub jsonrpc: String,

    /// Method name (e.g., "storage.store")
    pub method: String,

    /// Method parameters
    #[serde(default)]
    pub params: Value,

    /// Request ID (for matching responses)
    pub id: Value,
}

/// **JSON-RPC 2.0 RESPONSE**
///
/// Standard JSON-RPC 2.0 response format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// Protocol version (always "2.0")
    pub jsonrpc: String,

    /// Result (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,

    /// Error (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,

    /// Request ID (matches request)
    pub id: Value,
}

/// **JSON-RPC ERROR**
///
/// Standard JSON-RPC error format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,

    /// Error message
    pub message: String,

    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcResponse {
    /// Create success response
    #[must_use]
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Create error response
    #[must_use]
    pub fn error(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
            id,
        }
    }
}

/// **JSON-RPC HANDLER**
///
/// Handles JSON-RPC 2.0 requests over Unix sockets.
pub struct JsonRpcHandler<H> {
    pub(crate) handler: Arc<H>, // Made pub(crate) for Clone impl in server.rs
}

impl<H> JsonRpcHandler<H>
where
    H: RpcMethodHandler + Send + Sync + 'static,
{
    /// Create new JSON-RPC handler
    #[must_use]
    pub fn new(handler: H) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }

    /// Handle connection
    ///
    /// # Errors
    ///
    /// Returns error if connection handling fails
    pub async fn handle_connection(&self, mut stream: UnixStream) -> Result<()> {
        let mut buffer = vec![0u8; 65536]; // 64KB buffer

        loop {
            // Read request
            let _n = stream.readable().await.map_err(|e| {
                NestGateError::network_error(&format!("Socket not readable: {}", e))
            })?;

            let n = match stream.try_read(&mut buffer) {
                Ok(0) => {
                    debug!("Connection closed by peer");
                    break;
                }
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    error!("Failed to read from socket: {}", e);
                    break;
                }
            };

            // Parse request
            let request_str = String::from_utf8_lossy(&buffer[..n]);
            trace!("Received request: {}", request_str);

            let request: JsonRpcRequest = match serde_json::from_str(&request_str) {
                Ok(req) => req,
                Err(e) => {
                    error!("Invalid JSON-RPC request: {}", e);
                    let error_response = JsonRpcResponse::error(Value::Null, -32700, "Parse error");
                    let _ = self.send_response(&mut stream, &error_response).await;
                    continue;
                }
            };

            // Handle request
            let response = self.handle_request(request).await;

            // Send response
            if let Err(e) = self.send_response(&mut stream, &response).await {
                error!("Failed to send response: {}", e);
                break;
            }
        }

        Ok(())
    }

    /// Handle JSON-RPC request
    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Handling method: {}", request.method);

        match self
            .handler
            .handle_method(&request.method, request.params)
            .await
        {
            Ok(result) => JsonRpcResponse::success(request.id, result),
            Err(e) => {
                error!("Method '{}' failed: {}", request.method, e);
                JsonRpcResponse::error(request.id, -32603, format!("Internal error: {}", e))
            }
        }
    }

    async fn send_response(
        &self,
        stream: &mut UnixStream,
        response: &JsonRpcResponse,
    ) -> Result<()> {
        let response_str = serde_json::to_string(response).map_err(|e| {
            NestGateError::api_error(&format!("Failed to serialize response: {}", e))
        })?;

        trace!("Sending response: {}", response_str);

        stream
            .writable()
            .await
            .map_err(|e| NestGateError::network_error(&format!("Socket not writable: {}", e)))?;

        stream
            .write_all(response_str.as_bytes())
            .await
            .map_err(|e| {
                NestGateError::network_error(&format!("Failed to write response: {}", e))
            })?;

        Ok(())
    }
}

/// **RPC METHOD HANDLER TRAIT**
///
/// Trait for handling RPC method calls.
#[async_trait::async_trait]
pub trait RpcMethodHandler {
    /// Handle RPC method
    async fn handle_method(&self, method: &str, params: Value) -> Result<Value>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonrpc_request_parsing() {
        let json = r#"{"jsonrpc":"2.0","method":"test.method","params":{},"id":1}"#;
        let request: JsonRpcRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.method, "test.method");
        assert_eq!(request.jsonrpc, "2.0");
    }

    #[test]
    fn test_jsonrpc_response_success() {
        let response = JsonRpcResponse::success(Value::from(1), Value::from(true));
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_jsonrpc_response_error() {
        let response = JsonRpcResponse::error(Value::from(1), -32600, "Invalid Request");
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_none());
        assert!(response.error.is_some());
    }
}
