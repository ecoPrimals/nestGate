// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **CORE ERROR TYPES**
//! Core system error types and handling for the `NestGate` system.
// The main NestGateUnifiedError enum and core error handling.

use serde::{Deserialize, Serialize};
use std::borrow::Cow;
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
    pub field: Cow<'static, str>,
    /// Error message
    pub message: Cow<'static, str>,
    /// Current invalid value
    pub currentvalue: Option<Cow<'static, str>>,
    /// Expected value or format
    pub expected: Option<Cow<'static, str>>,
    /// Whether this is a user configuration error
    pub user_error: bool,
}

/// API error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("API error: {message}")]
/// Apierrordetails
pub struct ApiErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// HTTP status code
    pub status_code: Option<u16>,
    /// Request ID for tracing
    pub request_id: Option<Cow<'static, str>>,
    /// API endpoint that failed
    pub endpoint: Option<Cow<'static, str>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Storage error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Storage error: {message}")]
/// Storageerrordetails
pub struct StorageErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Storage operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Path or resource involved
    pub resource: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// Network operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Remote endpoint
    pub endpoint: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// Security operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// User or principal involved
    pub principal: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// Automation operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Target resource
    pub target: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// System component that failed
    pub component: Cow<'static, str>,
    /// System operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Internal error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Internal error: {message}")]
/// Internalerrordetails
pub struct InternalErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Component where error occurred
    pub component: Cow<'static, str>,
    /// Location in code (<file:line>)
    pub location: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// External service or dependency
    pub service: Cow<'static, str>,
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
    pub message: Cow<'static, str>,
    /// Field that failed validation
    pub field: Option<Cow<'static, str>>,
    /// Expected value or format
    pub expected: Option<Cow<'static, str>>,
    /// Actual value that failed
    pub actual: Option<Cow<'static, str>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Timeout error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Timeout error: {message}")]
/// Timeouterrordetails
pub struct TimeoutErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Operation that timed out
    pub operation: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// Path or resource involved
    pub path: Option<Cow<'static, str>>,
    /// I/O operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Resource exhausted error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Resource exhausted: {message}")]
/// Resourceexhaustederrordetails
pub struct ResourceExhaustedErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Resource that was exhausted
    pub resource: Cow<'static, str>,
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
    pub message: Cow<'static, str>,
    /// Test name that failed
    pub test_name: Option<Cow<'static, str>>,
    /// Type of test
    pub test_type: Option<TestType>,
    /// Assertion failure details
    pub assertion_failure: Option<Cow<'static, str>>,
    /// Expected value
    pub expected: Option<Cow<'static, str>>,
    /// Actual value
    pub actual: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// Performance operation that failed
    pub operation: Cow<'static, str>,
    /// Metric that failed
    pub metric: Option<Cow<'static, str>>,
    /// Expected performance value
    pub expected: Option<f64>,
    /// Actual performance value
    pub actual: Option<f64>,
    /// Performance unit (ms, MB/s, etc.)
    pub unit: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// Handler name that failed
    pub handler_name: Cow<'static, str>,
    /// Request that was being handled
    pub request_info: Option<Cow<'static, str>>,
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
    pub message: Cow<'static, str>,
    /// Number of available services
    pub available_services: Option<usize>,
    /// Algorithm being used
    pub algorithm: Option<Cow<'static, str>>,
}

