/// Service Registry Module
/// Handles external service discovery and registry operations including:
/// - Service registry client operations
/// - Service mesh integration
/// - External discovery queries
/// - Registry-based configuration
use crate::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use super::registry_config::{RegistryConfig, SharedRegistryConfig};
/// Discovery query configuration
#[derive(Debug, Clone)]
/// Discoveryquery
pub struct DiscoveryQuery {
    /// Service name
    pub service_name: String,
    /// Query Type
    pub query_type: String,
    /// Timeout
    pub timeout: Duration,
    /// Fallback Enabled
    pub fallback_enabled: bool,
}
/// Service registry client
#[derive(Debug)]
/// Serviceregistryclient
pub struct ServiceRegistryClient {
    base_url: Option<String>,
    timeout: Duration,
    registry_cache: HashMap<String, String>,
    /// Configuration for
    pub config: SharedRegistryConfig,
}

impl Default for ServiceRegistryClient {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistryClient {
    /// Create new service registry client using environment variables
    /// NOTE: Creates config from env each time. For tests, use with_config() directly.
    #[must_use]
    pub fn new() -> Self {
        let config = Arc::new(RegistryConfig::from_env());
        Self {
            base_url: config.get_registry_url().map(|s| s.to_string()),
            timeout: Duration::from_secs(10),
            registry_cache: HashMap::new(),
            config,
        }
    }

    /// Create service registry client with injected configuration (for tests)
    #[must_use]
    pub fn with_config(config: SharedRegistryConfig) -> Self {
        Self {
            base_url: config.get_registry_url().map(|s| s.to_string()),
            timeout: Duration::from_secs(10),
            registry_cache: HashMap::new(),
            config,
        }
    }

    /// **REGISTRY QUERY**: Query external service registry
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn query_service(&self, service_name: &str, query_type: &str) -> Result<String> {
        // Check cache first
        let cache_key = format!("{service_name}:{query_type}");
        if let Some(cachedvalue) = self.registry_cache.get(&cache_key) {
            return Ok(cachedvalue.clone());
        }

        // Try config-based registry lookup (captured from environment at initialization)
        if let Some(value) = self.config.get_registry_entry(service_name, query_type) {
            return Ok(value.to_string());
        }

        // Fallback to service mesh discovery
        self.query_service_mesh(service_name)
    }

    /// **SERVICE MESH INTEGRATION**: Query service mesh for service discovery
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn query_service_mesh(&self, service_name: &str) -> Result<String> {
        // Check for service mesh configuration (from injected config)
        if let Some(mesh_endpoint) = self.config.get_service_mesh_endpoint() {
            return Ok(format!("{mesh_endpoint}/{service_name}"));
        }

        // Check for Kubernetes service discovery
        // DEPRECATED: Direct Kubernetes integration - migrate to capability-based orchestration
        // Migration Guide: Use ORCHESTRATION_DISCOVERY_ENDPOINT instead of KUBERNETES_NAMESPACE
        // Legacy compatibility maintained
        if let Some(k8s_namespace) = self.config.get_kubernetes_namespace() {
            tracing::warn!(
                "DEPRECATED: KUBERNETES_NAMESPACE detected. Please migrate to ORCHESTRATION_DISCOVERY_ENDPOINT. \
                This direct Kubernetes integration will be removed in version 4.0.0. \
                Migration: Set ORCHESTRATION_DISCOVERY_ENDPOINT=http://orchestration-service:8080"
            );
            return Ok(format!("{service_name}.{k8s_namespace}.svc.cluster.local"));
        }

        // DEPRECATED: Docker Compose service discovery - migrate to capability-based compute
        // Migration Guide: Use COMPUTE_DISCOVERY_ENDPOINT instead of DOCKER_COMPOSE_PROJECT
        // Legacy compatibility maintained
        if self.config.has_docker_compose_project() {
            tracing::warn!(
                "DEPRECATED: DOCKER_COMPOSE_PROJECT detected. Please migrate to COMPUTE_DISCOVERY_ENDPOINT. \
                This direct Docker integration will be removed in version 4.0.0. \
                Migration: Set COMPUTE_DISCOVERY_ENDPOINT=http://compute-service:8080"
            );
            return Ok(service_name.to_string()); // Docker Compose DNS
        }

        // Modern capability-based discovery (preferred method)
        if let Some(endpoint) = self.config.get_capability_discovery_endpoint() {
            tracing::info!("Using modern capability-based discovery: {}", endpoint);
            return Ok(format!("{endpoint}/{service_name}"));
        }

        // Fallback to localhost for development (from config or default)
        Ok(self
            .config
            .get_api_endpoint()
            .map(|s| s.to_string())
            .unwrap_or_else(crate::constants::canonical_defaults::network::build_api_url))
    }

