// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::Arc;
use tests::config::ConsolidatedCanonicalConfig;
use tests::config::ConsolidatedCanonicalConfig;
use tests::config::ConsolidatedCanonicalConfig;
use tests::config::ConsolidatedCanonicalConfig;
use tokio::sync::RwLock;

/// Test service manager for dynamic port allocation
/// Eliminates hardcoded localhost endpoints in tests
pub struct TestServiceManager {
    /// Allocated ports for different services
    allocated_ports: Arc<RwLock<HashMap<String, u16>>>,
    /// Base port for allocation (starts here and goes up)
    base_port: u16,
}

impl TestServiceManager {
    /// Create a new test service manager
    pub fn new() -> Self {
        Self {
            allocated_ports: Arc::new(RwLock::new(HashMap::new())),
            base_port: nestgate_core::constants::DEFAULT_API_PORT, // Starting port for tests
        }
    }

    /// Create with custom base port
    pub fn with_base_port(base_port: u16) -> Self {
        Self {
            allocated_ports: Arc::new(RwLock::new(HashMap::new())),
            base_port,
        }
    }

    /// Allocate a free port for a service
    pub async fn allocate_service_port(&self, service: &str) -> u16 {
        let mut ports = self.allocated_ports.write().await;

        // Return existing port if already allocated
        if let Some(&port) = ports.get(service) {
            return port;
        }

        // Find a free port
        let port = self.find_free_port_from(self.base_port);
        ports.insert(service.to_string(), port);
        port
    }

    /// Get service endpoint with dynamic port
    pub async fn get_service_endpoint(&self, service: &str) -> String {
        let port = self.allocate_service_port(service).await;
        let hostname = std::env::var("TEST_HOSTNAME")
            .unwrap_or_else(|_| nestgate_core::constants::TEST_HOSTNAME.to_string());

        match service {
            "websocket" => format!("ws://{}:{}", hostname, port),
            _ => format!("http://{}:{}", hostname, port),
        }
    }

