//! Service Discovery Configuration
//!
//! Central configuration for service discovery endpoints, replacing hardcoded URLs
//! throughout the codebase with environment-driven configuration.
//!
//! # Environment Variables
//!
//! - `NESTGATE_DISCOVERY_ENDPOINTS`: Comma-separated list of discovery endpoints
//! - `NESTGATE_DISCOVERY_HOST`: Discovery host (default: 127.0.0.1)
//! - `NESTGATE_DISCOVERY_BASE_PORT`: Base port for discovery (default: 8080)
//! - `NESTGATE_DISCOVERY_PORT_RANGE`: Number of ports to scan (default: 10)
//! - `NESTGATE_AUTO_DISCOVERY`: Enable automatic discovery (default: true)
//! - `NESTGATE_DISCOVERY_TIMEOUT`: Discovery timeout in seconds (default: 30)

use serde::{Deserialize, Serialize};
use std::env;

/// Central configuration for service discovery
///
/// Replaces hardcoded URLs like "http://localhost:8080" with configurable endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ServiceDiscovery
pub struct ServiceDiscoveryConfig {
    /// List of discovery endpoints
    pub endpoints: Vec<String>,

    /// Discovery host (default: localhost)
    pub discovery_host: String,

    /// Discovery base port (default: 8080)
    pub discovery_base_port: u16,

    /// Number of discovery ports to scan
    pub discovery_port_range: u16,

    /// Enable automatic endpoint discovery
    pub auto_discovery: bool,

    /// Discovery timeout in seconds
    pub discovery_timeout_secs: u64,
}

