//! Error types for NestGate Core

use thiserror::Error;

/// Result type for NestGate operations
pub type Result<T> = std::result::Result<T, NestGateError>;

/// Main error type for NestGate
#[derive(Error, Debug)]
pub enum NestGateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Security error: {0}")]
    Security(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Operation failed: {0}")]
    OperationFailed(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("JSON error: {0}")]
    Json(String),
    
    #[error("YAML error: {0}")]
    Yaml(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Cache error: {0}")]
    Cache(String),
}

// Additional NestGate-specific error types can go here if needed 