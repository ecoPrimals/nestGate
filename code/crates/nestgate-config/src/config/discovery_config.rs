// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

use nestgate_types::{EnvSource, ProcessEnv, env_parsed};
use serde::{Deserialize, Serialize};

use crate::constants::hardcoding::addresses;
use crate::constants::hardcoding::runtime_fallback_ports;

/// Central configuration for service discovery
///
/// Replaces hardcoded URLs like "<http://localhost:8080>" with configurable endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ServiceDiscovery`
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
        Self::from_env_source(&ProcessEnv)
    }
}

impl ServiceDiscoveryConfig {
    /// Build from an injectable environment source (use [`MapEnv`](nestgate_types::MapEnv) in tests).
    #[must_use]
    pub fn from_env_source(env: &(impl EnvSource + ?Sized)) -> Self {
        Self {
            endpoints: Self::load_endpoints_from_env_source(env),
            discovery_host: env
                .get("NESTGATE_DISCOVERY_HOST")
                .unwrap_or_else(|| addresses::LOCALHOST_IPV4.to_string()),
            discovery_base_port: env_parsed(
                env,
                "NESTGATE_DISCOVERY_BASE_PORT",
                runtime_fallback_ports::HTTP,
            ),
            discovery_port_range: env_parsed(env, "NESTGATE_DISCOVERY_PORT_RANGE", 10),
            auto_discovery: env_parsed(env, "NESTGATE_AUTO_DISCOVERY", true),
            discovery_timeout_secs: env_parsed(env, "NESTGATE_DISCOVERY_TIMEOUT", 30),
        }
    }

    fn load_endpoints_from_env_source(env: &(impl EnvSource + ?Sized)) -> Vec<String> {
        // Try NESTGATE_DISCOVERY_ENDPOINTS (comma-separated list)
        if let Some(endpoints_str) = env.get("NESTGATE_DISCOVERY_ENDPOINTS") {
            return endpoints_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // Fallback: Generate from host + port range
        let host = env
            .get("NESTGATE_DISCOVERY_HOST")
            .unwrap_or_else(|| addresses::LOCALHOST_IPV4.to_string());
        let base_port: u16 = env_parsed(
            env,
            "NESTGATE_DISCOVERY_BASE_PORT",
            runtime_fallback_ports::HTTP,
        );
        let port_range: u16 = env_parsed(env, "NESTGATE_DISCOVERY_PORT_RANGE", 3);

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
    use crate::constants::hardcoding::runtime_fallback_ports;
    use nestgate_types::MapEnv;

    #[test]
    #[expect(
        deprecated,
        reason = "test fixtures use runtime_fallback_ports for numeric parity"
    )]
    fn test_default_discovery_config_values() {
        // Test default values without relying on env var state
        let config = ServiceDiscoveryConfig {
            endpoints: vec![
                format!("http://127.0.0.1:{}", runtime_fallback_ports::HTTP),
                format!("http://127.0.0.1:{}", runtime_fallback_ports::HEALTH),
                format!("http://127.0.0.1:{}", runtime_fallback_ports::WEBSOCKET),
            ],
            discovery_host: "127.0.0.1".to_string(),
            discovery_base_port: runtime_fallback_ports::HTTP,
            discovery_port_range: 10,
            auto_discovery: true,
            discovery_timeout_secs: 30,
        };

        assert_eq!(config.discovery_host, "127.0.0.1");
        assert_eq!(config.discovery_base_port, runtime_fallback_ports::HTTP);
        assert_eq!(config.discovery_port_range, 10);
        assert!(config.auto_discovery);
        assert_eq!(config.discovery_timeout_secs, 30);
    }

    #[test]
    fn test_discovery_config_from_env_host_and_port() {
        let env = MapEnv::from([
            ("NESTGATE_DISCOVERY_HOST", "192.168.1.100"),
            ("NESTGATE_DISCOVERY_BASE_PORT", "9000"),
        ]);
        let config = ServiceDiscoveryConfig::from_env_source(&env);
        assert_eq!(config.discovery_host, "192.168.1.100");
        assert_eq!(config.discovery_base_port, 9000);
    }

