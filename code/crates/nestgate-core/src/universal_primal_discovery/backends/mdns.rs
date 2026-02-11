//! mDNS discovery backend for local network discovery
//!
//! **EVOLVED FROM STUBS** - Complete mDNS implementation using mdns-sd crate.
//!
//! This backend uses multicast DNS (mDNS) to discover services on the local network.
//! It's perfect for local development, home networks, and edge deployments.
//!
//! ## Architecture
//!
//! **SAFETY**: This backend uses a try-then-fallback pattern:
//! 1. Attempt real mDNS operations via mdns-sd `ServiceDaemon`
//! 2. If daemon creation or mDNS ops fail (e.g. no multicast, port in use),
//!    gracefully fall back to cache-only mode
//! 3. Cache is always updated (local announcements, discovered peers) so the
//!    system works even when mDNS is unavailable (containers, restricted networks)
//!
//! **ARCHITECTURE**: The `ServiceDaemon` is stored in the struct and created
//! lazily on first use. This avoids fallible constructors while enabling
//! graceful degradation when mDNS is unavailable.
//!
//! ## Features
//! - ✅ **Zero-configuration discovery** - No manual configuration needed
//! - ✅ **Automatic service announcement** - Self-announce on startup
//! - ✅ **Real-time discovery** - Discover new services as they appear
//! - ✅ **Local network only** - No internet required, privacy-preserving
//! - ✅ **Low latency** - Multicast for fast peer discovery
//! - ✅ **Capability-based** - Discover by what services can do, not who they are
//!
//! ## Use Cases
//! - Local development environments (auto-discover local services)
//! - Home lab deployments (zero-config multi-node)
//! - Edge computing scenarios (discover edge nodes)
//! - IoT device discovery (find devices on local network)
//!
//! ## Protocol
//! - Service type: `_nestgate._tcp.local.`
//! - TXT records encode capabilities, metadata
//! - Multiple service instances per capability
//! - Automatic TTL management

use crate::universal_primal_discovery::capability_based_discovery::{
    DiscoveryBackend, DiscoveryQuery, HealthStatus, PeerDescriptor, PrimalCapability, PrimalId,
    PrimalSelfKnowledge, Protocol, ServiceEndpoint,
};
use crate::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// mDNS discovery backend
///
/// **COMPLETE IMPLEMENTATION** - Uses real mDNS protocol via mdns-sd crate.
///
/// Uses multicast DNS for zero-configuration service discovery on local networks.
/// Primals announce themselves via mDNS and discover peers through mDNS queries.
///
/// ## Internal State
/// - Service daemon (mdns-sd): Handles mDNS protocol, created lazily
/// - Service name: Unique identifier for this instance
/// - Peer cache: Recently discovered peers with TTL
/// - Announced services: Track what we've announced for cleanup
pub struct MdnsDiscoveryBackend {
    /// Service name prefix for mDNS (unique per instance).
    /// Used to construct per-capability service names for mDNS announcement and discovery.
    service_name: String,
    /// Local cache of discovered peers
    peer_cache: Arc<RwLock<HashMap<PrimalId, CachedPeer>>>,
    /// Configuration
    config: MdnsConfig,
    /// Track announced service fullnames for cleanup (unregister)
    announced_services: Arc<RwLock<Vec<String>>>,
    /// mDNS daemon - None when creation failed (cache-only fallback)
    daemon: Arc<RwLock<Option<ServiceDaemon>>>,
}

/// Configuration for mDNS backend
#[derive(Debug, Clone)]
pub struct MdnsConfig {
    /// Service type for mDNS (e.g., "_nestgate._tcp")
    pub service_type: String,
    /// Domain for mDNS (typically "local")
    pub domain: String,
    /// How long to wait for mDNS responses
    pub query_timeout: Duration,
    /// How long to cache peers
    pub cache_ttl: Duration,
}

