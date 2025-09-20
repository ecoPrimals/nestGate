// **CORE ERROR TYPES**
//! Core system error types and handling for the `NestGate` system.
// The main NestGateUnifiedError enum and core error handling.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

use super::super::context::ErrorContext;
use super::super::data::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// **THE** definitive `NestGate` error type - single source of truth for all errors
///
/// This enum is designed to be small in memory by boxing all the large variants.
/// This eliminates the `clippy::result_large_err` warnings while maintaining full functionality.
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum NestGateUnifiedError {
    /// Configuration-related errors (boxed for size efficiency)
    #[error("Configuration error: {0}")]
    Configuration(Box<ConfigurationErrorDetails>),

    /// API-related errors (boxed for size efficiency)
    #[error("API error: {0}")]
    Api(Box<ApiErrorDetails>),

    /// Storage and ZFS errors (boxed for size efficiency)
    #[error("Storage error: {0}")]
    Storage(Box<StorageErrorDetails>),

    /// Network and communication errors (boxed for size efficiency)
    #[error("Network error: {0}")]
    Network(Box<NetworkErrorDetails>),

    /// Security and authentication errors (boxed for size efficiency)
    #[error("Security error: {0}")]
    Security(Box<SecurityErrorDetails>),

    /// Automation system errors (boxed for size efficiency)
    #[error("Automation error: {0}")]
    Automation(Box<AutomationErrorDetails>),

    /// System resource and internal errors (boxed for size efficiency)
    #[error("System error: {0}")]
    System(Box<SystemErrorDetails>),

    /// Internal processing errors (boxed for size efficiency)
    #[error("Internal error: {0}")]
    Internal(Box<InternalErrorDetails>),

    /// External dependency errors (boxed for size efficiency)
    #[error("External error: {0}")]
    External(Box<ExternalErrorDetails>),

    /// Validation errors (boxed for size efficiency)
    #[error("Validation error: {0}")]
    Validation(Box<ValidationErrorDetails>),

    /// Timeout errors (boxed for size efficiency)
    #[error("Timeout error: {0}")]
    Timeout(Box<TimeoutErrorDetails>),

    /// I/O operation errors (boxed for size efficiency)
    #[error("I/O error: {0}")]
    Io(Box<IoErrorDetails>),

    /// Resource exhaustion errors (boxed for size efficiency)
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(Box<ResourceExhaustedErrorDetails>),

    /// Testing framework errors (boxed for size efficiency)
    #[error("Testing error: {0}")]
    Testing(Box<TestingErrorDetails>),

    /// Performance and benchmarking errors (boxed for size efficiency)
    #[error("Performance error: {0}")]
    Performance(Box<PerformanceErrorDetails>),

    /// Handler execution errors (boxed for size efficiency)
    #[error("Handler error: {0}")]
    Handler(Box<HandlerErrorDetails>),
}

// ==================== ERROR DETAIL STRUCTURES ====================

/// Configuration error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Configuration error in {field}: {message}")]
pub struct ConfigurationErrorDetails {
    /// The configuration field that caused the error
    pub field: String,
    /// Error message
    pub message: String,
    /// Current invalid value
    pub currentvalue: Option<String>,
    /// Expected value or format
    pub expected: Option<String>,
    /// Whether this is a user configuration error
    pub user_error: bool,
}

