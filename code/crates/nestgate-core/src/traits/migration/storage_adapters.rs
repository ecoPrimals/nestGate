// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
    /// Type alias for Config
    type Config = serde_json::Value;
    /// Type alias for Health
    type Health = serde_json::Value;
    /// Type alias for Metrics
    type Metrics = serde_json::Value;
    /// Type alias for Error
    type Error = NestGateError;

    /// Start
    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    /// Stop
    async fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    /// Health
    async fn health(&self) -> Result<Self::Health> {
        Ok(serde_json::json!({
            "status": "healthy",
            "adapter": "native-async-storage"
        }))
    }

    /// Config
    fn config(&self) -> &Self::Config {
        &self.config
    }

    /// Metrics
    async fn metrics(&self) -> Result<Self::Metrics> {
        Ok(serde_json::json!({
            "adapter_type": "native-async-storage"
        }))
    }

    /// Name
    fn name(&self) -> &str {
        &self.name
    }

    /// Version
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
    /// Type alias for Key
    type Key = ObjectId;
    /// Type alias for Value
    type Value = ObjectData;
    /// Type alias for Metadata
    type Metadata = ObjectMetadata;

    /// Read
    async fn read(&self, key: &Self::Key) -> Result<Option<Self::Value>> {
        self.inner
            .retrieve_object(key)
            .await
            .map(Some)
            .map_err(|e| NestGateError::storage_error(&format!("Read failed: {}", e)))
    }

    /// Write
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

    /// Deletes resource
    async fn delete(&self, key: &Self::Key) -> Result<()> {
        self.inner
            .delete_object(key)
            .await
            .map_err(|e| NestGateError::storage_error(&format!("Delete failed: {}", e)))
    }

    /// Exists
    async fn exists(&self, key: &Self::Key) -> Result<bool> {
        // Check if we can retrieve metadata (if exists, metadata call should succeed)
        match self.inner.get_metadata(key).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Metadata
    async fn metadata(&self, key: &Self::Key) -> Result<Self::Metadata> {
        self.inner
            .get_metadata(key)
            .await
            .map_err(|e| NestGateError::storage_error(&format!("Metadata read failed: {}", e)))
    }

    /// List
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
    /// Creates a new instance
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
    /// Type alias for Config
    type Config = serde_json::Value;
    /// Type alias for Health
    type Health = serde_json::Value;
    /// Type alias for Metrics
    type Metrics = serde_json::Value;
    /// Type alias for Error
    type Error = NestGateError;

    /// Start
    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    /// Stop
    async fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    /// Health
    async fn health(&self) -> Result<Self::Health> {
        Ok(serde_json::json!({
            "status": "healthy",
            "adapter": "storage-primal"
        }))
    }

    /// Config
    fn config(&self) -> &Self::Config {
        &self.config
    }

    /// Metrics
    async fn metrics(&self) -> Result<Self::Metrics> {
        Ok(serde_json::json!({
            "adapter_type": "storage-primal"
        }))
    }

    /// Name
    fn name(&self) -> &str {
        &self.name
    }

    /// Version
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
    /// Type alias for Key
    type Key = String;
    /// Type alias for Value
    type Value = Vec<u8>;
    /// Type alias for Metadata
    type Metadata = serde_json::Value;

    /// Read
    async fn read(&self, _key: &Self::Key) -> Result<Option<Self::Value>> {
        // StoragePrimalProvider uses UniversalRequest, so we'd need to construct one
        // This is a simplified implementation - real implementation would use the handle_request method
        Ok(None) // FUTURE: Implement proper request handling for migration adapter
    }

    /// Write
    async fn write(&self, _key: Self::Key, _value: Self::Value) -> Result<()> {
        // FUTURE: Implement using handle_request for migration adapter
        Ok(())
    }

    /// Deletes resource
    async fn delete(&self, _key: &Self::Key) -> Result<()> {
        // FUTURE: Implement using handle_request for migration adapter
        Ok(())
    }

    /// Exists
    async fn exists(&self, _key: &Self::Key) -> Result<bool> {
        Ok(false)
    }

    /// Metadata
    async fn metadata(&self, _key: &Self::Key) -> Result<Self::Metadata> {
        Ok(serde_json::json!({}))
    }

    /// List
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
    /// Creates a new instance
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
    /// Type alias for Config
    type Config = serde_json::Value;
    /// Type alias for Health
    type Health = serde_json::Value;
    /// Type alias for Metrics
    type Metrics = serde_json::Value;
    /// Type alias for Error
    type Error = NestGateError;

    /// Start
    async fn start(&mut self) -> Result<()> {
        Ok(())
    }

    /// Stop
    async fn stop(&mut self) -> Result<()> {
        Ok(())
    }

    /// Health
    async fn health(&self) -> Result<Self::Health> {
        Ok(serde_json::json!({
            "status": "healthy",
            "adapter": "zero-cost-storage"
        }))
    }

    /// Config
    fn config(&self) -> &Self::Config {
        &self.config
    }

    /// Metrics
    async fn metrics(&self) -> Result<Self::Metrics> {
        Ok(serde_json::json!({
            "adapter_type": "zero-cost-storage"
        }))
    }

    /// Name
    fn name(&self) -> &str {
        &self.name
    }

    /// Version
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
    /// Type alias for Key
    type Key = K;
    /// Type alias for Value
    type Value = V;
    /// Type alias for Metadata
    type Metadata = serde_json::Value;

    /// Read
    async fn read(&self, key: &Self::Key) -> Result<Option<Self::Value>> {
        // retrieve returns Option<V> directly
        Ok(self.inner.retrieve(key))
    }

    /// Write
    async fn write(&self, key: Self::Key, value: Self::Value) -> Result<()> {
        self.inner
            .store(key, value)
            .await
            .map_err(|e| NestGateError::storage_error(&format!("Write failed: {}", e)))
    }

    /// Deletes resource
    async fn delete(&self, key: &Self::Key) -> Result<()> {
        // delete returns bool
        let deleted = self.inner.delete(key);
        if deleted {
            Ok(())
        } else {
            Err(NestGateError::storage_error("Delete failed: key not found"))
        }
    }

    /// Exists
    async fn exists(&self, key: &Self::Key) -> Result<bool> {
        Ok(self.inner.retrieve(key).is_some())
    }

    /// Metadata
    async fn metadata(&self, _key: &Self::Key) -> Result<Self::Metadata> {
        // ZeroCostStorageProvider doesn't have metadata
        Ok(serde_json::json!({}))
    }

    /// List
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
/// ZeroCostStorageProvider trait
pub trait ZeroCostStorageProvider<K, V> {
    /// Store
    fn store(&self, key: K, value: V) -> impl Future<Output = Result<()>> + Send;
    /// Retrieve
    fn retrieve(&self, key: &K) -> Option<V>;
    /// Deletes resource
    fn delete(&self, key: &K) -> bool;
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementation for testing
    struct MockNativeAsyncStorage;

    impl NativeAsyncStorageProvider for MockNativeAsyncStorage {
        /// Type alias for ObjectId
        type ObjectId = String;
        /// Type alias for ObjectData
        type ObjectData = Vec<u8>;
        /// Type alias for ObjectMetadata
        type ObjectMetadata = serde_json::Value;

        /// Store Object
        async fn store_object(
            &self,
            _data: Self::ObjectData,
            _metadata: Self::ObjectMetadata,
        ) -> Result<Self::ObjectId> {
            Ok("test-id".to_string())
        }

        /// Retrieve Object
        async fn retrieve_object(&self, _id: &Self::ObjectId) -> Result<Self::ObjectData> {
            Ok(vec![1, 2, 3])
        }

        /// Deletes  Object
        async fn delete_object(&self, _id: &Self::ObjectId) -> Result<()> {
            Ok(())
        }

        /// List Objects
        async fn list_objects(&self) -> Result<Vec<Self::ObjectId>> {
            Ok(vec!["id1".to_string(), "id2".to_string()])
        }

        /// Gets Metadata
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

    #[test]
    fn test_native_async_adapter_with_metadata() {
        let storage = MockNativeAsyncStorage;
        let adapter = NativeAsyncStorageAdapter::with_metadata(
            storage,
            "custom-name".to_string(),
            "1.0.0".to_string(),
        );
        assert_eq!(adapter.name(), "custom-name");
        assert_eq!(adapter.version(), "1.0.0");
    }

    #[tokio::test]
    async fn test_native_async_adapter_read() {
        let storage = MockNativeAsyncStorage;
        let adapter = NativeAsyncStorageAdapter::new(storage);
        let key = "test-id".to_string();
        let result = adapter.read(&key).await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.is_some());
        assert_eq!(data.unwrap(), vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_native_async_adapter_exists() {
        let storage = MockNativeAsyncStorage;
        let adapter = NativeAsyncStorageAdapter::new(storage);
        let key = "test-id".to_string();
        let result = adapter.exists(&key).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_native_async_adapter_metadata() {
        let storage = MockNativeAsyncStorage;
        let adapter = NativeAsyncStorageAdapter::new(storage);
        let key = "test-id".to_string();
        let result = adapter.metadata(&key).await;
        assert!(result.is_ok());
        assert!(result.unwrap().get("test").is_some());
    }

    #[tokio::test]
    async fn test_native_async_adapter_list() {
        let storage = MockNativeAsyncStorage;
        let adapter = NativeAsyncStorageAdapter::new(storage);
        let result = adapter.list(None).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_native_async_adapter_health() {
        let storage = MockNativeAsyncStorage;
        let mut adapter = NativeAsyncStorageAdapter::new(storage);
        let _ = adapter.start().await;
        let health = adapter.health().await;
        assert!(health.is_ok());
        assert_eq!(health.unwrap()["status"], "healthy");
        let _ = adapter.stop().await;
    }

    struct MockStoragePrimal {
        config: serde_json::Value,
    }
    impl MockStoragePrimal {
        fn new() -> Self {
            Self {
                config: serde_json::json!({}),
            }
        }
    }

    impl crate::traits::canonical_hierarchy::CanonicalService for MockStoragePrimal {
        type Config = serde_json::Value;
        type Health = serde_json::Value;
        type Metrics = serde_json::Value;
        type Error = NestGateError;

        fn start(&mut self) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn stop(&mut self) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }
        fn health(&self) -> impl std::future::Future<Output = Result<Self::Health>> + Send {
            std::future::ready(Ok(serde_json::json!({"status":"ok"})))
        }
        fn config(&self) -> &Self::Config {
            &self.config
        }
        fn metrics(&self) -> impl std::future::Future<Output = Result<Self::Metrics>> + Send {
            std::future::ready(Ok(serde_json::json!({})))
        }
        fn name(&self) -> &str {
            "mock-primal"
        }
        fn version(&self) -> &str {
            "0.0.0"
        }
    }

    impl crate::traits::canonical_hierarchy::CanonicalStorage for MockStoragePrimal {
        type Key = String;
        type Value = Vec<u8>;
        type Metadata = serde_json::Value;

        async fn read(&self, _key: &Self::Key) -> Result<Option<Self::Value>> {
            Ok(None)
        }

        async fn write(&self, _key: Self::Key, _value: Self::Value) -> Result<()> {
            Ok(())
        }

        async fn delete(&self, _key: &Self::Key) -> Result<()> {
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

    #[test]
    fn test_storage_primal_adapter_new() {
        let storage = MockStoragePrimal::new();
        let adapter = StoragePrimalAdapter::new(storage);
        assert_eq!(adapter.name(), "storage-primal-adapter");
    }

    #[tokio::test]
    async fn test_storage_primal_adapter_health() {
        let storage = MockStoragePrimal::new();
        let mut adapter = StoragePrimalAdapter::new(storage);
        let _ = adapter.start().await;
        let health = adapter.health().await;
        assert!(health.is_ok());
        assert_eq!(health.unwrap()["adapter"], "storage-primal");
    }

    struct MockZeroCostStorage;

    impl ZeroCostStorageProvider<String, Vec<u8>> for MockZeroCostStorage {
        fn store(
            &self,
            _key: String,
            _value: Vec<u8>,
        ) -> impl std::future::Future<Output = Result<()>> + Send {
            std::future::ready(Ok(()))
        }

        fn retrieve(&self, key: &String) -> Option<Vec<u8>> {
            if key == "exists" {
                Some(vec![1, 2, 3])
            } else {
                None
            }
        }

        fn delete(&self, key: &String) -> bool {
            key == "exists"
        }
    }

    #[test]
    fn test_zero_cost_adapter_new() {
        let storage = MockZeroCostStorage;
        let adapter: ZeroCostStorageAdapter<_, String, Vec<u8>> =
            ZeroCostStorageAdapter::new(storage);
        assert_eq!(adapter.name(), "zero-cost-storage-adapter");
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_read() {
        let storage = MockZeroCostStorage;
        let adapter = ZeroCostStorageAdapter::new(storage);
        let key = "exists".to_string();
        let result = adapter.read(&key).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(vec![1, 2, 3]));
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_read_missing() {
        let storage = MockZeroCostStorage;
        let adapter = ZeroCostStorageAdapter::new(storage);
        let key = "missing".to_string();
        let result = adapter.read(&key).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_exists() {
        let storage = MockZeroCostStorage;
        let adapter = ZeroCostStorageAdapter::new(storage);
        assert!(adapter.exists(&"exists".to_string()).await.unwrap());
        assert!(!adapter.exists(&"missing".to_string()).await.unwrap());
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_delete() {
        let storage = MockZeroCostStorage;
        let adapter = ZeroCostStorageAdapter::new(storage);
        let result = adapter.delete(&"exists".to_string()).await;
        assert!(result.is_ok());
        let result = adapter.delete(&"missing".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_zero_cost_adapter_health() {
        let storage = MockZeroCostStorage;
        let mut adapter: ZeroCostStorageAdapter<_, String, Vec<u8>> =
            ZeroCostStorageAdapter::new(storage);
        let _ = adapter.start().await;
        let health = adapter.health().await;
        assert!(health.is_ok());
        assert_eq!(health.unwrap()["adapter"], "zero-cost-storage");
    }
}
