//! # Discovery Mechanism Abstraction
//!
//! **Vendor-agnostic service discovery for the infant discovery pattern.**
//!
//! This module provides a pluggable discovery mechanism that works across
//! different infrastructure: bare metal (mDNS), cloud (consul/etcd), or
//! orchestrated (Kubernetes).
//!
//! ## Philosophy
//!
//! Each primal starts with **zero knowledge** and discovers its ecosystem
//! at runtime using whatever discovery mechanism is available:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │ Primal Startup (Infant Discovery)                   │
//! ├─────────────────────────────────────────────────────┤
//! │                                                      │
//! │  1. Self-Awareness: "I am NestGate, I provide      │
//! │     Storage and ZFS capabilities"                   │
//! │                                                      │
//! │  2. Detect Discovery: Auto-detect available         │
//! │     mechanism (k8s? consul? mdns?)                  │
//! │                                                      │
//! │  3. Announce: Register my capabilities               │
//! │                                                      │
//! │  4. Discover: Find primals by capability             │
//! │     (not by name!)                                   │
//! │                                                      │
//! └─────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use nestgate_core::discovery_mechanism::{DiscoveryMechanism, ServiceInfo};
//! use nestgate_core::self_knowledge::SelfKnowledge;
//! use nestgate_core::capabilities::Capability;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // 1. Auto-detect available discovery mechanism
//! let discovery = DiscoveryMechanism::detect().await?
//!     .unwrap_or(DiscoveryMechanism::mdns_default());
//!
//! // 2. Build self-knowledge (only know ourselves)
//! let self_knowledge = SelfKnowledge::builder()
//!     .with_name("nestgate")
//!     .with_capability(Capability::Storage)
//!     .with_capability(Capability::ZfsManagement)
//!     .build()?;
//!
//! // 3. Announce ourselves
//! discovery.announce(&self_knowledge).await?;
//!
//! // 4. Discover others by capability (not by name!)
//! let orchestrators = discovery
//!     .find_by_capability(Capability::Orchestration)
//!     .await?;
//!
//! for orch in orchestrators {
//!     println!("Found orchestrator: {} at {}", orch.id, orch.endpoint);
//! }
//! # Ok(())
//! # }
//! ```

use crate::self_knowledge::SelfKnowledge;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Simplified capability type (will use proper taxonomy later)
pub type Capability = String;

/// Service discovery information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Unique service ID
    pub id: String,
    /// Service name (for logging/debugging only, not for discovery!)
    pub name: String,
    /// Capabilities this service provides
    pub capabilities: Vec<Capability>,
    /// Primary endpoint
    pub endpoint: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Health check endpoint
    pub health_endpoint: Option<String>,
}

/// Discovery mechanism trait
///
/// Implemented by different discovery backends (mDNS, Consul, Kubernetes, etc.)
#[async_trait::async_trait]
pub trait DiscoveryMechanism: Send + Sync {
    /// Announce this primal's presence
    async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()>;

    /// Find services by capability
    async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>>;

    /// Find a specific service by ID (for re-connection)
    async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>>;

    /// Health check: is this service still available?
    async fn health_check(&self, service_id: &str) -> Result<bool>;

    /// Deregister this primal (graceful shutdown)
    async fn deregister(&self, service_id: &str) -> Result<()>;

    /// Get mechanism name (for logging)
    fn mechanism_name(&self) -> &'static str;
}

/// Discovery mechanism builder
pub struct DiscoveryBuilder {
    /// Timeout for discovery operations
    timeout: Duration,
    /// Cache duration for discovered services
    cache_duration: Duration,
    /// Preferred mechanism (if multiple available)
    preferred_mechanism: Option<String>,
}

impl Default for DiscoveryBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            cache_duration: Duration::from_secs(60),
            preferred_mechanism: None,
        }
    }
}

impl DiscoveryBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set operation timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set cache duration
    pub fn with_cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    }

    /// Prefer a specific mechanism
    pub fn prefer_mechanism(mut self, mechanism: impl Into<String>) -> Self {
        self.preferred_mechanism = Some(mechanism.into());
        self
    }

    /// Auto-detect best available discovery mechanism
    ///
    /// Detection order (by preference):
    /// 1. Kubernetes (if KUBERNETES_SERVICE_HOST set)
    /// 2. Consul (if CONSUL_HTTP_ADDR set)
    /// 3. mDNS (default fallback)
    pub async fn detect(self) -> Result<Box<dyn DiscoveryMechanism>> {
        // Check for kubernetes
        if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
            #[cfg(feature = "kubernetes")]
            return Ok(Box::new(k8s::KubernetesDiscovery::new(self).await?));

            #[cfg(not(feature = "kubernetes"))]
            tracing::info!("Kubernetes detected but feature not enabled, falling back");
        }

        // Check for consul
        if std::env::var("CONSUL_HTTP_ADDR").is_ok() {
            #[cfg(feature = "consul")]
            return Ok(Box::new(consul::ConsulDiscovery::new(self).await?));

            #[cfg(not(feature = "consul"))]
            tracing::info!("Consul detected but feature not enabled, falling back");
        }

        // Default to mDNS
        Ok(Box::new(mdns::MdnsDiscovery::new(self).await?))
    }

    /// Build mDNS discovery (default)
    pub async fn build_mdns(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(mdns::MdnsDiscovery::new(self).await?))
    }

    /// Build Consul discovery
    #[cfg(feature = "consul")]
    pub async fn build_consul(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(consul::ConsulDiscovery::new(self).await?))
    }

    /// Build Kubernetes discovery
    #[cfg(feature = "kubernetes")]
    pub async fn build_kubernetes(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(k8s::KubernetesDiscovery::new(self).await?))
    }
}