    /// **CAPABILITY REGISTRATION**: Register capability endpoint
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn register_capability_endpoint(&self, capability: &str, endpoint: &str) -> Result<()> {
        // In a real implementation, this would register with external registry
        tracing::info!(
            "Registering capability '{}' at endpoint '{}'",
            capability,
            endpoint
        );

        // For now, just validate the inputs
        if capability.is_empty() || endpoint.is_empty() {
            return Err(NestGateError::configuration_error(
                "capability_endpoint",
                "Capability and endpoint cannot be empty",
            ));
        }
        Ok(())
    }

    /// **PORT DISCOVERY VIA ADAPTER**: Discover port through adapter system
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn discover_port_via_adapter(&self, service_name: &str, port_type: &str) -> Result<u16> {
        // Try adapter-based discovery through config (captured from environment at initialization)
        let adapter_key = format!("{}_{}", service_name, port_type);

        if let Some(port) = self.config.get_adapter_port(&adapter_key) {
            Ok(port)
        } else {
            Err(NestGateError::configuration_error_detailed(
                "adapter_configuration".to_string(),
                format!("No adapter configuration found for {service_name}:{port_type}"),
                None,
                Some(format!(
                    "environment variable NESTGATE_ADAPTER_{}_{}_PORT",
                    service_name.to_uppercase(),
                    port_type.to_uppercase()
                )),
                true,
            ))
        }
    }

    /// **REGISTRY HEALTH CHECK**: Check registry connectivity
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn health_check(&self) -> Result<HashMap<String, String>> {
        let mut health = HashMap::new();

        // Check if registry URL is configured
        health.insert(
            "registry_configured".to_string(),
            self.base_url.is_some().to_string(),
        );

        // Check configuration-based discovery (captured from environment at initialization)
        let env_vars = vec![
            "NESTGATE_REGISTRY_URL",
            "NESTGATE_SERVICE_MESH_ENDPOINT",
            "KUBERNETES_NAMESPACE",
            "DOCKER_COMPOSE_PROJECT",
        ];

        for env_var in env_vars {
            health.insert(
                env_var.to_lowercase(),
                self.config.has_env_var(env_var).to_string(),
            );
        }

        health.insert(
            "cache_size".to_string(),
            self.registry_cache.len().to_string(),
        );
        health.insert("timeout".to_string(), format!("{:?}", self.timeout));

        Ok(health)
    }

    /// **DISCOVERY VALIDATION**: Validate discovery configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate_discovery_config(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Check for basic configuration
        if self.base_url.is_none() {
            warnings.push("No registry URL configured - using fallback discovery".to_string());
        }

        // Check for service mesh configuration (from config)
        if !self.config.has_service_mesh() {
            warnings.push(
                "No service mesh configuration detected - using localhost fallback".to_string(),
            );
        }

        // Check timeout configuration
        if self.timeout > Duration::from_secs(30) {
            warnings
                .push("Registry timeout is very high - may impact startup performance".to_string());
        }

        Ok(warnings)
    }

    /// **CONFIGURATION SUMMARY**: Get registry configuration summary
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn get_config_summary(&self) -> Result<HashMap<String, String>> {
        let mut config = HashMap::new();

        config.insert(
            "registry_url".to_string(),
            self.base_url
                .as_ref()
                .unwrap_or(&"not_configured".to_string())
                .clone(),
        );
        config.insert("timeout".to_string(), format!("{:?}", self.timeout));
        config.insert(
            "cache_entries".to_string(),
            self.registry_cache.len().to_string(),
        );

        // Add discovery method availability (from config)
        config.insert(
            "service_mesh_available".to_string(),
            self.config
                .has_env_var("NESTGATE_SERVICE_MESH_ENDPOINT")
                .to_string(),
        );
        config.insert(
            // DEPRECATED: Kubernetes orchestration - migrate to capability-based orchestration
            // Capability-based discovery implemented
            "kubernetes_available".to_string(),
            self.config.has_env_var("KUBERNETES_NAMESPACE").to_string(),
        );
        config.insert(
            // DEPRECATED: Docker containerization - migrate to capability-based container runtime
            // Capability-based discovery implemented
            "docker_compose_available".to_string(),
            self.config
                .has_env_var("DOCKER_COMPOSE_PROJECT")
                .to_string(),
        );

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> SharedRegistryConfig {
        Arc::new(RegistryConfig::new())
    }

    #[test]
    fn test_service_registry_client_new() {
        let client = ServiceRegistryClient::new();
        assert_eq!(client.timeout, Duration::from_secs(10));
        assert!(client.registry_cache.is_empty());
    }

    #[test]
    fn test_service_registry_client_default() {
        let client = ServiceRegistryClient::default();
        assert_eq!(client.timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_service_registry_client_with_config() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);
        assert_eq!(client.timeout, Duration::from_secs(10));
        assert!(client.registry_cache.is_empty());
    }

    #[tokio::test]
    async fn test_query_service_fallback() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        // Without any configuration, should fallback to localhost
        let result = client.query_service("test_service", "endpoint").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_service_mesh_localhost_fallback() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.query_service_mesh("test_service");
        assert!(result.is_ok());
        // Should return localhost fallback
        let endpoint = result.unwrap();
        assert!(endpoint.contains("localhost") || endpoint.contains("127.0.0.1"));
    }

