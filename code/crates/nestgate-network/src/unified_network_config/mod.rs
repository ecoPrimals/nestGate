//! Breaking down the large unified_network_config.rs (865 lines) into focused modules
//! to achieve 2000-line file size compliance while maintaining functionality.
// Core network configuration and main types
pub mod network_core;
// VLAN and protocol configuration settings
pub mod network_settings;
// REMOVED: network_migrations - deprecated module eliminated as part of canonical modernization
// Re-export all public types for backward compatibility
pub use network_core::{NetworkExtensions, UnifiedNetworkConfig};
pub use network_settings::{
    NetworkApiSettings, NetworkConnectionSettings, NetworkFileSystemSettings,
    NetworkLoadBalancingSettings, NetworkPortSettings, NetworkProtocolSettings, NetworkQosSettings,
    NetworkVlanSettings,
};