impl Default for MdnsConfig {
    fn default() -> Self {
        Self {
            service_type: "_nestgate._tcp".to_string(),
            domain: "local".to_string(),
            query_timeout: Duration::from_secs(5),
            cache_ttl: Duration::from_secs(300),
        }
    }
}

/// Cached peer information
#[derive(Debug, Clone)]
struct CachedPeer {
    /// Peer descriptor
    descriptor: PeerDescriptor,
    /// When this peer was last seen
    last_seen: SystemTime,
}

impl MdnsDiscoveryBackend {
    /// Create a new mDNS backend with default configuration
    pub fn new() -> Self {
        Self::with_config(MdnsConfig::default())
    }

    /// Create a new mDNS backend with custom configuration
    pub fn with_config(config: MdnsConfig) -> Self {
        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("HOST"))
            .unwrap_or_else(|_| "unknown".to_string());

        // Generate unique service name with random suffix for multiple instances
        // ✅ EVOLVED: unwrap() → unwrap_or_default() for safety
        let unique_id = uuid::Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap_or_default()
            .to_string();

        Self {
            service_name: format!("nestgate-{}-{}", hostname, unique_id),
            peer_cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            announced_services: Arc::new(RwLock::new(Vec::new())),
            daemon: Arc::new(RwLock::new(None)),
        }
    }

    /// Ensure daemon exists; create lazily on first use.
    /// Returns Some(daemon) when real mDNS is available, None for cache-only fallback.
    ///
    /// **ARCHITECTURE**: Lazy creation avoids fallible constructors. If
    /// ServiceDaemon::new() fails (multicast unavailable, port 5353 in use),
    /// we return None and the caller uses cache-only path.
    fn ensure_daemon(&self) -> Option<mdns_sd::ServiceDaemon> {
        let mut daemon_guard = self.daemon.try_write().ok()?;
        if daemon_guard.is_some() {
            return daemon_guard.clone();
        }
        match ServiceDaemon::new() {
            Ok(daemon) => {
                let cloned = daemon.clone();
                *daemon_guard = Some(daemon);
                Some(cloned)
            }
            Err(e) => {
                warn!(
                    "mDNS: Daemon creation failed ({}), using cache-only mode",
                    e
                );
                None
            }
        }
    }

    /// Build full mDNS service type with domain (e.g. "_nestgate._tcp.local.")
    fn full_service_type(&self) -> String {
        format!("{}.{}.", self.config.service_type, self.config.domain)
    }

    /// Build hostname for ServiceInfo (host in DNS context).
    /// Uses binding address so resolvers get correct IP.
    fn hostname_from_binding(address: IpAddr) -> String {
        format!("{}.local.", address)
    }

    /// Convert binding info to TXT properties for mdns-sd ServiceInfo.
    /// Returns key-value pairs; mdns-sd accepts &[(K, V)] via IntoTxtProperties.
    fn binding_to_txt_properties(&self, knowledge: &PrimalSelfKnowledge) -> Vec<(String, String)> {
        let mut props = vec![
            ("id".to_string(), knowledge.id.as_str().to_string()),
            ("port".to_string(), knowledge.binding.port.to_string()),
            (
                "protocol".to_string(),
                format!("{:?}", knowledge.binding.protocol),
            ),
        ];

        // Add capabilities as comma-separated list for single "cap" key
        let caps: Vec<String> = knowledge
            .capabilities
            .iter()
            .map(|c| format!("{:?}", c))
            .collect();
        props.push(("cap".to_string(), caps.join(",")));

        // Health
        match &knowledge.health {
            HealthStatus::Healthy => {
                props.push(("health".to_string(), "healthy".to_string()));
            }
            HealthStatus::Degraded { reason } => {
                props.push(("health".to_string(), format!("degraded:{}", reason)));
            }
            HealthStatus::Unhealthy { reason } => {
                props.push(("health".to_string(), format!("unhealthy:{}", reason)));
            }
        }

        props
    }

    /// Announce using real mDNS protocol
    ///
    /// **COMPLETE IMPLEMENTATION** - Uses mdns-sd ServiceDaemon to register.
    ///
    /// 1. Always update local cache (fallback for when mDNS unavailable)
    /// 2. Try real mDNS registration if daemon available
    /// 3. Track announced fullnames for unannounce
    async fn announce_real(&self, knowledge: &PrimalSelfKnowledge) -> Result<()> {
        info!(
            "mDNS: Announcing primal '{}' with {} capabilities",
            knowledge.id.as_str(),
            knowledge.capabilities.len()
        );

        // ARCHITECTURE: Always update cache first - cache-only fallback path
        let socket_addr = SocketAddr::new(knowledge.binding.address, knowledge.binding.port);
        let endpoint = ServiceEndpoint {
            address: socket_addr,
            protocol: knowledge.binding.protocol,
            path: None,
        };

        let descriptor = PeerDescriptor {
            id: knowledge.id.clone(),
            capabilities: knowledge.capabilities.clone(),
            endpoint,
            last_seen: SystemTime::now(),
            health: HealthStatus::Healthy,
            latency: None,
        };

        let cached = CachedPeer {
            descriptor,
            last_seen: SystemTime::now(),
        };

        {
            let mut cache = self.peer_cache.write().await;
            cache.insert(knowledge.id.clone(), cached);
        }

        // Try real mDNS registration
        if let Some(daemon) = self.ensure_daemon() {
            let ty_domain = self.full_service_type();
            let host_name = Self::hostname_from_binding(knowledge.binding.address);
            let ip_str = knowledge.binding.address.to_string();
            let port = knowledge.binding.port;
            let props = self.binding_to_txt_properties(knowledge);

            // ServiceInfo::new accepts IntoTxtProperties - &[(String, String)] works
            let props_ref: Vec<(&str, &str)> = props
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect();

            match ServiceInfo::new(
                &ty_domain,
                &self.service_name,
                &host_name,
                ip_str.as_str(),
                port,
                props_ref.as_slice(),
            ) {
                Ok(service_info) => {
                    if let Err(e) = daemon.register(service_info.clone()) {
                        warn!(
                            "mDNS: Registration failed ({}), using cache-only for announce",
                            e
                        );
                    } else {
                        let fullname = service_info.get_fullname().to_string();
                        let mut announced = self.announced_services.write().await;
                        if !announced.contains(&fullname) {
                            announced.push(fullname);
                        }
                        debug!(
                            "mDNS: Registered service '{}' on network",
                            service_info.get_fullname()
                        );
                    }
                }
                Err(e) => {
                    warn!(
                        "mDNS: ServiceInfo creation failed ({}), using cache-only for announce",
                        e
                    );
                }
            }
        }

        debug!(
            "mDNS: Successfully announced {} capabilities for service '{}'",
            knowledge.capabilities.len(),
            self.service_name,
        );

        Ok(())
    }

    /// Discover peers via real mDNS browse, merge with cache.
    /// Returns peers from network + cache that match the capability.
    async fn discover_real(&self, capability: &PrimalCapability) -> Result<Vec<PeerDescriptor>> {
        let mut peers: Vec<PeerDescriptor> = Vec::new();

        // 1. Get cached peers (always available)
        {
            let cache = self.peer_cache.read().await;
            for p in cache
                .values()
                .filter(|p| p.descriptor.capabilities.contains(capability))
            {
                peers.push(p.descriptor.clone());
            }
        }

        // 2. Try real mDNS browse
        if let Some(daemon) = self.ensure_daemon() {
            let service_type = self.full_service_type();
            match daemon.browse(&service_type) {
                Ok(receiver) => {
                    let timeout = self.config.query_timeout;
                    match tokio::time::timeout(
                        timeout,
                        Self::collect_resolved_peers(receiver, capability),
                    )
                    .await
                    {
                        Ok(discovered) => {
                            for descriptor in discovered {
                                // Merge into cache
                                let cached = CachedPeer {
                                    descriptor: descriptor.clone(),
                                    last_seen: SystemTime::now(),
                                };
                                {
                                    let mut cache = self.peer_cache.write().await;
                                    cache.insert(descriptor.id.clone(), cached);
                                }
                                if descriptor.capabilities.contains(capability)
                                    && !peers.iter().any(|p| p.id == descriptor.id)
                                {
                                    peers.push(descriptor);
                                }
                            }
                        }
                        Err(_) => {
                            debug!("mDNS: Browse timed out after {:?}, using cache", timeout);
                        }
                    }
                }
                Err(e) => {
                    warn!("mDNS: Browse failed ({}), using cache", e);
                }
            }
        }

        Ok(peers)
    }

    /// Collect PeerDescriptors from ServiceEvent stream until timeout or channel close.
    /// On channel disconnect (RecvError), returns collected peers - no Err to avoid
    /// conflating "done" with "failure".
    async fn collect_resolved_peers(
        receiver: mdns_sd::Receiver<ServiceEvent>,
        _capability: &PrimalCapability,
    ) -> Vec<PeerDescriptor> {
        let mut peers = Vec::new();
        loop {
            match receiver.recv_async().await {
                Ok(ServiceEvent::ServiceResolved(resolved)) => {
                    if let Some(desc) = Self::descriptor_from_resolved(&resolved) {
                        peers.push(desc);
                    }
                }
                Ok(ServiceEvent::ServiceRemoved(_, _)) | Ok(ServiceEvent::SearchStopped(_)) => {
                    break;
                }
                Ok(_) => {}
                Err(_) => break, // Channel disconnected - return what we have
            }
        }
        peers
    }

    /// Build PeerDescriptor from ResolvedService.
    /// Returns None if TXT records don't match capability or parse fails.
    fn descriptor_from_resolved(resolved: &mdns_sd::ResolvedService) -> Option<PeerDescriptor> {
        if !resolved.is_valid() {
            return None;
        }
        let id_str = resolved
            .txt_properties
            .get_property_val_str("id")
            .map(|s| s.to_string())?;
        let id = PrimalId::from_string(id_str);

        let port = resolved.port;
        let addr = resolved.addresses.iter().next().and_then(|scoped| {
            let s = scoped.to_string();
            // IPv6 link-local may have scope: "fe80::1%eth0" -> strip after %
            let ip_str = s.split('%').next().unwrap_or(&s);
            ip_str.trim().parse::<IpAddr>().ok()
        })?;
        let socket_addr = SocketAddr::new(addr, port);

        let mut capabilities = Vec::new();
        if let Some(cap_str) = resolved.txt_properties.get_property_val_str("cap") {
            for part in cap_str.split(',') {
                let part = part.trim();
                if let Some(cap) = Self::parse_capability_str(part) {
                    capabilities.push(cap);
                }
            }
        }

        let health = resolved
            .txt_properties
            .get_property_val_str("health")
            .map(|s| {
                if s == "healthy" {
                    HealthStatus::Healthy
                } else if s.starts_with("degraded:") {
                    HealthStatus::Degraded {
                        reason: s.trim_start_matches("degraded:").to_string(),
                    }
                } else if s.starts_with("unhealthy:") {
                    HealthStatus::Unhealthy {
                        reason: s.trim_start_matches("unhealthy:").to_string(),
                    }
                } else {
                    HealthStatus::Healthy
                }
            })
            .unwrap_or(HealthStatus::Healthy);

        let protocol = resolved
            .txt_properties
            .get_property_val_str("protocol")
            .map(|s| {
                if s.contains("Http") {
                    Protocol::Http
                } else if s.contains("Https") {
                    Protocol::Https
                } else if s.contains("Udp") {
                    Protocol::Udp
                } else {
                    Protocol::Tcp
                }
            })
            .unwrap_or(Protocol::Tcp);

        Some(PeerDescriptor {
            id,
            capabilities,
            endpoint: ServiceEndpoint {
                address: socket_addr,
                protocol,
                path: None,
            },
            last_seen: SystemTime::now(),
            health,
            latency: None,
        })
    }

    /// Parse capability string from TXT record to PrimalCapability
    fn parse_capability_str(s: &str) -> Option<PrimalCapability> {
        let s = s.trim();
        Some(match s {
            "ZfsStorage" => PrimalCapability::ZfsStorage,
            "ApiGateway" => PrimalCapability::ApiGateway,
            "ServiceDiscovery" => PrimalCapability::ServiceDiscovery,
            "Observability" => PrimalCapability::Observability,
            "Authentication" => PrimalCapability::Authentication,
            "NetworkFileSystem(V3)" => PrimalCapability::NetworkFileSystem(
                crate::universal_primal_discovery::capability_based_discovery::NfsVersion::V3,
            ),
            "NetworkFileSystem(V4)" => PrimalCapability::NetworkFileSystem(
                crate::universal_primal_discovery::capability_based_discovery::NfsVersion::V4,
            ),
            "DataSync" => PrimalCapability::DataSync,
            _ if s.starts_with("Custom(") => {
                let inner = s.trim_start_matches("Custom(").trim_end_matches(')');
                PrimalCapability::Custom(inner.to_string())
            }
            _ => return None,
        })
    }

    /// Build mDNS service name from capability
    ///
    /// This constructs the full mDNS service name for a capability
    /// following mDNS conventions. Used for tracking announced services.
    #[allow(dead_code)]
    fn service_name_for_capability(&self, capability: &PrimalCapability) -> String {
        let cap_str = match capability {
            PrimalCapability::ZfsStorage => "zfs-storage",
            PrimalCapability::ApiGateway => "api-gateway",
            PrimalCapability::ServiceDiscovery => "service-discovery",
            PrimalCapability::Observability => "observability",
            PrimalCapability::Authentication => "authentication",
            PrimalCapability::NetworkFileSystem(_) => "nfs",
            PrimalCapability::DataSync => "data-sync",
            PrimalCapability::Custom(name) => name,
        };

        format!(
            "{}_{}.{}.{}",
            self.service_name, cap_str, self.config.service_type, self.config.domain
        )
    }

    /// Clean up stale cache entries
    async fn cleanup_stale(&self) {
        let mut cache = self.peer_cache.write().await;
        let now = SystemTime::now();
        let ttl = self.config.cache_ttl;

        let initial_count = cache.len();
        cache.retain(|_, peer| {
            now.duration_since(peer.last_seen)
                .map(|age| age < ttl)
                .unwrap_or(false)
        });

        let removed = initial_count - cache.len();
        if removed > 0 {
            debug!("Cleaned up {} stale mDNS peer entries", removed);
        }
    }

    /// Convert binding info to TXT records for mDNS (legacy format for tests)
    #[cfg(test)]
    #[allow(dead_code)]
    fn binding_to_txt_records(&self, knowledge: &PrimalSelfKnowledge) -> Vec<String> {
        self.binding_to_txt_properties(knowledge)
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect()
    }
}

