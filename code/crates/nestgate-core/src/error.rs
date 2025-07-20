//! Error types for NestGate Core
//!
//! Enhanced error handling with enhanced NestGate capabilities

use std::fmt;

#[derive(Debug, Clone)]
pub enum NestGateError {
    /// Internal system error
    Internal(String),

    /// Validation error
    Validation(String),

    /// Configuration error
    Configuration(String),

    /// Network error
    Network(String),

    /// Storage error
    Storage(String),

    /// Unauthorized access error
    Unauthorized(String),

    /// I/O error
    Io(String),

    /// Serialization error
    Serialization(String),

    /// External service error
    External(String),

    /// Parse error
    Parse(String),

    /// System error
    SystemError(String),

    /// File system error
    FileSystem(String),

    /// Invalid input error
    InvalidInput(String),

    /// Authentication failed error
    AuthenticationFailed,

    /// Security module unavailable error
    SecurityModuleUnavailable,

    /// Not found error
    NotFound(String),

    /// Database error
    Database(String),

    /// Authentication error
    Authentication(String),

    /// Authorization error
    Authorization(String),

    /// Timeout error
    Timeout(String),
}

impl fmt::Display for NestGateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NestGateError::Internal(msg) => write!(f, "Internal error: {msg}"),
            NestGateError::Validation(msg) => write!(f, "Validation error: {msg}"),
            NestGateError::Configuration(msg) => write!(f, "Configuration error: {msg}"),
            NestGateError::Network(msg) => write!(f, "Network error: {msg}"),
            NestGateError::Storage(msg) => write!(f, "Storage error: {msg}"),
            NestGateError::Unauthorized(msg) => write!(f, "Unauthorized: {msg}"),
            NestGateError::Io(msg) => write!(f, "I/O error: {msg}"),
            NestGateError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
            NestGateError::External(msg) => write!(f, "External service error: {msg}"),
            NestGateError::Parse(msg) => write!(f, "Parse error: {msg}"),
            NestGateError::SystemError(msg) => write!(f, "System error: {msg}"),
            NestGateError::FileSystem(msg) => write!(f, "File system error: {msg}"),
            NestGateError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            NestGateError::AuthenticationFailed => write!(f, "Authentication failed"),
            NestGateError::SecurityModuleUnavailable => write!(f, "Security module unavailable"),
            NestGateError::NotFound(msg) => write!(f, "Not found: {msg}"),
            NestGateError::Database(msg) => write!(f, "Database error: {msg}"),
            NestGateError::Authentication(msg) => write!(f, "Authentication error: {msg}"),
            NestGateError::Authorization(msg) => write!(f, "Authorization error: {msg}"),
            NestGateError::Timeout(msg) => write!(f, "Timeout error: {msg}"),
        }
    }
}

impl std::error::Error for NestGateError {}

/// Result type alias for NestGate operations
pub type Result<T> = std::result::Result<T, NestGateError>;

/// Universal result type for operations that may use different error types
/// This provides a common interface across the entire codebase
pub type UniversalResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

impl From<std::num::ParseIntError> for NestGateError {
    fn from(err: std::num::ParseIntError) -> Self {
        NestGateError::Parse(err.to_string())
    }
}

impl From<std::io::Error> for NestGateError {
    fn from(err: std::io::Error) -> Self {
        NestGateError::Io(err.to_string())
    }
}

impl From<reqwest::Error> for NestGateError {
    fn from(err: reqwest::Error) -> Self {
        NestGateError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for NestGateError {
    fn from(err: serde_json::Error) -> Self {
        NestGateError::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for NestGateError {
    fn from(err: toml::de::Error) -> Self {
        NestGateError::Serialization(err.to_string())
    }
}

impl From<anyhow::Error> for NestGateError {
    fn from(err: anyhow::Error) -> Self {
        NestGateError::Internal(err.to_string())
    }
}
