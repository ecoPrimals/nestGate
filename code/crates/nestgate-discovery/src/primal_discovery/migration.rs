// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Discovery Migration Helper
//
// Provides gradual migration from hardcoded values to capability-based discovery
// with robust fallback chains.

// HTTP removed — use orchestration / network capability discovery for external HTTP
use crate::primal_discovery::PrimalDiscovery;
use nestgate_config::constants::hardcoding::addresses;
use nestgate_types::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv};
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
/// ```rust,ignore
/// use nestgate_core::primal_discovery::migration::DiscoveryOrEnv;
/// let helper = DiscoveryOrEnv::new(discovery);
/// let auth_endpoint = helper.endpoint_for("security", "AUTH_SERVICE_PORT", 5000).await?;
/// ```
pub struct DiscoveryOrEnv {
    #[expect(
        dead_code,
        reason = "Reserved for capability-based resolution; endpoint helpers use cache/env only for now"
    )]
    discovery: Arc<PrimalDiscovery>,
    cache: Arc<RwLock<EndpointCache>>,
    config: MigrationConfig,
    env: Arc<dyn EnvSource>,
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum EndpointSource {
    #[expect(
        dead_code,
        reason = "Forward-compatible source tag; not yet written to cache"
    )]
    Discovery,
    Environment,
    Default,
}

struct EndpointCache {
    entries: HashMap<String, CachedEndpoint>,
}

impl DiscoveryOrEnv {
    /// Create new migration helper
    #[must_use]
    pub fn new(discovery: Arc<PrimalDiscovery>) -> Self {
        Self::with_config(discovery, MigrationConfig::default())
    }

    /// Create with custom configuration
    #[must_use]
    pub fn with_config(discovery: Arc<PrimalDiscovery>, config: MigrationConfig) -> Self {
        Self::with_config_and_env(discovery, config, Arc::new(ProcessEnv))
    }

    /// Create with custom configuration and injectable environment (use [`MapEnv`](nestgate_types::MapEnv) in tests)
    #[must_use]
    pub fn with_config_and_env(
        discovery: Arc<PrimalDiscovery>,
        config: MigrationConfig,
        env: Arc<dyn EnvSource>,
    ) -> Self {
        Self {
            discovery,
            cache: Arc::new(RwLock::new(EndpointCache {
                entries: HashMap::new(),
            })),
            config,
            env,
        }
    }

    /// Get endpoint for capability with fallback chain
    ///
    /// # Arguments
    ///
    /// * `capability` - Capability to discover (e.g., "security", "storage")
    /// * `env_var` - Environment variable name (e.g., "`AUTH_SERVICE_PORT`")
    /// * `default_port` - Default port for development
    ///
    /// # Returns
    ///
    /// Full endpoint URL (e.g., "<http://192.168.1.100:8080>")
    pub async fn endpoint_for(
        &self,
        capability: &str,
        env_var: &str,
        default_port: u16,
    ) -> Result<String> {
        // Check cache first
        if let Some((url, source)) = self.check_cache(capability).await {
            debug!(
                "Using cached endpoint for {}: {} (source: {:?})",
                capability, url, source
            );
            return Ok(url);
        }

        // Determine search order based on config
        let endpoint = if self.config.prefer_environment {
            self.try_environment(env_var)
                .or_else(|| Self::try_discovery_sync(capability))
                .or_else(|| self.try_default(default_port))
        } else {
            Self::try_discovery_sync(capability)
                .or_else(|| self.try_environment(env_var))
                .or_else(|| self.try_default(default_port))
        };

        match endpoint {
            Some((url, source)) => {
                info!("Resolved {} to {} via {:?}", capability, url, source);
                self.update_cache(capability, &url, source).await;
                Ok(url)
            }
            None => Err(NestGateError::configuration_error(
                "endpoint_resolution",
                format!("Could not resolve endpoint for capability: {capability}"),
            )),
        }
    }

    /// Try discovery (currently returns None, reserved for async integration)
    const fn try_discovery_sync(_capability: &str) -> Option<(String, EndpointSource)> {
        // Reserved for future async discovery integration
        // Currently using environment and defaults only
        None
    }

    /// Try environment variable
    fn try_environment(&self, env_var: &str) -> Option<(String, EndpointSource)> {
        self.env.get(env_var).and_then(|val| {
            val.parse::<u16>().ok().map_or_else(
                || {
                    if val.starts_with("http://") || val.starts_with("https://") {
                        debug!("Found full URL in environment: {}", val);
                        Some((val, EndpointSource::Environment))
                    } else {
                        warn!("Invalid value for {}: {}", env_var, val);
                        None
                    }
                },
                |port| {
                    let url = format!("http://{}:{port}", addresses::LOCALHOST_NAME);
                    debug!("Found {} in environment: {}", env_var, url);
                    Some((url, EndpointSource::Environment))
                },
            )
        })
    }

