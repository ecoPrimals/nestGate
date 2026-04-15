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

mod file_backend;

pub use file_backend::{FileMetadataBackend, METADATA_SERVICES_NAMESPACE};

use nestgate_types::error::Result;
use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
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
            let matches: Vec<ServiceRecord> = {
                let guard = self.services.read().await;
                guard
                    .values()
                    .filter(|s| s.capabilities.iter().any(|c| c == &capability))
                    .cloned()
                    .collect()
            };
            Ok(matches)
        }
    }

    fn list_services_by_name_prefix(
        &self,
        prefix: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let prefix = prefix.to_owned();
        async move {
            let mut matches: Vec<ServiceRecord> = {
                let guard = self.services.read().await;
                guard
                    .values()
                    .filter(|s| s.name.starts_with(&*prefix))
                    .cloned()
                    .collect()
            };
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
mod tests;
