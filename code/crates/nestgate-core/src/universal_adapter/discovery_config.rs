use crate::config::discovery_config::ServiceDiscoveryConfig;
use std::sync::Arc;

/// Thread-safe configuration for capability discovery
/// Captures environment variables at initialization to prevent race conditions
///
/// ✅ MIGRATED: Now uses `ServiceDiscoveryConfig` for discovery endpoints,
/// removing hardcoded fallbacks in favor of centralized configuration.
#[derive(Debug, Clone)]
/// Configuration for DiscoveryRuntime
pub struct DiscoveryRuntimeConfig {
    /// Central service discovery configuration (Week 2 migration)
    service_discovery: ServiceDiscoveryConfig,

    // Specific capability endpoints (still support custom overrides)
    security_endpoint: Option<String>,
    ai_endpoint: Option<String>,
    orchestration_endpoint: Option<String>,
    storage_endpoint: Option<String>,
    compute_endpoint: Option<String>,
}

/// Shared immutable reference to DiscoveryRuntimeConfig
pub type SharedDiscoveryRuntimeConfig = Arc<DiscoveryRuntimeConfig>;

impl DiscoveryRuntimeConfig {
    /// Create a new empty configuration (all values None, will use centralized defaults)
    pub fn new() -> Self {
        Self {
            service_discovery: ServiceDiscoveryConfig::default(),
            security_endpoint: None,
            ai_endpoint: None,
            orchestration_endpoint: None,
            storage_endpoint: None,
            compute_endpoint: None,
        }
    }

    /// Create configuration from current environment variables
    /// This captures env vars at initialization time, making it thread-safe
    pub fn from_env() -> Self {
        Self {
            service_discovery: ServiceDiscoveryConfig::default(),
            security_endpoint: std::env::var("NESTGATE_SECURITY_ENDPOINT").ok(),
            ai_endpoint: std::env::var("NESTGATE_AI_ENDPOINT").ok(),
            orchestration_endpoint: std::env::var("NESTGATE_ORCHESTRATION_ENDPOINT").ok(),
            storage_endpoint: std::env::var("NESTGATE_STORAGE_ENDPOINT").ok(),
            compute_endpoint: std::env::var("NESTGATE_COMPUTE_ENDPOINT").ok(),
        }
    }

    // Accessors

    /// Get discovery endpoints from centralized configuration
    /// ✅ MIGRATED: Now uses `ServiceDiscoveryConfig` instead of hardcoded fallbacks
    pub fn get_discovery_endpoints(&self) -> Vec<String> {
        self.service_discovery.get_endpoints().to_vec()
    }

    /// Get base endpoint from service discovery configuration
    /// ✅ MIGRATED: Uses configured discovery host instead of hardcoded localhost
    pub fn get_base_endpoint(&self) -> String {
        format!(
            "http://{}:{}",
            self.service_discovery.discovery_host, self.service_discovery.discovery_base_port
        )
    }

    /// Get security endpoint (custom override or derived from base)
    pub fn get_security_endpoint(&self, base_endpoint: &str) -> Vec<String> {
        let custom = self
            .security_endpoint
            .clone()
            .unwrap_or_else(|| format!("{}/auth", base_endpoint));

        vec![format!("{base_endpoint}/security"), custom]
    }

    /// Get AI endpoint (custom override or derived from base)
    pub fn get_ai_endpoint(&self, base_endpoint: &str) -> Vec<String> {
        let custom = self
            .ai_endpoint
            .clone()
            .unwrap_or_else(|| format!("{}/ml", base_endpoint));

        vec![format!("{base_endpoint}/ai"), custom]
    }

    /// Get orchestration endpoint (custom override or derived from base)
    pub fn get_orchestration_endpoint(&self, base_endpoint: &str) -> Vec<String> {
        let custom = self
            .orchestration_endpoint
            .clone()
            .unwrap_or_else(|| format!("{}/workflow", base_endpoint));

        vec![format!("{base_endpoint}/orchestration"), custom]
    }

    /// Get storage endpoint (custom override or derived from base)
    pub fn get_storage_endpoint(&self, base_endpoint: &str) -> Vec<String> {
        let custom = self
            .storage_endpoint
            .clone()
            .unwrap_or_else(|| format!("{}/zfs", base_endpoint));

        vec![format!("{base_endpoint}/storage"), custom]
    }

    /// Get compute endpoint (custom override or derived from base)
    pub fn get_compute_endpoint(&self, base_endpoint: &str) -> Vec<String> {
        let custom = self
            .compute_endpoint
            .clone()
            .unwrap_or_else(|| format!("{}/processing", base_endpoint));

        vec![format!("{base_endpoint}/compute"), custom]
    }

