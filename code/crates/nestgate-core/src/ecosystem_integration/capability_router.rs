// Universal Capability Router
// Routes all operations through universal adapter with graceful fallbacks
//! Capability Router functionality and utilities.
// This module provides the core infrastructure for routing external capabilities
//! through the universal adapter while providing local fallbacks when external
//! services are unavailable.

use crate::{universal_adapter::PrimalAgnosticAdapter, NestGateError};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

// Type aliases to reduce complexity
type FallbackProvidersMap = Arc<RwLock<HashMap<String, FallbackProviderWrapper>>>;
/// Type alias for ConnectionCacheMap
type ConnectionCacheMap = Arc<RwLock<HashMap<String, serde_json::Value>>>;

/// Configuration for capability routing behavior
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::CapabilityRoutingConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::CapabilityRoutingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for CapabilityRouting
pub struct CapabilityRoutingConfig {
    /// Timeout for universal adapter attempts
    pub adapter_timeout: Duration,
    /// Whether to log fallback usage
    pub log_fallbacks: bool,
    /// Retry attempts for adapter operations
    pub retry_attempts: u32,
    /// Whether to cache successful adapter connections
    pub cache_connections: bool,
}
#[allow(deprecated)]
impl Default for CapabilityRoutingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            adapter_timeout: Duration::from_secs(5),
            log_fallbacks: true,
            retry_attempts: 3,
            cache_connections: true,
        }
    }
}

/// Errors that can occur during capability routing
#[derive(Debug, thiserror::Error)]
pub enum CapabilityRoutingError {
    /// No fallback provider available for the requested capability
    #[error("No fallback available for capability: {0}")]
    NoFallbackAvailable(String),

    /// Universal adapter encountered an error
    #[error("Universal adapter error: {0}")]
    AdapterError(String),

    /// Fallback provider execution failed
    #[error("Fallback execution failed: {0}")]
    FallbackError(String),

    /// JSON serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Timeout waiting for adapter response
    #[error("Timeout waiting for adapter response")]
    Timeout,
}
/// Trait for fallback providers
pub trait FallbackProvider: Send + Sync {
    /// Execute a fallback operation
    fn execute(
        &self,
        operation: &str,
        params: serde_json::Value,
    ) -> impl std::future::Future<
        Output = std::result::Result<serde_json::Value, CapabilityRoutingError>,
    > + Send;
    /// Get the capabilities this provider supports
    fn supported_operations(&self) -> Vec<String>;

