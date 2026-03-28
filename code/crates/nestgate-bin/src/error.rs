// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module demonstrates the canonical error handling approach used throughout
// the NestGate ecosystem. It shows how domain-specific errors integrate with
// the unified `NestGateError` system while maintaining type safety and context.
//
// **CANONICAL PATTERNS**:
// - Domain-specific error types with rich context
// - Automatic conversion to unified `NestGateError`
// - Consistent error formatting and serialization
// - Idiomatic Result types with proper error propagation

//! Error module

use nestgate_core::error::{CanonicalResult, NestGateError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// **CANONICAL BINARY ERROR TYPE**
///
/// Domain-specific error type that follows canonical patterns:
/// - Rich context with structured data
/// - Consistent error messages
/// - Automatic conversion to `NestGateError`
/// - Serializable for logging and debugging
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum NestGateBinError {
    #[error("Command execution failed: {message}")]
    CommandExecutionFailed {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        command: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        exit_code: Option<i32>,
    },

    #[error("Configuration error: {message}")]
    ConfigurationError {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        config_path: Option<String>,
    },

    #[error("Argument parsing error: {message}")]
    ArgumentParsingError {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        argument: Option<String>,
    },

    #[error("Service initialization error: {message}")]
    ServiceInitializationError {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        service_name: Option<String>,
    },

    #[error("Runtime error: {message}")]
    RuntimeError {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        operation: Option<String>,
    },
}

// ==================== SECTION ====================

/// **UNIFIED ERROR SYSTEM MIGRATION**
///
/// The bin crate now uses the unified `NestGateError` system for all operations,
/// eliminating the fragmented `NestGateBinError` type.

/// **CANONICAL RESULT** - Use unified Result from nestgate-core
pub type Result<T> = nestgate_core::error::Result<T>;

/// **BIN RESULT** - Domain-specific result for bin operations
pub type BinResult<T> = CanonicalResult<T>;

// ==================== SECTION ====================

/// Helper functions for creating bin-specific errors using unified system
pub struct BinErrorHelper;

impl BinErrorHelper {
    /// Create command execution error using unified system
    pub fn command_execution_error(
        message: impl Into<String>,
        command: Option<String>,
        exit_code: Option<i32>,
    ) -> NestGateError {
        NestGateError::internal_error(
            format!(
                "Command execution failed: {} (command: {:?}, exit: {:?})",
                message.into(),
                command,
                exit_code
            ),
            "nestgate-bin",
        )
    }

    /// Create configuration error using unified system
    pub fn configuration_error(
        message: impl Into<String>,
        config_path: Option<String>,
    ) -> NestGateError {
        NestGateError::configuration_error(
            &config_path.unwrap_or_else(|| "bin_config".to_string()),
            &message.into(),
        )
    }

    /// Create argument parsing error using unified system
    pub fn argument_parsing_error(
        message: impl Into<String>,
        argument: Option<String>,
    ) -> NestGateError {
        NestGateError::validation(format!(
            "Argument parsing error: {} (arg: {})",
            message.into(),
            argument.unwrap_or_else(|| "unknown".to_string())
        ))
    }

    /// Create service initialization error using unified system
    pub fn service_initialization_error(
        message: impl Into<String>,
        service_name: Option<String>,
    ) -> NestGateError {
        NestGateError::internal_error(
            format!(
                "Service initialization error: {} (service: {})",
                message.into(),
                service_name.unwrap_or_else(|| "unknown_service".to_string())
            ),
            "nestgate-bin",
        )
    }

    /// Create runtime error using unified system
    pub fn runtime_error(message: impl Into<String>, operation: Option<String>) -> NestGateError {
        NestGateError::internal_error(
            format!(
                "Runtime error: {} (operation: {})",
                message.into(),
                operation.unwrap_or_else(|| "runtime".to_string())
            ),
            "nestgate-bin",
        )
    }
}

// ==================== SECTION ====================

/// **CANONICAL CONVERSION TRAIT IMPLEMENTATION**
///
/// Implements the canonical `IntoNestGateError` trait for seamless integration
impl From<NestGateBinError> for NestGateError {
    /// From
    fn from(err: NestGateBinError) -> NestGateError {
        match err {
            NestGateBinError::ArgumentParsingError { argument, message } => {
                NestGateError::validation(format!(
                    "Argument parsing: {} - {}",
                    argument.unwrap_or_else(|| "unknown".to_string()),
                    message
                ))
            }
            NestGateBinError::ConfigurationError {
                config_path,
                message,
            } => NestGateError::configuration_error(
                "config",
                &format!(
                    "{} - {}",
                    config_path.unwrap_or_else(|| "unknown".to_string()),
                    message
                ),
            ),
            NestGateBinError::RuntimeError { operation, message } => NestGateError::internal_error(
                format!(
                    "Runtime: {} - {}",
                    operation.unwrap_or_else(|| "runtime".to_string()),
                    message
                ),
                "nestgate-bin",
            ),
            NestGateBinError::ServiceInitializationError {
                service_name,
                message,
            } => NestGateError::internal_error(
                format!(
                    "Service: {} - {}",
                    service_name.unwrap_or_else(|| "service".to_string()),
                    message
                ),
                "nestgate-bin",
            ),
            NestGateBinError::CommandExecutionFailed {
                command,
                exit_code,
                message,
            } => NestGateError::internal_error(
                format!("Command: {command:?}, exit: {exit_code:?} - {message}"),
                "nestgate-bin",
            ),
        }
    }
}

// ==================== SECTION ====================

/// **CANONICAL ERROR BUILDERS**
///
/// Provides consistent error creation patterns following canonical conventions
impl NestGateBinError {
    /// Create a command execution error with rich context
    pub fn command_failed<S: Into<String>>(
        message: S,
        command: Option<String>,
        exit_code: Option<i32>,
    ) -> Self {
        Self::CommandExecutionFailed {
            message: message.into(),
            command,
            exit_code,
        }
    }

    /// Create a configuration error with optional path context
    pub fn config_error<S: Into<String>>(message: S, config_path: Option<String>) -> Self {
        Self::ConfigurationError {
            message: message.into(),
            config_path,
        }
    }

    /// Create an argument parsing error with optional argument context
    pub fn argument_error<S: Into<String>>(message: S, argument: Option<String>) -> Self {
        Self::ArgumentParsingError {
            message: message.into(),
            argument,
        }
    }

    /// Create a service initialization error with service context
    pub fn service_init_error<S: Into<String>>(message: S, service_name: Option<String>) -> Self {
        Self::ServiceInitializationError {
            message: message.into(),
            service_name,
        }
    }

    /// Create a runtime error with operation context
    pub fn runtime_error<S: Into<String>>(message: S, operation: Option<String>) -> Self {
        Self::RuntimeError {
            message: message.into(),
            operation,
        }
    }
}

// ==================== SECTION ====================

/// **CANONICAL RESULT EXTENSIONS**
///
/// Provides idiomatic extensions for working with `BinResult` types
pub trait BinResultExt<T> {
    /// Convert to canonical `NestGateError` with additional context
    fn with_context(self, context: &str) -> CanonicalResult<T>;

    /// Convert to canonical `NestGateError`
    fn into_canonical(self) -> CanonicalResult<T>;
}

impl<T> BinResultExt<T> for BinResult<T> {
    /// Builder method to set Context
    fn with_context(self, context: &str) -> CanonicalResult<T> {
        self.map_err(|_e| {
            NestGateError::internal_error(format!("Context: {context}"), "nestgate-bin")
        })
    }

    /// Into Canonical
    fn into_canonical(self) -> CanonicalResult<T> {
        self.map_err(|_e| NestGateError::internal_error("Conversion error", "nestgate-bin"))
    }
}

/// **CANONICAL ERROR SYSTEM COMPLETE**
///
/// This module demonstrates the complete canonical error handling approach:
/// 1. Domain-specific error types with rich context
/// 2. Automatic conversion to unified `NestGateError`
/// 3. Consistent error creation patterns
/// 4. Idiomatic Result type usage
/// 5. Seamless integration with the broader error ecosystem
pub struct CanonicalErrorDemo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_execution_failed() {
        let error =
            NestGateBinError::command_failed("test failed", Some("ls".to_string()), Some(1));
        assert!(error.to_string().contains("Command execution failed"));
        assert!(error.to_string().contains("test failed"));
    }

    #[test]
    fn test_command_execution_failed_minimal() {
        let error = NestGateBinError::command_failed("failed", None, None);
        assert!(error.to_string().contains("Command execution failed"));
    }

    #[test]
    fn test_configuration_error() {
        let error =
            NestGateBinError::config_error("invalid config", Some("/etc/config".to_string()));
        assert!(error.to_string().contains("Configuration error"));
        assert!(error.to_string().contains("invalid config"));
    }

    #[test]
    fn test_configuration_error_without_path() {
        let error = NestGateBinError::config_error("missing field", None);
        assert!(error.to_string().contains("Configuration error"));
    }

    #[test]
    fn test_argument_parsing_error() {
        let error = NestGateBinError::argument_error("invalid value", Some("--port".to_string()));
        assert!(error.to_string().contains("Argument parsing error"));
        assert!(error.to_string().contains("invalid value"));
    }

    #[test]
    fn test_argument_parsing_error_no_arg() {
        let error = NestGateBinError::argument_error("missing", None);
        assert!(error.to_string().contains("Argument parsing error"));
    }

    #[test]
    fn test_service_initialization_error() {
        let error = NestGateBinError::service_init_error(
            "failed to start",
            Some("http-server".to_string()),
        );
        assert!(error.to_string().contains("Service initialization error"));
        assert!(error.to_string().contains("failed to start"));
    }

    #[test]
    fn test_service_initialization_error_no_name() {
        let error = NestGateBinError::service_init_error("crashed", None);
        assert!(error.to_string().contains("Service initialization error"));
    }

    #[test]
    fn test_runtime_error() {
        let error =
            NestGateBinError::runtime_error("panic occurred", Some("data_processing".to_string()));
        assert!(error.to_string().contains("Runtime error"));
        assert!(error.to_string().contains("panic occurred"));
    }

    #[test]
    fn test_runtime_error_no_operation() {
        let error = NestGateBinError::runtime_error("general error", None);
        assert!(error.to_string().contains("Runtime error"));
    }

    #[test]
    fn test_error_clone() {
        let error = NestGateBinError::command_failed("test", None, None);
        let cloned = error.clone();
        assert_eq!(error.to_string(), cloned.to_string());
    }

    #[test]
    fn test_error_serialization() {
        let error = NestGateBinError::config_error("test", Some("path".to_string()));
        let serialized =
            serde_json::to_string(&error).expect("Test: error serialization should succeed");
        assert!(serialized.contains("Configuration"));
    }

    #[test]
    fn test_error_deserialization() {
        let json = r#"{"RuntimeError":{"message":"test","operation":"op"}}"#;
        let error: NestGateBinError =
            serde_json::from_str(json).expect("Test: error deserialization should succeed");
        assert!(error.to_string().contains("Runtime error"));
    }

    #[test]
    fn test_bin_error_helper_command_execution() {
        let error =
            BinErrorHelper::command_execution_error("failed", Some("cmd".to_string()), Some(127));
        assert!(error.to_string().contains("Command execution failed"));
    }

    #[test]
    fn test_bin_error_helper_configuration() {
        let error = BinErrorHelper::configuration_error("bad config", Some("/path".to_string()));
        assert!(error.to_string().contains("Configuration"));
    }

    #[test]
    fn test_bin_error_helper_argument_parsing() {
        let error = BinErrorHelper::argument_parsing_error("invalid", Some("--flag".to_string()));
        assert!(error.to_string().contains("Argument parsing error"));
    }

    #[test]
    fn test_bin_error_helper_service_init() {
        let error =
            BinErrorHelper::service_initialization_error("crashed", Some("api".to_string()));
        assert!(error.to_string().contains("Service initialization error"));
    }

    #[test]
    fn test_bin_error_helper_runtime() {
        let error = BinErrorHelper::runtime_error("error", Some("processing".to_string()));
        assert!(error.to_string().contains("Runtime error"));
    }

    #[test]
    fn test_conversion_to_nestgate_error_argument() {
        let bin_error = NestGateBinError::argument_error("test", Some("arg".to_string()));
        let nestgate_error: NestGateError = bin_error.into();
        assert!(nestgate_error.to_string().contains("Argument parsing"));
    }

    #[test]
    fn test_conversion_to_nestgate_error_config() {
        let bin_error = NestGateBinError::config_error("test", Some("path".to_string()));
        let nestgate_error: NestGateError = bin_error.into();
        assert!(nestgate_error.to_string().contains("Configuration"));
    }

    #[test]
    fn test_conversion_to_nestgate_error_runtime() {
        let bin_error = NestGateBinError::runtime_error("test", Some("op".to_string()));
        let nestgate_error: NestGateError = bin_error.into();
        assert!(nestgate_error.to_string().contains("Runtime"));
    }

    #[test]
    fn test_conversion_to_nestgate_error_service_init() {
        let bin_error = NestGateBinError::service_init_error("test", Some("service".to_string()));
        let nestgate_error: NestGateError = bin_error.into();
        assert!(nestgate_error.to_string().contains("Service"));
    }

    #[test]
    fn test_conversion_to_nestgate_error_command() {
        let bin_error = NestGateBinError::command_failed("test", Some("cmd".to_string()), Some(1));
        let nestgate_error: NestGateError = bin_error.into();
        assert!(nestgate_error.to_string().contains("Command"));
    }

    #[test]
    fn test_result_with_context() {
        let result: BinResult<()> = Err(NestGateError::internal_error("test", "test"));
        let with_ctx = result.with_context("additional context");
        assert!(with_ctx.is_err());
    }

    #[test]
    fn test_result_into_canonical() {
        let result: BinResult<()> = Err(NestGateError::internal_error("test", "test"));
        let canonical = result.into_canonical();
        assert!(canonical.is_err());
    }

    #[test]
    fn test_helper_functions_with_string_refs() {
        let error = BinErrorHelper::command_execution_error("test", None, None);
        assert!(error.to_string().contains("Command execution failed"));
    }

    #[test]
    fn test_all_error_variants_have_messages() {
        let errors = vec![
            NestGateBinError::command_failed("msg", None, None),
            NestGateBinError::config_error("msg", None),
            NestGateBinError::argument_error("msg", None),
            NestGateBinError::service_init_error("msg", None),
            NestGateBinError::runtime_error("msg", None),
        ];

        for error in errors {
            assert!(error.to_string().contains("msg"));
        }
    }

    #[test]
    fn test_error_with_exit_codes() {
        let codes = vec![0, 1, 127, 255];
        for code in codes {
            let error = NestGateBinError::command_failed("test", None, Some(code));
            let _ = error.to_string(); // Should not panic
        }
    }
}
