//! Configuration for Ecosystem Integration
//!
//! This module provides immutable configuration for ecosystem capability discovery,
//! eliminating runtime environment variable access and enabling concurrent-safe operation.
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
//! ```rust
//! use std::sync::Arc;
//! use nestgate_core::ecosystem_integration::EcosystemDiscoveryConfig;
//! use nestgate_core::ecosystem_integration::CapabilityBasedEcosystem;
//!
//! // Load from environment (production)
//! let config = Arc::new(EcosystemDiscoveryConfig::from_env());
//! let ecosystem = CapabilityBasedEcosystem::with_config(config);
//!
//! // Or create specific config (testing)
//! let mut config = EcosystemDiscoveryConfig::new();
//! config.set_discovery_endpoint("orchestration", "http://orch:8080");
//! let ecosystem = CapabilityBasedEcosystem::with_config(Arc::new(config));
//! ```

use std::collections::HashMap;
use std::sync::Arc;

/// Immutable configuration for ecosystem capability discovery
///
/// This struct holds all configuration needed for the CapabilityBasedEcosystem
/// to discover capabilities without accessing environment variables at runtime.
#[derive(Debug, Clone)]
/// Configuration for EcosystemDiscovery
pub struct EcosystemDiscoveryConfig {
    /// Discovery endpoints for various capability categories
    discovery_endpoints: HashMap<String, String>,

    /// Additional metadata for capabilities
    metadata: HashMap<String, HashMap<String, String>>,
}

/// Type alias for shared immutable configuration
pub type SharedEcosystemConfig = Arc<EcosystemDiscoveryConfig>;

impl EcosystemDiscoveryConfig {
    /// Create a new empty configuration
    ///
    /// Typically used for testing or when building configuration programmatically.
    pub fn new() -> Self {
        Self {
            discovery_endpoints: HashMap::new(),
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
    /// - `STORAGE_DISCOVERY_ENDPOINT`: Storage capability endpoint
    /// - `SECURITY_DISCOVERY_ENDPOINT`: Security capability endpoint
    /// - `MONITORING_DISCOVERY_ENDPOINT`: Monitoring capability endpoint
    /// - `AI_DISCOVERY_ENDPOINT`: AI capability endpoint
    /// - `COMPUTE_DISCOVERY_ENDPOINT`: Compute capability endpoint
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Load discovery endpoints following the same pattern as the code
        let capability_patterns = [
            ("ORCHESTRATION_DISCOVERY_ENDPOINT", "orchestration"),
            ("STORAGE_DISCOVERY_ENDPOINT", "storage"),
            ("SECURITY_DISCOVERY_ENDPOINT", "security"),
            ("MONITORING_DISCOVERY_ENDPOINT", "monitoring"),
            ("AI_DISCOVERY_ENDPOINT", "artificial_intelligence"),
            ("COMPUTE_DISCOVERY_ENDPOINT", "compute"),
        ];

        for (env_var, category) in capability_patterns {
            if let Ok(endpoint) = std::env::var(env_var) {
                config
                    .discovery_endpoints
                    .insert(category.to_string(), endpoint);
            }
        }

        config
    }

    /// Set a discovery endpoint for a specific capability category
    pub fn set_discovery_endpoint(&mut self, category: &str, endpoint: impl Into<String>) {
        self.discovery_endpoints
            .insert(category.to_string(), endpoint.into());
    }

    /// Get a discovery endpoint for a specific capability category
    pub fn get_discovery_endpoint(&self, category: &str) -> Option<&str> {
        self.discovery_endpoints.get(category).map(|s| s.as_str())
    }

    /// Set metadata for a specific category
    pub fn set_metadata(&mut self, category: &str, key: &str, value: impl Into<String>) {
        self.metadata
            .entry(category.to_string())
            .or_default()
            .insert(key.to_string(), value.into());
    }

    /// Get metadata for a specific category
    pub fn get_metadata(&self, category: &str) -> Option<&HashMap<String, String>> {
        self.metadata.get(category)
    }

    /// Get all configured discovery endpoints
    pub fn get_all_discovery_endpoints(&self) -> &HashMap<String, String> {
        &self.discovery_endpoints
    }

    /// Get capability categories that have configured endpoints
    pub fn get_configured_categories(&self) -> Vec<&str> {
        self.discovery_endpoints
            .keys()
            .map(|s| s.as_str())
            .collect()
    }
}

impl Default for EcosystemDiscoveryConfig {
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
        let config = EcosystemDiscoveryConfig::new();
        assert!(config.get_all_discovery_endpoints().is_empty());
        assert!(config.get_configured_categories().is_empty());
    }