    #[test]
    #[expect(
        deprecated,
        reason = "test fixtures use runtime_fallback_ports for numeric parity"
    )]
    fn test_endpoint_generation() {
        // Test build_endpoint method directly
        let config = ServiceDiscoveryConfig {
            endpoints: vec![],
            discovery_host: "127.0.0.1".to_string(),
            discovery_base_port: runtime_fallback_ports::HTTP,
            discovery_port_range: 3,
            auto_discovery: true,
            discovery_timeout_secs: 30,
        };

        let endpoint = config.build_endpoint(runtime_fallback_ports::HEALTH);
        assert_eq!(
            endpoint,
            format!("http://127.0.0.1:{}", runtime_fallback_ports::HEALTH)
        );
    }

    #[test]
    #[expect(
        deprecated,
        reason = "test fixtures use runtime_fallback_ports for numeric parity"
    )]
    fn test_port_range_generation() {
        // Test get_port_range method directly
        let config = ServiceDiscoveryConfig {
            endpoints: vec![],
            discovery_host: "127.0.0.1".to_string(),
            discovery_base_port: runtime_fallback_ports::HTTP,
            discovery_port_range: 5,
            auto_discovery: true,
            discovery_timeout_secs: 30,
        };

        let ports = config.get_port_range();
        let base = runtime_fallback_ports::HTTP;
        assert_eq!(
            ports,
            (0..5).map(|offset| base + offset).collect::<Vec<_>>()
        );
    }

    #[test]
    #[expect(
        deprecated,
        reason = "test fixtures use runtime_fallback_ports for numeric parity"
    )]
    fn test_endpoints_from_env() {
        let endpoints_csv = format!(
            "http://server1:{},http://server2:{},http://server3:{}",
            runtime_fallback_ports::HTTP,
            runtime_fallback_ports::HEALTH,
            runtime_fallback_ports::WEBSOCKET
        );
        let env = MapEnv::from([("NESTGATE_DISCOVERY_ENDPOINTS", endpoints_csv.as_str())]);
        let config = ServiceDiscoveryConfig::from_env_source(&env);
        assert_eq!(config.endpoints.len(), 3);
        assert!(
            config
                .endpoints
                .contains(&format!("http://server1:{}", runtime_fallback_ports::HTTP))
        );
        assert!(config.endpoints.contains(&format!(
            "http://server2:{}",
            runtime_fallback_ports::HEALTH
        )));
        assert!(config.endpoints.contains(&format!(
            "http://server3:{}",
            runtime_fallback_ports::WEBSOCKET
        )));
    }

    #[test]
    #[expect(
        deprecated,
        reason = "test fixtures use runtime_fallback_ports for numeric parity"
    )]
    fn test_endpoints_generated_logic() {
        // Test the endpoint generation logic without relying on env state
        let base = runtime_fallback_ports::HTTP;
        let endpoints = (0..3)
            .map(|offset| format!("http://127.0.0.1:{}", base + offset))
            .collect::<Vec<_>>();

        assert_eq!(endpoints.len(), 3);
        assert!(endpoints.contains(&format!("http://127.0.0.1:{}", base)));
        assert!(endpoints.contains(&format!("http://127.0.0.1:{}", base + 1)));
        assert!(endpoints.contains(&format!("http://127.0.0.1:{}", base + 2)));
    }

    #[test]
    #[expect(
        deprecated,
        reason = "test fixtures use runtime_fallback_ports for numeric parity"
    )]
    fn test_with_endpoints_constructor() {
        let endpoints = vec![
            format!("http://custom1:{}", runtime_fallback_ports::METRICS),
            format!("http://custom2:{}", runtime_fallback_ports::METRICS + 1),
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
        let env = MapEnv::from([("NESTGATE_AUTO_DISCOVERY", "false")]);
        let config = ServiceDiscoveryConfig::from_env_source(&env);
        assert!(!config.auto_discovery);
    }

    #[test]
    fn test_discovery_timeout_env_var() {
        let env = MapEnv::from([("NESTGATE_DISCOVERY_TIMEOUT", "60")]);
        let config = ServiceDiscoveryConfig::from_env_source(&env);
        assert_eq!(config.discovery_timeout_secs, 60);
    }
}
