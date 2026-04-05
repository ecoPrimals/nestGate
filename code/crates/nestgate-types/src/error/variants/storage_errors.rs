// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage error variants and utilities
//! Storage Errors functionality and utilities.
//! This module provides storage-specific error types and helper functions.

use std::borrow::Cow;

use super::core_errors::{NestGateUnifiedError, StorageErrorDetails};

impl StorageErrorDetails {
    /// Create a storage error with just a message
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
            operation: None,
            resource: None,
            storage_data: None,
            context: None,
        }
    }

    /// Create a storage error with operation context
    pub fn with_operation(
        message: impl Into<Cow<'static, str>>,
        operation: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            message: message.into(),
            operation: Some(operation.into()),
            resource: None,
            storage_data: None,
            context: None,
        }
    }

    /// Create a ZFS-specific error
    pub fn zfs_error(
        message: impl Into<Cow<'static, str>>,
        resource: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            message: message.into(),
            operation: Some(Cow::Borrowed("zfs_operation")),
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
    pub fn storage(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Storage(Box::new(StorageErrorDetails::new(message)))
    }

    /// Create a storage error with operation context
    pub fn storage_with_operation(
        message: impl Into<Cow<'static, str>>,
        operation: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::Storage(Box::new(StorageErrorDetails::with_operation(
            message, operation,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::StorageErrorDetails;
    use crate::error::variants::core_errors::NestGateUnifiedError;

    #[test]
    fn storage_error_details_constructors() {
        let d = StorageErrorDetails::new("m");
        assert_eq!(d.message.as_ref(), "m");
        let d = StorageErrorDetails::with_operation("m", "op");
        assert_eq!(d.operation.as_deref(), Some("op"));
        let d = StorageErrorDetails::zfs_error("z", "pool");
        assert!(d.resource.as_ref().is_some());
    }

    #[test]
    fn nestgate_unified_storage_constructors() {
        let e = NestGateUnifiedError::storage("s");
        assert!(matches!(e, NestGateUnifiedError::Storage(_)));
        let e = NestGateUnifiedError::storage_with_operation("s", "read");
        assert!(matches!(e, NestGateUnifiedError::Storage(_)));
    }
}
