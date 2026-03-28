// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::future::Future;

use crate::traits::canonical_hierarchy::{CanonicalService, CanonicalStorage};
use crate::{NestGateError, Result};

use super::traits::NativeAsyncStorageProvider;

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
