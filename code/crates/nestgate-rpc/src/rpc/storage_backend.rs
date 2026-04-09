// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Storage Backend Trait
//!
//! Defines the interface between the RPC layer and the storage implementation.
//! The tarpc server delegates all storage operations through this trait, allowing
//! `nestgate-core`'s `StorageManagerService` (filesystem-backed) to be injected
//! at daemon startup instead of using an in-memory `HashMap`.
//!
//! This resolves **NG-01**: the semantic router and tarpc paths now uniformly
//! route through the same storage backend as `nestgate-core`.

use std::collections::HashMap;
use std::sync::Arc;

use bytes::Bytes;

use crate::rpc::tarpc_types::{DatasetInfo, DatasetParams, ObjectInfo, OperationResult};
use nestgate_types::error::Result;

/// Pluggable storage backend for the tarpc/semantic-router RPC layer.
///
/// Implementors must be `Send + Sync` for use behind `Arc` in async contexts.
/// Two canonical implementations exist:
///
/// - **`InMemoryStorageBackend`** (this crate) — for tests and lightweight usage
/// - **`CoreStorageBackend`** (`nestgate-core`) — filesystem-backed via `StorageManagerService`
#[async_trait::async_trait]
pub trait StorageBackend: Send + Sync {
    /// Create a new dataset.
    async fn create_dataset(&self, name: &str, params: DatasetParams) -> Result<DatasetInfo>;

    /// List all datasets.
    async fn list_datasets(&self) -> Result<Vec<DatasetInfo>>;

    /// Get a single dataset by name.
    async fn get_dataset(&self, name: &str) -> Result<DatasetInfo>;

    /// Delete a dataset and all its objects.
    async fn delete_dataset(&self, name: &str) -> Result<OperationResult>;

