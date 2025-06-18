//! Error types for NestGate Core
//! 
//! Enhanced error handling with enhanced NestGate capabilities

use thiserror::Error;

/// Main error type for NestGate operations
#[derive(Error, Debug)]
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

    /// I/O error wrapper
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type alias for NestGate operations
pub type Result<T> = std::result::Result<T, NestGateError>; 