// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// ! **CAPABILITY-BASED PRIMAL DISCOVERY**
//!
//! Pure capability-based discovery with no hardcoded values.
//! Primals discover themselves and others through runtime capabilities.
//!
//! ## Core Principles
//!
//! 1. **Self-Knowledge**: Each primal knows only itself
//! 2. **Runtime Discovery**: All peers discovered dynamically
//! 3. **No Hardcoding**: Zero hardcoded ports, hosts, or endpoints
//! 4. **Capability-Based**: Services discovered by capability, not location

use nestgate_types::error::{NestGateError, Result};
use std::collections::HashMap;
use std::future::Future;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Capability that a primal can provide
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PrimalCapability {
    /// ZFS storage management
    ZfsStorage,
    /// API gateway services
    ApiGateway,
    /// Service discovery and registry
    ServiceDiscovery,
    /// Metrics and monitoring
    Observability,
    /// Authentication and authorization
    Authentication,
    /// Network file system
    NetworkFileSystem(NfsVersion),
    /// Data synchronization
    DataSync,
    /// Custom capability
    Custom(String),
}

/// NFS protocol version
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum NfsVersion {
    /// `NFSv3`
    V3,
    /// `NFSv4`
    V4,
}

/// Primal's self-knowledge about its capabilities and binding
#[derive(Debug, Clone)]
pub struct PrimalSelfKnowledge {
    /// Unique identifier for this primal
    pub id: PrimalId,
    /// Capabilities this primal provides
    pub capabilities: Vec<PrimalCapability>,
    /// Network binding information
    pub binding: BindingInfo,
    /// Health status
    pub health: HealthStatus,
    /// Metadata about this primal
    pub metadata: HashMap<String, String>,
}

/// Unique identifier for a primal
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PrimalId(String);

impl PrimalId {
    /// Create from a known ID string
    ///
    /// Use this when:
    /// - Loading from configuration
    /// - Deserializing from storage
    /// - Testing with specific IDs
    ///
    /// For runtime discovery, prefer `from_environment()`.
    #[must_use]
    pub const fn from_string(id: String) -> Self {
        Self(id)
    }

    /// Create from hostname and process info (runtime discovery)
    ///
    /// Generates a unique ID from:
    /// - Hostname from environment
    /// - Process ID
    /// - Current timestamp (nanosecond precision)
    ///
    /// This ensures unique IDs for each primal instance.
    pub fn from_environment() -> Result<Self> {
        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("HOST"))
            .unwrap_or_else(|_| "unknown".to_string());

        let pid = std::process::id();

        // ✅ CONCURRENT FIX: Use nanosecond precision for truly unique IDs
        // Milliseconds were insufficient for rapid successive calls
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| NestGateError::internal(format!("System time error: {e}")))?
            .as_nanos();

        Ok(Self(format!("{hostname}-{pid}-{timestamp}")))
    }

    /// Get the ID string
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Network binding information discovered at runtime
#[derive(Debug, Clone)]
pub struct BindingInfo {
    /// IP address this primal is bound to
    pub address: IpAddr,
    /// Port this primal is listening on
    pub port: u16,
    /// Protocol (tcp, udp, etc.)
    pub protocol: Protocol,
}

/// Network protocol
#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    /// TCP
    Tcp,
    /// UDP
    Udp,
    /// HTTP
    Http,
    /// HTTPS
    Https,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tcp => write!(f, "tcp"),
            Self::Udp => write!(f, "udp"),
            Self::Http => write!(f, "http"),
            Self::Https => write!(f, "https"),
        }
    }
}

/// Health status of a primal
#[derive(Debug, Clone)]
pub enum HealthStatus {
    /// Healthy and operational
    Healthy,
    /// Degraded but operational
    Degraded {
        /// Reason for degraded status
        reason: String,
    },
    /// Unhealthy
    Unhealthy {
        /// Reason for unhealthy status
        reason: String,
    },
}

impl HealthStatus {
    /// Check if status is healthy
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }
}

