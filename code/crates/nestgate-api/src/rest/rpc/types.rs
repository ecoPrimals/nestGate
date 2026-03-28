// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **UNIFIED RPC TYPES**
//
// Core type definitions for the unified RPC layer including requests,
// responses, configurations, and error types.

//! Types module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Unified RPC request that can be sent over either tarpc or JSON RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for UnifiedRpc operation
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
    pub _params: serde_json::Value,
    /// Request _metadata
    pub _metadata: HashMap<String, String>,
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
/// Requestpriority
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
    /// Returns the default instance
    fn default() -> Self {
        Self::Normal
    }
}

/// Unified RPC response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for UnifiedRpc operation
pub struct UnifiedRpcResponse {
    /// Request ID this response corresponds to
    pub request_id: Uuid,
    /// Response status
    pub success: bool,
    /// Response data (if successful)
    pub data: Option<serde_json::Value>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Response _metadata
    pub _metadata: HashMap<String, String>,
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Performance metrics
    pub metrics: ResponseMetrics,
}
/// Performance metrics for RPC responses
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Responsemetrics
pub struct ResponseMetrics {
    /// Time spent processing the request (milliseconds)
    pub processing_time_ms: u64,
    /// Network latency (milliseconds)
    pub network_latency_ms: Option<u64>,
    /// Connection pool utilization percentage
    pub connection_pool_utilization: Option<f32>,
}

/// RPC stream event for bidirectional communication
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Rpcstreamevent
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
/// Types of RpcConnection
pub enum RpcConnectionType {
    /// Binary RPC via tarpc (for security, high-performance)
    Tarpc,
    /// JSON RPC via HTTP (for orchestration, standard)
    JsonRpc,
    /// WebSocket (for real-time streams)
    WebSocket,
}
/// Unified RPC service trait - **ZERO-COST NATIVE ASYNC**
/// **CANONICAL MODERNIZATION**: High-performance native async patterns for RPC operations
pub trait UnifiedRpcService: Send + Sync {
    /// Send a request and wait for response
    fn call(
        &self,
        request: UnifiedRpcRequest,
    ) -> impl std::future::Future<Output = Result<UnifiedRpcResponse, RpcError>> + Send;
    /// Start a bidirectional stream
    fn start_stream(
        &self,
        request: UnifiedRpcRequest,
    ) -> impl std::future::Future<
        Output = Result<(mpsc::Sender<RpcStreamEvent>, mpsc::Receiver<RpcStreamEvent>), RpcError>,
    > + Send;

    /// Get connection type
    fn connection_type(&self) -> RpcConnectionType;

    /// Check if service is available
    fn health_check(&self) -> impl std::future::Future<Output = Result<bool, RpcError>> + Send;
}

/// **DYN-COMPATIBLE RPC SERVICE WRAPPER**
/// Wrapper enum for dynamic dispatch of RPC services
#[derive(Debug)]
/// Dynrpcservice
pub enum DynRpcService {
    /// JSON-RPC service implementation for HTTP-based communication
    JsonRpc(crate::rest::rpc::json_rpc_service::JsonRpcService),
}

impl DynRpcService {
    /// Execute an RPC call with the given request
    ///
    /// Dispatches the request to the appropriate underlying service implementation
    /// and returns the response or an error if the call fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn call(&self, request: UnifiedRpcRequest) -> Result<UnifiedRpcResponse, RpcError> {
        match self {
            Self::JsonRpc(service) => service.call(request).await,
        }
    }

    /// Start a bidirectional RPC stream
    ///
    /// Establishes a bidirectional communication stream for real-time
    /// data exchange between client and server.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn start_stream(
        &self,
        request: UnifiedRpcRequest,
    ) -> Result<(mpsc::Sender<RpcStreamEvent>, mpsc::Receiver<RpcStreamEvent>), RpcError> {
        match self {
            Self::JsonRpc(service) => service.start_stream(request).await,
        }
    }

    /// Get the connection type supported by this service
    ///
    /// Returns the type of RPC connection this service uses for
    /// routing and compatibility purposes.
    #[must_use]
    pub const fn connection_type(&self) -> RpcConnectionType {
        match self {
            Self::JsonRpc(_) => RpcConnectionType::JsonRpc,
        }
    }

    /// Perform a health check on the RPC service
    ///
    /// Verifies that the underlying service is healthy and ready
    /// to handle requests.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn health_check(&self) -> Result<bool, RpcError> {
        match self {
            Self::JsonRpc(service) => service.health_check().await,
        }
    }
}

/// RPC operation errors
#[derive(Debug, thiserror::Error, serde::Serialize)]
/// Errors that can occur during Rpc operations
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

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

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
    /// From
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}
