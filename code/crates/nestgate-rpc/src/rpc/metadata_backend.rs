// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Metadata Backend Trait
//!
//! Defines the interface between the RPC layer and the service metadata
//! implementation. The semantic router delegates metadata operations through
//! this trait, allowing `nestgate-core`'s `ServiceMetadataStore` to be injected
//! at startup instead of returning `not_implemented`.
//!
//! Follows the same dependency-injection pattern as
//! [`StorageBackend`](crate::rpc::storage_backend::StorageBackend).

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;

/// Service metadata record for inter-primal discovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRecord {
    /// Service name (for logging; discovery is by capability, not name)
    pub name: String,
    /// Capabilities this service provides
    pub capabilities: Vec<String>,
    /// Primary endpoint URL
    pub endpoint: Option<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

/// Subdirectory under [`FileMetadataBackend`] base for service JSON files (`{base}/services/*.json`).
pub const METADATA_SERVICES_NAMESPACE: &str = "services";

/// Default base directory for on-disk service metadata (`…/nestgate/metadata`).
///
/// Resolution order:
/// 1. [`etcetera::base_strategy::choose_base_strategy`] → `data_dir()/nestgate/metadata`
/// 2. `$XDG_DATA_HOME/nestgate/metadata`
/// 3. `$HOME/.local/share/nestgate/metadata`
/// 4. `/var/lib/nestgate/metadata`
#[must_use]
pub fn default_metadata_base_dir() -> PathBuf {
    use etcetera::BaseStrategy;

    if let Ok(strategy) = etcetera::base_strategy::choose_base_strategy() {
        return strategy.data_dir().join("nestgate").join("metadata");
    }

    std::env::var("XDG_DATA_HOME")
        .ok()
        .map(|p| PathBuf::from(p).join("nestgate").join("metadata"))
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|h| PathBuf::from(h).join(".local/share/nestgate/metadata"))
        })
        .unwrap_or_else(|| PathBuf::from("/var/lib/nestgate/metadata"))
}

fn service_record_path(base_dir: &Path, name: &str) -> PathBuf {
    let file_key = format!("{}.json", URL_SAFE_NO_PAD.encode(name.as_bytes()));
    base_dir.join(METADATA_SERVICES_NAMESPACE).join(file_key)
}

/// Pluggable metadata backend for the semantic router.
///
/// Implementations:
/// - **`FileMetadataBackend`** (this crate) — default for `SemanticRouter::new`; JSON on disk
/// - **`InMemoryMetadataBackend`** (this crate) — tests / ephemeral mode
/// - **`CoreMetadataBackend`** (`nestgate-core`) — backed by `ServiceMetadataStore`
#[async_trait]
pub trait MetadataBackend: Send + Sync {
    /// Store service metadata.
    async fn store_service(&self, record: ServiceRecord) -> Result<()>;

    /// Retrieve service metadata by name.
    async fn get_service(&self, name: &str) -> Result<ServiceRecord>;

    /// Search services by capability.
    async fn find_by_capability(&self, capability: &str) -> Result<Vec<ServiceRecord>>;

    /// List service records whose name starts with `prefix` (e.g. `session/` for session index).
    async fn list_services_by_name_prefix(&self, prefix: &str) -> Result<Vec<ServiceRecord>>;

    /// Remove a service record by name (session index entry, etc.).
    async fn delete_service(&self, name: &str) -> Result<()>;
}

/// In-memory metadata backend for tests and standalone mode.
#[derive(Clone, Default)]
pub struct InMemoryMetadataBackend {
    services: Arc<RwLock<HashMap<String, ServiceRecord>>>,
}

impl InMemoryMetadataBackend {
    /// Create an empty in-memory metadata store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl MetadataBackend for InMemoryMetadataBackend {
    async fn store_service(&self, record: ServiceRecord) -> Result<()> {
        self.services
            .write()
            .await
            .insert(record.name.clone(), record);
        Ok(())
    }

    async fn get_service(&self, name: &str) -> Result<ServiceRecord> {
        self.services
            .read()
            .await
            .get(name)
            .cloned()
            .ok_or_else(|| {
                nestgate_types::error::NestGateError::not_found(format!("service `{name}`"))
            })
    }

    async fn find_by_capability(&self, capability: &str) -> Result<Vec<ServiceRecord>> {
        let guard = self.services.read().await;
        let matches: Vec<ServiceRecord> = guard
            .values()
            .filter(|s| s.capabilities.iter().any(|c| c == capability))
            .cloned()
            .collect();
        Ok(matches)
    }

