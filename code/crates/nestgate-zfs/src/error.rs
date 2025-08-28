//
// This module provides canonical error handling for ZFS operations, integrating
// with the unified NestGateError system for consistent error reporting.

use nestgate_core::error::NestGateError;

// ==================== SECTION ====================

/// Canonical error creation helpers for ZFS operations
pub struct ZfsErrorBuilder;

impl ZfsErrorBuilder {
    /// Create a generic ZFS error (for backward compatibility)
    pub fn new(message: &str) -> NestGateError {
        NestGateError::Internal {
            message: message.to_string(),
            location: Some("zfs_operation".to_string()),
            context: None,
            is_bug: false,
        }
    }

    /// Create a ZFS error with operation context (for backward compatibility)
    pub fn new_with_operation(message: &str, _operation: impl std::fmt::Debug) -> NestGateError {
        NestGateError::Internal {
            message: message.to_string(),
            location: Some("zfs_operation".to_string()),
            debug_info: Some(format!("Operation: {:?}", _operation)),
            is_bug: false,
        }
    }

    /// Create a canonical ZFS pool error
    pub fn pool_error(message: &str, pool: &str) -> NestGateError {
        NestGateError::Internal {
            message: format!("Pool '{}': {}", pool, message),
            location: Some("zfs_pool_operation".to_string()),
            context: None,
            is_bug: false,
        }
    }

    /// Create a canonical ZFS dataset error
    pub fn dataset_error(message: &str, dataset: &str) -> NestGateError {
        NestGateError::Internal {
            message: format!("Dataset '{}': {}", dataset, message),
            location: Some("zfs_dataset_operation".to_string()),
            context: None,
            is_bug: false,
        }
    }

    /// Create a canonical ZFS snapshot error
    pub fn snapshot_error(message: &str, snapshot: &str) -> NestGateError {
        NestGateError::Internal {
            message: format!("Snapshot '{}': {}", snapshot, message),
            location: Some("zfs_snapshot_operation".to_string()),
            context: None,
            is_bug: false,
        }
    }

    /// Create a canonical ZFS command error
    pub fn command_error(command: &str, details: &str) -> NestGateError {
        NestGateError::Internal {
            message: format!("ZFS command '{}' failed: {}", command, details),
            location: Some("zfs_command_execution".to_string()),
            context: None,
            is_bug: false,
        }
    }
}

// ==================== SECTION ====================

/// Convert ZFS command output to appropriate error
pub fn zfs_command_error(command: &str, output: &str) -> NestGateError {
    ZfsErrorBuilder::command_error(command, output)
}

/// Convert ZFS operation context to error  
pub fn zfs_operation_error(operation: &str, details: &str) -> NestGateError {
    NestGateError::Internal {
        message: format!("ZFS {} operation failed: {}", operation, details),
        location: Some(format!("zfs_{}_operation", operation.to_lowercase())),
        context: None,
        is_bug: false,
    }
}

// ==================== SECTION ====================

/// **CANONICAL**: ZFS-specific Result type using IdioResult with ZfsError from types.rs
pub type ZfsResult<T> = std::result::Result<T, crate::types::ZfsError>;

/// **CANONICAL RESULT** - Use canonical Result from nestgate-core
pub type Result<T> = nestgate_core::error::Result<T, ZfsError>;
