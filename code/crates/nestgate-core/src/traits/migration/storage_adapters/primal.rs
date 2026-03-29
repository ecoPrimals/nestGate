// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use crate::traits::canonical_hierarchy::{CanonicalService, CanonicalStorage};
use crate::{NestGateError, Result};

use super::traits::StoragePrimalProvider;

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
#[expect(
    deprecated,
    reason = "Migration adapter targets hierarchy CanonicalStorage until full parity with traits::canonical::CanonicalStorage"
)]
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
