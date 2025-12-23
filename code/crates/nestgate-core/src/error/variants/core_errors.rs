// **CORE ERROR TYPES**
//! Core system error types and handling for the `NestGate` system.
// The main NestGateUnifiedError enum and core error handling.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

use super::super::context::ErrorContext;
use super::super::data::{
    AutomationErrorData, NetworkErrorData, SecurityErrorData, StorageErrorData,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Errorseverity
pub enum ErrorSeverity {
    /// Low
    Low,
    /// Medium
    Medium,
    /// High
    High,
    /// Critical
    Critical,
}

/// **THE** definitive `NestGate` error type - single source of truth for all errors
///
/// This enum is designed to be small in memory by boxing all the large variants.
/// This eliminates the `clippy::result_large_err` warnings while maintaining full functionality.
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
/// Errors that can occur during NestGateUnified operations
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

    /// Load balancer errors (boxed for size efficiency)
    #[error("Load balancer error: {0}")]
    LoadBalancer(Box<LoadBalancerErrorDetails>),

    /// Not implemented functionality (boxed for size efficiency)
    #[error("Not implemented: {0}")]
    NotImplemented(Box<NotImplementedErrorDetails>),
}

// ==================== ERROR DETAIL STRUCTURES ====================

/// Configuration error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Configuration error in {field}: {message}")]
/// Configurationerrordetails
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
/// Apierrordetails
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
/// Storageerrordetails
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
/// Networkerrordetails
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
/// Securityerrordetails
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
/// Automationerrordetails
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
/// Systemerrordetails
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
/// Internalerrordetails
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
/// Externalerrordetails
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
/// Validationerrordetails
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
/// Timeouterrordetails
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
/// Ioerrordetails
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
/// Resourceexhaustederrordetails
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
/// Testingerrordetails
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
/// Performanceerrordetails
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
/// Handlererrordetails
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

/// Load balancer error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Load balancer error: {message}")]
/// Loadbalancererrordetails
pub struct LoadBalancerErrorDetails {
    /// Error message
    pub message: String,
    /// Number of available services
    pub available_services: Option<usize>,
    /// Algorithm being used
    pub algorithm: Option<String>,
}

/// Not implemented error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Not implemented: {feature}")]
/// Notimplementederrordetails
pub struct NotImplementedErrorDetails {
    /// Feature that is not implemented
    pub feature: String,
    /// Additional context
    pub message: Option<String>,
    /// Planned version for implementation
    pub planned_version: Option<String>,
}

// ==================== SUPPORTING TYPES ====================

/// Test type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Test
pub enum TestType {
    /// Unit
    Unit,
    /// Integration
    Integration,
    /// E2E
    E2E,
    /// Performance
    Performance,
    /// Security
    Security,
    /// Chaos
    Chaos,
}

// Re-export the data types we need (if any exist)
// pub use super::super::data::{};

// ==================== CONVENIENCE CONSTRUCTORS ====================

