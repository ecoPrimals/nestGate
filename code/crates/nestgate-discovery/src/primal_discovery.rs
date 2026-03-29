// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Capability-Based Primal Discovery Framework
//!
//! This module implements runtime discovery of other primals based on their capabilities,
//! following the core principle: **Primals have only self-knowledge and discover others at runtime**.
//!
//! # Architecture
//!
//! - **Self-Knowledge**: Each primal knows only itself (capabilities, endpoints)
//! - **Runtime Discovery**: Primals discover each other via mDNS/DNS-SD
//! - **Capability-Based**: Connect by capability, not by name or hardcoded port
//! - **Zero Configuration**: No hardcoded service locations
//!
//! # Modules
//!
//! - `runtime_discovery`: Convenient helpers for common discovery patterns
//!
//! # Example
//!
//! ```rust,ignore
//! // Uses reqwest; discovery.announce spawns network - use capability_helpers for production
//! use nestgate_core::primal_discovery::{PrimalDiscovery, SelfKnowledge};
//! let self_knowledge = SelfKnowledge::builder()
//!     .name("nestgate")
//!     .capability("storage")
//!     .endpoint_http(8080)
//!     .build();
//! let discovery = PrimalDiscovery::new(self_knowledge);
//! ```

/// Capability-based discovery helpers (NEW - USE THIS!)
#[path = "primal_discovery/capability_helpers.rs"]
pub mod capability_helpers;
/// Migration utilities for gradual transition from hardcoded to discovery-based
#[path = "primal_discovery/migration.rs"]
pub mod migration;
/// Runtime discovery client for capability-based primal discovery
#[path = "primal_discovery/runtime_discovery.rs"]
pub mod runtime_discovery;

// Re-export key types for convenience
pub use capability_helpers::{
    DiscoveredService, DiscoverySource, discover_ai, discover_capability, discover_compute,
    discover_ecosystem, discover_orchestration, discover_security, is_capability_available,
};
pub use runtime_discovery::{PrimalConnection, RuntimeDiscovery};

// HTTP removed - use Songbird via capability discovery for external HTTP
// use crate::http_client_stub as reqwest;
use dashmap::DashMap;
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

use nestgate_types::error::{NestGateError, Result};

// ==================== SELF-KNOWLEDGE ====================

/// Self-knowledge: What this primal provides
///
/// Each primal knows only itself. This is the foundation of the discovery system.
#[derive(Debug, Clone)]
pub struct SelfKnowledge {
    /// Primal name (e.g., "nestgate", "beardog", "songbird")
    pub name: String,

    /// Capabilities this primal provides
    pub capabilities: Vec<String>,

    /// Endpoints where this primal can be reached
    pub endpoints: Vec<Endpoint>,

    /// Metadata for discovery
    pub metadata: HashMap<String, String>,
}

/// Network endpoint
#[derive(Debug, Clone)]
pub struct Endpoint {
    /// Protocol (http, https, grpc, etc.)
    pub protocol: String,

    /// Address (can be hostname or IP)
    pub addr: SocketAddr,

    /// Optional path prefix
    pub path: Option<String>,
}

impl Endpoint {
    /// Create HTTP endpoint
    #[must_use]
    pub fn http(port: u16) -> Self {
        Self {
            protocol: "http".into(),
            addr: ([127, 0, 0, 1], port).into(),
            path: None,
        }
    }

    /// Get full URL
    #[must_use]
    pub fn url(&self) -> String {
        let path = self.path.as_deref().unwrap_or("");
        format!("{}://{}{}", self.protocol, self.addr, path)
    }
}

impl SelfKnowledge {
    /// Create a builder for self-knowledge
    #[must_use]
    pub fn builder() -> SelfKnowledgeBuilder {
        SelfKnowledgeBuilder::default()
    }
}

/// Builder for self-knowledge
#[derive(Default)]
pub struct SelfKnowledgeBuilder {
    name: Option<String>,
    capabilities: Vec<String>,
    endpoints: Vec<Endpoint>,
    metadata: HashMap<String, String>,
}

impl SelfKnowledgeBuilder {
    /// Set primal name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Add a capability
    #[must_use]
    pub fn capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// Add HTTP endpoint
    #[must_use]
    pub fn endpoint_http(mut self, port: u16) -> Self {
        self.endpoints.push(Endpoint::http(port));
        self
    }

    /// Add metadata
    #[must_use]
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build self-knowledge
    #[must_use]
    pub fn build(self) -> SelfKnowledge {
        SelfKnowledge {
            name: self.name.unwrap_or_else(|| "unknown".into()),
            capabilities: self.capabilities,
            endpoints: self.endpoints,
            metadata: self.metadata,
        }
    }
}