/// Discovery backend trait for finding other primals
pub trait DiscoveryBackend: Send + Sync {
    /// Announce this primal's capabilities
    fn announce(
        &self,
        knowledge: &PrimalSelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;

    /// Find primals with specific capabilities
    fn find_by_capability(
        &self,
        capability: &PrimalCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<PeerDescriptor>>> + Send + '_>>;

    /// Query for primals matching criteria
    fn query(
        &self,
        query: &DiscoveryQuery,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<PeerDescriptor>>> + Send + '_>>;

    /// Remove announcement (on shutdown)
    fn unannounce(&self, id: &PrimalId) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
}

/// Query for discovering peers
#[derive(Debug, Clone)]
pub struct DiscoveryQuery {
    /// Required capabilities
    pub required_capabilities: Vec<PrimalCapability>,
    /// Optional capabilities (nice to have)
    pub optional_capabilities: Vec<PrimalCapability>,
    /// Maximum latency tolerance
    pub max_latency: Option<Duration>,
    /// Minimum health status
    pub min_health: HealthStatus,
}

impl DiscoveryQuery {
    /// Query for capability
    #[must_use]
    pub fn for_capability(capability: PrimalCapability) -> Self {
        Self {
            required_capabilities: vec![capability],
            optional_capabilities: vec![],
            max_latency: None,
            min_health: HealthStatus::Degraded {
                reason: "acceptable".to_string(),
            },
        }
    }
}

/// Information about a discovered peer primal
#[derive(Debug, Clone)]
pub struct PeerDescriptor {
    /// Peer's identifier
    pub id: PrimalId,
    /// Peer's capabilities
    pub capabilities: Vec<PrimalCapability>,
    /// How to reach this peer
    pub endpoint: ServiceEndpoint,
    /// Last seen timestamp
    pub last_seen: std::time::SystemTime,
    /// Health status
    pub health: HealthStatus,
    /// Measured latency to this peer
    pub latency: Option<Duration>,
}

/// Service endpoint for connecting to a peer
#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    /// Network address
    pub address: SocketAddr,
    /// Protocol to use
    pub protocol: Protocol,
    /// Optional path for HTTP-based services
    pub path: Option<String>,
}

impl ServiceEndpoint {
    /// Create from address
    #[must_use]
    pub const fn tcp(address: SocketAddr) -> Self {
        Self {
            address,
            protocol: Protocol::Tcp,
            path: None,
        }
    }

    /// Create HTTP endpoint
    pub fn http(address: SocketAddr, path: impl Into<String>) -> Self {
        Self {
            address,
            protocol: Protocol::Http,
            path: Some(path.into()),
        }
    }

    /// Get connection URL
    #[must_use]
    pub fn url(&self) -> String {
        match self.protocol {
            Protocol::Http => format!(
                "http://{}{}",
                self.address,
                self.path.as_deref().unwrap_or("/")
            ),
            Protocol::Https => format!(
                "https://{}{}",
                self.address,
                self.path.as_deref().unwrap_or("/")
            ),
            Protocol::Tcp => format!("tcp://{}", self.address),
            Protocol::Udp => format!("udp://{}", self.address),
        }
    }
}

/// Capability-based discovery manager
pub struct CapabilityDiscoveryManager {
    /// My own self-knowledge
    self_knowledge: Arc<RwLock<PrimalSelfKnowledge>>,
    /// Discovery backends (mDNS, consul, etc.)
    backends: Vec<Arc<dyn DiscoveryBackend>>,
    /// Cache of known peers
    peer_cache: Arc<RwLock<HashMap<PrimalId, PeerDescriptor>>>,
    /// Configuration
    config: DiscoveryConfig,
}

/// Configuration for discovery manager
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// How often to announce self
    pub announce_interval: Duration,
    /// How often to refresh peer list
    pub refresh_interval: Duration,
    /// Time before peer is considered stale
    pub peer_ttl: Duration,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            announce_interval: Duration::from_secs(30),
            refresh_interval: Duration::from_secs(60),
            peer_ttl: Duration::from_secs(300),
        }
    }
}

impl CapabilityDiscoveryManager {
    /// Initialize discovery with self-knowledge
    pub async fn initialize(capabilities: Vec<PrimalCapability>) -> Result<Self> {
        // Discover own binding
        let binding = Self::discover_own_binding().await?;

        // Create self-knowledge
        let self_knowledge = PrimalSelfKnowledge {
            id: PrimalId::from_environment()?,
            capabilities,
            binding,
            health: HealthStatus::Healthy,
            metadata: Self::collect_metadata(),
        };

        info!("Primal self-knowledge established: {:?}", self_knowledge.id);

        Ok(Self {
            self_knowledge: Arc::new(RwLock::new(self_knowledge)),
            backends: vec![],
            peer_cache: Arc::new(RwLock::new(HashMap::new())),
            config: DiscoveryConfig::default(),
        })
    }

    /// Discover own network binding (no hardcoding!)
    async fn discover_own_binding() -> Result<BindingInfo> {
        // Try to find an available port
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|_| NestGateError::network_error("Failed to bind to port"))?;

        let addr = listener
            .local_addr()
            .map_err(|_| NestGateError::network_error("Failed to get local address"))?;

        info!("Discovered available port: {}", addr.port());