    /// Get API endpoint for service
    pub async fn get_api_endpoint(&self, service: &str, path: &str) -> String {
        let base = self.get_service_endpoint(service).await;
        format!(
            "{}/{}",
            base.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    /// Get health check endpoint
    pub async fn get_health_endpoint(&self, service: &str) -> String {
        self.get_api_endpoint(service, "health").await
    }

    /// Get metrics endpoint  
    pub async fn get_metrics_endpoint(&self, service: &str) -> String {
        self.get_api_endpoint(service, "metrics").await
    }

    /// Release all allocated ports
    pub async fn release_all_ports(&self) {
        let mut ports = self.allocated_ports.write().await;
        ports.clear();
    }

    /// Get all allocated ports (for debugging)
    pub async fn get_allocated_ports(&self) -> HashMap<String, u16> {
        let ports = self.allocated_ports.read().await;
        ports.clone()
    }

    /// Find a free port starting from the given port
    fn find_free_port_from(&self, start_port: u16) -> u16 {
        for port in start_port..65535 {
            if self.is_port_free(port) {
                return port;
            }
        }
        panic!("No free ports available starting from {}", start_port);
    }

    /// Check if a port is free
    fn is_port_free(&self, port: u16) -> bool {
        match TcpListener::bind(("127.0.0.1", port)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

impl Default for TestServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global test service manager instance
static GLOBAL_TEST_MANAGER: tokio::sync::OnceCell<TestServiceManager> =
    tokio::sync::OnceCell::const_new();

/// Get global test service manager
pub async fn global_test_manager() -> &'static TestServiceManager {
    GLOBAL_TEST_MANAGER
        .get_or_init(|| async { TestServiceManager::new() })
        .await
}

/// Convenience function to get test service endpoint
pub async fn get_test_endpoint(service: &str) -> String {
    global_test_manager()
        .await
        .get_service_endpoint(service)
        .await
}

/// Convenience function to get test API endpoint
pub async fn get_test_api_endpoint(service: &str, path: &str) -> String {
    global_test_manager()
        .await
        .get_api_endpoint(service, path)
        .await
}

/// Test configuration builder
#[deprecated(
    since = "0.6.0",
    note = "Use ConsolidatedCanonicalConfig::test_config() instead"
)]
pub struct LegacyTestConfig {
    services: HashMap<String, String>,
}

impl TestConfig {
    /// Create new test configuration
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Add service with dynamic endpoint
    pub async fn with_service(mut self, service: &str) -> Self {
        let endpoint = get_test_endpoint(service).await;
        self.services.insert(service.to_string(), endpoint);
        self
    }

    /// Get service endpoint
    pub fn get_endpoint(&self, service: &str) -> Option<&str> {
        self.services.get(service).map(|s| s.as_str())
    }

    /// Build configuration as environment variables
    pub fn as_env_vars(&self) -> Vec<(String, String)> {
        self.services
            .iter()
            .map(|(service, endpoint)| {
                (
                    format!("{}_ENDPOINT", service.to_uppercase()),
                    endpoint.clone(),
                )
            })
            .collect()
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_port_allocation() -> Result<(), Box<dyn std::error::Error>> {
        let manager = TestServiceManager::new();

        // Allocate ports for different services
        let api_port = manager.allocate_service_port("api").await;
        let websocket_port = manager.allocate_service_port("websocket").await;
        let metrics_port = manager.allocate_service_port("metrics").await;

        // All ports should be different
        assert_ne!(api_port, websocket_port);
        assert_ne!(api_port, metrics_port);
        assert_ne!(websocket_port, metrics_port);

        // Requesting same service should return same port
        let api_port2 = manager.allocate_service_port("api").await;
        assert_eq!(api_port, api_port2);
        Ok(())
    }

    #[tokio::test]
    async fn test_endpoint_generation() -> Result<(), Box<dyn std::error::Error>> {
        let manager = TestServiceManager::new();

        // Test HTTP endpoint
        let api_endpoint = manager.get_service_endpoint("api").await;
        assert!(api_endpoint.starts_with("http://"));
        assert!(api_endpoint.contains(nestgate_core::constants::TEST_HOSTNAME));

        // Test WebSocket endpoint
        let ws_endpoint = manager.get_service_endpoint("websocket").await;
        assert!(ws_endpoint.starts_with("ws://"));
        assert!(ws_endpoint.contains(nestgate_core::constants::TEST_HOSTNAME));
        Ok(())
    }

    #[tokio::test]
    async fn test_no_hardcoded_ports() -> Result<(), Box<dyn std::error::Error>> {
        let manager = TestServiceManager::new();

        for service in &["api", "websocket", "metrics", "health"] {
            let endpoint = manager.get_service_endpoint(service).await;

            // Should not contain common hardcoded ports
            assert!(
                !endpoint.contains(":8080"),
                "Found hardcoded :8080 in {}",
                endpoint
            );
            assert!(
                !endpoint.contains(":8081"),
                "Found hardcoded :8081 in {}",
                endpoint
            );
            assert!(
                !endpoint.contains(":8082"),
                "Found hardcoded :8082 in {}",
                endpoint
            );
            Ok(())
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_test_config_builder() -> Result<(), Box<dyn std::error::Error>> {
        let config = TestConfig::new()
            .with_service("api")
            .await
            .with_service("websocket")
            .await;

        // Should have endpoints for both services
        assert!(config.get_endpoint("api").is_some());
        assert!(config.get_endpoint("websocket").is_some());

        // Environment variables should be generated
        let env_vars = config.as_env_vars();
        assert_eq!(env_vars.len(), 2);

        // Should contain API_ENDPOINT and WEBSOCKET_ENDPOINT
        let var_names: Vec<String> = env_vars.iter().map(|(k, _)| k.clone()).collect();
        assert!(var_names.contains(&"API_ENDPOINT".to_string()));
        assert!(var_names.contains(&"WEBSOCKET_ENDPOINT".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn test_global_test_manager() -> Result<(), Box<dyn std::error::Error>> {
        let manager = global_test_manager().await;
        let endpoint = manager.get_service_endpoint("test").await;

        assert!(endpoint.starts_with("http://"));
        assert!(!endpoint.contains("hardcoded"));
        Ok(())
    }
}
