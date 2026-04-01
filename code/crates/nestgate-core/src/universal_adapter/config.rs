// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Universal Adapter Configuration
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Universal adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for UniversalAdapter
pub struct UniversalAdapterConfig {
    /// Enable auto-discovery of primal providers
    pub auto_discovery: bool,
    /// Discovery interval in seconds
    pub discovery_interval: u64,
    /// Timeout for primal requests in seconds
    pub request_timeout: u64,
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Fallback behavior when no primal is available
    pub fallback_behavior: FallbackBehavior,
    /// Discovery methods to use
    pub discovery_methods: Vec<DiscoveryMethod>,
}
impl Default for UniversalAdapterConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            auto_discovery: true,
            discovery_interval: 30,
            request_timeout: 30,
            max_retries: 3,
            fallback_behavior: FallbackBehavior::NoOp,
            discovery_methods: vec![
                DiscoveryMethod::Environment,
                DiscoveryMethod::ServiceRegistry,
                DiscoveryMethod::NetworkScan,
            ],
        }
    }
}

/// Fallback behavior when no primal is available
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Fallbackbehavior
pub enum FallbackBehavior {
    /// Return an error
    Error,
    /// Return a no-op result
    NoOp,
    /// Use a local implementation
    Local,
}
/// Discovery methods for finding primal providers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Discoverymethod
pub enum DiscoveryMethod {
    /// Environment variables
    Environment,
    /// Service registry lookup
    ServiceRegistry,
    /// Network scanning
    NetworkScan,
    /// Configuration file
    Configuration,
}

/// Configuration for the universal adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AdapterConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AdapterConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Adapter
pub struct AdapterConfig {
    /// Discovery Timeout
    pub discovery_timeout: Duration,
    /// Retry Attempts
    pub retry_attempts: u32,
    /// Cache Ttl
    pub cache_ttl: Duration,
    /// Endpoints
    pub endpoints: Vec<String>,
    /// Fallback Enabled
    pub fallback_enabled: bool,
}

#[expect(deprecated, reason = "migration in progress")]
impl AdapterConfig {
    /// Create a new adapter configuration
    ///
    /// ✅ MIGRATED: Now uses `ServiceDiscoveryConfig` instead of hardcoded endpoints
    #[must_use]
    pub fn new() -> Self {
        use crate::config::discovery_config::ServiceDiscoveryConfig;
        use crate::constants::hardcoding::limits;

        // Use ServiceDiscoveryConfig for discovery endpoints (Week 2 migration)
        let service_discovery = ServiceDiscoveryConfig::default();
        let endpoints = service_discovery.get_endpoints().to_vec();

        Self {
            discovery_timeout: Duration::from_secs(limits::TIMEOUT_SECS),
            retry_attempts: limits::MAX_RETRIES,
            cache_ttl: Duration::from_secs(300), // 5 minutes
            endpoints,
            fallback_enabled: true,
        }
    }

    /// Set discovery timeout
    #[must_use]
    pub const fn with_discovery_timeout(mut self, timeout: Duration) -> Self {
        self.discovery_timeout = timeout;
        self
    }

    /// Set retry attempts
    #[must_use]
    pub const fn with_retry_attempts(mut self, attempts: u32) -> Self {
        self.retry_attempts = attempts;
        self
    }

    /// Add discovery endpoint
    #[must_use]
    pub fn add_endpoint(mut self, endpoint: String) -> Self {
        self.endpoints.push(endpoint);
        self
    }

    /// Enable or disable fallback providers
    #[must_use]
    pub const fn with_fallback(mut self, enabled: bool) -> Self {
        self.fallback_enabled = enabled;
        self
    }
}

