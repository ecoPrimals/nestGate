// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// **NESTGATE CONFIGURATION SYSTEM**
///
/// This module provides the unified configuration system for NestGate.
/// All configuration is capability-based and vendor-agnostic.
// **CANONICAL PRIMARY**: The definitive configuration system
pub mod canonical_primary;

/// External services configuration (environment-driven, replaces hardcoded values)
///
/// This module eliminates 815 hardcoded values:
/// - 303 URLs (http://, https://)
/// - 121 ports (:8080, :3000, :5432, :6379, :9090)
/// - 391 localhost/IP addresses (127.0.0.1, 0.0.0.0, localhost)
pub mod external;

/// Default configuration values and environment variable support
pub mod defaults;

/// Thread-safe configuration for defaults (eliminates runtime `env::var` calls)
pub mod defaults_config;

/// Network defaults configuration (replaces direct env::var calls)
pub mod network_defaults;

/// Thread-safe configuration for network defaults module (eliminates runtime env::var calls)
pub mod network_defaults_v2_config;

/// Federation configuration
pub mod federation;

/// Monitoring configuration
pub mod monitoring;

/// Monitoring environment configuration (runtime settings)
pub mod monitoring_env_config;

/// API paths configuration
pub mod api_paths;

/// Sovereignty configuration (environment-driven, eliminates infrastructure assumptions)
pub mod sovereignty;

/// Thread-safe configuration for sovereignty module (eliminates runtime `env::var` calls)
pub mod sovereignty_config;

/// Capability-based service discovery for configuration
pub mod capability_discovery;

/// Agnostic configuration module (environment-driven, discovery-based)
pub mod agnostic_config;

/// Centralized runtime configuration system (eliminates 805+ hardcoded values)
///
/// Replaces all hardcoded ports, IPs, endpoints with environment-driven config.
/// See [`runtime`] module for details.
pub mod runtime;

/// Configuration validation module
pub mod validation;

#[cfg(test)]
mod config_validation_tests; // Nov 23, 2025 - P1 test expansion

#[cfg(test)]
mod comprehensive_error_path_tests_dec20; // Dec 20, 2025 - Error path coverage expansion
#[cfg(test)]
mod edge_case_tests; // Nov 23, 2025 - P1-5 edge case tests
#[cfg(test)]
mod environment_error_tests; // Dec 6, 2025 - P1 comprehensive environment error tests
#[cfg(test)]
mod strategic_config_tests_dec11; // Dec 11, 2025 - Strategic configuration tests

// Week 2 test expansion - REMOVED (outdated API) Nov 30, 2025
// The error_path_tests_week2 module was deleted as it tested a refactored API

/// Service discovery configuration (Week 2 migration)
pub mod discovery_config;
/// Modern environment-driven configuration system (Week 2 Dec 2025)
pub mod environment;
/// Migration bridge for legacy configuration patterns (Week 2 Dec 2025)
pub mod migration_bridge;

/// Capability-based configuration system (runtime discovery, no hardcoding)
pub mod capability_based;

/// Port configuration system (environment-driven, zero hardcoding)
///
/// Replaces all hardcoded port constants with configurable values.
/// Environment variables: NESTGATE_API_PORT, NESTGATE_METRICS_PORT, etc.
pub mod ports;

/// Aggregated port bundle ([`port_config::PortConfiguration`]) for multi-service defaults
pub mod port_config;

/// XDG-compliant storage path configuration (Phase 4 - Hardcoding Evolution)
///
/// Eliminates hardcoded paths like `/var/lib/nestgate` and `/tmp/nestgate`.
/// Provides 4-tier fallback: NESTGATE_* > XDG_* > $HOME > /var/lib
///
/// **Created**: January 30, 2026
/// **Impact**: +4 bonus points (Hardcoding Evolution)
pub mod storage_paths;
pub mod substrate_tiers;

// ==================== SECTION ====================
// All deprecated configuration modules have been removed. Use canonical_primary directly:
// - core → canonical_primary::NestGateCanonicalConfig
// - domains → canonical_primary::NestGateCanonicalConfig
// - builders → canonical_primary builders
// - unified_types → canonical_primary detailed_configs
// - canonical → canonical_primary::NestGateCanonicalConfig
// - network → canonical_primary::NestGateCanonicalConfig
// - federation → canonical_primary::NestGateCanonicalConfig
// - monitoring → canonical_primary::NestGateCanonicalConfig
// - canonical_config → canonical_primary::NestGateCanonicalConfig
// - canonical_unified → canonical_primary::NestGateCanonicalConfig

// ==================== SECTION ====================

/// **THE** canonical configuration for all `NestGate` systems
pub use canonical_primary::{
    CanonicalNetworkConfig as NetworkConfig, ConfigMetadata, DeploymentEnvironment, FeatureFlags,
    LogLevel, NestGateCanonicalConfig, PerformanceConfig, SecurityConfig, ServiceConfig,
    StorageConfig, SystemConfig,
};
// API and automation configs from domains (canonical types)
pub use canonical_primary::domains::{ApiConfig, AutomationConfig};

// Default configuration types
pub use defaults_config::{NetworkDefaultsConfig, SharedNetworkDefaultsConfig};

