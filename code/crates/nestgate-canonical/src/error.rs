//! Canonical Error Types for `NestGate`,
//!
//! Unified error handling system that consolidates all error types across
//! the `NestGate` ecosystem into a single, coherent error hierarchy.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Canonical `NestGate` Error Type,
///
/// This replaces all fragmented error types across crates with a unified system
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Errors that can occur during NestGate operations
pub enum NestGateError {
    /// Configuration-related errors
    Configuration {
        message: String,
        field: Option<String>,
    },
    /// Network and communication errors
    Network {
        message: String,
        endpoint: Option<String>,
    },

    /// Storage and file system errors
    Storage {
        message: String,
        path: Option<String>,
    },

    /// Security and authentication errors
    Security {
        message: String,
        details: Option<String>,
    },

    /// Internal system errors
    Internal { message: String, component: String },

    /// Validation errors
    Validation { message: String, field: String },
}

impl fmt::Display for NestGateError {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration { message, field, .. } => {
                write!(f, "Configuration Error: {message}")?;
                if let Some(field) = field {
                    write!(f, " (field: {field})")?;
                }
                Ok(())
            }
            NestGateError::Network {
                message, endpoint, ..
            } => {
                write!(f, "Network Error: {message}")?;
                if let Some(endpoint) = endpoint {
                    write!(f, " (endpoint: {endpoint})")?;
                }
                Ok(())
            }
            NestGateError::Storage { message, path, .. } => {
                write!(f, "Storage Error: {message}")?;
                if let Some(path_info) = path {
                    write!(f, " (Path: {path_info})")?;
                }
                Ok(())
            }
            NestGateError::Security {
                message, details, ..
            } => {
                write!(f, "Security Error: {message}")?;
                if let Some(details_info) = details {
                    write!(f, " (Details: {details_info})")?;
                }
                Ok(())
            }
            NestGateError::Internal { message, component } => {
                write!(f, "Internal Error: {message} (Component: {component})")?;
                Ok(())
            }
            NestGateError::Validation { message, field } => {
                write!(f, "Validation Error: {message} (Field: {field})")?;
                Ok(())
            }
        }
    }
}

impl std::error::Error for NestGateError {}

// ==================== SECTION: CANONICAL ERROR TYPES ====================

// CANONICAL MODERNIZATION: Use local Result type since this is the canonical crate
// This crate defines the canonical error types

/// **CANONICAL RESULT TYPE**
///
/// Both T and E are generic for maximum ecosystem compatibility.
/// Uses `NestGateError` as the default error type.
pub type Result<T, E = NestGateError> = std::result::Result<T, E>;
/// Type alias for Nestgateresult
pub type NestGateResult<T> = Result<T>;
// ==================== SECTION ====================