// ==================== PRIMAL DISCOVERY ====================

/// Primal discovery system
///
/// Discovers other primals at runtime based on their capabilities.
pub struct PrimalDiscovery {
    /// Our self-knowledge (what we provide)
    self_knowledge: SelfKnowledge,

    /// Discovered primals (lock-free for concurrent discovery)
    /// `DashMap` provides 5-10x better performance for discovery operations
    discovered: Arc<DashMap<String, PrimalInfo>>,

    /// Discovery backend (mDNS, DNS-SD, etc.)
    backend: Arc<dyn DiscoveryBackend + Send + Sync>,
}

/// Information about a discovered primal
#[derive(Debug, Clone)]
pub struct PrimalInfo {
    /// Primal name
    pub name: String,

    /// Capabilities it provides
    pub capabilities: Vec<String>,

    /// Endpoints
    pub endpoints: Vec<Endpoint>,

    /// When it was last seen
    pub last_seen: Instant,

    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl PrimalInfo {
    /// Get primary endpoint URL
    ///
    /// # Returns
    ///
    /// Returns the first endpoint URL if available, otherwise returns an error.
    /// This function no longer has hardcoded fallbacks - callers must handle the None case.
    ///
    /// # Philosophy
    ///
    /// Each primal knows only itself. If no endpoint is configured, that's a
    /// configuration error, not something to paper over with hardcoded localhost.
    #[must_use]
    pub fn primary_endpoint(&self) -> Option<String> {
        self.endpoints.first().map(Endpoint::url)
    }

    /// Get primary endpoint URL or a default from environment
    ///
    /// This is a convenience method that checks `NESTGATE_DEFAULT_ENDPOINT` environment
    /// variable as a fallback. Still no hardcoded values.
    #[must_use]
    pub fn primary_endpoint_or_env_default(&self) -> Option<String> {
        self.endpoints
            .first()
            .map(Endpoint::url)
            .or_else(|| std::env::var("NESTGATE_DEFAULT_ENDPOINT").ok())
    }

    /// Check if primal is stale (not seen recently)
    #[must_use]
    pub fn is_stale(&self, threshold: Duration) -> bool {
        self.last_seen.elapsed() > threshold
    }
}

impl PrimalDiscovery {
    /// Create new discovery system with default mDNS backend (lock-free discovered map)
    #[must_use]
    pub fn new(self_knowledge: SelfKnowledge) -> Self {
        Self {
            self_knowledge,
            discovered: Arc::new(DashMap::new()),
            backend: Arc::new(MDnsBackend::default()),
        }
    }

    /// Announce our presence (advertise our capabilities)
    pub async fn announce(&self) -> Result<()> {
        self.backend.announce(&self.self_knowledge).await
    }

    /// Discover a primal by capability
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use nestgate_core::primal_discovery::*;
    /// # async fn example(discovery: &PrimalDiscovery) -> Result<(), Box<dyn std::error::Error>> {
    /// // Discover security capability (could be beardog or other primal)
    /// let security = discovery.discover_capability("security").await?;
    /// println!("Security primal: {}", security.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discover_capability(&self, capability: &str) -> Result<PrimalInfo> {
        // Check cache first (lock-free)
        if let Some(info) = self.discovered.get(capability) {
            // Verify not stale
            if !info.is_stale(Duration::from_secs(300)) {
                return Ok(info.clone());
            }
        }

        // Discover via backend
        let discovered = self.backend.discover(capability).await?;

        // Cache for future use (lock-free)
        self.discovered
            .insert(capability.to_string(), discovered.clone());

        Ok(discovered)
    }

