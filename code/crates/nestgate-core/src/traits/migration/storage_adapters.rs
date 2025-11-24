//! **STORAGE TRAIT MIGRATION ADAPTERS**
//!
//! Adapters that wrap old storage provider traits and implement `CanonicalStorage`.
//! These enable gradual migration from old traits to the canonical hierarchy.

#![allow(deprecated)]

use std::future::Future;
use std::marker::PhantomData;

use crate::traits::canonical_hierarchy::{CanonicalService, CanonicalStorage};
use crate::{NestGateError, Result};

// ==================== NATIVE ASYNC STORAGE ADAPTER ====================

/// Adapter for `NativeAsyncStorageProvider` → `CanonicalStorage`
///
/// This is the **easiest** migration because the source trait already uses
/// native async (no `async_trait` overhead).
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::traits::migration::NativeAsyncStorageAdapter;
///
/// let old_storage = MyNativeAsyncStorage::new();
/// let canonical = NativeAsyncStorageAdapter::new(old_storage);
///
/// // Now use it as CanonicalStorage
/// canonical.write(key, value).await?;
/// ```
pub struct NativeAsyncStorageAdapter<T> {
    inner: T,
    name: String,
    version: String,
    config: serde_json::Value,
}

impl<T> NativeAsyncStorageAdapter<T> {
    /// Create a new adapter wrapping an old trait implementation
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            name: "native-async-storage-adapter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config: serde_json::json!({}),
        }
    }

    /// Create with custom name and version
    pub fn with_metadata(inner: T, name: String, version: String) -> Self {
        Self {
            inner,
            name,
            version,
            config: serde_json::json!({}),
        }
    }
}

// Implement CanonicalService (required base trait)
impl<T> CanonicalService for NativeAsyncStorageAdapter<T>
where
    T: Send + Sync + 'static,
{
    type Config = serde_json::Value;
    type Health = serde_json::Value;
    type Metrics = serde_json::Value;
    type Error = NestGateError;

    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    async fn health(&self) -> Result<Self::Health> {
        Ok(serde_json::json!({
            "status": "healthy",
            "adapter": "native-async-storage"
        }))
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn metrics(&self) -> Result<Self::Metrics> {
        Ok(serde_json::json!({
            "adapter_type": "native-async-storage"
        }))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }
}

// Implement CanonicalStorage by delegating to old trait methods
impl<T, ObjectId, ObjectData, ObjectMetadata> CanonicalStorage for NativeAsyncStorageAdapter<T>
where
    T: NativeAsyncStorageProvider<
            ObjectId = ObjectId,
            ObjectData = ObjectData,
            ObjectMetadata = ObjectMetadata,
        > + Send
        + Sync
        + 'static,
    ObjectId: Clone + Send + Sync + 'static,
    ObjectData: Clone + Send + Sync + 'static,
    ObjectMetadata: Clone + Send + Sync + Default + 'static,
{
    type Key = ObjectId;
    type Value = ObjectData;
    type Metadata = ObjectMetadata;

    async fn read(&self, key: &Self::Key) -> Result<Option<Self::Value>> {
        self.inner
            .retrieve_object(key)
            .await
            .map(Some)
            .map_err(|e| NestGateError::storage_error(&format!("Read failed: {}", e)))
    }

    fn write(
        &self,
        _key: Self::Key,
        value: Self::Value,
    ) -> impl Future<Output = Result<()>> + Send {
        let inner = &self.inner;
        async move {
            // NativeAsyncStorageProvider doesn't take key in store_object,
            // it returns the ID. We need to get metadata first or use empty metadata
            // For this adapter, we'll use Default::default() for metadata
            let metadata = ObjectMetadata::default();
            inner
                .store_object(value, metadata)
                .await
                .map(|_id| ())
                .map_err(|e| NestGateError::storage_error(&format!("Write failed: {}", e)))
        }
    }

    async fn delete(&self, key: &Self::Key) -> Result<()> {
        self.inner
            .delete_object(key)
            .await
            .map_err(|e| NestGateError::storage_error(&format!("Delete failed: {}", e)))
    }

    async fn exists(&self, key: &Self::Key) -> Result<bool> {
        // Check if we can retrieve metadata (if exists, metadata call should succeed)
        match self.inner.get_metadata(key).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    async fn metadata(&self, key: &Self::Key) -> Result<Self::Metadata> {
        self.inner
            .get_metadata(key)
            .await
            .map_err(|e| NestGateError::storage_error(&format!("Metadata read failed: {}", e)))
    }

    async fn list(&self, _prefix: Option<&str>) -> Result<Vec<Self::Key>> {
        self.inner
            .list_objects()
            .await
            .map_err(|e| NestGateError::storage_error(&format!("List failed: {}", e)))
    }
}

/// Trait bound helper for NativeAsyncStorageProvider
/// This allows the adapter to work with any implementation of the old trait
/// **DEPRECATED**: Migration complete - use canonical storage
#[deprecated(
    since = "0.9.0",
    note = "Migration to native async complete - use crate::traits::canonical::CanonicalStorage"
)]
pub trait NativeAsyncStorageProvider {
    type ObjectId: Clone + Send + Sync + 'static;
    type ObjectData: Clone + Send + Sync + 'static;
    type ObjectMetadata: Clone + Send + Sync + 'static;

