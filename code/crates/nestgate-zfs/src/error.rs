//
// This module provides canonical error handling for ZFS operations, integrating
// with the unified NestGateError system for consistent error reporting.

use nestgate_core::error::{InternalErrorDetails, NestGateError};

// ==================== SECTION ====================

/// Canonical error creation helpers for ZFS operations
pub struct ZfsErrorBuilder;
impl ZfsErrorBuilder {
    /// Create a generic ZFS error (for backward compatibility)
    pub fn new(message: &str) -> NestGateError {
        NestGateError::internal_error(message, "zfs-generic")
    }

    /// Create a ZFS error with operation context (for backward compatibility)
    pub fn new_with_operation(
        message: &str,
        _operation: impl std::fmt::Debug,
    ) -> NestGateError {
        NestGateError::internal_error(message, "zfs-operation")
    }

    /// Create a canonical ZFS pool error
    pub fn pool_error(message: &str, pool: &str) -> NestGateError {
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message: format!("Pool error: {message}"),
            component: "zfs-pool".to_string(),
            location: Some(format!("pool:{pool}")),
            context: None,
            is_bug: false,
        }))
    }

    /// Create a canonical ZFS dataset error
    pub fn dataset_error(message: &str, dataset: &str) -> NestGateError {
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message: format!("Dataset error: {message}"),
            component: "zfs-dataset".to_string(),
            location: Some(format!("dataset:{dataset}")),
            context: None,
            is_bug: false,
        }))
    }

    /// Create a canonical ZFS snapshot error
    pub fn snapshot_error(message: &str, snapshot: &str) -> NestGateError {
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message: format!("Snapshot error: {message}"),
            component: "zfs-snapshot".to_string(),
            location: Some(format!("snapshot:{snapshot}")),
            context: None,
            is_bug: false,
        }))
    }

    /// Create a canonical ZFS command error
    pub fn command_error(command: &str, details: &str) -> NestGateError {
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message: format!("Command '{}' failed: {}", command, details),
            component: "zfs-command".to_string(),
            location: Some("zfs_command_execution".to_string()),
            context: None,
            is_bug: false,
        }))
    }

    /// Create a simple ZFS error
    pub fn zfs_error(message: &str) -> NestGateError {
        NestGateError::internal_error(message, "zfs-core")
    }

    /// Create a ZFS error with operation context
    pub fn zfs_operation_error(message: &str) -> NestGateError {
        NestGateError::internal_error(message, "zfs-operation")
    }

    /// Create a generic internal error with component and location (migration helper)
    pub fn internal(
        message: String,
        component: String,
        location: Option<String>,
    ) -> NestGateError {
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message,
            component,
            location,
            context: None,
            is_bug: false,
        }))
    }
}

// ==================== MIGRATION HELPERS ====================
// These functions help migrate old error creation patterns

/// Create ZFS internal error from old struct pattern (migration helper)
pub fn zfs_internal(
    message: String,
    component: String,
    location: Option<String>,
    _is_bug: bool,
    _context: Option<()>,
) -> NestGateError {
    ZfsErrorBuilder::internal(message, component, location)
}

// ==================== SECTION ====================

/// Convert ZFS command output to appropriate error
pub fn zfs_command_error(command: &str, output: &str) -> NestGateError {
    ZfsErrorBuilder::command_error(command, output)
}

/// Convert ZFS operation context to error  
pub fn zfs_operation_error(operation: &str, details: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("ZFS operation failed: {} - {}", operation, details),
        component: "zfs-operation".to_string(),
        location: Some(format!("zfs_{operation}_operation")),
        context: None,
        is_bug: false,
    }))
}

/// Create ZFS error with operation context - helper function
pub fn create_zfs_error(message: String, operation: ZfsOperation) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message,
        component: "zfs-core".to_string(),
        location: Some(format!("{operation:?}").to_lowercase()),
        context: None,
        is_bug: false,
    }))
}

/// ZFS operation types for error context
#[derive(Debug, Clone)]
pub enum ZfsOperation {
    PoolCreate,
    PoolDestroy,
    PoolImport,
    PoolExport,
    DatasetCreate,
    DatasetDestroy,
    DatasetMount,
    DatasetUnmount,
    SnapshotCreate,
    SnapshotDestroy,
    Command,
    SystemCheck,
    Configuration,
}

// ==================== SECTION: CANONICAL ERROR SYSTEM ====================

// CANONICAL MODERNIZATION: Remove duplicate type aliases
// All ZFS errors now use the unified nestgate_core error system

// REMOVED DUPLICATES:
// - pub type ZfsResult<T> = std::result::Result<T, crate::types::ZfsError>;
// - pub type Result<T> = nestgate_core::error::Result<T, ZfsError>;

// USE CANONICAL TYPES:
pub use nestgate_core::error::{Result, ZfsError, ZfsResult};

// ==================== SECTION ====================
