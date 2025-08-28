//! **ERROR DATA STRUCTURES**
//!
//! This module provides domain-specific error data structures that carry
//! additional context and information for different types of errors.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// ==================== SECTION ====================

/// Storage-specific error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageErrorData {
    /// Storage backend that failed
    pub backend: String,
    /// Filesystem or storage type
    pub filesystem_type: Option<String>,
    /// Available disk space information
    pub disk_space: Option<DiskSpaceInfo>,
    /// I/O statistics at time of error
    pub io_stats: Option<IoStats>,
}

/// ZFS-specific error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsErrorData {
    /// ZFS operation that failed
    pub operation: String,
    /// ZFS pool involved
    pub pool: Option<String>,
    /// ZFS dataset involved
    pub dataset: Option<String>,
    /// ZFS snapshot involved
    pub snapshot: Option<String>,
    /// ZFS command that was executed
    pub command: Option<String>,
    /// ZFS command exit code
    pub exit_code: Option<i32>,
    /// ZFS command output
    pub command_output: Option<String>,
    /// Pool health status
    pub pool_health: Option<String>,
    /// Available space information
    pub space_info: Option<ZfsSpaceInfo>,
}

/// Disk space information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskSpaceInfo {
    pub total: u64,
    pub available: u64,
    pub used: u64,
    pub percentage_used: f64,
}

/// I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoStats {
    pub reads_per_second: u64,
    pub writes_per_second: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub queue_depth: u32,
}

/// ZFS space information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSpaceInfo {
    pub total_size: u64,
    pub used_size: u64,
    pub available_size: u64,
    pub compression_ratio: f64,
    pub deduplication_ratio: f64,
}

// ==================== SECTION ====================

/// Network-specific error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkErrorData {
    /// Network protocol that failed
    pub protocol: String,
    /// Local address
    pub local_address: Option<String>,
    /// Remote address
    pub remote_address: Option<String>,
    /// Connection timeout
    pub timeout: Option<Duration>,
    /// Connection statistics
    pub connection_stats: Option<ConnectionStats>,
    /// Network interface information
    pub interface_info: Option<NetworkInterfaceInfo>,
}

/// Connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub connection_duration: Duration,
}

/// Network interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceInfo {
    pub interface_name: String,
    pub mtu: u32,
    pub speed_mbps: Option<u64>,
    pub duplex: Option<String>,
    pub is_up: bool,
}

// ==================== SECTION ====================

/// Security-specific error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityErrorData {
    /// Security subject (user, service, etc.)
    pub subject: Option<String>,
    /// Security action attempted
    pub action: Option<String>,
    /// Resource being accessed
    pub resource: Option<String>,
    /// Security context
    pub security_context: Option<SecurityContext>,
    /// Authentication information
    pub auth_info: Option<AuthenticationInfo>,
    /// Permission details
    pub permissions: Option<Vec<String>>,
    /// Security severity level
    pub severity: SecuritySeverity,
}

/// Security context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    pub session_id: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub timestamp: SystemTime,
    pub security_level: String,
}

/// Authentication information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationInfo {
    pub auth_method: String,
    pub token_type: Option<String>,
    pub token_expiry: Option<SystemTime>,
    pub last_login: Option<SystemTime>,
    pub login_attempts: u32,
}

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

// ==================== SECTION ====================

/// MCP (Model Context Protocol) specific error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpErrorData {
    /// MCP message type
    pub message_type: String,
    /// Protocol version
    pub protocol_version: Option<String>,
    /// Message ID for correlation
    pub message_id: Option<String>,
    /// Session information
    pub session_info: Option<McpSessionInfo>,
    /// Transport details
    pub transport_info: Option<McpTransportInfo>,
}

/// MCP session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSessionInfo {
    pub session_id: String,
    pub client_info: Option<String>,
    pub server_info: Option<String>,
    pub established_at: SystemTime,
    pub message_count: u64,
}

/// MCP transport information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTransportInfo {
    pub transport_type: String,
    pub endpoint: Option<String>,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

// ==================== SECTION ====================

/// API-specific error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorData {
    /// HTTP method
    pub method: Option<String>,
    /// Request path
    pub path: Option<String>,
    /// HTTP status code
    pub status_code: Option<u16>,
    /// Request ID
    pub request_id: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Request headers (filtered for security)
    pub headers: Option<HashMap<String, String>>,
    /// Response time
    pub response_time: Option<Duration>,
}

// ==================== SECTION ====================

/// Automation-specific error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationErrorData {
    /// Current workflow step
    pub current_step: Option<String>,
    /// Total steps in workflow
    pub total_steps: Option<u32>,
    /// Step execution time
    pub step_duration: Option<Duration>,
    /// Workflow context
    pub workflow_context: Option<HashMap<String, String>>,
    /// Retry information
    pub retry_count: u32,
    /// Next retry time
    pub next_retry_at: Option<SystemTime>,
}

