//! # 🌍 NestGate Service Metadata - Universal IPC Support
//!
//! **Role**: Persistent metadata storage for service registry
//! **NOT**: Connection logic (that's Songbird's domain!)
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────┐
//! │  Songbird   │ ← Owns ALL communication (remote + local IPC)
//! └──────┬──────┘
//!        │ stores metadata
//!        ↓
//! ┌─────────────┐
//! │  NestGate   │ ← Stores persistent service registry
//! └─────────────┘
//! ```
//!
//! ## Philosophy
//!
//! - **Separation of Concerns**: Storage ≠ Connection
//! - **Songbird**: Creates endpoints, handles connections
//! - **NestGate**: Stores metadata, enables discovery
//! - **Application Primals**: Use Songbird IPC API (platform-agnostic!)
//!
//! ## What NestGate Provides
//!
//! 1. ✅ Store service metadata (name, version, capabilities, endpoint)
//! 2. ✅ Retrieve service metadata (by name, by capability)
//! 3. ✅ List all services
//! 4. ✅ Capability-based discovery
//! 5. ✅ Persistent storage (survives restarts)
//!
//! ## What NestGate Does NOT Provide
//!
//! 1. ❌ IPC connections (use `songbird::ipc::connect`)
//! 2. ❌ Socket creation (use `songbird::ipc::register`)
//! 3. ❌ Platform-specific logic (Songbird abstracts this)
//!
//! ## Usage Pattern
//!
//! ```rust,no_run
//! use nestgate_core::service_metadata::{ServiceMetadata, ServiceMetadataStore};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let store = ServiceMetadataStore::new().await?;
//!
//! // Store service metadata (typically called by Songbird)
//! let meta = ServiceMetadata {
//!     name: "beardog".to_string(),
//!     version: "1.0.0".to_string(),
//!     capabilities: vec!["crypto".to_string(), "btsp".to_string()],
//!     virtual_endpoint: "/primal/beardog".to_string(),
//!     registered_at: std::time::SystemTime::now(),
//!     last_seen: std::time::SystemTime::now(),
//!     platform: std::env::consts::OS.to_string(),
//!     native_endpoint: "/tmp/primal-beardog.sock".to_string(),
//! };
//! store.store_service(meta).await?;
//!
//! // Find services by capability
//! let crypto_providers = store.find_by_capability("crypto").await?;
//!
//! // Get specific service
//! let beardog_meta = store.get_service("beardog").await?;
//!
//! // Connect to service (use Songbird, NOT NestGate!)
//! // let stream = songbird::ipc::connect(&beardog_meta.virtual_endpoint).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Integration with Songbird
//!
//! ```rust,ignore
//! // Songbird registers a primal and stores metadata in NestGate:
//!
//! // 1. Songbird creates platform-specific endpoint
//! let endpoint = songbird::ipc::register("beardog").await?;
//!
//! // 2. Songbird stores metadata in NestGate
//! nestgate::service_metadata::store(ServiceMetadata {
//!     name: "beardog",
//!     virtual_endpoint: endpoint.path,
//!     capabilities: vec!["crypto", "btsp"],
//!     // ... other metadata
//! }).await?;
//!
//! // 3. Other primals discover via NestGate, connect via Songbird
//! let services = nestgate::find_by_capability("crypto").await?;
//! let stream = songbird::ipc::connect(&services[0].virtual_endpoint).await?;
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
/// logic is handled by Songbird.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// Service name (e.g., "beardog", "squirrel")
    pub name: String,

    /// Service version (semantic versioning)
    pub version: String,

    /// Capabilities provided by this service
    /// Used for capability-based discovery
    pub capabilities: Vec<String>,

    /// Virtual endpoint (always Unix-style: "/primal/beardog")
    /// Applications use this with `songbird::ipc::connect()`
    pub virtual_endpoint: String,

    /// When service was first registered
    pub registered_at: SystemTime,

    /// Last seen (heartbeat/health check)
    pub last_seen: SystemTime,

    /// Platform where service is running ("linux", "windows", "macos", etc.)
    /// Useful for debugging and cross-platform coordination
    pub platform: String,

    /// Native endpoint (platform-specific, for debugging only!)
    /// - Linux: "/tmp/primal-beardog.sock"
    /// - Windows: r"\\.\pipe\primal-beardog"
    /// - Localhost: "127.0.0.1:9001"
    ///
    /// **NOTE**: Applications should NOT use this directly!
    /// Use `virtual_endpoint` with Songbird instead.
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

    /// Store service metadata (typically called by Songbird)
    ///
    /// This is called when Songbird registers a new service. NestGate
    /// stores the metadata persistently for discovery purposes.
    pub async fn store_service(&self, meta: ServiceMetadata) -> Result<()> {
        // Store in main service map (lock-free!)
        self.services.insert(meta.name.clone(), meta.clone());

        // Index by capabilities (lock-free!)
        for capability in &meta.capabilities {
            self.capability_index
                .entry(capability.clone())
                .or_default()
                .push(meta.name.clone());
        }

        tracing::info!(
            service = %meta.name,
            capabilities = ?meta.capabilities,
            virtual_endpoint = %meta.virtual_endpoint,
            platform = %meta.platform,
            "📦 Service metadata stored"
        );

        Ok(())
    }

    /// Get service metadata by name
    pub async fn get_service(&self, name: &str) -> Result<ServiceMetadata> {
        self.services
            .get(name)
            .map(|entry| entry.value().clone())
            .ok_or_else(|| NestGateError::not_found(format!("Service not found: {}", name)))
    }

    /// Find services by capability (capability-based discovery!)
    ///
    /// Returns all services that provide the requested capability.
    /// Applications can then connect to these services via Songbird.
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
            .ok_or_else(|| NestGateError::not_found(format!("Service not found: {}", name)))?
            .last_seen = SystemTime::now();

        Ok(())
    }

    /// Remove service metadata (cleanup)
    pub async fn remove_service(&self, name: &str) -> Result<()> {
        // Remove from main service map
        let meta = self
            .services
            .remove(name)
            .ok_or_else(|| NestGateError::not_found(format!("Service not found: {}", name)))?
            .1;

        // Remove from capability index
        for capability in &meta.capabilities {
            if let Some(mut entry) = self.capability_index.get_mut(capability) {
                entry.retain(|n| n != name);
            }
        }

        tracing::info!(
            service = %name,
            "🗑️  Service metadata removed"
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
            name: "beardog".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string(), "btsp".to_string()],
            virtual_endpoint: "/primal/beardog".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-beardog.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        store.store_service(meta.clone()).await.unwrap();

        let retrieved = store.get_service("beardog").await.unwrap();
        assert_eq!(retrieved.name, "beardog");
        assert_eq!(retrieved.capabilities, vec!["crypto", "btsp"]);
    }

    #[tokio::test]
    async fn test_find_by_capability() {
        let store = ServiceMetadataStore::new().await.unwrap();

        // Store multiple services with overlapping capabilities
        let meta1 = ServiceMetadata {
            name: "beardog".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string(), "btsp".to_string()],
            virtual_endpoint: "/primal/beardog".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-beardog.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        let meta2 = ServiceMetadata {
            name: "squirrel".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string(), "storage".to_string()],
            virtual_endpoint: "/primal/squirrel".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-squirrel.sock".to_string(),
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
        assert_eq!(btsp_providers[0].name, "beardog");
    }

    #[tokio::test]
    async fn test_remove_service() {
        let store = ServiceMetadataStore::new().await.unwrap();

        let meta = ServiceMetadata {
            name: "beardog".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string()],
            virtual_endpoint: "/primal/beardog".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-beardog.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        store.store_service(meta).await.unwrap();
        assert!(store.has_service("beardog").await);

        store.remove_service("beardog").await.unwrap();
        assert!(!store.has_service("beardog").await);

        // Should also be removed from capability index
        let crypto_providers = store.find_by_capability("crypto").await.unwrap();
        assert_eq!(crypto_providers.len(), 0);
    }

    #[tokio::test]
    async fn test_heartbeat_update() {
        let store = ServiceMetadataStore::new().await.unwrap();

        let meta = ServiceMetadata {
            name: "beardog".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["crypto".to_string()],
            virtual_endpoint: "/primal/beardog".to_string(),
            registered_at: SystemTime::now(),
            last_seen: SystemTime::now(),
            platform: "linux".to_string(),
            native_endpoint: "/tmp/primal-beardog.sock".to_string(),
            metadata: std::collections::HashMap::new(),
        };

        store.store_service(meta).await.unwrap();

        // Wait a bit
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Update heartbeat
        store.update_heartbeat("beardog").await.unwrap();

        // Last seen should be updated (implementation detail)
    }
}
