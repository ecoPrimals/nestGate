// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::future::Future;

use super::canonical_service::CanonicalService;

/// **THE** canonical storage trait
///
/// Replaces ALL storage provider traits:
/// - UnifiedStorageBackend
/// - CanonicalStorageBackend
/// - ZeroCostUnifiedStorageBackend
/// - StorageBackend
/// - 6+ other storage trait variants
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::{CanonicalService, CanonicalStorage};
///
/// pub struct ZfsStorage {
///     pool: String,
///     config: ZfsConfig,
/// }
///
/// impl CanonicalService for ZfsStorage {
///     // ... implement CanonicalService
/// }
///
/// impl CanonicalStorage for ZfsStorage {
///     type Key = String;
///     type Value = Vec<u8>;
///     type Metadata = ZfsMetadata;
///
///     async fn read(&self, key: &Self::Key) -> Result<Option<Self::Value>, Self::Error> {
///         // ZFS read implementation
///         todo!()
///     }
///
///     async fn write(&self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error> {
///         // ZFS write implementation
///         todo!()
///     }
///
///     async fn delete(&self, key: &Self::Key) -> Result<(), Self::Error> {
///         // ZFS delete implementation
///         todo!()
///     }
///
///     async fn exists(&self, key: &Self::Key) -> Result<bool, Self::Error> {
///         // ZFS exists check
///         todo!()
///     }
///
///     async fn metadata(&self, key: &Self::Key) -> Result<Self::Metadata, Self::Error> {
///         // Get ZFS metadata
///         todo!()
///     }
///
///     async fn list(&self, prefix: Option<&str>) -> Result<Vec<Self::Key>, Self::Error> {
///         // List ZFS datasets
///         todo!()
///     }
/// }
/// ```
///
/// **DEPRECATED**: Use `crate::traits::canonical::CanonicalStorage` instead.
/// This is a duplicate definition maintained for backward compatibility only.
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalStorage instead - unified in canonical_unified_traits module"
)]
/// CanonicalStorage trait
pub trait CanonicalStorage: CanonicalService {
    /// Storage key type
    type Key: Clone + Send + Sync + 'static;

    /// Storage value type
    type Value: Clone + Send + Sync + 'static;

    /// Metadata type
    type Metadata: Clone + Send + Sync + 'static;

    // ==================== BASIC OPERATIONS ====================

    /// Read a value by key
    fn read(
        &self,
        key: &Self::Key,
    ) -> impl Future<Output = Result<Option<Self::Value>, Self::Error>> + Send;

    /// Write a value
    fn write(
        &self,
        key: Self::Key,
        value: Self::Value,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Delete a value
    fn delete(&self, key: &Self::Key) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Check if key exists
    fn exists(&self, key: &Self::Key) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    // ==================== BATCH OPERATIONS ====================

    /// Batch read with default implementation
    ///
    /// Override for optimized batch operations if your backend supports it.
    fn batch_read(
        &self,
        keys: &[Self::Key],
    ) -> impl Future<Output = Result<Vec<Option<Self::Value>>, Self::Error>> + Send {
        let keys = keys.to_vec(); // Clone to avoid borrowing issues
        async move {
            let mut results = Vec::with_capacity(keys.len());
            for key in &keys {
                results.push(self.read(key).await?);
            }
            Ok(results)
        }
    }

    /// Batch write with default implementation
    ///
    /// Override for optimized batch operations if your backend supports it.
    fn batch_write(
        &self,
        items: Vec<(Self::Key, Self::Value)>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            for (key, value) in items {
                self.write(key, value).await?;
            }
            Ok(())
        }
    }

    // ==================== METADATA & LISTING ====================

    /// Get metadata for a key
    fn metadata(
        &self,
        key: &Self::Key,
    ) -> impl Future<Output = Result<Self::Metadata, Self::Error>> + Send;

    /// List keys with optional prefix
    fn list(
        &self,
        prefix: Option<&str>,
    ) -> impl Future<Output = Result<Vec<Self::Key>, Self::Error>> + Send;

    // ==================== ADVANCED OPERATIONS ====================

    /// Copy a value from one key to another
    ///
    /// Default implementation uses read + write.
    /// Override for optimized copy if your backend supports it.
    fn copy(
        &self,
        from: &Self::Key,
        to: Self::Key,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            if let Some(value) = self.read(from).await? {
                self.write(to, value).await?;
            }
            Ok(())
        }
    }

    /// Move a value from one key to another
    ///
    /// Default implementation uses copy + delete.
    /// Override for optimized move if your backend supports it.
    fn move_key(
        &self,
        from: &Self::Key,
        to: Self::Key,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            self.copy(from, to).await?;
            self.delete(from).await?;
            Ok(())
        }
    }
}