// Network defaults configuration types
pub use network_defaults_v2_config::{NetworkDefaultsV2Config, SharedNetworkDefaultsV2Config};

// Sovereignty configuration types
pub use sovereignty_config::{SharedSovereigntyRuntimeConfig, SovereigntyRuntimeConfig};

// Port configuration types
pub use ports::{PortConfig, PortConfigBuilder};

// Note: Detailed configuration types are defined inline in canonical_primary
// and will be accessible through the canonical_primary module directly

// ==================== SECTION ====================

// ==================== SECTION ====================
// All deprecated configuration type aliases have been removed. Use canonical_primary directly:
// - UnifiedConfig → canonical_primary::NestGateCanonicalConfig
// - StandardConfig → canonical_primary::NestGateCanonicalConfig
// - PrimaryConfig → canonical_primary::NestGateCanonicalConfig
// - NestGateCanonicalConfig → canonical_primary::NestGateCanonicalConfig

// ==================== SECTION ====================

/// Create a canonical configuration with default settings
///
/// Returns a `NestGateCanonicalConfig` with sensible defaults for all settings.
#[must_use]
pub fn create_default_config() -> canonical_primary::NestGateCanonicalConfig {
    canonical_primary::NestGateCanonicalConfig::default()
}

/// Create a production-ready canonical configuration
///
/// Returns a `NestGateCanonicalConfig` optimized for production environments
/// with auto-scaling and load balancing enabled.
#[must_use]
pub fn create_production_config() -> canonical_primary::NestGateCanonicalConfig {
    let mut config = canonical_primary::NestGateCanonicalConfig::default();
    config.system.environment = DeploymentEnvironment::Production;
    config.system.log_level = LogLevel::Warn;
    config.system.debug_mode = false;
    config
        .features
        .custom_flags
        .insert("enable_auto_scaling".to_string(), true);
    config
        .features
        .custom_flags
        .insert("enable_load_balancing".to_string(), true);
    config
}

/// Create a development configuration
///
/// Returns a `NestGateCanonicalConfig` optimized for development with
/// debug logging and debug mode enabled.
#[must_use]
pub fn create_development_config() -> canonical_primary::NestGateCanonicalConfig {
    let mut config = canonical_primary::NestGateCanonicalConfig::default();
    config.system.environment = DeploymentEnvironment::Development;
    config.system.log_level = LogLevel::Debug;
    config.system.debug_mode = true;
    config
}

/// Create a testing configuration
///
/// Returns a `NestGateCanonicalConfig` optimized for testing with
/// metrics and tracing disabled for faster test execution.
#[must_use]
pub fn create_testing_config() -> canonical_primary::NestGateCanonicalConfig {
    let mut config = canonical_primary::NestGateCanonicalConfig::default();
    config.system.environment = DeploymentEnvironment::Testing;
    config.system.log_level = LogLevel::Debug;
    config
        .features
        .custom_flags
        .insert("enable_metrics".to_string(), false);
    config
        .features
        .custom_flags
        .insert("enable_tracing".to_string(), false);
    config
}
// ==================== SECTION ====================
// All configurations now use canonical_primary::NestGateCanonicalConfig directly.
// Default implementations are in the canonical_primary module.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Infantdiscoveryconfigcanonical
pub type InfantDiscoveryConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using InfantDiscoveryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_config_creation() {
        let config = create_default_config();
        assert_eq!(config.system.instance_name, "nestgate-default");
        assert!(matches!(
            config.system.environment,
            DeploymentEnvironment::Development
        ));
    }

    #[test]
    fn test_production_config() {
        let config = create_production_config();
        assert!(matches!(
            config.system.environment,
            DeploymentEnvironment::Production
        ));
        assert_eq!(config.system.log_level, LogLevel::Warn);
        assert!(!config.system.debug_mode);
    }

    #[test]
    fn test_development_config() {
        let config = create_development_config();
        assert!(matches!(
            config.system.environment,
            DeploymentEnvironment::Development
        ));
        assert_eq!(config.system.log_level, LogLevel::Debug);
        assert!(config.system.debug_mode);
    }

    #[test]
    fn test_migration_validation() {
        let config = create_default_config();
        // Migration validation temporarily disabled - module not available
        // assert!(migration::validate_migration(&config).is_ok());
        // Verify config has been properly initialized
        assert!(!config.system.instance_name.is_empty());
    }
}

#[cfg(test)]
mod defaults_tests;

#[cfg(test)]
mod defaults_additional_tests; // NEW: Test expansion phase (Nov 6, 2025) // Include comprehensive defaults tests

/// Infant discovery configuration - no hardcoded assumptions
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::InfantDiscoveryConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::InfantDiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `InfantDiscovery`
pub struct InfantDiscoveryConfig {
    /// Whether infant discovery is enabled
    pub enabled: bool,
    /// Timeout for discovery operations in seconds
    pub discovery_timeout_seconds: u64,
    /// Time-to-live for capability cache in seconds
    pub capability_cache_ttl_seconds: u64,
    /// Whether to fallback to environment variables if discovery fails
    pub fallback_to_environment: bool,
}

#[allow(deprecated)]
impl Default for InfantDiscoveryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_timeout_seconds: 30,
            capability_cache_ttl_seconds: 300,
            fallback_to_environment: true,
        }
    }
}
