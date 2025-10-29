use std::collections::HashMap;
// CLEANED: Removed unused Future import as part of canonical modernization
// use std::future::Future;
/// Extracted from `native_async_final_services.rs` to maintain file size compliance
/// Contains data structures, enums, and type definitions for native async services
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
// UniversalResponseStatus removed - use canonical response types

/// Service request type - re-export from canonical traits
// UniversalServiceRequest removed - use domain-specific request types
/// Service response type
#[derive(Debug, Clone)]
pub struct ServiceResponse {
    pub success: bool,
    pub data: Vec<u8>,
    pub request_id: Option<String>,
    pub status: crate::canonical_types::ResponseStatus,
    pub headers: HashMap<String, String>,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub duration: std::time::Duration,
    pub processing_time: u64,
    pub tags: HashMap<String, String>,
    pub error_details: Option<String>,
    pub correlation_id: Option<String>,
    pub trace_id: Option<String>,
}
/// Load balancer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub service_stats: HashMap<String, ServiceStats>,
    pub algorithm: String,
    pub health_aware: bool,
    pub uptime_seconds: u64,
}
/// Service statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStats {
    pub requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time: f64,
    pub current_load: f64,
    pub health_score: f64,
    pub last_request_time: Option<SystemTime>,
}
impl Default for ServiceStats {
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
pub struct CommunicationMessage {
    pub message_id: String,
    pub sender: String,
    pub recipient: String,
    pub unified_message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: SystemTime,
    pub priority: MessagePriority,
}
/// Message priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePriority {
    Low,
    Normal,
    High,
    Critical,
}
/// Network address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAddress {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub metadata: HashMap<String, String>,
}
/// Connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub connection_id: String,
    pub endpoint: NetworkAddress,
    pub established_at: SystemTime,
    pub status: ConnectionStatus,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,
    Error(String),
}
/// MCP session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPSessionInfo {
    pub session_id: String,
    pub client_name: String,
    pub protocol_version: u32,
    pub created_at: SystemTime,
    pub last_activity: SystemTime,
    pub message_count: u64,
    pub capabilities: Vec<String>,
}
/// MCP protocol message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPMessage {
    pub message_id: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
    pub timestamp: SystemTime,
}
/// MCP protocol response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    pub message_id: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<MCPError>,
    pub timestamp: SystemTime,
}
/// MCP error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}
/// Workflow execution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub execution_id: String,
    pub workflow_id: String,
    pub started_at: SystemTime,
    pub completed_at: Option<SystemTime>,
    pub status: ExecutionStatus,
    pub progress: f64,
    pub parameters: HashMap<String, serde_json::Value>,
}
/// Execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Cancelled,
}
