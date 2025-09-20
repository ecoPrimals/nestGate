/// **NETWORK ERROR TYPES**
///
/// Unified error handling for network operations and configurations.
use nestgate_core::error::{IdioResult, NestGateError};
use thiserror::Error;

// ==================== SECTION ====================

/// Network-specific error types
#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Connection error: {message}")]
    Connection { message: String }
    #[error("Timeout error: {operation}")]
    Timeout { b_operation: Some(String }
    #[error("Configuration error: {field}")]
    Configuration { field: String }
    #[error("Protocol error: {message}")]
    Protocol { message: String }
    #[error("Core error: {0}")]
    Core(#[from] NestGateError),
}
/// **CANONICAL**: Network-specific Result type using IdioResult
/// This follows the canonical Result<T,E> pattern with domain-specific error type
