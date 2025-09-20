/// Domain-Specific Error Data Structures
///
/// This module contains the structured error data for each domain in the NestGate system.
/// All errors flow through the main NestGateError enum in core.rs, but use these structures
/// to provide rich, domain-specific context.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
// ==================== SECTION ====================

/// Automation system error data
/// Consolidates errors from nestgate-automation crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationErrorData {
    /// Error message
    pub message: String,
    /// Automation operation that failed
    /// Target resource (dataset, pool, etc.)
    pub target: Option<String>,
    /// Analysis context if applicable
    pub analysis_context: Option<AnalysisContext>,
    /// Discovery context if applicable
    pub discovery_context: Option<DiscoveryContext>,
    /// Cache context if applicable
    pub cache_context: Option<CacheContext>,
}
/// Automation operations that can fail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationOperation {
    Configuration,
    Discovery,
    Connection,
    FileAnalysis,
    Service,
    Analysis,
    Cache,
    Prediction,
    Optimization,
    Scheduling,
}
/// Analysis context for automation errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisContext {
    /// File or dataset being analyzed
    pub target: String,
    /// Analysis type
    pub analysis_type: String,
    /// Progress when error occurred (0.0-1.0)
    pub progress: f64,
    /// Partial results if available
    pub partial_results: Option<serde_json::Value>,
}
/// Discovery context for automation errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryContext {
    /// Discovery target (endpoint, service, etc.)
    pub target: String,
    /// Discovery method
    pub method: String,
    /// Discovered services before error
    pub discovered_services: Vec<String>,
    /// Last successful discovery time
    pub last_success: Option<SystemTime>,
}
/// Cache context for automation errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheContext {
    /// Cache operation that failed
    /// Cache key
    pub key: Option<String>,
    /// Cache size at time of error
    pub cache_size: Option<usize>,
    /// Cache hit rate
    pub hit_rate: Option<f64>,
}
// ==================== SECTION ====================

/// Middleware system error data
/// Consolidates errors from nestgate-middleware crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareErrorData {
    /// Error message
    pub message: String,
    /// Middleware component that failed
    pub component: MiddlewareComponent,
    /// Request context if applicable
    pub request_context: Option<RequestContext>,
    /// Validation context if applicable
    pub validation_context: Option<ValidationContext>,
    /// Handler context if applicable
    pub handler_context: Option<HandlerContext>,
}
/// Middleware components that can fail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MiddlewareComponent {
    Validator,
    ErrorHandler,
    RequestProcessor,
    ResponseProcessor,
    AuthMiddleware,
    LoggingMiddleware,
    MetricsMiddleware,
    RateLimiter,
    Cors,
    Compression,
}
/// Request context for middleware errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    /// HTTP method
    pub method: String,
    /// Request path
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request ID for tracing
    pub request_id: String,
    /// Client IP address
    pub client_ip: Option<String>,
}
/// Validation context for middleware errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationContext {
    /// Field that failed validation
    pub field: String,
    /// Validation rule that failed
    pub rule: String,
    /// Actual value
    pub actualvalue: Option<String>,
    /// Expected value or pattern
    pub expected: Option<String>,
    /// All validation errors
    pub all_errors: Vec<String>,
}
/// Handler context for middleware errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerContext {
    /// Handler name
    pub handler: String,
    /// Handler chain position
    pub chain_position: usize,
    /// Processing time before error
    pub processing_time: Duration,
    /// Previous handlers in chain
    pub previous_handlers: Vec<String>,
}
// ==================== SECTION ====================

/// File system monitor error data
/// Consolidates errors from nestgate-fsmonitor crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMonitorErrorData {
    /// Error message
    pub message: String,
    /// Monitor operation that failed
    /// File system path
    /// Watch context if applicable
    pub watch_context: Option<WatchContext>,
    /// Event context if applicable
    pub event_context: Option<EventContext>,
}
/// File system monitor operations that can fail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FsMonitorOperation {
    Watch,
    Unwatch,
    EventProcessing,
    PathValidation,
    PermissionCheck,
    EventFiltering,
    NotificationDelivery,
}
/// Watch context for file system monitor errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchContext {
    /// Path being watched
    /// Watch options
    pub options: HashMap<String, serde_json::Value>,
    /// Number of active watches
    pub active_watches: usize,
    /// Watch start time
    pub started_at: SystemTime,
}
/// Event context for file system monitor errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContext {
    /// Event type (create, modify, delete, etc.)
    pub event_type: String,
    /// File path that triggered event
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event queue size
    pub queue_size: Option<usize>,
}
// ==================== SECTION ====================

