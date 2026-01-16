// Discovery Migration Helper
//
// Provides gradual migration from hardcoded values to capability-based discovery
// with robust fallback chains.

use crate::http_client_stub as reqwest;
use crate::error::{NestGateError, Result};
use crate::primal_discovery::PrimalDiscovery;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Helper for migrating from hardcoded values to capability discovery
///
/// Provides a 3-tier fallback strategy:
/// 1. Discovery (preferred) - Find via capability
/// 2. Environment (container/K8s) - Read from env vars
/// 3. Default (development) - Sensible defaults
///
/// # Example
///
/// ```rust
/// use nestgate_core::primal_discovery::migration::DiscoveryOrEnv;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let helper = DiscoveryOrEnv::new(discovery);
///
/// // Automatically tries: discovery → env → default
/// let auth_endpoint = helper
///     .endpoint_for("security", "AUTH_SERVICE_PORT", 5000)
///     .await?;
///     
/// // Connect with discovered/configured endpoint
/// let client = reqwest::Client::new();
/// let response = client.get(&auth_endpoint).send().await?;
/// # Ok(())
/// # }
/// ```
pub struct DiscoveryOrEnv {
    #[allow(dead_code)] // Reserved for future async discovery integration
    discovery: Arc<PrimalDiscovery>,
    cache: Arc<RwLock<EndpointCache>>,
    config: MigrationConfig,
}

/// Configuration for migration behavior
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// How long to cache discovered endpoints
    pub cache_ttl: Duration,

    /// Whether to prefer environment over discovery
    /// (useful for gradual rollout)
    pub prefer_environment: bool,

    /// Whether to allow fallback to defaults
    /// (disable in production for strictness)
    pub allow_defaults: bool,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            cache_ttl: Duration::from_secs(300), // 5 minutes
            prefer_environment: false,           // Prefer discovery
            allow_defaults: true,                // Allow defaults (dev-friendly)
        }
    }
}

/// Cached endpoint information
struct CachedEndpoint {
    url: String,
    source: EndpointSource,
    cached_at: Instant,
}

#[derive(Debug, Clone, PartialEq)]
enum EndpointSource {
    #[allow(dead_code)] // Reserved for future async discovery
    Discovery,
    Environment,
    Default,
}

struct EndpointCache {
    entries: HashMap<String, CachedEndpoint>,
}

