//! Storage error variants and utilities
//! Storage Errors functionality and utilities.
//! This module provides storage-specific error types and helper functions.

use super::core_errors::StorageErrorDetails;

impl StorageErrorDetails {
    /// Create a storage error with just a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            operation: None,
            resource: None,
            storage_data: None,
            context: None,
        }
    }

    /// Create a storage error with operation context
    pub fn with_operation(message: impl Into<String>, operation: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            operation: Some(operation.into()),
            resource: None,
            storage_data: None,
            context: None,
        }
    }

    /// Create a ZFS-specific error
    pub fn zfs_error(message: impl Into<String>, resource: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            operation: Some("zfs_operation".to_string()),
            resource: Some(resource.into()),
            storage_data: None,
            context: None,
        }
    }
}