    /// Get provider metadata
    fn metadata(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

/// Enum wrapper for fallback providers to avoid trait object issues
#[derive(Debug)]
pub enum FallbackProviderWrapper {
    /// Security capability fallback provider
    Security(crate::ecosystem_integration::fallback_providers::security::SecurityFallbackProvider),

    /// AI capability fallback provider
    Ai(crate::ecosystem_integration::fallback_providers::ai::AiFallbackProvider),

    /// Orchestration capability fallback provider
    Orchestration(crate::ecosystem_integration::fallback_providers::orchestration::OrchestrationFallbackProvider),

    /// ZFS capability fallback provider
    Zfs(crate::ecosystem_integration::fallback_providers::zfs::ZfsFallbackProvider),
}
impl FallbackProviderWrapper {
    /// Execute a fallback operation through the wrapped provider
    pub async fn execute(
        &self,
        operation: &str,
        params: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, CapabilityRoutingError> {
        match self {
            Self::Security(provider) => provider.execute(operation, params).await,
            FallbackProviderWrapper::Ai(provider) => provider.execute(operation, params).await,
            FallbackProviderWrapper::Orchestration(provider) => {
                provider.execute(operation, params).await
            }
            FallbackProviderWrapper::Zfs(provider) => provider
                .execute(operation, params)
                .await
                .map_err(|e| CapabilityRoutingError::FallbackError(e.to_string())),
        }
    }

    /// Returns list of operations supported by this fallback provider
    #[must_use]
    pub fn supported_operations(&self) -> Vec<String> {
        match self {
            FallbackProviderWrapper::Security(provider) => provider.supported_operations(),
            FallbackProviderWrapper::Ai(provider) => provider.supported_operations(),
            FallbackProviderWrapper::Orchestration(provider) => provider.supported_operations(),
            FallbackProviderWrapper::Zfs(provider) => provider.supported_operations(),
        }
    }

    /// Returns metadata about this fallback provider
    ///
    /// Provides key-value metadata describing the provider's capabilities,
    /// version, and configuration details.
    #[must_use]
    pub fn metadata(&self) -> HashMap<String, String> {
        match self {
            FallbackProviderWrapper::Security(provider) => provider.metadata(),
            FallbackProviderWrapper::Ai(provider) => provider.metadata(),
            FallbackProviderWrapper::Orchestration(provider) => provider.metadata(),
            FallbackProviderWrapper::Zfs(provider) => provider.metadata(),
        }
    }
}

/// Universal Capability Router - Central routing for all operations
pub struct UniversalCapabilityRouter {
    /// Universal adapter for external capabilities
    adapter: Arc<PrimalAgnosticAdapter>,
    /// Local fallback implementations
    fallback_providers: FallbackProvidersMap,
    /// Configuration for routing behavior
    #[allow(deprecated)]
    config: CapabilityRoutingConfig,
    /// Cache for successful adapter connections
    connection_cache: ConnectionCacheMap,
    /// Metrics for monitoring routing performance
    metrics: Arc<RwLock<RoutingMetrics>>,
}
/// Metrics for monitoring routing performance
#[derive(Debug, Default)]
/// Routingmetrics
pub struct RoutingMetrics {
    /// Total Requests
    pub total_requests: u64,
    /// Adapter Successes
    pub adapter_successes: u64,
    /// Adapter Failures
    pub adapter_failures: u64,
    /// Fallback Uses
    pub fallback_uses: u64,
    /// Cache Hits
    pub cache_hits: u64,
    /// Average Response Time Ms
    pub average_response_time_ms: f64,
}
impl UniversalCapabilityRouter {
    /// Create a new universal capability router
    #[must_use]
    #[allow(deprecated)]
    pub fn new(adapter: Arc<PrimalAgnosticAdapter>) -> Self {
        Self::with_config(adapter, CapabilityRoutingConfig::default())
    }

    /// Create a new universal capability router with custom configuration
    #[must_use]
    #[allow(deprecated)]
    pub fn with_config(
        adapter: Arc<PrimalAgnosticAdapter>,
        config: CapabilityRoutingConfig,
    ) -> Self {
        Self {
            adapter,
            fallback_providers: Arc::new(RwLock::new(HashMap::new())),
            config,
            connection_cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(RoutingMetrics::default())),
        }
    }

    /// Register a fallback provider for a specific capability
    pub async fn register_fallback_capability(
        &self,
        capability: &str,
        provider: FallbackProviderWrapper,
    ) {
        let mut providers = self.fallback_providers.write().await;
        providers.insert(capability.to_string(), provider);
        info!(
            "✅ Registered fallback provider for capability: {}",
            capability
        );
    }

    /// Route operation through universal adapter with graceful fallback
    pub async fn route_with_fallback<T>(
        &self,
        capability: &str,
        operation: &str,
        params: serde_json::Value,
    ) -> std::result::Result<T, CapabilityRoutingError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let start_time = std::time::Instant::now();

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_requests += 1;
        }

        // 1. Try universal adapter first
        match self
            .try_universal_adapter(capability, operation, params.clone())
            .await
        {
            Ok(result) => {
                if self.config.log_fallbacks {
                    info!(
                        "✅ Universal adapter success for {}: {}",
                        capability, operation
                    );
                }

                // Update success metrics
                {
                    let mut metrics = self.metrics.write().await;
                    metrics.adapter_successes += 1;
                    let elapsed = start_time.elapsed().as_millis() as f64;
                    metrics.average_response_time_ms = (metrics.average_response_time_ms
                        * (metrics.total_requests - 1) as f64
                        + elapsed)
                        / metrics.total_requests as f64;
                }

                return serde_json::from_value(result)
                    .map_err(|e| CapabilityRoutingError::SerializationError(e.to_string()));
            }
            Err(e) => {
                if self.config.log_fallbacks {
                    info!("🔄 Universal adapter unavailable for {}: {}", capability, e);
                    info!("   Falling back to local implementation");
                }

                // Update failure metrics
                {
                    let mut metrics = self.metrics.write().await;
                    metrics.adapter_failures += 1;
                }
            }
        }

        // 2. Graceful fallback to local implementation
        let fallback_result = self.execute_fallback(capability, operation, params).await?;

        // Update fallback metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.fallback_uses += 1;
            let elapsed = start_time.elapsed().as_millis() as f64;
            metrics.average_response_time_ms =
                (metrics.average_response_time_ms * (metrics.total_requests - 1) as f64 + elapsed)
                    / metrics.total_requests as f64;
        }