/// Installer system error data
/// Consolidates errors from nestgate-installer crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerErrorData {
    /// Error message
    pub message: String,
    /// Installation operation that failed
    /// Installation context
    pub install_context: Option<InstallContext>,
    /// System requirements context
    pub requirements_context: Option<RequirementsContext>,
}
/// Installer operations that can fail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstallerOperation {
    PreInstallCheck,
    SystemRequirements,
    DependencyInstall,
    ServiceInstall,
    Configuration,
    PostInstallSetup,
    Validation,
    Rollback,
    Uninstall,
}
/// Installation context for installer errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallContext {
    /// Installation target directory
    pub target_dir: String,
    /// Installation mode (user, system, container)
    pub install_mode: String,
    /// Installation progress (0.0-1.0)
    pub progress: f64,
    /// Completed steps
    pub completed_steps: Vec<String>,
    /// Failed step
    pub failed_step: Option<String>,
}
/// System requirements context for installer errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementsContext {
    /// Operating system
    pub os: String,
    /// OS version
    pub os_version: String,
    /// Architecture
    pub arch: String,
    /// Available memory (bytes)
    pub available_memory: u64,
    /// Available disk space (bytes)
    pub available_disk: u64,
    /// Missing requirements
    pub missing_requirements: Vec<String>,
}
// ==================== SECTION ====================

/// ZFS system error data
/// Consolidates errors from nestgate-zfs crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsErrorData {
    /// Error message
    pub message: String,
    /// ZFS operation that failed
    /// Pool name if applicable
    pub pool: Option<String>,
    /// Dataset name if applicable
    pub dataset: Option<String>,
    /// Snapshot name if applicable
    pub snapshot: Option<String>,
    /// ZFS command that failed
    pub command: Option<String>,
    /// System error code if available
    pub error_code: Option<i32>,
    /// Recovery suggestions
}
/// ZFS operations that can fail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsOperation {
    PoolCreate,
    PoolDestroy,
    PoolImport,
    PoolExport,
    DatasetCreate,
    DatasetDestroy,
    DatasetMount,
    DatasetUnmount,
    SnapshotCreate,
    SnapshotDestroy,
    SnapshotClone,
    Migration,
    Performance,
    Configuration,
    SystemCheck,
    Permission,
    Network,
    Storage,
    Command,
}
// ==================== SECTION ====================

/// Primal SDK error data
/// Consolidates errors from ecoprimal SDK
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalErrorData {
    /// Error message
    pub message: String,
    /// Primal operation that failed
    /// Primal ID if applicable
    pub primal_id: Option<String>,
    /// Request context if applicable
    pub request_context: Option<PrimalRequestContext>,
    /// Capability context if applicable
    pub capability: Option<String>,
    /// Metadata for debugging
    pub metadata: HashMap<String, String>,
}
/// Primal operations that can fail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalOperation {
    Configuration,
    Initialization,
    RequestProcessing,
    Resource,
    Network,
    Authentication,
    Authorization,
    Timeout,
    Internal,
    ExternalDependency,
    Validation,
    NotFound,
    Conflict,
    RateLimit,
    ServiceUnavailable,
}
/// Primal request context for error debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequestContext {
    /// Request ID
    pub request_id: String,
    /// Operation being performed
    /// Parameters passed
    pub parameters: HashMap<String, String>,
    /// Timestamp of request
    pub timestamp: SystemTime,
}
// ==================== SECTION ====================

