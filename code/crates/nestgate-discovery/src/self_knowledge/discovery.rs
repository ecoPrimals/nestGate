// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Runtime discovery of other primals in the ecosystem
//!
//! **Philosophy**: Discover primals by their capabilities, not by their names or addresses.

use super::{PrimalId, PrimalInfo, SelfKnowledge};
use anyhow::{Result, bail};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Discovery service for finding other primals in the ecosystem
///
/// **Core Principles**:
/// - Query by capability, not by name
/// - Cache discoveries with TTL
/// - Health check discovered primals
/// - Graceful degradation when primals unavailable
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::self_knowledge::{SelfKnowledge, PrimalDiscovery};
///
/// # async fn example() -> anyhow::Result<()> {
/// let self_knowledge = SelfKnowledge::builder()
///     .with_id("nestgate")
///     .with_capability("storage")
///     .build()?;
///
/// let discovery = PrimalDiscovery::new(self_knowledge);
///
/// // Announce ourselves
/// discovery.announce().await?;
///
/// // Find primals with orchestration capability
/// let orchestrators = discovery
///     .find_capability("orchestration")
///     .await?;
///
/// for orch in orchestrators {
///     println!("Found: {} at {:?}", orch.name, orch.endpoints);
/// }
/// # Ok(())
/// # }
/// ```
pub struct PrimalDiscovery {
    /// Our own self-knowledge
    self_knowledge: Arc<SelfKnowledge>,

    /// Discovered primals (cached)
    discovered: Arc<RwLock<HashMap<PrimalId, DiscoveredPrimal>>>,

    /// Discovery backends to use
    backends: Vec<Box<dyn DiscoveryBackend>>,

    /// Configuration
    config: DiscoveryConfig,
}

/// Internal wrapper for discovered primals with cache metadata
#[derive(Debug, Clone)]
struct DiscoveredPrimal {
    info: PrimalInfo,
    cached_at: SystemTime,
}

/// Configuration for discovery behavior
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// How long to cache discovered primals before re-querying
    pub cache_ttl: Duration,

    /// How often to health check discovered primals
    pub health_check_interval: Duration,

    /// Timeout for discovery queries
    pub query_timeout: Duration,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            cache_ttl: Duration::from_secs(300),            // 5 minutes
            health_check_interval: Duration::from_secs(30), // 30 seconds
            query_timeout: Duration::from_secs(5),          // 5 seconds
        }
    }
}

impl PrimalDiscovery {
    /// Create a new discovery service with default configuration
    #[must_use]
    pub fn new(self_knowledge: SelfKnowledge) -> Self {
        Self::with_config(self_knowledge, DiscoveryConfig::default())
    }

    /// Create a new discovery service with custom configuration
    #[must_use]
    pub fn with_config(self_knowledge: SelfKnowledge, config: DiscoveryConfig) -> Self {
        Self {
            self_knowledge: Arc::new(self_knowledge),
            discovered: Arc::new(RwLock::new(HashMap::new())),
            backends: Vec::new(),
            config,
        }
    }

    /// Add a discovery backend
    ///
    /// Multiple backends can be added for redundancy and broader reach.
    /// Examples: mDNS, Consul, Kubernetes service discovery, etc.
    pub fn add_backend(&mut self, backend: Box<dyn DiscoveryBackend>) {
        self.backends.push(backend);
    }

    /// Announce our presence to the discovery system
    ///
    /// This makes this primal discoverable by others.
    pub async fn announce(&self) -> Result<()> {
        if self.backends.is_empty() {
            warn!("No discovery backends configured, announcement will not propagate");
            return Ok(());
        }

        info!(
            "Announcing primal '{}' with {} capabilities",
            self.self_knowledge.id,
            self.self_knowledge.capabilities.len()
        );

        let mut errors = Vec::new();

        for backend in &self.backends {
            match backend.announce(&self.self_knowledge).await {
                Ok(()) => {
                    debug!("Successfully announced to backend: {}", backend.name());
                }
                Err(e) => {
                    warn!("Failed to announce to backend {}: {}", backend.name(), e);
                    errors.push(e);
                }
            }
        }

        if errors.len() == self.backends.len() {
            bail!("Failed to announce to all backends");
        }

        Ok(())
    }

    /// Find primals that provide a specific capability
    ///
    /// **Philosophy**: Query by what you need, not who you think provides it.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// # use nestgate_core::self_knowledge::PrimalDiscovery;
    /// # async fn example(discovery: &PrimalDiscovery) -> anyhow::Result<()> {
    /// // Don't ask for "orchestrator" by name
    /// // Ask for "orchestration capability"
    /// let providers = discovery
    ///     .find_capability("orchestration")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn find_capability(&self, capability: &str) -> Result<Vec<PrimalInfo>> {
        // Check cache first
        let cached = self.get_from_cache(capability).await;
        if !cached.is_empty() {
            debug!(
                "Found {} primals in cache for capability: {}",
                cached.len(),
                capability
            );
            return Ok(cached);
        }

        // Query backends
        if self.backends.is_empty() {
            warn!("No discovery backends configured");
            return Ok(Vec::new());
        }

