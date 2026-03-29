// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
/// Errors that can occur during `NestGateUnified` operations
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
    pub test_data: Option<Box<Self>>,
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
    pub performance_data: Option<Box<Self>>,
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
    pub handler_data: Option<Box<Self>>,
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

    /// Create a feature not enabled error
    ///
    /// Used when optional features are accessed but not enabled
    #[must_use]
    pub fn feature_not_enabled(feature: impl Into<String>, message: impl Into<String>) -> Self {
        Self::NotImplemented(Box::new(NotImplementedErrorDetails {
            feature: feature.into(),
            message: Some(message.into()),
            planned_version: None,
        }))
    }

    /// Create a storage operation error with operation name
    #[must_use]
    pub fn storage_operation(message: impl Into<String>, _recoverable: bool) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: message.into(),
            operation: Some("storage_operation".to_string()),
            resource: None,
            storage_data: None,
            context: None,
        }))
    }

    /// Create a simple error from a message string.
    ///
    /// Convenience constructor that wraps the message as an [`Internal`] error
    /// with automatic module tracking. Prefer domain-specific constructors
    /// (`configuration_error`, `network_error`, etc.) when the error category
    /// is known; use `simple` for quick prototyping and macro-generated errors.
    #[must_use]
    pub fn simple(message: impl Into<String>) -> Self {
        Self::Internal(Box::new(InternalErrorDetails {
            message: message.into(),
            component: "simple".to_string(),
            location: None,
            is_bug: false,
            context: None,
        }))
    }
}

#[cfg(test)]
mod core_errors_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn error_severity_roundtrip_serde() {
        let s = ErrorSeverity::High;
        let json = serde_json::to_string(&s).unwrap();
        let back: ErrorSeverity = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    macro_rules! assert_display_contains {
        ($err:expr, $needle:expr) => {
            let d = $err.to_string();
            assert!(d.contains($needle), "expected {} in {}", $needle, d);
        };
    }

