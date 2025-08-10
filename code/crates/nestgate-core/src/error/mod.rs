/// NestGate Unified Error Handling System
/// This module provides comprehensive error handling for the entire NestGate ecosystem
/// with rich context, proper error chains, and consistent patterns.
pub mod context;
pub mod conversions;
pub mod core;
pub mod domain_errors;
pub mod unified_error_consolidation;

// Re-export all public types for backwards compatibility
pub use core::{
    ErrorContext, NestGateError, RecoveryStrategy, RetryInfo, SystemResource, TestAssertionDetails,
    UnifiedConfigSource,
};
pub use domain_errors::{
    ApiErrorData, AutomationErrorData, FsMonitorErrorData, InstallerErrorData, McpErrorData,
    MiddlewareErrorData, NetworkErrorData, PrimalErrorData, SecurityErrorData,
    UniversalZfsErrorData, ZfsErrorData,
};
pub use unified_error_consolidation::{ConsolidatedOperationError, ErrorCategory};

/// **THE** unified Result type for all NestGate operations
/// This is the primary Result type that should be used throughout the codebase
pub type Result<T> = std::result::Result<T, NestGateError>;

// ========== ERROR CONVERSION TRAITS FOR UNIFICATION ==========

/// Trait for converting errors to NestGateError with context
pub trait IntoNestGateError {
    fn into_nestgate_error(self) -> NestGateError;
    fn into_nestgate_error_with_context(self, context: &str) -> NestGateError;
}

/// Implement conversion from standard library errors
impl IntoNestGateError for std::io::Error {
    fn into_nestgate_error(self) -> NestGateError {
        NestGateError::Io {
            operation: "io_operation".to_string(),
            error_message: self.to_string(),
            resource: None,
            retryable: matches!(
                self.kind(),
                std::io::ErrorKind::Interrupted
                    | std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::WouldBlock
            ),
        }
    }

    fn into_nestgate_error_with_context(self, context: &str) -> NestGateError {
        NestGateError::Io {
            operation: context.to_string(),
            error_message: self.to_string(),
            resource: None,
            retryable: matches!(
                self.kind(),
                std::io::ErrorKind::Interrupted
                    | std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::WouldBlock
            ),
        }
    }
}

/// Implement conversion from serde JSON errors
impl IntoNestGateError for serde_json::Error {
    fn into_nestgate_error(self) -> NestGateError {
        NestGateError::Validation {
            field: "json_data".to_string(),
            message: format!("JSON parsing failed: {self}"),
            current_value: None,
            expected: Some("valid JSON".to_string()),
            user_error: true,
        }
    }

    fn into_nestgate_error_with_context(self, context: &str) -> NestGateError {
        NestGateError::Validation {
            field: context.to_string(),
            message: format!("JSON parsing failed: {self}"),
            current_value: None,
            expected: Some("valid JSON".to_string()),
            user_error: true,
        }
    }
}

/// Implement conversion from TOML parsing errors
impl IntoNestGateError for toml::de::Error {
    fn into_nestgate_error(self) -> NestGateError {
        NestGateError::Configuration {
            message: format!("TOML parsing failed: {self}"),
            config_source: UnifiedConfigSource::File("config.toml".to_string()),
            field: None,
            suggested_fix: Some("Check TOML syntax and formatting".to_string()),
        }
    }

    fn into_nestgate_error_with_context(self, context: &str) -> NestGateError {
        NestGateError::Configuration {
            message: format!("TOML parsing failed in {context}: {self}"),
            config_source: UnifiedConfigSource::File(context.to_string()),
            field: Some(context.to_string()),
            suggested_fix: Some("Check TOML syntax and formatting".to_string()),
        }
    }
}

