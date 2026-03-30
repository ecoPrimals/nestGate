// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **JSON-RPC 2.0 HANDLER**
//!
//! JSON-RPC 2.0 protocol implementation for TRUE PRIMAL communication.

use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::future::Future;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
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

impl JsonRpcError {
    /// Parse error (-32700)
    #[must_use]
    pub fn parse_error() -> Self {
        Self {
            code: -32700,
            message: "Parse error".to_string(),
            data: None,
        }
    }

    /// Invalid request (-32600)
    #[must_use]
    pub fn invalid_request() -> Self {
        Self {
            code: -32600,
            message: "Invalid Request".to_string(),
            data: None,
        }
    }

    /// Method not found (-32601)
    #[must_use]
    pub fn method_not_found() -> Self {
        Self {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        }
    }

    /// Internal error (-32603)
    #[must_use]
    pub fn internal_error() -> Self {
        Self {
            code: -32603,
            message: "Internal error".to_string(),
            data: None,
        }
    }
}

impl JsonRpcResponse {
    /// Create success response
    #[must_use]
    pub fn success(id: impl Into<Value>, result: impl Into<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result.into()),
            error: None,
            id: id.into(),
        }
    }

    /// Create error response
    #[must_use]
    pub fn error(id: impl Into<Value>, error: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(error),
            id: id.into(),
        }
    }

    /// Create error response with code and message
    #[must_use]
    pub fn error_with_code(id: impl Into<Value>, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
            id: id.into(),
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
            stream
                .readable()
                .await
                .map_err(|e| NestGateError::network_error(format!("Socket not readable: {e}")))?;

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

            let request: JsonRpcRequest = match serde_json::from_slice(&buffer[..n]) {
                Ok(req) => req,
                Err(e) => {
                    error!("Invalid JSON-RPC request: {}", e);
                    let error_response =
                        JsonRpcResponse::error_with_code(Value::Null, -32700, "Parse error");
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
                let err_msg = e.to_string();
                let (code, message) = if err_msg.contains("Unknown method") {
                    (-32601, "Method not found".to_string())
                } else {
                    (-32603, format!("Internal error: {e}"))
                };
                error!("Method '{}' failed: {}", request.method, e);
                JsonRpcResponse::error_with_code(request.id, code, message)
            }
        }
    }

    async fn send_response(
        &self,
        stream: &mut UnixStream,
        response: &JsonRpcResponse,
    ) -> Result<()> {
        let response_str = serde_json::to_string(response)
            .map_err(|e| NestGateError::api_error(format!("Failed to serialize response: {e}")))?;

        trace!("Sending response: {}", response_str);

        stream
            .writable()
            .await
            .map_err(|e| NestGateError::network_error(format!("Socket not writable: {e}")))?;

        stream
            .write_all(response_str.as_bytes())
            .await
            .map_err(|e| NestGateError::network_error(format!("Failed to write response: {e}")))?;

        Ok(())
    }
}

/// **RPC METHOD HANDLER TRAIT**
///
/// Trait for handling RPC method calls.
pub trait RpcMethodHandler {
    /// Handle RPC method
    fn handle_method(
        &self,
        method: &str,
        params: Value,
    ) -> impl Future<Output = Result<Value>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::error::Result;

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
        let response = JsonRpcResponse::error_with_code(Value::from(1), -32600, "Invalid Request");
        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn jsonrpc_error_standard_codes() {
        assert_eq!(JsonRpcError::parse_error().code, -32700);
        assert_eq!(JsonRpcError::invalid_request().code, -32600);
        assert_eq!(JsonRpcError::method_not_found().code, -32601);
        assert_eq!(JsonRpcError::internal_error().code, -32603);
    }

    #[test]
    fn jsonrpc_response_error_constructor_roundtrip() {
        let e = JsonRpcError::method_not_found();
        let r = JsonRpcResponse::error(Value::from(42), e.clone());
        assert!(r.result.is_none());
        assert_eq!(r.error.as_ref().map(|x| x.code), Some(-32601));
        assert_eq!(r.id, Value::from(42));
    }

    struct EchoHandler;

    impl RpcMethodHandler for EchoHandler {
        async fn handle_method(&self, method: &str, params: Value) -> Result<Value> {
            match method {
                "echo" => Ok(params),
                _ => Err(NestGateError::api_error("Unknown method")),
            }
        }
    }

    #[tokio::test]
    async fn handle_request_success_and_unknown_method() {
        let h = JsonRpcHandler::new(EchoHandler);
        let ok = h
            .handle_request(JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "echo".to_string(),
                params: Value::String("hi".to_string()),
                id: Value::from(1),
            })
            .await;
        assert_eq!(ok.result, Some(Value::String("hi".to_string())));

        let bad = h
            .handle_request(JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "missing".to_string(),
                params: Value::Null,
                id: Value::from(2),
            })
            .await;
        assert!(bad.error.is_some());
        assert_eq!(bad.error.as_ref().unwrap().code, -32601);
    }

    #[tokio::test]
    async fn handle_request_internal_error_when_not_unknown_method() {
        struct FailHandler;
        impl RpcMethodHandler for FailHandler {
            async fn handle_method(&self, _method: &str, _params: Value) -> Result<Value> {
                Err(NestGateError::api_error("boom"))
            }
        }
        let h = JsonRpcHandler::new(FailHandler);
        let r = h
            .handle_request(JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "x".to_string(),
                params: Value::Null,
                id: Value::Null,
            })
            .await;
        assert_eq!(r.error.as_ref().unwrap().code, -32603);
    }
}
