/// Service Registry Module
/// Handles external service discovery and registry operations including:
/// - Service registry client operations
/// - Service mesh integration
/// - External discovery queries
/// - Registry-based configuration
use crate::{NestGateError, Result};
use std::collections::HashMap;
use std::time::Duration;
/// Discovery query configuration
#[derive(Debug, Clone)]
pub struct DiscoveryQuery {
    pub service_name: String,
    pub query_type: String,
    pub timeout: Duration,
    pub fallback_enabled: bool,
}
/// Service registry client
#[derive(Debug)]
pub struct ServiceRegistryClient {
    base_url: Option<String>,
    timeout: Duration,
    registry_cache: HashMap<String, String>,
}
impl Default for ServiceRegistryClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistryClient {
    /// Create new service registry client
    #[must_use]
    pub fn new() -> Self {
        Self {
            base_url: std::env::var("NESTGATE_REGISTRY_URL").ok(),
            timeout: Duration::from_secs(10),
            registry_cache: HashMap::new(),
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
    #[must_use]
    pub async fn query_service(&self, service_name: &str, query_type: &str) -> Result<String> {
        // Check cache first
        let cache_key = format!("{service_name}:{query_type}");
        if let Some(cachedvalue) = self.registry_cache.get(&cache_key) {
            return Ok(cachedvalue.clone());
        }

        // Try environment-based registry lookup
        let env_key = format!(
            "NESTGATE_REGISTRY_{}_{}",
            service_name.to_uppercase(),
            query_type.to_uppercase()
        );
        if let Ok(value) = std::env::var(&env_key) {
            return Ok(value);
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
    #[must_use]
    pub fn query_service_mesh(&self, service_name: &str) -> Result<String> {
        // Check for service mesh environment variables
        if let Ok(mesh_endpoint) = std::env::var("NESTGATE_SERVICE_MESH_ENDPOINT") {
            return Ok(format!("{mesh_endpoint}/{service_name}"));
        }

        // Check for Kubernetes service discovery
        // DEPRECATED: Direct Kubernetes integration - migrate to capability-based orchestration
        // Migration Guide: Use ORCHESTRATION_DISCOVERY_ENDPOINT instead of KUBERNETES_NAMESPACE
        // Legacy compatibility maintained
        if let Ok(k8s_namespace) = std::env::var("KUBERNETES_NAMESPACE") {
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
        if std::env::var("DOCKER_COMPOSE_PROJECT").is_ok() {
            tracing::warn!(
                "DEPRECATED: DOCKER_COMPOSE_PROJECT detected. Please migrate to COMPUTE_DISCOVERY_ENDPOINT. \
                This direct Docker integration will be removed in version 4.0.0. \
                Migration: Set COMPUTE_DISCOVERY_ENDPOINT=http://compute-service:8080"
            );
            return Ok(service_name.to_string()); // Docker Compose DNS
        }

        // Modern capability-based discovery (preferred method)
        if let Ok(endpoint) = std::env::var("CAPABILITY_DISCOVERY_ENDPOINT") {
            tracing::info!("Using modern capability-based discovery: {}", endpoint);
            return Ok(format!("{endpoint}/{service_name}"));
        }

        // Fallback to localhost for development
        Ok(std::env::var("NESTGATE_API_ENDPOINT")
            .unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_api_url()))
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
        // Try adapter-based discovery through environment
        let adapter_env_key = format!(
            "NESTGATE_ADAPTER_{}_{}_PORT",
            service_name.to_uppercase(),
            port_type.to_uppercase()
        );

        if let Ok(port_str) = std::env::var(&adapter_env_key) {
            port_str.parse::<u16>().map_err(|e| {
                NestGateError::configuration_error_detailed(
                    "port_configuration".to_string(),
                    format!("Invalid port configuration '{port_str}': {e}"),
                    Some(port_str.clone()),
                    Some("valid u16 integer".to_string()),
                    true,
                )
            })
        } else {
            Err(NestGateError::configuration_error_detailed(
                "adapter_configuration".to_string(),
                format!("No adapter configuration found for {service_name}:{port_type}"),
                None,
                Some(format!("environment variable {adapter_env_key}")),
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
    #[must_use]
    pub fn health_check(&self) -> Result<HashMap<String, String>> {
        let mut health = HashMap::new();

        // Check if registry URL is configured
        health.insert(
            "registry_configured".to_string(),
            self.base_url.is_some().to_string(),
        );

        // Check environment-based discovery
        let env_vars = vec![
            "NESTGATE_REGISTRY_URL",
            "NESTGATE_SERVICE_MESH_ENDPOINT",
            "KUBERNETES_NAMESPACE",
            "DOCKER_COMPOSE_PROJECT",
        ];

        for env_var in env_vars {
            health.insert(
                env_var.to_lowercase(),
                std::env::var(env_var).is_ok().to_string(),
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
    #[must_use]
    pub fn validate_discovery_config(&self) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Check for basic configuration
        if self.base_url.is_none() {
            warnings.push("No registry URL configured - using fallback discovery".to_string());
        }

        // Check for service mesh configuration
        if std::env::var("NESTGATE_SERVICE_MESH_ENDPOINT").is_err()
            && std::env::var("KUBERNETES_NAMESPACE").is_err()
            && std::env::var("DOCKER_COMPOSE_PROJECT").is_err()
        {
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
    #[must_use]
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

        // Add discovery method availability
        config.insert(
            "service_mesh_available".to_string(),
            std::env::var("NESTGATE_SERVICE_MESH_ENDPOINT")
                .is_ok()
                .to_string(),
        );
        config.insert(
            // DEPRECATED: Kubernetes orchestration - migrate to capability-based orchestration
            // Capability-based discovery implemented
            "kubernetes_available".to_string(),
            std::env::var("KUBERNETES_NAMESPACE").is_ok().to_string(),
        );
        config.insert(
            // DEPRECATED: Docker containerization - migrate to capability-based container runtime
            // Capability-based discovery implemented
            "docker_compose_available".to_string(),
            std::env::var("DOCKER_COMPOSE_PROJECT").is_ok().to_string(),
        );

        Ok(config)
    }
}