    fn store_object(
        &self,
        data: Self::ObjectData,
        metadata: Self::ObjectMetadata,
    ) -> impl Future<Output = Result<Self::ObjectId>> + Send;

    fn retrieve_object(
        &self,
        id: &Self::ObjectId,
    ) -> impl Future<Output = Result<Self::ObjectData>> + Send;

    fn delete_object(&self, id: &Self::ObjectId) -> impl Future<Output = Result<()>> + Send;

    fn list_objects(&self) -> impl Future<Output = Result<Vec<Self::ObjectId>>> + Send;

    fn get_metadata(
        &self,
        id: &Self::ObjectId,
    ) -> impl Future<Output = Result<Self::ObjectMetadata>> + Send;
}

// ==================== STORAGE PRIMAL ADAPTER ====================

/// Adapter for `StoragePrimalProvider` → `CanonicalStorage`
///
/// This adapter bridges the primal-specific storage interface to the canonical storage trait.
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::traits::migration::StoragePrimalAdapter;
///
/// let primal_storage = MyStoragePrimal::new();
/// let canonical = StoragePrimalAdapter::new(primal_storage);
/// ```
pub struct StoragePrimalAdapter<T> {
    _inner: T,
    name: String,
    config: serde_json::Value,
}

impl<T> StoragePrimalAdapter<T> {
    pub fn new(inner: T) -> Self {
        Self {
            _inner: inner,
            name: "storage-primal-adapter".to_string(),
            config: serde_json::json!({}),
        }
    }
}

