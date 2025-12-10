//! ZFS error types and result aliases
//!
//! Domain: Error handling, error conversion, result types

// Re-export canonical error type
pub use nestgate_core::error::Result;

/// ZFS-specific error type for backward compatibility
///
/// **MIGRATION NOTE**: Prefer using `nestgate_core::error::NestGateError` for new code.
/// This type exists for backward compatibility with existing ZFS-specific code.
#[derive(Debug, thiserror::Error)]
pub enum ZfsError {
    /// Pool operation failed
    #[error("Pool operation failed: {message}")]
    PoolError {
        /// Error message describing the failure
        message: String,
    },

    /// Dataset operation failed
    #[error("Dataset operation failed: {message}")]
    DatasetError {
        /// Error message describing the failure
        message: String,
    },

    /// Snapshot operation failed
    #[error("Snapshot operation failed: {message}")]
    SnapshotError {
        /// Error message describing the failure
        message: String,
    },

    /// Command execution failed
    #[error("Command execution failed: {message}")]
    CommandError {
        /// Error message describing the failure
        message: String,
    },

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigError {
        /// Error message describing the failure
        message: String,
    },

    /// I/O operation error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl ZfsError {
    /// Create a pool error with a message
    pub fn pool_error(message: impl Into<String>) -> Self {
        Self::PoolError {
            message: message.into(),
        }
    }

    /// Create a dataset error with a message
    pub fn dataset_error(message: impl Into<String>) -> Self {
        Self::DatasetError {
            message: message.into(),
        }
    }

    /// Create a snapshot error with a message
    pub fn snapshot_error(message: impl Into<String>) -> Self {
        Self::SnapshotError {
            message: message.into(),
        }
    }

    /// Create a command error with a message
    pub fn command_error(message: impl Into<String>) -> Self {
        Self::CommandError {
            message: message.into(),
        }
    }

    /// Create a configuration error with a message
    pub fn config_error(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }
}

/// ZFS-specific Result type for backward compatibility
///
/// **MIGRATION NOTE**: Prefer using `nestgate_core::error::Result<T>` for new code.
pub type ZfsResult<T> = std::result::Result<T, ZfsError>;

#[cfg(test)]
#[path = "errors_strategic_tests.rs"]
mod errors_strategic_tests;