impl Default for MdnsDiscoveryBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl DiscoveryBackend for MdnsDiscoveryBackend {
    async fn announce(&self, knowledge: &PrimalSelfKnowledge) -> Result<()> {
        info!(
            "mDNS: Announcing primal '{}' with {} capabilities",
            knowledge.id.as_str(),
            knowledge.capabilities.len()
        );
        self.announce_real(knowledge).await
    }

    async fn find_by_capability(
        &self,
        capability: &PrimalCapability,
    ) -> Result<Vec<PeerDescriptor>> {
        self.cleanup_stale().await;

        debug!("mDNS: Querying for capability {:?}", capability);

        let peers = self.discover_real(capability).await?;

        debug!("mDNS: Found {} peers with capability", peers.len());

        Ok(peers)
    }

    async fn query(&self, query: &DiscoveryQuery) -> Result<Vec<PeerDescriptor>> {
        self.cleanup_stale().await;

        debug!("mDNS: Processing discovery query");

        let mut all_peers: HashMap<PrimalId, PeerDescriptor> = HashMap::new();

        for capability in &query.required_capabilities {
            let peers = self.find_by_capability(capability).await?;

            for peer in peers {
                let has_all_required = query
                    .required_capabilities
                    .iter()
                    .all(|cap| peer.capabilities.contains(cap));

                if has_all_required {
                    all_peers.insert(peer.id.clone(), peer);
                }
            }
        }

        let results: Vec<PeerDescriptor> = all_peers.into_values().collect();

        debug!("mDNS: Query returned {} matching peers", results.len());

        Ok(results)
    }

