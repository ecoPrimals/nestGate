//! Configuration for Dynamic Service Discovery
//!
//! This module provides thread-safe configuration for the dynamic endpoint system,
//! replacing runtime environment variable access with dependency injection.
//!
//! # Concurrency Safety
//!
//! Previously, service discovery read environment variables at runtime in hot paths,
//! creating race conditions. This config module loads configuration once at startup
//! and provides immutable, thread-safe access.

use std::collections::HashMap;
use std::sync::Arc;

/// Configuration for dynamic service discovery
///
/// This replaces runtime `env::var()` calls with immutable configuration,
/// making service discovery truly thread-safe.
///
/// # Example
///
/// ```rust
/// use nestgate_core::service_discovery::DynamicEndpointsConfig;
/// use std::sync::Arc;
///
/// // Production: Load from environment
/// let config = DynamicEndpointsConfig::from_env();
///
/// // Testing: Inject specific endpoints
/// let mut config = DynamicEndpointsConfig::new();
/// config.set_endpoint("api", "http://api:8080");
/// ```
#[derive(Debug, Clone)]
pub struct DynamicEndpointsConfig {
    /// Service-specific endpoint overrides
    /// Key: service type (e.g., "api", "storage")
    /// Value: endpoint URL
    endpoints: HashMap<String, String>,

    /// Default hostname for dynamic allocation
    hostname: String,

    /// Port allocation strategy
    port_base: u16,
}

impl DynamicEndpointsConfig {
    /// Create new empty configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
            hostname: "localhost".to_string(),
            port_base: 8000,
        }
    }

    /// Load configuration from environment variables
    ///
    /// Scans for service-specific endpoint overrides:
    /// - `SERVICE_ENDPOINT` - Override for specific service
    /// - `NESTGATE_HOSTNAME` - Default hostname
    #[must_use]
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Load hostname
        if let Ok(hostname) = std::env::var("NESTGATE_HOSTNAME") {
            config.hostname = hostname;
        }

        // Scan for service-specific endpoints
        // Common service types
        let service_types = [
            "API",
            "STORAGE",
            "CACHE",
            "DATABASE",
            "METRICS",
            "HEALTH",
            "DISCOVERY",
            "ADAPTER",
            "ORCHESTRATION",
            "SECURITY",
            "COMPUTE",
            "AI",
            "MONITORING",
        ];

        for service_type in &service_types {
            let env_key = format!("{}_ENDPOINT", service_type);
            if let Ok(endpoint) = std::env::var(&env_key) {
                config.set_endpoint(&service_type.to_lowercase(), &endpoint);
            }
        }

        config
    }

    /// Set endpoint for a service type
    pub fn set_endpoint(&mut self, service_type: &str, endpoint: &str) {
        self.endpoints
            .insert(service_type.to_string(), endpoint.to_string());
    }

    /// Get endpoint for a service type
    #[must_use]
    pub fn get_endpoint(&self, service_type: &str) -> Option<&str> {
        self.endpoints.get(service_type).map(String::as_str)
    }

    /// Get default hostname
    #[must_use]
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    /// Set default hostname
    pub fn set_hostname(&mut self, hostname: String) {
        self.hostname = hostname;
    }

    /// Get port base for dynamic allocation
    #[must_use]
    pub fn port_base(&self) -> u16 {
        self.port_base
    }

    /// Set port base for dynamic allocation
    pub fn set_port_base(&mut self, port_base: u16) {
        self.port_base = port_base;
    }

    /// Check if configuration has any endpoints
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.endpoints.is_empty()
    }

    /// Get number of configured endpoints
    #[must_use]
    pub fn len(&self) -> usize {
        self.endpoints.len()
    }

    /// Get all configured endpoints
    #[must_use]
    pub fn endpoints(&self) -> &HashMap<String, String> {
        &self.endpoints
    }
}

impl Default for DynamicEndpointsConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe shared configuration
pub type SharedEndpointsConfig = Arc<DynamicEndpointsConfig>;

/// Helper to create shared config from environment
#[must_use]
pub fn shared_config_from_env() -> SharedEndpointsConfig {
    Arc::new(DynamicEndpointsConfig::from_env())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_is_empty() {
        let config = DynamicEndpointsConfig::new();
        assert!(config.is_empty());
        assert_eq!(config.len(), 0);
    }

    #[test]
    fn test_config_set_get_endpoint() {
        let mut config = DynamicEndpointsConfig::new();
        config.set_endpoint("api", "http://api:8080");

        assert_eq!(config.get_endpoint("api"), Some("http://api:8080"));
        assert_eq!(config.get_endpoint("nonexistent"), None);
        assert_eq!(config.len(), 1);
    }

    #[test]
    fn test_config_hostname() {
        let mut config = DynamicEndpointsConfig::new();
        assert_eq!(config.hostname(), "localhost");

        config.set_hostname("production-host".to_string());
        assert_eq!(config.hostname(), "production-host");
    }

    #[test]
    fn test_config_port_base() {
        let mut config = DynamicEndpointsConfig::new();
        assert_eq!(config.port_base(), 8000);

        config.set_port_base(9000);
        assert_eq!(config.port_base(), 9000);
    }

    #[test]
    fn test_config_multiple_endpoints() {
        let mut config = DynamicEndpointsConfig::new();
        config.set_endpoint("api", "http://api:8080");
        config.set_endpoint("storage", "http://storage:8081");
        config.set_endpoint("cache", "http://cache:8082");

        assert_eq!(config.len(), 3);
        assert!(!config.is_empty());

        let endpoints = config.endpoints();
        assert_eq!(endpoints.len(), 3);
    }

    #[test]
    fn test_config_overwrite_endpoint() {
        let mut config = DynamicEndpointsConfig::new();
        config.set_endpoint("test", "http://old:8080");
        config.set_endpoint("test", "http://new:8081");

        assert_eq!(config.get_endpoint("test"), Some("http://new:8081"));
        assert_eq!(config.len(), 1);
    }

    #[test]
    fn test_config_clone() {
        let mut config = DynamicEndpointsConfig::new();
        config.set_endpoint("service", "http://service:8080");

        let cloned = config.clone();
        assert_eq!(cloned.get_endpoint("service"), Some("http://service:8080"));
    }

    #[test]
    fn test_config_default() {
        let config = DynamicEndpointsConfig::default();
        assert!(config.is_empty());
        assert_eq!(config.hostname(), "localhost");
    }

    #[test]
    fn test_shared_config() {
        let config = Arc::new(DynamicEndpointsConfig::new());
        let _clone1 = Arc::clone(&config);
        let _clone2 = Arc::clone(&config);

        assert_eq!(Arc::strong_count(&config), 3);
    }
}