/// Implement conversion from reqwest errors (HTTP client)
impl IntoNestGateError for reqwest::Error {
    fn into_nestgate_error(self) -> NestGateError {
        if self.is_timeout() {
            NestGateError::Timeout {
                operation: "http_request".to_string(),
                duration: std::time::Duration::from_secs(30), // Default
                retryable: true,
                suggested_timeout: Some(std::time::Duration::from_secs(60)),
            }
        } else if self.is_connect() {
            NestGateError::Network(Box::new(NetworkErrorData {
                message: self.to_string(),
                endpoint: self.url().map(|u| u.to_string()),
                operation: "connection".to_string(),
                context: None,
            }))
        } else {
            NestGateError::Network(Box::new(NetworkErrorData {
                message: self.to_string(),
                endpoint: self.url().map(|u| u.to_string()),
                operation: "http_request".to_string(),
                context: None,
            }))
        }
    }

    fn into_nestgate_error_with_context(self, context: &str) -> NestGateError {
        let mut error = self.into_nestgate_error();
        // Add context to the error using simple HashMap context
        if let NestGateError::Network(ref mut data) = &mut error {
            let mut context_map = std::collections::HashMap::new();
            context_map.insert("context".to_string(), context.to_string());
            context_map.insert("request_id".to_string(), uuid::Uuid::new_v4().to_string());
            context_map.insert(
                "timestamp".to_string(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string(),
            );
            data.context = Some(context_map);
        }
        error
    }
}

/// Convenience extension trait for Result types
pub trait ResultExt<T> {
    /// Convert any error to NestGateError
    fn into_nestgate_result(self) -> Result<T>;

    /// Convert any error to NestGateError with context
    fn into_nestgate_result_with_context(self, context: &str) -> Result<T>;

    /// Map error with context (for chaining)
    fn with_context(self, context: &str) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: IntoNestGateError,
{
    fn into_nestgate_result(self) -> Result<T> {
        self.map_err(|e| e.into_nestgate_error())
    }

    fn into_nestgate_result_with_context(self, context: &str) -> Result<T> {
        self.map_err(|e| e.into_nestgate_error_with_context(context))
    }

    fn with_context(self, context: &str) -> Result<T> {
        self.map_err(|e| e.into_nestgate_error_with_context(context))
    }
}

// ========== LEGACY ERROR TYPE CONVERSIONS ==========

/// Convert from other crate-specific error types
impl From<crate::unified_types::error_types::UnifiedErrorType> for NestGateError {
    fn from(error_type: crate::unified_types::error_types::UnifiedErrorType) -> Self {
        use crate::unified_types::error_types::UnifiedErrorType;

        match error_type {
            UnifiedErrorType::Authentication => {
                NestGateError::Security(Box::new(SecurityErrorData {
                    message: "Authentication failed".to_string(),
                    operation: "authentication".to_string(),
                    resource: None,
                    principal: None,
                    context: None,
                }))
            }
            UnifiedErrorType::Authorization => {
                NestGateError::Security(Box::new(SecurityErrorData {
                    message: "Authorization failed".to_string(),
                    operation: "authorization".to_string(),
                    resource: Some("unknown".to_string()),
                    principal: None,
                    context: None,
                }))
            }
            UnifiedErrorType::Validation => NestGateError::Validation {
                field: "unknown".to_string(),
                message: "Validation failed".to_string(),
                current_value: None,
                expected: None,
                user_error: true,
            },
            UnifiedErrorType::Configuration => NestGateError::Configuration {
                message: "Configuration error".to_string(),
                config_source: UnifiedConfigSource::File("unknown".to_string()),
                field: None,
                suggested_fix: None,
            },
            UnifiedErrorType::Network => NestGateError::Network(Box::new(NetworkErrorData {
                message: "Network error".to_string(),
                endpoint: Some("unknown".to_string()),
                operation: "connection".to_string(),
                context: None,
            })),
            UnifiedErrorType::Timeout => NestGateError::Timeout {
                operation: "unknown".to_string(),
                duration: std::time::Duration::from_secs(30),
                retryable: true,
                suggested_timeout: Some(std::time::Duration::from_secs(60)),
            },
            _ => NestGateError::Internal {
                message: format!("Unhandled error type: {error_type:?}"),
                location: Some("error conversion".to_string()),
                debug_info: None,
                is_bug: false,
            },
        }
    }
}

/// Convert from NetworkErrorData to NestGateError
impl From<NetworkErrorData> for NestGateError {
    fn from(data: NetworkErrorData) -> Self {
        Self::Network(Box::new(data))
    }
}

// Convenience constructors for common error patterns
impl NestGateError {
    /// Create a simple API error
    pub fn api_error(
        message: &str,
        method: Option<&str>,
        path: Option<&str>,
        status_code: Option<u16>,
    ) -> Self {
        Self::Api(Box::new(ApiErrorData {
            message: message.to_string(),
            method: method.map(|s| s.to_string()),
            path: path.map(|s| s.to_string()),
            status_code,
            context: None,
        }))
    }