    /// Store an object in a dataset.
    async fn store_object(
        &self,
        dataset: &str,
        key: &str,
        data: Bytes,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<ObjectInfo>;

    /// Retrieve an object's raw bytes from a dataset.
    async fn retrieve_object(&self, dataset: &str, key: &str) -> Result<Bytes>;

    /// Get object metadata without the body.
    async fn get_object_metadata(&self, dataset: &str, key: &str) -> Result<ObjectInfo>;

    /// List objects in a dataset with optional prefix filter and limit.
    async fn list_objects(
        &self,
        dataset: &str,
        prefix: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<ObjectInfo>>;

    /// Delete a single object.
    async fn delete_object(&self, dataset: &str, key: &str) -> Result<OperationResult>;
}

// ---------------------------------------------------------------------------
// In-memory implementation (tests / lightweight standalone)
// ---------------------------------------------------------------------------

use std::time::SystemTime;
use tokio::sync::RwLock;

fn unix_ts() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

#[inline]
fn byte_len(n: usize) -> u64 {
    u64::try_from(n).unwrap_or(u64::MAX)
}

type StoredPayload = (Bytes, HashMap<String, String>);
type ObjectMapKey = (Arc<str>, Arc<str>);

/// In-memory storage backend for tests and lightweight standalone mode.
#[derive(Default)]
struct InnerStore {
    datasets: HashMap<String, DatasetInfo>,
    objects: HashMap<ObjectMapKey, StoredPayload>,
}

/// In-memory [`StorageBackend`] backed by `HashMap`.
///
/// Useful for tests, fuzz targets, and standalone mode when `nestgate-core` is
/// not wired in. Production daemons should use the `CoreStorageBackend` from
/// `nestgate-core` instead.
#[derive(Clone, Default)]
pub struct InMemoryStorageBackend {
    inner: Arc<RwLock<InnerStore>>,
}

impl InMemoryStorageBackend {
    /// Create an empty in-memory store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl StorageBackend for InMemoryStorageBackend {
    async fn create_dataset(&self, name: &str, params: DatasetParams) -> Result<DatasetInfo> {
        let mut g = self.inner.write().await;
        if g.datasets.contains_key(name) {
            return Err(nestgate_types::error::NestGateError::storage_error(
                format!("Dataset already exists: {name}"),
            ));
        }
        let ts = unix_ts();
        let info = DatasetInfo {
            name: name.to_string(),
            description: params.description.clone(),
            created_at: ts,
            modified_at: ts,
            size_bytes: 0,
            object_count: 0,
            compression_ratio: 1.0,
            params: params.clone(),
            status: String::from("active"),
        };
        g.datasets.insert(name.to_string(), info.clone());
        Ok(info)
    }

    async fn list_datasets(&self) -> Result<Vec<DatasetInfo>> {
        let g = self.inner.read().await;
        Ok(g.datasets.values().cloned().collect())
    }

    async fn get_dataset(&self, name: &str) -> Result<DatasetInfo> {
        let g = self.inner.read().await;
        g.datasets.get(name).cloned().ok_or_else(|| {
            nestgate_types::error::NestGateError::storage_not_found(format!("dataset {name}"))
        })
    }

    async fn delete_dataset(&self, name: &str) -> Result<OperationResult> {
        let mut g = self.inner.write().await;
        if g.datasets.remove(name).is_none() {
            return Err(nestgate_types::error::NestGateError::storage_not_found(
                format!("dataset {name}"),
            ));
        }
        g.objects.retain(|k, _| k.0.as_ref() != name);
        Ok(OperationResult {
            success: true,
            message: format!("Dataset {name} deleted successfully"),
            metadata: HashMap::new(),
        })
    }

    async fn store_object(
        &self,
        dataset: &str,
        key: &str,
        data: Bytes,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<ObjectInfo> {
        let mut g = self.inner.write().await;
        if !g.datasets.contains_key(dataset) {
            return Err(nestgate_types::error::NestGateError::storage_not_found(
                format!("dataset {dataset}"),
            ));
        }
        let ts = unix_ts();
        let size = byte_len(data.len());
        let meta = metadata.unwrap_or_default();
        let info = ObjectInfo {
            key: key.to_string(),
            dataset: dataset.to_string(),
            size_bytes: size,
            created_at: ts,
            modified_at: ts,
            content_type: None,
            checksum: None,
            encrypted: false,
            compressed: false,
            metadata: meta.clone(),
        };
        let dk: ObjectMapKey = (Arc::from(dataset), Arc::from(key));
        g.objects.insert(dk, (data, meta));

        let object_count = byte_len(
            g.objects
                .keys()
                .filter(|(d, _)| d.as_ref() == dataset)
                .count(),
        );
        let used_bytes: u64 = g
            .objects
            .iter()
            .filter(|((d, _), _)| d.as_ref() == dataset)
            .map(|(_, (b, _))| byte_len(b.len()))
            .sum();
        if let Some(ds) = g.datasets.get_mut(dataset) {
            ds.object_count = object_count;
            ds.size_bytes = used_bytes;
            ds.modified_at = ts;
        }
        Ok(info)
    }

    async fn retrieve_object(&self, dataset: &str, key: &str) -> Result<Bytes> {
        let g = self.inner.read().await;
        let lookup: ObjectMapKey = (Arc::from(dataset), Arc::from(key));
        g.objects
            .get(&lookup)
            .map(|(b, _)| b.clone())
            .ok_or_else(|| {
                nestgate_types::error::NestGateError::storage_not_found(format!(
                    "object {dataset}/{key}",
                ))
            })
    }

    async fn get_object_metadata(&self, dataset: &str, key: &str) -> Result<ObjectInfo> {
        let g = self.inner.read().await;
        let lookup: ObjectMapKey = (Arc::from(dataset), Arc::from(key));
        g.objects
            .get(&lookup)
            .map(|(data, meta)| ObjectInfo {
                key: key.to_string(),
                dataset: dataset.to_string(),
                size_bytes: byte_len(data.len()),
                created_at: unix_ts(),
                modified_at: unix_ts(),
                content_type: None,
                checksum: None,
                encrypted: false,
                compressed: false,
                metadata: meta.clone(),
            })
            .ok_or_else(|| {
                nestgate_types::error::NestGateError::storage_not_found(format!(
                    "object {dataset}/{key}",
                ))
            })
    }

    async fn list_objects(
        &self,
        dataset: &str,
        prefix: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<ObjectInfo>> {
        let g = self.inner.read().await;
        let mut results = Vec::new();
        for ((ds, key), (data, meta)) in &g.objects {
            if ds.as_ref() != dataset {
                continue;
            }
            if let Some(pfx) = prefix
                && !key.starts_with(pfx)
            {
                continue;
            }
            results.push(ObjectInfo {
                key: key.as_ref().to_string(),
                dataset: dataset.to_string(),
                size_bytes: byte_len(data.len()),
                created_at: unix_ts(),
                modified_at: unix_ts(),
                content_type: None,
                checksum: None,
                encrypted: false,
                compressed: false,
                metadata: meta.clone(),
            });
            if let Some(lim) = limit
                && results.len() >= lim
            {
                break;
            }
        }
        Ok(results)
    }

    async fn delete_object(&self, dataset: &str, key: &str) -> Result<OperationResult> {
        let mut g = self.inner.write().await;
        let lookup: ObjectMapKey = (Arc::from(dataset), Arc::from(key));
        if g.objects.remove(&lookup).is_none() {
            return Err(nestgate_types::error::NestGateError::storage_not_found(
                format!("object {dataset}/{key}"),
            ));
        }
        let object_count = byte_len(
            g.objects
                .keys()
                .filter(|(d, _)| d.as_ref() == dataset)
                .count(),
        );
        let used_bytes: u64 = g
            .objects
            .iter()
            .filter(|((d, _), _)| d.as_ref() == dataset)
            .map(|(_, (b, _))| byte_len(b.len()))
            .sum();
        if let Some(ds) = g.datasets.get_mut(dataset) {
            ds.object_count = object_count;
            ds.size_bytes = used_bytes;
            ds.modified_at = unix_ts();
        }
        Ok(OperationResult {
            success: true,
            message: format!("Object {dataset}/{key} deleted successfully"),
            metadata: HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn in_memory_roundtrip() {
        let backend = InMemoryStorageBackend::new();
        backend
            .create_dataset("ds", DatasetParams::default())
            .await
            .expect("create");
        backend
            .store_object("ds", "k1", Bytes::from(vec![1, 2, 3]), None)
            .await
            .expect("store");
        let data = backend.retrieve_object("ds", "k1").await.expect("get");
        assert_eq!(data.as_ref(), [1u8, 2, 3]);
        let meta = backend.get_object_metadata("ds", "k1").await.expect("meta");
        assert_eq!(meta.size_bytes, 3);
        let listed = backend.list_objects("ds", None, None).await.expect("list");
        assert_eq!(listed.len(), 1);
        backend.delete_object("ds", "k1").await.expect("del obj");
        backend.delete_dataset("ds").await.expect("del ds");
        assert!(backend.list_datasets().await.expect("list").is_empty());
    }
}
