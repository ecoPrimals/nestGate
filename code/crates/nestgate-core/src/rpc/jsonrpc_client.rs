//! # 🔌 JSON-RPC 2.0 Client for Universal IPC
//!
//! **LIGHTWEIGHT JSON-RPC CLIENT FOR SONGBIRD COMMUNICATION**
//!
//! Provides a clean JSON-RPC client for calling Songbird's IPC service
//! and other JSON-RPC endpoints.
//!
//! ## Philosophy
//!
//! - **Service-Based**: Call Songbird as a SERVICE (not a library!)
//! - **Platform-Agnostic**: Works over Unix sockets, Named Pipes, etc.
//! - **Zero Dependencies**: Just tokio + serde_json
//! - **Modern Async**: Native async/await
//! - **Type-Safe**: Strong Result<T, E> patterns
//!
//! ## Usage
//!
//! ### Call Songbird for IPC Discovery
//!
//! ```no_run
//! use nestgate_core::rpc::JsonRpcClient;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Connect to Songbird's JSON-RPC service
//! let mut client = JsonRpcClient::connect_unix("/primal/songbird").await?;
//!
//! // Ask Songbird to resolve a primal's endpoint
//! let response = client.call("ipc.resolve", json!({
//!     "primal": "beardog"
//! })).await?;
//!
//! let endpoint = response["endpoint"].as_str().unwrap();
//! println!("BearDog is at: {}", endpoint);
//! # Ok(())
//! # }
//! ```
//!
//! ### Register With Songbird
//!
//! ```no_run
//! use nestgate_core::rpc::JsonRpcClient;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let mut client = JsonRpcClient::connect_unix("/primal/songbird").await?;
//!
//! // Register ourselves with Songbird
//! let response = client.call("ipc.register", json!({
//!     "primal": "nestgate",
//!     "capabilities": ["storage", "discovery"],
//!     "endpoint": "/primal/nestgate"
//! })).await?;
//!
//! println!("Registered: {:?}", response);
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{debug, warn};

use crate::error::{NestGateError, Result};

/// JSON-RPC 2.0 request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name (e.g., "ipc.resolve")
    pub method: String,
    /// Method parameters
    pub params: Value,
    /// Request ID (for matching responses)
    pub id: u64,
}

/// JSON-RPC 2.0 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
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
    pub message: String,
    /// Optional additional data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC client for calling external services
///
/// Supports Unix socket connections (primary) with future support for
/// other transports (HTTP, Named Pipes, etc.)
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
    /// * `path` - Unix socket path (e.g., "/primal/songbird")
    ///
    /// # Errors
    /// Returns error if connection fails
    ///
    /// # Example
    /// ```no_run
    /// use nestgate_core::rpc::JsonRpcClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = JsonRpcClient::connect_unix("/primal/songbird").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_unix(path: &str) -> Result<Self> {
        debug!("Connecting to JSON-RPC service at: {}", path);

        let stream = UnixStream::connect(path).await.map_err(|e| {
            NestGateError::network_error(&format!(
                "Failed to connect to JSON-RPC service at {}: {}",
                path, e
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
    /// ```no_run
    /// use nestgate_core::rpc::JsonRpcClient;
    /// use std::time::Duration;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = JsonRpcClient::connect_unix("/primal/songbird").await?;
    /// client.set_timeout(Duration::from_secs(10));
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_timeout(&mut self, timeout: Duration) {
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
    /// ```no_run
    /// use nestgate_core::rpc::JsonRpcClient;
    /// use serde_json::json;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = JsonRpcClient::connect_unix("/primal/songbird").await?;
    ///
    /// let result = client.call("ipc.resolve", json!({
    ///     "primal": "beardog"
    /// })).await?;
    ///
    /// println!("Endpoint: {}", result["endpoint"]);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call(&mut self, method: &str, params: Value) -> Result<Value> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            NestGateError::network_error("JSON-RPC client not connected")
        })?;

        // Create request
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: self.next_id,
        };
        self.next_id += 1;

        debug!("JSON-RPC request: {} (id={})", method, request.id);

        // Serialize request
        let request_json = serde_json::to_string(&request).map_err(|e| {
            NestGateError::api_internal_error(&format!("Failed to serialize request: {}", e))
        })?;

        // Send request (JSON-RPC over newline-delimited JSON)
        let request_line = format!("{}\n", request_json);
        
        tokio::time::timeout(self.timeout, stream.write_all(request_line.as_bytes()))
            .await
            .map_err(|_| {
                NestGateError::timeout_error("JSON-RPC request", self.timeout)
            })?
            .map_err(|e| {
                NestGateError::network_error(&format!("Failed to send request: {}", e))
            })?;

        tokio::time::timeout(self.timeout, stream.flush())
            .await
            .map_err(|_| {
                NestGateError::timeout_error("JSON-RPC flush", self.timeout)
            })?
            .map_err(|e| {
                NestGateError::network_error(&format!("Failed to flush request: {}", e))
            })?;

        // Read response (newline-delimited JSON)
        let mut reader = BufReader::new(stream);
        let mut response_line = String::new();
        
        tokio::time::timeout(self.timeout, reader.read_line(&mut response_line))
            .await
            .map_err(|_| {
                NestGateError::timeout_error("JSON-RPC response", self.timeout)
            })?
            .map_err(|e| {
                NestGateError::network_error(&format!("Failed to read response: {}", e))
            })?;

        // Parse response
        let response: JsonRpcResponse = serde_json::from_str(&response_line).map_err(|e| {
            warn!("Invalid JSON-RPC response: {}", response_line);
            NestGateError::api_internal_error(&format!("Failed to parse response: {}", e))
        })?;

        debug!(
            "JSON-RPC response: id={}, success={}",
            response.id,
            response.error.is_none()
        );

        // Check for errors
        if let Some(error) = response.error {
            return Err(NestGateError::api_error(&format!(
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
    /// ```no_run
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
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = JsonRpcClient::connect_unix("/primal/songbird").await?;
    ///
    /// let info: ServiceInfo = client.call_typed("ipc.resolve", json!({
    ///     "primal": "beardog"
    /// })).await?;
    ///
    /// println!("BearDog endpoint: {}", info.endpoint);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn call_typed<T>(&mut self, method: &str, params: Value) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let result = self.call(method, params).await?;
        serde_json::from_value(result).map_err(|e| {
            NestGateError::api_internal_error(&format!(
                "Failed to deserialize JSON-RPC result: {}",
                e
            ))
        })
    }

    /// Close the connection
    pub async fn close(&mut self) -> Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream.shutdown().await.map_err(|e| {
                NestGateError::network_error(&format!("Failed to close connection: {}", e))
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "ipc.resolve".to_string(),
            params: json!({"primal": "beardog"}),
            id: 1,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"method\":\"ipc.resolve\""));
        assert!(json.contains("\"id\":1"));
    }

    #[test]
    fn test_response_deserialization_success() {
        let json = r#"{"jsonrpc":"2.0","result":{"endpoint":"/primal/beardog"},"id":1}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_response_deserialization_error() {
        let json = r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":1}"#;
        let response: JsonRpcResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, 1);
        assert!(response.result.is_none());
        assert!(response.error.is_some());
        
        let error = response.error.unwrap();
        assert_eq!(error.code, -32601);
        assert_eq!(error.message, "Method not found");
    }
}
