// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use super::*;
use crate::traits::canonical_hierarchy::{CanonicalService, CanonicalStorage};
use crate::{NestGateError, Result};

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
    let adapter: ZeroCostStorageAdapter<_, String, Vec<u8>> = ZeroCostStorageAdapter::new(storage);
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
