// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # 🔌 JSON-RPC 2.0 Client for Universal IPC
//!
//! **LIGHTWEIGHT JSON-RPC CLIENT FOR CAPABILITY-PEER IPC**
//!
//! Provides a clean JSON-RPC client for calling peer capability services
//! and other JSON-RPC endpoints.
//!
//! ## Philosophy
//!
//! - **Service-Based**: Call capability providers as services (not libraries!)
//! - **Platform-Agnostic**: Works over Unix sockets, Named Pipes, etc.
//! - **Zero Dependencies**: Just tokio + `serde_json`
//! - **Modern Async**: Native async/await
//! - **Type-Safe**: Strong Result<T, E> patterns
//!
//! ## Usage
//!
//! ### Call the orchestration provider for IPC discovery
//!
//! ```rust,ignore
//! use nestgate_core::rpc::JsonRpcClient;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), nestgate_types::NestGateError> {
//! // Connect to the orchestration provider's JSON-RPC service
//! let mut client = JsonRpcClient::connect_unix("/run/capability/orchestration.sock").await?;
//!
//! // Ask the orchestrator to resolve a capability
//! let response = client.call("ipc.resolve", json!({
//!     "capability": "security"
//! })).await?;
//!
//! let endpoint = response["endpoint"].as_str().unwrap_or("");
//! println!("Security provider at: {}", endpoint);
//! # Ok(())
//! # }
//! ```
//!
//! ### Register With Orchestration Provider
//!
//! ```rust,ignore
//! use nestgate_core::rpc::JsonRpcClient;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), nestgate_types::NestGateError> {
//! let mut client = JsonRpcClient::connect_unix("/run/capability/orchestration.sock").await?;
//!
//! // Register ourselves with the orchestration provider
//! let response = client.call("ipc.register", json!({
//!     "service_id": "nestgate",
//!     "capabilities": ["storage", "discovery"],
//!     "endpoint": "/run/capability/storage.sock"
//! })).await?;
//!
//! println!("Registered: {:?}", response);
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{debug, warn};

use nestgate_types::error::{NestGateError, Result};

/// JSON-RPC 2.0 request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: Arc<str>,
    /// Method name (e.g., "ipc.resolve")
    pub method: Arc<str>,
    /// Method parameters
    pub params: Value,
    /// Request ID (for matching responses)
    pub id: u64,
}

/// JSON-RPC 2.0 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: Arc<str>,
    /// Result (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID
    pub id: u64,
}

/// JSON-RPC 2.0 error object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i64,
    /// Error message
    pub message: Cow<'static, str>,
    /// Optional additional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC client for calling external services
///
/// Supports Unix socket connections (primary) with future support for
/// other transports (HTTP, Named Pipes, etc.)
#[derive(Debug)]
pub struct JsonRpcClient {
    /// Unix socket stream
    stream: Option<UnixStream>,
    /// Request ID counter
    next_id: u64,
    /// Request timeout
    timeout: Duration,
}

impl JsonRpcClient {
    /// Connect to a JSON-RPC service over Unix socket
    ///
    /// # Arguments
    /// * `path` - Unix socket path (e.g., "/run/capability/orchestration.sock")
    ///
    /// # Errors
    /// Returns error if connection fails
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::JsonRpcClient;
    ///
    /// # async fn example() -> Result<(), nestgate_types::NestGateError> {
    /// let client = JsonRpcClient::connect_unix("/run/capability/orchestration.sock").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_unix(path: &str) -> Result<Self> {
        debug!("Connecting to JSON-RPC service at: {}", path);

        let stream = UnixStream::connect(path).await.map_err(|e| {
            NestGateError::network_error(format!(
                "Failed to connect to JSON-RPC service at {path}: {e}"
            ))
        })?;