/// API error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("API error: {message}")]
pub struct ApiErrorDetails {
    /// Error message
    pub message: String,
    /// HTTP status code
    pub status_code: Option<u16>,
    /// Request ID for tracing
    pub request_id: Option<String>,
    /// API endpoint that failed
    pub endpoint: Option<String>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Storage error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Storage error: {message}")]
pub struct StorageErrorDetails {
    /// Error message
    pub message: String,
    /// Storage operation that failed
    pub operation: Option<String>,
    /// Path or resource involved
    pub resource: Option<String>,
    /// Storage-specific error data
    pub storage_data: Option<Box<StorageErrorData>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Network error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Network error: {message}")]
pub struct NetworkErrorDetails {
    /// Error message
    pub message: String,
    /// Network operation that failed
    pub operation: Option<String>,
    /// Remote endpoint
    pub endpoint: Option<String>,
    /// Network-specific error data
    pub network_data: Option<Box<NetworkErrorData>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Security error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Security error: {message}")]
pub struct SecurityErrorDetails {
    /// Error message
    pub message: String,
    /// Security operation that failed
    pub operation: Option<String>,
    /// User or principal involved
    pub principal: Option<String>,
    /// Security-specific error data
    pub security_data: Option<Box<SecurityErrorData>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Automation error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Automation error: {message}")]
pub struct AutomationErrorDetails {
    /// Error message
    pub message: String,
    /// Automation operation that failed
    pub operation: Option<String>,
    /// Target resource
    pub target: Option<String>,
    /// Automation-specific error data
    pub automation_data: Option<Box<AutomationErrorData>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// System error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("System error: {message}")]
pub struct SystemErrorDetails {
    /// Error message
    pub message: String,
    /// System component that failed
    pub component: String,
    /// System operation that failed
    pub operation: Option<String>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Internal error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Internal error: {message}")]
pub struct InternalErrorDetails {
    /// Error message
    pub message: String,
    /// Component where error occurred
    pub component: String,
    /// Location in code (<file:line>)
    pub location: Option<String>,
    /// Whether this indicates a bug
    pub is_bug: bool,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// External error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("External error: {message}")]
pub struct ExternalErrorDetails {
    /// Error message
    pub message: String,
    /// External service or dependency
    pub service: String,
    /// Whether the operation is retryable
    pub retryable: bool,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Validation error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Validation error: {message}")]
pub struct ValidationErrorDetails {
    /// Error message
    pub message: String,
    /// Field that failed validation
    pub field: Option<String>,
    /// Expected value or format
    pub expected: Option<String>,
    /// Actual value that failed
    pub actual: Option<String>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Timeout error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Timeout error: {message}")]
pub struct TimeoutErrorDetails {
    /// Error message
    pub message: String,
    /// Operation that timed out
    pub operation: Option<String>,
    /// Timeout duration
    pub timeout: Duration,
    /// Whether the operation is retryable
    pub retryable: bool,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// I/O error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("I/O error: {message}")]
pub struct IoErrorDetails {
    /// Error message
    pub message: String,
    /// Path or resource involved
    pub path: Option<String>,
    /// I/O operation that failed
    pub operation: Option<String>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Resource exhausted error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Resource exhausted: {message}")]
pub struct ResourceExhaustedErrorDetails {
    /// Error message
    pub message: String,
    /// Resource that was exhausted
    pub resource: String,
    /// Current usage
    pub current: Option<u64>,
    /// Maximum limit
    pub limit: Option<u64>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Testing error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Testing error: {message}")]
pub struct TestingErrorDetails {
    /// Error message
    pub message: String,
    /// Test name that failed
    pub test_name: Option<String>,
    /// Type of test
    pub test_type: Option<TestType>,
    /// Assertion failure details
    pub assertion_failure: Option<String>,
    /// Expected value
    pub expected: Option<String>,
    /// Actual value
    pub actual: Option<String>,
    /// Test-specific error data
    pub test_data: Option<Box<TestingErrorDetails>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Performance error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Performance error: {message}")]
pub struct PerformanceErrorDetails {
    /// Error message
    pub message: String,
    /// Performance operation that failed
    pub operation: String,
    /// Metric that failed
    pub metric: Option<String>,
    /// Expected performance value
    pub expected: Option<f64>,
    /// Actual performance value
    pub actual: Option<f64>,
    /// Performance unit (ms, MB/s, etc.)
    pub unit: Option<String>,
    /// Performance-specific error data
    pub performance_data: Option<Box<PerformanceErrorDetails>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Handler error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Handler error in {handler_name}: {message}")]
pub struct HandlerErrorDetails {
    /// Error message
    pub message: String,
    /// Handler name that failed
    pub handler_name: String,
    /// Request that was being handled
    pub request_info: Option<String>,
    /// Handler-specific error data
    pub handler_data: Option<Box<HandlerErrorDetails>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

// ==================== SUPPORTING TYPES ====================

/// Test type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    E2E,
    Performance,
    Security,
    Chaos,
}

// Re-export the data types we need (if any exist)
// pub use super::super::data::{};

// ==================== CONVENIENCE CONSTRUCTORS ====================

impl NestGateUnifiedError {
    /// Create a configuration error
    #[must_use]
    pub const fn configuration_error(field: &str, message: &str) -> Self {
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.to_string(),
            message: message.to_string(),
            currentvalue: None,
            expected: None,
            user_error: false,
        }))
    }