// ==================== SECTION ====================

/// Universal ZFS error data for enhanced operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalZfsErrorData {
    /// Backend that failed
    pub backend: Option<String>,
    /// Resource involved
    pub resource: Option<String>,
    /// Operation timeout
    pub timeout: Option<Duration>,
    /// Circuit breaker state
    pub circuit_breaker_open: bool,
    /// Backend health status
    pub backend_health: Option<String>,
    /// Load balancing information
    pub load_balance_info: Option<LoadBalanceInfo>,
}

/// Load balancing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalanceInfo {
    pub current_backend: String,
    pub available_backends: Vec<String>,
    pub backend_weights: HashMap<String, f64>,
    pub last_rotation: SystemTime,
}

// ==================== SECTION ====================

/// Middleware-specific error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareErrorData {
    /// Middleware name
    pub middleware_name: String,
    /// Request processing stage
    pub stage: String,
    /// Processing duration
    pub duration: Option<Duration>,
    /// Middleware configuration
    pub config: Option<HashMap<String, String>>,
    /// Request context
    pub request_context: Option<MiddlewareRequestContext>,
}

/// Middleware request context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareRequestContext {
    pub request_id: String,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub middleware_chain: Vec<String>,
    pub processing_time: Duration,
}

// ==================== SECTION ====================

/// File system monitor error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMonitorErrorData {
    /// File path being monitored
    pub path: String,
    /// Watch operation that failed
    pub operation: String,
    /// File system event type
    pub event_type: Option<String>,
    /// File metadata
    pub file_metadata: Option<FileMetadata>,
    /// Watch configuration
    pub watch_config: Option<WatchConfig>,
}

/// File metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub size: u64,
    pub modified_at: SystemTime,
    pub created_at: Option<SystemTime>,
    pub permissions: String,
    pub file_type: String,
}

/// Watch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchConfig {
    pub recursive: bool,
    pub events: Vec<String>,
    pub debounce_ms: u64,
    pub max_events_per_second: u32,
}

// ==================== SECTION ====================

/// Installer error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerErrorData {
    /// Installation step that failed
    pub step: String,
    /// Component being installed
    pub component: Option<String>,
    /// Installation progress
    pub progress: Option<f64>,
    /// System requirements
    pub system_requirements: Option<SystemRequirements>,
    /// Installation environment
    pub install_environment: Option<InstallEnvironment>,
}

/// System requirements information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirements {
    pub min_memory_mb: u64,
    pub min_disk_space_mb: u64,
    pub required_features: Vec<String>,
    pub supported_platforms: Vec<String>,
}

/// Installation environment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallEnvironment {
    pub os_version: String,
    pub architecture: String,
    pub available_memory_mb: u64,
    pub available_disk_space_mb: u64,
    pub user_permissions: String,
}

// ==================== SECTION ====================

/// Primal SDK error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalErrorData {
    /// Primal service name
    pub service_name: String,
    /// Primal operation
    pub operation: String,
    /// Primal capability
    pub capability: Option<String>,
    /// Service endpoint
    pub endpoint: Option<String>,
    /// Service version
    pub version: Option<String>,
    /// Request correlation ID
    pub correlation_id: Option<String>,
}

// ==================== SECTION ====================

/// Performance metrics for error context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub disk_io: u64,
    pub network_io: u64,
    pub load_average: Option<[f64; 3]>,
    pub uptime_seconds: u64,
}

// ==================== SECTION ====================

impl std::fmt::Display for ZfsErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZFS operation '{}' failed", self.operation)
    }
}

impl std::fmt::Display for NetworkErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Network protocol '{}' failed", self.protocol)
    }
}

impl std::fmt::Display for SecurityErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Security error - subject: {:?}", self.subject)
    }
}

impl std::fmt::Display for McpErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MCP error - message type: {}", self.message_type)
    }
}

impl std::fmt::Display for ApiErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API error - method: {:?}, path: {:?}", self.method, self.path)
    }
}

impl std::fmt::Display for AutomationErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Automation error - step: {:?}", self.current_step)
    }
}

impl std::fmt::Display for UniversalZfsErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Universal ZFS error - backend: {:?}", self.backend)
    }
}

impl std::fmt::Display for MiddlewareErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Middleware '{}' error at stage '{}'", self.middleware_name, self.stage)
    }
}

impl std::fmt::Display for FsMonitorErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FsMonitor error - operation '{}' on path '{}'", self.operation, self.path)
    }
}

impl std::fmt::Display for InstallerErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Installer error at step '{}'", self.step)
    }
}

impl std::fmt::Display for PrimalErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Primal '{}' operation '{}' failed", self.service_name, self.operation)
    }
} 