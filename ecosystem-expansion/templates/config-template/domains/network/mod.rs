//! **CANONICAL NETWORK CONFIGURATION - MODULARIZED**
//!
//! This module consolidates ALL network configuration variants across the NestGate ecosystem
//! into focused, modular structures for maintainability.
//!
//! **CONSOLIDATES**:
//! - nestgate-network/src/types.rs → NetworkConfig
//! - nestgate-canonical/src/types.rs → NetworkConfig  
//! - nestgate-core/src/network/native_async/config.rs → NetworkConfig
//! - nestgate-core/src/canonical_modernization/unified_types.rs → UnifiedNetworkConfig
//! - 15+ other NetworkConfig variants

// ==================== NETWORK CONFIGURATION MODULES ====================

/// Core API server configuration
pub mod api;

/// Network orchestration configuration
pub mod orchestration;

/// Protocol-specific configurations
pub mod protocols;

/// VLAN and network segmentation
pub mod vlan;

/// Service discovery configuration
pub mod discovery;

/// Performance and optimization settings
pub mod performance;

/// Security and authentication settings
pub mod security;

/// Monitoring and observability
pub mod monitoring;

/// Environment-specific overrides
pub mod environment;

// ==================== RE-EXPORTS ====================

pub use api::NetworkApiConfig;
pub use orchestration::NetworkOrchestrationConfig;
pub use protocols::NetworkProtocolConfig;
pub use vlan::NetworkVlanConfig;
pub use discovery::NetworkDiscoveryConfig;
pub use performance::NetworkPerformanceConfig;
pub use security::NetworkSecurityConfig;
pub use monitoring::NetworkMonitoringConfig;
pub use environment::NetworkEnvironmentConfig;

// ==================== MAIN CONFIGURATION STRUCTURE ====================

use serde::{Deserialize, Serialize};
use crate::Result;

/// **THE** canonical network configuration for the entire NestGate ecosystem
/// This replaces ALL other NetworkConfig variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalNetworkConfig {
    /// Core API server configuration
    pub api: NetworkApiConfig,
    
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
    pub fn development_optimized() -> Self {
        Self {
            api: NetworkApiConfig::development_optimized(),
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
    pub fn production_hardened() -> Self {
        Self {
            api: NetworkApiConfig::production_hardened(),
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
    fn default() -> Self {
        Self::development_optimized()
    }
} 