    /// Create an API error
    #[must_use]
    pub const fn api_error(message: &str) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: message.to_string(),
            status_code: None,
            request_id: None,
            endpoint: None,
            context: None,
        }))
    }

    /// Create a storage error
    #[must_use]
    pub const fn storage_error(message: &str) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: message.to_string(),
            resource: None,
            storage_data: None,
            operation: None,
            context: None,
        }))
    }

    /// Create a security error
    #[must_use]
    pub const fn security(message: &str) -> Self {
        Self::Security(Box::new(SecurityErrorDetails {
            message: message.to_string(),
            operation: None,
            principal: None,
            security_data: None,
            context: None,
        }))
    }

    /// Create a security error
    #[must_use]
    pub const fn security_error(message: &str) -> Self {
        Self::Security(Box::new(SecurityErrorDetails {
            message: message.to_string(),
            operation: None,
            principal: None,
            security_data: None,
            context: None,
        }))
    }

    /// Create a network error
    #[must_use]
    pub const fn network_error(message: &str) -> Self {
        Self::Network(Box::new(NetworkErrorDetails {
            message: message.to_string(),
            endpoint: None,
            network_data: None,
            operation: None,
            context: None,
        }))
    }

    /// Create a validation error
    #[must_use]
    pub const fn validation_error(message: &str) -> Self {
        Self::Validation(Box::new(ValidationErrorDetails {
            message: message.to_string(),
            field: None,
            expected: None,
            actual: None,
            context: None,
        }))
    }

    // internal method moved to system_errors.rs to avoid duplication

    /// Create a timeout error
    #[must_use]
    pub const fn timeout_error(operation: &str, duration: std::time::Duration) -> Self {
        Self::Timeout(Box::new(TimeoutErrorDetails {
            message: format!("Operation '{operation}' timed out after {duration:?}"),
            operation: Some(operation.to_string()),
            timeout: duration,
            retryable: true,
            context: None,
        }))
    }

    // ==================== BACKWARD COMPATIBILITY CONSTRUCTORS ====================
    // These methods maintain the old struct-style construction patterns for easier migration

    /// Create configuration error with detailed fields (backward compatibility)
    #[must_use]
    pub const fn configuration_error_detailed(
        field: String,
        message: String,
        currentvalue: Option<String>,
        expected: Option<String>,
        user_error: bool,
    ) -> Self {
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field,
            message,
            currentvalue,
            expected,
            user_error,
        }))
    }

    /// Create API error with detailed fields (backward compatibility)
    #[must_use]
    pub const fn api_error_detailed(
        message: String,
        status_code: Option<u16>,
        request_id: Option<String>,
        endpoint: Option<String>,
    ) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message,
            status_code,
            request_id,
            endpoint,
            context: None,
        }))
    }

    /// Create storage error with detailed fields (backward compatibility)
    #[must_use]
    pub const fn storage_error_detailed(message: String, operation: Option<String>) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message,
            operation,
            resource: None,
            storage_data: None,
            context: None,
        }))
    }

    /// Create network error with detailed fields (backward compatibility)
    #[must_use]
    pub const fn network_error_detailed(
        message: String,
        operation: Option<String>,
        endpoint: Option<String>,
    ) -> Self {
        Self::Network(Box::new(NetworkErrorDetails {
            message,
            operation,
            endpoint,
            network_data: None,
            context: None,
        }))
    }

    /// Create validation error with detailed fields (backward compatibility)
    #[must_use]
    pub const fn validation_error_detailed(
        message: String,
        field: Option<String>,
        expected: Option<String>,
        actual: Option<String>,
    ) -> Self {
        Self::Validation(Box::new(ValidationErrorDetails {
            message,
            field,
            expected,
            actual,
            context: None,
        }))
    }
}
