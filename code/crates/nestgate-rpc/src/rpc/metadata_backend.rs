// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Metadata Backend Trait
//!
//! Defines the interface between the RPC layer and the service metadata
//! implementation. The semantic router delegates metadata operations through
//! this trait, allowing `nestgate-core`'s `ServiceMetadataStore` to be injected
//! at startup instead of returning `not_implemented`.
//!
//! Follows the same dependency-injection pattern as [`StorageBackend`].

use async_trait::async_trait;
use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

/// Pluggable metadata backend for the semantic router.
///
/// Two canonical implementations:
/// - **`InMemoryMetadataBackend`** (this crate) — for tests / standalone mode
/// - **`CoreMetadataBackend`** (`nestgate-core`) — backed by `ServiceMetadataStore`
#[async_trait]
pub trait MetadataBackend: Send + Sync {
    /// Store service metadata.
    async fn store_service(&self, record: ServiceRecord) -> Result<()>;

    /// Retrieve service metadata by name.
    async fn get_service(&self, name: &str) -> Result<ServiceRecord>;

    /// Search services by capability.
    async fn find_by_capability(&self, capability: &str) -> Result<Vec<ServiceRecord>>;
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
