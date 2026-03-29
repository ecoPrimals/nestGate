// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Configuration for UniversalAdapter discovery
//!
//! This module provides immutable configuration for the UniversalAdapter's
//! capability discovery, eliminating the need for runtime environment variable
//! access and enabling concurrent-safe operation.
//!
//! # Architecture
//!
//! The configuration follows NestGate's modern concurrency pattern:
//! - Immutable configuration loaded once
//! - Wrapped in Arc for zero-cost sharing
//! - No global mutable state
//! - Thread-safe by design
//!
//! # Usage
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use nestgate_core::universal_adapter::AdapterDiscoveryConfig;
//!
//! // Load from environment (production)
//! let config = Arc::new(AdapterDiscoveryConfig::from_env());
//! let adapter = UniversalAdapter::with_discovery_config(config, endpoint);
//!
//! // Or create specific config (testing)
//! let mut config = AdapterDiscoveryConfig::new();
//! config.set_discovery_endpoint("orchestration", "http://orch:8080");
//! let adapter = UniversalAdapter::with_discovery_config(Arc::new(config), endpoint);
//! ```

use std::collections::HashMap;
use std::sync::Arc;

/// Immutable configuration for UniversalAdapter capability discovery
///
/// This struct holds all configuration needed for the UniversalAdapter to
/// discover capabilities without accessing environment variables at runtime.
///
/// Note: This is separate from the deprecated `UniversalAdapterConfig` which
/// handled cache/timeout settings. This config specifically manages discovery
/// endpoints for eliminating runtime `env::var()` calls.
#[derive(Debug, Clone)]
/// Configuration for AdapterDiscovery
pub struct AdapterDiscoveryConfig {
    /// Discovery endpoints for various capability categories
    discovery_endpoints: HashMap<String, String>,

    /// Adapter endpoint override
    adapter_endpoint: Option<String>,

    /// Host configuration (for fallback endpoint construction)
    host: String,

    /// Port configuration (for fallback endpoint construction)
    port: String,

    /// Additional metadata for capabilities
    metadata: HashMap<String, HashMap<String, String>>,
}

/// Type alias for shared immutable configuration
pub type SharedDiscoveryConfig = Arc<AdapterDiscoveryConfig>;

impl AdapterDiscoveryConfig {
    /// Create a new empty configuration
    ///
    /// ✅ MIGRATED: Now uses centralized runtime configuration
    /// Typically used for testing or when building configuration programmatically.
    #[must_use]
    pub fn new() -> Self {
        use crate::config::runtime::get_config;
        let config = get_config();

        Self {
            discovery_endpoints: HashMap::new(),
            adapter_endpoint: None,
            host: config.network.api_host.to_string(),
            port: config.network.api_port.to_string(),
            metadata: HashMap::new(),
        }
    }

    /// Load configuration from environment variables
    ///
    /// This is the primary constructor for production use. It reads all relevant
    /// environment variables once and creates an immutable configuration.
    ///
    /// # Environment Variables
    ///
    /// - `ORCHESTRATION_DISCOVERY_ENDPOINT`: Orchestration capability endpoint
    /// - `COMPUTE_DISCOVERY_ENDPOINT`: Compute capability endpoint
    /// - `SECURITY_DISCOVERY_ENDPOINT`: Security capability endpoint
    /// - `AI_DISCOVERY_ENDPOINT`: AI capability endpoint
    /// - `ECOSYSTEM_DISCOVERY_ENDPOINT`: Ecosystem capability endpoint
    /// - `UNIVERSAL_ADAPTER_ENDPOINT`: Adapter endpoint override
    /// - `NESTGATE_HOST`: Host for fallback endpoints (default: "localhost")
    /// - `NESTGATE_PORT`: Port for fallback endpoints (default: "8080")
    #[must_use]
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Load discovery endpoints
        if let Ok(endpoint) = std::env::var("ORCHESTRATION_DISCOVERY_ENDPOINT") {
            config
                .discovery_endpoints
                .insert("orchestration".to_string(), endpoint);
        }

        if let Ok(endpoint) = std::env::var("COMPUTE_DISCOVERY_ENDPOINT") {
            config
                .discovery_endpoints
                .insert("compute".to_string(), endpoint);
        }

        if let Ok(endpoint) = std::env::var("SECURITY_DISCOVERY_ENDPOINT") {
            config
                .discovery_endpoints
                .insert("security".to_string(), endpoint);
        }

