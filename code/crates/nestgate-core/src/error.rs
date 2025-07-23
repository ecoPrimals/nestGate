use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
/// # NestGate Unified Error Architecture
///
/// **THE DEFINITIVE ERROR HANDLING SYSTEM**
///
/// This module provides the comprehensive, unified error handling architecture for the entire
/// NestGate ecosystem. It eliminates error type duplication, provides rich error context,
/// implements proper error chains, and establishes consistent error handling patterns.
///
/// ## Design Principles
///
/// - **Single Source of Truth**: All error types defined in one place
/// - **Rich Context**: Every error carries structured context information
/// - **Error Chains**: Proper cause tracking and error propagation
/// - **Recovery Strategies**: Each error provides recovery guidance
/// - **Consistent Formatting**: Standardized error messages and logging
/// - **Zero Crashes**: No unwrap/panic patterns in production code
// Removed unused std import
use thiserror::Error;
// Removed unused tracing import

// ==================== CORE ERROR ARCHITECTURE ====================

// ==================== BOXED ERROR DATA STRUCTURES ====================

/// Boxed data for large ZFS error variants
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("{error}")]
pub struct ZfsErrorData {
    pub error: ZfsError,
    /// Additional context for the ZFS operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
}

/// Boxed data for large Network error variants  
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("{error}")]
pub struct NetworkErrorData {
    pub error: NetworkError,
    /// Network-specific context (endpoint, retry count, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
}

/// Boxed data for large MCP error variants
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("{error}")]
pub struct McpErrorData {
    pub error: McpError,
    /// MCP session and protocol context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
}

/// Boxed data for large API error variants
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("{error}")]
pub struct ApiErrorData {
    pub error: ApiError,
    /// HTTP request/response context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ErrorContext>,
}

/// Boxed data for large Security error variants
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[error("{error}")]
pub struct SecurityErrorData {
    pub error: SecurityError,
    /// Security context (user, operation, resource)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<SecurityContext>,
}

/// **THE UNIVERSAL NESTGATE ERROR TYPE**
///
/// This is the root error type for all NestGate operations. It provides comprehensive
/// error classification, rich context, and structured recovery information.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum NestGateError {
    // ===== DOMAIN-SPECIFIC ERRORS =====
    /// ZFS storage system errors
    #[error("ZFS Error: {0}")]
    Zfs(Box<ZfsErrorData>),

    /// Network and communication errors
    #[error("Network Error: {0}")]
    Network(Box<NetworkErrorData>),

    /// MCP (Model Context Protocol) errors
    #[error("MCP Error: {0}")]
    Mcp(Box<McpErrorData>),

    /// API and HTTP service errors
    #[error("API Error: {0}")]
    Api(Box<ApiErrorData>),

    /// Configuration and initialization errors  
    #[error("Configuration Error: {message}")]
    Configuration {
        message: String,
        /// Configuration source (file, environment, defaults)
        config_source: ConfigSource,
        /// Field name or section that caused the error
        field: Option<String>,
        /// Suggested fix for the configuration issue
        suggested_fix: Option<String>,
    },

    /// Authentication and authorization errors
    #[error("Security Error: {0}")]
    Security(Box<SecurityErrorData>),

    /// System resource and infrastructure errors
    #[error("System Error: {message}")]
    System {
        message: String,
        /// System resource type (memory, disk, CPU, etc.)
        resource: SystemResource,
        /// Current resource utilization
        utilization: Option<f64>,
        /// Recovery strategy
        recovery: RecoveryStrategy,
    },

    /// I/O and filesystem errors
    #[error("I/O Error: {operation} - {error_message}")]
    Io {
        operation: String,
        error_message: String,
        /// File path or resource identifier
        resource: Option<String>,
        /// Whether operation is retryable
        retryable: bool,
    },

    /// Data validation and parsing errors
    #[error("Validation Error: {field} - {message}")]
    Validation {
        field: String,
        message: String,
        /// Current value that failed validation
        current_value: Option<String>,
        /// Expected format or constraints
        expected: Option<String>,
        /// Whether this is a user input error
        user_error: bool,
    },

    /// Timeout and performance errors
    #[error("Timeout Error: {operation} timed out after {duration:?}")]
    Timeout {
        operation: String,
        duration: Duration,
        /// Whether operation might succeed with more time
        retryable: bool,
        /// Suggested timeout value for retry
        suggested_timeout: Option<Duration>,
    },

    /// Resource exhaustion errors
    #[error("Resource Exhausted: {resource} - {current}/{limit}")]
    ResourceExhausted {
        resource: String,
        current: u64,
        limit: u64,
        /// How long to wait before retry
        retry_after: Option<Duration>,
        /// Scaling suggestions
        scaling_suggestion: Option<String>,
    },

    /// Service dependency errors
    #[error("Dependency Error: {service} - {message}")]
    Dependency {
        service: String,
        message: String,
        /// Service endpoint or identifier
        endpoint: Option<String>,
        /// Whether service might recover
        recoverable: bool,
        /// Circuit breaker state
        circuit_breaker_open: bool,
    },

    /// Feature not implemented or available
    #[error("Feature Unavailable: {feature} - {reason}")]
    FeatureUnavailable {
        feature: String,
        reason: String,
        /// When feature might become available
        available_in: Option<String>,
        /// Alternative features or workarounds
        alternatives: Vec<String>,
    },

    /// Internal logic errors (should never happen in production)
    #[error("Internal Error: {message}")]
    Internal {
        message: String,
        /// Source location (file:line)
        location: Option<String>,
        /// Additional debugging information
        debug_info: Option<String>,
        /// Whether this indicates a bug
        is_bug: bool,
    },
}

