/// Core Error Types and Architecture
/// This module contains the main NestGateError enum and boxed data structures
/// for the unified error handling system.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use thiserror::Error;

// Import domain error data types from the data module
use super::data::{
    NetworkErrorData, SecurityErrorData, McpErrorData, 
    ApiErrorData, AutomationErrorData, UniversalZfsErrorData,
    MiddlewareErrorData, FsMonitorErrorData, InstallerErrorData, PrimalErrorData,
};

/// Retry configuration information for error handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryInfo {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Whether exponential backoff should be used
    pub exponential_backoff: bool,
}

/// Generic error context for additional debugging information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Operation being performed when error occurred
    pub operation: String,
    /// Component or module where error occurred
    pub component: String,
    /// Additional key-value context
    pub metadata: HashMap<String, String>,
    /// Timestamp when error occurred
    pub timestamp: SystemTime,
}

impl ErrorContext {
    /// Create a new error context
    pub fn new(component: &str, operation: &str) -> Self {
        Self {
            operation: operation.to_string(),
            component: component.to_string(),
            metadata: HashMap::new(),
            timestamp: SystemTime::now(),
        }
    }
    
    /// Add metadata to the error context
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedConfigSource {
    File(String),
    Environment,
    Defaults,
    CommandLine,
    Database,
    UserProvided,
    Runtime,
    Builder(String),
    Validation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemResource {
    Memory,
    Disk,
    Cpu,
    Network,
    FileDescriptors,
    Threads,
    FileSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy {
    Retry,
    Fallback,
    ScaleUp,
    WaitAndRetry,
}

/// Test assertion failure details for rich debugging context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestAssertionDetails {
    /// What was expected
    pub expected: String,
    /// What was actually received
    pub current_value: String,
    /// Assertion description
    pub description: String,
}

/// Test error data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestErrorData {
    /// Test operation that failed (setup, assertion, operation, etc.)
    pub operation: String,
    /// Error message
    pub message: String,
    /// Test context and environment
    pub test_context: Option<String>,
    /// Expected vs actual for assertion failures
    pub assertion_details: Option<TestAssertionDetails>,
    /// Test file and line location
    pub location: Option<String>,
    /// Whether this indicates a test framework bug
    pub is_framework_bug: bool,
}

impl std::fmt::Display for TestErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.operation, self.message)
    }
}

/// **LEGACY ERROR SYSTEM**
/// This is the legacy error type for NestGate operations. 
/// **DEPRECATED**: Use NestGateUnifiedError from unified.rs instead.
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum NestGateLegacyError {
    // ===== DOMAIN-SPECIFIC ERRORS =====
    /// ZFS storage system errors
    #[error("ZFS Error: {0}")]
    Zfs(Box<UniversalZfsErrorData>),

    /// Network and communication errors
    #[error("Network Error: {0}")]
    Network(Box<NetworkErrorData>),

    /// MCP (Model Context Protocol) errors
    #[error("MCP Error: {0}")]
    Mcp(Box<McpErrorData>),

    /// API and HTTP service errors
    #[error("API Error: {0}")]
    Api(Box<ApiErrorData>),

    /// Authentication and authorization errors
    #[error("Security Error: {0}")]
    Security(Box<SecurityErrorData>),

    /// **TEST FRAMEWORK ERRORS**
    /// Testing-related errors with rich context for debugging test failures
    #[error("Test Error: {0}")]
    Testing(Box<TestErrorData>),

    /// **AUTOMATION SYSTEM ERRORS**
    /// Automation-related errors with rich context for debugging automation failures
    #[error("Automation Error: {0}")]
    Automation(Box<AutomationErrorData>),

    /// **MIDDLEWARE ERRORS**
    /// Middleware-related errors with rich context for debugging middleware failures
    #[error("Middleware Error: {0}")]
    Middleware(Box<MiddlewareErrorData>),

    /// **FILE SYSTEM MONITOR ERRORS**
    /// File system monitoring errors with rich context for debugging watch failures
    #[error("FsMonitor Error: {0}")]
    FsMonitor(Box<FsMonitorErrorData>),

    /// **INSTALLER ERRORS**
    /// Installation and setup errors with rich context for debugging install failures
    #[error("Installer Error: {0}")]
    Installer(Box<InstallerErrorData>),

    /// **UNIVERSAL ZFS ERRORS**
    /// Enhanced ZFS errors with rich context for debugging ZFS operations
    #[error("Universal ZFS Error: {0}")]
    UniversalZfs(Box<UniversalZfsErrorData>),

    /// **PRIMAL SDK ERRORS**
    /// EcoPrimal SDK errors with rich context for debugging primal operations
    #[error("Primal Error: {0}")]
    Primal(Box<PrimalErrorData>),

    /// Configuration and initialization errors
    #[error("Configuration Error: {message}")]
    Configuration {
        message: String,
        /// Field name or section that caused the error
        field: Option<String>,
    },

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

    /// Validation errors
    #[error("Validation Error: {field} - {message}")]
    Validation {
        field: String,
        message: String,
        current_value: Option<String>,
        expected: Option<String>,
        user_error: bool,
    },

    /// Access denied error
    #[error("Access Denied: {reason}")]
    AccessDenied { reason: String },

    /// License required for corporate usage
    #[error("License Required: {message}")]
    LicenseRequired { message: String },

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

    /// Internal system errors (unexpected conditions)
    #[error("Internal Error: {message}")]
    Internal {
        message: String,
        location: Option<String>,
        debug_info: Option<String>,
        is_bug: bool,
    },

    /// Storage-related errors
    #[error("Storage Error: {operation} - {details}")]
    Storage { operation: String, details: String },

    /// Service discovery and capability matching errors
    #[error("Service Discovery Error: {message} (Requirements: {requirements})")]
    ServiceDiscovery {
        message: String,
        requirements: String,
    },

    /// Authentication and authorization failures
    #[error("Unauthorized: {message}")]
    Unauthorized {
        message: String,
        location: Option<String>,
    },

    /// Resource not found errors
    #[error("Not Found: {0}")]
    NotFound(String),

    /// Load balancer configuration and operation errors
    #[error("Load Balancer Error: {message}")]
    LoadBalancer {
        message: String,
        location: Option<String>,
    },

    /// Feature not yet implemented
    #[error("Not Implemented: {feature}")]
    NotImplemented {
        feature: String,
        location: Option<String>,
    },
}

// Add From implementation for serde_json::Error
impl From<serde_json::Error> for NestGateLegacyError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation {
            field: "JSON".to_string(),
            message: format!("JSON serialization error: {err}"),
            current_value: None,
            expected: Some("Valid JSON format".to_string()),
            user_error: false,
        }
    }
}