        if let Ok(endpoint) = std::env::var("AI_DISCOVERY_ENDPOINT") {
            config
                .discovery_endpoints
                .insert("artificial_intelligence".to_string(), endpoint);
        }

        if let Ok(endpoint) = std::env::var("ECOSYSTEM_DISCOVERY_ENDPOINT") {
            config
                .discovery_endpoints
                .insert("ecosystem".to_string(), endpoint);
        }

        // ✅ MIGRATED: Load from centralized runtime config
        use crate::config::runtime::get_config;
        let runtime_config = get_config();

        // Load adapter endpoint override
        config.adapter_endpoint = std::env::var("UNIVERSAL_ADAPTER_ENDPOINT").ok();

        // Load host/port from centralized config
        config.host = runtime_config.network.api_host.to_string();
        config.port = runtime_config.network.api_port.to_string();

        config
    }

    /// Set a discovery endpoint for a specific capability category
    pub fn set_discovery_endpoint(&mut self, category: &str, endpoint: impl Into<String>) {
        self.discovery_endpoints
            .insert(category.to_string(), endpoint.into());
    }

    /// Get a discovery endpoint for a specific capability category
    #[must_use]
    pub fn get_discovery_endpoint(&self, category: &str) -> Option<&str> {
        self.discovery_endpoints.get(category).map(|s| s.as_str())
    }

    /// Set the adapter endpoint override
    pub fn set_adapter_endpoint(&mut self, endpoint: impl Into<String>) {
        self.adapter_endpoint = Some(endpoint.into());
    }

    /// Get the adapter endpoint override
    #[must_use]
    pub fn get_adapter_endpoint(&self) -> Option<&str> {
        self.adapter_endpoint.as_deref()
    }

    /// Get the default adapter endpoint constructed from host and port
    #[must_use]
    pub fn get_default_adapter_endpoint(&self) -> String {
        format!("http://{}:{}/adapter", self.host, self.port)
    }

    /// Get the adapter endpoint (override or default)
    #[must_use]
    pub fn get_effective_adapter_endpoint(&self) -> String {
        self.adapter_endpoint
            .clone()
            .unwrap_or_else(|| self.get_default_adapter_endpoint())
    }

    /// Set host for fallback endpoint construction
    pub fn set_host(&mut self, host: impl Into<String>) {
        self.host = host.into();
    }

    /// Get host
    #[must_use]
    pub fn get_host(&self) -> &str {
        &self.host
    }

    /// Set port for fallback endpoint construction
    pub fn set_port(&mut self, port: impl Into<String>) {
        self.port = port.into();
    }

    /// Get port
    #[must_use]
    pub fn get_port(&self) -> &str {
        &self.port
    }

    /// Set metadata for a specific category
    pub fn set_metadata(&mut self, category: &str, key: &str, value: impl Into<String>) {
        self.metadata
            .entry(category.to_string())
            .or_default()
            .insert(key.to_string(), value.into());
    }

    /// Get metadata for a specific category
    #[must_use]
    pub fn get_metadata(&self, category: &str) -> Option<&HashMap<String, String>> {
        self.metadata.get(category)
    }

    /// Get all configured discovery endpoints
    #[must_use]
    pub const fn get_all_discovery_endpoints(&self) -> &HashMap<String, String> {
        &self.discovery_endpoints
    }
}

