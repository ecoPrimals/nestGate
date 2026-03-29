// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Configuration types for capability discovery
//!
//! This module provides thread-safe, testable configuration for the discovery system,
//! eliminating reliance on global mutable state (environment variables).
//!
//! # Modern Rust Patterns
//!
//! - **Dependency Injection**: Configuration is passed in, not read from globals
//! - **Immutable by Default**: Configuration is immutable after construction
//! - **Thread-Safe**: Safe to share across threads via `Arc`
//! - **Testable**: Easy to test without environment pollution
//! - **Zero-Cost**: No runtime overhead vs direct env var access

use std::collections::HashMap;
use std::sync::Arc;

/// Configuration for environment-based discovery
///
/// This replaces direct `env::var()` calls with injectable configuration,
/// making the code thread-safe and testable.
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::discovery::EnvironmentDiscoveryConfig;
///
/// // Production: Load from environment
/// let config = EnvironmentDiscoveryConfig::from_env();
///
/// // Testing: Inject test values (no env var pollution!)
/// let mut config = EnvironmentDiscoveryConfig::new();
/// config.set_endpoint("orchestration", "http://test:8080");
/// ```
#[derive(Debug, Clone, Default)]
/// Configuration for `EnvironmentDiscovery`
pub struct EnvironmentDiscoveryConfig {
    /// Discovered capability endpoints
    /// Key: capability type (e.g., "orchestration")
    /// Value: endpoint URL
    endpoints: HashMap<String, String>,

    /// Additional metadata per capability
    /// Key: (`capability_type`, `metadata_key`)
    /// Value: metadata value
    metadata: HashMap<(String, String), String>,
}

impl EnvironmentDiscoveryConfig {
    /// Create a new empty configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Load configuration from environment variables
    ///
    /// This reads environment variables once at startup, avoiding
    /// repeated global state access during discovery.
    ///
    /// # Patterns Scanned
    ///
    /// - `*_DISCOVERY_ENDPOINT` - Capability endpoints
    /// - `*_AUTH_KEY` - Authentication keys
    /// - `*_TIMEOUT_MS` - Timeout values
    #[must_use]
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Standard discovery endpoint patterns
        let patterns = [
            "ORCHESTRATION_DISCOVERY_ENDPOINT",
            "SECURITY_DISCOVERY_ENDPOINT",
            "AI_DISCOVERY_ENDPOINT",
            "STORAGE_DISCOVERY_ENDPOINT",
            "MONITORING_DISCOVERY_ENDPOINT",
            "COMPUTE_DISCOVERY_ENDPOINT",
            "NETWORK_DISCOVERY_ENDPOINT",
        ];

        for pattern in &patterns {
            if let Ok(endpoint) = std::env::var(pattern) {
                let capability_type = pattern
                    .strip_suffix("_DISCOVERY_ENDPOINT")
                    .unwrap_or(pattern)
                    .to_lowercase();

                config.set_endpoint(&capability_type, &endpoint);

                // Load associated metadata
                let auth_key = format!("{}_AUTH_KEY", capability_type.to_uppercase());
                if let Ok(auth) = std::env::var(&auth_key) {
                    config.set_metadata(&capability_type, "auth_key", &auth);
                }

                let timeout_key = format!("{}_TIMEOUT_MS", capability_type.to_uppercase());
                if let Ok(timeout) = std::env::var(&timeout_key) {
                    config.set_metadata(&capability_type, "timeout_ms", &timeout);
                }
            }
        }

        config
    }

    /// Set an endpoint for a capability type
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use nestgate_core::discovery::EnvironmentDiscoveryConfig;
    /// let mut config = EnvironmentDiscoveryConfig::new();
    /// config.set_endpoint("orchestration", "http://orch:8080");
    /// ```
    pub fn set_endpoint(&mut self, capability_type: &str, endpoint: &str) {
        self.endpoints
            .insert(capability_type.to_string(), endpoint.to_string());
    }

    /// Get an endpoint for a capability type
    #[must_use]
    pub fn get_endpoint(&self, capability_type: &str) -> Option<&str> {
        self.endpoints.get(capability_type).map(String::as_str)
    }

    /// Set metadata for a capability
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// # use nestgate_core::discovery::EnvironmentDiscoveryConfig;
    /// let mut config = EnvironmentDiscoveryConfig::new();
    /// config.set_metadata("orchestration", "auth_key", "secret123");
    /// ```
    pub fn set_metadata(&mut self, capability_type: &str, key: &str, value: &str) {
        self.metadata.insert(
            (capability_type.to_string(), key.to_string()),
            value.to_string(),
        );
    }

    /// Get metadata for a capability
    #[must_use]
    pub fn get_metadata(&self, capability_type: &str, key: &str) -> Option<&str> {
        self.metadata
            .get(&(capability_type.to_string(), key.to_string()))
            .map(String::as_str)
    }

    /// Get all endpoints
    #[must_use]
    pub const fn endpoints(&self) -> &HashMap<String, String> {
        &self.endpoints
    }

    /// Get all metadata for a capability
    #[must_use]
    pub fn all_metadata(&self, capability_type: &str) -> HashMap<String, String> {
        self.metadata
            .iter()
            .filter(|((cap_type, _), _)| cap_type == capability_type)
            .map(|((_, key), value)| (key.clone(), value.clone()))
            .collect()
    }

    /// Check if configuration is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.endpoints.is_empty()
    }

    /// Get the number of configured endpoints
    #[must_use]
    pub fn len(&self) -> usize {
        self.endpoints.len()
    }

    /// Add a custom pattern to discover
    ///
    /// This allows extending discovery to custom capability types
    /// not covered by the default patterns.
    pub fn add_custom_pattern(&mut self, env_var_name: &str) {
        if let Ok(endpoint) = std::env::var(env_var_name) {
            let capability_type = env_var_name
                .strip_suffix("_DISCOVERY_ENDPOINT")
                .unwrap_or(env_var_name)
                .to_lowercase();
            self.set_endpoint(&capability_type, &endpoint);
        }
    }
}