    async fn list_services_by_name_prefix(&self, prefix: &str) -> Result<Vec<ServiceRecord>> {
        let guard = self.services.read().await;
        let mut matches: Vec<ServiceRecord> = guard
            .values()
            .filter(|s| s.name.starts_with(prefix))
            .cloned()
            .collect();
        matches.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(matches)
    }

    async fn delete_service(&self, name: &str) -> Result<()> {
        let removed = self.services.write().await.remove(name);
        if removed.is_some() {
            Ok(())
        } else {
            Err(nestgate_types::error::NestGateError::not_found(format!(
                "service `{name}`"
            )))
        }
    }
}

/// File-backed metadata: one JSON file per service under `{base_dir}/services/{key}.json`
/// where `key` is the URL-safe base64 of the UTF-8 service name.
#[derive(Debug, Clone)]
pub struct FileMetadataBackend {
    base_dir: PathBuf,
}

impl FileMetadataBackend {
    /// Open or create the backend, ensuring `{base_dir}/services` exists.
    ///
    /// # Errors
    ///
    /// Returns if the directory cannot be created.
    pub fn new(base_dir: PathBuf) -> Result<Self> {
        let services = base_dir.join(METADATA_SERVICES_NAMESPACE);
        std::fs::create_dir_all(&services)?;
        Ok(Self { base_dir })
    }

    /// Base directory passed to [`Self::new`].
    #[must_use]
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    async fn iter_service_records(&self) -> Result<Vec<ServiceRecord>> {
        let dir = self.base_dir.join(METADATA_SERVICES_NAMESPACE);
        let mut entries = match fs::read_dir(&dir).await {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e.into()),
        };

        let mut out = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            if !path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            {
                continue;
            }
            let Ok(bytes) = fs::read(&path).await else {
                continue;
            };
            if let Ok(record) = serde_json::from_slice::<ServiceRecord>(&bytes) {
                out.push(record);
            }
        }
        Ok(out)
    }
}

