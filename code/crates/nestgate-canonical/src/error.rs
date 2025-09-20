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
            path: path.map(|p| p.into()),
        }
    }
}
