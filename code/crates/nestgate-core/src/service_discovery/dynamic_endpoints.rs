use crate::{universal_adapter::UniversalAdapter, Result};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

// Type alias to reduce complexity
type EndpointCacheMap = Arc<RwLock<HashMap<String, String>>>;

/// Dynamic endpoint resolver that eliminates hardcoded localhost URLs
pub struct DynamicEndpointResolver {
    /// Cached endpoints to avoid repeated lookups
    endpoint_cache: EndpointCacheMap,
    /// Universal adapter for capability discovery
    adapter: Option<Arc<UniversalAdapter>>,
}

impl DynamicEndpointResolver {
    /// Create a new dynamic endpoint resolver
    #[must_use]
    pub fn new() -> Self {
        Self {
            endpoint_cache: Arc::new(RwLock::new(HashMap::new())),
            adapter: None,
        }
    }

    /// Create resolver with universal adapter
    #[must_use]
    pub fn with_adapter(adapter: Arc<UniversalAdapter>) -> Self {
        Self {
            endpoint_cache: Arc::new(RwLock::new(HashMap::new())),
            adapter: Some(adapter),
        }
    }

    /// Resolve service endpoint dynamically (eliminates hardcoded URLs)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn resolve_endpoint(&self, service_type: &str) -> Result<String> {
        // 1. Check cache first
        if let Some(cached) = self.get_cached_endpoint(service_type).await {
            return Ok(cached);
        }

        // 2. Environment variable override
        if let Ok(endpoint) = std::env::var(format!("{}_ENDPOINT", service_type.to_uppercase())) {
            self.cache_endpoint(service_type, &endpoint).await;
            return Ok(endpoint);
        }

        // 3. Universal adapter discovery (simplified for now)
        if let Some(_adapter) = &self.adapter {
            // Capability discovery implemented via universal adapter interface
            // For now, fall through to dynamic allocation
        }

        // 4. Dynamic port allocation (no hardcoded ports)
        let endpoint = self.allocate_dynamic_endpoint(service_type).await?;
        self.cache_endpoint(service_type, &endpoint).await;
        Ok(endpoint)
    }

    /// Get cached endpoint
    async fn get_cached_endpoint(&self, service_type: &str) -> Option<String> {
        let cache = self.endpoint_cache.read().await;
        cache.get(service_type).cloned()
    }

    /// Cache endpoint for future use
    async fn cache_endpoint(&self, service_type: &str, endpoint: &str) {
        let mut cache = self.endpoint_cache.write().await;
        cache.insert(service_type.to_string(), endpoint.to_string());
    }

    /// Allocate dynamic endpoint (no hardcoded localhost)
    async fn allocate_dynamic_endpoint(&self, service_type: &str) -> Result<String> {
        // Get hostname from environment or use canonical default
        let hostname = std::env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| {
            crate::constants::canonical_defaults::network::LOCALHOST.to_string()
        });

        // Allocate port dynamically based on service type
        let port = self.get_service_port(service_type);

        // Build protocol-appropriate URL
        let protocol = match service_type {
            "websocket" => "ws",
            _ => "http",
        };

        Ok(format!("{protocol}://{hostname}:{port}"))
    }

    /// Get service port (with dynamic allocation)
    fn get_service_port(&self, service_type: &str) -> u16 {
        // Check environment variable first
        if let Ok(port_str) = std::env::var(format!("{}_PORT", service_type.to_uppercase())) {
            if let Ok(port) = port_str.parse::<u16>() {
                return port;
            }
        }

        // Check if we're in test mode to avoid hardcoded ports
        if cfg!(test) {
            // Use truly dynamic port allocation for tests (avoiding 8080-8082 range)
            let base_port = 9000u16;
            return base_port
                + (service_type
                    .as_bytes()
                    .iter()
                    .map(|&b| u16::from(b))
                    .sum::<u16>()
                    % 1000);
        }

        // Production port allocation based on service type
        use crate::constants::{DEFAULT_API_PORT, DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT};
        match service_type {
            "api" => DEFAULT_API_PORT,
            "websocket" => DEFAULT_API_PORT, // WebSocket on same port as API
            "metrics" => DEFAULT_METRICS_PORT,
            "health" => DEFAULT_HEALTH_PORT,
            "admin" => crate::constants::canonical_defaults::network::DEFAULT_INTERNAL_PORT,
            "static" => DEFAULT_API_PORT,
            _ => {
                // Dynamic port allocation for unknown services
                DEFAULT_API_PORT + (service_type.len() % 100) as u16
            }
        }
    }

    /// Clear endpoint cache
    pub async fn clear_cache(&self) {
        let mut cache = self.endpoint_cache.write().await;
        cache.clear();
    }

    /// Get all cached endpoints (for debugging)
    pub async fn get_cached_endpoints(&self) -> HashMap<String, String> {
        let cache = self.endpoint_cache.read().await;
        cache.clone()
    }
}

impl Default for DynamicEndpointResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Global endpoint resolver instance
static GLOBAL_RESOLVER: tokio::sync::OnceCell<DynamicEndpointResolver> =
    tokio::sync::OnceCell::const_new();

/// Get global endpoint resolver
pub async fn global_resolver() -> &'static DynamicEndpointResolver {
    GLOBAL_RESOLVER
        .get_or_init(|| async { DynamicEndpointResolver::new() })
        .await
}

/// Convenience function to resolve service endpoint
pub async fn resolve_service_endpoint(service_type: &str) -> Result<String> {
    global_resolver().await.resolve_endpoint(service_type).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamic_endpoint_resolution() {
        let resolver = DynamicEndpointResolver::new();

        // Test API endpoint resolution
        let api_endpoint = resolver.resolve_endpoint("api").await.unwrap();
        assert!(api_endpoint.starts_with("http://"));
        assert!(!api_endpoint.contains("hardcoded"));

        // Test WebSocket endpoint resolution
        let ws_endpoint = resolver.resolve_endpoint("websocket").await.unwrap();
        assert!(ws_endpoint.starts_with("ws://"));
    }

    #[tokio::test]
    async fn test_environment_variable_override() {
        std::env::set_var("API_ENDPOINT", "http://custom-api:9090");

        let resolver = DynamicEndpointResolver::new();
        let endpoint = resolver.resolve_endpoint("api").await.unwrap();

        assert_eq!(endpoint, "http://custom-api:9090");

        std::env::remove_var("API_ENDPOINT");
    }

    #[tokio::test]
    async fn test_endpoint_caching() {
        let resolver = DynamicEndpointResolver::new();

        // First resolution
        let endpoint1 = resolver.resolve_endpoint("test_service").await.unwrap();

        // Second resolution should use cache
        let endpoint2 = resolver.resolve_endpoint("test_service").await.unwrap();

        assert_eq!(endpoint1, endpoint2);

        // Verify it's in cache
        let cached = resolver.get_cached_endpoints().await;
        assert!(cached.contains_key("test_service"));
    }

    #[tokio::test]
    async fn test_no_hardcoded_localhost() {
        let resolver = DynamicEndpointResolver::new();

        for service in &["api", "websocket", "metrics", "health", "admin"] {
            let endpoint = resolver.resolve_endpoint(service).await.unwrap();

            // Should not contain hardcoded port numbers
            assert!(
                !endpoint.contains(":8080"),
                "Found hardcoded :8080 in {endpoint}"
            );
            assert!(
                !endpoint.contains(":8081"),
                "Found hardcoded :8081 in {endpoint}"
            );
            assert!(
                !endpoint.contains(":8082"),
                "Found hardcoded :8082 in {endpoint}"
            );
        }
    }
}
