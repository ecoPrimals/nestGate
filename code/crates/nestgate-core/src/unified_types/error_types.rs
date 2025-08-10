/// Unified Error Types Module
/// Consolidated error context and reporting structures
/// **PROBLEM SOLVED**: Eliminates fragmented error handling patterns
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Core error information for the unified error context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedErrorCore {
    /// Human-readable error message
    pub message: String,
    /// Error code identifier
    pub error_code: String,
    /// Error severity level
    pub severity: UnifiedErrorSeverity,
    /// Error type classification
    pub error_type: UnifiedErrorType,
    /// Service name where error occurred
    pub service_name: String,
    /// Timestamp when error occurred
    pub timestamp: SystemTime,
    /// Debug message for developers
    pub debug_message: Option<String>,
}

/// Request context information for errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedRequestContext {
    /// Request identifier for tracing
    pub request_id: String,
    /// HTTP method (if applicable)
    pub method: Option<String>,
    /// API endpoint (if applicable)
    pub endpoint: Option<String>,
    /// Request parameters
    pub parameters: HashMap<String, String>,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request processing duration
    pub duration: Option<Duration>,
}

/// System state context at time of error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSystemContext {
    /// System load metrics
    pub system_load: Option<f64>,
    /// Memory usage percentage
    pub memory_usage: Option<f64>,
    /// CPU usage percentage
    pub cpu_usage: Option<f64>,
    /// Disk usage percentage
    pub disk_usage: Option<f64>,
    /// Network connectivity status
    pub network_status: Option<String>,
    /// Active connections count
    pub active_connections: Option<u32>,
    /// System uptime
    pub uptime: Option<Duration>,
}

/// User context information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedUserContext {
    /// User identifier (if authenticated)
    pub user_id: Option<String>,
    /// User session ID
    pub session_id: Option<String>,
    /// Client IP address
    pub client_ip: Option<String>,
    /// User agent string
    pub user_agent: Option<String>,
    /// User's current permissions
    pub permissions: Vec<String>,
    /// User's roles
    pub roles: Vec<String>,
}

/// Error statistics for aggregated error reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedErrorStatistics {
    /// Total error count
    pub total_count: u32,
    /// Error count by status code
    pub errors_by_status: HashMap<u16, u32>,
    /// Error count by endpoint
    pub errors_by_endpoint: HashMap<String, u32>,
    /// Error count by error type
    pub errors_by_type: HashMap<String, u32>,
    /// Error count over time (hourly buckets)
    pub errors_over_time: HashMap<String, u32>,
    /// First occurrence timestamp
    pub first_occurrence: SystemTime,
    /// Last occurrence timestamp
    pub last_occurrence: SystemTime,
}

/// Unified error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnifiedErrorSeverity {
    /// Low impact - informational
    Info,
    /// Medium impact - warning
    Warning,
    /// High impact - error
    Error,
    /// Critical impact - system failure
    Critical,
    /// Catastrophic impact - data loss possible
    Fatal,
}

/// Unified error types for categorization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnifiedErrorType {
    /// Authentication/authorization errors
    Authentication,
    /// Authorization/permission errors
    Authorization,
    /// Input validation errors
    Validation,
    /// Configuration errors
    Configuration,
    /// Network connectivity errors
    Network,
    /// Database operation errors
    Database,
    /// File system operation errors
    FileSystem,
    /// External service errors
    ExternalService,
    /// Timeout errors
    Timeout,
    /// Rate limiting errors
    RateLimit,
    /// Resource exhaustion errors
    ResourceExhaustion,
    /// Internal system errors
    Internal,
    /// Unknown/unclassified errors
    Unknown,
}

/// Comprehensive error context for the unified error system
/// Provides rich debugging information and recovery guidance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedErrorContext {
    /// Core error information
    pub error: UnifiedErrorCore,
    /// Request context (if applicable)
    pub request_context: Option<UnifiedRequestContext>,
    /// System context at time of error
    pub system_context: Option<UnifiedSystemContext>,
    /// User context (if applicable)
    pub user_context: Option<UnifiedUserContext>,
    /// Error statistics
    pub statistics: Option<UnifiedErrorStatistics>,
    /// Recovery suggestions for users/operators
    pub recovery_suggestions: Vec<String>,
    /// Additional metadata for debugging
    pub metadata: HashMap<String, serde_json::Value>,

    // Convenience fields for backward compatibility - directly accessible
    /// Error code (convenience field)
    pub error_code: String,
    /// Error message (convenience field)
    pub message: String,
    /// Service name (convenience field)
    pub service_name: String,
    /// Timestamp (convenience field)
    pub timestamp: SystemTime,
    /// Additional context dictionary (convenience field)
    pub context: HashMap<String, serde_json::Value>,
}

