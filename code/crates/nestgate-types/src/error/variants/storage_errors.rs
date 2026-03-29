// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage error variants and utilities
//! Storage Errors functionality and utilities.
//! This module provides storage-specific error types and helper functions.

use super::core_errors::{NestGateUnifiedError, StorageErrorDetails};

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

// ==================== CONVENIENCE CONSTRUCTORS ON MAIN ERROR TYPE ====================

impl NestGateUnifiedError {
    /// Create a storage error (convenience constructor)
    ///
    /// # Example
    /// ```
    /// use nestgate_types::error::NestGateError;
    /// let error = NestGateError::storage("Failed to read file");
    /// ```
    pub fn storage(message: impl Into<String>) -> Self {
        Self::Storage(Box::new(StorageErrorDetails::new(message)))
    }

    /// Create a storage error with operation context
    pub fn storage_with_operation(
        message: impl Into<String>,
        operation: impl Into<String>,
    ) -> Self {
        Self::Storage(Box::new(StorageErrorDetails::with_operation(
            message, operation,
        )))
    }
}