        Ok(Self {
            stream: Some(stream),
            next_id: 1,
            timeout: Duration::from_secs(5),
        })
    }

    /// Set request timeout
    ///
    /// # Arguments
    /// * `timeout` - Timeout duration
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::JsonRpcClient;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), nestgate_types::NestGateError> {
    /// let mut client = JsonRpcClient::connect_unix("/run/capability/orchestration.sock").await?;
    /// client.set_timeout(Duration::from_secs(10));
    /// # Ok(())
    /// # }
    /// ```
    pub const fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    /// Call a JSON-RPC method
    ///
    /// # Arguments
    /// * `method` - Method name (e.g., "ipc.resolve")
    /// * `params` - Method parameters as JSON value
    ///
    /// # Returns
    /// Returns the result value from the response
    ///
    /// # Errors
    /// Returns error if:
    /// - Connection is closed
    /// - Request fails to send
    /// - Response is invalid
    /// - Server returns an error
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::JsonRpcClient;
    /// use serde_json::json;
    ///
    /// # async fn example() -> Result<(), nestgate_types::NestGateError> {
    /// let mut client = JsonRpcClient::connect_unix("/run/capability/orchestration.sock").await?;
    ///
    /// let result = client.call("ipc.resolve", json!({
    ///     "capability": "security"
    /// })).await?;
    ///
    /// println!("Endpoint: {}", result["endpoint"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call(&mut self, method: &str, params: Value) -> Result<Value> {
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| NestGateError::network_error("JSON-RPC client not connected"))?;

        // Create request
        let request = JsonRpcRequest {
            jsonrpc: Arc::from("2.0"),
            method: Arc::from(method),
            params,
            id: self.next_id,
        };
        self.next_id += 1;

        debug!("JSON-RPC request: {} (id={})", method, request.id);

        // Serialize request
        let request_json = serde_json::to_string(&request).map_err(|e| {
            NestGateError::api_internal_error(format!("Failed to serialize request: {e}"))
        })?;

        // Send request (JSON-RPC over newline-delimited JSON)
        let request_line = format!("{request_json}\n");

        tokio::time::timeout(self.timeout, stream.write_all(request_line.as_bytes()))
            .await
            .map_err(|_| NestGateError::timeout_error("JSON-RPC request", self.timeout))?
            .map_err(|e| NestGateError::network_error(format!("Failed to send request: {e}")))?;

        tokio::time::timeout(self.timeout, stream.flush())
            .await
            .map_err(|_| NestGateError::timeout_error("JSON-RPC flush", self.timeout))?
            .map_err(|e| NestGateError::network_error(format!("Failed to flush request: {e}")))?;

        // Read response (newline-delimited JSON)
        let mut reader = BufReader::new(stream);
        let mut response_line = String::new();

        tokio::time::timeout(self.timeout, reader.read_line(&mut response_line))
            .await
            .map_err(|_| NestGateError::timeout_error("JSON-RPC response", self.timeout))?
            .map_err(|e| NestGateError::network_error(format!("Failed to read response: {e}")))?;

        // Parse response
        let response: JsonRpcResponse = serde_json::from_str(&response_line).map_err(|e| {
            warn!("Invalid JSON-RPC response: {}", response_line);
            NestGateError::api_internal_error(format!("Failed to parse response: {e}"))
        })?;

        debug!(
            "JSON-RPC response: id={}, success={}",
            response.id,
            response.error.is_none()
        );

        // Check for errors
        if let Some(error) = response.error {
            return Err(NestGateError::api_error(format!(
                "JSON-RPC error {}: {}",
                error.code, error.message
            )));
        }

        // Return result
        response.result.ok_or_else(|| {
            NestGateError::api_internal_error("JSON-RPC response missing result field")
        })
    }

    /// Call a JSON-RPC method and deserialize result to specific type
    ///
    /// # Arguments
    /// * `method` - Method name
    /// * `params` - Method parameters
    ///
    /// # Returns
    /// Returns the deserialized result
    ///
    /// # Errors
    /// Returns error if call fails or deserialization fails
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::JsonRpcClient;
    /// use serde::{Deserialize, Serialize};
    /// use serde_json::json;
    ///
    /// #[derive(Debug, Deserialize)]
    /// struct ServiceInfo {
    ///     endpoint: String,
    ///     capabilities: Vec<String>,
    /// }
    ///
    /// # async fn example() -> Result<(), nestgate_types::NestGateError> {
    /// let mut client = JsonRpcClient::connect_unix("/run/capability/orchestration.sock").await?;
    ///
    /// let info: ServiceInfo = client.call_typed("ipc.resolve", json!({
    ///     "capability": "security"
    /// })).await?;
    ///
    /// println!("Security endpoint: {}", info.endpoint);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_typed<T>(&mut self, method: &str, params: Value) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let result = self.call(method, params).await?;
        serde_json::from_value(result).map_err(|e| {
            NestGateError::api_internal_error(format!("Failed to deserialize JSON-RPC result: {e}"))
        })
    }

    /// Close the connection
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] if shutting down the underlying Unix stream fails.
    pub async fn close(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream.shutdown().await.map_err(|e| {
                NestGateError::network_error(format!("Failed to close connection: {e}"))
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
impl JsonRpcClient {
    /// Construct a client around an existing Unix stream (in-process pair testing).
    pub(crate) fn from_unix_stream_for_test(stream: UnixStream) -> Self {
        Self {
            stream: Some(stream),
            next_id: 1,
            timeout: Duration::from_secs(5),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde::Deserialize;
    use serde_json::json;
    use std::borrow::Cow;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    #[test]
    fn test_request_serialization() {
        use serde_json::json;

        let request = JsonRpcRequest {
            jsonrpc: Arc::from("2.0"),
            method: Arc::from("ipc.resolve"),
            params: json!({"capability": "security"}),
            id: 1,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"method\":\"ipc.resolve\""));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_response_deserialization_success() {
        let json = r#"{"jsonrpc":"2.0","result":{"endpoint":"/capability/security"},"id":1}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.jsonrpc.as_ref(), "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_response_deserialization_error() {
        let json =
            r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":1}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.jsonrpc.as_ref(), "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_none());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, -32601);
        assert_eq!(error.message.as_ref(), "Method not found");
    }

    #[test]
    fn jsonrpc_error_roundtrips_optional_data() {
        let err = JsonRpcError {
            code: -32000,
            message: Cow::Borrowed("oops"),
            data: Some(json!({"hint": "x"})),
        };
        let s = serde_json::to_string(&err).unwrap();
        let back: JsonRpcError = serde_json::from_str(&s).unwrap();
        assert_eq!(back.code, -32000);
        assert_eq!(back.data.as_ref().unwrap()["hint"], "x");
    }

    #[tokio::test]
    async fn connect_unix_missing_socket_errors() {
        let r = JsonRpcClient::connect_unix(
            "/nonexistent/nestgate/jsonrpc_test_socket_please_ignore.sock",
        )
        .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn call_roundtrip_over_unix_pair() {
        let (server, client_sock) = UnixStream::pair().unwrap();
        let (rh, mut wh) = server.into_split();
        let server_task = tokio::spawn(async move {
            let mut buf = String::new();
            let mut br = BufReader::new(rh);
            br.read_line(&mut buf).await.unwrap();
            let response = r#"{"jsonrpc":"2.0","result":{"ping":true},"id":1}"#;
            wh.write_all(format!("{response}\n").as_bytes())
                .await
                .unwrap();
            wh.flush().await.unwrap();
        });

        let mut client = JsonRpcClient::from_unix_stream_for_test(client_sock);
        let v = client.call("demo.method", json!({"a": 1})).await.unwrap();
        assert_eq!(v["ping"], true);
        server_task.await.unwrap();
    }

    #[tokio::test]
    async fn call_propagates_jsonrpc_error_object() {
        let (server, client_sock) = UnixStream::pair().unwrap();
        let (rh, mut wh) = server.into_split();
        tokio::spawn(async move {
            let mut line = String::new();
            let mut br = BufReader::new(rh);
            br.read_line(&mut line).await.unwrap();
            let response = r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"nope"},"id":1}"#;
            wh.write_all(format!("{response}\n").as_bytes())
                .await
                .unwrap();
        });

        let mut client = JsonRpcClient::from_unix_stream_for_test(client_sock);
        let e = client.call("x", json!({})).await.unwrap_err();
        assert!(e.to_string().contains("JSON-RPC error") || e.to_string().contains("-32601"));
    }

    #[tokio::test]
    async fn call_typed_ok_and_bad_shape() {
        let (server, client_sock) = UnixStream::pair().unwrap();
        let (rh, mut wh) = server.into_split();
        tokio::spawn(async move {
            let mut line = String::new();
            let mut br = BufReader::new(rh);
            br.read_line(&mut line).await.unwrap();
            let response = r#"{"jsonrpc":"2.0","result":{"k":42},"id":1}"#;
            wh.write_all(format!("{response}\n").as_bytes())
                .await
                .unwrap();
        });

        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct R {
            k: i32,
        }

        let mut client = JsonRpcClient::from_unix_stream_for_test(client_sock);
        let ok: R = client.call_typed("m", json!({})).await.unwrap();
        assert_eq!(ok, R { k: 42 });

        let (server2, client_sock2) = UnixStream::pair().unwrap();
        let (rh2, mut wh2) = server2.into_split();
        tokio::spawn(async move {
            let mut line = String::new();
            let mut br = BufReader::new(rh2);
            br.read_line(&mut line).await.unwrap();
            let response = r#"{"jsonrpc":"2.0","result":"not-an-object","id":1}"#;
            wh2.write_all(format!("{response}\n").as_bytes())
                .await
                .unwrap();
        });
        let mut client2 = JsonRpcClient::from_unix_stream_for_test(client_sock2);
        let err = client2
            .call_typed::<R>("m", json!({}))
            .await
            .expect_err("wrong shape");
        assert!(err.to_string().contains("deserialize") || err.to_string().contains("JSON-RPC"));
    }

    #[tokio::test]
    async fn set_timeout_and_close_then_call_fails() {
        let (_s, client_sock) = UnixStream::pair().unwrap();
        let mut client = JsonRpcClient::from_unix_stream_for_test(client_sock);
        client.set_timeout(Duration::from_secs(60));
        client.close().await.unwrap();
        let e = client.call("m", json!({})).await.unwrap_err();
        assert!(e.to_string().contains("not connected"));
    }
}
