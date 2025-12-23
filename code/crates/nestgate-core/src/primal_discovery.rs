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
//! ```rust
//! use nestgate_core::primal_discovery::{PrimalDiscovery, SelfKnowledge};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 1. Announce self (what we provide)
//! let self_knowledge = SelfKnowledge::builder()
//!     .name("nestgate")
//!     .capability("storage")
//!     .capability("zfs_management")
//!     .endpoint_http(8080)
//!     .build();
//!
//! let discovery = PrimalDiscovery::new(self_knowledge);
//! discovery.announce().await?;
//!
//! // 2. Discover others by capability (NOT by name!)
//! let security_primal = discovery.discover_capability("security").await?;
//! let auth_url = security_primal.primary_endpoint();
//!
//! // 3. Connect (no hardcoded ports!)
//! let response = reqwest::get(&auth_url).await?;
//! # Ok(())
//! # }
//! ```

/// Migration utilities for gradual transition from hardcoded to discovery-based
pub mod migration;
/// Runtime discovery client for capability-based primal discovery
pub mod runtime_discovery;

// Re-export key types for convenience
pub use runtime_discovery::{PrimalConnection, RuntimeDiscovery};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::error::{NestGateError, Result};

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
    pub fn http(port: u16) -> Self {
        Self {
            protocol: "http".into(),
            addr: ([127, 0, 0, 1], port).into(),
            path: None,
        }
    }

    /// Get full URL
    pub fn url(&self) -> String {
        let path = self.path.as_deref().unwrap_or("");
        format!("{}://{}{}", self.protocol, self.addr, path)
    }
}

impl SelfKnowledge {
    /// Create a builder for self-knowledge
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
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Add a capability
    pub fn capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    /// Add HTTP endpoint
    pub fn endpoint_http(mut self, port: u16) -> Self {
        self.endpoints.push(Endpoint::http(port));
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build self-knowledge
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

    /// Discovered primals (capability -> primal info)
    discovered: Arc<RwLock<HashMap<String, PrimalInfo>>>,

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
    pub fn primary_endpoint(&self) -> String {
        self.endpoints
            .first()
            .map(|e| e.url())
            .unwrap_or_else(|| "http://localhost:8080".into())
    }

    /// Check if primal is stale (not seen recently)
    pub fn is_stale(&self, threshold: Duration) -> bool {
        self.last_seen.elapsed() > threshold
    }
}

impl PrimalDiscovery {
    /// Create new discovery system with default mDNS backend
    pub fn new(self_knowledge: SelfKnowledge) -> Self {
        Self {
            self_knowledge,
            discovered: Arc::new(RwLock::new(HashMap::new())),
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
    /// ```rust
    /// # use nestgate_core::primal_discovery::*;
    /// # async fn example(discovery: &PrimalDiscovery) -> Result<(), Box<dyn std::error::Error>> {
    /// // Discover security capability (could be beardog or other primal)
    /// let security = discovery.discover_capability("security").await?;
    /// println!("Security primal: {}", security.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discover_capability(&self, capability: &str) -> Result<PrimalInfo> {
        // Check cache first
        {
            let cache = self.discovered.read().await;
            if let Some(info) = cache.get(capability) {
                // Verify not stale
                if !info.is_stale(Duration::from_secs(300)) {
                    return Ok(info.clone());
                }
            }
        }

        // Discover via backend
        let discovered = self.backend.discover(capability).await?;

        // Cache for future use
        self.discovered
            .write()
            .await
            .insert(capability.to_string(), discovered.clone());

        Ok(discovered)
    }

    /// Get all discovered primals
    pub async fn list_discovered(&self) -> Vec<PrimalInfo> {
        self.discovered.read().await.values().cloned().collect()
    }

    /// Clear stale discoveries
    pub async fn prune_stale(&self, threshold: Duration) {
        self.discovered
            .write()
            .await
            .retain(|_, info| !info.is_stale(threshold));
    }
}

// ==================== DISCOVERY BACKEND ====================

/// Discovery backend trait
///
/// Implement this for different discovery mechanisms (mDNS, DNS-SD, Consul, etc.)
#[async_trait::async_trait]
pub trait DiscoveryBackend {
    /// Announce our presence
    async fn announce(&self, knowledge: &SelfKnowledge) -> Result<()>;

    /// Discover a primal by capability
    async fn discover(&self, capability: &str) -> Result<PrimalInfo>;
}

/// mDNS/DNS-SD discovery backend
#[derive(Default)]
struct MDnsBackend {
    // Will be implemented with mdns library
}

#[async_trait::async_trait]
impl DiscoveryBackend for MDnsBackend {
    async fn announce(&self, knowledge: &SelfKnowledge) -> Result<()> {
        // TODO: Implement mDNS announcement
        // For now, log that we would announce
        tracing::info!(
            "Would announce {} with capabilities: {:?}",
            knowledge.name,
            knowledge.capabilities
        );
        Ok(())
    }

    async fn discover(&self, capability: &str) -> Result<PrimalInfo> {
        // TODO: Implement mDNS discovery
        // For now, return error indicating discovery not yet implemented
        let msg = format!(
            "mDNS discovery not yet implemented for capability: {}",
            capability
        );
        Err(NestGateError::validation_error(&msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        discovery
            .discovered
            .write()
            .await
            .insert("test".into(), test_info.clone());

        // Should retrieve from cache
        let list = discovery.list_discovered().await;
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "test-primal");
    }
}