    #[test]
    fn display_nest_gate_unified_error_configuration() {
        assert_display_contains!(
            NestGateUnifiedError::configuration_error("f", "m"),
            "Configuration error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_api() {
        assert_display_contains!(NestGateUnifiedError::api_error("x"), "API error:");
    }

    #[test]
    fn display_nest_gate_unified_error_storage() {
        assert_display_contains!(NestGateUnifiedError::storage_error("x"), "Storage error:");
    }

    #[test]
    fn display_nest_gate_unified_error_network() {
        assert_display_contains!(NestGateUnifiedError::network_error("x"), "Network error:");
    }

    #[test]
    fn display_nest_gate_unified_error_security() {
        assert_display_contains!(NestGateUnifiedError::security_error("x"), "Security error:");
    }

    #[test]
    fn display_nest_gate_unified_error_automation() {
        assert_display_contains!(
            NestGateUnifiedError::Automation(Box::new(AutomationErrorDetails {
                message: "a".into(),
                operation: None,
                target: None,
                automation_data: None,
                context: None,
            })),
            "Automation error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_system() {
        assert_display_contains!(
            NestGateUnifiedError::System(Box::new(SystemErrorDetails {
                message: "s".into(),
                component: "c".into(),
                operation: None,
                context: None,
            })),
            "System error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_internal() {
        assert_display_contains!(
            NestGateUnifiedError::Internal(Box::new(InternalErrorDetails {
                message: "i".into(),
                component: "c".into(),
                location: None,
                is_bug: false,
                context: None,
            })),
            "Internal error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_external() {
        assert_display_contains!(
            NestGateUnifiedError::External(Box::new(ExternalErrorDetails {
                message: "e".into(),
                service: "svc".into(),
                retryable: true,
                context: None,
            })),
            "External error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_validation() {
        assert_display_contains!(
            NestGateUnifiedError::validation_error("v"),
            "Validation error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_timeout() {
        assert_display_contains!(
            NestGateUnifiedError::timeout_error("op", Duration::from_secs(1)),
            "Timeout error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_io() {
        assert_display_contains!(
            NestGateUnifiedError::Io(Box::new(IoErrorDetails {
                message: "io".into(),
                path: None,
                operation: None,
                context: None,
            })),
            "I/O error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_resource_exhausted() {
        assert_display_contains!(
            NestGateUnifiedError::ResourceExhausted(Box::new(ResourceExhaustedErrorDetails {
                message: "r".into(),
                resource: "cpu".into(),
                current: None,
                limit: None,
                context: None,
            })),
            "Resource exhausted:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_testing() {
        assert_display_contains!(
            NestGateUnifiedError::Testing(Box::new(TestingErrorDetails {
                message: "t".into(),
                test_name: None,
                test_type: Some(TestType::Unit),
                assertion_failure: None,
                expected: None,
                actual: None,
                test_data: None,
                context: None,
            })),
            "Testing error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_performance() {
        assert_display_contains!(
            NestGateUnifiedError::Performance(Box::new(PerformanceErrorDetails {
                message: "p".into(),
                operation: "op".into(),
                metric: None,
                expected: None,
                actual: None,
                unit: None,
                performance_data: None,
                context: None,
            })),
            "Performance error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_handler() {
        assert_display_contains!(
            NestGateUnifiedError::Handler(Box::new(HandlerErrorDetails {
                message: "h".into(),
                handler_name: "hn".into(),
                request_info: None,
                handler_data: None,
                context: None,
            })),
            "Handler error"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_load_balancer() {
        assert_display_contains!(
            NestGateUnifiedError::LoadBalancer(Box::new(LoadBalancerErrorDetails {
                message: "lb".into(),
                available_services: None,
                algorithm: None,
            })),
            "Load balancer error:"
        );
    }

    #[test]
    fn display_nest_gate_unified_error_not_implemented() {
        assert_display_contains!(
            NestGateUnifiedError::NotImplemented(Box::new(NotImplementedErrorDetails {
                feature: "f".into(),
                message: None,
                planned_version: None,
            })),
            "Not implemented:"
        );
    }

    #[test]
    fn constructor_helpers_non_empty_display() {
        let samples = [
            NestGateUnifiedError::network_connection_failed("h", 80, "refused").to_string(),
            NestGateUnifiedError::network_timeout("http://x", Duration::from_secs(2)).to_string(),
            NestGateUnifiedError::storage_not_found("/p").to_string(),
            NestGateUnifiedError::storage_permission_denied("/p", "read").to_string(),
            NestGateUnifiedError::storage_disk_full("/p", 100, 10).to_string(),
            NestGateUnifiedError::validation_field("f", "bad").to_string(),
            NestGateUnifiedError::validation_schema("s", "bad", None).to_string(),
            NestGateUnifiedError::security_authentication_failed("u", "bad").to_string(),
            NestGateUnifiedError::security_authorization_failed("u", "act", "res").to_string(),
            NestGateUnifiedError::security_encryption_failed("aes", "bad").to_string(),
            NestGateUnifiedError::api_not_found("/x").to_string(),
            NestGateUnifiedError::api_bad_request("bad").to_string(),
            NestGateUnifiedError::api_internal_error("bad").to_string(),
            NestGateUnifiedError::configuration_invalid_value("f", "v", "e").to_string(),
            NestGateUnifiedError::configuration_missing_required("f").to_string(),
            NestGateUnifiedError::feature_not_enabled("f", "msg").to_string(),
            NestGateUnifiedError::storage_operation("op", true).to_string(),
            NestGateUnifiedError::simple("quick").to_string(),
        ];
        for s in samples {
            assert!(!s.is_empty());
        }
    }

    #[test]
    fn detailed_constructors_roundtrip_serde() {
        let e = NestGateUnifiedError::configuration_error_detailed(
            "field".into(),
            "msg".into(),
            Some("cur".into()),
            Some("exp".into()),
            true,
        );
        let json = serde_json::to_string(&e).unwrap();
        let back: NestGateUnifiedError = serde_json::from_str(&json).unwrap();
        assert_eq!(e.to_string(), back.to_string());
    }

    #[test]
    fn error_severity_all_variants_debug_and_clone() {
        for s in [
            ErrorSeverity::Low,
            ErrorSeverity::Medium,
            ErrorSeverity::High,
            ErrorSeverity::Critical,
        ] {
            let c = s.clone();
            assert_eq!(format!("{s:?}"), format!("{c:?}"));
        }
    }

    #[test]
    fn configuration_error_details_display_and_clone() {
        let d = ConfigurationErrorDetails {
            field: "f".into(),
            message: "m".into(),
            currentvalue: Some("c".into()),
            expected: Some("e".into()),
            user_error: true,
        };
        let _ = d.to_string();
        let d2 = d.clone();
        assert_eq!(d.field, d2.field);
    }

    // --- Round 6: exhaustive Display smoke + constructor coverage ---

    macro_rules! r6_assert_nonempty {
        ($($e:expr),+ $(,)?) => {
            $(assert!(!$e.to_string().is_empty(), "empty display");)+
        };
    }

    #[test]
    fn r6_display_automation_system_external() {
        r6_assert_nonempty!(
            NestGateUnifiedError::Automation(Box::new(AutomationErrorDetails {
                message: "a".into(),
                operation: Some("op".into()),
                target: Some("t".into()),
                automation_data: None,
                context: None,
            })),
            NestGateUnifiedError::System(Box::new(SystemErrorDetails {
                message: "s".into(),
                component: "c".into(),
                operation: Some("o".into()),
                context: None,
            })),
            NestGateUnifiedError::External(Box::new(ExternalErrorDetails {
                message: "e".into(),
                service: "svc".into(),
                retryable: false,
                context: None,
            }))
        );
    }

    #[test]
    fn r6_timeout_io_details_display() {
        let t = TimeoutErrorDetails {
            message: "t".into(),
            operation: Some("o".into()),
            timeout: Duration::from_nanos(1),
            retryable: false,
            context: None,
        };
        assert!(!t.to_string().is_empty());
        let io = IoErrorDetails {
            message: "i".into(),
            path: Some("/p".into()),
            operation: Some("read".into()),
            context: None,
        };
        assert!(!io.to_string().is_empty());
    }

    #[test]
    fn r6_resource_testing_details_display() {
        let r = ResourceExhaustedErrorDetails {
            message: "m".into(),
            resource: "r".into(),
            current: Some(1),
            limit: Some(2),
            context: None,
        };
        assert!(!r.to_string().is_empty());
        let te = TestingErrorDetails {
            message: "m".into(),
            test_name: Some("n".into()),
            test_type: Some(TestType::Integration),
            assertion_failure: Some("a".into()),
            expected: Some("e".into()),
            actual: Some("a".into()),
            test_data: None,
            context: None,
        };
        assert!(!te.to_string().is_empty());
    }

    #[test]
    fn r6_performance_handler_lb_not_impl_display() {
        let p = PerformanceErrorDetails {
            message: "m".into(),
            operation: "op".into(),
            metric: Some("met".into()),
            expected: Some(1.0),
            actual: Some(2.0),
            unit: Some("ms".into()),
            performance_data: None,
            context: None,
        };
        assert!(!p.to_string().is_empty());
        let h = HandlerErrorDetails {
            message: "m".into(),
            handler_name: "h".into(),
            request_info: Some("r".into()),
            handler_data: None,
            context: None,
        };
        assert!(!h.to_string().is_empty());
        let l = LoadBalancerErrorDetails {
            message: "m".into(),
            available_services: Some(0),
            algorithm: Some("rr".into()),
        };
        assert!(!l.to_string().is_empty());
        let n = NotImplementedErrorDetails {
            feature: "f".into(),
            message: Some("msg".into()),
            planned_version: Some("v".into()),
        };
        assert!(!n.to_string().is_empty());
    }

    #[test]
    fn r6_api_storage_network_validation_details_serde() {
        let a = ApiErrorDetails {
            message: "m".into(),
            status_code: Some(418),
            request_id: Some("id".into()),
            endpoint: Some("/e".into()),
            context: None,
        };
        let json = serde_json::to_string(&a).unwrap();
        let _: ApiErrorDetails = serde_json::from_str(&json).unwrap();

        let s = StorageErrorDetails {
            message: "m".into(),
            operation: Some("o".into()),
            resource: Some("r".into()),
            storage_data: None,
            context: None,
        };
        let _ = serde_json::to_string(&s).unwrap();

        let v = ValidationErrorDetails {
            message: "m".into(),
            field: Some("f".into()),
            expected: Some("e".into()),
            actual: Some("a".into()),
            context: None,
        };
        let _ = serde_json::to_string(&v).unwrap();
    }

    #[test]
    fn r6_security_automation_network_details_roundtrip() {
        let sec = SecurityErrorDetails {
            message: "m".into(),
            operation: Some("o".into()),
            principal: Some("p".into()),
            security_data: None,
            context: None,
        };
        let _ = serde_json::to_string(&sec).unwrap();
        let net = NetworkErrorDetails {
            message: "m".into(),
            operation: Some("o".into()),
            endpoint: Some("e".into()),
            network_data: None,
            context: None,
        };
        let _ = serde_json::to_string(&net).unwrap();
    }

    #[test]
    fn r6_internal_error_details_fields() {
        let i = InternalErrorDetails {
            message: "m".into(),
            component: "c".into(),
            location: Some("loc".into()),
            is_bug: true,
            context: None,
        };
        assert!(i.to_string().contains("m"));
    }

    #[test]
    fn r6_test_type_all_variants_serde() {
        for t in [
            TestType::Unit,
            TestType::Integration,
            TestType::E2E,
            TestType::Performance,
            TestType::Security,
            TestType::Chaos,
        ] {
            let j = serde_json::to_string(&t).unwrap();
            let _: TestType = serde_json::from_str(&j).unwrap();
        }
    }

    #[test]
    fn r6_nest_gate_unified_error_serde_roundtrip_variants() {
        let samples = vec![
            NestGateUnifiedError::configuration_error("f", "m"),
            NestGateUnifiedError::api_error("a"),
            NestGateUnifiedError::simple("s"),
        ];
        for e in samples {
            let j = serde_json::to_string(&e).unwrap();
            let back: NestGateUnifiedError = serde_json::from_str(&j).unwrap();
            assert_eq!(e.to_string(), back.to_string());
        }
    }

    /// Round 6 bulk: independent Display smoke tests (unique names for coverage granularity).
    macro_rules! r6_smoke {
        ($($name:ident => $e:expr);+ $(;)?) => {
            $(
                #[test]
                fn $name() {
                    assert!(!$e.to_string().is_empty());
                }
            )+
        };
    }

    r6_smoke! {
        r6_smoke_cfg01 => NestGateUnifiedError::configuration_error("a", "b");
        r6_smoke_cfg02 => NestGateUnifiedError::configuration_missing_required("req");
        r6_smoke_api01 => NestGateUnifiedError::api_bad_request("br");
        r6_smoke_api02 => NestGateUnifiedError::api_internal_error("ie");
        r6_smoke_api03 => NestGateUnifiedError::api_not_found("/x");
        r6_smoke_net01 => NestGateUnifiedError::network_error("n");
        r6_smoke_net02 => NestGateUnifiedError::network_connection_failed("h", 443, "x");
        r6_smoke_net03 => NestGateUnifiedError::network_timeout("http://u", Duration::from_secs(1));
        r6_smoke_sec01 => NestGateUnifiedError::security_error("s");
        r6_smoke_sec02 => NestGateUnifiedError::security_authentication_failed("u", "bad");
        r6_smoke_sec03 => NestGateUnifiedError::security_authorization_failed("u", "a", "r");
        r6_smoke_sec04 => NestGateUnifiedError::security_encryption_failed("aes", "e");
        r6_smoke_sto01 => NestGateUnifiedError::storage_error("st");
        r6_smoke_sto02 => NestGateUnifiedError::storage_not_found("/p");
        r6_smoke_sto03 => NestGateUnifiedError::storage_permission_denied("/p", "w");
        r6_smoke_sto04 => NestGateUnifiedError::storage_disk_full("/p", 100, 1);
        r6_smoke_sto05 => NestGateUnifiedError::storage_operation("op", false);
        r6_smoke_val01 => NestGateUnifiedError::validation_error("v");
        r6_smoke_val02 => NestGateUnifiedError::validation_field("f", "bad");
        r6_smoke_val03 => NestGateUnifiedError::validation_schema("s", "bad", Some("p".into()));
        r6_smoke_to01 => NestGateUnifiedError::timeout_error("op", Duration::from_millis(500));
        r6_smoke_feat01 => NestGateUnifiedError::feature_not_enabled("feat", "msg");
        r6_smoke_cfg_inv => NestGateUnifiedError::configuration_invalid_value("f", "v", "e");
        r6_smoke_sim01 => NestGateUnifiedError::simple("quick");
        r6_smoke_sim02 => NestGateUnifiedError::simple("another");
        r6_smoke_det_api => NestGateUnifiedError::api_error_detailed(
            "m".into(),
            Some(502),
            Some("rid".into()),
            Some("/e".into()),
        );
        r6_smoke_det_cfg => NestGateUnifiedError::configuration_error_detailed(
            "f".into(),
            "msg".into(),
            Some("c".into()),
            Some("e".into()),
            false,
        );
        r6_smoke_det_sto => NestGateUnifiedError::storage_error_detailed("m".into(), Some("o".into()));
        r6_smoke_det_net => NestGateUnifiedError::network_error_detailed(
            "m".into(),
            Some("o".into()),
            Some("ep".into()),
        );
        r6_smoke_det_val => NestGateUnifiedError::validation_error_detailed(
            "m".into(),
            Some("f".into()),
            Some("e".into()),
            Some("a".into()),
        );
    }
}