impl NestGateUnifiedError {
    /// Create a configuration error
    #[must_use]
    pub fn configuration_error(field: &str, message: &str) -> Self {
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
    pub fn api_error(message: &str) -> Self {
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
    pub fn storage_error(message: &str) -> Self {
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
    pub fn security_error(message: &str) -> Self {
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
    pub fn network_error(message: &str) -> Self {
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
    pub fn validation_error(message: &str) -> Self {
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
    pub fn timeout_error(operation: &str, duration: std::time::Duration) -> Self {
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
    pub fn configuration_error_detailed(
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
    pub fn api_error_detailed(
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
    pub fn storage_error_detailed(message: String, operation: Option<String>) -> Self {
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
    pub fn network_error_detailed(
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
    pub fn validation_error_detailed(
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

    // ==================== ERGONOMIC MIGRATION HELPERS ====================
    // These helpers make migration from domain_errors.rs patterns easier
    // They use impl Into<String> for maximum ergonomics

    /// Create a network connection failure error - migration helper
    ///
    /// Replaces: `NetworkError::ConnectionFailed { address, port, error, timeout }`
    #[must_use]
    pub fn network_connection_failed(
        address: impl Into<String>,
        port: u16,
        reason: impl Into<String>,
    ) -> Self {
        let address = address.into();
        let reason = reason.into();
        Self::Network(Box::new(NetworkErrorDetails {
            message: format!("Connection failed: {address}:{port} - {reason}"),
            endpoint: Some(format!("{address}:{port}")),
            operation: Some("connect".to_string()),
            network_data: None,
            context: None,
        }))
    }

    /// Create a network timeout error - migration helper
    ///
    /// Replaces: `NetworkError::Timeout { url, timeout, method }`
    #[must_use]
    pub fn network_timeout(url: impl Into<String>, duration: Duration) -> Self {
        let url = url.into();
        Self::Timeout(Box::new(TimeoutErrorDetails {
            message: format!("Request timeout: {url} after {duration:?}"),
            operation: Some("network_request".to_string()),
            timeout: duration,
            retryable: true,
            context: None,
        }))
    }

    /// Create a storage file not found error - migration helper
    ///
    /// Replaces: `StorageError::FileNotFound { path, operation }`
    #[must_use]
    pub fn storage_not_found(path: impl Into<String>) -> Self {
        let path = path.into();
        Self::Storage(Box::new(StorageErrorDetails {
            message: format!("File not found: {path}"),
            resource: Some(path),
            operation: Some("read".to_string()),
            storage_data: None,
            context: None,
        }))
    }

    /// Create a storage permission denied error - migration helper
    ///
    /// Replaces: `StorageError::PermissionDenied { path, operation, required_permissions }`
    #[must_use]
    pub fn storage_permission_denied(
        path: impl Into<String>,
        operation: impl Into<String>,
    ) -> Self {
        let path = path.into();
        let operation = operation.into();
        Self::Storage(Box::new(StorageErrorDetails {
            message: format!("Permission denied: {path} for operation '{operation}'"),
            resource: Some(path),
            operation: Some(operation),
            storage_data: None,
            context: None,
        }))
    }

    /// Create a storage disk full error - migration helper
    ///
    /// Replaces: `StorageError::DiskFull { path, available, required }`
    #[must_use]
    pub fn storage_disk_full(
        path: impl Into<String>,
        required_bytes: u64,
        available_bytes: u64,
    ) -> Self {
        let path = path.into();
        Self::ResourceExhausted(Box::new(ResourceExhaustedErrorDetails {
            message: format!(
                "Disk full: {path} (required: {required_bytes} bytes, available: {available_bytes} bytes)"
            ),
            resource: "disk_space".to_string(),
            limit: Some(required_bytes),
            current: Some(available_bytes),
            context: None,
        }))
    }

    /// Create a validation field error - migration helper
    ///
    /// Replaces: `ValidationError::FieldValidation { field, message, constraint }`
    #[must_use]
    pub fn validation_field(field: impl Into<String>, message: impl Into<String>) -> Self {
        let field_str = field.into();
        let message_str = message.into();
        Self::Validation(Box::new(ValidationErrorDetails {
            message: format!("Field '{field_str}': {message_str}"),
            field: Some(field_str),
            expected: None,
            actual: None,
            context: None,
        }))
    }

    /// Create a validation schema error - migration helper
    ///
    /// Replaces: `ValidationError::SchemaValidation { schema, message, path }`
    #[must_use]
    pub fn validation_schema(
        schema: impl Into<String>,
        message: impl Into<String>,
        path: Option<String>,
    ) -> Self {
        let schema = schema.into();
        let message = message.into();
        Self::Validation(Box::new(ValidationErrorDetails {
            message: format!("Schema validation failed ({schema}): {message}"),
            field: path,
            expected: Some(schema),
            actual: None,
            context: None,
        }))
    }

    /// Create a security authentication failed error - migration helper
    ///
    /// Replaces: `SecurityError::AuthenticationFailed { principal, reason }`
    #[must_use]
    pub fn security_authentication_failed(
        principal: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        let principal = principal.into();
        let reason = reason.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!("Authentication failed for '{principal}': {reason}"),
            operation: Some("authenticate".to_string()),
            principal: Some(principal),
            security_data: None,
            context: None,
        }))
    }

    /// Create a security authorization failed error - migration helper
    ///
    /// Replaces: `SecurityError::AuthorizationFailed { principal, action, resource }`
    #[must_use]
    pub fn security_authorization_failed(
        principal: impl Into<String>,
        action: impl Into<String>,
        resource: impl Into<String>,
    ) -> Self {
        let principal = principal.into();
        let action = action.into();
        let resource = resource.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!(
                "Authorization failed: '{principal}' cannot '{action}' on '{resource}'"
            ),
            operation: Some(action),
            principal: Some(principal),
            security_data: None,
            context: None,
        }))
    }

    /// Create a security encryption error - migration helper
    ///
    /// Replaces: `SecurityError::EncryptionFailed { algorithm, reason }`
    #[must_use]
    pub fn security_encryption_failed(
        algorithm: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        let algorithm = algorithm.into();
        let reason = reason.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!("Encryption failed ({algorithm}): {reason}"),
            operation: Some("encrypt".to_string()),
            principal: None,
            security_data: None,
            context: None,
        }))
    }

    /// Create an API not found error - migration helper
    ///
    /// Replaces: `ApiError::NotFound { endpoint }`
    #[must_use]
    pub fn api_not_found(endpoint: impl Into<String>) -> Self {
        let endpoint = endpoint.into();
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Endpoint not found: {endpoint}"),
            status_code: Some(404),
            request_id: None,
            endpoint: Some(endpoint),
            context: None,
        }))
    }

    /// Create an API bad request error - migration helper
    ///
    /// Replaces: `ApiError::BadRequest { reason }`
    #[must_use]
    pub fn api_bad_request(reason: impl Into<String>) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Bad request: {}", reason.into()),
            status_code: Some(400),
            request_id: None,
            endpoint: None,
            context: None,
        }))
    }

    /// Create an API internal server error - migration helper
    ///
    /// Replaces: `ApiError::InternalError { message }`
    #[must_use]
    pub fn api_internal_error(message: impl Into<String>) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Internal server error: {}", message.into()),
            status_code: Some(500),
            request_id: None,
            endpoint: None,
            context: None,
        }))
    }

    /// Create a configuration invalid value error - migration helper
    ///
    /// Replaces: `ConfigurationError::InvalidValue { field, value, expected }`
    #[must_use]
    pub fn configuration_invalid_value(
        field: impl Into<String>,
        value: impl Into<String>,
        expected: impl Into<String>,
    ) -> Self {
        let field = field.into();
        let value = value.into();
        let expected = expected.into();
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.clone(),
            message: format!("Invalid value for '{field}': got '{value}', expected '{expected}'"),
            currentvalue: Some(value),
            expected: Some(expected),
            user_error: true,
        }))
    }

    /// Create a configuration missing required field error - migration helper
    ///
    /// Replaces: `ConfigurationError::MissingRequired { field }`
    #[must_use]
    pub fn configuration_missing_required(field: impl Into<String>) -> Self {
        let field = field.into();
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.clone(),
            message: format!("Missing required configuration field: '{field}'"),
            currentvalue: None,
            expected: Some("required value".to_string()),
            user_error: true,
        }))
    }
}