impl Default for AdapterDiscoveryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        use crate::constants::hardcoding::runtime_fallback_ports;
        let config = AdapterDiscoveryConfig::new();
        // Runtime config uses 127.0.0.1 (IpAddr format) instead of "localhost"
        assert_eq!(config.get_host(), "127.0.0.1");
        assert_eq!(config.get_port(), &runtime_fallback_ports::HTTP.to_string());
        assert!(config.get_adapter_endpoint().is_none());
        assert!(config.get_all_discovery_endpoints().is_empty());
    }

    #[test]
    fn test_config_set_get_discovery_endpoint() {
        use crate::constants::hardcoding::runtime_fallback_ports;
        let mut config = AdapterDiscoveryConfig::new();

        let orch_endpoint = format!("http://orch:{}", runtime_fallback_ports::HTTP);
        config.set_discovery_endpoint("orchestration", &orch_endpoint);
        config.set_discovery_endpoint(
            "compute",
            format!("http://compute:{}", runtime_fallback_ports::PROMETHEUS),
        );

        assert_eq!(
            config.get_discovery_endpoint("orchestration"),
            Some(orch_endpoint.as_str())
        );
        assert_eq!(
            config.get_discovery_endpoint("compute"),
            Some(format!("http://compute:{}", runtime_fallback_ports::PROMETHEUS).as_str())
        );
        assert_eq!(config.get_discovery_endpoint("security"), None);
    }

    #[test]
    fn test_config_adapter_endpoint() {
        use crate::constants::hardcoding::runtime_fallback_ports;
        let mut config = AdapterDiscoveryConfig::new();

        // Runtime config uses 127.0.0.1 instead of "localhost"
        let default_endpoint = format!("http://127.0.0.1:{}/adapter", runtime_fallback_ports::HTTP);
        // Default endpoint
        assert_eq!(config.get_default_adapter_endpoint(), default_endpoint);
        assert_eq!(config.get_effective_adapter_endpoint(), default_endpoint);

        // Set override
        let custom_endpoint = format!("http://custom:{}/adapter", runtime_fallback_ports::API);
        config.set_adapter_endpoint(&custom_endpoint);
        assert_eq!(
            config.get_adapter_endpoint(),
            Some(custom_endpoint.as_str())
        );
        assert_eq!(config.get_effective_adapter_endpoint(), custom_endpoint);
    }

    #[test]
    fn test_config_host_port() {
        let mut config = AdapterDiscoveryConfig::new();

        config.set_host("example.com");
        config.set_port("9000");

        assert_eq!(config.get_host(), "example.com");
        assert_eq!(config.get_port(), "9000");
        assert_eq!(
            config.get_default_adapter_endpoint(),
            "http://example.com:9000/adapter"
        );
    }

    #[test]
    fn test_config_metadata() {
        let mut config = AdapterDiscoveryConfig::new();

        config.set_metadata("orchestration", "tier", "premium");
        config.set_metadata("orchestration", "region", "us-west");
        config.set_metadata("compute", "tier", "standard");

        let orch_meta = config
            .get_metadata("orchestration")
            .expect("Test: orchestration metadata should exist");
        assert_eq!(orch_meta.get("tier"), Some(&"premium".to_string()));
        assert_eq!(orch_meta.get("region"), Some(&"us-west".to_string()));

        let compute_meta = config
            .get_metadata("compute")
            .expect("Test: compute metadata should exist");
        assert_eq!(compute_meta.get("tier"), Some(&"standard".to_string()));

        assert!(config.get_metadata("security").is_none());
    }

    #[test]
    fn test_config_from_env_with_no_vars() {
        // This test runs without setting env vars, should get defaults
        let config = AdapterDiscoveryConfig::from_env();

        use crate::constants::hardcoding::runtime_fallback_ports;
        // Runtime config uses 127.0.0.1 instead of "localhost"
        assert_eq!(config.get_host(), "127.0.0.1");
        assert_eq!(config.get_port(), &runtime_fallback_ports::HTTP.to_string());
        // discovery_endpoints might be empty if no env vars are set
    }

    #[test]
    fn test_config_shared() {
        let config = Arc::new(AdapterDiscoveryConfig::new());
        let config_clone = Arc::clone(&config);

        // Both should point to same data
        assert_eq!(config.get_host(), config_clone.get_host());
        assert_eq!(Arc::strong_count(&config), 2);
    }

    #[test]
    fn test_config_all_discovery_endpoints() {
        use crate::constants::hardcoding::runtime_fallback_ports;
        let mut config = AdapterDiscoveryConfig::new();

        let orch_endpoint = format!("http://orch:{}", runtime_fallback_ports::HTTP);
        config.set_discovery_endpoint("orchestration", &orch_endpoint);
        config.set_discovery_endpoint(
            "compute",
            format!("http://compute:{}", runtime_fallback_ports::PROMETHEUS),
        );
        let sec_endpoint = "http://sec:7070".to_string();
        config.set_discovery_endpoint("security", &sec_endpoint);

        let endpoints = config.get_all_discovery_endpoints();
        assert_eq!(endpoints.len(), 3);
        assert_eq!(endpoints.get("orchestration"), Some(&orch_endpoint));
        assert_eq!(
            endpoints.get("compute"),
            Some(&format!(
                "http://compute:{}",
                runtime_fallback_ports::PROMETHEUS
            ))
        );
        assert_eq!(
            endpoints.get("security"),
            Some(&"http://sec:7070".to_string())
        );
    }

    #[test]
    fn test_config_default() {
        let config1 = AdapterDiscoveryConfig::default();
        let config2 = AdapterDiscoveryConfig::new();

        assert_eq!(config1.get_host(), config2.get_host());
        assert_eq!(config1.get_port(), config2.get_port());
    }
}