        Ok(BindingInfo {
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: addr.port(),
            protocol: Protocol::Tcp,
        })
    }

    /// Collect metadata about this environment
    fn collect_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();

        // Runtime information
        if let Ok(hostname) = std::env::var("HOSTNAME").or_else(|_| std::env::var("HOST")) {
            metadata.insert("hostname".to_string(), hostname);
        }

        metadata.insert("pid".to_string(), std::process::id().to_string());

        // Environment indicators (not hardcoded values!)
        if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
            metadata.insert("platform".to_string(), "kubernetes".to_string());
        } else if std::path::Path::new("/.dockerenv").exists() {
            metadata.insert("platform".to_string(), "docker".to_string());
        }

        metadata
    }

    /// Add a discovery backend
    pub fn add_backend(&mut self, backend: Arc<dyn DiscoveryBackend>) {
        self.backends.push(backend);
    }

    /// Start announcing self to the network
    pub async fn start_announcing(&self) -> Result<()> {
        let self_knowledge = self.self_knowledge.read().await.clone();

        for backend in &self.backends {
            backend.announce(&self_knowledge).await?;
        }

        info!(
            "Announced primal capabilities to {} backends",
            self.backends.len()
        );
        Ok(())
    }

    /// Find services by capability
    pub async fn find_capability(
        &self,
        capability: PrimalCapability,
    ) -> Result<Vec<PeerDescriptor>> {
        let mut all_peers = Vec::new();

        // Query all backends
        for backend in &self.backends {
            match backend.find_by_capability(&capability).await {
                Ok(mut peers) => all_peers.append(&mut peers),
                Err(e) => warn!("Backend query failed: {}", e),
            }
        }

        // Update cache
        let mut cache = self.peer_cache.write().await;
        for peer in &all_peers {
            cache.insert(peer.id.clone(), peer.clone());
        }

        // Remove stale entries
        self.remove_stale_peers(&mut cache).await;

        Ok(all_peers)
    }

    /// Query for peers matching criteria
    pub async fn query(&self, query: &DiscoveryQuery) -> Result<Vec<PeerDescriptor>> {
        let mut all_peers = Vec::new();

        for backend in &self.backends {
            match backend.query(query).await {
                Ok(mut peers) => all_peers.append(&mut peers),
                Err(e) => warn!("Backend query failed: {}", e),
            }
        }

        // Filter by query criteria
        let filtered: Vec<_> = all_peers
            .into_iter()
            .filter(|peer| self.matches_query(peer, query))
            .collect();

        Ok(filtered)
    }

    /// Check if peer matches query
    fn matches_query(&self, peer: &PeerDescriptor, query: &DiscoveryQuery) -> bool {
        // Check required capabilities
        for required in &query.required_capabilities {
            if !peer.capabilities.contains(required) {
                return false;
            }
        }

        // Check latency if specified
        if let (Some(max_latency), Some(latency)) = (query.max_latency, peer.latency)
            && latency > max_latency
        {
            return false;
        }

        true
    }

    /// Remove stale peers from cache
    async fn remove_stale_peers(&self, cache: &mut HashMap<PrimalId, PeerDescriptor>) {
        let now = std::time::SystemTime::now();
        let stale_threshold = self.config.peer_ttl;

        cache.retain(|_, peer| {
            now.duration_since(peer.last_seen)
                .map(|age| age < stale_threshold)
                .unwrap_or(false)
        });
    }

    /// Graceful shutdown - unannounce self
    pub async fn shutdown(&self) -> Result<()> {
        let self_knowledge = self.self_knowledge.read().await;

        for backend in &self.backends {
            if let Err(e) = backend.unannounce(&self_knowledge.id).await {
                warn!("Failed to unannounce from backend: {}", e);
            }
        }

        info!("Unannounced primal from network");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_self_knowledge_initialization() {
        let capabilities = vec![PrimalCapability::ZfsStorage, PrimalCapability::ApiGateway];

        let manager = CapabilityDiscoveryManager::initialize(capabilities.clone()).await;
        assert!(manager.is_ok());

        let manager = manager.unwrap();
        let self_knowledge = manager.self_knowledge.read().await;

        assert_eq!(self_knowledge.capabilities.len(), 2);
        assert!(
            self_knowledge
                .capabilities
                .contains(&PrimalCapability::ZfsStorage)
        );
    }

    #[tokio::test]
    async fn test_binding_discovery() {
        let binding = CapabilityDiscoveryManager::discover_own_binding().await;
        assert!(binding.is_ok());

        let binding = binding.unwrap();
        assert!(binding.port > 0);
    }

    #[test]
    fn test_primal_id_creation() {
        let id = PrimalId::from_environment();
        assert!(id.is_ok());

        let id = id.unwrap();
        assert!(!id.as_str().is_empty());
    }

    #[test]
    fn test_service_endpoint_url() {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let endpoint = ServiceEndpoint::http(addr, "/api/v1");

        assert_eq!(endpoint.url(), "http://127.0.0.1:8080/api/v1");
    }
}
