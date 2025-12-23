//! **CORE NETWORK CONFIGURATION**
//!
//! Essential network settings required for basic NestGate networking.
//! This module provides the foundation for network connectivity.
//!
//! ## Purpose
//!
//! Consolidates core networking configuration that every deployment needs:
//! - Basic network connectivity settings
//! - VLAN and protocol configuration
//! - Connection management
//! - Port allocation
//! - Quality of Service (QoS)
//!
//! ## When to Use
//!
//! Use this module for:
//! - ✅ Basic network connectivity
//! - ✅ Standard API endpoints
//! - ✅ Simple load balancing
//! - ✅ Essential network features
//!
//! For advanced features (orchestration, service discovery), see `unified_network_extensions`.
//!
//! ## Module Organization
//!
//! - `network_core` - Main `UnifiedNetworkConfig` type and core logic
//! - `network_settings` - Supporting configuration types
//!
//! ## Example
//!
//! ```rust,ignore
//! use nestgate_network::unified_network_config::UnifiedNetworkConfig;
//!
//! let config = UnifiedNetworkConfig::default();
//! // Use for basic networking
//! ```
//!
//! ## See Also
//!
//! - `unified_network_extensions` - Advanced orchestration features
//! - Architecture docs: `UNIFIED_NETWORK_STRUCTURE_EVALUATION_NOV_8_2025.md`

// Core network configuration and main types
pub mod network_core;
// VLAN and protocol configuration settings
pub mod network_settings;

// Re-export all public types for backward compatibility
pub use network_core::{NetworkExtensions, UnifiedNetworkConfig};
pub use network_settings::{
    NetworkApiSettings, NetworkConnectionSettings, NetworkFileSystemSettings,
    NetworkLoadBalancingSettings, NetworkPortSettings, NetworkProtocolSettings, NetworkQosSettings,
    NetworkVlanSettings,
};