/// mDNS-based discovery (default, works everywhere)
pub mod mdns {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// In-memory service registry for mDNS
    ///
    /// This is a simple implementation that stores services in memory.
    /// In production, this would use actual mDNS protocol (avahi-daemon, dns-sd, etc.)
    type ServiceRegistry = Arc<RwLock<HashMap<String, ServiceInfo>>>;

    /// mDNS discovery mechanism
    pub struct MdnsDiscovery {
        timeout: Duration,
        cache_duration: Duration,
        registry: ServiceRegistry,
        announced_service_id: Arc<RwLock<Option<String>>>,
    }

    impl MdnsDiscovery {
        /// Create new mDNS discovery
        pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
            Ok(Self {
                timeout: builder.timeout,
                cache_duration: builder.cache_duration,
                registry: Arc::new(RwLock::new(HashMap::new())),
                announced_service_id: Arc::new(RwLock::new(None)),
            })
        }

        /// Create service info from self-knowledge
        fn create_service_info(self_knowledge: &SelfKnowledge) -> ServiceInfo {
            let primary_endpoint = self_knowledge
                .endpoints
                .get("api")
                .map(|addr| addr.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let health_endpoint = self_knowledge
                .endpoints
                .get("health")
                .map(|addr| addr.to_string());

            let mut metadata = HashMap::new();
            metadata.insert("version".to_string(), self_knowledge.version.clone());
            metadata.insert(
                "endpoints".to_string(),
                format!("{:?}", self_knowledge.endpoints),
            );

            ServiceInfo {
                id: self_knowledge.id.as_str().to_string(),
                name: self_knowledge.name.clone(),
                capabilities: self_knowledge.capabilities.clone(),
                endpoint: primary_endpoint,
                metadata,
                health_endpoint,
            }
        }
    }

    #[async_trait::async_trait]
    impl DiscoveryMechanism for MdnsDiscovery {
        async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()> {
            tracing::info!(
                "mDNS announce: {} with capabilities: {:?}",
                self_knowledge.name,
                self_knowledge.capabilities
            );

            let service_info = Self::create_service_info(self_knowledge);
            let service_id = service_info.id.clone();

            // Store in registry
            let mut registry = self.registry.write().await;
            registry.insert(service_id.clone(), service_info);

            // Remember our service ID
            let mut announced = self.announced_service_id.write().await;
            *announced = Some(service_id);

            tracing::info!("Successfully announced to mDNS registry");
            Ok(())
        }

        async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
            tracing::debug!("mDNS query for capability: {:?}", capability);

            let registry = self.registry.read().await;
            let matching_services: Vec<ServiceInfo> = registry
                .values()
                .filter(|service| service.capabilities.contains(&capability))
                .cloned()
                .collect();

            tracing::debug!(
                "Found {} services with capability '{}'",
                matching_services.len(),
                capability
            );

            Ok(matching_services)
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
            tracing::debug!("mDNS lookup service: {}", id);

            let registry = self.registry.read().await;
            let service = registry.get(id).cloned();

            if service.is_some() {
                tracing::debug!("Found service: {}", id);
            } else {
                tracing::debug!("Service not found: {}", id);
            }

            Ok(service)
        }

        async fn health_check(&self, service_id: &str) -> Result<bool> {
            tracing::debug!("mDNS health check: {}", service_id);

            // Check if service exists in registry
            let registry = self.registry.read().await;
            let healthy = registry.contains_key(service_id);

            tracing::debug!("Service {} health: {}", service_id, healthy);
            Ok(healthy)
        }

        async fn deregister(&self, service_id: &str) -> Result<()> {
            tracing::info!("mDNS deregister: {}", service_id);

            let mut registry = self.registry.write().await;
            registry.remove(service_id);

            // Clear announced service if it was us
            let mut announced = self.announced_service_id.write().await;
            if announced.as_ref().map(|id| id.as_str()) == Some(service_id) {
                *announced = None;
            }

            tracing::info!("Successfully deregistered from mDNS registry");
            Ok(())
        }

        fn mechanism_name(&self) -> &'static str {
            "mdns"
        }
    }
}

/// Consul-based discovery (cloud/datacenter)
#[cfg(feature = "consul")]
pub mod consul {
    use super::*;