/// Not implemented error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Not implemented: {feature}")]
/// Notimplementederrordetails
pub struct NotImplementedErrorDetails {
    /// Feature that is not implemented
    pub feature: Cow<'static, str>,
    /// Additional context
    pub message: Option<Cow<'static, str>>,
    /// Planned version for implementation
    pub planned_version: Option<Cow<'static, str>>,
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
    pub fn configuration_error(
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.into(),
            message: message.into(),
            currentvalue: None,
            expected: None,
            user_error: false,
        }))
    }

    /// Create an API error
    #[must_use]
    pub fn api_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: message.into(),
            status_code: None,
            request_id: None,
            endpoint: None,
            context: None,
        }))
    }

    /// Create a storage error
    #[must_use]
    pub fn storage_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: message.into(),
            resource: None,
            storage_data: None,
            operation: None,
            context: None,
        }))
    }

    /// Create a security error
    #[must_use]
    pub fn security_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Security(Box::new(SecurityErrorDetails {
            message: message.into(),
            operation: None,
            principal: None,
            security_data: None,
            context: None,
        }))
    }

    /// Create a network error
    #[must_use]
    pub fn network_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Network(Box::new(NetworkErrorDetails {
            message: message.into(),
            endpoint: None,
            network_data: None,
            operation: None,
            context: None,
        }))
    }

    /// Create a validation error
    #[must_use]
    pub fn validation_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Validation(Box::new(ValidationErrorDetails {
            message: message.into(),
            field: None,
            expected: None,
            actual: None,
            context: None,
        }))
    }

    // internal method moved to system_errors.rs to avoid duplication

    /// Create a timeout error
    #[must_use]
    pub fn timeout_error(
        operation: impl Into<Cow<'static, str>>,
        duration: std::time::Duration,
    ) -> Self {
        let operation = operation.into();
        Self::Timeout(Box::new(TimeoutErrorDetails {
            message: format!("Operation '{operation}' timed out after {duration:?}").into(),
            operation: Some(operation),
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
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
        currentvalue: Option<Cow<'static, str>>,
        expected: Option<Cow<'static, str>>,
        user_error: bool,
    ) -> Self {
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.into(),
            message: message.into(),
            currentvalue,
            expected,
            user_error,
        }))
    }

    /// Create API error with detailed fields (backward compatibility)
    #[must_use]
    pub fn api_error_detailed(
        message: impl Into<Cow<'static, str>>,
        status_code: Option<u16>,
        request_id: Option<Cow<'static, str>>,
        endpoint: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: message.into(),
            status_code,
            request_id,
            endpoint,
            context: None,
        }))
    }

    /// Create storage error with detailed fields (backward compatibility)
    #[must_use]
    pub fn storage_error_detailed(
        message: impl Into<Cow<'static, str>>,
        operation: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: message.into(),
            operation,
            resource: None,
            storage_data: None,
            context: None,
        }))
    }

    /// Create network error with detailed fields (backward compatibility)
    #[must_use]
    pub fn network_error_detailed(
        message: impl Into<Cow<'static, str>>,
        operation: Option<Cow<'static, str>>,
        endpoint: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Network(Box::new(NetworkErrorDetails {
            message: message.into(),
            operation,
            endpoint,
            network_data: None,
            context: None,
        }))
    }

    /// Create validation error with detailed fields (backward compatibility)
    #[must_use]
    pub fn validation_error_detailed(
        message: impl Into<Cow<'static, str>>,
        field: Option<Cow<'static, str>>,
        expected: Option<Cow<'static, str>>,
        actual: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Validation(Box::new(ValidationErrorDetails {
            message: message.into(),
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
        address: impl Into<Cow<'static, str>>,
        port: u16,
        reason: impl Into<Cow<'static, str>>,
    ) -> Self {
        let address = address.into();
        let reason = reason.into();
        Self::Network(Box::new(NetworkErrorDetails {
            message: format!("Connection failed: {address}:{port} - {reason}").into(),
            endpoint: Some(format!("{address}:{port}").into()),
            operation: Some(Cow::Borrowed("connect")),
            network_data: None,
            context: None,
        }))
    }

    /// Create a network timeout error - migration helper
    ///
    /// Replaces: `NetworkError::Timeout { url, timeout, method }`
    #[must_use]
    pub fn network_timeout(url: impl Into<Cow<'static, str>>, duration: Duration) -> Self {
        let url = url.into();
        Self::Timeout(Box::new(TimeoutErrorDetails {
            message: format!("Request timeout: {url} after {duration:?}").into(),
            operation: Some(Cow::Borrowed("network_request")),
            timeout: duration,
            retryable: true,
            context: None,
        }))
    }

    /// Create a storage file not found error - migration helper
    ///
    /// Replaces: `StorageError::FileNotFound { path, operation }`
    #[must_use]
    pub fn storage_not_found(path: impl Into<Cow<'static, str>>) -> Self {
        let path = path.into();
        Self::Storage(Box::new(StorageErrorDetails {
            message: format!("File not found: {path}").into(),
            resource: Some(path.clone()),
            operation: Some(Cow::Borrowed("read")),
            storage_data: None,
            context: None,
        }))
    }

    /// Create a storage permission denied error - migration helper
    ///
    /// Replaces: `StorageError::PermissionDenied { path, operation, required_permissions }`
    #[must_use]
    pub fn storage_permission_denied(
        path: impl Into<Cow<'static, str>>,
        operation: impl Into<Cow<'static, str>>,
    ) -> Self {
        let path = path.into();
        let operation = operation.into();
        Self::Storage(Box::new(StorageErrorDetails {
            message: format!("Permission denied: {path} for operation '{operation}'").into(),
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
        path: impl Into<Cow<'static, str>>,
        required_bytes: u64,
        available_bytes: u64,
    ) -> Self {
        let path = path.into();
        Self::ResourceExhausted(Box::new(ResourceExhaustedErrorDetails {
            message: format!(
                "Disk full: {path} (required: {required_bytes} bytes, available: {available_bytes} bytes)"
            )
            .into(),
            resource: Cow::Borrowed("disk_space"),
            limit: Some(required_bytes),
            current: Some(available_bytes),
            context: None,
        }))
    }

    /// Create a validation field error - migration helper
    ///
    /// Replaces: `ValidationError::FieldValidation { field, message, constraint }`
    #[must_use]
    pub fn validation_field(
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        let field_str = field.into();
        let message_str = message.into();
        Self::Validation(Box::new(ValidationErrorDetails {
            message: format!("Field '{field_str}': {message_str}").into(),
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
        schema: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
        path: Option<Cow<'static, str>>,
    ) -> Self {
        let schema = schema.into();
        let message = message.into();
        Self::Validation(Box::new(ValidationErrorDetails {
            message: format!("Schema validation failed ({schema}): {message}").into(),
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
        principal: impl Into<Cow<'static, str>>,
        reason: impl Into<Cow<'static, str>>,
    ) -> Self {
        let principal = principal.into();
        let reason = reason.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!("Authentication failed for '{principal}': {reason}").into(),
            operation: Some(Cow::Borrowed("authenticate")),
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
        principal: impl Into<Cow<'static, str>>,
        action: impl Into<Cow<'static, str>>,
        resource: impl Into<Cow<'static, str>>,
    ) -> Self {
        let principal = principal.into();
        let action = action.into();
        let resource = resource.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!(
                "Authorization failed: '{principal}' cannot '{action}' on '{resource}'"
            )
            .into(),
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
        algorithm: impl Into<Cow<'static, str>>,
        reason: impl Into<Cow<'static, str>>,
    ) -> Self {
        let algorithm = algorithm.into();
        let reason = reason.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!("Encryption failed ({algorithm}): {reason}").into(),
            operation: Some(Cow::Borrowed("encrypt")),
            principal: None,
            security_data: None,
            context: None,
        }))
    }

    /// Create an API not found error - migration helper
    ///
    /// Replaces: `ApiError::NotFound { endpoint }`
    #[must_use]
    pub fn api_not_found(endpoint: impl Into<Cow<'static, str>>) -> Self {
        let endpoint = endpoint.into();
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Endpoint not found: {endpoint}").into(),
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
    pub fn api_bad_request(reason: impl Into<Cow<'static, str>>) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Bad request: {}", reason.into()).into(),
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
    pub fn api_internal_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Internal server error: {}", message.into()).into(),
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
        field: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
        expected: impl Into<Cow<'static, str>>,
    ) -> Self {
        let field = field.into();
        let value = value.into();
        let expected = expected.into();
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.clone(),
            message: format!("Invalid value for '{field}': got '{value}', expected '{expected}'")
                .into(),
            currentvalue: Some(value),
            expected: Some(expected),
            user_error: true,
        }))
    }

    /// Create a configuration missing required field error - migration helper
    ///
    /// Replaces: `ConfigurationError::MissingRequired { field }`
    #[must_use]
    pub fn configuration_missing_required(field: impl Into<Cow<'static, str>>) -> Self {
        let field = field.into();
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.clone(),
            message: format!("Missing required configuration field: '{field}'").into(),
            currentvalue: None,
            expected: Some(Cow::Borrowed("required value")),
            user_error: true,
        }))
    }

    /// Create a feature not enabled error
    ///
    /// Used when optional features are accessed but not enabled
    #[must_use]
    pub fn feature_not_enabled(
        feature: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::NotImplemented(Box::new(NotImplementedErrorDetails {
            feature: feature.into(),
            message: Some(message.into()),
            planned_version: None,
        }))
    }

    /// Create a storage operation error with operation name
    #[must_use]
    pub fn storage_operation(message: impl Into<Cow<'static, str>>, _recoverable: bool) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: message.into(),
            operation: Some(Cow::Borrowed("storage_operation")),
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
    pub fn simple(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Internal(Box::new(InternalErrorDetails {
            message: message.into(),
            component: Cow::Borrowed("simple"),
            location: None,
            is_bug: false,
            context: None,
        }))
    }
}
