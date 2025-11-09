use super::network_settings::{
    NetworkApiSettings, NetworkConnectionSettings, NetworkFileSystemSettings,
    NetworkLoadBalancingSettings, NetworkPortSettings, NetworkProtocolSettings, NetworkQosSettings,
    NetworkVlanSettings,
};
// CANONICAL MODERNIZATION: Use canonical unified types instead
use crate::types::NetworkConfig;
/// Contains the main `UnifiedNetworkConfig` struct and core network configuration logic.
/// Extracted from the large `unified_network_config.rs` to achieve file size compliance.
use serde::{Deserialize, Serialize};
use std::time::Duration;
// ==================== SECTION ====================

/// Network-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
/// CANONICAL MODERNIZATION: Simplified type alias without type parameters
pub type UnifiedNetworkConfig = crate::types::NetworkConfig;

pub trait NetworkConfigExt {
    fn development() -> Self;
    fn production() -> Self;
    fn high_performance() -> Self;
    fn testing() -> Self;
}

impl NetworkConfigExt for NetworkConfig {
    /// Create development configuration optimized for local development
    fn development() -> Self {
        Self::default()
    }

    /// Create production configuration optimized for high-load production
    fn production() -> Self {
        let mut config = Self::default();
        config.api.max_connections = 10000;
        config.api.connection_timeout = Duration::from_secs(60);
        config
    }

    /// Create high-performance configuration for maximum throughput
    fn high_performance() -> Self {
        let mut config = Self::default();
        config.api.max_connections = 50000;
        config.api.connection_timeout = Duration::from_secs(120);
        config
    }

    /// Create testing configuration optimized for integration tests
    fn testing() -> Self {
        let mut config = Self::default();
        config.api.max_connections = 100;
        config.api.connection_timeout = Duration::from_secs(10);
        config
    }
}