    /// Consul discovery mechanism
    pub struct ConsulDiscovery {
        timeout: Duration,
        cache_duration: Duration,
        consul_addr: String,
        // TODO: Add actual Consul client
    }

    impl ConsulDiscovery {
        /// Create new Consul discovery
        pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
            let consul_addr = std::env::var("CONSUL_HTTP_ADDR")
                .unwrap_or_else(|_| "http://localhost:8500".to_string());

            Ok(Self {
                timeout: builder.timeout,
                cache_duration: builder.cache_duration,
                consul_addr,
            })
        }
    }

    #[async_trait::async_trait]
    impl DiscoveryMechanism for ConsulDiscovery {
        async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()> {
            tracing::info!("Consul announce: {}", self_knowledge.name);
            // TODO: Implement actual Consul registration
            Ok(())
        }

        async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
            tracing::debug!("Consul query for capability: {:?}", capability);
            // TODO: Implement actual Consul query
            Ok(vec![])
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
            tracing::debug!("Consul lookup service: {}", id);
            // TODO: Implement actual Consul lookup
            Ok(None)
        }

        async fn health_check(&self, service_id: &str) -> Result<bool> {
            tracing::debug!("Consul health check: {}", service_id);
            // TODO: Implement actual health check
            Ok(true)
        }

        async fn deregister(&self, service_id: &str) -> Result<()> {
            tracing::info!("Consul deregister: {}", service_id);
            // TODO: Implement actual deregistration
            Ok(())
        }

        fn mechanism_name(&self) -> &'static str {
            "consul"
        }
    }
}

/// Kubernetes-based discovery (orchestrated)
#[cfg(feature = "kubernetes")]
pub mod k8s {
    use super::*;

    /// Kubernetes discovery mechanism
    pub struct KubernetesDiscovery {
        timeout: Duration,
        cache_duration: Duration,
        namespace: String,
        // TODO: Add actual k8s client
    }

    impl KubernetesDiscovery {
        /// Create new Kubernetes discovery
        pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
            let namespace = std::env::var("NAMESPACE")
                .or_else(|_| std::env::var("POD_NAMESPACE"))
                .unwrap_or_else(|_| "default".to_string());

            Ok(Self {
                timeout: builder.timeout,
                cache_duration: builder.cache_duration,
                namespace,
            })
        }
    }

    #[async_trait::async_trait]
    impl DiscoveryMechanism for KubernetesDiscovery {
        async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()> {
            tracing::info!("k8s announce: {}", self_knowledge.name);
            // TODO: Implement actual k8s service/endpoint registration
            Ok(())
        }

        async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
            tracing::debug!("k8s query for capability: {:?}", capability);
            // TODO: Implement actual k8s service discovery by label
            Ok(vec![])
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
            tracing::debug!("k8s lookup service: {}", id);
            // TODO: Implement actual k8s service lookup
            Ok(None)
        }

        async fn health_check(&self, service_id: &str) -> Result<bool> {
            tracing::debug!("k8s health check: {}", service_id);
            // TODO: Implement actual health check via k8s endpoint
            Ok(true)
        }

        async fn deregister(&self, service_id: &str) -> Result<()> {
            tracing::info!("k8s deregister: {}", service_id);
            // TODO: Implement actual deregistration
            Ok(())
        }

        fn mechanism_name(&self) -> &'static str {
            "kubernetes"
        }
    }
}

/// Testing utilities
pub mod testing;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mdns_discovery_creation() {
        let discovery = DiscoveryBuilder::new()
            .with_timeout(Duration::from_secs(10))
            .build_mdns()
            .await;

        assert!(discovery.is_ok());
        let discovery = discovery.unwrap();
        assert_eq!(discovery.mechanism_name(), "mdns");
    }

    #[tokio::test]
    async fn test_auto_detect_defaults_to_mdns() {
        // When no discovery env vars set, should default to mDNS
        let discovery = DiscoveryBuilder::new().detect().await;

        assert!(discovery.is_ok());
        let discovery = discovery.unwrap();
        assert_eq!(discovery.mechanism_name(), "mdns");
    }

    #[tokio::test]
    async fn test_mdns_announce_and_find() {
        let discovery = DiscoveryBuilder::new().build_mdns().await.unwrap();

        // Create and announce a service
        let self_knowledge = SelfKnowledge::builder()
            .with_id("test-storage")
            .with_name("Test Storage")
            .with_version("1.0.0")
            .with_capability("storage")
            .with_capability("zfs")
            .with_endpoint("api", "0.0.0.0:8080".parse().unwrap())
            .build()
            .unwrap();

        discovery.announce(&self_knowledge).await.unwrap();

        // Should be able to find by capability
        let storage_services = discovery
            .find_by_capability("storage".to_string())
            .await
            .unwrap();

        assert_eq!(storage_services.len(), 1);
        assert_eq!(storage_services[0].id, "test-storage");
        assert_eq!(storage_services[0].name, "Test Storage");
        assert!(storage_services[0]
            .capabilities
            .contains(&"storage".to_string()));
    }
}
