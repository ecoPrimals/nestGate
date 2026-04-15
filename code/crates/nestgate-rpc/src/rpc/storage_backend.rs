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
use std::future::Future;
use std::sync::Arc;

use bytes::Bytes;

use crate::rpc::tarpc_types::{DatasetInfo, DatasetParams, ObjectInfo, OperationResult};
use nestgate_types::error::Result;

/// Pluggable storage backend for the tarpc/semantic-router RPC layer.
///
/// Implementors must be `Send + Sync` for use behind `Arc` in async contexts.
/// All async methods return `Send` futures for use with `tokio::spawn`.
///
/// Two canonical implementations exist:
///
/// - **`InMemoryStorageBackend`** (this crate) — for tests and lightweight usage
/// - **`CoreStorageBackend`** (`nestgate-core`) — filesystem-backed via `StorageManagerService`
pub trait StorageBackend: Send + Sync {
    /// Create a new dataset.
    fn create_dataset(
        &self,
        name: &str,
        params: DatasetParams,
    ) -> impl Future<Output = Result<DatasetInfo>> + Send + '_;

    /// List all datasets.
    fn list_datasets(&self) -> impl Future<Output = Result<Vec<DatasetInfo>>> + Send + '_;

    /// Get a single dataset by name.
    fn get_dataset(&self, name: &str) -> impl Future<Output = Result<DatasetInfo>> + Send + '_;

    /// Delete a dataset and all its objects.
    fn delete_dataset(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<OperationResult>> + Send + '_;

    /// Store an object in a dataset.
    fn store_object(
        &self,
        dataset: &str,
        key: &str,
        data: Bytes,
        metadata: Option<HashMap<String, String>>,
    ) -> impl Future<Output = Result<ObjectInfo>> + Send + '_;

    /// Retrieve an object's raw bytes from a dataset.
    fn retrieve_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl Future<Output = Result<Bytes>> + Send + '_;

    /// Get object metadata without the body.
    fn get_object_metadata(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl Future<Output = Result<ObjectInfo>> + Send + '_;

    /// List objects in a dataset with optional prefix filter and limit.
    fn list_objects(
        &self,
        dataset: &str,
        prefix: Option<&str>,
        limit: Option<usize>,
    ) -> impl Future<Output = Result<Vec<ObjectInfo>>> + Send + '_;

    /// Delete a single object.
    fn delete_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl Future<Output = Result<OperationResult>> + Send + '_;
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

impl StorageBackend for InMemoryStorageBackend {
    fn create_dataset(
        &self,
        name: &str,
        params: DatasetParams,
    ) -> impl Future<Output = Result<DatasetInfo>> + Send + '_ {
        let name = name.to_owned();
        async move {
            let mut g = self.inner.write().await;
            if g.datasets.contains_key(&name) {
                return Err(nestgate_types::error::NestGateError::storage_error(
                    format!("Dataset already exists: {name}"),
                ));
            }
            let ts = unix_ts();
            let info = DatasetInfo {
                name: name.clone(),
                description: params.description.clone(),
                created_at: ts,
                modified_at: ts,
                size_bytes: 0,
                object_count: 0,
                compression_ratio: 1.0,
                params: params.clone(),
                status: String::from("active"),
            };
            g.datasets.insert(name, info.clone());
            drop(g);
            Ok(info)
        }
    }

    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn list_datasets(&self) -> impl Future<Output = Result<Vec<DatasetInfo>>> + Send + '_ {
        async move {
            let g = self.inner.read().await;
            Ok(g.datasets.values().cloned().collect())
        }
    }

    fn get_dataset(&self, name: &str) -> impl Future<Output = Result<DatasetInfo>> + Send + '_ {
        let name = name.to_owned();
        async move {
            let g = self.inner.read().await;
            g.datasets.get(&name).cloned().ok_or_else(|| {
                nestgate_types::error::NestGateError::storage_not_found(format!("dataset {name}"))
            })
        }
    }

    fn delete_dataset(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<OperationResult>> + Send + '_ {
        let name = name.to_owned();
        async move {
            let mut g = self.inner.write().await;
            if g.datasets.remove(&name).is_none() {
                return Err(nestgate_types::error::NestGateError::storage_not_found(
                    format!("dataset {name}"),
                ));
            }
            g.objects.retain(|k, _| k.0.as_ref() != &*name);
            drop(g);
            Ok(OperationResult {
                success: true,
                message: format!("Dataset {name} deleted successfully"),
                metadata: HashMap::new(),
            })
        }
    }