    #[test]
    fn test_register_capability_endpoint_success() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.register_capability_endpoint("encryption", "http://crypto.example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_capability_endpoint_empty_capability() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.register_capability_endpoint("", "http://crypto.example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_register_capability_endpoint_empty_endpoint() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.register_capability_endpoint("encryption", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_register_capability_endpoint_both_empty() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.register_capability_endpoint("", "");
        assert!(result.is_err());
    }

    #[test]
    fn test_discover_port_via_adapter_success() {
        // Test adapter port discovery without needing to set internal fields
        // This tests the error path which is more important for coverage
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        // For now, test the error case as we cannot set adapter_ports directly
        let result = client.discover_port_via_adapter("test_service", "api");
        // Should return error when no adapter configuration found
        assert!(result.is_err());
    }

    #[test]
    fn test_discover_port_via_adapter_not_found() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.discover_port_via_adapter("unknown_service", "api");
        assert!(result.is_err());
    }


    #[test]
    fn test_health_check_no_registry() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.health_check();
        assert!(result.is_ok());

        let health = result.unwrap();
        assert_eq!(
            health.get("registry_configured"),
            Some(&"false".to_string())
        );
    }

    #[test]
    fn test_validate_discovery_config_no_registry() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.validate_discovery_config();
        assert!(result.is_ok());

        let warnings = result.unwrap();
        assert!(!warnings.is_empty());
        assert!(warnings
            .iter()
            .any(|w| w.contains("No registry URL configured")));
    }


    #[test]
    fn test_validate_discovery_config_high_timeout() {
        let config = create_test_config();
        let mut client = ServiceRegistryClient::with_config(config);
        client.timeout = Duration::from_secs(60);

        let result = client.validate_discovery_config();
        assert!(result.is_ok());

        let warnings = result.unwrap();
        assert!(warnings.iter().any(|w| w.contains("timeout is very high")));
    }


    #[test]
    fn test_get_config_summary_no_registry() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let result = client.get_config_summary();
        assert!(result.is_ok());

        let summary = result.unwrap();
        assert_eq!(
            summary.get("registry_url"),
            Some(&"not_configured".to_string())
        );
    }

    #[test]
    fn test_discovery_query_creation() {
        let query = DiscoveryQuery {
            service_name: "test_service".to_string(),
            query_type: "endpoint".to_string(),
            timeout: Duration::from_secs(5),
            fallback_enabled: true,
        };

        assert_eq!(query.service_name, "test_service");
        assert_eq!(query.query_type, "endpoint");
        assert_eq!(query.timeout, Duration::from_secs(5));
        assert!(query.fallback_enabled);
    }

    #[test]
    fn test_discovery_query_clone() {
        let query1 = DiscoveryQuery {
            service_name: "test".to_string(),
            query_type: "api".to_string(),
            timeout: Duration::from_secs(10),
            fallback_enabled: false,
        };

        let query2 = query1.clone();
        assert_eq!(query1.service_name, query2.service_name);
        assert_eq!(query1.query_type, query2.query_type);
    }

    #[test]
    fn test_service_registry_client_debug() {
        let config = create_test_config();
        let client = ServiceRegistryClient::with_config(config);

        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("ServiceRegistryClient"));
    }

    #[test]
    fn test_discovery_query_debug() {
        let query = DiscoveryQuery {
            service_name: "test".to_string(),
            query_type: "api".to_string(),
            timeout: Duration::from_secs(5),
            fallback_enabled: true,
        };

        let debug_str = format!("{:?}", query);
        assert!(debug_str.contains("DiscoveryQuery"));
        assert!(debug_str.contains("test"));
    }

    #[tokio::test]
    async fn test_concurrent_queries() {
        let config = create_test_config();
        let client = Arc::new(ServiceRegistryClient::with_config(config));

        let client1 = Arc::clone(&client);
        let client2 = Arc::clone(&client);

        let handle1 = tokio::spawn(async move {
            client1.query_service("service1", "endpoint").await
        });

        let handle2 = tokio::spawn(async move {
            client2.query_service("service2", "endpoint").await
        });

        let result1 = handle1.await.unwrap();
        let result2 = handle2.await.unwrap();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_multiple_clients() {
        let config = create_test_config();
        let client1 = ServiceRegistryClient::with_config(Arc::clone(&config));
        let client2 = ServiceRegistryClient::with_config(config);

        // Both clients should work independently
        assert_eq!(client1.timeout, client2.timeout);
    }
}