impl<T> CanonicalService for StoragePrimalAdapter<T>
where
    T: Send + Sync + 'static,
{
    type Config = serde_json::Value;
    type Health = serde_json::Value;
    type Metrics = serde_json::Value;
    type Error = NestGateError;

    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    async fn health(&self) -> Result<Self::Health> {
        Ok(serde_json::json!({
            "status": "healthy",
            "adapter": "storage-primal"
        }))
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn metrics(&self) -> Result<Self::Metrics> {
        Ok(serde_json::json!({
            "adapter_type": "storage-primal"
        }))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

// For StoragePrimalAdapter, we implement CanonicalStorage with String keys and Vec<u8> values
// since the primal interface is more generic
impl<T> CanonicalStorage for StoragePrimalAdapter<T>
where
    T: StoragePrimalProvider + Send + Sync + 'static,
{
    type Key = String;
    type Value = Vec<u8>;
    type Metadata = serde_json::Value;

    async fn read(&self, _key: &Self::Key) -> Result<Option<Self::Value>> {
        // StoragePrimalProvider uses UniversalRequest, so we'd need to construct one
        // This is a simplified implementation - real implementation would use the handle_request method
        Ok(None) // FUTURE: Implement proper request handling for migration adapter
    }

    async fn write(&self, _key: Self::Key, _value: Self::Value) -> Result<()> {
        // FUTURE: Implement using handle_request for migration adapter
        Ok(())
    }

    async fn delete(&self, _key: &Self::Key) -> Result<()> {
        // FUTURE: Implement using handle_request for migration adapter
        Ok(())
    }

    async fn exists(&self, _key: &Self::Key) -> Result<bool> {
        Ok(false)
    }

    async fn metadata(&self, _key: &Self::Key) -> Result<Self::Metadata> {
        Ok(serde_json::json!({}))
    }

    async fn list(&self, _prefix: Option<&str>) -> Result<Vec<Self::Key>> {
        Ok(vec![])
    }
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

// ==================== ZERO COST STORAGE ADAPTER ====================

/// Adapter for simple `ZeroCostStorageProvider<Key, Value>` → `CanonicalStorage`
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::traits::migration::ZeroCostStorageAdapter;
///
/// let old_storage = MyZeroCostStorage::new();
/// let canonical = ZeroCostStorageAdapter::new(old_storage);
/// ```
pub struct ZeroCostStorageAdapter<T, K, V> {
    inner: T,
    name: String,
    config: serde_json::Value,
    _phantom: PhantomData<(K, V)>,
}

impl<T, K, V> ZeroCostStorageAdapter<T, K, V> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            name: "zero-cost-storage-adapter".to_string(),
            config: serde_json::json!({}),
            _phantom: PhantomData,
        }
    }
}

impl<T, K, V> CanonicalService for ZeroCostStorageAdapter<T, K, V>
where
    T: Send + Sync + 'static,
    K: Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    type Config = serde_json::Value;
    type Health = serde_json::Value;
    type Metrics = serde_json::Value;
    type Error = NestGateError;

    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    async fn health(&self) -> Result<Self::Health> {
        Ok(serde_json::json!({
            "status": "healthy",
            "adapter": "zero-cost-storage"
        }))
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn metrics(&self) -> Result<Self::Metrics> {
        Ok(serde_json::json!({
            "adapter_type": "zero-cost-storage"
        }))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

impl<T, K, V> CanonicalStorage for ZeroCostStorageAdapter<T, K, V>
where
    T: ZeroCostStorageProvider<K, V> + Send + Sync + 'static,
    K: Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    type Key = K;
    type Value = V;
    type Metadata = serde_json::Value;

    async fn read(&self, key: &Self::Key) -> Result<Option<Self::Value>> {
        // retrieve returns Option<V> directly
        Ok(self.inner.retrieve(key))
    }

    async fn write(&self, key: Self::Key, value: Self::Value) -> Result<()> {
        self.inner
            .store(key, value)
            .await
            .map_err(|e| NestGateError::storage_error(&format!("Write failed: {}", e)))
    }

    async fn delete(&self, key: &Self::Key) -> Result<()> {
        // delete returns bool
        let deleted = self.inner.delete(key);
        if deleted {
            Ok(())
        } else {
            Err(NestGateError::storage_error("Delete failed: key not found"))
        }
    }

    async fn exists(&self, key: &Self::Key) -> Result<bool> {
        Ok(self.inner.retrieve(key).is_some())
    }

    async fn metadata(&self, _key: &Self::Key) -> Result<Self::Metadata> {
        // ZeroCostStorageProvider doesn't have metadata
        Ok(serde_json::json!({}))
    }

    async fn list(&self, _prefix: Option<&str>) -> Result<Vec<Self::Key>> {
        // ZeroCostStorageProvider doesn't have list operation
        Ok(vec![])
    }
}

/// Trait bound helper for simple ZeroCostStorageProvider
/// This matches the actual trait in zero_cost/traits.rs
/// **DEPRECATED**: Zero-cost patterns consolidated into canonical storage
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::unified_storage::UnifiedStorage - includes zero-cost optimizations"
)]
pub trait ZeroCostStorageProvider<K, V> {
    fn store(&self, key: K, value: V) -> impl Future<Output = Result<()>> + Send;
    fn retrieve(&self, key: &K) -> Option<V>;
    fn delete(&self, key: &K) -> bool;
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing
    struct MockNativeAsyncStorage;

    impl NativeAsyncStorageProvider for MockNativeAsyncStorage {
        type ObjectId = String;
        type ObjectData = Vec<u8>;
        type ObjectMetadata = serde_json::Value;

        async fn store_object(
            &self,
            _data: Self::ObjectData,
            _metadata: Self::ObjectMetadata,
        ) -> Result<Self::ObjectId> {
            Ok("test-id".to_string())
        }

        async fn retrieve_object(&self, _id: &Self::ObjectId) -> Result<Self::ObjectData> {
            Ok(vec![1, 2, 3])
        }

        async fn delete_object(&self, _id: &Self::ObjectId) -> Result<()> {
            Ok(())
        }

        async fn list_objects(&self) -> Result<Vec<Self::ObjectId>> {
            Ok(vec!["id1".to_string(), "id2".to_string()])
        }

        async fn get_metadata(&self, _id: &Self::ObjectId) -> Result<Self::ObjectMetadata> {
            Ok(serde_json::json!({"test": "metadata"}))
        }
    }

    #[tokio::test]
    async fn test_native_async_adapter() {
        let storage = MockNativeAsyncStorage;
        let adapter = NativeAsyncStorageAdapter::new(storage);

        // Test that it implements CanonicalStorage
        let key = "test-id".to_string();
        let result = adapter.delete(&key).await;
        assert!(result.is_ok());
    }
}
