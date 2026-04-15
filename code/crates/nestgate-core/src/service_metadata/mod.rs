// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # NestGate service metadata — universal IPC support
//!
//! **Role**: persistent metadata storage for the service registry  
//! **NOT**: connection logic (that belongs to the IPC orchestration layer)
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────┐
//! │ IPC orchestration    │ ← owns transport (remote + local IPC)
//! └──────────┬───────────┘
//!            │ stores metadata
//!            ↓
//! ┌─────────────┐
//! │  NestGate   │ ← stores persistent service registry
//! └─────────────┘
//! ```
//!
//! ## Philosophy
//!
//! - **Separation of concerns**: storage ≠ connection
//! - **Orchestration layer**: creates endpoints, handles connections
//! - **NestGate**: stores metadata, enables discovery
//! - **Application services**: use the orchestration IPC API (platform-agnostic)
//!
//! ## What NestGate provides
//!
//! 1. Store service metadata (name, version, capabilities, endpoint)
//! 2. Retrieve service metadata (by name, by capability)
//! 3. List all services
//! 4. Capability-based discovery
//! 5. Persistent storage (survives restarts)
//!
//! ## What NestGate does not provide
//!
//! 1. IPC connections (use the orchestration layer’s `ipc::connect` pattern)
//! 2. Socket creation (use the orchestration layer’s registration API)
//! 3. Platform-specific logic (abstracted by the orchestration layer)
//!
//! ## Usage pattern
//!
//! ```rust
//! use nestgate_core::service_metadata::{ServiceMetadata, ServiceMetadataStore};
//! use std::time::SystemTime;
//!
//! # async fn example() -> nestgate_core::Result<()> {
//! let store = ServiceMetadataStore::new().await?;
//! let now = SystemTime::now();
//! let meta = ServiceMetadata {
//!     name: "example-crypto-provider".to_string(),
//!     version: "1.0.0".to_string(),
//!     capabilities: vec!["crypto".to_string(), "btsp".to_string()],
//!     virtual_endpoint: "/capability/crypto-provider".to_string(),
//!     registered_at: now,
//!     last_seen: now,
//!     platform: std::env::consts::OS.to_string(),
//!     native_endpoint: "/tmp/example-crypto-provider.sock".to_string(),
//!     metadata: std::collections::HashMap::new(),
//! };
//! store.store_service(meta).await?;
//! let _crypto_providers = store.find_by_capability("crypto").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Integration (orchestration layer)
//!
//! ```rust,ignore
//! // The orchestration layer registers a service and stores metadata in NestGate:
//!
//! // 1. Create platform-specific endpoint (orchestration API)
//! let endpoint = orchestration::ipc::register("example-crypto-provider").await?;
//!
//! // 2. Store metadata in NestGate
//! nestgate::service_metadata::store(ServiceMetadata {
//!     name: "example-crypto-provider",
//!     virtual_endpoint: endpoint.path,
//!     capabilities: vec!["crypto", "btsp"],
//!     // ... other metadata
//! }).await?;
//!
//! // 3. Other services discover via NestGate, connect via orchestration IPC
//! let services = nestgate::find_by_capability("crypto").await?;
//! let stream = orchestration::ipc::connect(&services[0].virtual_endpoint).await?;
//! ```
//!
//! ## References
//!
//! - `UNIVERSAL_IPC_ARCHITECTURE_HANDOFF_JAN_19_2026.md`
//! - `ecoPrimals/wateringHole/UNIBIN_ARCHITECTURE_STANDARD.md`
//! - `ecoPrimals/wateringHole/ECOBIN_ARCHITECTURE_STANDARD.md`

use crate::error::{NestGateError, Result};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::SystemTime;

