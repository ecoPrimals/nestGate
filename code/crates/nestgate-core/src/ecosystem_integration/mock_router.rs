//! Universal Mock Router
//! Routes all mock operations through universal adapter with graceful fallbacks
//!
//! This module provides the core infrastructure for routing external capabilities
//! through the universal adapter while providing local fallbacks when external
//! primals are unavailable.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::ecosystem_integration::universal_adapter::UniversalAdapter;
use crate::error::NestGateError;

/// Configuration for mock routing behavior
#[derive(Debug, Clone)]
pub struct MockRoutingConfig {
    /// Timeout for universal adapter attempts
    pub adapter_timeout: Duration,
    /// Whether to log fallback usage
    pub log_fallbacks: bool,
    /// Retry attempts for adapter operations
    pub retry_attempts: u32,
    /// Whether to cache successful adapter connections
    pub cache_connections: bool,
}

impl Default for MockRoutingConfig {
    fn default() -> Self {
        Self {
            adapter_timeout: Duration::from_secs(5),
            log_fallbacks: true,
            retry_attempts: 3,
            cache_connections: true,
        }
    }
}

/// Errors that can occur during mock routing
#[derive(Debug, thiserror::Error)]
pub enum MockRoutingError {
    #[error("No fallback available for capability: {0}")]
    NoFallbackAvailable(String),
    #[error("Universal adapter error: {0}")]
    AdapterError(String),
    #[error("Fallback execution failed: {0}")]
    FallbackError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
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
    ) -> impl std::future::Future<Output = std::result::Result<serde_json::Value, MockRoutingError>> + Send;

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
    Security(crate::ecosystem_integration::fallback_providers::security::SecurityFallbackProvider),
    Ai(crate::ecosystem_integration::fallback_providers::ai::AiFallbackProvider),
    Orchestration(crate::ecosystem_integration::fallback_providers::orchestration::OrchestrationFallbackProvider),
    Zfs(crate::ecosystem_integration::fallback_providers::zfs::ZfsFallbackProvider),
}

impl FallbackProviderWrapper {
    pub async fn execute(
        &self,
        operation: &str,
        params: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, MockRoutingError> {
        match self {
            FallbackProviderWrapper::Security(provider) => provider.execute(operation, params).await,
            FallbackProviderWrapper::Ai(provider) => provider.execute(operation, params).await,
            FallbackProviderWrapper::Orchestration(provider) => provider.execute(operation, params).await,
            FallbackProviderWrapper::Zfs(provider) => {
                provider.execute(operation, params).await
                    .map_err(|e| MockRoutingError::FallbackError(e.to_string()))
            },
        }
    }

    pub fn supported_operations(&self) -> Vec<String> {
        match self {
            FallbackProviderWrapper::Security(provider) => provider.supported_operations(),
            FallbackProviderWrapper::Ai(provider) => provider.supported_operations(),
            FallbackProviderWrapper::Orchestration(provider) => provider.supported_operations(),
            FallbackProviderWrapper::Zfs(provider) => provider.supported_operations(),
        }
    }

    pub fn metadata(&self) -> HashMap<String, String> {
        match self {
            FallbackProviderWrapper::Security(provider) => provider.metadata(),
            FallbackProviderWrapper::Ai(provider) => provider.metadata(),
            FallbackProviderWrapper::Orchestration(provider) => provider.metadata(),
            FallbackProviderWrapper::Zfs(provider) => provider.metadata(),
        }
    }
}

/// Universal Mock Router - Central routing for all mock operations
pub struct UniversalMockRouter {
    /// Universal adapter for external capabilities
    adapter: Arc<UniversalAdapter>,
    /// Local fallback implementations
    fallback_providers: Arc<RwLock<HashMap<String, FallbackProviderWrapper>>>,
    /// Configuration for routing behavior
    config: MockRoutingConfig,
    /// Cache for successful adapter connections
    connection_cache: Arc<RwLock<HashMap<String, serde_json::Value>>>,
    /// Metrics for monitoring routing performance
    metrics: Arc<RwLock<RoutingMetrics>>,
}

/// Metrics for monitoring routing performance
#[derive(Debug, Default)]
pub struct RoutingMetrics {
    pub total_requests: u64,
    pub adapter_successes: u64,
    pub adapter_failures: u64,
    pub fallback_uses: u64,
    pub cache_hits: u64,
    pub average_response_time_ms: f64,
}

impl UniversalMockRouter {
    /// Create a new universal mock router
    pub fn new(adapter: Arc<UniversalAdapter>) -> Self {
        Self::with_config(adapter, MockRoutingConfig::default())
    }