impl NestGateError {
    /// Create a configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
            field: None,
        }
    }

    /// Create a configuration error with field context
    pub fn configuration_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
            field: Some(field.into()),
        }
    }

    /// Create a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            endpoint: None,
        }
    }

    /// Create a network error with endpoint context
    pub fn network_endpoint(message: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self::Network {
            message: message.into(),
            endpoint: Some(endpoint.into()),
        }
    }

    /// Create a storage error
    pub fn storage(message: impl Into<String>) -> Self {
        Self::Storage {
            message: message.into(),
            path: None,
        }
    }

    /// Create a storage error with path context
    pub fn storage_path(message: impl Into<String>, path: impl Into<String>) -> Self {
        Self::Storage {
            message: message.into(),
            path: Some(path.into()),
        }
    }

    /// Create a security error
    pub fn security(message: impl Into<String>) -> Self {
        Self::Security {
            message: message.into(),
            details: None,
        }
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
            component: "unknown".to_string(),
        }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
            field: "unknown".to_string(),
        }
    }

    /// Create a file system error (alias for storage)
    pub fn file_system(message: impl Into<String>, path: Option<impl Into<String>>) -> Self {
        Self::Storage {
            message: message.into(),
            path: path.map(std::convert::Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_error() {
        let err = NestGateError::configuration("invalid config");
        assert!(err.to_string().contains("Configuration Error"));
        assert!(err.to_string().contains("invalid config"));
    }

    #[test]
    fn test_configuration_error_with_field() {
        let err = NestGateError::configuration_field("missing value", "port");
        let msg = err.to_string();
        assert!(msg.contains("Configuration Error"));
        assert!(msg.contains("missing value"));
        assert!(msg.contains("port"));
    }

    #[test]
    fn test_network_error() {
        let err = NestGateError::network("connection refused");
        assert!(err.to_string().contains("Network Error"));
        assert!(err.to_string().contains("connection refused"));
    }

    #[test]
    fn test_network_error_with_endpoint() {
        let err = NestGateError::network_endpoint("timeout", "localhost:8080");
        let msg = err.to_string();
        assert!(msg.contains("Network Error"));
        assert!(msg.contains("timeout"));
        assert!(msg.contains("localhost:8080"));
    }

    #[test]
    fn test_storage_error() {
        let err = NestGateError::storage("disk full");
        assert!(err.to_string().contains("Storage Error"));
        assert!(err.to_string().contains("disk full"));
    }

    #[test]
    fn test_storage_error_with_path() {
        let err = NestGateError::storage_path("read failed", "/data/file.txt");
        let msg = err.to_string();
        assert!(msg.contains("Storage Error"));
        assert!(msg.contains("read failed"));
        assert!(msg.contains("/data/file.txt"));
    }

    #[test]
    fn test_security_error() {
        let err = NestGateError::security("unauthorized");
        assert!(err.to_string().contains("Security Error"));
        assert!(err.to_string().contains("unauthorized"));
    }

    #[test]
    fn test_internal_error() {
        let err = NestGateError::internal("unexpected state");
        let msg = err.to_string();
        assert!(msg.contains("Internal Error"));
        assert!(msg.contains("unexpected state"));
    }

    #[test]
    fn test_validation_error() {
        let err = NestGateError::validation("field cannot be empty");
        let msg = err.to_string();
        assert!(msg.contains("Validation Error"));
        assert!(msg.contains("field cannot be empty"));
    }

    #[test]
    fn test_file_system_error() {
        let err = NestGateError::file_system("file not found", Some("/path/to/file"));
        assert!(err.to_string().contains("Storage Error"));
    }

    #[test]
    fn test_error_clone() {
        let err1 = NestGateError::network("test");
        let err2 = err1.clone();
        assert_eq!(err1.to_string(), err2.to_string());
    }

    #[test]
    fn test_error_debug() {
        let err = NestGateError::validation("test");
        let debug = format!("{err:?}");
        assert!(debug.contains("Validation"));
    }

    #[test]
    #[allow(clippy::unnecessary_literal_unwrap)]
    fn test_result_ok() {
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.expect("Operation failed"), 42);
    }

    #[test]
    fn test_result_err() {
        let result: Result<i32> = Err(NestGateError::validation("error"));
        assert!(result.is_err());
    }

    #[test]
    fn test_nestgate_result_type() {
        let result: NestGateResult<String> = Ok("success".to_string());
        assert!(result.is_ok());
    }

    // ==================== Additional Comprehensive Tests ====================

    #[test]
    fn test_configuration_error_without_field() {
        let err = NestGateError::Configuration {
            message: "general error".to_string(),
            field: None,
        };
        let msg = err.to_string();
        assert!(msg.contains("Configuration Error"));
        assert!(msg.contains("general error"));
    }

    #[test]
    fn test_network_error_without_endpoint() {
        let err = NestGateError::Network {
            message: "network unavailable".to_string(),
            endpoint: None,
        };
        let msg = err.to_string();
        assert!(msg.contains("Network Error"));
        assert!(msg.contains("network unavailable"));
    }

    #[test]
    fn test_storage_error_without_path() {
        let err = NestGateError::Storage {
            message: "storage failure".to_string(),
            path: None,
        };
        let msg = err.to_string();
        assert!(msg.contains("Storage Error"));
        assert!(msg.contains("storage failure"));
    }

    #[test]
    fn test_security_error_with_details() {
        let err = NestGateError::Security {
            message: "auth failed".to_string(),
            details: Some("invalid token".to_string()),
        };
        let msg = err.to_string();
        assert!(msg.contains("Security Error"));
        assert!(msg.contains("auth failed"));
        assert!(msg.contains("invalid token"));
    }

    #[test]
    fn test_security_error_without_details() {
        let err = NestGateError::Security {
            message: "unauthorized".to_string(),
            details: None,
        };
        let msg = err.to_string();
        assert!(msg.contains("Security Error"));
        assert!(msg.contains("unauthorized"));
    }

    #[test]
    fn test_internal_error_component() {
        let err = NestGateError::Internal {
            message: "crash".to_string(),
            component: "database".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Internal Error"));
        assert!(msg.contains("crash"));
        assert!(msg.contains("database"));
    }

    #[test]
    fn test_validation_error_field() {
        let err = NestGateError::Validation {
            message: "invalid format".to_string(),
            field: "email".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Validation Error"));
        assert!(msg.contains("invalid format"));
        assert!(msg.contains("email"));
    }

    #[test]
    fn test_error_trait_implementation() {
        let err = NestGateError::validation("test");
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_error_source_is_none() {
        let err = NestGateError::configuration("test");
        use std::error::Error;
        assert!(err.source().is_none());
    }

    #[test]
    fn test_serialization_configuration() {
        let err = NestGateError::configuration("test");
        let json = serde_json::to_string(&err).expect("String operation failed");
        assert!(json.contains("Configuration"));
    }

    #[test]
    fn test_deserialization_configuration() {
        let json = r#"{"Configuration":{"message":"test","field":null}}"#;
        let err: NestGateError = serde_json::from_str(json).expect("Failed to convert from string");
        assert!(matches!(err, NestGateError::Configuration { .. }));
    }

    #[test]
    fn test_serialization_network() {
        let err = NestGateError::network_endpoint("test", "endpoint");
        let json = serde_json::to_string(&err).expect("String operation failed");
        assert!(json.contains("Network"));
    }

    #[test]
    fn test_serialization_storage() {
        let err = NestGateError::storage_path("test", "/path");
        let json = serde_json::to_string(&err).expect("String operation failed");
        assert!(json.contains("Storage"));
    }

    #[test]
    fn test_serialization_security() {
        let err = NestGateError::security("test");
        let json = serde_json::to_string(&err).expect("String operation failed");
        assert!(json.contains("Security"));
    }

    #[test]
    fn test_serialization_internal() {
        let err = NestGateError::internal("test");
        let json = serde_json::to_string(&err).expect("String operation failed");
        assert!(json.contains("Internal"));
    }

    #[test]
    fn test_serialization_validation() {
        let err = NestGateError::validation("test");
        let json = serde_json::to_string(&err).expect("String operation failed");
        assert!(json.contains("Validation"));
    }

    #[test]
    fn test_round_trip_serialization() {
        let original = NestGateError::network_endpoint("timeout", "localhost:8080");
        let json = serde_json::to_string(&original).expect("String operation failed");
        let deserialized: NestGateError =
            serde_json::from_str(&json).expect("Failed to convert from string");
        assert_eq!(original.to_string(), deserialized.to_string());
    }

    #[test]
    fn test_clone_all_variants() {
        let errors = vec![
            NestGateError::configuration("test"),
            NestGateError::network("test"),
            NestGateError::storage("test"),
            NestGateError::security("test"),
            NestGateError::internal("test"),
            NestGateError::validation("test"),
        ];

        for err in errors {
            let cloned = err.clone();
            assert_eq!(err.to_string(), cloned.to_string());
        }
    }

    #[test]
    fn test_debug_all_variants() {
        let errors = vec![
            NestGateError::configuration("test"),
            NestGateError::network("test"),
            NestGateError::storage("test"),
            NestGateError::security("test"),
            NestGateError::internal("test"),
            NestGateError::validation("test"),
        ];

        for err in errors {
            let debug_str = format!("{:?}", err);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_empty_message() {
        let err = NestGateError::configuration("");
        let msg = err.to_string();
        assert!(msg.contains("Configuration Error"));
    }

    #[test]
    fn test_long_message() {
        let long_msg = "x".repeat(500);
        let err = NestGateError::configuration(&long_msg);
        let msg = err.to_string();
        assert!(msg.contains(&long_msg));
    }

    #[test]
    fn test_special_characters() {
        let err = NestGateError::configuration("Error: <test> & \"quote\"");
        let msg = err.to_string();
        assert!(msg.contains("<test>"));
        assert!(msg.contains("&"));
    }

    #[test]
    fn test_unicode_message() {
        let err = NestGateError::configuration("错误: test 🚀");
        let msg = err.to_string();
        assert!(msg.contains("错误"));
        assert!(msg.contains("🚀"));
    }

    #[test]
    fn test_result_error_propagation() {
        /// Returns Error
        fn returns_error() -> Result<()> {
            Err(NestGateError::validation("test"))
        }

        let result = returns_error();
        assert!(result.is_err());
    }

    #[test]
    fn test_result_ok_value() {
        /// Returns Ok
        fn returns_ok() -> Result<String> {
            Ok("success".to_string())
        }

        let result = returns_ok();
        assert!(result.is_ok());
        assert_eq!(result.expect("Operation failed"), "success");
    }

    #[test]
    fn test_result_map() {
        let result: Result<i32> = Ok(42);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped.expect("Operation failed"), 84);
    }

    #[test]
    fn test_result_and_then() {
        let result: Result<i32> = Ok(42);
        let chained = result.map(|x| x + 1);
        assert_eq!(chained.unwrap_or(0), 43);
    }

    #[test]
    fn test_file_system_error_with_path() {
        let err = NestGateError::file_system("read error", Some("/tmp/file.txt"));
        let msg = err.to_string();
        assert!(msg.contains("Storage Error"));
        assert!(msg.contains("read error"));
        assert!(msg.contains("/tmp/file.txt"));
    }

    #[test]
    fn test_file_system_error_without_path() {
        let err = NestGateError::file_system("disk error", None::<String>);
        let msg = err.to_string();
        assert!(msg.contains("Storage Error"));
        assert!(msg.contains("disk error"));
    }

    #[test]
    fn test_nestgate_result_with_error() {
        let result: NestGateResult<i32> = Err(NestGateError::validation("failed"));
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.to_string().contains("Validation Error"));
        }
    }

    #[test]
    #[allow(clippy::unnecessary_literal_unwrap)]
    fn test_nestgate_result_with_ok() {
        let result: NestGateResult<String> = Ok("data".to_string());
        assert!(result.is_ok());
        assert_eq!(result.expect("Operation failed"), "data");
    }
}