    /// Try default (development fallback)
    fn try_default(&self, port: u16) -> Option<(String, EndpointSource)> {
        if self.config.allow_defaults {
            let url = format!("http://{}:{port}", addresses::LOCALHOST_NAME);
            warn!("Using default endpoint: {}", url);
            Some((url, EndpointSource::Default))
        } else {
            debug!("Defaults disabled, no fallback available");
            None
        }
    }

    /// Check cache for endpoint
    async fn check_cache(&self, capability: &str) -> Option<(String, EndpointSource)> {
        let cache = self.cache.read().await;
        let out = cache.entries.get(capability).and_then(|entry| {
            if entry.cached_at.elapsed() < self.config.cache_ttl {
                Some((entry.url.clone(), entry.source))
            } else {
                None
            }
        });
        let stale_hit = cache.entries.contains_key(capability) && out.is_none();
        drop(cache);
        if stale_hit {
            debug!("Cache entry for {} is stale", capability);
        }
        out
    }

    /// Update cache with new endpoint
    async fn update_cache(&self, capability: &str, url: &str, source: EndpointSource) {
        let mut cache = self.cache.write().await;
        cache.entries.insert(
            capability.to_string(),
            CachedEndpoint {
                url: url.to_string(),
                source,
                cached_at: Instant::now(),
            },
        );
    }

    /// Clear cache for capability
    pub async fn invalidate(&self, capability: &str) {
        {
            let mut cache = self.cache.write().await;
            cache.entries.remove(capability);
        }
        debug!("Invalidated cache for {}", capability);
    }