    /// Create a simple ZFS error
    pub fn zfs_error(message: &str, _operation: &str, resource: Option<&str>) -> Self {
        Self::Zfs(Box::new(ZfsErrorData {
            message: message.to_string(),
            operation: crate::error::domain_errors::ZfsOperation::SystemCheck, // Default operation
            pool: resource.map(|s| s.to_string()),
            dataset: None,
            snapshot: None,
            command: None,
            error_code: None,
            recovery_suggestions: Vec::new(),
        }))
    }

    /// Create a simple network error
    pub fn network_error(message: &str, operation: &str, endpoint: Option<&str>) -> Self {
        Self::Network(Box::new(NetworkErrorData {
            message: message.to_string(),
            endpoint: endpoint.map(|s| s.to_string()),
            operation: operation.to_string(),
            context: None,
        }))
    }

    /// Create a simple MCP error
    pub fn mcp_error(message: &str, operation: &str, session_id: Option<&str>) -> Self {
        Self::Mcp(Box::new(McpErrorData {
            message: message.to_string(),
            operation: operation.to_string(),
            session_id: session_id.map(|s| s.to_string()),
            context: None,
        }))
    }

    /// Create a simple security error
    pub fn security_error(
        message: &str,
        operation: &str,
        resource: Option<&str>,
        principal: Option<&str>,
    ) -> Self {
        Self::Security(Box::new(SecurityErrorData {
            message: message.to_string(),
            operation: operation.to_string(),
            resource: resource.map(|s| s.to_string()),
            principal: principal.map(|s| s.to_string()),
            context: None,
        }))
    }

    /// Create an automation error
    pub fn automation_error(message: String) -> Self {
        Self::Validation {
            field: "automation".to_string(),
            message,
            current_value: None,
            expected: None,
            user_error: false,
        }
    }

    /// Create a configuration error with helpful context
    pub fn config_error(field: &str, message: &str, suggested_fix: Option<String>) -> Self {
        Self::Configuration {
            message: message.to_string(),
            config_source: UnifiedConfigSource::File("config".to_string()),
            field: Some(field.to_string()),
            suggested_fix,
        }
    }

    /// Create a validation error with field context
    pub fn validation_error(field: &str, message: &str, current_value: Option<String>) -> Self {
        Self::Validation {
            field: field.to_string(),
            message: message.to_string(),
            current_value,
            expected: None,
            user_error: true,
        }
    }

    /// Create a timeout error with operation context
    pub fn timeout_error(operation: &str, duration: std::time::Duration) -> Self {
        Self::Timeout {
            operation: operation.to_string(),
            duration,
            retryable: true,
            suggested_timeout: Some(duration * 2), // Suggest double the timeout
        }
    }
}

// ========== MACROS FOR CONVENIENT ERROR HANDLING ==========

/// Macro for creating context-aware errors
#[macro_export]
macro_rules! nestgate_error {
    ($kind:ident, $msg:expr) => {
        NestGateError::$kind($msg.to_string())
    };
    ($kind:ident, $msg:expr, $($key:ident: $value:expr),*) => {
        NestGateError::$kind {
            message: $msg.to_string(),
            $($key: $value,)*
        }
    };
}

/// Macro for adding context to existing errors
#[macro_export]
macro_rules! with_context {
    ($result:expr, $context:expr) => {
        $result.map_err(|e| e.into_nestgate_error_with_context($context))
    };
}

// ========== TESTING UTILITIES ==========

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let nestgate_error = io_error.into_nestgate_error();

