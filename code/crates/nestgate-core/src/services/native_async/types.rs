// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Extracted from `native_async_final_services.rs` to maintain file size compliance
/// Contains data structures, enums, and type definitions for native async services
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
// UniversalResponseStatus removed - use canonical response types

/// Service request type - re-export from canonical traits
// UniversalServiceRequest removed - use domain-specific request types
/// Service response type
#[derive(Debug, Clone)]
/// Response data for Service operation
pub struct ServiceResponse {
    /// Success
    pub success: bool,
    /// Data
    pub data: Vec<u8>,
    /// Request identifier
    pub request_id: Option<String>,
    /// Status
    pub status: crate::canonical_types::ResponseStatus,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Payload
    pub payload: serde_json::Value,
    /// Timestamp
    pub timestamp: u64,
    /// Duration
    pub duration: std::time::Duration,
    /// Processing Time
    pub processing_time: u64,
    /// Tags
    pub tags: HashMap<String, String>,
    /// Error Details
    pub error_details: Option<String>,
    /// Correlation identifier
    pub correlation_id: Option<String>,
    /// Trace identifier
    pub trace_id: Option<String>,
}
/// Load balancer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadbalancerstats
pub struct LoadBalancerStats {
    /// Total Requests
    pub total_requests: u64,
    /// Successful Requests
    pub successful_requests: u64,
    /// Failed Requests
    pub failed_requests: u64,
    /// Average Response Time
    pub average_response_time: f64,
    /// Service Stats
    pub service_stats: HashMap<String, ServiceStats>,
    /// Algorithm
    pub algorithm: String,
    /// Health Aware
    pub health_aware: bool,
    /// Uptime Seconds
    pub uptime_seconds: u64,
}
/// Service statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicestats
pub struct ServiceStats {
    /// Requests
    pub requests: u64,
    /// Successful Requests
    pub successful_requests: u64,
    /// Failed Requests
    pub failed_requests: u64,
    /// Average Response Time
    pub average_response_time: f64,
    /// Current Load
    pub current_load: f64,
    /// Health Score
    pub health_score: f64,
    /// Last Request Time
    pub last_request_time: Option<SystemTime>,
}
impl Default for ServiceStats {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: 0.0,
            current_load: 0.0,
            health_score: 1.0,
            last_request_time: None,
        }
    }
}

/// Communication message
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Communicationmessage
pub struct CommunicationMessage {
    /// Message identifier
    pub message_id: String,
    /// Sender
    pub sender: String,
    /// Recipient
    pub recipient: String,
    /// Unified Message Type
    pub unified_message_type: String,
    /// Payload
    pub payload: serde_json::Value,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Priority
    pub priority: MessagePriority,
}
/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Messagepriority
pub enum MessagePriority {
    /// Low
    Low,
    /// Normal
    Normal,
    /// High
    High,
    /// Critical
    Critical,
}
/// Network address
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkaddress
pub struct NetworkAddress {
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Protocol
    pub protocol: String,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Connectioninfo
pub struct ConnectionInfo {
    /// Connection identifier
    pub connection_id: String,
    /// Endpoint
    pub endpoint: NetworkAddress,
    /// Established At
    pub established_at: SystemTime,
    /// Status
    pub status: ConnectionStatus,
    /// Bytes Sent
    pub bytes_sent: u64,
    /// Bytes Received
    pub bytes_received: u64,
}
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Connection
pub enum ConnectionStatus {
    /// Connecting
    Connecting,
    /// Connected
    Connected,
    /// Disconnecting
    Disconnecting,
    /// Disconnected
    Disconnected,
    /// An error occurred during the connection.
    Error(String),
}
/// MCP session information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mcpsessioninfo
pub struct MCPSessionInfo {
    /// Session identifier
    pub session_id: String,
    /// Client name
    pub client_name: String,
    /// Protocol Version
    pub protocol_version: u32,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Last Activity
    pub last_activity: SystemTime,
    /// Count of message
    pub message_count: u64,
    /// Capabilities
    pub capabilities: Vec<String>,
}
/// MCP protocol message
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mcpmessage
pub struct MCPMessage {
    /// Message identifier
    pub message_id: String,
    /// Method
    pub method: String,
    /// Params
    pub params: Option<serde_json::Value>,
    /// Timestamp
    pub timestamp: SystemTime,
}
/// MCP protocol response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for MCP operation
pub struct MCPResponse {
    /// Message identifier
    pub message_id: String,
    /// Result
    pub result: Option<serde_json::Value>,
    /// Error
    pub error: Option<MCPError>,
    /// Timestamp
    pub timestamp: SystemTime,
}
/// MCP error information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Error type for MCP operations
pub struct MCPError {
    /// Code
    pub code: i32,
    /// Message
    pub message: String,
    /// Data
    pub data: Option<serde_json::Value>,
}
/// Workflow execution information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workflowexecution
pub struct WorkflowExecution {
    /// Execution identifier
    pub execution_id: String,
    /// Workflow identifier
    pub workflow_id: String,
    /// Started At
    pub started_at: SystemTime,
    /// Completed At
    pub completed_at: Option<SystemTime>,
    /// Status
    pub status: ExecutionStatus,
    /// Progress
    pub progress: f64,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
}
/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Execution
pub enum ExecutionStatus {
    /// Pending
    Pending,
    /// Running
    Running,
    /// Completed
    Completed,
    /// Failed with error message
    Failed(String),
    /// Cancelled
    Cancelled,
}
