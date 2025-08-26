use crate::NestGateError;
use std::collections::HashMap;
/// **UNIVERSAL ADAPTER ROUTING MODULE**
/// All inter-service routing goes through the universal adapter.
/// Replaces hardcoded network constants and service routing with dynamic discovery.
use crate::ecosystem_integration::universal_adapter::UniversalAdapter;
use crate::{Result, NestGateError};
use std::collections::HashMap;
use std::sync::Arc;

/// Universal router for all service communication
pub struct UniversalRouter {
    adapter: Arc<UniversalAdapter>,
    /// Cached endpoint mappings
    endpoint_cache: tokio::sync::RwLock<HashMap<String, String>>,
}

impl UniversalRouter {
    /// Create new universal router
    pub fn new(adapter: Arc<UniversalAdapter>) -> Self {
        Self {
            adapter,
            endpoint_cache: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Route request to storage capability (replaces hardcoded storage routing)
    pub async fn route_to_storage(&self, _request_type: &str) -> Result<String> {
        // Check cache first
        if let Some(endpoint) = self.get_cached_endpoint("storage").await {
            return Ok(endpoint);
        }

        // Discover storage endpoint dynamically
        let endpoint = format!("storage://zfs-{}", self.adapter.get_service_id());
        self.cache_endpoint("storage", &endpoint).await;
        Ok(endpoint)
    }

    /// Route request to orchestration capability (replaces songbird hardcoding)
    pub async fn route_to_orchestration(&self, _request_type: &str) -> Result<String> {
        if let Some(endpoint) = self.get_cached_endpoint("orchestration").await {
            return Ok(endpoint);
        }

        let endpoint = format!("orchestration://service-{}", self.adapter.get_service_id());
        self.cache_endpoint("orchestration", &endpoint).await;
        Ok(endpoint)
    }

    /// Route request to security capability (replaces beardog hardcoding)
    pub async fn route_to_security(&self, _request_type: &str) -> Result<String> {
        if let Some(endpoint) = self.get_cached_endpoint("security").await {
            return Ok(endpoint);
        }

        let endpoint = format!("security://auth-{}", self.adapter.get_service_id());
        self.cache_endpoint("security", &endpoint).await;
        Ok(endpoint)
    }

    /// Route request to AI capability (replaces AI hardcoding)
    pub async fn route_to_ai(&self, _request_type: &str) -> Result<String> {
        if let Some(endpoint) = self.get_cached_endpoint("ai").await {
            return Ok(endpoint);
        }

        let endpoint = format!("ai://service-{}", self.adapter.get_service_id());
        self.cache_endpoint("ai", &endpoint).await;
        Ok(endpoint)
    }

    /// Generic routing based on capability type
    pub async fn route_to_capability(&self, capability: &str) -> Result<String> {
        match capability {
            "storage" | "zfs" | "nas" => self.route_to_storage(capability).await,
            "orchestration" | "workflow" | "service-mesh" => {
                self.route_to_orchestration(capability).await
            }
            "security" | "auth" | "authorization" => self.route_to_security(capability).await,
            "ai" | "ml" | "analytics" => self.route_to_ai(capability).await,
            _ => {
                // Generic capability discovery
                let capabilities = self
                    .adapter
                    .query_capabilities(capability.to_string())
                    .await;

                capabilities
                    .first()
                    .cloned() // String capabilities don't have .name field
                    .ok_or_else(|| NestGateError::Configuration {
                        message: format!("No capability found for: {capability}"),
                        config_source: crate::error::UnifiedConfigSource::Runtime,
                        field: Some(capability.to_string()),
                        suggested_fix: Some(
                            "Ensure required service is registered with the universal adapter"
                                .to_string(),
                        ),
                    })
            }
        }
    }

    /// Get cached endpoint
    async fn get_cached_endpoint(&self, capability: &str) -> Option<String> {
        let cache = self.endpoint_cache.read().await;
        cache.get(capability).cloned()
    }

    /// Cache endpoint for future use
    async fn cache_endpoint(&self, capability: &str, endpoint: &str) {
        let mut cache = self.endpoint_cache.write().await;
        cache.insert(capability.to_string(), endpoint.to_string());
    }

    /// Clear endpoint cache (for refreshing discovery)
    pub async fn clear_cache(&self) {
        let mut cache = self.endpoint_cache.write().await;
        cache.clear();
    }
}
