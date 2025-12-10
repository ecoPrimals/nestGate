//! In-memory discovery backend for testing and development
//!
//! This backend stores all primal registrations in memory and is useful for:
//! - Unit testing
//! - Integration testing
//! - Local development
//! - Fallback when other backends are unavailable

use crate::universal_primal_discovery::capability_based_discovery::{
    DiscoveryBackend, DiscoveryQuery, PeerDescriptor, PrimalCapability, PrimalId,
    PrimalSelfKnowledge,
};
use crate::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// In-memory discovery backend
///
/// Stores all primal registrations in a HashMap. Thread-safe via RwLock.
/// This is the simplest backend and serves as reference implementation.
pub struct InMemoryDiscoveryBackend {
    /// Registered primals
    primals: Arc<RwLock<HashMap<PrimalId, RegisteredPrimal>>>,
    /// Configuration
    config: InMemoryConfig,
}

/// Configuration for in-memory backend
#[derive(Debug, Clone)]
pub struct InMemoryConfig {
    /// Maximum number of primals to store
    pub max_primals: usize,
    /// How long before a primal is considered stale
    pub stale_threshold: Duration,
}

impl Default for InMemoryConfig {
    fn default() -> Self {
        Self {
            max_primals: 1000,
            stale_threshold: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// A registered primal with metadata
#[derive(Debug, Clone)]
struct RegisteredPrimal {
    /// The primal's self-knowledge
    knowledge: PrimalSelfKnowledge,
    /// When this primal was registered
    registered_at: SystemTime,
    /// When this primal was last updated
    last_updated: SystemTime,
}

impl InMemoryDiscoveryBackend {
    /// Create a new in-memory backend with default configuration
    pub fn new() -> Self {
        Self::with_config(InMemoryConfig::default())
    }

    /// Create a new in-memory backend with custom configuration
    pub fn with_config(config: InMemoryConfig) -> Self {
        Self {
            primals: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Clean up stale registrations
    async fn cleanup_stale(&self) {
        let mut primals = self.primals.write().await;
        let now = SystemTime::now();
        let threshold = self.config.stale_threshold;

        let initial_count = primals.len();
        primals.retain(|_, registered| {
            now.duration_since(registered.last_updated)
                .map(|age| age < threshold)
                .unwrap_or(false)
        });

        let removed = initial_count - primals.len();
        if removed > 0 {
            debug!("Cleaned up {} stale primal registrations", removed);
        }
    }

    /// Check if we've reached capacity
    fn check_capacity(&self, primals: &HashMap<PrimalId, RegisteredPrimal>) -> Result<()> {
        if primals.len() >= self.config.max_primals {
            return Err(NestGateError::internal(format!(
                "Maximum primal capacity reached: {}",
                self.config.max_primals
            )));
        }
        Ok(())
    }
}

impl Default for InMemoryDiscoveryBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl DiscoveryBackend for InMemoryDiscoveryBackend {
    async fn announce(&self, knowledge: &PrimalSelfKnowledge) -> Result<()> {
        // Clean up stale entries first
        self.cleanup_stale().await;

        let mut primals = self.primals.write().await;

        // Check capacity before adding
        if !primals.contains_key(&knowledge.id) {
            self.check_capacity(&primals)?;
        }

        let now = SystemTime::now();
        let registered = RegisteredPrimal {
            knowledge: knowledge.clone(),
            registered_at: primals
                .get(&knowledge.id)
                .map(|p| p.registered_at)
                .unwrap_or(now),
            last_updated: now,
        };

        primals.insert(knowledge.id.clone(), registered);

        info!(
            "Registered primal '{}' with {} capabilities",
            knowledge.id.as_str(),
            knowledge.capabilities.len()
        );

        Ok(())
    }

    async fn find_by_capability(
        &self,
        capability: &PrimalCapability,
    ) -> Result<Vec<PeerDescriptor>> {
        self.cleanup_stale().await;

        let primals = self.primals.read().await;

        let peers: Vec<PeerDescriptor> = primals
            .values()
            .filter(|p| p.knowledge.capabilities.contains(capability))
            .map(|p| {
                let binding = &p.knowledge.binding;
                PeerDescriptor {
                    id: p.knowledge.id.clone(),
                    capabilities: p.knowledge.capabilities.clone(),
                    endpoint: crate::universal_primal_discovery::capability_based_discovery::ServiceEndpoint::tcp(
                        std::net::SocketAddr::new(binding.address, binding.port),
                    ),
                    last_seen: p.last_updated,
                    health: p.knowledge.health.clone(),
                    latency: Some(Duration::from_millis(1)), // Local, so minimal latency
                }
            })
            .collect();

        debug!(
            "Found {} primals with capability {:?}",
            peers.len(),
            capability
        );

        Ok(peers)
    }

    async fn query(&self, query: &DiscoveryQuery) -> Result<Vec<PeerDescriptor>> {
        self.cleanup_stale().await;

        let primals = self.primals.read().await;

        let peers: Vec<PeerDescriptor> = primals
            .values()
            .filter(|p| {
                // Check required capabilities
                query
                    .required_capabilities
                    .iter()
                    .all(|cap| p.knowledge.capabilities.contains(cap))
            })
            .map(|p| {
                let binding = &p.knowledge.binding;
                PeerDescriptor {
                    id: p.knowledge.id.clone(),
                    capabilities: p.knowledge.capabilities.clone(),
                    endpoint: crate::universal_primal_discovery::capability_based_discovery::ServiceEndpoint::tcp(
                        std::net::SocketAddr::new(binding.address, binding.port),
                    ),
                    last_seen: p.last_updated,
                    health: p.knowledge.health.clone(),
                    latency: Some(Duration::from_millis(1)),
                }
            })
            .collect();

        debug!("Query returned {} matching primals", peers.len());

        Ok(peers)
    }

    async fn unannounce(&self, id: &PrimalId) -> Result<()> {
        let mut primals = self.primals.write().await;

        if primals.remove(id).is_some() {
            info!("Unregistered primal '{}'", id.as_str());
            Ok(())
        } else {
            Err(NestGateError::not_found(format!(
                "Primal '{}' not found in registry",
                id.as_str()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal_primal_discovery::capability_based_discovery::{
        BindingInfo, HealthStatus, Protocol,
    };
    use std::net::{IpAddr, Ipv4Addr};

    fn create_test_knowledge(id: &str, capabilities: Vec<PrimalCapability>) -> PrimalSelfKnowledge {
        // Use provided ID for testing - this allows testing capacity limits and multiple primals
        let test_id = PrimalId::from_string(id.to_string());

        PrimalSelfKnowledge {
            id: test_id,
            capabilities,
            binding: BindingInfo {
                address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                port: 8080,
                protocol: Protocol::Tcp,
            },
            health: HealthStatus::Healthy,
            metadata: std::collections::HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_announce_and_find() {
        let backend = InMemoryDiscoveryBackend::new();
        let knowledge = create_test_knowledge("test1", vec![PrimalCapability::ZfsStorage]);

        // Announce
        backend.announce(&knowledge).await.unwrap();

        // Find by capability
        let peers = backend
            .find_by_capability(&PrimalCapability::ZfsStorage)
            .await
            .unwrap();

        assert_eq!(peers.len(), 1);
        assert!(peers[0]
            .capabilities
            .contains(&PrimalCapability::ZfsStorage));
    }

    #[tokio::test]
    async fn test_multiple_primals() {
        let backend = InMemoryDiscoveryBackend::new();

        let k1 = create_test_knowledge("test1", vec![PrimalCapability::ZfsStorage]);
        let k2 = create_test_knowledge("test2", vec![PrimalCapability::ApiGateway]);
        let k3 = create_test_knowledge(
            "test3",
            vec![PrimalCapability::ZfsStorage, PrimalCapability::ApiGateway],
        );

        backend.announce(&k1).await.unwrap();
        backend.announce(&k2).await.unwrap();
        backend.announce(&k3).await.unwrap();

        // Find ZFS storage
        let zfs_peers = backend
            .find_by_capability(&PrimalCapability::ZfsStorage)
            .await
            .unwrap();
        assert_eq!(zfs_peers.len(), 2);

        // Find API gateway
        let api_peers = backend
            .find_by_capability(&PrimalCapability::ApiGateway)
            .await
            .unwrap();
        assert_eq!(api_peers.len(), 2);
    }

    #[tokio::test]
    async fn test_query() {
        let backend = InMemoryDiscoveryBackend::new();

        let knowledge = create_test_knowledge(
            "test",
            vec![PrimalCapability::ZfsStorage, PrimalCapability::ApiGateway],
        );

        backend.announce(&knowledge).await.unwrap();

        // Query for both capabilities
        let query = DiscoveryQuery {
            required_capabilities: vec![PrimalCapability::ZfsStorage, PrimalCapability::ApiGateway],
            optional_capabilities: vec![],
            max_latency: None,
            min_health: HealthStatus::Healthy,
        };

        let peers = backend.query(&query).await.unwrap();
        assert_eq!(peers.len(), 1);
    }

    #[tokio::test]
    async fn test_unannounce() {
        let backend = InMemoryDiscoveryBackend::new();
        let knowledge = create_test_knowledge("test", vec![PrimalCapability::ZfsStorage]);

        backend.announce(&knowledge).await.unwrap();

        // Verify it's there
        let peers = backend
            .find_by_capability(&PrimalCapability::ZfsStorage)
            .await
            .unwrap();
        assert_eq!(peers.len(), 1);

        // Unannounce
        backend.unannounce(&knowledge.id).await.unwrap();

        // Verify it's gone
        let peers = backend
            .find_by_capability(&PrimalCapability::ZfsStorage)
            .await
            .unwrap();
        assert_eq!(peers.len(), 0);
    }

    #[tokio::test]
    async fn test_capacity_limit() {
        let config = InMemoryConfig {
            max_primals: 2,
            stale_threshold: Duration::from_secs(300),
        };
        let backend = InMemoryDiscoveryBackend::with_config(config);

        let k1 = create_test_knowledge("test1", vec![PrimalCapability::ZfsStorage]);
        let k2 = create_test_knowledge("test2", vec![PrimalCapability::ApiGateway]);
        let k3 = create_test_knowledge("test3", vec![PrimalCapability::Observability]);

        backend.announce(&k1).await.unwrap();
        backend.announce(&k2).await.unwrap();

        // Third should fail
        let result = backend.announce(&k3).await;
        assert!(result.is_err());
    }
}
