// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::marker::PhantomData;

use crate::traits::canonical_hierarchy::{CanonicalService, CanonicalStorage};
use crate::{NestGateError, Result};

use super::traits::ZeroCostStorageProvider;

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