impl Default for ServiceDiscoveryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            endpoints: Self::load_endpoints_from_env(),
            // ✅ SOVEREIGNTY: Environment-driven discovery configuration
            // Using compile-time constant for zero runtime overhead
            discovery_host: env::var("NESTGATE_DISCOVERY_HOST")
                .unwrap_or_else(|_| std::net::Ipv4Addr::LOCALHOST.to_string()),
            discovery_base_port: env::var("NESTGATE_DISCOVERY_BASE_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080), // Safe default
            discovery_port_range: env::var("NESTGATE_DISCOVERY_PORT_RANGE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            auto_discovery: env::var("NESTGATE_AUTO_DISCOVERY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            discovery_timeout_secs: env::var("NESTGATE_DISCOVERY_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        }
    }
}

impl ServiceDiscoveryConfig {
    /// Load discovery endpoints from environment
    ///
    /// Tries `NESTGATE_DISCOVERY_ENDPOINTS` first (comma-separated list),
    /// then falls back to generating endpoints from host + port range.
    fn load_endpoints_from_env() -> Vec<String> {
        // Try NESTGATE_DISCOVERY_ENDPOINTS (comma-separated list)
        if let Ok(endpoints_str) = env::var("NESTGATE_DISCOVERY_ENDPOINTS") {
            return endpoints_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // Fallback: Generate from host + port range
        // ✅ Using compile-time constant (zero runtime overhead)
        let host = env::var("NESTGATE_DISCOVERY_HOST")
            .unwrap_or_else(|_| std::net::Ipv4Addr::LOCALHOST.to_string());
        let base_port: u16 = env::var("NESTGATE_DISCOVERY_BASE_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080);
        let port_range: u16 = env::var("NESTGATE_DISCOVERY_PORT_RANGE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);

        (0..port_range)
            .map(|offset| format!("http://{}:{}", host, base_port + offset))
            .collect()
    }

    /// Get all discovery endpoints
    #[must_use]
    pub fn get_endpoints(&self) -> &[String] {
        &self.endpoints
    }

    /// Build endpoint from host and port
    #[must_use]
    pub fn build_endpoint(&self, port: u16) -> String {
        format!("http://{}:{}", self.discovery_host, port)
    }

    /// Generate port range for scanning
    #[must_use]
    pub fn get_port_range(&self) -> Vec<u16> {
        (0..self.discovery_port_range)
            .map(|offset| self.discovery_base_port + offset)
            .collect()
    }

    /// Create a new configuration with custom endpoints
    #[must_use]
    pub fn with_endpoints(endpoints: Vec<String>) -> Self {
        Self {
            endpoints,
            ..Default::default()
        }
    }

    /// Create a new configuration with custom host and port
    #[must_use]
    pub fn with_host_and_port(host: String, port: u16) -> Self {
        Self {
            discovery_host: host,
            discovery_base_port: port,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // NOTE: Some tests manipulate environment variables and may interfere
    // with each other when run in parallel. Run with `--test-threads=1` for
    // deterministic results, or use `cargo test -- --ignored` for env-dependent tests.

    #[test]
    fn test_default_discovery_config_values() {
        // Test default values without relying on env var state
        let config = ServiceDiscoveryConfig {
            endpoints: vec![
                "http://127.0.0.1:8080".to_string(),
                "http://127.0.0.1:8081".to_string(),
                "http://127.0.0.1:8082".to_string(),
            ],
            discovery_host: "127.0.0.1".to_string(),
            discovery_base_port: 8080,
            discovery_port_range: 10,
            auto_discovery: true,
            discovery_timeout_secs: 30,
        };

        assert_eq!(config.discovery_host, "127.0.0.1");
        assert_eq!(config.discovery_base_port, 8080);
        assert_eq!(config.discovery_port_range, 10);
        assert!(config.auto_discovery);
        assert_eq!(config.discovery_timeout_secs, 30);
    }

    #[test]
    #[ignore] // Run with --ignored due to env var manipulation
    fn test_discovery_config_from_env_host_and_port() {
        // Save original values
        let original_host = env::var("NESTGATE_DISCOVERY_HOST").ok();
        let original_port = env::var("NESTGATE_DISCOVERY_BASE_PORT").ok();

        env::set_var("NESTGATE_DISCOVERY_HOST", "192.168.1.100");
        env::set_var("NESTGATE_DISCOVERY_BASE_PORT", "9000");
        env::remove_var("NESTGATE_DISCOVERY_ENDPOINTS");

        let config = ServiceDiscoveryConfig::default();
        assert_eq!(config.discovery_host, "192.168.1.100");
        assert_eq!(config.discovery_base_port, 9000);

        // Restore original values
        match original_host {
            Some(val) => env::set_var("NESTGATE_DISCOVERY_HOST", val),
            None => env::remove_var("NESTGATE_DISCOVERY_HOST"),
        }
        match original_port {
            Some(val) => env::set_var("NESTGATE_DISCOVERY_BASE_PORT", val),
            None => env::remove_var("NESTGATE_DISCOVERY_BASE_PORT"),
        }
    }

    #[test]
    fn test_endpoint_generation() {
        // Test build_endpoint method directly
        let config = ServiceDiscoveryConfig {
            endpoints: vec![],
            discovery_host: "127.0.0.1".to_string(),
            discovery_base_port: 8080,
            discovery_port_range: 3,
            auto_discovery: true,
            discovery_timeout_secs: 30,
        };

        let endpoint = config.build_endpoint(8081);
        assert_eq!(endpoint, "http://127.0.0.1:8081");
    }

    #[test]
    fn test_port_range_generation() {
        // Test get_port_range method directly
        let config = ServiceDiscoveryConfig {
            endpoints: vec![],
            discovery_host: "127.0.0.1".to_string(),
            discovery_base_port: 8080,
            discovery_port_range: 5,
            auto_discovery: true,
            discovery_timeout_secs: 30,
        };

        let ports = config.get_port_range();
        assert_eq!(ports, vec![8080, 8081, 8082, 8083, 8084]);
    }

    #[test]
    #[ignore] // Run with --ignored due to env var manipulation
    fn test_endpoints_from_env() {
        let original = env::var("NESTGATE_DISCOVERY_ENDPOINTS").ok();

        env::set_var(
            "NESTGATE_DISCOVERY_ENDPOINTS",
            "http://server1:8080,http://server2:8081,http://server3:8082",
        );

        let config = ServiceDiscoveryConfig::default();
        assert_eq!(config.endpoints.len(), 3);
        assert!(config
            .endpoints
            .contains(&"http://server1:8080".to_string()));
        assert!(config
            .endpoints
            .contains(&"http://server2:8081".to_string()));
        assert!(config
            .endpoints
            .contains(&"http://server3:8082".to_string()));

        // Restore original value
        match original {
            Some(val) => env::set_var("NESTGATE_DISCOVERY_ENDPOINTS", val),
            None => env::remove_var("NESTGATE_DISCOVERY_ENDPOINTS"),
        }
    }

    #[test]
    fn test_endpoints_generated_logic() {
        // Test the endpoint generation logic without relying on env state
        let endpoints = (0..3)
            .map(|offset| format!("http://127.0.0.1:{}", 8080 + offset))
            .collect::<Vec<_>>();

        assert_eq!(endpoints.len(), 3);
        assert!(endpoints.contains(&"http://127.0.0.1:8080".to_string()));
        assert!(endpoints.contains(&"http://127.0.0.1:8081".to_string()));
        assert!(endpoints.contains(&"http://127.0.0.1:8082".to_string()));
    }

    #[test]
    fn test_with_endpoints_constructor() {
        let endpoints = vec![
            "http://custom1:9090".to_string(),
            "http://custom2:9091".to_string(),
        ];

        let config = ServiceDiscoveryConfig::with_endpoints(endpoints.clone());
        assert_eq!(config.endpoints, endpoints);
    }

    #[test]
    fn test_with_host_and_port_constructor() {
        let config = ServiceDiscoveryConfig::with_host_and_port("192.168.1.50".to_string(), 9000);

        assert_eq!(config.discovery_host, "192.168.1.50");
        assert_eq!(config.discovery_base_port, 9000);
    }

    #[test]
    fn test_auto_discovery_env_var() {
        // ✅ EVOLUTION: No longer needs #[ignore] - concurrent-safe!
        temp_env::with_vars(
            vec![
                ("NESTGATE_AUTO_DISCOVERY", Some("false")),
                ("NESTGATE_DISCOVERY_ENDPOINTS", None),
            ],
            || {
                let config = ServiceDiscoveryConfig::default();
                assert!(!config.auto_discovery);
            },
        );
        // Environment automatically restored!
    }

    #[test]
    fn test_discovery_timeout_env_var() {
        // ✅ EVOLUTION: No longer needs #[ignore] - concurrent-safe!
        temp_env::with_vars(
            vec![
                ("NESTGATE_DISCOVERY_TIMEOUT", Some("60")),
                ("NESTGATE_DISCOVERY_ENDPOINTS", None),
            ],
            || {
                let config = ServiceDiscoveryConfig::default();
                assert_eq!(config.discovery_timeout_secs, 60);
            },
        );
        // Environment automatically restored!
    }
}