    async fn unannounce(&self, id: &PrimalId) -> Result<()> {
        info!("mDNS: Unannouncing primal '{}'", id.as_str());

        let to_unregister: Vec<String> = {
            let mut announced = self.announced_services.write().await;
            std::mem::take(&mut *announced)
        };

        if let Some(daemon) = self.ensure_daemon() {
            for fullname in &to_unregister {
                match daemon.unregister(fullname) {
                    Ok(_rx) => {
                        debug!("mDNS: Unregistered service '{}'", fullname);
                    }
                    Err(e) => {
                        warn!("mDNS: Unregister failed for '{}': {}", fullname, e);
                    }
                }
            }
        }

        let mut cache = self.peer_cache.write().await;
        cache.remove(id);

        debug!("mDNS: Successfully unannounced primal '{}'", id.as_str());

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal_primal_discovery::capability_based_discovery::{BindingInfo, Protocol};
    use std::net::Ipv4Addr;

    fn create_test_knowledge() -> PrimalSelfKnowledge {
        PrimalSelfKnowledge {
            id: PrimalId::from_environment().unwrap(),
            capabilities: vec![PrimalCapability::ZfsStorage, PrimalCapability::ApiGateway],
            binding: BindingInfo {
                address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
                port: 8080,
                protocol: Protocol::Tcp,
            },
            health: HealthStatus::Healthy,
            metadata: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_mdns_backend_creation() {
        let backend = MdnsDiscoveryBackend::new();
        assert!(!backend.service_name.is_empty());
    }

    #[tokio::test]
    async fn test_service_name_generation() {
        let backend = MdnsDiscoveryBackend::new();
        let name = backend.service_name_for_capability(&PrimalCapability::ZfsStorage);
        assert!(name.contains("zfs-storage"));
        assert!(name.contains(&backend.config.service_type));
    }

    #[tokio::test]
    async fn test_txt_records_generation() {
        let backend = MdnsDiscoveryBackend::new();
        let knowledge = create_test_knowledge();
        let records = backend.binding_to_txt_records(&knowledge);

        assert!(!records.is_empty());
        assert!(records.iter().any(|r| r.starts_with("id=")));
        assert!(records.iter().any(|r| r.starts_with("port=")));
        assert!(records.iter().any(|r| r.starts_with("protocol=")));
        assert!(records.iter().any(|r| r.starts_with("health=")));
    }

    #[tokio::test]
    async fn test_announce() {
        let backend = MdnsDiscoveryBackend::new();
        let knowledge = create_test_knowledge();

        let result = backend.announce(&knowledge).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_by_capability_empty() {
        let backend = MdnsDiscoveryBackend::new();

        let peers = backend
            .find_by_capability(&PrimalCapability::ZfsStorage)
            .await
            .unwrap();

        // Without announcing, cache should be empty
        assert_eq!(peers.len(), 0);
    }

    #[tokio::test]
    async fn test_announce_then_find_self() {
        let backend = MdnsDiscoveryBackend::new();
        let knowledge = create_test_knowledge();

        backend.announce(&knowledge).await.unwrap();

        let peers = backend
            .find_by_capability(&PrimalCapability::ZfsStorage)
            .await
            .unwrap();

        // Should find self in cache
        assert!(!peers.is_empty());
        assert!(peers.iter().any(|p| p.id == knowledge.id));
    }
}