// ==================== DOMAIN-SPECIFIC ERROR TYPES ====================

/// **ZFS Storage System Errors**
///
/// Comprehensive error types for all ZFS operations including pools, datasets,
/// snapshots, migration, and performance monitoring.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ZfsError {
    /// Pool-related operations
    #[error("Pool Error: {operation} on '{pool}' - {message}")]
    Pool {
        operation: ZfsOperation,
        pool: String,
        message: String,
        health: Option<PoolHealth>,
    },

    /// Dataset operations
    #[error("Dataset Error: {operation} on '{dataset}' - {message}")]
    Dataset {
        operation: ZfsOperation,
        dataset: String,
        message: String,
        properties: Option<DatasetProperties>,
    },

    /// Snapshot management
    #[error("Snapshot Error: {operation} on '{snapshot}' - {message}")]
    Snapshot {
        operation: ZfsOperation,
        snapshot: String,
        message: String,
        size: Option<u64>,
    },

    /// Data migration and tiering
    #[error("Migration Error: {operation} - {message}")]
    Migration {
        operation: String,
        message: String,
        source_tier: Option<String>,
        target_tier: Option<String>,
        progress: Option<f64>,
    },

    /// Performance and monitoring
    #[error("Performance Error: {metric} - {message}")]
    Performance {
        metric: String,
        message: String,
        current_value: Option<f64>,
        threshold: Option<f64>,
    },

    /// ZFS command execution failures
    #[error("ZFS Command Failed: {command} - {stderr}")]
    CommandFailed {
        command: String,
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
}

/// **Network and Communication Errors**
///
/// Errors related to network operations, HTTP requests, and inter-service communication.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum NetworkError {
    /// Connection establishment failures
    #[error("Connection Error: {endpoint} - {message}")]
    Connection {
        endpoint: String,
        message: String,
        retry_count: u32,
        last_attempt: SystemTime,
    },

    /// Request/response errors
    #[error("Request Error: {method} {url} - {}", status.as_ref().map(|s| s.to_string()).unwrap_or_else(|| "No Status".to_string()))]
    Request {
        method: String,
        url: String,
        status: Option<u16>,
        body: Option<String>,
    },

    /// Protocol-specific errors
    #[error("Protocol Error: {protocol} - {message}")]
    Protocol {
        protocol: String,
        message: String,
        version: Option<String>,
    },

    /// DNS and service discovery
    #[error("DNS Error: {hostname} - {message}")]
    Dns { hostname: String, message: String },

    /// TLS/SSL certificate issues
    #[error("TLS Error: {message}")]
    Tls {
        message: String,
        certificate_expired: bool,
        certificate_invalid: bool,
    },
}

