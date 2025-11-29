//! Runtime Configuration for Network Discovery
//!
//! This module provides immutable runtime configuration for network discovery operations,
//! eliminating direct environment variable access and enabling concurrent-safe execution.
//!
//! # Architecture
//!
//! Follows NestGate's modern concurrency pattern:
//! - Immutable configuration loaded once
//! - Wrapped in Arc for zero-cost sharing
//! - No global mutable state
//! - Thread-safe by design
//!
//! # Usage
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use nestgate_core::universal_primal_discovery::NetworkRuntimeConfig;
//!
//! // Load from environment (production)
//! let config = Arc::new(NetworkRuntimeConfig::from_env());
//! let discovery = NetworkDiscovery::with_runtime_config(config);
//!
//! // Or create specific config (testing)
//! let mut config = NetworkRuntimeConfig::new();
//! config.set_bind_address("api", "192.168.1.100");
//! let discovery = NetworkDiscovery::with_runtime_config(Arc::new(config));
//! ```

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;

/// Immutable runtime configuration for network discovery
///
/// This struct holds all runtime configuration needed for NetworkDiscovery
/// without accessing environment variables during discovery operations.
///
/// Note: This is separate from the deprecated `NetworkDiscoveryConfig` which
/// handled scan timeouts and preferences. This config manages runtime values
/// like bind addresses, ports, and endpoints.
#[derive(Debug, Clone)]
/// Configuration for NetworkRuntime
pub struct NetworkRuntimeConfig {
    /// Bind addresses for specific services (`NESTGATE_<SERVICE>_BIND_ADDRESS`)
    bind_addresses: HashMap<String, IpAddr>,

    /// Ports for specific services (`NESTGATE_<SERVICE>_PORT`)
    bind_ports: HashMap<String, u16>,

    /// Service endpoints (`NESTGATE_<SERVICE>_ENDPOINT`)
    service_endpoints: HashMap<String, String>,

    /// Capability endpoints (`NESTGATE_<CAPABILITY>_ENDPOINT`)
    capability_endpoints: HashMap<String, String>,
}

/// Type alias for shared immutable configuration
pub type SharedNetworkRuntimeConfig = Arc<NetworkRuntimeConfig>;

impl NetworkRuntimeConfig {
    /// Create a new empty configuration
    ///
    /// Typically used for testing or when building configuration programmatically.
    pub fn new() -> Self {
        Self {
            bind_addresses: HashMap::new(),
            bind_ports: HashMap::new(),
            service_endpoints: HashMap::new(),
            capability_endpoints: HashMap::new(),
        }
    }

    /// Load configuration from environment variables
    ///
    /// This is the primary constructor for production use. It scans for all
    /// NESTGATE_*_BIND_ADDRESS, NESTGATE_*_PORT, and NESTGATE_*_ENDPOINT
    /// environment variables and loads them once.
    ///
    /// # Environment Variable Patterns
    ///
    /// - `NESTGATE_\<SERVICE\>_BIND_ADDRESS`: Bind address for a service
    /// - `NESTGATE_\<SERVICE\>_PORT`: Port for a service
    /// - `NESTGATE_\<SERVICE\>_ENDPOINT`: Endpoint for a service
    /// - `NESTGATE_\<CAPABILITY\>_ENDPOINT`: Endpoint for a capability
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Load all environment variables and extract NESTGATE_* patterns
        for (key, value) in std::env::vars() {
            if let Some(service_key) = key.strip_prefix("NESTGATE_") {
                if let Some(service_name) = service_key.strip_suffix("_BIND_ADDRESS") {
                    if let Ok(addr) = value.parse::<IpAddr>() {
                        config
                            .bind_addresses
                            .insert(service_name.to_lowercase(), addr);
                    }
                } else if let Some(service_name) = service_key.strip_suffix("_PORT") {
                    if let Ok(port) = value.parse::<u16>() {
                        config.bind_ports.insert(service_name.to_lowercase(), port);
                    }
                } else if let Some(service_name) = service_key.strip_suffix("_ENDPOINT") {
                    config
                        .service_endpoints
                        .insert(service_name.to_lowercase(), value.clone());
                    config
                        .capability_endpoints
                        .insert(service_name.to_lowercase(), value);
                }
            }
        }

