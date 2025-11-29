//
// This module provides canonical error handling for ZFS operations, integrating
// with the unified NestGateError system for consistent error reporting.

use nestgate_core::error::{InternalErrorDetails, NestGateError};

// ==================== SECTION ====================

/// Canonical error creation helpers for ZFS operations
pub struct ZfsErrorBuilder;
impl ZfsErrorBuilder {
    /// Create a generic ZFS error (for backward compatibility)
    #[allow(clippy::new_ret_no_self)]
    #[must_use]
    pub fn new(message: &str) -> NestGateError {
        NestGateError::internal_error(message, "zfs-generic")
    }

    /// Create a ZFS error with operation context (for backward compatibility)
    pub fn new_with_operation(message: &str, _operation: impl std::fmt::Debug) -> NestGateError {
        NestGateError::internal_error(message, "zfs-operation")
    }

    /// Create a canonical ZFS pool error
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn command_error(command: &str, details: &str) -> NestGateError {
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message: format!("Command '{command}' failed: {details}"),
            component: "zfs-command".to_string(),
            location: Some("zfs_command_execution".to_string()),
            context: None,
            is_bug: false,
        }))
    }

    /// Create a simple ZFS error
    #[must_use]
    pub fn zfs_error(message: &str) -> NestGateError {
        NestGateError::internal_error(message, "zfs-core")
    }

    /// Create a ZFS error with operation context
    #[must_use]
    pub fn zfs_operation_error(message: &str) -> NestGateError {
        NestGateError::internal_error(message, "zfs-operation")
    }

    /// Create a generic internal error with component and location (migration helper)
    #[must_use]
    pub fn internal(message: String, component: String, location: Option<String>) -> NestGateError {
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
#[must_use]
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
#[must_use]
pub fn zfs_command_error(command: &str, output: &str) -> NestGateError {
    ZfsErrorBuilder::command_error(command, output)
}

/// Convert ZFS operation context to error  
#[must_use]
pub fn zfs_operation_error(operation: &str, details: &str) -> NestGateError {
    NestGateError::Internal(Box::new(InternalErrorDetails {
        message: format!("ZFS operation failed: {operation} - {details}"),
        component: "zfs-operation".to_string(),
        location: Some(format!("zfs_{operation}_operation")),
        context: None,
        is_bug: false,
    }))
}

/// Create ZFS error with operation context - helper function
#[must_use]
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
/// Zfsoperation
pub enum ZfsOperation {
    /// Poolcreate
    PoolCreate,
    /// Pooldestroy
    PoolDestroy,
    /// Poolimport
    PoolImport,
    /// Poolexport
    PoolExport,
    /// Datasetcreate
    DatasetCreate,
    /// Datasetdestroy
    DatasetDestroy,
    /// Datasetmount
    DatasetMount,
    /// Datasetunmount
    DatasetUnmount,
    /// Snapshotcreate
    SnapshotCreate,
    /// Snapshotdestroy
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

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== ZFSERRORBUILDER TESTS ====================

    #[test]
    fn test_zfs_error_builder_new() {
        let error = ZfsErrorBuilder::new("Test error");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Test error"));
        assert!(error_string.contains("zfs-generic"));
    }

    #[test]
    fn test_zfs_error_builder_new_with_operation() {
        let error = ZfsErrorBuilder::new_with_operation("Operation failed", "PoolCreate");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Operation failed"));
        assert!(error_string.contains("zfs-operation"));
    }

    #[test]
    fn test_zfs_error_builder_pool_error() {
        let error = ZfsErrorBuilder::pool_error("Pool not found", "tank");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Pool error"));
        assert!(error_string.contains("Pool not found"));
        assert!(error_string.contains("zfs-pool"));
        assert!(error_string.contains("pool:tank"));
    }

    #[test]
    fn test_zfs_error_builder_dataset_error() {
        let error = ZfsErrorBuilder::dataset_error("Dataset creation failed", "tank/data");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Dataset error"));
        assert!(error_string.contains("Dataset creation failed"));
        assert!(error_string.contains("zfs-dataset"));
        assert!(error_string.contains("dataset:tank/data"));
    }

    #[test]
    fn test_zfs_error_builder_snapshot_error() {
        let error = ZfsErrorBuilder::snapshot_error("Snapshot failed", "tank/data@snap1");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Snapshot error"));
        assert!(error_string.contains("Snapshot failed"));
        assert!(error_string.contains("zfs-snapshot"));
        assert!(error_string.contains("snapshot:tank/data@snap1"));
    }

    #[test]
    fn test_zfs_error_builder_command_error() {
        let error = ZfsErrorBuilder::command_error("zpool create", "insufficient permissions");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Command 'zpool create' failed"));
        assert!(error_string.contains("insufficient permissions"));
        assert!(error_string.contains("zfs-command"));
    }

    #[test]
    fn test_zfs_error_builder_zfs_error() {
        let error = ZfsErrorBuilder::zfs_error("Generic ZFS error");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Generic ZFS error"));
        assert!(error_string.contains("zfs-core"));
    }

    #[test]
    fn test_zfs_error_builder_zfs_operation_error() {
        let error = ZfsErrorBuilder::zfs_operation_error("Operation timeout");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Operation timeout"));
        assert!(error_string.contains("zfs-operation"));
    }

    #[test]
    fn test_zfs_error_builder_internal() {
        let error = ZfsErrorBuilder::internal(
            "Internal error occurred".to_string(),
            "zfs-manager".to_string(),
            Some("manager::create_pool".to_string()),
        );
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Internal error occurred"));
        assert!(error_string.contains("zfs-manager"));
        assert!(error_string.contains("manager::create_pool"));
    }

    #[test]
    fn test_zfs_error_builder_internal_no_location() {
        let error = ZfsErrorBuilder::internal(
            "Error without location".to_string(),
            "zfs-core".to_string(),
            None,
        );
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Error without location"));
        assert!(error_string.contains("zfs-core"));
    }

    // ==================== MIGRATION HELPER TESTS ====================

    #[test]
    fn test_zfs_internal() {
        let error = zfs_internal(
            "Internal ZFS error".to_string(),
            "zfs-system".to_string(),
            Some("system::check".to_string()),
            false,
            None,
        );
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Internal ZFS error"));
        assert!(error_string.contains("zfs-system"));
        assert!(error_string.contains("system::check"));
    }

    #[test]
    fn test_zfs_internal_with_bug_flag() {
        let error = zfs_internal(
            "Unexpected state".to_string(),
            "zfs-bug".to_string(),
            None,
            true, // is_bug flag (ignored but tested)
            None,
        );
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Unexpected state"));
    }

    #[test]
    fn test_zfs_command_error() {
        let error = zfs_command_error("zfs destroy", "dataset is busy");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Command 'zfs destroy' failed"));
        assert!(error_string.contains("dataset is busy"));
    }

    #[test]
    fn test_zfs_command_error_with_special_chars() {
        let error = zfs_command_error("zfs send", "cannot open 'tank@snap': invalid name");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Command 'zfs send' failed"));
        assert!(error_string.contains("cannot open"));
    }

    #[test]
    fn test_zfs_operation_error() {
        let error = zfs_operation_error("pool_import", "pool not found");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("ZFS operation failed: pool_import"));
        assert!(error_string.contains("pool not found"));
        assert!(error_string.contains("zfs-operation"));
    }

    #[test]
    fn test_zfs_operation_error_multiple_words() {
        let error = zfs_operation_error("dataset create", "parent dataset does not exist");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("dataset create"));
        assert!(error_string.contains("parent dataset does not exist"));
    }

    #[test]
    fn test_create_zfs_error_pool_create() {
        let error = create_zfs_error("Pool creation failed".to_string(), ZfsOperation::PoolCreate);
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Pool creation failed"));
        assert!(error_string.contains("zfs-core"));
        assert!(error_string.contains("poolcreate"));
    }

    #[test]
    fn test_create_zfs_error_pool_destroy() {
        let error = create_zfs_error(
            "Pool destruction failed".to_string(),
            ZfsOperation::PoolDestroy,
        );
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Pool destruction failed"));
        assert!(error_string.contains("pooldestroy"));
    }

    #[test]
    fn test_create_zfs_error_dataset_create() {
        let error = create_zfs_error(
            "Dataset creation failed".to_string(),
            ZfsOperation::DatasetCreate,
        );
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Dataset creation failed"));
        assert!(error_string.contains("datasetcreate"));
    }

    #[test]
    fn test_create_zfs_error_snapshot_create() {
        let error = create_zfs_error(
            "Snapshot creation failed".to_string(),
            ZfsOperation::SnapshotCreate,
        );
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Snapshot creation failed"));
        assert!(error_string.contains("snapshotcreate"));
    }

    #[test]
    fn test_create_zfs_error_command() {
        let error = create_zfs_error(
            "Command execution failed".to_string(),
            ZfsOperation::Command,
        );
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Command execution failed"));
        assert!(error_string.contains("command"));
    }

    // ==================== ZFSOPERATION TESTS ====================

    #[test]
    fn test_zfs_operation_debug() {
        let op = ZfsOperation::PoolCreate;
        let debug_str = format!("{op:?}");
        assert_eq!(debug_str, "PoolCreate");
    }

    #[test]
    fn test_zfs_operation_clone() {
        let op1 = ZfsOperation::DatasetCreate;
        let op2 = op1.clone();
        assert!(format!("{op1:?}") == format!("{op2:?}"));
    }

    #[test]
    fn test_all_zfs_operation_variants() {
        let operations = vec![
            ZfsOperation::PoolCreate,
            ZfsOperation::PoolDestroy,
            ZfsOperation::PoolImport,
            ZfsOperation::PoolExport,
            ZfsOperation::DatasetCreate,
            ZfsOperation::DatasetDestroy,
            ZfsOperation::DatasetMount,
            ZfsOperation::DatasetUnmount,
            ZfsOperation::SnapshotCreate,
            ZfsOperation::SnapshotDestroy,
            ZfsOperation::Command,
            ZfsOperation::SystemCheck,
            ZfsOperation::Configuration,
        ];

        // Verify all variants can be created and formatted
        for op in operations {
            let debug_str = format!("{op:?}");
            assert!(!debug_str.is_empty());

            // Verify error creation works with each operation
            let error = create_zfs_error(format!("{debug_str} failed"), op);
            assert!(format!("{error:?}").contains("failed"));
        }
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_empty_error_message() {
        let error = ZfsErrorBuilder::new("");
        let error_string = format!("{error:?}");
        // Should still create valid error
        assert!(error_string.contains("zfs-generic"));
    }

    #[test]
    fn test_very_long_error_message() {
        let long_message = "a".repeat(1000);
        let error = ZfsErrorBuilder::pool_error(&long_message, "tank");
        let error_string = format!("{error:?}");
        assert!(error_string.contains(&long_message));
    }

    #[test]
    fn test_error_with_special_characters() {
        let error =
            ZfsErrorBuilder::dataset_error("Error with 特殊字符 and émojis 🔥", "tank/data");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("特殊字符"));
    }

    #[test]
    fn test_error_with_newlines() {
        let error =
            ZfsErrorBuilder::command_error("zpool status", "Error output:\nLine 1\nLine 2\nLine 3");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Line 1"));
    }

    #[test]
    fn test_pool_error_empty_pool_name() {
        let error = ZfsErrorBuilder::pool_error("Pool error", "");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("Pool error"));
        assert!(error_string.contains("pool:"));
    }

    #[test]
    fn test_dataset_error_with_slashes() {
        let error = ZfsErrorBuilder::dataset_error("Cannot create", "tank/data/subdata/nested");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("tank/data/subdata/nested"));
    }

    #[test]
    fn test_snapshot_error_with_at_symbol() {
        let error = ZfsErrorBuilder::snapshot_error("Snapshot exists", "tank/data@snapshot1");
        let error_string = format!("{error:?}");
        assert!(error_string.contains("tank/data@snapshot1"));
    }

    // ==================== CONCURRENT ACCESS TESTS ====================

    #[tokio::test]
    async fn test_concurrent_error_creation() {
        let handles: Vec<_> = (0..100)
            .map(|i| {
                tokio::spawn(async move {
                    let error = ZfsErrorBuilder::pool_error(
                        &format!("Concurrent error {i}"),
                        &format!("pool{i}"),
                    );
                    assert!(format!("{error:?}").contains(&format!("Concurrent error {i}")));
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task should complete");
        }
    }
}

// USE CANONICAL TYPES:
pub use nestgate_core::error::Result;
// Re-export ZfsError from types module for backward compatibility
pub use crate::types::ZfsError;
// Define ZfsResult as an alias to Result with ZfsError
pub type ZfsResult<T> = Result<T>;

// ==================== SECTION ====================