        let mut all_results = Vec::new();

        for backend in &self.backends {
            match tokio::time::timeout(
                self.config.query_timeout,
                backend.query_capability(capability),
            )
            .await
            {
                Ok(Ok(results)) => {
                    debug!(
                        "Backend {} returned {} results for capability: {}",
                        backend.name(),
                        results.len(),
                        capability
                    );
                    all_results.extend(results);
                }
                Ok(Err(e)) => {
                    warn!("Backend {} query failed: {}", backend.name(), e);
                }
                Err(_) => {
                    warn!("Backend {} query timed out", backend.name());
                }
            }
        }

        // Deduplicate by primal ID
        let mut unique: HashMap<PrimalId, PrimalInfo> = HashMap::new();
        for info in all_results {
            unique.insert(info.id.clone(), info);
        }

        let results: Vec<PrimalInfo> = unique.into_values().collect();

        // Update cache
        self.update_cache(&results).await;

        Ok(results)
    }

    /// Find a specific primal by ID
    ///
    /// **Note**: Prefer [`find_capability`](Self::find_capability) when possible.
    /// Only use this when you specifically need a particular primal.
    pub async fn find_primal(&self, id: &PrimalId) -> Result<Option<PrimalInfo>> {
        // Check cache
        let discovered = self.discovered.read().await;
        if let Some(cached) = discovered.get(id)
            && self.is_cache_valid(&cached.cached_at)
        {
            return Ok(Some(cached.info.clone()));
        }
        drop(discovered);

        // Query backends
        for backend in &self.backends {
            match tokio::time::timeout(self.config.query_timeout, backend.query_primal(id)).await {
                Ok(Ok(Some(info))) => {
                    self.cache_single(&info).await;
                    return Ok(Some(info));
                }
                Ok(Ok(None)) => continue,
                Ok(Err(e)) => {
                    warn!("Backend {} query failed: {}", backend.name(), e);
                }
                Err(_) => {
                    warn!("Backend {} query timed out", backend.name());
                }
            }
        }

        Ok(None)
    }

    /// Get all discovered primals
    pub async fn all_primals(&self) -> Vec<PrimalInfo> {
        let discovered = self.discovered.read().await;
        discovered
            .values()
            .filter(|d| self.is_cache_valid(&d.cached_at))
            .map(|d| d.info.clone())
            .collect()
    }

    /// Clear the discovery cache
    pub async fn clear_cache(&self) {
        let mut discovered = self.discovered.write().await;
        discovered.clear();
        debug!("Discovery cache cleared");
    }

    /// Refresh a specific primal's information
    pub async fn refresh_primal(&self, id: &PrimalId) -> Result<()> {
        // Remove from cache to force re-query
        {
            let mut discovered = self.discovered.write().await;
            discovered.remove(id);
        }

        // Query fresh data
        self.find_primal(id).await?;

        Ok(())
    }

    // Internal cache management

    async fn get_from_cache(&self, capability: &str) -> Vec<PrimalInfo> {
        let discovered = self.discovered.read().await;
        discovered
            .values()
            .filter(|d| {
                self.is_cache_valid(&d.cached_at)
                    && d.info.capabilities.iter().any(|c| c == capability)
            })
            .map(|d| d.info.clone())
            .collect()
    }

    async fn update_cache(&self, primals: &[PrimalInfo]) {
        let mut discovered = self.discovered.write().await;
        let now = SystemTime::now();

        for info in primals {
            discovered.insert(
                info.id.clone(),
                DiscoveredPrimal {
                    info: info.clone(),
                    cached_at: now,
                },
            );
        }
    }

    async fn cache_single(&self, info: &PrimalInfo) {
        let mut discovered = self.discovered.write().await;
        discovered.insert(
            info.id.clone(),
            DiscoveredPrimal {
                info: info.clone(),
                cached_at: SystemTime::now(),
            },
        );
    }

    fn is_cache_valid(&self, cached_at: &SystemTime) -> bool {
        match SystemTime::now().duration_since(*cached_at) {
            Ok(elapsed) => elapsed < self.config.cache_ttl,
            Err(_) => false, // Clock went backwards, invalidate cache
        }
    }
}

/// Backend for discovering primals
///
/// Implement this trait to add support for different discovery mechanisms:
/// - mDNS (local network)
/// - Consul (service mesh)
/// - Kubernetes (container orchestration)
/// - Etcd (distributed key-value)
/// - Custom protocol
pub trait DiscoveryBackend: Send + Sync {
    /// Name of this backend (for logging)
    fn name(&self) -> &str;

