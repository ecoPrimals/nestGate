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

    /// Capacity too small for operation
    #[error("Capacity too small: {message}")]
    CapacityTooSmall {
        /// Error message describing the capacity constraint
        message: String,
    },

    /// Capacity exceeded
    #[error("Capacity exceeded: {message}")]
    CapacityExceeded {
        /// Error message describing the capacity violation
        message: String,
    },

    /// Invalid path provided
    #[error("Invalid path: {message}")]
    InvalidPath {
        /// Error message describing the invalid path
        message: String,
    },

    /// Invalid property specified
    #[error("Invalid property: {message}")]
    InvalidProperty {
        /// Error message describing the invalid property
        message: String,
    },

    /// Cross-pool rename attempted (not supported)
    #[error("Cross-pool rename not supported: {message}")]
    CrossPoolRename {
        /// Error message describing the attempted cross-pool operation
        message: String,
    },
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

    /// Create a capacity too small error
    pub fn capacity_too_small(message: impl Into<String>) -> Self {
        Self::CapacityTooSmall {
            message: message.into(),
        }
    }

    /// Create a capacity exceeded error
    pub fn capacity_exceeded(message: impl Into<String>) -> Self {
        Self::CapacityExceeded {
            message: message.into(),
        }
    }

    /// Create an invalid path error
    pub fn invalid_path(message: impl Into<String>) -> Self {
        Self::InvalidPath {
            message: message.into(),
        }
    }

    /// Create an invalid property error
    pub fn invalid_property(message: impl Into<String>) -> Self {
        Self::InvalidProperty {
            message: message.into(),
        }
    }

    /// Create a cross-pool rename error
    pub fn cross_pool_rename(message: impl Into<String>) -> Self {
        Self::CrossPoolRename {
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
