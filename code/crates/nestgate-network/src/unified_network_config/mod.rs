/// **UNIFIED NETWORK CONFIG MODULE SYSTEM**
/// Breaking down the large unified_network_config.rs (865 lines) into focused modules
/// to achieve 2000-line file size compliance while maintaining functionality.

// Core network configuration and main types
pub mod network_core;
// VLAN and protocol configuration settings
pub mod network_settings;
// Migration functions and legacy compatibility
pub mod network_migrations;

// Re-export all public types for backward compatibility
pub use network_core::{UnifiedNetworkConfig, NetworkExtensions};
pub use network_settings::{
    NetworkVlanSettings, NetworkProtocolSettings, NetworkConnectionSettings,
    NetworkPortSettings, NetworkApiSettings, NetworkFileSystemSettings,
    NetworkLoadBalancingSettings, NetworkQosSettings
}; 