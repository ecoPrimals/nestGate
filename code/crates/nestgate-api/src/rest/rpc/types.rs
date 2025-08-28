// **UNIFIED RPC TYPES**
//
// Core type definitions for the unified RPC layer including requests,
// responses, configurations, and error types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Unified RPC request that can be sent over either tarpc or JSON RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedRpcRequest {
    /// Unique request identifier
    pub id: Uuid,
    /// Source service (e.g., "nestgate")
    pub source: String,
    /// Target service capability (e.g., "security", "orchestration")
    pub target: String,
    /// RPC method name
    pub method: String,
    /// Request parameters
    pub params: serde_json::Value,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Whether this request expects a streaming response
    pub streaming: bool,
    /// Priority level for request processing
    pub priority: RequestPriority,
    /// Timeout for the request
    pub timeout: Option<Duration>,
}

/// Request priority levels for processing order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestPriority {
    /// Low priority - background tasks
    Low = 1,
    /// Normal priority - standard requests
    Normal = 2,
    /// High priority - user-facing operations
    High = 3,
    /// Critical priority - system operations
    Critical = 4,
}

impl Default for RequestPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Unified RPC response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedRpcResponse {
    /// Request ID this response corresponds to
    pub request_id: Uuid,
    /// Response status
    pub success: bool,
    /// Response data (if successful)
    pub data: Option<serde_json::Value>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Performance metrics
    pub metrics: ResponseMetrics,
}

/// Performance metrics for RPC responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetrics {
    /// Time spent processing the request (milliseconds)
    pub processing_time_ms: u64,
    /// Network latency (milliseconds)
    pub network_latency_ms: Option<u64>,
    /// Connection pool utilization percentage
    pub connection_pool_utilization: Option<f32>,
}

impl Default for ResponseMetrics {
    fn default() -> Self {
        Self {
            processing_time_ms: 0,
            network_latency_ms: None,
            connection_pool_utilization: None,
        }
    }
}

/// RPC stream event for bidirectional communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcStreamEvent {
    /// Stream identifier
    pub stream_id: Uuid,
    /// Event type
    pub event_type: String,
    /// Event data
    pub data: serde_json::Value,
    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// RPC connection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpcConnectionType {
    /// Binary RPC via tarpc (for beardog, high-performance)
    Tarpc,
    /// JSON RPC via HTTP (for songbird, standard)
    JsonRpc,
    /// WebSocket (for real-time streams)
    WebSocket,
}

/// Unified RPC service trait - **ZERO-COST NATIVE ASYNC**
/// **CANONICAL MODERNIZATION**: High-performance native async patterns for RPC operations
pub trait UnifiedRpcService: Send + Sync {
    /// Send a request and wait for response
    fn call(&self, request: UnifiedRpcRequest) -> impl std::future::Future<Output = Result<UnifiedRpcResponse, RpcError>> + Send;

    /// Start a bidirectional stream
    fn start_stream(
        &self,
        request: UnifiedRpcRequest,
    ) -> impl std::future::Future<Output = Result<(mpsc::Sender<RpcStreamEvent>, mpsc::Receiver<RpcStreamEvent>), RpcError>> + Send;

    /// Get connection type
    fn connection_type(&self) -> RpcConnectionType;

    /// Check if service is available
    fn health_check(&self) -> impl std::future::Future<Output = Result<bool, RpcError>> + Send;
}

/// RPC operation errors
#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum RpcError {
    /// Failed to establish RPC connection
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// RPC request timed out
    #[error("Request timeout: {0}")]
    Timeout(String),

    /// Failed to serialize/deserialize RPC data
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// RPC service is not available
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Error in RPC stream handling
    #[error("Stream error: {0}")]
    StreamError(String),

    /// Internal RPC system error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for RpcError {
    fn from(err: serde_json::Error) -> Self {
        RpcError::Serialization(err.to_string())
    }
}