    /// Announce a primal's presence
    fn announce(
        &self,
        knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;

    /// Query for primals with a specific capability
    fn query_capability(
        &self,
        capability: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<PrimalInfo>>> + Send + '_>>;

    /// Query for a specific primal by ID
    fn query_primal(
        &self,
        id: &PrimalId,
    ) -> Pin<Box<dyn Future<Output = Result<Option<PrimalInfo>>> + Send + '_>>;
}

/// In-memory discovery backend for testing and development
///
/// **Note**: This is not persistent across restarts. Use for:
/// - Unit tests
/// - Integration tests
/// - Local development
/// - Single-instance deployments
pub struct InMemoryBackend {
    primals: Arc<RwLock<HashMap<PrimalId, PrimalInfo>>>,
}

impl InMemoryBackend {
    /// Create a new in-memory backend
    #[must_use]
    pub fn new() -> Self {
        Self {
            primals: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryBackend for InMemoryBackend {
    fn name(&self) -> &'static str {
        "in-memory"
    }

    fn announce(
        &self,
        knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let knowledge = knowledge.clone();
        Box::pin(async move {
            let info = PrimalInfo {
                id: knowledge.id.clone(),
                name: knowledge.name.clone(),
                version: knowledge.version.clone(),
                capabilities: knowledge.capabilities.clone(),
                endpoints: knowledge.endpoints.clone(),
                health: knowledge.health,
                discovered_at: SystemTime::now(),
                last_health_check: SystemTime::now(),
            };

            let mut primals = self.primals.write().await;
            primals.insert(knowledge.id.clone(), info);

            Ok(())
        })
    }

    fn query_capability(
        &self,
        capability: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<PrimalInfo>>> + Send + '_>> {
        let capability = capability.to_string();
        Box::pin(async move {
            let primals = self.primals.read().await;
            Ok(primals
                .values()
                .filter(|p| p.capabilities.iter().any(|c| c == &capability))
                .cloned()
                .collect())
        })
    }

    fn query_primal(
        &self,
        id: &PrimalId,
    ) -> Pin<Box<dyn Future<Output = Result<Option<PrimalInfo>>> + Send + '_>> {
        let id = id.clone();
        Box::pin(async move {
            let primals = self.primals.read().await;
            Ok(primals.get(&id).cloned())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_with_in_memory_backend() {
        let self_knowledge = SelfKnowledge::builder()
            .with_id("test")
            .with_capability("storage")
            .build()
            .unwrap();

        let mut discovery = PrimalDiscovery::new(self_knowledge);
        discovery.add_backend(Box::new(InMemoryBackend::new()));

        // Announce ourselves
        discovery.announce().await.unwrap();

        // Should be able to find ourselves by capability
        let results = discovery.find_capability("storage").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id.as_str(), "test");
    }

    #[tokio::test]
    async fn test_find_capability_multiple_primals() {
        let backend = Arc::new(InMemoryBackend::new());

        // Register multiple primals with same capability
        for i in 1..=3 {
            let knowledge = SelfKnowledge::builder()
                .with_id(format!("primal{}", i))
                .with_capability("storage")
                .build()
                .unwrap();

            backend.announce(&knowledge).await.unwrap();
        }

        let self_knowledge = SelfKnowledge::builder().with_id("test").build().unwrap();

        let mut discovery = PrimalDiscovery::new(self_knowledge);
        discovery.add_backend(Box::new(InMemoryBackend::new()));

        // Manually add the primals from the shared backend
        for i in 1..=3 {
            let knowledge = SelfKnowledge::builder()
                .with_id(format!("primal{}", i))
                .with_capability("storage")
                .build()
                .unwrap();
            // The backend we just added will store these
            if let Some(backend_ref) = discovery.backends.last() {
                backend_ref.announce(&knowledge).await.unwrap();
            }
        }

        let results = discovery.find_capability("storage").await.unwrap();
        assert_eq!(results.len(), 3);
    }

    #[tokio::test]
    async fn test_cache_ttl() {
        let self_knowledge = SelfKnowledge::builder()
            .with_id("test")
            .with_capability("test")
            .build()
            .unwrap();

        let config = DiscoveryConfig {
            cache_ttl: Duration::from_millis(100),
            ..Default::default()
        };

        let mut discovery = PrimalDiscovery::with_config(self_knowledge, config);
        discovery.add_backend(Box::new(InMemoryBackend::new()));

        discovery.announce().await.unwrap();

        // First query should hit backend
        let results1 = discovery.find_capability("test").await.unwrap();
        assert_eq!(results1.len(), 1);

        // Immediate second query should hit cache
        let results2 = discovery.find_capability("test").await.unwrap();
        assert_eq!(results2.len(), 1);

        // Wait for cache to expire
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should query backend again
        let results3 = discovery.find_capability("test").await.unwrap();
        assert_eq!(results3.len(), 1);
    }

    #[tokio::test]
    async fn test_find_primal_by_id() {
        let backend = InMemoryBackend::new();

        let knowledge = SelfKnowledge::builder()
            .with_id("target")
            .with_capability("test")
            .build()
            .unwrap();

        backend.announce(&knowledge).await.unwrap();

        let self_knowledge = SelfKnowledge::builder().with_id("test").build().unwrap();

        let mut discovery = PrimalDiscovery::new(self_knowledge);
        discovery.add_backend(Box::new(backend));

        let result = discovery
            .find_primal(&PrimalId::new("target"))
            .await
            .unwrap();

        assert!(result.is_some());
        assert_eq!(result.unwrap().id.as_str(), "target");
    }
}