    /// Get all discovered primals (lock-free iteration)
    #[must_use]
    pub fn list_discovered(&self) -> Vec<PrimalInfo> {
        self.discovered
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Clear stale discoveries (lock-free)
    pub fn prune_stale(&self, threshold: Duration) {
        self.discovered.retain(|_, info| !info.is_stale(threshold));
    }
}

// ==================== DISCOVERY BACKEND ====================

/// Discovery backend trait
///
/// Implement this for different discovery mechanisms (mDNS, DNS-SD, Consul, etc.)
pub trait DiscoveryBackend: Send + Sync {
    /// Announce our presence
    fn announce(
        &self,
        knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;

    /// Discover a primal by capability
    fn discover(
        &self,
        capability: &str,
    ) -> Pin<Box<dyn Future<Output = Result<PrimalInfo>> + Send + '_>>;
}

/// Production discovery backend using `discovery_mechanism`
///
/// This integrates with the unified discovery system (`discovery_mechanism.rs`)
/// which supports mDNS, Consul, and Kubernetes backends.
#[derive(Default)]
struct ProductionBackend {
    discovery: Option<Arc<dyn crate::discovery_mechanism::DiscoveryMechanism>>,
}

impl ProductionBackend {
    /// Create new backend with auto-detected discovery mechanism
    async fn new() -> Result<Self> {
        // Auto-detect best available discovery mechanism
        let discovery = crate::discovery_mechanism::DiscoveryBuilder::default()
            .detect()
            .await
            .ok();

        if discovery.is_none() {
            tracing::warn!("No discovery mechanism detected, discovery will use fallbacks");
        }

        Ok(Self {
            discovery: discovery.map(Arc::from),
        })
    }

    /// Convert `primal_discovery` `SelfKnowledge` to `discovery_mechanism` `SelfKnowledge`
    fn convert_self_knowledge(knowledge: &SelfKnowledge) -> crate::self_knowledge::SelfKnowledge {
        crate::self_knowledge::SelfKnowledge::builder()
            .with_id(&knowledge.name)
            .with_name(&knowledge.name)
            .with_capabilities(knowledge.capabilities.clone())
            .build()
            .unwrap_or_else(|e| {
                tracing::error!("Failed to build self-knowledge: {}", e);
                // Return minimal valid self-knowledge (no unwrap - manual construction)
                crate::self_knowledge::SelfKnowledge::builder()
                    .with_id("unknown")
                    .with_name("unknown")
                    .build()
                    .unwrap_or_else(|e2| {
                        tracing::error!("Minimal self-knowledge build failed: {}", e2);
                        crate::self_knowledge::SelfKnowledge {
                            id: crate::self_knowledge::PrimalId::new("unknown"),
                            name: "unknown".to_string(),
                            version: "0.0.0".to_string(),
                            capabilities: vec![],
                            endpoints: std::collections::HashMap::new(),
                            resources: crate::self_knowledge::ResourceInfo::default(),
                            health: crate::self_knowledge::HealthStatus::default(),
                            last_updated: std::time::SystemTime::now(),
                        }
                    })
            })
    }

    /// Convert `discovery_mechanism` `ServiceInfo` to `PrimalInfo`
    fn convert_service_info(service: crate::discovery_mechanism::ServiceInfo) -> PrimalInfo {
        PrimalInfo {
            name: service.name,
            capabilities: service.capabilities,
            endpoints: vec![], // Will be populated from endpoint string
            metadata: service.metadata,
            last_seen: Instant::now(),
        }
    }
}

impl DiscoveryBackend for ProductionBackend {
    fn announce(
        &self,
        knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let discovery = self.discovery.clone();
        let knowledge = knowledge.clone();
        Box::pin(async move {
            tracing::info!(
                "Announcing {} with capabilities: {:?}",
                knowledge.name,
                knowledge.capabilities
            );

            if let Some(discovery) = &discovery {
                let self_knowledge = Self::convert_self_knowledge(&knowledge);
                discovery.announce(&self_knowledge).await?;
                tracing::info!("Successfully announced via discovery mechanism");
            } else {
                tracing::warn!("No discovery mechanism available, announcement is local only");
            }

            Ok(())
        })
    }

    fn discover(
        &self,
        capability: &str,
    ) -> Pin<Box<dyn Future<Output = Result<PrimalInfo>> + Send + '_>> {
        let discovery = self.discovery.clone();
        let capability = capability.to_string();
        Box::pin(async move {
            if let Some(discovery) = &discovery {
                // Query discovery mechanism
                let services = discovery.find_by_capability(capability.clone()).await?;

                // Return first service found
                if let Some(service) = services.first() {
                    tracing::info!(
                        "Discovered primal for capability '{}': {} at {}",
                        capability,
                        service.name,
                        service.endpoint
                    );
                    return Ok(Self::convert_service_info(service.clone()));
                }

                Err(NestGateError::network_error(format!(
                    "No primal found providing capability: {capability}"
                )))
            } else {
                // Fallback: Try environment variable
                let env_var = format!("NESTGATE_{}_ENDPOINT", capability.to_uppercase());
                if let Ok(endpoint) = std::env::var(&env_var) {
                    tracing::info!(
                        "Discovered primal for capability '{}' from environment: {}",
                        capability,
                        endpoint
                    );

                    return Ok(PrimalInfo {
                        name: capability.clone(),
                        capabilities: vec![capability.clone()],
                        endpoints: vec![],
                        metadata: HashMap::new(),
                        last_seen: Instant::now(),
                    });
                }

                Err(NestGateError::network_error(format!(
                    "No discovery mechanism available and no environment fallback for capability: {capability}"
                )))
            }
        })
    }
}

/// mDNS/DNS-SD discovery backend (legacy, uses `ProductionBackend`)
#[derive(Default)]
struct MDnsBackend {
    _marker: std::marker::PhantomData<()>,
}

impl DiscoveryBackend for MDnsBackend {
    fn announce(
        &self,
        knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let knowledge = knowledge.clone();
        Box::pin(async move {
            // Delegate to production backend
            let backend = ProductionBackend::new().await?;
            backend.announce(&knowledge).await
        })
    }