impl DiscoveryOrEnv {
    /// Create new migration helper
    pub fn new(discovery: Arc<PrimalDiscovery>) -> Self {
        Self::with_config(discovery, MigrationConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(discovery: Arc<PrimalDiscovery>, config: MigrationConfig) -> Self {
        Self {
            discovery,
            cache: Arc::new(RwLock::new(EndpointCache {
                entries: HashMap::new(),
            })),
            config,
        }
    }

    /// Get endpoint for capability with fallback chain
    ///
    /// # Arguments
    ///
    /// * `capability` - Capability to discover (e.g., "security", "storage")
    /// * `env_var` - Environment variable name (e.g., "AUTH_SERVICE_PORT")
    /// * `default_port` - Default port for development
    ///
    /// # Returns
    ///
    /// Full endpoint URL (e.g., "http://192.168.1.100:8080")
    pub async fn endpoint_for(
        &self,
        capability: &str,
        env_var: &str,
        default_port: u16,
    ) -> Result<String> {
        // Check cache first
        if let Some(cached) = self.check_cache(capability).await {
            debug!(
                "Using cached endpoint for {}: {} (source: {:?})",
                capability, cached.url, cached.source
            );
            return Ok(cached.url);
        }

        // Determine search order based on config
        let endpoint = if self.config.prefer_environment {
            self.try_environment(env_var)
                .or_else(|| self.try_discovery_sync(capability))
                .or_else(|| self.try_default(default_port))
        } else {
            self.try_discovery_sync(capability)
                .or_else(|| self.try_environment(env_var))
                .or_else(|| self.try_default(default_port))
        };

        match endpoint {
            Some((url, source)) => {
                info!("Resolved {} to {} via {:?}", capability, url, source);
                self.update_cache(capability, url.clone(), source).await;
                Ok(url)
            }
            None => Err(NestGateError::configuration_error(
                "endpoint_resolution",
                &format!("Could not resolve endpoint for capability: {}", capability),
            )),
        }
    }

    /// Try discovery (currently returns None, reserved for async integration)
    fn try_discovery_sync(&self, _capability: &str) -> Option<(String, EndpointSource)> {
        // Reserved for future async discovery integration
        // Currently using environment and defaults only
        None
    }

    /// Try environment variable
    fn try_environment(&self, env_var: &str) -> Option<(String, EndpointSource)> {
        std::env::var(env_var).ok().and_then(|val| {
            // Try as port first
            if let Ok(port) = val.parse::<u16>() {
                let url = format!("http://localhost:{}", port);
                debug!("Found {} in environment: {}", env_var, url);
                Some((url, EndpointSource::Environment))
            } else if val.starts_with("http://") || val.starts_with("https://") {
                // Full URL in environment
                debug!("Found full URL in environment: {}", val);
                Some((val, EndpointSource::Environment))
            } else {
                warn!("Invalid value for {}: {}", env_var, val);
                None
            }
        })
    }

    /// Try default (development fallback)
    fn try_default(&self, port: u16) -> Option<(String, EndpointSource)> {
        if self.config.allow_defaults {
            let url = format!("http://localhost:{}", port);
            warn!("Using default endpoint: {}", url);
            Some((url, EndpointSource::Default))
        } else {
            debug!("Defaults disabled, no fallback available");
            None
        }
    }

    /// Check cache for endpoint
    async fn check_cache(&self, capability: &str) -> Option<CachedEndpoint> {
        let cache = self.cache.read().await;
        cache.entries.get(capability).and_then(|entry| {
            if entry.cached_at.elapsed() < self.config.cache_ttl {
                Some(CachedEndpoint {
                    url: entry.url.clone(),
                    source: entry.source.clone(),
                    cached_at: entry.cached_at,
                })
            } else {
                debug!("Cache entry for {} is stale", capability);
                None
            }
        })
    }

    /// Update cache with new endpoint
    async fn update_cache(&self, capability: &str, url: String, source: EndpointSource) {
        let mut cache = self.cache.write().await;
        cache.entries.insert(
            capability.to_string(),
            CachedEndpoint {
                url,
                source,
                cached_at: Instant::now(),
            },
        );
    }

    /// Clear cache for capability
    pub async fn invalidate(&self, capability: &str) {
        let mut cache = self.cache.write().await;
        cache.entries.remove(capability);
        debug!("Invalidated cache for {}", capability);
    }

    /// Clear all cached endpoints
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        let count = cache.entries.len();
        cache.entries.clear();
        debug!("Cleared {} cached endpoints", count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primal_discovery::SelfKnowledge;

    fn create_test_discovery() -> Arc<PrimalDiscovery> {
        let knowledge = SelfKnowledge::builder()
            .name("test")
            .capability("test")
            .endpoint_http(9999)
            .build();
        Arc::new(PrimalDiscovery::new(knowledge))
    }

    #[tokio::test]
    async fn test_environment_fallback() {
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        // Set environment variable
        std::env::set_var("TEST_SERVICE_PORT", "7777");

        let endpoint = helper
            .endpoint_for("nonexistent", "TEST_SERVICE_PORT", 8888)
            .await
            .unwrap();

        assert_eq!(endpoint, "http://localhost:7777");

        // Cleanup
        std::env::remove_var("TEST_SERVICE_PORT");
    }

    #[tokio::test]
    async fn test_default_fallback() {
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        // Ensure env var doesn't exist
        std::env::remove_var("NONEXISTENT_PORT");

        let endpoint = helper
            .endpoint_for("nonexistent", "NONEXISTENT_PORT", 6666)
            .await
            .unwrap();

        assert_eq!(endpoint, "http://localhost:6666");
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        // Set up environment
        std::env::set_var("TEST_PORT", "5555");

        // First call - should cache
        let endpoint1 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();

        // Change environment
        std::env::set_var("TEST_PORT", "3333");

        // Should still use cache
        let endpoint2 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();
        assert_eq!(endpoint1, endpoint2);

        // Invalidate cache
        helper.invalidate("test").await;

        // Should use new environment value
        let endpoint3 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();
        assert_eq!(endpoint3, "http://localhost:3333");

        // Cleanup
        std::env::remove_var("TEST_PORT");
    }

    #[tokio::test]
    async fn test_strict_mode_no_defaults() {
        let discovery = create_test_discovery();
        let mut config = MigrationConfig::default();
        config.allow_defaults = false; // Strict mode

        let helper = DiscoveryOrEnv::with_config(discovery, config);

        // Should fail without discovery or environment
        let result = helper
            .endpoint_for("nonexistent", "NONEXISTENT_VAR", 9999)
            .await;

        assert!(result.is_err());
    }

    // ==================== NEW ERROR PATH TESTS ====================

    #[tokio::test]
    async fn test_invalid_port_in_environment_falls_back_to_default() {
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        // Set invalid port value (too large)
        std::env::set_var("INVALID_PORT", "99999");

        let endpoint = helper
            .endpoint_for("test_service", "INVALID_PORT", 8080)
            .await
            .unwrap();

        // Should fall back to default since parsing fails
        assert_eq!(endpoint, "http://localhost:8080");

        std::env::remove_var("INVALID_PORT");
    }

    #[tokio::test]
    async fn test_full_url_in_environment_variable() {
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        // Set full URL in environment
        std::env::set_var("SERVICE_URL", "https://api.example.com:8443");

        let endpoint = helper
            .endpoint_for("test_service", "SERVICE_URL", 8080)
            .await
            .unwrap();

        assert_eq!(endpoint, "https://api.example.com:8443");

        std::env::remove_var("SERVICE_URL");
    }

    #[tokio::test]
    async fn test_cache_expiration_via_invalidation() {
        // Modern concurrent pattern: Use explicit invalidation instead of time-based expiration
        // This is robust, deterministic, and doesn't rely on sleep/timing
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        std::env::set_var("TEST_TTL_PORT", "5000");

        // First call - caches the value
        let endpoint1 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();
        assert_eq!(endpoint1, "http://localhost:5000");

        // Change environment
        std::env::set_var("TEST_TTL_PORT", "6000");

        // Cache still returns old value
        let endpoint2 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();
        assert_eq!(
            endpoint2, "http://localhost:5000",
            "Cache should still have old value"
        );

        // Explicitly invalidate cache (modern pattern: explicit > implicit)
        helper.invalidate("ttl_test").await;

        // Now should use new value
        let endpoint3 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();
        assert_eq!(
            endpoint3, "http://localhost:6000",
            "After invalidation, should use new value"
        );

        std::env::remove_var("TEST_TTL_PORT");
    }

    #[tokio::test]
    async fn test_prefer_environment_over_discovery() {
        let discovery = create_test_discovery();
        let mut config = MigrationConfig::default();
        config.prefer_environment = true; // Prefer environment

        let helper = DiscoveryOrEnv::with_config(discovery, config);

        std::env::set_var("PRIORITY_PORT", "7000");

        let endpoint = helper
            .endpoint_for("test", "PRIORITY_PORT", 8000)
            .await
            .unwrap();

        // Should use environment (preferred)
        assert_eq!(endpoint, "http://localhost:7000");

        std::env::remove_var("PRIORITY_PORT");
    }

    #[tokio::test]
    async fn test_clear_cache_removes_all_entries() {
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        // Add multiple cached entries
        std::env::set_var("SERVICE1_PORT", "5001");
        std::env::set_var("SERVICE2_PORT", "5002");
        std::env::set_var("SERVICE3_PORT", "5003");

        let _ = helper.endpoint_for("service1", "SERVICE1_PORT", 9000).await;
        let _ = helper.endpoint_for("service2", "SERVICE2_PORT", 9000).await;
        let _ = helper.endpoint_for("service3", "SERVICE3_PORT", 9000).await;

        // Clear all cache
        helper.clear_cache().await;

        // Change all environments
        std::env::set_var("SERVICE1_PORT", "6001");
        std::env::set_var("SERVICE2_PORT", "6002");
        std::env::set_var("SERVICE3_PORT", "6003");

        // Should use new values (cache was cleared)
        let e1 = helper
            .endpoint_for("service1", "SERVICE1_PORT", 9000)
            .await
            .unwrap();
        let e2 = helper
            .endpoint_for("service2", "SERVICE2_PORT", 9000)
            .await
            .unwrap();
        let e3 = helper
            .endpoint_for("service3", "SERVICE3_PORT", 9000)
            .await
            .unwrap();

        assert_eq!(e1, "http://localhost:6001");
        assert_eq!(e2, "http://localhost:6002");
        assert_eq!(e3, "http://localhost:6003");

        // Cleanup
        std::env::remove_var("SERVICE1_PORT");
        std::env::remove_var("SERVICE2_PORT");
        std::env::remove_var("SERVICE3_PORT");
    }

    #[tokio::test]
    async fn test_empty_environment_variable_falls_back() {
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        // Set empty environment variable
        std::env::set_var("EMPTY_VAR", "");

        let endpoint = helper
            .endpoint_for("test", "EMPTY_VAR", 7070)
            .await
            .unwrap();

        // Should fall back to default
        assert_eq!(endpoint, "http://localhost:7070");

        std::env::remove_var("EMPTY_VAR");
    }

    #[tokio::test]
    async fn test_malformed_url_in_environment_falls_back() {
        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        // Set malformed URL
        std::env::set_var("MALFORMED_URL", "not-a-valid-url");

        let endpoint = helper
            .endpoint_for("test", "MALFORMED_URL", 8181)
            .await
            .unwrap();

        // Should fall back to default
        assert_eq!(endpoint, "http://localhost:8181");

        std::env::remove_var("MALFORMED_URL");
    }

    #[tokio::test]
    async fn test_concurrent_endpoint_resolution() {
        let discovery = create_test_discovery();
        let helper = Arc::new(DiscoveryOrEnv::new(discovery));

        // Set up environment
        for i in 0..10 {
            std::env::set_var(format!("CONCURRENT_PORT_{}", i), format!("{}", 5000 + i));
        }

        let mut handles = vec![];

        // Spawn concurrent resolutions
        for i in 0..10 {
            let helper = helper.clone();
            let handle = tokio::spawn(async move {
                helper
                    .endpoint_for(
                        &format!("service_{}", i),
                        &format!("CONCURRENT_PORT_{}", i),
                        9000,
                    )
                    .await
            });
            handles.push(handle);
        }

        // Wait for all to complete
        let mut results = vec![];
        for handle in handles {
            let result = handle.await.expect("Task should not panic");
            results.push(result);
        }

        // All should succeed
        assert_eq!(results.len(), 10);
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_ok());
            assert_eq!(
                result.as_ref().unwrap(),
                &format!("http://localhost:{}", 5000 + i)
            );
        }

        // Cleanup
        for i in 0..10 {
            std::env::remove_var(format!("CONCURRENT_PORT_{}", i));
        }
    }
}