/// Service metadata (stored persistently in NestGate)
///
/// This is what NestGate stores about each service. The actual connection
/// logic is handled by the IPC orchestration layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// Service name (e.g. `"example-primal"`, `"my-service"`)
    pub name: String,

    /// Service version (semantic versioning)
    pub version: String,

    /// Capabilities provided by this service
    /// Used for capability-based discovery
    pub capabilities: Vec<String>,

    /// Virtual endpoint (Unix-style path, e.g. `"/capability/crypto-provider"`)
    /// Applications use this with the orchestration layer’s `ipc::connect` pattern
    pub virtual_endpoint: String,

    /// When service was first registered
    pub registered_at: SystemTime,

    /// Last seen (heartbeat/health check)
    pub last_seen: SystemTime,

    /// Platform where service is running ("linux", "windows", "macos", etc.)
    /// Useful for debugging and cross-platform coordination
    pub platform: String,

    /// Native endpoint (platform-specific, for debugging only!)
    /// - Linux: `"/tmp/primal-example-primal.sock"`
    /// - Windows: `r"\\.\pipe\primal-example-primal"`
    /// - Loopback TCP: `"127.0.0.1:9001"` (from config when applicable)
    ///
    /// **NOTE**: applications should not use this directly — use `virtual_endpoint`
    /// with the orchestration layer instead.
    pub native_endpoint: String,

    /// Additional metadata (extensible)
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

/// In-memory service metadata store (lock-free with DashMap!)
///
/// This is the core storage layer for service discovery. In production,
/// this should be backed by persistent storage (NestGate's key-value store).
#[derive(Clone)]
pub struct ServiceMetadataStore {
    /// Map of service name → metadata (lock-free concurrent access!)
    services: Arc<DashMap<String, ServiceMetadata>>,

    /// Index: capability → list of service names (lock-free concurrent!)
    capability_index: Arc<DashMap<String, Vec<String>>>,
}

#[expect(
    clippy::unused_async,
    reason = "store API is async-shaped for future persistence; in-memory operations are synchronous"
)]
impl ServiceMetadataStore {
    /// Create a new in-memory metadata store
    ///
    /// **FUTURE**: Add persistence layer using NestGate's storage backend
    /// to preserve metadata across restarts (enhancement for v0.12+).
    pub async fn new() -> Result<Self> {
        Ok(Self {
            services: Arc::new(DashMap::new()),
            capability_index: Arc::new(DashMap::new()),
        })
    }

    /// Store service metadata (typically called by the orchestration layer)
    ///
    /// This is called when a new service is registered. NestGate
    /// stores the metadata persistently for discovery purposes.
    pub async fn store_service(&self, meta: ServiceMetadata) -> Result<()> {
        tracing::info!(
            service = %meta.name,
            capabilities = ?meta.capabilities,
            virtual_endpoint = %meta.virtual_endpoint,
            platform = %meta.platform,
            "Service metadata stored"
        );

        let service_name = meta.name.clone();
        for capability in &meta.capabilities {
            self.capability_index
                .entry(capability.clone())
                .or_default()
                .push(service_name.clone());
        }

        self.services.insert(service_name, meta);

        Ok(())
    }

    /// Get service metadata by name
    pub async fn get_service(&self, name: &str) -> Result<ServiceMetadata> {
        self.services
            .get(name)
            .map(|entry| entry.value().clone())
            .ok_or_else(|| NestGateError::not_found(format!("Service not found: {name}")))
    }

    /// Find services by capability (capability-based discovery!)
    ///
    /// Returns all services that provide the requested capability.
    /// Applications can then connect to these services via the orchestration layer.
    pub async fn find_by_capability(&self, capability: &str) -> Result<Vec<ServiceMetadata>> {
        let service_names = self
            .capability_index
            .get(capability)
            .map(|entry| entry.value().clone())
            .unwrap_or_default();

        let mut services = Vec::new();
        for name in service_names {
            if let Some(meta) = self.services.get(&name) {
                services.push(meta.value().clone());
            }
        }

        Ok(services)
    }

    /// List all registered services
    pub async fn list_services(&self) -> Result<Vec<ServiceMetadata>> {
        Ok(self
            .services
            .iter()
            .map(|entry| entry.value().clone())
            .collect())
    }

