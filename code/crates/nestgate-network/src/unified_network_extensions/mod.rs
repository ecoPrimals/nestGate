//
// This module provides modularized network configuration extensions,
// split from the original 937-line monolithic file for better maintainability.
//
// **MODULAR ORGANIZATION**:
// - `orchestration.rs` - Service discovery and orchestration settings
// - `protocols.rs` - TCP/UDP and protocol-specific settings
// - `vlan.rs` - VLAN and network segmentation settings
// - `connections.rs` - Connection management and pooling settings
// - `routing.rs` - Load balancing and routing settings
// - `qos.rs` - Quality of Service settings
// - `security.rs` - Network security settings

// ==================== SECTION ====================

//! Service discovery and orchestration settings
/// Connection management and pooling settings
pub mod connections;
pub mod orchestration;
/// TCP/UDP and protocol-specific settings
pub mod protocols;
/// Quality of Service settings
pub mod qos;
/// Load balancing and routing settings
pub mod routing;
/// Network security settings
pub mod security;
/// VLAN and network segmentation settings
pub mod vlan;
// ==================== SECTION ====================

// Re-export all types for backward compatibility
pub use connections::*;
pub use orchestration::*;
pub use protocols::*;
pub use qos::*;
pub use routing::*;
pub use security::*;
pub use vlan::*;

// ==================== SECTION ====================

use serde::{Deserialize, Serialize};

// **UNIFIED NETWORK EXTENSIONS**
/// Consolidates all network-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedNetworkExtensions {
    /// Orchestration and service discovery settings
    pub orchestration: NetworkOrchestrationSettings,
    /// Protocol configuration settings
    pub protocols: NetworkProtocolSettings,
    /// VLAN and network segmentation settings
    pub vlan: NetworkVlanSettings,
    /// Connection management settings
    pub connections: NetworkConnectionSettings,
    /// Load balancing and routing settings
    pub routing: NetworkRoutingSettings,
    /// Quality of Service settings
    pub qos: NetworkQosSettings,
    /// Network security settings
    pub security: NetworkSecuritySettings,
}

/// Network configuration using `StandardDomainConfig` pattern
pub type UnifiedNetworkConfig = UnifiedNetworkExtensions;

#[cfg(test)]
mod orchestration_tests;
