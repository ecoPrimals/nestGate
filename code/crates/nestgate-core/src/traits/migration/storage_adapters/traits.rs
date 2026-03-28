// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::future::Future;

use crate::Result;

/// Trait bound helper for NativeAsyncStorageProvider
/// This allows the adapter to work with any implementation of the old trait
/// **DEPRECATED**: Migration complete - use canonical storage
#[deprecated(
    since = "0.9.0",
    note = "Migration to native async complete - use crate::traits::canonical::CanonicalStorage"
)]
/// NativeAsyncStorageProvider trait
pub trait NativeAsyncStorageProvider {
    /// Type alias for ObjectId
    type ObjectId: Clone + Send + Sync + 'static;
    /// Type alias for ObjectData
    type ObjectData: Clone + Send + Sync + 'static;
    /// Type alias for ObjectMetadata
    type ObjectMetadata: Clone + Send + Sync + 'static;

    /// Store Object
    fn store_object(
        &self,
        data: Self::ObjectData,
        metadata: Self::ObjectMetadata,
    ) -> impl Future<Output = Result<Self::ObjectId>> + Send;

    /// Retrieve Object
    fn retrieve_object(
        &self,
        id: &Self::ObjectId,
    ) -> impl Future<Output = Result<Self::ObjectData>> + Send;

    /// Deletes  Object
    fn delete_object(&self, id: &Self::ObjectId) -> impl Future<Output = Result<()>> + Send;

    /// List Objects
    fn list_objects(&self) -> impl Future<Output = Result<Vec<Self::ObjectId>>> + Send;

    /// Gets Metadata
    fn get_metadata(
        &self,
        id: &Self::ObjectId,
    ) -> impl Future<Output = Result<Self::ObjectMetadata>> + Send;
}

/// Trait bound helper for StoragePrimalProvider
/// Storage trait re-exported from canonical source
///
/// **CONSOLIDATED**: This trait definition was replaced with a re-export to eliminate duplication.
/// See: `crate::traits::canonical_hierarchy::CanonicalStorage` for the unified implementation.
///
/// **Migration**: Update implementations to use `CanonicalStorage` directly.
/// ```rust,ignore
/// use nestgate_core::traits::{CanonicalStorage};
///
/// impl CanonicalStorage for MyStorage {
///     // ... implementation
/// }
/// ```
pub use crate::traits::canonical_hierarchy::CanonicalStorage as StoragePrimalProvider;

/// Trait bound helper for simple ZeroCostStorageProvider
/// This matches the actual trait in zero_cost/traits.rs
/// **DEPRECATED**: Zero-cost patterns consolidated into canonical storage
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::unified_storage::UnifiedStorage - includes zero-cost optimizations"
)]
/// ZeroCostStorageProvider trait
pub trait ZeroCostStorageProvider<K, V> {
    /// Store
    fn store(&self, key: K, value: V) -> impl Future<Output = Result<()>> + Send;
    /// Retrieve
    fn retrieve(&self, key: &K) -> Option<V>;
    /// Deletes resource
    fn delete(&self, key: &K) -> bool;
}