impl Default for UnifiedErrorContext {
    fn default() -> Self {
        Self {
            error: UnifiedErrorCore {
                message: "Unknown error occurred".to_string(),
                error_code: "UNKNOWN_ERROR".to_string(),
                severity: UnifiedErrorSeverity::Error,
                error_type: UnifiedErrorType::Unknown,
                service_name: "unknown".to_string(),
                timestamp: SystemTime::now(),
                debug_message: None,
            },
            request_context: None,
            system_context: None,
            user_context: None,
            statistics: None,
            recovery_suggestions: Vec::new(),
            metadata: HashMap::new(),

            // Initialize convenience fields
            error_code: "UNKNOWN_ERROR".to_string(),
            message: "Unknown error occurred".to_string(),
            service_name: "unknown".to_string(),
            timestamp: SystemTime::now(),
            context: HashMap::new(),
        }
    }
}

impl UnifiedErrorContext {
    /// Create a simple error context with just core error information
    pub fn simple(message: &str, error_code: &str, service_name: &str) -> Self {
        Self {
            error: UnifiedErrorCore {
                message: message.to_string(),
                error_code: error_code.to_string(),
                severity: UnifiedErrorSeverity::Error,
                error_type: UnifiedErrorType::Unknown,
                service_name: service_name.to_string(),
                timestamp: SystemTime::now(),
                debug_message: None,
            },
            error_code: error_code.to_string(),
            message: message.to_string(),
            service_name: service_name.to_string(),
            timestamp: SystemTime::now(),
            context: HashMap::new(),
            ..Default::default()
        }
    }

    /// Create error context with request information
    pub fn with_request(
        mut self,
        request_id: &str,
        method: Option<&str>,
        endpoint: Option<&str>,
    ) -> Self {
        self.request_context = Some(UnifiedRequestContext {
            request_id: request_id.to_string(),
            method: method.map(|m| m.to_string()),
            endpoint: endpoint.map(|e| e.to_string()),
            parameters: HashMap::new(),
            headers: HashMap::new(),
            duration: None,
        });
        self
    }

    /// Add recovery suggestions
    pub fn with_recovery_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.recovery_suggestions = suggestions;
        self
    }

    /// Create a statistics response from the error context
    pub fn to_statistics_response(&self) -> SimpleErrorResponse {
        SimpleErrorResponse {
            error_code: self.error.error_code.clone(),
            message: self.error.message.clone(),
            timestamp: self.error.timestamp,
            request_id: self
                .request_context
                .as_ref()
                .map(|rc| rc.request_id.clone()),
        }
    }

    /// Add additional context to the error
    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }

    /// Create a detailed response from the error context
    pub fn to_detailed_response(&self) -> DetailedErrorResponse {
        DetailedErrorResponse {
            error: self.error.clone(),
            request_context: self.request_context.clone(),
            recovery_suggestions: self.recovery_suggestions.clone(),
            metadata: self.metadata.clone(),
        }
    }

    /// Add metadata for debugging
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self
    }

    /// Convert to simple error response for API responses
    pub fn to_simple_response(&self) -> SimpleErrorResponse {
        SimpleErrorResponse {
            message: self.error.message.clone(),
            error_code: self.error.error_code.clone(),
            timestamp: self.error.timestamp,
            request_id: self
                .request_context
                .as_ref()
                .map(|rc| rc.request_id.clone()),
        }
    }
}

/// Simple error response for public APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleErrorResponse {
    pub message: String,
    pub error_code: String,
    pub timestamp: SystemTime,
    pub request_id: Option<String>,
}

/// Detailed error response for internal APIs and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedErrorResponse {
    pub error: UnifiedErrorCore,
    pub request_context: Option<UnifiedRequestContext>,
    pub recovery_suggestions: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}