#[expect(deprecated, reason = "migration in progress")]
impl Default for AdapterConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Adapterconfigcanonical
pub type AdapterConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AdapterConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_adapter_config_default() {
        let config = UniversalAdapterConfig::default();

        assert!(config.auto_discovery);
        assert_eq!(config.discovery_interval, 30);
        assert_eq!(config.request_timeout, 30);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.fallback_behavior, FallbackBehavior::NoOp);
        assert_eq!(config.discovery_methods.len(), 3);
        assert!(
            config
                .discovery_methods
                .contains(&DiscoveryMethod::Environment)
        );
        assert!(
            config
                .discovery_methods
                .contains(&DiscoveryMethod::ServiceRegistry)
        );
        assert!(
            config
                .discovery_methods
                .contains(&DiscoveryMethod::NetworkScan)
        );
    }

    #[test]
    fn test_fallback_behavior_variants() {
        let error = FallbackBehavior::Error;
        let noop = FallbackBehavior::NoOp;
        let local = FallbackBehavior::Local;

        assert_ne!(error, noop);
        assert_ne!(noop, local);
        assert_ne!(error, local);

        // Test cloning
        let cloned = noop;
        assert_eq!(cloned, FallbackBehavior::NoOp);
    }

    #[test]
    fn test_discovery_method_variants() {
        let env = DiscoveryMethod::Environment;
        let registry = DiscoveryMethod::ServiceRegistry;
        let scan = DiscoveryMethod::NetworkScan;
        let config = DiscoveryMethod::Configuration;

        assert_ne!(env, registry);
        assert_ne!(registry, scan);
        assert_ne!(scan, config);

        // Test cloning
        let cloned = env;
        assert_eq!(cloned, DiscoveryMethod::Environment);
    }

    #[test]
    fn test_adapter_config_default() {
        let config = AdapterConfig::default();

        assert_eq!(config.discovery_timeout, Duration::from_secs(30));
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.cache_ttl, Duration::from_secs(300));
        // ✅ MIGRATED: ServiceDiscoveryConfig generates 3 endpoints by default
        assert_eq!(config.endpoints.len(), 3);
        assert!(config.fallback_enabled);
    }

    #[test]
    fn test_adapter_config_new() {
        let config = AdapterConfig::new();

        assert_eq!(config.discovery_timeout, Duration::from_secs(30));
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.cache_ttl, Duration::from_secs(300));
        // ✅ MIGRATED: Now uses ServiceDiscoveryConfig, which generates endpoints
        // Default: 3 endpoints starting from 127.0.0.1:8080
        assert_eq!(config.endpoints.len(), 3);
        assert!(
            config
                .endpoints
                .iter()
                .any(|e| e.starts_with("http://127.0.0.1:"))
        );
    }

    #[test]
    fn test_adapter_config_builder_with_discovery_timeout() {
        let timeout = Duration::from_secs(60);
        let config = AdapterConfig::new().with_discovery_timeout(timeout);

        assert_eq!(config.discovery_timeout, timeout);
        assert_eq!(config.retry_attempts, 3); // Other fields unchanged
    }

    #[test]
    fn test_adapter_config_builder_with_retry_attempts() {
        let config = AdapterConfig::new().with_retry_attempts(5);

        assert_eq!(config.retry_attempts, 5);
        assert_eq!(config.discovery_timeout, Duration::from_secs(30)); // Other fields unchanged
    }

    #[test]
    fn test_adapter_config_builder_add_endpoint() {
        let config = AdapterConfig::new().add_endpoint("http://custom:9000/discovery".to_string());

        // ✅ MIGRATED: ServiceDiscoveryConfig generates 3 endpoints + 1 custom = 4
        assert_eq!(config.endpoints.len(), 4);
        assert!(
            config
                .endpoints
                .contains(&"http://custom:9000/discovery".to_string())
        );
    }

    #[test]
    fn test_adapter_config_builder_with_fallback() {
        let config_enabled = AdapterConfig::new().with_fallback(true);
        assert!(config_enabled.fallback_enabled);

        let config_disabled = AdapterConfig::new().with_fallback(false);
        assert!(!config_disabled.fallback_enabled);
    }

    #[test]
    fn test_adapter_config_builder_chaining() {
        let config = AdapterConfig::new()
            .with_discovery_timeout(Duration::from_secs(45))
            .with_retry_attempts(10)
            .add_endpoint("http://endpoint1:8080/discovery".to_string())
            .add_endpoint("http://endpoint2:8080/discovery".to_string())
            .with_fallback(false);

        assert_eq!(config.discovery_timeout, Duration::from_secs(45));
        assert_eq!(config.retry_attempts, 10);
        // ✅ MIGRATED: ServiceDiscoveryConfig generates 3 default + 2 added = 5
        assert_eq!(config.endpoints.len(), 5);
        assert!(!config.fallback_enabled);
    }

    #[test]
    fn test_universal_adapter_config_clone() {
        let config = UniversalAdapterConfig::default();
        let cloned = config.clone();

        assert_eq!(config.auto_discovery, cloned.auto_discovery);
        assert_eq!(config.discovery_interval, cloned.discovery_interval);
        assert_eq!(config.request_timeout, cloned.request_timeout);
        assert_eq!(config.max_retries, cloned.max_retries);
        assert_eq!(config.fallback_behavior, cloned.fallback_behavior);
    }

    #[test]
    fn test_adapter_config_clone() {
        let config = AdapterConfig::new();
        let cloned = config.clone();

        assert_eq!(config.retry_attempts, cloned.retry_attempts);
        assert_eq!(config.discovery_timeout, cloned.discovery_timeout);
        assert_eq!(config.cache_ttl, cloned.cache_ttl);
        assert_eq!(config.fallback_enabled, cloned.fallback_enabled);
        assert_eq!(config.endpoints.len(), cloned.endpoints.len());
    }

    #[test]
    fn test_edge_cases_zero_values() {
        let config = AdapterConfig::new()
            .with_retry_attempts(0)
            .with_discovery_timeout(Duration::from_secs(0));

        assert_eq!(config.retry_attempts, 0);
        assert_eq!(config.discovery_timeout, Duration::from_secs(0));
    }

    #[test]
    fn test_edge_cases_large_values() {
        let config = AdapterConfig::new()
            .with_retry_attempts(u32::MAX)
            .with_discovery_timeout(Duration::from_secs(86400)); // 24 hours

        assert_eq!(config.retry_attempts, u32::MAX);
        assert_eq!(config.discovery_timeout, Duration::from_secs(86400));
    }

    #[test]
    fn test_multiple_endpoint_additions() {
        let mut config = AdapterConfig::new();

        for i in 0..10 {
            config = config.add_endpoint(format!("http://endpoint{i}:8080/discovery"));
        }

        // ✅ MIGRATED: ServiceDiscoveryConfig generates 3 default + 10 added = 13
        assert_eq!(config.endpoints.len(), 13);
    }
}