/// **MCP (Model Context Protocol) Errors**
///
/// Errors specific to MCP operations, sessions, and protocol handling.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum McpError {
    /// Session management errors
    #[error("Session Error: {session_id} - {message}")]
    Session {
        session_id: String,
        message: String,
        state: SessionState,
    },

    /// Protocol violations
    #[error("Protocol Violation: {message}")]
    ProtocolViolation {
        message: String,
        expected: String,
        received: String,
    },

    /// Security and authentication
    #[error("MCP Security Error: {message}")]
    Security {
        message: String,
        token_invalid: bool,
        permission_denied: bool,
    },

    /// Resource management
    #[error("MCP Resource Error: {resource} - {message}")]
    Resource {
        resource: String,
        message: String,
        quota_exceeded: bool,
    },
}

/// **API and HTTP Service Errors**
///
/// REST API, HTTP request/response, and web service errors with proper HTTP status mapping.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ApiError {
    /// Client request errors (4xx)
    #[error("Bad Request: {message}")]
    BadRequest {
        message: String,
        field: Option<String>,
        validation_errors: Vec<String>,
    },

    /// Authentication errors (401)
    #[error("Unauthorized: {message}")]
    Unauthorized {
        message: String,
        auth_method: Option<String>,
    },

    /// Authorization errors (403)
    #[error("Forbidden: {operation} on {resource}")]
    Forbidden {
        operation: String,
        resource: String,
        required_permission: Option<String>,
    },

    /// Resource not found (404)
    #[error("Not Found: {resource_type} '{resource_id}'")]
    NotFound {
        resource_type: String,
        resource_id: String,
    },

    /// Method not allowed (405)
    #[error("Method Not Allowed: {method} not supported for {endpoint}")]
    MethodNotAllowed {
        method: String,
        endpoint: String,
        allowed_methods: Vec<String>,
    },

    /// Request rate limiting (429)
    #[error("Rate Limited: {limit} requests per {window:?}")]
    RateLimited {
        limit: u32,
        window: Duration,
        retry_after: Duration,
    },

    /// Server errors (5xx)
    #[error("Internal Server Error: {message}")]
    InternalServerError {
        message: String,
        request_id: Option<String>,
    },

    /// Service unavailable (503)
    #[error("Service Unavailable: {service} - {message}")]
    ServiceUnavailable {
        service: String,
        message: String,
        retry_after: Option<Duration>,
    },
}

/// **Security and Authentication Errors**
///
/// Authentication, authorization, and security-related errors with proper context.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SecurityError {
    /// Authentication failures
    #[error("Authentication Failed: {reason}")]
    AuthenticationFailed {
        reason: String,
        auth_method: String,
        user: Option<String>,
    },

    /// Authorization failures
    #[error("Authorization Denied: {user} cannot {action} on {resource}")]
    AuthorizationDenied {
        user: String,
        action: String,
        resource: String,
        required_role: Option<String>,
    },

    /// Token and session errors
    #[error("Token Error: {message}")]
    TokenError {
        message: String,
        token_expired: bool,
        token_invalid: bool,
    },

    /// Cryptographic errors
    #[error("Crypto Error: {operation} - {message}")]
    CryptoError { operation: String, message: String },

    /// Security policy violations
    #[error("Security Policy Violation: {policy} - {message}")]
    PolicyViolation {
        policy: String,
        message: String,
        severity: SecuritySeverity,
    },
}

// ==================== ERROR CONTEXT AND METADATA ====================