/// Thread-safe shared configuration
///
/// Use this when you need to share configuration across threads.
/// The `Arc` provides cheap cloning and thread-safe reference counting.
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::discovery::SharedDiscoveryConfig;
/// use std::sync::Arc;
///
/// let config = SharedDiscoveryConfig::from_env();
///
/// // Cheap clone for use in multiple threads
/// let config_clone = Arc::clone(&config);
/// tokio::spawn(async move {
///     // Use config_clone in async task
/// });
/// ```
pub type SharedDiscoveryConfig = Arc<EnvironmentDiscoveryConfig>;

/// Helper to create a shared config from environment
#[must_use]
pub fn shared_config_from_env() -> SharedDiscoveryConfig {
    Arc::new(EnvironmentDiscoveryConfig::from_env())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_is_empty() {
        let config = EnvironmentDiscoveryConfig::new();
        assert!(config.is_empty());
        assert_eq!(config.len(), 0);
    }

    #[test]
    fn test_config_set_get_endpoint() {
        let mut config = EnvironmentDiscoveryConfig::new();
        config.set_endpoint("orchestration", "http://orch:8080");

        assert_eq!(
            config.get_endpoint("orchestration"),
            Some("http://orch:8080")
        );
        assert_eq!(config.get_endpoint("nonexistent"), None);
        assert_eq!(config.len(), 1);
    }

    #[test]
    fn test_config_set_get_metadata() {
        let mut config = EnvironmentDiscoveryConfig::new();
        config.set_endpoint("security", "http://sec:8081");
        config.set_metadata("security", "auth_key", "secret123");
        config.set_metadata("security", "timeout_ms", "5000");

        assert_eq!(
            config.get_metadata("security", "auth_key"),
            Some("secret123")
        );
        assert_eq!(config.get_metadata("security", "timeout_ms"), Some("5000"));
        assert_eq!(config.get_metadata("security", "nonexistent"), None);
    }

    #[test]
    fn test_config_all_metadata() {
        let mut config = EnvironmentDiscoveryConfig::new();
        config.set_metadata("ai", "auth_key", "key1");
        config.set_metadata("ai", "timeout_ms", "3000");
        config.set_metadata("ai", "retry_count", "3");

        let metadata = config.all_metadata("ai");
        assert_eq!(metadata.len(), 3);
        assert_eq!(metadata.get("auth_key"), Some(&"key1".to_string()));
        assert_eq!(metadata.get("timeout_ms"), Some(&"3000".to_string()));
        assert_eq!(metadata.get("retry_count"), Some(&"3".to_string()));
    }

    #[test]
    fn test_config_clone() {
        let mut config = EnvironmentDiscoveryConfig::new();
        config.set_endpoint("storage", "http://storage:8083");

        let cloned = config.clone();
        assert_eq!(cloned.get_endpoint("storage"), Some("http://storage:8083"));
    }

    #[test]
    fn test_config_default() {
        let config = EnvironmentDiscoveryConfig::default();
        assert!(config.is_empty());
    }

    #[test]
    fn test_shared_config() {
        let config = Arc::new(EnvironmentDiscoveryConfig::new());
        let clone1 = Arc::clone(&config);
        let _clone2 = Arc::clone(&config);

        // All point to same data
        assert!(Arc::ptr_eq(&config, &clone1));
        assert_eq!(Arc::strong_count(&config), 3);
    }

    #[test]
    fn test_config_multiple_endpoints() {
        let mut config = EnvironmentDiscoveryConfig::new();
        config.set_endpoint("orch", "http://orch:8080");
        config.set_endpoint("sec", "http://sec:8081");
        config.set_endpoint("ai", "http://ai:8082");

        assert_eq!(config.len(), 3);
        assert!(!config.is_empty());

        let endpoints = config.endpoints();
        assert_eq!(endpoints.len(), 3);
    }

    #[test]
    fn test_config_overwrite_endpoint() {
        let mut config = EnvironmentDiscoveryConfig::new();
        config.set_endpoint("test", "http://old:8080");
        config.set_endpoint("test", "http://new:8081");

        assert_eq!(config.get_endpoint("test"), Some("http://new:8081"));
        assert_eq!(config.len(), 1);
    }
}