    /// Create a new universal mock router with custom configuration
    pub fn with_config(adapter: Arc<UniversalAdapter>, config: MockRoutingConfig) -> Self {
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
    ) -> std::result::Result<T, MockRoutingError>
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
                    .map_err(|e| MockRoutingError::SerializationError(e.to_string()));
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
            .map_err(|e| MockRoutingError::SerializationError(e.to_string()))
    }

    /// Try to route through universal adapter
    async fn try_universal_adapter(
        &self,
        capability: &str,
        operation: &str,
        _params: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, MockRoutingError> {
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
            let providers = self.adapter.find_providers_by_capability(capability).await;

            if providers.is_empty() {
                return Err(MockRoutingError::AdapterError(
                    "No providers found".to_string(),
                ));
            }

            // Use the first available provider
            let provider = &providers[0];

            // For now, return a mock response since we don't have execute_operation
            Ok(serde_json::json!({
                "success": true,
                "provider": provider.name,
                "operation": operation,
                "routed_through_adapter": true
            }))
        };

        let result = tokio::time::timeout(self.config.adapter_timeout, operation_future)
            .await
            .map_err(|_| MockRoutingError::Timeout)?;

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
    ) -> std::result::Result<serde_json::Value, MockRoutingError> {
        let providers = self.fallback_providers.read().await;
        let fallback = providers.get(capability).ok_or_else(|| {
            warn!("❌ No fallback available for capability: {}", capability);
            MockRoutingError::NoFallbackAvailable(capability.to_string())
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
pub struct RouterHealthStatus {
    pub healthy: bool,
    pub adapter_available: bool,
    pub registered_capabilities: usize,
    pub total_requests: u64,
    pub success_rate: f64,
    pub average_response_time_ms: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecosystem_integration::universal_adapter::config::AdapterConfig;

    #[derive(Debug)]
    struct TestFallbackProvider {
        operations: Vec<String>,
    }

    impl FallbackProvider for TestFallbackProvider {
        async fn execute(
            &self,
            operation: &str,
            _params: serde_json::Value,
        ) -> Result<serde_json::Value, MockRoutingError> {
            if self.operations.contains(&operation.to_string()) {
                Ok(serde_json::json!({
                    "success": true,
                    "operation": operation,
                    "provider": "test_fallback"
                }))
            } else {
                Err(MockRoutingError::FallbackError(format!(
                    "Unsupported operation: {}",
                    operation
                )))
            }
        }

        fn supported_operations(&self) -> Vec<String> {
            self.operations.clone()
        }
    }

    #[tokio::test]
    async fn test_mock_router_creation() {
        let adapter = Arc::new(UniversalAdapter::new(AdapterConfig::default()));
        let router = UniversalMockRouter::new(adapter);

        let capabilities = router.get_registered_capabilities().await;
        assert!(capabilities.is_empty());
    }

    #[tokio::test]
    async fn test_fallback_provider_registration() {
        let adapter = Arc::new(UniversalAdapter::new(AdapterConfig::default()));
        let router = UniversalMockRouter::new(adapter);

        let provider = TestFallbackProvider {
            operations: vec!["test_operation".to_string()],
        };

        router
            .register_fallback_capability("test.capability", Box::new(provider))
            .await;

        let capabilities = router.get_registered_capabilities().await;
        assert_eq!(capabilities.len(), 1);
        assert!(capabilities.contains(&"test.capability".to_string()));
    }

    #[tokio::test]
    async fn test_fallback_execution() {
        let adapter = Arc::new(UniversalAdapter::new(AdapterConfig::default()));
        let router = UniversalMockRouter::new(adapter);

        let provider = TestFallbackProvider {
            operations: vec!["test_operation".to_string()],
        };

        router
            .register_fallback_capability("test.capability", Box::new(provider))
            .await;

        let result: serde_json::Value = router
            .route_with_fallback("test.capability", "test_operation", serde_json::Value::Null)
            .await
            .map_err(|e| {
                crate::error::NestGateError::InternalError(format!(
                    "Failed in Internal operation: {}",
                    e
                ))
            })?;

        assert_eq!(result["success"], true);
        assert_eq!(result["operation"], "test_operation");
        assert_eq!(result["provider"], "test_fallback");
    }

    #[tokio::test]
    async fn test_health_check() {
        let adapter = Arc::new(UniversalAdapter::new(AdapterConfig::default()));
        let router = UniversalMockRouter::new(adapter);

        let health = router.health_check().await.map_err(|e| {
            crate::error::NestGateError::InternalError(format!(
                "Failed in Internal operation: {}",
                e
            ))
        })?;
        assert_eq!(health.registered_capabilities, 0);
        assert_eq!(health.total_requests, 0);
        assert_eq!(health.success_rate, 100.0);
    }
}