#[async_trait]
impl MetadataBackend for FileMetadataBackend {
    async fn store_service(&self, record: ServiceRecord) -> Result<()> {
        let path = service_record_path(&self.base_dir, &record.name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let bytes = serde_json::to_vec_pretty(&record)?;
        fs::write(&path, bytes).await?;
        Ok(())
    }

    async fn get_service(&self, name: &str) -> Result<ServiceRecord> {
        let path = service_record_path(&self.base_dir, name);
        let bytes = fs::read(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                nestgate_types::error::NestGateError::not_found(format!("service `{name}`"))
            } else {
                e.into()
            }
        })?;
        let record: ServiceRecord = serde_json::from_slice(&bytes)?;
        Ok(record)
    }

    async fn find_by_capability(&self, capability: &str) -> Result<Vec<ServiceRecord>> {
        let all = self.iter_service_records().await?;
        let mut out: Vec<ServiceRecord> = all
            .into_iter()
            .filter(|s| s.capabilities.iter().any(|c| c == capability))
            .collect();
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    async fn list_services_by_name_prefix(&self, prefix: &str) -> Result<Vec<ServiceRecord>> {
        let all = self.iter_service_records().await?;
        let mut matches: Vec<ServiceRecord> = all
            .into_iter()
            .filter(|s| s.name.starts_with(prefix))
            .collect();
        matches.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(matches)
    }

    async fn delete_service(&self, name: &str) -> Result<()> {
        let path = service_record_path(&self.base_dir, name);
        match fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(
                nestgate_types::error::NestGateError::not_found(format!("service `{name}`")),
            ),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::fs;

    #[tokio::test]
    async fn in_memory_metadata_roundtrip() {
        let backend = InMemoryMetadataBackend::new();
        let record = ServiceRecord {
            name: "test-primal".into(),
            capabilities: vec!["storage".into(), "compute".into()],
            endpoint: Some("http://localhost:8080".into()),
            metadata: HashMap::new(),
        };
        backend.store_service(record).await.expect("store");
        let fetched = backend.get_service("test-primal").await.expect("get");
        assert_eq!(fetched.capabilities.len(), 2);

        let by_cap = backend.find_by_capability("storage").await.expect("find");
        assert_eq!(by_cap.len(), 1);
        assert_eq!(by_cap[0].name, "test-primal");

        let empty = backend.find_by_capability("quantum").await.expect("find");
        assert!(empty.is_empty());
    }

    #[tokio::test]
    async fn file_metadata_roundtrip() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let record = ServiceRecord {
            name: "test-primal".into(),
            capabilities: vec!["storage".into(), "compute".into()],
            endpoint: Some("http://localhost:8080".into()),
            metadata: HashMap::new(),
        };
        backend.store_service(record).await.expect("store");
        let fetched = backend.get_service("test-primal").await.expect("get");
        assert_eq!(fetched.capabilities.len(), 2);

        let by_cap = backend.find_by_capability("storage").await.expect("find");
        assert_eq!(by_cap.len(), 1);
        assert_eq!(by_cap[0].name, "test-primal");

        let listed = backend
            .list_services_by_name_prefix("test-")
            .await
            .expect("list");
        assert_eq!(listed.len(), 1);
    }

    #[tokio::test]
    async fn file_metadata_persists_across_instances() {
        let dir = tempdir().expect("tempdir");
        let base = dir.path().to_path_buf();
        {
            let backend = FileMetadataBackend::new(base.clone()).expect("new");
            backend
                .store_service(ServiceRecord {
                    name: "persisted".into(),
                    capabilities: vec!["x".into()],
                    endpoint: None,
                    metadata: HashMap::new(),
                })
                .await
                .expect("store");
        }
        let backend2 = FileMetadataBackend::new(base).expect("reopen");
        let got = backend2.get_service("persisted").await.expect("get");
        assert_eq!(got.name, "persisted");
    }

    #[tokio::test]
    async fn file_metadata_delete_service() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        backend
            .store_service(ServiceRecord {
                name: "gone".into(),
                capabilities: vec![],
                endpoint: None,
                metadata: HashMap::new(),
            })
            .await
            .expect("store");
        backend.delete_service("gone").await.expect("delete");
        let err = backend.get_service("gone").await.expect_err("gone");
        assert!(!err.to_string().is_empty());
    }

    #[tokio::test]
    async fn file_metadata_creates_nonexistent_base_directory() {
        let root = tempdir().expect("tempdir");
        let deep = root.path().join("nested/new/metadata_base");
        assert!(!deep.exists());
        let backend = FileMetadataBackend::new(deep.clone()).expect("new");
        assert!(deep.join(METADATA_SERVICES_NAMESPACE).is_dir());
        backend
            .store_service(ServiceRecord {
                name: "probe".into(),
                capabilities: vec!["x".into()],
                endpoint: None,
                metadata: HashMap::new(),
            })
            .await
            .expect("store");
        assert!(backend.get_service("probe").await.is_ok());
    }

    #[tokio::test]
    async fn file_metadata_skips_non_utf8_json_files_in_services_dir() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let junk_path = dir
            .path()
            .join(METADATA_SERVICES_NAMESPACE)
            .join("corrupt.json");
        fs::write(&junk_path, &[0xFF, 0xFE, 0x80])
            .await
            .expect("write junk");
        let all = backend
            .list_services_by_name_prefix("")
            .await
            .expect("list");
        assert!(
            all.is_empty(),
            "invalid UTF-8 / JSON should be skipped, not crash"
        );
    }

    #[tokio::test]
    async fn file_metadata_service_name_utf8_edge_cases_roundtrip() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let name = "primal-\u{1F980}-\u{0301}A";
        backend
            .store_service(ServiceRecord {
                name: name.into(),
                capabilities: vec!["c".into()],
                endpoint: None,
                metadata: HashMap::new(),
            })
            .await
            .expect("store");
        let got = backend.get_service(name).await.expect("get");
        assert_eq!(got.name, name);
    }

    #[tokio::test]
    async fn file_metadata_find_by_capability_multiple_services_sorted() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        for n in ["zebra", "alpha", "mango"] {
            backend
                .store_service(ServiceRecord {
                    name: n.into(),
                    capabilities: vec!["shared-cap".into(), "other".into()],
                    endpoint: None,
                    metadata: HashMap::new(),
                })
                .await
                .expect("store");
        }
        let found = backend
            .find_by_capability("shared-cap")
            .await
            .expect("find");
        assert_eq!(found.len(), 3);
        assert_eq!(
            found.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(),
            vec!["alpha", "mango", "zebra"]
        );
    }

    #[tokio::test]
    async fn file_metadata_concurrent_stores() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let tasks: Vec<_> = (0u32..16)
            .map(|i| {
                let b = backend.clone();
                tokio::spawn(async move {
                    b.store_service(ServiceRecord {
                        name: format!("concurrent-{i}"),
                        capabilities: vec!["cc".into()],
                        endpoint: None,
                        metadata: HashMap::new(),
                    })
                    .await
                })
            })
            .collect();
        for t in tasks {
            t.await.expect("join").expect("store");
        }
        let listed = backend
            .list_services_by_name_prefix("concurrent-")
            .await
            .expect("list");
        assert_eq!(listed.len(), 16);
        let caps = backend.find_by_capability("cc").await.expect("find");
        assert_eq!(caps.len(), 16);
    }
}
