// SPDX-License-Identifier: AGPL-3.0-or-later
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

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use nestgate_types::error::Result;
use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
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
    default_metadata_base_dir_from_env_source(&ProcessEnv)
}

/// Like [`default_metadata_base_dir`], but reads `XDG_DATA_HOME` / `HOME` from an injectable [`EnvSource`].
#[must_use]
pub fn default_metadata_base_dir_from_env_source(env: &(impl EnvSource + ?Sized)) -> PathBuf {
    use etcetera::BaseStrategy;

    if let Ok(strategy) = etcetera::base_strategy::choose_base_strategy() {
        return strategy.data_dir().join("nestgate").join("metadata");
    }

    env.get("XDG_DATA_HOME")
        .map(|p| PathBuf::from(p).join("nestgate").join("metadata"))
        .or_else(|| {
            env.get("HOME")
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
pub trait MetadataBackend: Send + Sync {
    /// Store service metadata.
    fn store_service(&self, record: ServiceRecord) -> impl Future<Output = Result<()>> + Send + '_;

    /// Retrieve service metadata by name.
    fn get_service(&self, name: &str) -> impl Future<Output = Result<ServiceRecord>> + Send + '_;

    /// Search services by capability.
    fn find_by_capability(
        &self,
        capability: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_;

    /// List service records whose name starts with `prefix` (e.g. `session/` for session index).
    fn list_services_by_name_prefix(
        &self,
        prefix: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_;

    /// Remove a service record by name (session index entry, etc.).
    fn delete_service(&self, name: &str) -> impl Future<Output = Result<()>> + Send + '_;
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

impl MetadataBackend for InMemoryMetadataBackend {
    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn store_service(&self, record: ServiceRecord) -> impl Future<Output = Result<()>> + Send + '_ {
        async move {
            self.services
                .write()
                .await
                .insert(record.name.clone(), record);
            Ok(())
        }
    }

    fn get_service(&self, name: &str) -> impl Future<Output = Result<ServiceRecord>> + Send + '_ {
        let name = name.to_owned();
        async move {
            self.services
                .read()
                .await
                .get(&name)
                .cloned()
                .ok_or_else(|| {
                    nestgate_types::error::NestGateError::not_found(format!("service `{name}`"))
                })
        }
    }

    fn find_by_capability(
        &self,
        capability: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let capability = capability.to_owned();
        async move {
            let guard = self.services.read().await;
            let matches: Vec<ServiceRecord> = guard
                .values()
                .filter(|s| s.capabilities.iter().any(|c| c == &capability))
                .cloned()
                .collect();
            Ok(matches)
        }
    }

    fn list_services_by_name_prefix(
        &self,
        prefix: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let prefix = prefix.to_owned();
        async move {
            let guard = self.services.read().await;
            let mut matches: Vec<ServiceRecord> = guard
                .values()
                .filter(|s| s.name.starts_with(&*prefix))
                .cloned()
                .collect();
            matches.sort_by(|a, b| a.name.cmp(&b.name));
            Ok(matches)
        }
    }

    fn delete_service(&self, name: &str) -> impl Future<Output = Result<()>> + Send + '_ {
        let name = name.to_owned();
        async move {
            let removed = self.services.write().await.remove(&name);
            if removed.is_some() {
                Ok(())
            } else {
                Err(nestgate_types::error::NestGateError::not_found(format!(
                    "service `{name}`"
                )))
            }
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

impl MetadataBackend for FileMetadataBackend {
    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn store_service(&self, record: ServiceRecord) -> impl Future<Output = Result<()>> + Send + '_ {
        async move {
            let path = service_record_path(&self.base_dir, &record.name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let bytes = serde_json::to_vec_pretty(&record)?;
            fs::write(&path, bytes).await?;
            Ok(())
        }
    }

    fn get_service(&self, name: &str) -> impl Future<Output = Result<ServiceRecord>> + Send + '_ {
        let name = name.to_owned();
        async move {
            let path = service_record_path(&self.base_dir, &name);
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
    }

    fn find_by_capability(
        &self,
        capability: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let capability = capability.to_owned();
        async move {
            let all = self.iter_service_records().await?;
            let mut out: Vec<ServiceRecord> = all
                .into_iter()
                .filter(|s| s.capabilities.iter().any(|c| c == &capability))
                .collect();
            out.sort_by(|a, b| a.name.cmp(&b.name));
            Ok(out)
        }
    }

    fn list_services_by_name_prefix(
        &self,
        prefix: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let prefix = prefix.to_owned();
        async move {
            let all = self.iter_service_records().await?;
            let mut matches: Vec<ServiceRecord> = all
                .into_iter()
                .filter(|s| s.name.starts_with(&*prefix))
                .collect();
            matches.sort_by(|a, b| a.name.cmp(&b.name));
            Ok(matches)
        }
    }

    fn delete_service(&self, name: &str) -> impl Future<Output = Result<()>> + Send + '_ {
        let name = name.to_owned();
        async move {
            let path = service_record_path(&self.base_dir, &name);
            match fs::remove_file(&path).await {
                Ok(()) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(
                    nestgate_types::error::NestGateError::not_found(format!("service `{name}`")),
                ),
                Err(e) => Err(e.into()),
            }
        }
    }
}

/// Enum dispatch for runtime metadata backend selection (file vs in-memory).
///
/// Used by `SemanticRouter::new()` where the backend is chosen at startup
/// based on environment. Avoids `dyn` boxing while preserving runtime polymorphism.
#[derive(Clone)]
pub enum DefaultMetadataBackend {
    /// File-backed (production default)
    File(FileMetadataBackend),
    /// In-memory (development fallback)
    InMemory(InMemoryMetadataBackend),
}

impl MetadataBackend for DefaultMetadataBackend {
    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn store_service(&self, record: ServiceRecord) -> impl Future<Output = Result<()>> + Send + '_ {
        async move {
            match self {
                Self::File(f) => f.store_service(record).await,
                Self::InMemory(m) => m.store_service(record).await,
            }
        }
    }

    fn get_service(&self, name: &str) -> impl Future<Output = Result<ServiceRecord>> + Send + '_ {
        let name = name.to_owned();
        async move {
            match self {
                Self::File(f) => f.get_service(&name).await,
                Self::InMemory(m) => m.get_service(&name).await,
            }
        }
    }

    fn find_by_capability(
        &self,
        capability: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let capability = capability.to_owned();
        async move {
            match self {
                Self::File(f) => f.find_by_capability(&capability).await,
                Self::InMemory(m) => m.find_by_capability(&capability).await,
            }
        }
    }

    fn list_services_by_name_prefix(
        &self,
        prefix: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let prefix = prefix.to_owned();
        async move {
            match self {
                Self::File(f) => f.list_services_by_name_prefix(&prefix).await,
                Self::InMemory(m) => m.list_services_by_name_prefix(&prefix).await,
            }
        }
    }

    fn delete_service(&self, name: &str) -> impl Future<Output = Result<()>> + Send + '_ {
        let name = name.to_owned();
        async move {
            match self {
                Self::File(f) => f.delete_service(&name).await,
                Self::InMemory(m) => m.delete_service(&name).await,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::error::NestGateError;
    use tempfile::tempdir;
    use tokio::fs;
    use tokio::task::JoinSet;

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

    #[tokio::test]
    async fn in_memory_get_service_not_found() {
        let backend = InMemoryMetadataBackend::new();
        let err = backend
            .get_service("missing")
            .await
            .expect_err("missing service");
        assert!(matches!(err, NestGateError::Api(_)));
    }

    #[tokio::test]
    async fn in_memory_delete_service_not_found() {
        let backend = InMemoryMetadataBackend::new();
        let err = backend
            .delete_service("nope")
            .await
            .expect_err("delete missing");
        assert!(matches!(err, NestGateError::Api(_)));
    }

    #[tokio::test]
    async fn file_metadata_get_service_not_found() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let err = backend.get_service("absent").await.expect_err("not found");
        assert!(matches!(err, NestGateError::Api(_)));
    }

    #[tokio::test]
    async fn file_metadata_get_service_invalid_json() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let path = service_record_path(backend.base_dir(), "bad-json");
        fs::write(&path, b"{\"name\":")
            .await
            .expect("write truncated json");
        let err = backend.get_service("bad-json").await.expect_err("serde");
        assert!(!err.to_string().is_empty());
    }

    #[tokio::test]
    async fn file_metadata_delete_service_not_found() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let err = backend
            .delete_service("never-stored")
            .await
            .expect_err("not found");
        assert!(matches!(err, NestGateError::Api(_)));
    }

    #[tokio::test]
    async fn file_metadata_iter_skips_subdirectories_and_non_json_files() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let svc = dir.path().join(METADATA_SERVICES_NAMESPACE);
        fs::create_dir(svc.join("nested"))
            .await
            .expect("mkdir nested");
        fs::write(svc.join("note.txt"), b"{}").await.expect("txt");
        backend
            .store_service(ServiceRecord {
                name: "only-valid".into(),
                capabilities: vec!["x".into()],
                endpoint: None,
                metadata: HashMap::new(),
            })
            .await
            .expect("store");
        let listed = backend
            .list_services_by_name_prefix("")
            .await
            .expect("list");
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].name, "only-valid");
    }

    #[tokio::test]
    async fn file_metadata_read_dir_error_when_services_path_is_file() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let services_path = dir.path().join(METADATA_SERVICES_NAMESPACE);
        fs::remove_dir_all(&services_path)
            .await
            .expect("remove services dir");
        fs::write(&services_path, b"not-a-dir")
            .await
            .expect("replace with file");
        let err = backend
            .find_by_capability("any")
            .await
            .expect_err("read_dir should fail");
        assert!(!err.to_string().is_empty());
    }

    #[tokio::test]
    async fn default_metadata_backend_enum_dispatches_file_and_memory() {
        let dir = tempdir().expect("tempdir");
        let file = DefaultMetadataBackend::File(
            FileMetadataBackend::new(dir.path().to_path_buf()).expect("new"),
        );
        let mem = DefaultMetadataBackend::InMemory(InMemoryMetadataBackend::new());

        for (label, backend) in [("file", file), ("mem", mem)] {
            let name = format!("svc-{label}");
            backend
                .store_service(ServiceRecord {
                    name: name.clone(),
                    capabilities: vec!["c".into()],
                    endpoint: None,
                    metadata: HashMap::new(),
                })
                .await
                .expect("store");
            let got = backend.get_service(&name).await.expect("get");
            assert_eq!(got.name, name);
            backend.delete_service(&name).await.expect("delete");
        }
    }

    #[tokio::test]
    async fn file_metadata_concurrent_mixed_ops() {
        let dir = tempdir().expect("tempdir");
        let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
        let mut set = JoinSet::new();
        for i in 0u32..32 {
            let b = backend.clone();
            set.spawn(async move {
                let name = format!("mix-{i}");
                b.store_service(ServiceRecord {
                    name: name.clone(),
                    capabilities: vec!["m".into()],
                    endpoint: None,
                    metadata: HashMap::new(),
                })
                .await?;
                let _ = b.get_service(&name).await?;
                if i % 4 == 0 {
                    b.delete_service(&name).await?;
                }
                let _ = b.list_services_by_name_prefix("mix-").await?;
                let _ = b.find_by_capability("m").await?;
                Ok::<(), nestgate_types::error::NestGateError>(())
            });
        }
        while let Some(res) = set.join_next().await {
            res.expect("join").expect("task");
        }
        let remaining = backend
            .list_services_by_name_prefix("mix-")
            .await
            .expect("list");
        assert_eq!(remaining.len(), 24);
    }

    #[test]
    fn default_metadata_base_dir_paths_contain_nestgate_metadata() {
        let p = default_metadata_base_dir();
        let s = p.to_string_lossy();
        assert!(s.contains("nestgate"), "{s}");
        assert!(s.contains("metadata"), "{s}");
    }

    #[test]
    fn default_metadata_base_dir_from_env_source_ends_with_nestgate_metadata_segment() {
        // Resolution may use etcetera or env fallbacks; path always ends with …/nestgate/metadata.
        let p = default_metadata_base_dir_from_env_source(&nestgate_types::MapEnv::new());
        assert!(p.ends_with("nestgate/metadata"), "{}", p.display());
    }
}
