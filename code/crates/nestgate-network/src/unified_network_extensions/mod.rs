//! **ADVANCED NETWORK FEATURES**
//!
//! Optional advanced networking capabilities for specialized deployments.
//! Extends `unified_network_config` with orchestration and service discovery.
//!
//! ## Purpose
//!
//! Provides advanced networking features beyond core functionality:
//! - Service discovery and orchestration
//! - Advanced routing and load balancing
//! - Complex QoS policies
//! - Network segmentation (VLANs)
//! - Enhanced security policies
//!
//! ## When to Use
//!
//! Use this module when you need:
//! - ✅ Service discovery and orchestration
//! - ✅ Complex routing policies
//! - ✅ Advanced QoS and traffic shaping
//! - ✅ Multi-service coordination
//! - ✅ Distributed system networking
//!
//! For basic networking, use `unified_network_config` instead.
//!
//! ## Distinction from Core Config
//!
//! | Feature | Core Config | Extensions |
//! |---------|-------------|------------|
//! | **Purpose** | Essential networking | Advanced features |
//! | **Use Case** | Basic connectivity | Orchestration |
//! | **Complexity** | Simple | Advanced |
//! | **Required** | Yes | Optional |
//!
//! ## Module Organization
//!
//! - `orchestration` - Service discovery and orchestration settings
//! - `protocols` - TCP/UDP and protocol-specific settings
//! - `vlan` - VLAN and network segmentation settings
//! - `connections` - Connection management and pooling settings
//! - `routing` - Load balancing and routing settings
//! - `qos` - Quality of Service settings
//! - `security` - Network security settings
//!
//! ## Example
//!
//! ```rust,ignore
//! use nestgate_network::unified_network_extensions::{
//!     UnifiedNetworkExtensions,
//!     NetworkOrchestrationSettings,
//! };
//!
//! let extensions = UnifiedNetworkExtensions::default();
//! // Enable service discovery
//! extensions.orchestration.service_discovery_enabled = true;
//! ```
//!
//! ## See Also
//!
//! - `unified_network_config` - Core networking (use this first)
//! - Architecture docs: `UNIFIED_NETWORK_STRUCTURE_EVALUATION_NOV_8_2025.md`

/// Connection management and pooling settings
pub mod connections;
/// Service discovery and orchestration settings
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
/// Unifiednetworkextensions
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