    /// Get access to the underlying ServiceDiscoveryConfig
    pub fn service_discovery(&self) -> &ServiceDiscoveryConfig {
        &self.service_discovery
    }

    // Builder methods for tests

    /// Builder method for custom service discovery config (for testing)
    pub fn with_service_discovery(mut self, config: ServiceDiscoveryConfig) -> Self {
        self.service_discovery = config;
        self
    }

    /// Builder method to set Security Endpoint
    pub fn with_security_endpoint(mut self, endpoint: String) -> Self {
        self.security_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Ai Endpoint
    pub fn with_ai_endpoint(mut self, endpoint: String) -> Self {
        self.ai_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Orchestration Endpoint
    pub fn with_orchestration_endpoint(mut self, endpoint: String) -> Self {
        self.orchestration_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Storage Endpoint
    pub fn with_storage_endpoint(mut self, endpoint: String) -> Self {
        self.storage_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Compute Endpoint
    pub fn with_compute_endpoint(mut self, endpoint: String) -> Self {
        self.compute_endpoint = Some(endpoint);
        self
    }
}

impl Default for DiscoveryRuntimeConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_config_new() {
        let config = DiscoveryRuntimeConfig::new();

        // Should use ServiceDiscoveryConfig defaults
        let endpoints = config.get_discovery_endpoints();
        assert!(!endpoints.is_empty());
        // ServiceDiscoveryConfig uses 127.0.0.1 by default
        assert!(endpoints[0].contains("127.0.0.1"));
    }

    #[test]
    fn test_discovery_config_builder() {
        let custom_discovery = ServiceDiscoveryConfig::with_host_and_port("test".to_string(), 8000);

        let config = DiscoveryRuntimeConfig::new()
            .with_service_discovery(custom_discovery)
            .with_security_endpoint("http://security:9000".to_string());

        assert!(config.get_base_endpoint().contains("test:8000"));
        let base = config.get_base_endpoint();
        let security = config.get_security_endpoint(&base);
        assert!(security.contains(&"http://security:9000".to_string()));
    }

    #[test]
    fn test_discovery_config_all_capabilities() {
        let config = DiscoveryRuntimeConfig::new();
        let base = config.get_base_endpoint();

        // All capabilities should have endpoints
        assert!(!config.get_security_endpoint(&base).is_empty());
        assert!(!config.get_ai_endpoint(&base).is_empty());
        assert!(!config.get_orchestration_endpoint(&base).is_empty());
        assert!(!config.get_storage_endpoint(&base).is_empty());
        assert!(!config.get_compute_endpoint(&base).is_empty());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_discovery_config_access() {
        // Create two different configurations
        let config1 = Arc::new(DiscoveryRuntimeConfig::new().with_service_discovery(
            ServiceDiscoveryConfig::with_host_and_port("config1".to_string(), 5000),
        ));
        let config2 = Arc::new(DiscoveryRuntimeConfig::new().with_service_discovery(
            ServiceDiscoveryConfig::with_host_and_port("config2".to_string(), 6000),
        ));

        // Spawn concurrent tasks accessing different configs
        let handle1 = {
            let config = Arc::clone(&config1);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert!(config.get_base_endpoint().contains("config1:5000"));
                }
            })
        };

        let handle2 = {
            let config = Arc::clone(&config2);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert!(config.get_base_endpoint().contains("config2:6000"));
                }
            })
        };

        handle1.await.unwrap();
        handle2.await.unwrap();
    }

    #[test]
    fn test_discovery_endpoints_custom() {
        let endpoints = vec![
            "http://a:8080/discovery".to_string(),
            "http://b:9090/discovery".to_string(),
        ];
        let discovery = ServiceDiscoveryConfig::with_endpoints(endpoints.clone());
        let config = DiscoveryRuntimeConfig::new().with_service_discovery(discovery);

        let result = config.get_discovery_endpoints();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "http://a:8080/discovery");
        assert_eq!(result[1], "http://b:9090/discovery");
    }

    #[test]
    fn test_capability_endpoints_with_custom() {
        let discovery = ServiceDiscoveryConfig::with_host_and_port("mybase".to_string(), 8000);
        let config = DiscoveryRuntimeConfig::new()
            .with_service_discovery(discovery)
            .with_security_endpoint("http://sec:9000".to_string())
            .with_ai_endpoint("http://ai:9001".to_string());

        let base = config.get_base_endpoint();
        let security = config.get_security_endpoint(&base);
        let ai = config.get_ai_endpoint(&base);

        // Should have both base-relative and custom endpoints
        assert!(security.iter().any(|e| e.contains("mybase")));
        assert!(security.iter().any(|e| e == "http://sec:9000"));
        assert!(ai.iter().any(|e| e.contains("mybase")));
        assert!(ai.iter().any(|e| e == "http://ai:9001"));
    }
}