        config
    }

    /// Set bind address for a service
    pub fn set_bind_address(&mut self, service_name: &str, address: impl Into<IpAddr>) {
        self.bind_addresses
            .insert(service_name.to_lowercase(), address.into());
    }

    /// Get bind address for a service
    pub fn get_bind_address(&self, service_name: &str) -> Option<IpAddr> {
        self.bind_addresses
            .get(&service_name.to_lowercase())
            .copied()
    }

    /// Set port for a service
    pub fn set_bind_port(&mut self, service_name: &str, port: u16) {
        self.bind_ports.insert(service_name.to_lowercase(), port);
    }

    /// Get port for a service
    pub fn get_bind_port(&self, service_name: &str) -> Option<u16> {
        self.bind_ports.get(&service_name.to_lowercase()).copied()
    }

    /// Set service endpoint
    pub fn set_service_endpoint(&mut self, service_name: &str, endpoint: impl Into<String>) {
        self.service_endpoints
            .insert(service_name.to_lowercase(), endpoint.into());
    }

    /// Get service endpoint
    pub fn get_service_endpoint(&self, service_name: &str) -> Option<&str> {
        self.service_endpoints
            .get(&service_name.to_lowercase())
            .map(|s| s.as_str())
    }

    /// Set capability endpoint
    pub fn set_capability_endpoint(&mut self, capability: &str, endpoint: impl Into<String>) {
        self.capability_endpoints
            .insert(capability.to_lowercase(), endpoint.into());
    }

    /// Get capability endpoint
    pub fn get_capability_endpoint(&self, capability: &str) -> Option<&str> {
        self.capability_endpoints
            .get(&capability.to_lowercase())
            .map(|s| s.as_str())
    }

    /// Get all configured service names
    pub fn get_configured_services(&self) -> Vec<&str> {
        let mut services: Vec<&str> = self.service_endpoints.keys().map(|s| s.as_str()).collect();
        services.sort_unstable();
        services.dedup();
        services
    }

    /// Get all configured capabilities
    pub fn get_configured_capabilities(&self) -> Vec<&str> {
        let mut capabilities: Vec<&str> = self
            .capability_endpoints
            .keys()
            .map(|s| s.as_str())
            .collect();
        capabilities.sort_unstable();
        capabilities.dedup();
        capabilities
    }
}

