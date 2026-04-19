// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **CANONICAL NETWORK CONFIGURATION - MODULARIZED**
//! Module definitions and exports.
// This module consolidates ALL network configuration variants across the NestGate ecosystem
//! into focused, modular structures for maintainability.
//! Module definitions and exports.
// **CONSOLIDATES**:
//! - nestgate-network/src/types.rs → `NetworkConfig`
//! - nestgate-canonical/src/types.rs → `NetworkConfig`  
//! - nestgate-core/src/network/native_async/config.rs → `NetworkConfig`
//! - nestgate-core/src/canonical_modernization/unified_types.rs → `UnifiedNetworkConfig`
//! - 15+ other `NetworkConfig` variants

// ==================== NETWORK CONFIGURATION MODULES ====================

/// Core API server configuration
pub mod api;

#[cfg(test)]
mod api_tests;

/// Service discovery configuration
pub mod discovery;
/// Environment-specific overrides
pub mod environment;
/// Monitoring and observability
pub mod monitoring;
/// Network orchestration configuration
pub mod orchestration;
/// Performance and optimization settings
pub mod performance;
/// Protocol-specific configurations
pub mod protocols;
/// Security and authentication settings
pub mod security;
/// VLAN and network segmentation
pub mod vlan;
// ==================== RE-EXPORTS ====================

// **CANONICAL API CONFIGURATION EXPORTS**
pub use api::{
    ApiAlertConfig, ApiConfig, ApiMonitoringConfig, ApiPerformanceConfig, ApiSecurityConfig,
    RateLimitingConfig, TlsConfig,
};

// Note: NetworkApiConfig deprecated and removed - use ApiConfig directly

// Other network domain exports
pub use discovery::NetworkDiscoveryConfig;
pub use environment::NetworkEnvironmentConfig;
pub use monitoring::NetworkMonitoringConfig;
pub use orchestration::NetworkOrchestrationConfig;
pub use performance::NetworkPerformanceConfig;
pub use protocols::NetworkProtocolConfig;
pub use security::NetworkSecurityConfig;
pub use vlan::NetworkVlanConfig;

// ==================== MAIN CONFIGURATION STRUCTURE ====================

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};

// **THE** canonical network configuration for the entire NestGate ecosystem
// This replaces ALL other NetworkConfig variants
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CanonicalNetwork`
pub struct CanonicalNetworkConfig {
    /// Core API server configuration
    pub api: ApiConfig,

    /// Network orchestration configuration
    pub orchestration: NetworkOrchestrationConfig,

    /// Protocol-specific configurations
    pub protocols: NetworkProtocolConfig,

    /// VLAN and network segmentation
    pub vlan: NetworkVlanConfig,

    /// Service discovery configuration
    pub discovery: NetworkDiscoveryConfig,

    /// Performance and optimization settings
    pub performance: NetworkPerformanceConfig,

    /// Security and authentication settings
    pub security: NetworkSecurityConfig,

    /// Monitoring and observability
    pub monitoring: NetworkMonitoringConfig,

    /// Environment-specific overrides
    pub environment: NetworkEnvironmentConfig,
}
impl CanonicalNetworkConfig {
    /// Create a development-optimized configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            api: ApiConfig::development_optimized(),
            orchestration: NetworkOrchestrationConfig::development_optimized(),
            protocols: NetworkProtocolConfig::development_optimized(),
            vlan: NetworkVlanConfig::development_optimized(),
            discovery: NetworkDiscoveryConfig::development_optimized(),
            performance: NetworkPerformanceConfig::development_optimized(),
            security: NetworkSecurityConfig::development_optimized(),
            monitoring: NetworkMonitoringConfig::development_optimized(),
            environment: NetworkEnvironmentConfig::development_optimized(),
        }
    }

    /// Create a production-hardened configuration
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            api: ApiConfig::production_hardened(),
            orchestration: NetworkOrchestrationConfig::production_hardened(),
            protocols: NetworkProtocolConfig::production_hardened(),
            vlan: NetworkVlanConfig::production_hardened(),
            discovery: NetworkDiscoveryConfig::production_hardened(),
            performance: NetworkPerformanceConfig::production_hardened(),
            security: NetworkSecurityConfig::production_hardened(),
            monitoring: NetworkMonitoringConfig::production_hardened(),
            environment: NetworkEnvironmentConfig::production_hardened(),
        }
    }

    /// Validate the configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<()> {
        self.api.validate()?;
        self.orchestration.validate()?;
        self.protocols.validate()?;
        self.vlan.validate()?;
        self.discovery.validate()?;
        self.performance.validate()?;
        self.security.validate()?;
        self.monitoring.validate()?;
        self.environment.validate()?;
        Ok(())
    }

    /// Merge with another configuration
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.api = self.api.merge(other.api);
        self.orchestration = self.orchestration.merge(other.orchestration);
        self.protocols = self.protocols.merge(other.protocols);
        self.vlan = self.vlan.merge(other.vlan);
        self.discovery = self.discovery.merge(other.discovery);
        self.performance = self.performance.merge(other.performance);
        self.security = self.security.merge(other.security);
        self.monitoring = self.monitoring.merge(other.monitoring);
        self.environment = self.environment.merge(other.environment);
        self
    }
}

impl Default for CanonicalNetworkConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development_optimized()
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for existing `NetworkConfig` usage
/// **MIGRATION PATH**: All `NetworkConfig` variants should migrate to `CanonicalNetworkConfig`
pub type NetworkConfig = CanonicalNetworkConfig;

/// Backward compatibility alias for `UnifiedNetworkConfig`
pub type UnifiedNetworkConfig = CanonicalNetworkConfig;

/// Backward compatibility alias for `MinimalNetworkConfig`
pub type MinimalNetworkConfig = CanonicalNetworkConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_matches_development_optimized() {
        let a = CanonicalNetworkConfig::default();
        let b = CanonicalNetworkConfig::development_optimized();
        assert_eq!(a.api.port, b.api.port);
        assert_eq!(a.api.enabled, b.api.enabled);
    }

    #[test]
    fn validate_succeeds_for_default() {
        let net = CanonicalNetworkConfig::default();
        assert!(net.validate().is_ok());
    }

    #[test]
    fn production_hardened_has_https_api_port() {
        let net = CanonicalNetworkConfig::production_hardened();
        assert_eq!(
            net.api.port,
            crate::constants::hardcoding::runtime_fallback_ports::HTTPS
        );
    }

    #[test]
    fn merge_overwrites_from_other() {
        let base = CanonicalNetworkConfig::default();
        let other = CanonicalNetworkConfig::production_hardened();
        let merged = base.merge(other.clone());
        assert_eq!(merged.api.port, other.api.port);
        assert_eq!(merged.api.bind_address, other.api.bind_address);
    }

    #[test]
    fn development_and_production_constructors_differ() {
        let dev = CanonicalNetworkConfig::development_optimized();
        let prod = CanonicalNetworkConfig::production_hardened();
        assert_ne!(dev.api.port, prod.api.port);
    }
}
