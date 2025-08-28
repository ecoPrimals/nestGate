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

use nestgate_core::error::{CanonicalResult, NestGateError, IntoNestGateError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
pub type BinResult<T> = nestgate_core::error::IdioResult<T, nestgate_core::NestGateError>;

// ==================== SECTION ====================

/// Helper functions for creating bin-specific errors using unified system
pub struct BinErrorHelper;

impl BinErrorHelper {
    /// Create command execution error using unified system
    pub fn command_execution_error(message: impl Into<String>, command: Option<String>, exit_code: Option<i32>) -> NestGateError {
        NestGateError::Internal {
            message: format!("Command execution failed: {}", message.into()),
            component: format!("command: {:?}, exit_code: {:?}", command, exit_code),
            location: None,
            bug_report: false,
            context: None,
        }
    }

    /// Create configuration error using unified system
    pub fn configuration_error(message: impl Into<String>, config_path: Option<String>) -> NestGateError {
        NestGateError::Configuration {
            field: config_path.unwrap_or_else(|| "bin_config".to_string()),
            message: message.into(),
            current_value: None,
            expected: None,
            user_error: true,
        }
    }

    /// Create argument parsing error using unified system
    pub fn argument_parsing_error(message: impl Into<String>, argument: Option<String>) -> NestGateError {
        NestGateError::Validation {
            message: format!("Argument parsing error: {}", message.into()),
            field: argument.unwrap_or_else(|| "command_line_args".to_string()),
            value: None,
            current_value: None,
            expected: Some("valid command line arguments".to_string()),
            context: None,
        }
    }

    /// Create service initialization error using unified system
    pub fn service_initialization_error(message: impl Into<String>, service_name: Option<String>) -> NestGateError {
        NestGateError::Internal {
            message: format!("Service initialization error: {}", message.into()),
            component: service_name.unwrap_or_else(|| "unknown_service".to_string()),
            location: None,
            bug_report: false,
            context: None,
        }
    }

    /// Create runtime error using unified system
    pub fn runtime_error(message: impl Into<String>, operation: Option<String>) -> NestGateError {
        NestGateError::Internal {
            message: format!("Runtime error: {}", message.into()),
            component: operation.unwrap_or_else(|| "runtime".to_string()),
            location: None,
            bug_report: false,
            context: None,
        }
    }
}

// ==================== SECTION ====================

/// **DEPRECATED**: Legacy conversion from NestGateBinError to unified system
/// This is maintained for backward compatibility during migration.
impl From<NestGateBinError> for NestGateError {
    fn from(err: NestGateBinError) -> Self {
        match err {
            NestGateBinError::CommandExecutionFailed { message, command, exit_code } => {
                let mut context = std::collections::HashMap::new();
                if let Some(cmd) = command {
                    context.insert("command".to_string(), cmd);
                }
                if let Some(code) = exit_code {
                    context.insert("exit_code".to_string(), code.to_string());
                }
                
                NestGateError::execution_error(
                    format!("Command execution failed: {}", message),
                    Some("binary_execution".to_string()),
                    Some(context),
                )
            }
            NestGateBinError::ConfigurationError { message, config_path } => {
                let mut context = std::collections::HashMap::new();
                if let Some(path) = config_path {
                    context.insert("config_path".to_string(), path);
                }
                
                NestGateError::configuration_error(
                    message,
                    Some("binary_configuration".to_string()),
                )
            }
            NestGateBinError::ArgumentParsingError { message, argument } => {
                let mut context = std::collections::HashMap::new();
                if let Some(arg) = argument {
                    context.insert("argument".to_string(), arg);
                }
                
                NestGateError::validation_error(
                    "argument_parsing".to_string(),
                    message,
                    None,
                    Some("Check command line arguments format".to_string()),
                    true, // user_error
                )
            }
            NestGateBinError::ServiceInitializationError { message, service_name } => {
                let mut context = std::collections::HashMap::new();
                if let Some(name) = service_name {
                    context.insert("service_name".to_string(), name);
                }
                
                NestGateError::initialization_error(
                    format!("Service initialization failed: {}", message),
                    Some("service_startup".to_string()),
                    Some(context),
                )
            }
            NestGateBinError::RuntimeError { message, operation } => {
                let mut context = std::collections::HashMap::new();
                if let Some(op) = operation {
                    context.insert("operation".to_string(), op);
                }
                
                NestGateError::runtime_error(
                    message,
                    Some("binary_runtime".to_string()),
                    Some(context),
                )
            }
        }
    }
}

/// **CANONICAL CONVERSION TRAIT IMPLEMENTATION**
/// 
/// Implements the canonical IntoNestGateError trait for seamless integration
impl IntoNestGateError for NestGateBinError {
    fn into_nestgate_error(self) -> NestGateError {
        self.into()
    }

    fn into_nestgate_error_with_context(self, context: &str) -> NestGateError {
        let mut nestgate_error = self.into_nestgate_error();
        nestgate_error.add_context(context);
        nestgate_error
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
    fn with_context(self, context: &str) -> CanonicalResult<T, NestGateError>;
    
    /// Convert to canonical NestGateError
    fn into_canonical(self) -> CanonicalResult<T, NestGateError>;
}

impl<T> BinResultExt<T> for BinResult<T> {
    fn with_context(self, context: &str) -> CanonicalResult<T, NestGateError> {
        self.map_err(|e| e.into_nestgate_error_with_context(context))
    }
    
    fn into_canonical(self) -> CanonicalResult<T, NestGateError> {
        self.map_err(|e| e.into_nestgate_error())
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
