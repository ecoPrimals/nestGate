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

// ==================== MODULE DECLARATIONS ====================

/// Service discovery and orchestration settings
pub mod orchestration;

/// TCP/UDP and protocol-specific settings
pub mod protocols;

/// VLAN and network segmentation settings
pub mod vlan;

/// Connection management and pooling settings
pub mod connections;

/// Load balancing and routing settings
pub mod routing;

/// Quality of Service settings
pub mod qos;

/// Network security settings
pub mod security;

// ==================== RE-EXPORTS ====================

// Re-export all types for backward compatibility
pub use orchestration::*;
pub use protocols::*;
pub use vlan::*;
pub use connections::*;
pub use routing::*;
pub use qos::*;
pub use security::*;

// ==================== MAIN UNIFIED STRUCTURE ====================

use serde::{Deserialize, Serialize};

/// **UNIFIED NETWORK EXTENSIONS**
/// Consolidates all network-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for UnifiedNetworkExtensions {
    fn default() -> Self {
        Self {
            orchestration: NetworkOrchestrationSettings::default(),
            protocols: NetworkProtocolSettings::default(),
            vlan: NetworkVlanSettings::default(),
            connections: NetworkConnectionSettings::default(),
            routing: NetworkRoutingSettings::default(),
            qos: NetworkQosSettings::default(),
            security: NetworkSecuritySettings::default(),
        }
    }
}

/// Network configuration using StandardDomainConfig pattern
pub type UnifiedNetworkConfig = UnifiedNetworkExtensions;