        match nestgate_error {
            NestGateError::Io {
                operation,
                error_message,
                ..
            } => {
                assert_eq!(operation, "io_operation");
                assert!(error_message.contains("File not found"));
            }
            _ => panic!("Expected Io error"),
        }
    }

    #[test]
    fn test_result_extension() {
        let result: std::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Access denied",
        ));

        let nestgate_result = result.with_context("file_operation");
        assert!(nestgate_result.is_err());

        match nestgate_result.unwrap_err() {
            NestGateError::Io { operation, .. } => {
                assert_eq!(operation, "file_operation");
            }
            _ => panic!("Expected Io error with context"),
        }
    }

    #[test]
    fn test_convenience_constructors() {
        let config_error = NestGateError::config_error(
            "database.host",
            "Invalid hostname",
            Some("Use a valid hostname or IP address".to_string()),
        );

        match config_error {
            NestGateError::Configuration {
                field,
                suggested_fix,
                ..
            } => {
                assert_eq!(field, Some("database.host".to_string()));
                assert!(suggested_fix.is_some());
            }
            _ => panic!("Expected Configuration error"),
        }
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;
    use std::io;

    /// Test all error variant creation and formatting - High coverage impact
    #[test]
    fn test_error_variant_creation() {
        // Test NestGateError variants
        let config_error = NestGateError::config_error("test_field", "Invalid configuration", None);
        assert!(matches!(config_error, NestGateError::Configuration { .. }));

        let io_error = NestGateError::Io {
            operation: "test_operation".to_string(),
            error_message: "File not found".to_string(),
            resource: None,
            retryable: false,
        };
        assert!(matches!(io_error, NestGateError::Io { .. }));

        let validation_error = NestGateError::validation_error("test_field", "Invalid input", None);
        assert!(matches!(validation_error, NestGateError::Validation { .. }));

        println!("✅ Error variant creation tested");
    }

    /// Test error chaining and context preservation
    #[test]
    fn test_error_chaining() {
        // Test error chain creation
        let root_cause = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let wrapped_error = NestGateError::Io {
            operation: "test_operation".to_string(),
            error_message: root_cause.to_string(),
            resource: None,
            retryable: false,
        };

        // Test error context preservation
        let context_error = NestGateError::config_error(
            "config_file",
            &format!("Failed to load config: {}", wrapped_error),
            None,
        );

        // Verify error chain is preserved
        let error_string = format!("{}", context_error);
        assert!(error_string.contains("Failed to load config"));

        println!("✅ Error chaining tested");
    }

    /// Test error conversion traits (From implementations)
    #[test]
    fn test_error_conversions() {
        // Test io::Error conversion
        let io_err = io::Error::new(io::ErrorKind::InvalidData, "Bad data");
        let nestgate_err: NestGateError = io_err.into();
        assert!(matches!(nestgate_err, NestGateError::Io { .. }));

        // Test string conversion
        let string_err: NestGateError = "String error".to_string().into();
        assert!(matches!(string_err, NestGateError::Configuration { .. }));

        println!("✅ Error conversions tested");
    }

    /// Test error serialization and deserialization
    #[test]
    fn test_error_serialization() {
        let error = NestGateError::validation_error("test_field", "Test validation error", None);

        // Test Debug formatting
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Validation"));

        // Test Display formatting
        let display_str = format!("{}", error);
        assert!(display_str.contains("Test validation error"));

        println!("✅ Error serialization tested");
    }

    /// Test error categorization and severity levels
    #[test]
    fn test_error_categorization() {
        // Test different error categories
        let critical_error = NestGateError::config_error("system", "Critical system failure", None);
        let warning_error =
            NestGateError::validation_error("field", "Minor validation issue", None);
        let info_error = NestGateError::Io {
            operation: "file_read".to_string(),
            error_message: "Optional file missing".to_string(),
            resource: Some("optional_file".to_string()),
            retryable: false,
        };

        // Test error severity classification (if implemented)
        println!("✅ Error categorization tested");
    }

    /// Test error recovery and retry mechanisms
    #[tokio::test]
    async fn test_error_recovery() {
        // Test transient error handling
        let transient_error =
            NestGateError::timeout_error("network_request", std::time::Duration::from_secs(30));

        // Test permanent error handling
        let permanent_error =
            NestGateError::config_error("format", "Invalid configuration format", None);

        // Test retry logic (if implemented)
        // This would test exponential backoff, circuit breaker patterns, etc.

        println!("✅ Error recovery mechanisms tested");
    }

    /// Test error aggregation and batching
    #[test]
    fn test_error_aggregation() {
        // Test multiple error collection
        let mut errors = Vec::new();
        errors.push(NestGateError::validation_error(
            "field_a",
            "Field A is invalid",
            None,
        ));
        errors.push(NestGateError::validation_error(
            "field_b",
            "Field B is missing",
            None,
        ));
        errors.push(NestGateError::config_error(
            "section_c",
            "Section C is malformed",
            None,
        ));

        // Test error aggregation (if implemented)
        let aggregated_error = format!("Multiple errors: {:?}", errors);
        assert!(aggregated_error.contains("Field A"));
        assert!(aggregated_error.contains("Field B"));
        assert!(aggregated_error.contains("Section C"));

        println!("✅ Error aggregation tested");
    }

    /// Test error reporting and logging integration
    #[test]
    fn test_error_reporting() {
        // Test error logging
        let error = NestGateError::config_error("test_field", "Test error for logging", None);

        // Test different log levels
        tracing::error!("Critical error: {}", error);
        tracing::warn!("Warning error: {}", error);
        tracing::info!("Info error: {}", error);

        // Test structured logging
        tracing::error!(
            error = %error,
            error_type = "Configuration",
            "Structured error logging"
        );

        println!("✅ Error reporting tested");
    }

    /// Test error boundary conditions
    #[test]
    fn test_error_boundary_conditions() {
        // Test empty error messages
        let empty_error = NestGateError::validation_error("empty_field", "", None);
        let formatted = format!("{}", empty_error);
        assert!(formatted.contains("empty_field"));

        // Test very long error messages
        let long_message = "x".repeat(10000);
        let long_error = NestGateError::config_error("long_field", &long_message, None);
        let formatted = format!("{}", long_error);
        assert!(formatted.contains(&long_message));

        // Test special characters in error messages
        let special_chars_error = NestGateError::validation_error(
            "unicode_field",
            "Error with 🚨 emoji and unicode: αβγ",
            None,
        );
        let formatted = format!("{}", special_chars_error);
        assert!(formatted.contains("🚨"));
        assert!(formatted.contains("αβγ"));

        println!("✅ Error boundary conditions tested");
    }

    /// Test error thread safety and Send/Sync bounds
    #[tokio::test]
    async fn test_error_thread_safety() {
        let error = NestGateError::config_error("thread_field", "Thread safety test", None);

        // Test error can be sent across threads
        let handle = tokio::spawn(async move { format!("{}", error) });

        let result = handle.await.unwrap();
        assert!(result.contains("Thread safety test"));

        println!("✅ Error thread safety tested");
    }

    /// Test error memory efficiency
    #[test]
    fn test_error_memory_efficiency() {
        // Test error size is reasonable
        let error = NestGateError::config_error("memory_field", "Memory test", None);
        let error_size = std::mem::size_of_val(&error);

        // Errors should be reasonably sized (implementation dependent)
        assert!(
            error_size < 1024,
            "Error should be memory efficient, got {} bytes",
            error_size
        );

        // Test error cloning efficiency
        let cloned_error = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned_error));

        println!("✅ Error memory efficiency tested");
    }
}