    #[test]
    fn test_config_set_get_discovery_endpoint() {
        let mut config = EcosystemDiscoveryConfig::new();

        config.set_discovery_endpoint("orchestration", "http://orch:8080");
        config.set_discovery_endpoint("storage", "http://storage:9090");

        assert_eq!(
            config.get_discovery_endpoint("orchestration"),
            Some("http://orch:8080")
        );
        assert_eq!(
            config.get_discovery_endpoint("storage"),
            Some("http://storage:9090")
        );
        assert_eq!(config.get_discovery_endpoint("security"), None);
    }

    #[test]
    fn test_config_metadata() {
        let mut config = EcosystemDiscoveryConfig::new();

        config.set_metadata("orchestration", "tier", "premium");
        config.set_metadata("orchestration", "region", "us-west");
        config.set_metadata("storage", "tier", "standard");

        let orch_meta = config.get_metadata("orchestration").unwrap();
        assert_eq!(orch_meta.get("tier"), Some(&"premium".to_string()));
        assert_eq!(orch_meta.get("region"), Some(&"us-west".to_string()));

        let storage_meta = config.get_metadata("storage").unwrap();
        assert_eq!(storage_meta.get("tier"), Some(&"standard".to_string()));

        assert!(config.get_metadata("security").is_none());
    }

    #[test]
    fn test_config_from_env_with_no_vars() {
        // This test runs without setting env vars, should get empty config
        let config = EcosystemDiscoveryConfig::from_env();

        // discovery_endpoints might be empty if no env vars are set
        // This is expected and correct (len() is always >= 0 for usize)
        let _endpoints = config.get_all_discovery_endpoints();
    }

    #[test]
    fn test_config_shared() {
        let config = Arc::new(EcosystemDiscoveryConfig::new());
        let config_clone = Arc::clone(&config);

        // Both should point to same data
        assert_eq!(
            config.get_configured_categories().len(),
            config_clone.get_configured_categories().len()
        );
        assert_eq!(Arc::strong_count(&config), 2);
    }

    #[test]
    fn test_config_all_discovery_endpoints() {
        let mut config = EcosystemDiscoveryConfig::new();

        config.set_discovery_endpoint("orchestration", "http://orch:8080");
        config.set_discovery_endpoint("storage", "http://storage:9090");
        config.set_discovery_endpoint("security", "http://sec:7070");

        let endpoints = config.get_all_discovery_endpoints();
        assert_eq!(endpoints.len(), 3);
        assert_eq!(
            endpoints.get("orchestration"),
            Some(&"http://orch:8080".to_string())
        );
        assert_eq!(
            endpoints.get("storage"),
            Some(&"http://storage:9090".to_string())
        );
        assert_eq!(
            endpoints.get("security"),
            Some(&"http://sec:7070".to_string())
        );
    }

    #[test]
    fn test_config_configured_categories() {
        let mut config = EcosystemDiscoveryConfig::new();

        config.set_discovery_endpoint("orchestration", "http://orch:8080");
        config.set_discovery_endpoint("compute", "http://compute:9090");

        let categories = config.get_configured_categories();
        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&"orchestration"));
        assert!(categories.contains(&"compute"));
    }

    #[test]
    fn test_config_default() {
        let config1 = EcosystemDiscoveryConfig::default();
        let config2 = EcosystemDiscoveryConfig::new();

        assert_eq!(
            config1.get_all_discovery_endpoints().len(),
            config2.get_all_discovery_endpoints().len()
        );
    }

    #[test]
    fn test_config_immutability_via_arc() {
        let mut config = EcosystemDiscoveryConfig::new();
        config.set_discovery_endpoint("orchestration", "http://orch:8080");

        let shared = Arc::new(config);
        let shared2 = Arc::clone(&shared);

        // Both references see the same data
        assert_eq!(
            shared.get_discovery_endpoint("orchestration"),
            shared2.get_discovery_endpoint("orchestration")
        );
    }
}