    fn discover(
        &self,
        capability: &str,
    ) -> Pin<Box<dyn Future<Output = Result<PrimalInfo>> + Send + '_>> {
        let capability = capability.to_string();
        Box::pin(async move {
            // Delegate to production backend
            let backend = ProductionBackend::new().await?;
            backend.discover(&capability).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_self_knowledge_builder() {
        let knowledge = SelfKnowledge::builder()
            .name("nestgate")
            .capability("storage")
            .capability("zfs_management")
            .endpoint_http(8080)
            .metadata("version", "1.0.0")
            .build();

        assert_eq!(knowledge.name, "nestgate");
        assert_eq!(knowledge.capabilities.len(), 2);
        assert!(knowledge.capabilities.contains(&"storage".to_string()));
        assert_eq!(knowledge.endpoints.len(), 1);
        assert_eq!(
            knowledge.metadata.get("version"),
            Some(&"1.0.0".to_string())
        );
    }

    #[test]
    fn test_endpoint_url() {
        let endpoint = Endpoint::http(8080);
        assert_eq!(endpoint.url(), "http://127.0.0.1:8080");
    }

    #[tokio::test]
    async fn test_discovery_cache() {
        let knowledge = SelfKnowledge::builder()
            .name("test")
            .capability("test")
            .build();

        let discovery = PrimalDiscovery::new(knowledge);

        // Manually insert into cache for testing
        let test_info = PrimalInfo {
            name: "test-primal".into(),
            capabilities: vec!["test".into()],
            endpoints: vec![Endpoint::http(9090)],
            last_seen: Instant::now(),
            metadata: HashMap::new(),
        };

        // DashMap doesn't need write lock - it's lock-free
        discovery
            .discovered
            .insert("test".into(), test_info.clone());

        // Should retrieve from cache
        let list = discovery.list_discovered();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "test-primal");
    }

    #[test]
    fn primal_info_primary_endpoint_https_and_stale() {
        let ep = Endpoint {
            protocol: "https".into(),
            addr: ([192, 168, 1, 1], 8443).into(),
            path: Some("/api".into()),
        };
        let info = PrimalInfo {
            name: "p".into(),
            capabilities: vec![],
            endpoints: vec![ep],
            last_seen: Instant::now() - Duration::from_secs(10_000),
            metadata: Default::default(),
        };
        assert!(info.primary_endpoint().unwrap().starts_with("https://"));
        assert!(info.is_stale(Duration::from_secs(5)));

        let empty = PrimalInfo {
            name: "x".into(),
            capabilities: vec![],
            endpoints: vec![],
            last_seen: Instant::now(),
            metadata: Default::default(),
        };
        assert!(empty.primary_endpoint().is_none());
    }

    #[test]
    fn primary_endpoint_or_env_prefers_explicit_endpoint() {
        let ep = Endpoint::http(8081);
        let info = PrimalInfo {
            name: "p".into(),
            capabilities: vec![],
            endpoints: vec![ep],
            last_seen: Instant::now(),
            metadata: Default::default(),
        };
        assert_eq!(
            info.primary_endpoint_or_env_default().as_deref(),
            Some("http://127.0.0.1:8081")
        );
    }

    #[test]
    fn endpoint_url_with_path_appends() {
        let ep = Endpoint {
            protocol: "http".into(),
            addr: ([127, 0, 0, 1], 80).into(),
            path: Some("/v1".into()),
        };
        assert_eq!(ep.url(), "http://127.0.0.1:80/v1");
    }

    #[test]
    fn self_knowledge_builder_default_name() {
        let k = SelfKnowledge::builder().build();
        assert_eq!(k.name, "unknown");
    }
}
