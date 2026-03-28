// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe configuration for service registry
/// Captures environment variables at initialization to prevent race conditions
#[derive(Debug, Clone)]
/// Configuration for Registry
pub struct RegistryConfig {
    // Core registry settings
    registry_url: Option<String>,

    // Service mesh configuration
    service_mesh_endpoint: Option<String>,

    // Orchestration settings (deprecated but supported)
    kubernetes_namespace: Option<String>,
    docker_compose_project: Option<String>,

    // Modern capability-based discovery
    capability_discovery_endpoint: Option<String>,

    // API endpoint fallback
    api_endpoint: Option<String>,

    // Dynamic registry entries (NESTGATE_REGISTRY_*)
    registry_entries: HashMap<String, String>,

    // Adapter port mappings
    adapter_ports: HashMap<String, u16>,
}

/// Shared immutable reference to RegistryConfig
pub type SharedRegistryConfig = Arc<RegistryConfig>;

impl RegistryConfig {
    /// Create a new empty configuration (all values None)
    pub fn new() -> Self {
        Self {
            registry_url: None,
            service_mesh_endpoint: None,
            kubernetes_namespace: None,
            docker_compose_project: None,
            capability_discovery_endpoint: None,
            api_endpoint: None,
            registry_entries: HashMap::new(),
            adapter_ports: HashMap::new(),
        }
    }

    /// Create configuration from current environment variables
    /// This captures env vars at initialization time, making it thread-safe
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Core registry URL
        config.registry_url = std::env::var("NESTGATE_REGISTRY_URL").ok();

        // Service mesh configuration
        config.service_mesh_endpoint = std::env::var("NESTGATE_SERVICE_MESH_ENDPOINT").ok();

        // Orchestration settings (deprecated)
        config.kubernetes_namespace = std::env::var("KUBERNETES_NAMESPACE").ok();
        config.docker_compose_project = std::env::var("DOCKER_COMPOSE_PROJECT").ok();

        // Modern capability-based discovery
        config.capability_discovery_endpoint = std::env::var("CAPABILITY_DISCOVERY_ENDPOINT").ok();

        // API endpoint fallback
        config.api_endpoint = std::env::var("NESTGATE_API_ENDPOINT").ok();

        // Scan for dynamic NESTGATE_REGISTRY_* entries
        for (key, value) in std::env::vars() {
            if key.starts_with("NESTGATE_REGISTRY_") && key != "NESTGATE_REGISTRY_URL" {
                config.registry_entries.insert(key, value);
            }
        }

        // Scan for adapter port entries (NESTGATE_ADAPTER_*_PORT)
        for (key, value) in std::env::vars() {
            if key.starts_with("NESTGATE_ADAPTER_") && key.ends_with("_PORT") {
                if let Ok(port) = value.parse::<u16>() {
                    config.adapter_ports.insert(key, port);
                }
            }
        }