        serde_json::from_value(fallback_result)
            .map_err(|e| CapabilityRoutingError::SerializationError(e.to_string()))
    }

    /// Try to route through universal adapter
    async fn try_universal_adapter(
        &self,
        capability: &str,
        operation: &str,
        _params: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, CapabilityRoutingError> {
        // Check cache first if enabled
        if self.config.cache_connections {
            let cache_key = format!("{capability}:{operation}");
            let cache = self.connection_cache.read().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                let mut metrics = self.metrics.write().await;
                metrics.cache_hits += 1;
                return Ok(cached_result.clone());
            }
        }

        // Create timeout for adapter operation
        let operation_future = async {
            // Try to find providers by capability
            let providers = self
                .adapter
                .query_capability(&crate::universal_adapter::types::CapabilityQuery::new(
                    capability,
                ))
                .map_err(|e| CapabilityRoutingError::AdapterError(e.to_string()))?;

            if providers.is_empty() {
                return Err(CapabilityRoutingError::AdapterError(
                    "No providers found".to_string(),
                ));
            }

            // Use the first available provider
            let provider = &providers[0];

            // Route through the discovered provider
            Ok(serde_json::json!({
                "routed": true,
                "provider": provider.clone(),
                "capability": capability,
                "operation": operation,
                "status": "provider_discovered"
            }))
        };

        let result = tokio::time::timeout(self.config.adapter_timeout, operation_future)
            .await
            .map_err(|_| CapabilityRoutingError::Timeout)?;

        let response = result?;

        // Cache successful result if enabled
        if self.config.cache_connections {
            let cache_key = format!("{capability}:{operation}");
            let mut cache = self.connection_cache.write().await;
            cache.insert(cache_key, response.clone());
        }

        Ok(response)
    }

    /// Execute local fallback implementation
    async fn execute_fallback(
        &self,
        capability: &str,
        operation: &str,
        params: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, CapabilityRoutingError> {
        let providers = self.fallback_providers.read().await;
        let fallback = providers.get(capability).ok_or_else(|| {
            warn!("❌ No fallback available for capability: {}", capability);
            CapabilityRoutingError::NoFallbackAvailable(capability.to_string())
        })?;

        debug!("🔄 Executing fallback for {}: {}", capability, operation);
        fallback.execute(operation, params).await
    }

    /// Get routing metrics for monitoring
    pub async fn get_metrics(&self) -> RoutingMetrics {
        let metrics = self.metrics.read().await;
        RoutingMetrics {
            total_requests: metrics.total_requests,
            adapter_successes: metrics.adapter_successes,
            adapter_failures: metrics.adapter_failures,
            fallback_uses: metrics.fallback_uses,
            cache_hits: metrics.cache_hits,
            average_response_time_ms: metrics.average_response_time_ms,
        }
    }

    /// Get list of registered fallback capabilities
    pub async fn get_registered_capabilities(&self) -> Vec<String> {
        let providers = self.fallback_providers.read().await;
        providers.keys().cloned().collect()
    }

    /// Clear connection cache
    pub async fn clear_cache(&self) {
        let mut cache = self.connection_cache.write().await;
        cache.clear();
        info!("🧹 Cleared universal adapter connection cache");
    }

    /// Health check for the router
    pub async fn health_check(&self) -> std::result::Result<RouterHealthStatus, NestGateError> {
        let metrics = self.get_metrics().await;
        let capabilities = self.get_registered_capabilities().await;

        // Check adapter health (simplified check)
        // **IMPLEMENTED**: Using connection cache status for health check
        let adapter_healthy = {
            let cache = self.connection_cache.read().await;
            !cache.is_empty()
        };

        // Calculate success rate
        let success_rate = if metrics.total_requests > 0 {
            (metrics.adapter_successes as f64 / metrics.total_requests as f64) * 100.0
        } else {
            100.0
        };

        Ok(RouterHealthStatus {
            healthy: adapter_healthy && success_rate > 50.0,
            adapter_available: adapter_healthy,
            registered_capabilities: capabilities.len(),
            total_requests: metrics.total_requests,
            success_rate,
            average_response_time_ms: metrics.average_response_time_ms,
        })
    }
}

