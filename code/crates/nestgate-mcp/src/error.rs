//! Enhanced Error Handling
//!
//! Comprehensive error types with advanced integration error handling
//! with v2 orchestrator error management

use serde::{Deserialize, Serialize};
// Removed unused std import
use std::fmt;
use std::time::SystemTime;

/// Enhanced Error type integrating enhanced NestGate capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
    pub details: Option<String>,
    pub timestamp: SystemTime,
    pub source: Option<String>,
    pub error_code: Option<String>,
}

impl Error {
    /// Create a new error
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
            details: None,
            timestamp: SystemTime::now(),
            source: None,
            error_code: None,
        }
    }

    /// Create a new internal error
    pub fn internal(message: String) -> Self {
        Self::new(ErrorType::InternalError, message)
    }

    /// Create a new network error
    pub fn network(err: impl fmt::Display) -> Self {
        Self::new(ErrorType::Network, format!("Network error: {err}"))
    }

    /// Create a new authentication error
    pub fn authentication(message: String) -> Self {
        Self::new(ErrorType::Auth, message)
    }

    /// Create a new authorization error
    pub fn authorization(message: String) -> Self {
        Self::new(ErrorType::PermissionDenied, message)
    }

    /// Create a new validation error
    pub fn validation(message: String) -> Self {
        Self::new(ErrorType::Validation, message)
    }

    /// Create a new storage error
    pub fn storage(message: String) -> Self {
        Self::new(ErrorType::Storage, message)
    }

    /// Create a new protocol error
    pub fn protocol(message: String) -> Self {
        Self::new(ErrorType::Protocol, message)
    }

    /// Create a new federation error
    pub fn federation(message: String) -> Self {
        Self::new(ErrorType::Network, format!("Federation error: {message}"))
    }

    /// Create a new orchestrator error
    pub fn orchestrator(message: String) -> Self {
        Self::new(
            ErrorType::InternalError,
            format!("Orchestrator error: {message}"),
        )
    }

    /// Create a new service error
    pub fn service(message: String) -> Self {
        Self::new(ErrorType::ServiceUnavailable, message)
    }

    /// Create a new configuration error
    pub fn configuration(message: String) -> Self {
        Self::new(ErrorType::Config, message)
    }

    /// Create a new timeout error
    pub fn timeout(message: String) -> Self {
        Self::new(ErrorType::Timeout, message)
    }

    /// Create a new resource error
    pub fn resource(message: String) -> Self {
        Self::new(ErrorType::NotFound, message)
    }

    /// Create a new unsupported error
    pub fn unsupported(message: String) -> Self {
        Self::new(ErrorType::Unsupported, message)
    }

    /// Create a new invalid payload error
    pub fn invalid_payload(message: &str) -> Self {
        Self::new(ErrorType::InvalidRequest, message.to_string())
    }

    /// Create a new session error
    pub fn session(message: String) -> Self {
        Self::new(ErrorType::Auth, message)
    }

    /// Add details to the error
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    /// Add source information
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// Add error code
    pub fn with_code(mut self, code: String) -> Self {
        self.error_code = Some(code);
        self
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.error_type,
            ErrorType::Network
                | ErrorType::Timeout
                | ErrorType::ServiceUnavailable
                | ErrorType::Connection
                | ErrorType::InternalError // Internal errors can be retryable (e.g., temporary resource issues)
        )
    }

    /// Check if error is permanent (not retryable)
    pub fn is_permanent(&self) -> bool {
        matches!(
            self.error_type,
            ErrorType::Auth
                | ErrorType::PermissionDenied
                | ErrorType::Validation
                | ErrorType::Unsupported
                | ErrorType::Config
                | ErrorType::InvalidRequest
                | ErrorType::NotFound
                | ErrorType::AlreadyExists
        )
    }

    /// Get error severity
    pub fn severity(&self) -> ErrorSeverity {
        self.error_type.default_severity()
    }
}

