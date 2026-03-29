// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **Canonical Storage Trait**
//!
//! Comprehensive storage trait for all storage operations.
//!
//! **Extracted**: November 19, 2025 - From canonical_unified_traits.rs
//! **Lines**: ~220 (from original 1,100-line file)

use super::service::CanonicalService;
use super::types::{
    DataStream, SnapshotInfo, StorageBackendType, StorageCapability, StorageUsageStats, WriteStream,
};
use std::future::Future;

/// Result type for [`CanonicalStorage::batch_read`] to keep RPITIT signatures readable.
pub type CanonicalBatchReadResult<K, I, E> = std::result::Result<Vec<(K, Option<I>)>, E>;

// ==================== CANONICAL STORAGE TRAIT ====================

/// **THE** canonical storage trait that replaces ALL storage traits
///
/// This trait consolidates and replaces:
/// - `StorageProvider` (from `canonical_provider_unification.rs`) ✨ **Deprecated Nov 9, 2025**
/// - `UnifiedStorageBackend` (from `unified_storage_traits.rs`)
/// - `CanonicalStorageBackend` (from `canonical_storage.rs`)  
/// - `StorageBackend` (from backends/mod.rs)
/// - `ZeroCostStorageProvider` (from `migration::storage_adapters`)
/// - `NativeAsyncStorageProvider` (from `migration::storage_adapters`)
/// - `ZeroCostUnifiedStorageBackend` (from `zero_cost_unified_storage_traits.rs`)
/// - `EnterpriseStorageCapabilities` (from enterprise/traits.rs)
///
/// **ENHANCED**: November 9, 2025 - Comprehensive storage interface (17+ methods)
/// **PERFORMANCE**: Native async throughout - zero `async_trait` overhead
/// **COMPLETENESS**: Covers all storage operations (CRUD, metadata, batch, lifecycle)
/// **UNIFICATION**: Single source of truth for all storage implementations
///
/// # Consolidated Methods (17+ total)
/// - **Core operations (5)**: read, write, delete, list, exists
/// - **Metadata (2)**: get_metadata, set_metadata
/// - **Batch operations (3)**: batch_read, batch_write, batch_delete
/// - **Advanced (3)**: copy, move_item, usage_stats
/// - **Backend management (4)**: backend_type, capabilities, initialize, shutdown
/// - **Optional features (3)**: create_snapshot, stream_read, stream_write
///
/// # Type Safety
/// Generic over `Item`, `Key`, `Metadata`, and `BackendConfig` types:
/// - Flexible type system supports any storage backend (filesystem, object store, database, etc.)
/// - Type-safe operations prevent common storage bugs
/// - Zero-cost abstractions with compile-time optimization
///
/// # Performance Characteristics
/// - **Native async (RPITIT)**: Zero `async_trait` overhead
/// - **Batch operations**: Efficient multi-item operations
/// - **Stream support**: Large data handling without memory bloat
/// - **Backend abstraction**: Zero-cost dispatch for different backends
///
/// # Example Implementation
/// ```ignore
/// impl CanonicalStorage for MyStorageBackend {
///     type Item = Vec<u8>;
///     type Key = String;
///     type Metadata = HashMap<String, String>;
///     type BackendConfig = MyConfig;
///     
///     async fn read(&self, key: Self::Key) -> Result<Option<Self::Item>, Self::Error> {
///         // Read from storage backend
///         Ok(Some(vec![1, 2, 3]))
///     }
///     
///     async fn write(&self, key: Self::Key, item: Self::Item) -> Result<(), Self::Error> {
///         // Write to storage backend
///         Ok(())
///     }
///     
///     // ... implement remaining methods
/// }
/// ```
pub trait CanonicalStorage: CanonicalService {
    /// Storage item type - can be bytes, structured data, or custom types
    type Item: Clone + Send + Sync + 'static;

    /// Storage key type - flexible key system (string, path, custom)
    type Key: Clone + Send + Sync + 'static;

    /// Storage metadata type - extensible metadata system
    type Metadata: Clone + Send + Sync + 'static;

    /// Backend-specific configuration type
    type BackendConfig: Clone + Send + Sync + 'static;

    // ==================== CORE STORAGE OPERATIONS ====================

    /// Read data from storage - native async
    fn read(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<Self::Item>, Self::Error>> + Send;

    /// Write data to storage - native async
    fn write(
        &self,
        key: Self::Key,
        item: Self::Item,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Delete data from storage - native async
    fn delete(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    /// List storage keys with optional prefix - native async
    fn list(
        &self,
        prefix: Option<Self::Key>,
    ) -> impl Future<Output = std::result::Result<Vec<Self::Key>, Self::Error>> + Send;

    /// Check if key exists - native async
    fn exists(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    // ==================== METADATA OPERATIONS ====================

    /// Get metadata for a storage item - native async
    fn get_metadata(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<Self::Metadata>, Self::Error>> + Send;

    /// Set metadata for a storage item - native async
    fn set_metadata(
        &self,
        key: Self::Key,
        metadata: Self::Metadata,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    // ==================== BATCH OPERATIONS ====================

    /// Batch read multiple items - native async
    fn batch_read(
        &self,
        keys: Vec<Self::Key>,
    ) -> impl Future<Output = CanonicalBatchReadResult<Self::Key, Self::Item, Self::Error>> + Send;

    /// Batch write multiple items - native async
    fn batch_write(
        &self,
        items: Vec<(Self::Key, Self::Item)>,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Batch delete multiple items - native async
    fn batch_delete(
        &self,
        keys: Vec<Self::Key>,
    ) -> impl Future<Output = std::result::Result<Vec<(Self::Key, bool)>, Self::Error>> + Send;

    // ==================== ADVANCED OPERATIONS ====================

    /// Copy item to new location - native async
    fn copy(
        &self,
        source: Self::Key,
        destination: Self::Key,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Move item to new location - native async  
    fn move_item(
        &self,
        source: Self::Key,
        destination: Self::Key,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get storage usage statistics - native async
    fn usage_stats(
        &self,
    ) -> impl Future<Output = std::result::Result<StorageUsageStats, Self::Error>> + Send;

    // ==================== BACKEND MANAGEMENT ====================

    /// Get backend type identifier
    fn backend_type(&self) -> StorageBackendType;

    /// Get backend capabilities
    fn capabilities(&self) -> Vec<StorageCapability>;

    /// Initialize backend with configuration - native async
    fn initialize_backend(
        &mut self,
        config: Self::BackendConfig,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Shutdown backend gracefully - native async
    fn shutdown_backend(
        &mut self,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    // ==================== OPTIONAL ADVANCED FEATURES ====================

    /// Create a snapshot (if supported) - native async
    fn create_snapshot(
        &self,
        name: &str,
    ) -> impl Future<Output = std::result::Result<Option<SnapshotInfo>, Self::Error>> + Send {
        async move {
            let _ = name;
            Ok(None) // Default: not supported
        }
    }

    /// Stream read large data (if supported) - native async  
    fn stream_read(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<DataStream>, Self::Error>> + Send {
        async move {
            let _ = key;
            Ok(None) // Default: not supported
        }
    }

    /// Stream write large data (if supported) - native async
    fn stream_write(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<WriteStream>, Self::Error>> + Send {
        async move {
            let _ = key;
            Ok(None) // Default: not supported
        }
    }
}