impl Default for NetworkRuntimeConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_config_new() {
        let config = NetworkRuntimeConfig::new();
        assert!(config.get_configured_services().is_empty());
        assert!(config.get_configured_capabilities().is_empty());
    }

    #[test]
    fn test_bind_address() {
        let mut config = NetworkRuntimeConfig::new();

        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        config.set_bind_address("api", addr);

        assert_eq!(config.get_bind_address("api"), Some(addr));
        assert_eq!(config.get_bind_address("API"), Some(addr)); // Case insensitive
        assert_eq!(config.get_bind_address("web"), None);
    }

    #[test]
    fn test_bind_address_ipv6() {
        let mut config = NetworkRuntimeConfig::new();

        let addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        config.set_bind_address("service", addr);

        assert_eq!(config.get_bind_address("service"), Some(addr));
    }

    #[test]
    fn test_bind_port() {
        let mut config = NetworkRuntimeConfig::new();

        config.set_bind_port("api", 8080);
        config.set_bind_port("web", 3000);

        assert_eq!(config.get_bind_port("api"), Some(8080));
        assert_eq!(config.get_bind_port("API"), Some(8080)); // Case insensitive
        assert_eq!(config.get_bind_port("web"), Some(3000));
        assert_eq!(config.get_bind_port("unknown"), None);
    }

    #[test]
    fn test_service_endpoint() {
        let mut config = NetworkRuntimeConfig::new();

        config.set_service_endpoint("storage", "http://storage:9000");
        config.set_service_endpoint("cache", "redis://cache:6379");

        assert_eq!(
            config.get_service_endpoint("storage"),
            Some("http://storage:9000")
        );
        assert_eq!(
            config.get_service_endpoint("STORAGE"),
            Some("http://storage:9000")
        ); // Case insensitive
        assert_eq!(
            config.get_service_endpoint("cache"),
            Some("redis://cache:6379")
        );
        assert_eq!(config.get_service_endpoint("unknown"), None);
    }

    #[test]
    fn test_capability_endpoint() {
        let mut config = NetworkRuntimeConfig::new();

        config.set_capability_endpoint("orchestration", "http://orch:8080");
        config.set_capability_endpoint("security", "http://sec:7070");

        assert_eq!(
            config.get_capability_endpoint("orchestration"),
            Some("http://orch:8080")
        );
        assert_eq!(
            config.get_capability_endpoint("ORCHESTRATION"),
            Some("http://orch:8080")
        ); // Case insensitive
        assert_eq!(
            config.get_capability_endpoint("security"),
            Some("http://sec:7070")
        );
        assert_eq!(config.get_capability_endpoint("unknown"), None);
    }

    #[test]
    fn test_configured_services() {
        let mut config = NetworkRuntimeConfig::new();

        config.set_service_endpoint("api", "http://api:8000");
        config.set_service_endpoint("web", "http://web:3000");
        config.set_service_endpoint("cache", "redis://cache:6379");

        let services = config.get_configured_services();
        assert_eq!(services.len(), 3);
        assert!(services.contains(&"api"));
        assert!(services.contains(&"web"));
        assert!(services.contains(&"cache"));
    }

    #[test]
    fn test_configured_capabilities() {
        let mut config = NetworkRuntimeConfig::new();

        config.set_capability_endpoint("orchestration", "http://orch:8080");
        config.set_capability_endpoint("compute", "http://compute:9090");

        let capabilities = config.get_configured_capabilities();
        assert_eq!(capabilities.len(), 2);
        assert!(capabilities.contains(&"orchestration"));
        assert!(capabilities.contains(&"compute"));
    }

    #[test]
    fn test_from_env_empty() {
        // This test runs without setting specific NESTGATE_* env vars
        let config = NetworkRuntimeConfig::from_env();

        // Config should be created successfully even if empty
        // Actual values depend on environment (len() is always >= 0 for usize)
        let _services = config.get_configured_services();
    }

    #[test]
    fn test_config_shared() {
        let mut config = NetworkRuntimeConfig::new();
        config.set_bind_address("api", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        let shared = Arc::new(config);
        let shared2 = Arc::clone(&shared);

        // Both references see the same data
        assert_eq!(
            shared.get_bind_address("api"),
            shared2.get_bind_address("api")
        );
        assert_eq!(Arc::strong_count(&shared), 2);
    }

    #[test]
    fn test_case_insensitive_keys() {
        let mut config = NetworkRuntimeConfig::new();

        config.set_bind_address("MyService", IpAddr::V4(Ipv4Addr::LOCALHOST));
        config.set_bind_port("MyService", 8080);
        config.set_service_endpoint("MyService", "http://localhost:8080");

        // All access methods should be case-insensitive
        assert!(config.get_bind_address("myservice").is_some());
        assert!(config.get_bind_address("MYSERVICE").is_some());
        assert!(config.get_bind_port("myservice").is_some());
        assert!(config.get_service_endpoint("myservice").is_some());
    }

    #[test]
    fn test_default() {
        let config1 = NetworkRuntimeConfig::default();
        let config2 = NetworkRuntimeConfig::new();

        assert_eq!(
            config1.get_configured_services().len(),
            config2.get_configured_services().len()
        );
    }
}