    /// Clear all cached endpoints
    pub async fn clear_cache(&self) {
        let count = {
            let mut cache = self.cache.write().await;
            let count = cache.entries.len();
            cache.entries.clear();
            count
        };
        debug!("Cleared {} cached endpoints", count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primal_discovery::SelfKnowledge;
    use nestgate_types::MapEnv;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    /// For tests that need to mutate env between await points
    #[derive(Clone)]
    struct MutableTestEnv(Arc<Mutex<HashMap<String, String>>>);

    impl EnvSource for MutableTestEnv {
        fn get(&self, key: &str) -> Option<String> {
            self.0.lock().ok()?.get(key).cloned()
        }

        fn vars(&self) -> Vec<(String, String)> {
            self.0
                .lock()
                .ok()
                .map(|g| g.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default()
        }
    }

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
        let env: Arc<dyn EnvSource> = Arc::new(MapEnv::from([("TEST_SERVICE_PORT", "7777")]));
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let endpoint = helper
            .endpoint_for("nonexistent", "TEST_SERVICE_PORT", 8888)
            .await
            .unwrap();

        assert_eq!(endpoint, "http://localhost:7777");
    }

    #[tokio::test]
    async fn test_default_fallback() {
        let env: Arc<dyn EnvSource> = Arc::new(MapEnv::new());
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let endpoint = helper
            .endpoint_for("nonexistent", "NONEXISTENT_PORT", 6666)
            .await
            .unwrap();

        assert_eq!(endpoint, "http://localhost:6666");
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let map = Arc::new(Mutex::new(HashMap::from([(
            "TEST_PORT".to_string(),
            "5555".to_string(),
        )])));
        let env: Arc<dyn EnvSource> = Arc::new(MutableTestEnv(map.clone()));
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let endpoint1 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();

        map.lock()
            .expect("lock")
            .insert("TEST_PORT".to_string(), "3333".to_string());

        let endpoint2 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();

        helper.invalidate("test").await;

        let endpoint3 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();

        assert_eq!(endpoint1, endpoint2);
        assert_eq!(endpoint3, "http://localhost:3333");
    }

    #[tokio::test]
    async fn test_strict_mode_no_defaults() {
        let env: Arc<dyn EnvSource> = Arc::new(MapEnv::new());
        let mut config = MigrationConfig::default();
        config.allow_defaults = false; // Strict mode

        let helper = DiscoveryOrEnv::with_config_and_env(create_test_discovery(), config, env);

        // Should fail without discovery or environment
        let result = helper
            .endpoint_for("nonexistent", "NONEXISTENT_VAR", 9999)
            .await;

        assert!(result.is_err());
    }

    // ==================== NEW ERROR PATH TESTS ====================

    #[tokio::test]
    async fn test_invalid_port_in_environment_falls_back_to_default() {
        let env: Arc<dyn EnvSource> = Arc::new(MapEnv::from([("INVALID_PORT", "99999")]));
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let endpoint = helper
            .endpoint_for("test_service", "INVALID_PORT", 8080)
            .await
            .unwrap();

        assert_eq!(endpoint, "http://localhost:8080");
    }

    #[tokio::test]
    async fn test_full_url_in_environment_variable() {
        let env: Arc<dyn EnvSource> = Arc::new(MapEnv::from([(
            "SERVICE_URL",
            "https://api.example.com:8443",
        )]));
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let endpoint = helper
            .endpoint_for("test_service", "SERVICE_URL", 8080)
            .await
            .unwrap();

        assert_eq!(endpoint, "https://api.example.com:8443");
    }

    #[tokio::test]
    async fn test_cache_expiration_via_invalidation() {
        let map = Arc::new(Mutex::new(HashMap::from([(
            "TEST_TTL_PORT".to_string(),
            "5000".to_string(),
        )])));
        let env: Arc<dyn EnvSource> = Arc::new(MutableTestEnv(map.clone()));
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let endpoint1 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();

        map.lock()
            .expect("lock")
            .insert("TEST_TTL_PORT".to_string(), "6000".to_string());

        let endpoint2 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();

        helper.invalidate("ttl_test").await;

        let endpoint3 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();

        assert_eq!(endpoint1, "http://localhost:5000");
        assert_eq!(endpoint2, "http://localhost:5000");
        assert_eq!(endpoint3, "http://localhost:6000");
    }

    #[tokio::test]
    async fn test_prefer_environment_over_discovery() {
        let env: Arc<dyn EnvSource> = Arc::new(MapEnv::from([("PRIORITY_PORT", "7000")]));
        let mut config = MigrationConfig::default();
        config.prefer_environment = true;
        let helper = DiscoveryOrEnv::with_config_and_env(create_test_discovery(), config, env);

        let endpoint = helper
            .endpoint_for("test", "PRIORITY_PORT", 8000)
            .await
            .unwrap();

        assert_eq!(endpoint, "http://localhost:7000");
    }

    #[tokio::test]
    async fn test_clear_cache_removes_all_entries() {
        let map = Arc::new(Mutex::new(HashMap::from([
            ("SERVICE1_PORT".to_string(), "5001".to_string()),
            ("SERVICE2_PORT".to_string(), "5002".to_string()),
            ("SERVICE3_PORT".to_string(), "5003".to_string()),
        ])));
        let env: Arc<dyn EnvSource> = Arc::new(MutableTestEnv(map.clone()));
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let _ = helper.endpoint_for("service1", "SERVICE1_PORT", 9000).await;
        let _ = helper.endpoint_for("service2", "SERVICE2_PORT", 9000).await;
        let _ = helper.endpoint_for("service3", "SERVICE3_PORT", 9000).await;

        helper.clear_cache().await;

        {
            let mut g = map.lock().expect("lock");
            g.insert("SERVICE1_PORT".to_string(), "6001".to_string());
            g.insert("SERVICE2_PORT".to_string(), "6002".to_string());
            g.insert("SERVICE3_PORT".to_string(), "6003".to_string());
        }

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
    }

    #[tokio::test]
    async fn test_empty_environment_variable_falls_back() {
        let env: Arc<dyn EnvSource> = Arc::new(MapEnv::from([("EMPTY_VAR", "")]));
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let endpoint = helper
            .endpoint_for("test", "EMPTY_VAR", 7070)
            .await
            .unwrap();

        assert_eq!(endpoint, "http://localhost:7070");
    }

    #[tokio::test]
    async fn test_malformed_url_in_environment_falls_back() {
        let env: Arc<dyn EnvSource> =
            Arc::new(MapEnv::from([("MALFORMED_URL", "not-a-valid-url")]));
        let helper = DiscoveryOrEnv::with_config_and_env(
            create_test_discovery(),
            MigrationConfig::default(),
            env,
        );

        let endpoint = helper
            .endpoint_for("test", "MALFORMED_URL", 8181)
            .await
            .unwrap();

        assert_eq!(endpoint, "http://localhost:8181");
    }

    #[tokio::test]
    async fn test_concurrent_endpoint_resolution() {
        let mut h = HashMap::new();
        for i in 0..10 {
            h.insert(format!("CONCURRENT_PORT_{i}"), format!("{}", 5000 + i));
        }
        let env: Arc<dyn EnvSource> = Arc::new(MapEnv(h));
        let discovery = create_test_discovery();
        let helper = Arc::new(DiscoveryOrEnv::with_config_and_env(
            discovery,
            MigrationConfig::default(),
            env,
        ));

        let mut handles = vec![];
        for i in 0..10 {
            let helper = helper.clone();
            let handle = tokio::spawn(async move {
                helper
                    .endpoint_for(
                        &format!("service_{i}"),
                        &format!("CONCURRENT_PORT_{i}"),
                        9000,
                    )
                    .await
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            let result = handle.await.expect("Task should not panic");
            results.push(result);
        }

        assert_eq!(results.len(), 10);
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_ok());
            assert_eq!(
                result.as_ref().unwrap(),
                &format!("http://localhost:{}", 5000 + i)
            );
        }
    }
}