/// Universal ZFS error data
/// Enhanced ZFS errors with universal adapter context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalZfsErrorData {
    /// Error message
    pub message: String,
    /// Universal ZFS operation
    /// Backend that failed
    pub backend: Option<String>,
    /// Resource involved
    /// Timeout duration if applicable
    pub duration: Option<Duration>,
    /// Circuit breaker state
    pub circuit_breaker_open: bool,
    /// Rate limit information
    pub rate_limit_info: Option<RateLimitInfo>,
}
/// Rate limiting information for errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {
    /// Current limit
    pub limit: u32,
    /// Time window
    pub window: Duration,
    /// Current usage
    pub current_usage: u32,
    /// Reset time
    pub reset_time: SystemTime,
}
// ==================== SECTION ====================

/// Network system error data
/// Consolidates errors from network operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkErrorData {
    /// Error message
    pub message: String,
    /// Network operation that failed
    /// Network endpoint if applicable
    pub endpoint: Option<String>,
    /// Additional context for debugging
    pub context: Option<HashMap<String, String>>,
}
// ==================== SECTION ====================

/// MCP (Model Context Protocol) error data
/// Consolidates errors from MCP operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpErrorData {
    /// Error message
    pub message: String,
    /// MCP operation that failed
    /// Session ID if applicable
    pub session_id: Option<String>,
    /// Additional context for debugging
    pub context: Option<HashMap<String, String>>,
}
// ==================== SECTION ====================

/// API system error data
/// Consolidates errors from API operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorData {
    /// Error message
    pub message: String,
    /// HTTP method
    pub method: Option<String>,
    /// Request path
    /// HTTP status code
    pub status_code: Option<u16>,
    /// Additional context for debugging
    pub context: Option<HashMap<String, String>>,
}
// ==================== SECTION ====================

/// Security system error data
/// Consolidates errors from security operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityErrorData {
    /// Error message
    pub message: String,
    /// Security operation (auth, authz, etc.)
    /// Resource being accessed
    /// User or principal
    pub principal: Option<String>,
    /// Additional context for debugging
    pub context: Option<HashMap<String, String>>,
}
// ==================== SECTION ====================

impl From<AutomationErrorData> for crate::error::NestGateError {
    fn from(data: AutomationErrorData) -> Self {
        crate::error::NestGateError::Automation(Box::new(data))
    }
}

impl From<MiddlewareErrorData> for crate::error::NestGateError {
    fn from(data: MiddlewareErrorData) -> Self {
        crate::error::NestGateError::Middleware(Box::new(data))
    }
}

impl From<FsMonitorErrorData> for crate::error::NestGateError {
    fn from(data: FsMonitorErrorData) -> Self {
        crate::error::NestGateError::FsMonitor(Box::new(data))
    }
}

impl From<InstallerErrorData> for crate::error::NestGateError {
    fn from(data: InstallerErrorData) -> Self {
        crate::error::NestGateError::Installer(Box::new(data))
    }
}

impl From<UniversalZfsErrorData> for crate::error::NestGateError {
    fn from(data: UniversalZfsErrorData) -> Self {
        crate::error::NestGateError::UniversalZfs(Box::new(data))
    }
}

// ==================== SECTION ====================

impl std::fmt::Display for ZfsErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZFS Error: {}", self.message)
    }
}

impl std::fmt::Display for PrimalErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Primal Error: {}", self.message)
    }
}

impl std::fmt::Display for UniversalZfsErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Universal ZFS Error: {}", self.message)
    }
}

impl std::fmt::Display for AutomationErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Automation Error: {}", self.message)
    }
}

impl std::fmt::Display for MiddlewareErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Middleware Error: {}", self.message)
    }
}

impl std::fmt::Display for FsMonitorErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File System Monitor Error: {}", self.message)
    }
}

impl std::fmt::Display for InstallerErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Installer Error: {}", self.message)
    }
}

impl std::fmt::Display for NetworkErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Network Error: {}", self.message)
    }
}

impl std::fmt::Display for ApiErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API Error: {}", self.message)
    }
}

impl std::fmt::Display for SecurityErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Security Error: {}", self.message)
    }
}

impl std::fmt::Display for McpErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MCP Error: {}", self.message)
    }
}

// Note: Removed conflicting boxed Display implementations
// Rust automatically implements Display for Box<T> when T implements Display
