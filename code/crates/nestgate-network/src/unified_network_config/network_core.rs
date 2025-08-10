/// **UNIFIED NETWORK CONFIG - CORE MODULE**
/// Contains the main UnifiedNetworkConfig struct and core network configuration logic.
/// Extracted from the large unified_network_config.rs to achieve file size compliance.

use serde::{Deserialize, Serialize};
use nestgate_core::unified_config_consolidation::StandardDomainConfig;
use super::network_settings::*;

// ==================== NETWORK-SPECIFIC EXTENSIONS ====================

/// Network-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkExtensions {
    /// VLAN configuration
    pub vlan: NetworkVlanSettings,
    /// Protocol management settings
    pub protocols: NetworkProtocolSettings,
    /// Connection management
    pub connections: NetworkConnectionSettings,
    /// Port allocation management
    pub ports: NetworkPortSettings,
    /// API endpoint configuration
    pub api_endpoints: NetworkApiSettings,
    /// Network file system settings (NFS, SMB)
    pub file_systems: NetworkFileSystemSettings,
    /// Load balancing settings
    pub load_balancing: NetworkLoadBalancingSettings,
    /// Quality of Service settings
    pub qos: NetworkQosSettings,
    }

/// **UNIFIED NETWORK CONFIGURATION**
/// The single source of truth for all network configuration across the system
pub type UnifiedNetworkConfig = StandardDomainConfig<NetworkExtensions>;

impl Default for NetworkExtensions {
    fn default() -> Self {
        Self {
            vlan: NetworkVlanSettings::default(),
            protocols: NetworkProtocolSettings::default(),
            connections: NetworkConnectionSettings::default(),
            ports: NetworkPortSettings::default(),
            api_endpoints: NetworkApiSettings::default(),
            file_systems: NetworkFileSystemSettings::default(),
            load_balancing: NetworkLoadBalancingSettings::default(),
            qos: NetworkQosSettings::default(),
    }
    }
    }

impl UnifiedNetworkConfig {
    /// Create development configuration optimized for local development
    pub fn development() -> Self {
        Self::create_for_environment("development")
    }

    /// Create production configuration optimized for high-load production
    pub fn production() -> Self {
        Self::create_for_environment("production")
    }

    /// Create high-performance configuration for maximum throughput
    pub fn high_performance() -> Self {
        Self::create_for_workload("high-performance")
    }

    /// Create testing configuration optimized for integration tests
    pub fn testing() -> Self {
        Self::create_for_environment("testing")
    }
} 