/// Error types for MCP operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ErrorType {
    /// Connection-related errors
    Connection,
    /// Authentication/authorization errors
    Auth,
    /// Protocol-related errors
    Protocol,
    /// Configuration errors
    Config,
    /// Storage-related errors
    Storage,
    /// Network-related errors
    Network,
    /// Timeout errors
    Timeout,
    /// Resource not found
    NotFound,
    /// Resource already exists
    AlreadyExists,
    /// Insufficient permissions
    PermissionDenied,
    /// Invalid request
    InvalidRequest,
    /// Service unavailable
    ServiceUnavailable,
    /// Internal server error
    InternalError,
    /// Unsupported operation
    Unsupported,
    /// Mount-related errors
    Mount,
    /// Volume-related errors
    Volume,
    /// Filesystem errors
    Filesystem,
    /// Permission errors
    Permission,
    /// Validation errors
    Validation,
    /// Parsing errors
    Parsing,
    /// Serialization errors
    Serialization,
}

impl ErrorType {
    /// Get the default severity for this error type
    pub fn default_severity(&self) -> ErrorSeverity {
        match self {
            ErrorType::Connection => ErrorSeverity::Medium,
            ErrorType::Auth => ErrorSeverity::High,
            ErrorType::Protocol => ErrorSeverity::Medium,
            ErrorType::Config => ErrorSeverity::High,
            ErrorType::Storage => ErrorSeverity::High,
            ErrorType::Network => ErrorSeverity::Medium,
            ErrorType::Timeout => ErrorSeverity::Low,
            ErrorType::NotFound => ErrorSeverity::Low,
            ErrorType::AlreadyExists => ErrorSeverity::Low,
            ErrorType::PermissionDenied => ErrorSeverity::High,
            ErrorType::InvalidRequest => ErrorSeverity::Medium,
            ErrorType::ServiceUnavailable => ErrorSeverity::High,
            ErrorType::InternalError => ErrorSeverity::Critical,
            ErrorType::Unsupported => ErrorSeverity::Low,
            ErrorType::Mount => ErrorSeverity::High,
            ErrorType::Volume => ErrorSeverity::High,
            ErrorType::Filesystem => ErrorSeverity::High,
            ErrorType::Permission => ErrorSeverity::High,
            ErrorType::Validation => ErrorSeverity::Low,
            ErrorType::Parsing => ErrorSeverity::Medium,
            ErrorType::Serialization => ErrorSeverity::Medium,
        }
    }
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ErrorSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium,
    /// High severity - error
    High,
    /// Critical severity - system failure
    Critical,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)?;
        if let Some(details) = &self.details {
            write!(f, " ({details})")?;
        }
        if let Some(source) = &self.source {
            write!(f, " [source: {source}]")?;
        }
        if let Some(code) = &self.error_code {
            write!(f, " [code: {code}]")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

// Implement From traits for common error types

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::internal(format!("IO error: {err}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(ErrorType::Serialization, format!("JSON error: {err}"))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Error::timeout(format!("Request timeout: {err}"))
        } else if err.is_connect() {
            Error::network(err)
        } else {
            Error::internal(format!("HTTP client error: {err}"))
        }
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(err: tokio::time::error::Elapsed) -> Self {
        Error::timeout(format!("Operation timeout: {err}"))
    }
}

/// Use universal MCP result type from nestgate-core  
pub type Result<T> = nestgate_core::McpResult<T>;

/// Error Context for enhanced error tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub operation: String,
    pub component: String,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new(operation: String, component: String) -> Self {
        Self {
            operation,
            component,
            request_id: None,
            user_id: None,
            session_id: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Enhanced Error with Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualError {
    pub error: Error,
    pub context: ErrorContext,
}

impl ContextualError {
    pub fn new(error: Error, context: ErrorContext) -> Self {
        Self { error, context }
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} in {} (operation: {})",
            self.error, self.context.component, self.context.operation
        )?;
        if let Some(request_id) = &self.context.request_id {
            write!(f, " [request_id: {request_id}]")?;
        }
        Ok(())
    }
}

impl std::error::Error for ContextualError {}

/// Error Handler trait for different error handling strategies
pub trait ErrorHandler {
    fn handle_error(&self, error: &Error) -> ErrorHandlingAction;
    fn should_retry(&self, error: &Error, attempt: u32) -> bool;
    fn get_retry_delay(&self, error: &Error, attempt: u32) -> std::time::Duration;
}

/// Error Handling Actions
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorHandlingAction {
    Retry,
    Fallback,
    Escalate,
    Ignore,
    Fail,
}

/// Default Error Handler implementation
pub struct DefaultErrorHandler {
    max_retries: u32,
    base_delay: std::time::Duration,
}

impl DefaultErrorHandler {
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            base_delay: std::time::Duration::from_millis(100),
        }
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_base_delay(mut self, base_delay: std::time::Duration) -> Self {
        self.base_delay = base_delay;
        self
    }
}