        config
    }

    // Accessor methods

    /// Gets Registry Url
    pub fn get_registry_url(&self) -> Option<&str> {
        self.registry_url.as_deref()
    }

    /// Gets Service Mesh Endpoint
    pub fn get_service_mesh_endpoint(&self) -> Option<&str> {
        self.service_mesh_endpoint.as_deref()
    }

    /// Gets Kubernetes Namespace
    pub fn get_kubernetes_namespace(&self) -> Option<&str> {
        self.kubernetes_namespace.as_deref()
    }

    /// Checks if has Docker Compose Project
    pub fn has_docker_compose_project(&self) -> bool {
        self.docker_compose_project.is_some()
    }

    /// Gets Capability Discovery Endpoint
    pub fn get_capability_discovery_endpoint(&self) -> Option<&str> {
        self.capability_discovery_endpoint.as_deref()
    }

    /// Gets Api Endpoint
    pub fn get_api_endpoint(&self) -> Option<&str> {
        self.api_endpoint.as_deref()
    }

    /// Get a registry entry by constructing the key from service name and query type
    pub fn get_registry_entry(&self, service_name: &str, query_type: &str) -> Option<&str> {
        let key = format!(
            "NESTGATE_REGISTRY_{}_{}",
            service_name.to_uppercase(),
            query_type.to_uppercase()
        );
        self.registry_entries.get(&key).map(|s| s.as_str())
    }

    /// Get adapter port by adapter name
    pub fn get_adapter_port(&self, adapter_name: &str) -> Option<u16> {
        let key = format!("NESTGATE_ADAPTER_{}_PORT", adapter_name.to_uppercase());
        self.adapter_ports.get(&key).copied()
    }

    /// Check if any service mesh configuration is present
    pub fn has_service_mesh(&self) -> bool {
        self.service_mesh_endpoint.is_some()
            || self.kubernetes_namespace.is_some()
            || self.docker_compose_project.is_some()
    }

    /// Check if a specific environment variable was present
    /// Used for health checks and diagnostics
    pub fn has_env_var(&self, var_name: &str) -> bool {
        match var_name {
            "NESTGATE_SERVICE_MESH_ENDPOINT" => self.service_mesh_endpoint.is_some(),
            "KUBERNETES_NAMESPACE" => self.kubernetes_namespace.is_some(),
            "DOCKER_COMPOSE_PROJECT" => self.docker_compose_project.is_some(),
            _ => self.registry_entries.contains_key(var_name),
        }
    }

    // Builder methods for tests

    /// Builder method to set Registry Url
    pub fn with_registry_url(mut self, url: String) -> Self {
        self.registry_url = Some(url);
        self
    }

    /// Builder method to set Service Mesh Endpoint
    pub fn with_service_mesh_endpoint(mut self, endpoint: String) -> Self {
        self.service_mesh_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Kubernetes Namespace
    pub fn with_kubernetes_namespace(mut self, namespace: String) -> Self {
        self.kubernetes_namespace = Some(namespace);
        self
    }

    /// Builder method to set Capability Discovery Endpoint
    pub fn with_capability_discovery_endpoint(mut self, endpoint: String) -> Self {
        self.capability_discovery_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Registry Entry
    pub fn with_registry_entry(mut self, service: &str, query_type: &str, value: String) -> Self {
        let key = format!(
            "NESTGATE_REGISTRY_{}_{}",
            service.to_uppercase(),
            query_type.to_uppercase()
        );
        self.registry_entries.insert(key, value);
        self
    }

    /// Builder method to set Adapter Port
    pub fn with_adapter_port(mut self, adapter_name: &str, port: u16) -> Self {
        let key = format!("NESTGATE_ADAPTER_{}_PORT", adapter_name.to_uppercase());
        self.adapter_ports.insert(key, port);
        self
    }
}

impl Default for RegistryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_config_new() {
        let config = RegistryConfig::new();

        assert!(config.get_registry_url().is_none());
        assert!(config.get_service_mesh_endpoint().is_none());
        assert!(!config.has_service_mesh());
    }

    #[test]
    fn test_registry_config_builder() {
        let config = RegistryConfig::new()
            .with_registry_url("http://registry:8080".to_string())
            .with_service_mesh_endpoint("http://mesh:9090".to_string())
            .with_registry_entry("database", "endpoint", "postgres://db:5432".to_string())
            .with_adapter_port("storage", 8000);

        assert_eq!(config.get_registry_url(), Some("http://registry:8080"));
        assert_eq!(config.get_service_mesh_endpoint(), Some("http://mesh:9090"));
        assert!(config.has_service_mesh());
        assert_eq!(
            config.get_registry_entry("database", "endpoint"),
            Some("postgres://db:5432")
        );
        assert_eq!(config.get_adapter_port("storage"), Some(8000));
    }

    #[test]
    fn test_registry_config_has_env_var() {
        let config =
            RegistryConfig::new().with_service_mesh_endpoint("http://mesh:9090".to_string());

        assert!(config.has_env_var("NESTGATE_SERVICE_MESH_ENDPOINT"));
        assert!(!config.has_env_var("KUBERNETES_NAMESPACE"));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_registry_config_access() {
        // Create two different configurations
        let config1 =
            Arc::new(RegistryConfig::new().with_registry_url("http://registry1:8080".to_string()));
        let config2 =
            Arc::new(RegistryConfig::new().with_registry_url("http://registry2:8080".to_string()));

        // Spawn concurrent tasks accessing different configs
        let handle1 = {
            let config = Arc::clone(&config1);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_registry_url(), Some("http://registry1:8080"));
                }
            })
        };

        let handle2 = {
            let config = Arc::clone(&config2);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_registry_url(), Some("http://registry2:8080"));
                }
            })
        };

        handle1.await.unwrap();
        handle2.await.unwrap();
    }

    #[test]
    fn test_registry_entry_key_construction() {
        let config = RegistryConfig::new().with_registry_entry(
            "my_service",
            "endpoint",
            "http://service:8080".to_string(),
        );

        // Should construct NESTGATE_REGISTRY_MY_SERVICE_ENDPOINT
        assert_eq!(
            config.get_registry_entry("my_service", "endpoint"),
            Some("http://service:8080")
        );
        assert_eq!(
            config.get_registry_entry("MY_SERVICE", "ENDPOINT"),
            Some("http://service:8080")
        );
    }
}