/// **Rich Error Context**
///
/// Additional context information that can be attached to any error for better
/// debugging and monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Unique request/operation identifier
    pub request_id: Option<String>,
    /// User or system actor
    pub actor: Option<String>,
    /// Operation being performed
    pub operation: Option<String>,
    /// Resource being acted upon
    pub resource: Option<String>,
    /// Timestamp when error occurred
    pub timestamp: SystemTime,
    /// Additional key-value metadata
    pub metadata: std::collections::HashMap<String, String>,
    /// Stack trace or call chain
    pub trace: Option<Vec<String>>,
}

/// **Security-Specific Context**
///
/// Security-related context for authentication and authorization errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub source_ip: Option<String>,
    pub user_agent: Option<String>,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
}

// ==================== SUPPORTING ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsOperation {
    Pool(String),
    Dataset(String),
    Snapshot(String),
    Volume(String),
}

impl std::fmt::Display for ZfsOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZfsOperation::Pool(name) => write!(f, "pool '{name}'"),
            ZfsOperation::Dataset(name) => write!(f, "dataset '{name}'"),
            ZfsOperation::Snapshot(name) => write!(f, "snapshot '{name}'"),
            ZfsOperation::Volume(name) => write!(f, "volume '{name}'"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolHealth {
    Online,
    Degraded,
    Faulted,
    Offline,
    Removed,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetProperties {
    pub compression: String,
    pub mountpoint: String,
    pub quota: Option<u64>,
    pub reservation: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigSource {
    File(String),
    Environment,
    Defaults,
    CommandLine,
    Database,
    UserProvided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemResource {
    Memory,
    Disk,
    Cpu,
    Network,
    FileDescriptors,
    Threads,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    Retry,
    Fallback,
    ScaleUp,
    WaitAndRetry,
    ManualIntervention,
    Ignore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionState {
    Active,
    Expired,
    Terminated,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

// ==================== RESULT TYPE UNIFICATION ====================

/// **THE UNIVERSAL NESTGATE RESULT TYPE**
///
/// This is the single Result type used throughout the entire NestGate ecosystem.
/// No more conflicting Result type definitions!
pub type Result<T> = std::result::Result<T, NestGateError>;

// Specific domain result types for convenience
pub type ZfsResult<T> = std::result::Result<T, ZfsError>;
pub type NetworkResult<T> = std::result::Result<T, NetworkError>;
pub type McpResult<T> = std::result::Result<T, McpError>;
pub type ApiResult<T> = std::result::Result<T, ApiError>;
pub type SecurityResult<T> = std::result::Result<T, SecurityError>;

// ==================== ERROR CONTEXT BUILDERS ====================

impl Default for ErrorContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorContext {
    /// Create a new error context with timestamp
    pub fn new() -> Self {
        Self {
            request_id: None,
            actor: None,
            operation: None,
            resource: None,
            timestamp: SystemTime::now(),
            metadata: std::collections::HashMap::new(),
            trace: None,
        }
    }

    /// Builder pattern for rich error context
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    pub fn with_actor(mut self, actor: String) -> Self {
        self.actor = Some(actor);
        self
    }

    pub fn with_operation(mut self, operation: String) -> Self {
        self.operation = Some(operation);
        self
    }

    pub fn with_resource(mut self, resource: String) -> Self {
        self.resource = Some(resource);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

// ==================== HELPER MACROS FOR SAFE ERROR HANDLING ====================

/// **SAFE UNWRAP MACRO**
///
/// Replaces `.unwrap()` with proper error handling and context.
#[macro_export]
macro_rules! safe_unwrap {
    ($expr:expr, $msg:expr) => {
        $expr.ok_or_else(|| NestGateError::Internal {
            message: $msg.to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: None,
            is_bug: true,
        })
    };
}

/// **SAFE EXPECT MACRO**  
///
/// Replaces `.expect()` with proper error handling and context.
#[macro_export]
macro_rules! safe_expect {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                return Err(NestGateError::Internal {
                    message: format!("{}: {}", $msg, e),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: Some(format!("{:?}", e)),
                    is_bug: false,
                })
            }
        }
    };
}

// ==================== ERROR CONVERSION IMPLEMENTATIONS ====================

impl From<std::io::Error> for NestGateError {
    fn from(error: std::io::Error) -> Self {
        NestGateError::Io {
            operation: "unknown".to_string(),
            error_message: error.to_string(),
            resource: None,
            retryable: true,
        }
    }
}

impl From<serde_json::Error> for NestGateError {
    fn from(error: serde_json::Error) -> Self {
        NestGateError::Validation {
            field: "json".to_string(),
            message: error.to_string(),
            current_value: None,
            expected: Some("valid JSON".to_string()),
            user_error: false,
        }
    }
}

// ==================== NESTGATE ERROR CONSTRUCTORS ====================

impl NestGateError {
    /// Create a new ZFS error with context
    pub fn zfs(error: ZfsError, context: Option<ErrorContext>) -> Self {
        Self::Zfs(Box::new(ZfsErrorData { error, context }))
    }

    /// Create a new ZFS error without context
    pub fn zfs_simple(error: ZfsError) -> Self {
        Self::zfs(error, None)
    }

    /// Create a new Network error with context
    pub fn network(error: NetworkError, context: Option<ErrorContext>) -> Self {
        Self::Network(Box::new(NetworkErrorData { error, context }))
    }

    /// Create a new Network error without context
    pub fn network_simple(error: NetworkError) -> Self {
        Self::network(error, None)
    }

    /// Create a new MCP error with context
    pub fn mcp(error: McpError, context: Option<ErrorContext>) -> Self {
        Self::Mcp(Box::new(McpErrorData { error, context }))
    }

    /// Create a new MCP error without context
    pub fn mcp_simple(error: McpError) -> Self {
        Self::mcp(error, None)
    }

    /// Create a new API error with context
    pub fn api(error: ApiError, context: Option<ErrorContext>) -> Self {
        Self::Api(Box::new(ApiErrorData { error, context }))
    }

    /// Create a new API error without context
    pub fn api_simple(error: ApiError) -> Self {
        Self::api(error, None)
    }

    /// Create a new Security error with context
    pub fn security(error: SecurityError, context: Option<SecurityContext>) -> Self {
        Self::Security(Box::new(SecurityErrorData { error, context }))
    }

    /// Create a new Security error without context
    pub fn security_simple(error: SecurityError) -> Self {
        Self::security(error, None)
    }
}

// ==================== HTTP STATUS CODE MAPPING ====================

impl ApiError {
    /// Get the appropriate HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            ApiError::BadRequest { .. } => 400,
            ApiError::Unauthorized { .. } => 401,
            ApiError::Forbidden { .. } => 403,
            ApiError::NotFound { .. } => 404,
            ApiError::MethodNotAllowed { .. } => 405,
            ApiError::RateLimited { .. } => 429,
            ApiError::InternalServerError { .. } => 500,
            ApiError::ServiceUnavailable { .. } => 503,
        }
    }
}

impl From<reqwest::Error> for NestGateError {
    fn from(error: reqwest::Error) -> Self {
        Self::Network(Box::new(NetworkErrorData {
            error: NetworkError::Request {
                method: "UNKNOWN".to_string(),
                url: error.url().map(|u| u.to_string()).unwrap_or_default(),
                status: error.status().map(|s| s.as_u16()),
                body: None,
            },
            context: Some(ErrorContext {
                request_id: None,
                actor: None,
                operation: Some("HTTP request".to_string()),
                resource: error.url().map(|u| u.to_string()),
                timestamp: SystemTime::now(),
                metadata: {
                    let mut metadata = std::collections::HashMap::new();
                    metadata.insert("error_source".to_string(), "reqwest".to_string());
                    metadata.insert("error_details".to_string(), error.to_string());
                    metadata
                },
                trace: Some(vec![format!("{}:{}", file!(), line!())]),
            }),
        }))
    }
}
