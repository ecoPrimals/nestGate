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

use nestgate_core::error::{CanonicalResult, NestGateError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// **CANONICAL BINARY ERROR TYPE**
///
/// Domain-specific error type that follows canonical patterns:
/// - Rich context with structured data
/// - Consistent error messages
/// - Automatic conversion to NestGateError
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
/// The bin crate now uses the unified NestGateError system for all operations,
/// eliminating the fragmented NestGateBinError type.

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
/// Implements the canonical IntoNestGateError trait for seamless integration
impl From<NestGateBinError> for NestGateError {
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
                format!(
                    "Command: {:?}, exit: {:?} - {}",
                    command, exit_code, message
                ),
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
/// Provides idiomatic extensions for working with BinResult types
pub trait BinResultExt<T> {
    /// Convert to canonical NestGateError with additional context
    fn with_context(self, context: &str) -> CanonicalResult<T>;

    /// Convert to canonical NestGateError
    fn into_canonical(self) -> CanonicalResult<T>;
}

impl<T> BinResultExt<T> for BinResult<T> {
    fn with_context(self, context: &str) -> CanonicalResult<T> {
        self.map_err(|_e| {
            NestGateError::internal_error(format!("Context: {}", context), "nestgate-bin")
        })
    }

    fn into_canonical(self) -> CanonicalResult<T> {
        self.map_err(|_e| NestGateError::internal_error("Conversion error", "nestgate-bin"))
    }
}

/// **CANONICAL ERROR SYSTEM COMPLETE**
///
/// This module demonstrates the complete canonical error handling approach:
/// 1. Domain-specific error types with rich context
/// 2. Automatic conversion to unified NestGateError
/// 3. Consistent error creation patterns
/// 4. Idiomatic Result type usage
/// 5. Seamless integration with the broader error ecosystem
pub struct CanonicalErrorDemo;