    /// Update service last_seen timestamp (for health monitoring)
    pub async fn update_heartbeat(&self, name: &str) -> Result<()> {
        self.services
            .get_mut(name)
            .ok_or_else(|| NestGateError::not_found(format!("Service not found: {name}")))?
            .last_seen = SystemTime::now();

        Ok(())
    }

    /// Remove service metadata (cleanup)
    pub async fn remove_service(&self, name: &str) -> Result<()> {
        let meta = self
            .services
            .remove(name)
            .ok_or_else(|| NestGateError::not_found(format!("Service not found: {name}")))?
            .1;

        for capability in &meta.capabilities {
            if let Some(mut entry) = self.capability_index.get_mut(capability) {
                entry.retain(|n| n != name);
            }
        }

        tracing::info!(
            service = %name,
            "Service metadata removed"
        );

        Ok(())
    }

    /// Check if service exists
    pub async fn has_service(&self, name: &str) -> bool {
        self.services.contains_key(name)
    }

    /// Get service count
    pub async fn service_count(&self) -> usize {
        self.services.len()
    }

    /// Get capability count
    pub async fn capability_count(&self) -> usize {
        self.capability_index.len()
    }
}

impl Default for ServiceMetadataStore {
    fn default() -> Self {
        Self {
            services: Arc::new(DashMap::new()),
            capability_index: Arc::new(DashMap::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_and_retrieve_service() {
        let store = ServiceMetadataStore::new().await.unwrap();

        let meta = ServiceMetadata {
            name: "example-primal".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string(), "btsp".to_string()],
            virtual_endpoint: "/primal/example-primal".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-example-primal.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        store.store_service(meta.clone()).await.unwrap();

        let retrieved = store.get_service("example-primal").await.unwrap();
        assert_eq!(retrieved.name, "example-primal");
        assert_eq!(retrieved.capabilities, vec!["crypto", "btsp"]);
    }

    #[tokio::test]
    async fn test_find_by_capability() {
        let store = ServiceMetadataStore::new().await.unwrap();

        // Store multiple services with overlapping capabilities
        let meta1 = ServiceMetadata {
            name: "example-primal".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string(), "btsp".to_string()],
            virtual_endpoint: "/primal/example-primal".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-example-primal.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        let meta2 = ServiceMetadata {
            name: "example-storage".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string(), "storage".to_string()],
            virtual_endpoint: "/primal/example-storage".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-example-storage.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        store.store_service(meta1).await.unwrap();
        store.store_service(meta2).await.unwrap();

        // Find all services providing 'crypto'
        let crypto_providers = store.find_by_capability("crypto").await.unwrap();
        assert_eq!(crypto_providers.len(), 2);

        // Find services providing 'btsp'
        let btsp_providers = store.find_by_capability("btsp").await.unwrap();
        assert_eq!(btsp_providers.len(), 1);
        assert_eq!(btsp_providers[0].name, "example-primal");
    }

    #[tokio::test]
    async fn test_remove_service() {
        let store = ServiceMetadataStore::new().await.unwrap();

        let meta = ServiceMetadata {
            name: "example-primal".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string()],
            virtual_endpoint: "/primal/example-primal".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-example-primal.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        store.store_service(meta).await.unwrap();
        assert!(store.has_service("example-primal").await);

        store.remove_service("example-primal").await.unwrap();
        assert!(!store.has_service("example-primal").await);

        // Should also be removed from capability index
        let crypto_providers = store.find_by_capability("crypto").await.unwrap();
        assert_eq!(crypto_providers.len(), 0);
    }

    #[tokio::test]
    async fn test_heartbeat_update() {
        let store = ServiceMetadataStore::new().await.unwrap();

        let meta = ServiceMetadata {
            name: "example-primal".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string()],
            virtual_endpoint: "/primal/example-primal".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-example-primal.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        store.store_service(meta).await.unwrap();

        let before = store.get_service("example-primal").await.unwrap().last_seen;

        store.update_heartbeat("example-primal").await.unwrap();

        let after = store.get_service("example-primal").await.unwrap().last_seen;
        assert!(after >= before, "heartbeat should advance last_seen");
    }
}