    fn store_object(
        &self,
        dataset: &str,
        key: &str,
        data: Bytes,
        metadata: Option<HashMap<String, String>>,
    ) -> impl Future<Output = Result<ObjectInfo>> + Send + '_ {
        let dataset = dataset.to_owned();
        let key = key.to_owned();
        async move {
            let mut g = self.inner.write().await;
            if !g.datasets.contains_key(&dataset) {
                return Err(nestgate_types::error::NestGateError::storage_not_found(
                    format!("dataset {dataset}"),
                ));
            }
            let ts = unix_ts();
            let size = byte_len(data.len());
            let meta = metadata.unwrap_or_default();
            let dk: ObjectMapKey = (Arc::from(dataset.as_str()), Arc::from(key.as_str()));
            let info = ObjectInfo {
                key,
                dataset,
                size_bytes: size,
                created_at: ts,
                modified_at: ts,
                content_type: None,
                checksum: None,
                encrypted: false,
                compressed: false,
                metadata: meta.clone(),
            };
            g.objects.insert(dk, (data, meta));

            let dataset_name = info.dataset.as_str();
            let object_count = byte_len(
                g.objects
                    .keys()
                    .filter(|(d, _)| d.as_ref() == dataset_name)
                    .count(),
            );
            let used_bytes: u64 = g
                .objects
                .iter()
                .filter(|((d, _), _)| d.as_ref() == dataset_name)
                .map(|(_, (b, _))| byte_len(b.len()))
                .sum();
            if let Some(ds) = g.datasets.get_mut(info.dataset.as_str()) {
                ds.object_count = object_count;
                ds.size_bytes = used_bytes;
                ds.modified_at = ts;
            }
            drop(g);
            Ok(info)
        }
    }

    fn retrieve_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl Future<Output = Result<Bytes>> + Send + '_ {
        let dataset = dataset.to_owned();
        let key = key.to_owned();
        async move {
            let g = self.inner.read().await;
            let lookup: ObjectMapKey = (Arc::from(&*dataset), Arc::from(&*key));
            g.objects
                .get(&lookup)
                .map(|(b, _)| b.clone())
                .ok_or_else(|| {
                    nestgate_types::error::NestGateError::storage_not_found(format!(
                        "object {dataset}/{key}",
                    ))
                })
        }
    }

    fn get_object_metadata(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl Future<Output = Result<ObjectInfo>> + Send + '_ {
        let dataset = dataset.to_owned();
        let key = key.to_owned();
        async move {
            let g = self.inner.read().await;
            let not_found = format!("object {dataset}/{key}");
            let lookup: ObjectMapKey = (Arc::from(dataset.as_str()), Arc::from(key.as_str()));
            g.objects
                .get(&lookup)
                .map(|(data, meta)| ObjectInfo {
                    key,
                    dataset,
                    size_bytes: byte_len(data.len()),
                    created_at: unix_ts(),
                    modified_at: unix_ts(),
                    content_type: None,
                    checksum: None,
                    encrypted: false,
                    compressed: false,
                    metadata: meta.clone(),
                })
                .ok_or_else(|| nestgate_types::error::NestGateError::storage_not_found(not_found))
        }
    }

    fn list_objects(
        &self,
        dataset: &str,
        prefix: Option<&str>,
        limit: Option<usize>,
    ) -> impl Future<Output = Result<Vec<ObjectInfo>>> + Send + '_ {
        let dataset = dataset.to_owned();
        let prefix = prefix.map(str::to_owned);
        async move {
            let take_n = limit.unwrap_or(usize::MAX);
            let results: Vec<ObjectInfo> = self
                .inner
                .read()
                .await
                .objects
                .iter()
                .filter(|((ds, key), _)| {
                    if ds.as_ref() != &*dataset {
                        return false;
                    }
                    prefix.as_deref().is_none_or(|pfx| key.starts_with(pfx))
                })
                .take(take_n)
                .map(|((_, key), (data, meta))| ObjectInfo {
                    key: key.as_ref().to_string(),
                    dataset: dataset.clone(),
                    size_bytes: byte_len(data.len()),
                    created_at: unix_ts(),
                    modified_at: unix_ts(),
                    content_type: None,
                    checksum: None,
                    encrypted: false,
                    compressed: false,
                    metadata: meta.clone(),
                })
                .collect();
            Ok(results)
        }
    }

    fn delete_object(
        &self,
        dataset: &str,
        key: &str,
    ) -> impl Future<Output = Result<OperationResult>> + Send + '_ {
        let dataset = dataset.to_owned();
        let key = key.to_owned();
        async move {
            let mut g = self.inner.write().await;
            let lookup: ObjectMapKey = (Arc::from(&*dataset), Arc::from(&*key));
            if g.objects.remove(&lookup).is_none() {
                return Err(nestgate_types::error::NestGateError::storage_not_found(
                    format!("object {dataset}/{key}"),
                ));
            }
            let object_count = byte_len(
                g.objects
                    .keys()
                    .filter(|(d, _)| d.as_ref() == &*dataset)
                    .count(),
            );
            let used_bytes: u64 = g
                .objects
                .iter()
                .filter(|((d, _), _)| d.as_ref() == &*dataset)
                .map(|(_, (b, _))| byte_len(b.len()))
                .sum();
            if let Some(ds) = g.datasets.get_mut(&dataset) {
                ds.object_count = object_count;
                ds.size_bytes = used_bytes;
                ds.modified_at = unix_ts();
            }
            drop(g);
            Ok(OperationResult {
                success: true,
                message: format!("Object {dataset}/{key} deleted successfully"),
                metadata: HashMap::new(),
            })
        }
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