impl Default for DefaultErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: &Error) -> ErrorHandlingAction {
        match error.severity() {
            ErrorSeverity::Critical => ErrorHandlingAction::Escalate,
            ErrorSeverity::High => {
                if error.is_retryable() {
                    ErrorHandlingAction::Retry
                } else {
                    ErrorHandlingAction::Fail
                }
            }
            ErrorSeverity::Medium => {
                if error.is_retryable() {
                    ErrorHandlingAction::Retry
                } else {
                    ErrorHandlingAction::Fallback
                }
            }
            ErrorSeverity::Low => {
                if error.is_permanent() {
                    ErrorHandlingAction::Fail
                } else {
                    ErrorHandlingAction::Retry
                }
            }
        }
    }

    fn should_retry(&self, error: &Error, attempt: u32) -> bool {
        attempt < self.max_retries && error.is_retryable()
    }

    fn get_retry_delay(&self, _error: &Error, attempt: u32) -> std::time::Duration {
        // Exponential backoff with jitter
        let delay_ms = self.base_delay.as_millis() * (2_u128.pow(attempt));
        let jitter = fastrand::u64(0..=delay_ms as u64 / 10);
        std::time::Duration::from_millis(delay_ms as u64 + jitter)
    }
}

/// Error Metrics for monitoring and alerting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    pub error_count: u64,
    pub error_rate: f64,
    pub errors_by_type: std::collections::HashMap<ErrorType, u64>,
    pub errors_by_severity: std::collections::HashMap<ErrorSeverity, u64>,
    pub last_error: Option<Error>,
    pub last_updated: SystemTime,
}

impl ErrorMetrics {
    pub fn new() -> Self {
        Self {
            error_count: 0,
            error_rate: 0.0,
            errors_by_type: std::collections::HashMap::new(),
            errors_by_severity: std::collections::HashMap::new(),
            last_error: None,
            last_updated: SystemTime::now(),
        }
    }

    pub fn record_error(&mut self, error: &Error) {
        self.error_count += 1;
        *self
            .errors_by_type
            .entry(error.error_type.clone())
            .or_insert(0) += 1;
        *self.errors_by_severity.entry(error.severity()).or_insert(0) += 1;
        self.last_error = Some(error.clone());
        self.last_updated = SystemTime::now();
    }

    pub fn calculate_error_rate(&mut self, total_requests: u64) {
        if total_requests > 0 {
            self.error_rate = self.error_count as f64 / total_requests as f64 * 100.0;
        }
    }
}

impl Default for ErrorMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = Error::internal("Test error".to_string());
        assert_eq!(error.error_type, ErrorType::InternalError);
        assert_eq!(error.message, "Test error");
        assert!(error.is_retryable());
    }

    #[test]
    fn test_error_severity() {
        let internal_error = Error::internal("Test".to_string());
        assert_eq!(internal_error.severity(), ErrorSeverity::Critical);

        let validation_error = Error::validation("Test".to_string());
        assert_eq!(validation_error.severity(), ErrorSeverity::Low);
    }

    #[test]
    fn test_error_handler() {
        let handler = DefaultErrorHandler::new();
        let error = Error::network("Test".to_string());

        assert_eq!(handler.handle_error(&error), ErrorHandlingAction::Retry);
        assert!(handler.should_retry(&error, 1));
        assert!(!handler.should_retry(&error, 5));
    }

    #[test]
    fn test_error_metrics() {
        let mut metrics = ErrorMetrics::new();
        let error = Error::internal("Test".to_string());

        metrics.record_error(&error);
        assert_eq!(metrics.error_count, 1);
        assert_eq!(metrics.errors_by_type[&ErrorType::InternalError], 1);
        assert_eq!(metrics.errors_by_severity[&ErrorSeverity::Critical], 1);
    }
}
