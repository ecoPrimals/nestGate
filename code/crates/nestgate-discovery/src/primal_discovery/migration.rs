// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// Discovery Migration Helper
//
// Provides gradual migration from hardcoded values to capability-based discovery
// with robust fallback chains.

// HTTP removed — use orchestration / network capability discovery for external HTTP
use crate::primal_discovery::PrimalDiscovery;
use nestgate_types::error::{NestGateError, Result};
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
/// // Requires Arc<PrimalDiscovery> and reqwest; see tests for full usage
/// use nestgate_core::primal_discovery::migration::DiscoveryOrEnv;
/// let helper = DiscoveryOrEnv::new(discovery);
/// let auth_endpoint = helper.endpoint_for("security", "AUTH_SERVICE_PORT", 5000).await?;
/// ```
pub struct DiscoveryOrEnv {
    #[allow(
        dead_code,
        reason = "Reserved for capability-based resolution; endpoint helpers use cache/env only for now"
    )]
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
    #[allow(
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
                format!("Could not resolve endpoint for capability: {capability}"),
            )),
        }
    }

    /// Try discovery (currently returns None, reserved for async integration)
    const fn try_discovery_sync(&self, _capability: &str) -> Option<(String, EndpointSource)> {
        // Reserved for future async discovery integration
        // Currently using environment and defaults only
        None
    }

    /// Try environment variable
    fn try_environment(&self, env_var: &str) -> Option<(String, EndpointSource)> {
        std::env::var(env_var).ok().and_then(|val| {
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
                    let url = format!("http://localhost:{port}");
                    debug!("Found {} in environment: {}", env_var, url);
                    Some((url, EndpointSource::Environment))
                },
            )
        })
    }

    /// Try default (development fallback)
    fn try_default(&self, port: u16) -> Option<(String, EndpointSource)> {
        if self.config.allow_defaults {
            let url = format!("http://localhost:{port}");
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
        let orig = std::env::var("TEST_SERVICE_PORT").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("TEST_SERVICE_PORT", "7777");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let endpoint = helper
            .endpoint_for("nonexistent", "TEST_SERVICE_PORT", 8888)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("TEST_SERVICE_PORT", v),
            None => nestgate_platform::env_process::remove_var("TEST_SERVICE_PORT"),
        }
        assert_eq!(endpoint, "http://localhost:7777");
    }

    #[tokio::test]
    async fn test_default_fallback() {
        let orig = std::env::var("NONEXISTENT_PORT").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::remove_var("NONEXISTENT_PORT");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let endpoint = helper
            .endpoint_for("nonexistent", "NONEXISTENT_PORT", 6666)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("NONEXISTENT_PORT", v),
            None => {}
        }
        assert_eq!(endpoint, "http://localhost:6666");
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let orig = std::env::var("TEST_PORT").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("TEST_PORT", "5555");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let endpoint1 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();

        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("TEST_PORT", "3333");

        let endpoint2 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();

        helper.invalidate("test").await;

        let endpoint3 = helper
            .endpoint_for("test", "TEST_PORT", 4444)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("TEST_PORT", v),
            None => nestgate_platform::env_process::remove_var("TEST_PORT"),
        }
        assert_eq!(endpoint1, endpoint2);
        assert_eq!(endpoint3, "http://localhost:3333");
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
        let orig = std::env::var("INVALID_PORT").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("INVALID_PORT", "99999");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let endpoint = helper
            .endpoint_for("test_service", "INVALID_PORT", 8080)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("INVALID_PORT", v),
            None => nestgate_platform::env_process::remove_var("INVALID_PORT"),
        }
        assert_eq!(endpoint, "http://localhost:8080");
    }

    #[tokio::test]
    async fn test_full_url_in_environment_variable() {
        let orig = std::env::var("SERVICE_URL").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("SERVICE_URL", "https://api.example.com:8443");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let endpoint = helper
            .endpoint_for("test_service", "SERVICE_URL", 8080)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("SERVICE_URL", v),
            None => nestgate_platform::env_process::remove_var("SERVICE_URL"),
        }
        assert_eq!(endpoint, "https://api.example.com:8443");
    }

    #[tokio::test]
    async fn test_cache_expiration_via_invalidation() {
        let orig = std::env::var("TEST_TTL_PORT").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("TEST_TTL_PORT", "5000");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let endpoint1 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();

        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("TEST_TTL_PORT", "6000");

        let endpoint2 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();

        helper.invalidate("ttl_test").await;

        let endpoint3 = helper
            .endpoint_for("ttl_test", "TEST_TTL_PORT", 9000)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("TEST_TTL_PORT", v),
            None => nestgate_platform::env_process::remove_var("TEST_TTL_PORT"),
        }
        assert_eq!(endpoint1, "http://localhost:5000");
        assert_eq!(endpoint2, "http://localhost:5000");
        assert_eq!(endpoint3, "http://localhost:6000");
    }

    #[tokio::test]
    async fn test_prefer_environment_over_discovery() {
        let orig = std::env::var("PRIORITY_PORT").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("PRIORITY_PORT", "7000");

        let discovery = create_test_discovery();
        let mut config = MigrationConfig::default();
        config.prefer_environment = true;
        let helper = DiscoveryOrEnv::with_config(discovery, config);

        let endpoint = helper
            .endpoint_for("test", "PRIORITY_PORT", 8000)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("PRIORITY_PORT", v),
            None => nestgate_platform::env_process::remove_var("PRIORITY_PORT"),
        }
        assert_eq!(endpoint, "http://localhost:7000");
    }

    #[tokio::test]
    async fn test_clear_cache_removes_all_entries() {
        let orig1 = std::env::var("SERVICE1_PORT").ok();
        let orig2 = std::env::var("SERVICE2_PORT").ok();
        let orig3 = std::env::var("SERVICE3_PORT").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("SERVICE1_PORT", "5001");
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("SERVICE2_PORT", "5002");
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("SERVICE3_PORT", "5003");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let _ = helper.endpoint_for("service1", "SERVICE1_PORT", 9000).await;
        let _ = helper.endpoint_for("service2", "SERVICE2_PORT", 9000).await;
        let _ = helper.endpoint_for("service3", "SERVICE3_PORT", 9000).await;

        helper.clear_cache().await;

        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("SERVICE1_PORT", "6001");
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("SERVICE2_PORT", "6002");
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("SERVICE3_PORT", "6003");

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

        match orig1 {
            Some(v) => nestgate_platform::env_process::set_var("SERVICE1_PORT", v),
            None => nestgate_platform::env_process::remove_var("SERVICE1_PORT"),
        }
        match orig2 {
            Some(v) => nestgate_platform::env_process::set_var("SERVICE2_PORT", v),
            None => nestgate_platform::env_process::remove_var("SERVICE2_PORT"),
        }
        match orig3 {
            Some(v) => nestgate_platform::env_process::set_var("SERVICE3_PORT", v),
            None => nestgate_platform::env_process::remove_var("SERVICE3_PORT"),
        }
        assert_eq!(e1, "http://localhost:6001");
        assert_eq!(e2, "http://localhost:6002");
        assert_eq!(e3, "http://localhost:6003");
    }

    #[tokio::test]
    async fn test_empty_environment_variable_falls_back() {
        let orig = std::env::var("EMPTY_VAR").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("EMPTY_VAR", "");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let endpoint = helper
            .endpoint_for("test", "EMPTY_VAR", 7070)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("EMPTY_VAR", v),
            None => nestgate_platform::env_process::remove_var("EMPTY_VAR"),
        }
        assert_eq!(endpoint, "http://localhost:7070");
    }

    #[tokio::test]
    async fn test_malformed_url_in_environment_falls_back() {
        let orig = std::env::var("MALFORMED_URL").ok();
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("MALFORMED_URL", "not-a-valid-url");

        let discovery = create_test_discovery();
        let helper = DiscoveryOrEnv::new(discovery);

        let endpoint = helper
            .endpoint_for("test", "MALFORMED_URL", 8181)
            .await
            .unwrap();

        match orig {
            Some(v) => nestgate_platform::env_process::set_var("MALFORMED_URL", v),
            None => nestgate_platform::env_process::remove_var("MALFORMED_URL"),
        }
        assert_eq!(endpoint, "http://localhost:8181");
    }

    #[tokio::test]
    async fn test_concurrent_endpoint_resolution() {
        let saved: Vec<(String, Option<String>)> = (0..10)
            .map(|i| {
                let k = format!("CONCURRENT_PORT_{}", i);
                (k.clone(), std::env::var(&k).ok())
            })
            .collect();

        for i in 0..10 {
            // SAFETY: single-threaded test context.
            nestgate_platform::env_process::set_var(
                format!("CONCURRENT_PORT_{}", i),
                format!("{}", 5000 + i),
            );
        }

        let discovery = create_test_discovery();
        let helper = Arc::new(DiscoveryOrEnv::new(discovery));

        let mut handles = vec![];
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

        let mut results = vec![];
        for handle in handles {
            let result = handle.await.expect("Task should not panic");
            results.push(result);
        }

        for (k, v) in &saved {
            match v {
                Some(val) => nestgate_platform::env_process::set_var(k, val),
                None => nestgate_platform::env_process::remove_var(k),
            }
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
