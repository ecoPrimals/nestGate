//! Error types for NestGate Core
//! 
//! Enhanced error handling with enhanced NestGate capabilities

use thiserror::Error;

/// Main error type for NestGate operations
#[derive(Error, Debug, Clone)]
pub enum NestGateError {
    /// Generic internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Invalid input provided
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Network-related error
    #[error("Network error: {0}")]
    Network(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Authorization error
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// Configuration error with enhanced handling
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Validation error with enhanced handling
    #[error("Validation error: {0}")]
    Validation(String),

    /// System error with enhanced handling
    #[error("System error: {0}")]
    SystemError(String),

    /// File system error with enhanced handling
    #[error("File system error: {0}")]
    FileSystem(String),

    /// Serialization error with enhanced handling
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Not found error (used in diagnostics)
    #[error("Not found: {0}")]
    NotFound(String),

    /// Timeout error
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// I/O error wrapper
    #[error("I/O error: {0}")]
    Io(String),

    /// Resource exhausted error
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    /// External service error
    #[error("External service error: {0}")]
    ExternalService(String),

    /// Cache error
    #[error("Cache error: {0}")]
    Cache(String),

    /// Storage error
    #[error("Storage error: {0}")]
    Storage(String),

    /// Compute error
    #[error("Compute error: {0}")]
    Compute(String),

    /// Federation error
    #[error("Federation error: {0}")]
    Federation(String),

    /// MCP error
    #[error("MCP error: {0}")]
    Mcp(String),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Result type alias for NestGate operations
pub type Result<T> = std::result::Result<T, NestGateError>;

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