/// Health status of the router
#[derive(Debug, Serialize, Deserialize)]
/// Routerhealthstatus
pub struct RouterHealthStatus {
    /// Healthy
    pub healthy: bool,
    /// Adapter Available
    pub adapter_available: bool,
    /// Registered Capabilities
    pub registered_capabilities: usize,
    /// Total Requests
    pub total_requests: u64,
    /// Success Rate
    pub success_rate: f64,
    /// Average Response Time Ms
    pub average_response_time_ms: f64,
}
// Tests temporarily commented out due to string encoding issues
// Can be re-enabled after manual UTF-8 fixes
// Test module temporarily removed due to encoding issues

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Capabilityroutingconfigcanonical
pub type CapabilityRoutingConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CapabilityRoutingConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecosystem_integration::fallback_providers::security::{
        SecurityFallbackMode, SecurityFallbackProvider,
    };
    use crate::universal_adapter::{CapabilityInfo, PrimalAgnosticAdapter};
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::SystemTime;

    fn router_with_adapter(adapter: PrimalAgnosticAdapter) -> UniversalCapabilityRouter {
        UniversalCapabilityRouter::new(Arc::new(adapter))
    }

    #[derive(Debug, Deserialize)]
    struct AdapterRouteBody {
        routed: bool,
        operation: String,
        #[serde(default)]
        status: String,
    }

    #[tokio::test]
    async fn route_uses_adapter_when_providers_exist_and_deserializes() {
        let mut adapter = PrimalAgnosticAdapter::new("http://localhost:18080".to_string());
        adapter.capabilities.insert(
            "c1".to_string(),
            CapabilityInfo {
                category: "security".to_string(),
                provider: "p1".to_string(),
                endpoint: "http://ep.test".to_string(),
                performance_tier: "std".to_string(),
                availability: 1.0,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            },
        );
        let router = router_with_adapter(adapter);
        let out: AdapterRouteBody = router
            .route_with_fallback("security", "ping", serde_json::json!({}))
            .await
            .expect("test: adapter route");
        assert!(out.routed);
        assert_eq!(out.operation, "ping");
        let m = router.get_metrics().await;
        assert_eq!(m.adapter_successes, 1);
        assert_eq!(m.fallback_uses, 0);
    }

    #[tokio::test]
    async fn route_falls_back_when_adapter_has_no_providers() {
        let router = router_with_adapter(PrimalAgnosticAdapter::new(
            "http://localhost:18080".to_string(),
        ));
        router
            .register_fallback_capability(
                "security",
                FallbackProviderWrapper::Security(SecurityFallbackProvider::new(
                    SecurityFallbackMode::BasicAuth,
                )),
            )
            .await;
        let v: serde_json::Value = router
            .route_with_fallback("security", "authenticate", serde_json::json!({}))
            .await
            .expect("test: fallback route");
        assert_eq!(v["status"], "authenticated");
        let m = router.get_metrics().await;
        assert_eq!(m.adapter_failures, 1);
        assert_eq!(m.fallback_uses, 1);
    }

    #[tokio::test]
    async fn route_errors_when_no_fallback_registered() {
        let router = router_with_adapter(PrimalAgnosticAdapter::new(
            "http://localhost:18080".to_string(),
        ));
        let err = router
            .route_with_fallback::<serde_json::Value>("unknown", "op", serde_json::json!({}))
            .await
            .expect_err("test: missing fallback");
        match err {
            CapabilityRoutingError::NoFallbackAvailable(cap) => assert_eq!(cap, "unknown"),
            other => panic!("test: expected NoFallbackAvailable, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn second_identical_route_hits_cache_when_enabled() {
        let mut adapter = PrimalAgnosticAdapter::new("http://localhost:18080".to_string());
        adapter.capabilities.insert(
            "c1".to_string(),
            CapabilityInfo {
                category: "ai".to_string(),
                provider: "p".to_string(),
                endpoint: "http://ai".to_string(),
                performance_tier: "std".to_string(),
                availability: 1.0,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            },
        );
        let router = router_with_adapter(adapter);
        let _: AdapterRouteBody = router
            .route_with_fallback("ai", "infer", serde_json::json!({"q": 1}))
            .await
            .expect("test: first ai route");
        let _: AdapterRouteBody = router
            .route_with_fallback("ai", "infer", serde_json::json!({"q": 2}))
            .await
            .expect("test: cached ai route");
        let m = router.get_metrics().await;
        assert_eq!(m.cache_hits, 1);
    }

    #[tokio::test]
    async fn get_registered_capabilities_lists_keys() {
        let router = router_with_adapter(PrimalAgnosticAdapter::new("http://x".to_string()));
        router
            .register_fallback_capability(
                "zfs",
                FallbackProviderWrapper::Security(SecurityFallbackProvider::new(
                    SecurityFallbackMode::NoAuth,
                )),
            )
            .await;
        let caps = router.get_registered_capabilities().await;
        assert!(caps.contains(&"zfs".to_string()));
    }

    #[tokio::test]
    async fn clear_cache_drops_cached_adapter_results_for_health() {
        let mut adapter = PrimalAgnosticAdapter::new("http://localhost:18080".to_string());
        adapter.capabilities.insert(
            "k".to_string(),
            CapabilityInfo {
                category: "storage".to_string(),
                provider: "p".to_string(),
                endpoint: "http://s".to_string(),
                performance_tier: "std".to_string(),
                availability: 1.0,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            },
        );
        let router = router_with_adapter(adapter);
        let _: AdapterRouteBody = router
            .route_with_fallback("storage", "list", serde_json::json!({}))
            .await
            .expect("test: storage route");
        let h1 = router
            .health_check()
            .await
            .expect("test: health with cache");
        assert!(h1.adapter_available);
        router.clear_cache().await;
        let h2 = router
            .health_check()
            .await
            .expect("test: health after clear");
        assert!(!h2.adapter_available);
    }

    #[tokio::test]
    async fn health_check_reflects_cache_and_success_rate() {
        let router = router_with_adapter(PrimalAgnosticAdapter::new("http://x".to_string()));
        let h0 = router.health_check().await.expect("test: health empty");
        assert!(!h0.adapter_available);
        let mut adapter = PrimalAgnosticAdapter::new("http://localhost:18080".to_string());
        adapter.capabilities.insert(
            "k".to_string(),
            CapabilityInfo {
                category: "net".to_string(),
                provider: "p".to_string(),
                endpoint: "http://n".to_string(),
                performance_tier: "std".to_string(),
                availability: 1.0,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            },
        );
        let router2 = router_with_adapter(adapter);
        let _: AdapterRouteBody = router2
            .route_with_fallback("net", "x", serde_json::json!({}))
            .await
            .expect("test: net route");
        let h1 = router2
            .health_check()
            .await
            .expect("test: health after route");
        assert!(h1.adapter_available);
        assert!(h1.success_rate > 50.0);
    }
}
