//! **PRIMAL DISCOVERY STUBS** - Development/Testing Only
//!
//! ⚠️ **WARNING**: This module contains hardcoded values and should NEVER be used in production!
//!
//! ## ⛔ DO NOT USE IN PRODUCTION
//!
//! This module provides stub implementations with **hardcoded fallbacks** for testing only.
//! Production code MUST use [`CapabilityAwareDiscovery`] which has zero hardcoding.
//!
//! ## Purpose
//!
//! Provides fallback implementations for discovery operations **in development/test environments only**:
//! - Unit testing without real discovery infrastructure
//! - CI/CD pipelines
//! - Local development
//! - Integration tests
//!
//! ## ✅ Production Alternative
//!
//! ```rust,ignore
//! // ❌ NEVER DO THIS IN PRODUCTION:
//! use nestgate_core::dev_stubs::primal_discovery::discover_port;
//! let port = discover_port("api")?;  // Returns hardcoded 8080!
//!
//! // ✅ PRODUCTION CODE SHOULD USE:
//! use nestgate_core::universal_primal_discovery::production_capability_bridge::*;
//! let discovery = CapabilityAwareDiscovery::initialize(&config).await?;
//! let services = discovery.find_service("api").await?;  // Runtime discovery!
//! ```
//!
//! ## Feature Gate
//!
//! This module should be gated behind `#[cfg(test)]` or a `dev-mode` feature flag.
//!
//! ## Deprecation Timeline
//!
//! - **v0.12.0** (Current): Available but deprecated, warnings added
//! - **v0.13.0** (Q1 2026): Only available in test configuration
//! - **v0.14.0** (Q2 2026): Removed entirely
//!
//! [`CapabilityAwareDiscovery`]: crate::universal_primal_discovery::production_capability_bridge::CapabilityAwareDiscovery
use crate::Result;
// **MIGRATED**: Using canonical config system instead of deprecated unified_types
#[allow(deprecated)]
use crate::config::canonical_primary::{
    domains::network::CanonicalNetworkConfig as UnifiedNetworkConfig, NestGateCanonicalConfig,
};
use crate::universal_adapter::stats::AdapterStats;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, RwLock};
use std::time::Duration;
// Deprecated type alias removed - use UnifiedNetworkConfig directly

/// Discover bind address for a service
///
/// ⚠️ **WARNING**: Returns hardcoded values! Do not use in production!
///
/// **Production Alternative**: Use `CapabilityAwareDiscovery::find_service()` instead.
#[deprecated(
    since = "0.12.0",
    note = "Use CapabilityAwareDiscovery for production - this returns hardcoded values!"
)]
pub fn discover_bind_address(service_name: &str) -> Result<IpAddr> {
    match service_name {
        "api" | "web" | "http" => Ok(crate::safe_operations::safe_parse_ip_with_fallback(
            "0.0.0.0",
            std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
            "stub_service_discovery",
        )),
        "internal" | "database" | "cache" => {
            Ok(crate::safe_operations::safe_parse_ip_with_fallback(
                "127.0.0.1",
                std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                "stub_internal_services",
            ))
        }
        _ => Ok(crate::safe_operations::safe_parse_ip_with_fallback(
            "127.0.0.1",
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            "stub_default_fallback",
        )),
    }
}
/// Discover endpoint for a service
///
/// ⚠️ **WARNING**: Returns hardcoded values! Do not use in production!
#[deprecated(
    since = "0.12.0",
    note = "Use CapabilityAwareDiscovery for production - this returns hardcoded values!"
)]
pub fn discover_endpoint(service_name: &str) -> Result<SocketAddr> {
    let port = get_fallback_port(service_name);
    let addr = discover_bind_address(service_name)?;
    Ok(SocketAddr::new(addr, port))
}
/// Discover limit for a service
///
/// ⚠️ **WARNING**: Returns hardcoded values! Do not use in production!
#[deprecated(
    since = "0.12.0",
    note = "Use CapabilityAwareDiscovery for production - this returns hardcoded values!"
)]
pub fn discover_limit(resource_type: &str) -> Result<usize> {
    match resource_type {
        "connections" => Ok(1000),
        "requests_per_second" => Ok(100),
        "memory_mb" => Ok(512),
        "disk_mb" => Ok(1024),
        _ => Ok(100),
    }
}
/// Discover port for a service
///
/// ⚠️ **WARNING**: Returns hardcoded values! Do not use in production!
#[deprecated(
    since = "0.12.0",
    note = "Use CapabilityAwareDiscovery for production - this returns hardcoded values!"
)]
pub fn discover_port(service_name: &str) -> Result<u16> {
    Ok(get_fallback_port(service_name))
}
/// Discover timeout for a service
pub fn discover_timeout(operation: &str) -> crate::Result<Duration> {
    match operation {
        "connect" => Ok(Duration::from_secs(10)),
        "request" => Ok(Duration::from_secs(30)),
        "health_check" => Ok(Duration::from_secs(5)),
        "discovery" => Ok(Duration::from_secs(15)),
        _ => Ok(Duration::from_secs(30)),
    }
}
/// Get fallback port for a service
#[must_use]
pub fn get_fallback_port(service_name: &str) -> u16 {
    use crate::constants::port_defaults::{
        DEFAULT_ADMIN_PORT, DEFAULT_API_PORT, DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT,
    };

    match service_name {
        "api" => DEFAULT_API_PORT,         // 8080
        "web" => DEFAULT_API_PORT,         // 8080
        "metrics" => DEFAULT_METRICS_PORT, // 9090
        "health" => DEFAULT_HEALTH_PORT,   // 8081
        "admin" => DEFAULT_ADMIN_PORT,     // 8082
        "websocket" => 8083,               // WebSocket default
        "network" => 8084,                 // Network service default
        "storage" => 8085,                 // Storage service default
        "zfs" => 8086,                     // ZFS service default
        "mcp" => 8087,                     // MCP service default
        "automation" => 8088,              // Automation service default
        _ => DEFAULT_API_PORT,
    }
}
/// Network configuration adapter for universal discovery
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_primary::domains::network`
#[deprecated(
    since = "0.9.0",
    note = "Use canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
#[allow(deprecated)]
/// Networkconfigadapter
pub struct NetworkConfigAdapter {
    #[allow(dead_code)]
    service_name: String,
    #[allow(dead_code)]
    config: NestGateCanonicalConfig,
    #[allow(dead_code)]
    discovery_manager: Arc<RwLock<()>>, // Placeholder for capability registry
    #[allow(dead_code)]
    stats: Arc<RwLock<AdapterStats>>,
}
#[allow(deprecated)]
impl NetworkConfigAdapter {
    /// Creates a new NetworkConfigAdapter with default configuration
    ///
    /// # Arguments
    /// * `service_name` - Name of the service to configure
    #[must_use]
    pub fn new(service_name: String) -> Self {
        let network_config = UnifiedNetworkConfig {
            // Use default NetworkConfig structure - fields updated to match unified config
            ..Default::default()
        };

        let config = crate::config::canonical_primary::NestGateCanonicalConfig {
            network: network_config,
            ..Default::default()
        };

        Self {
            service_name,
            config,
            discovery_manager: Arc::new(RwLock::new(())), // Placeholder
            stats: Arc::new(RwLock::new(AdapterStats::default())),
        }
    }

    /// Returns a reference to the network configuration
    #[must_use]
    pub fn config(&self) -> &UnifiedNetworkConfig {
        &self.config.network
    }
}

/// Standalone network adapter for isolated deployments
pub struct StandaloneNetworkAdapter {
    #[allow(dead_code)]
    service_name: String,
    #[allow(dead_code)]
    config: NestGateCanonicalConfig,
    #[allow(dead_code)]
    discovery_manager: Arc<RwLock<()>>, // Placeholder for capability registry
    #[allow(dead_code)]
    stats: Arc<RwLock<AdapterStats>>,
    endpoints: HashMap<String, SocketAddr>,
}
impl StandaloneNetworkAdapter {
    /// Creates a new standalone network adapter for development/testing.
    ///
    /// This adapter provides a simple standalone configuration for development
    /// environments where full service discovery is not needed.
    ///
    /// # Arguments
    /// * `service_name` - The name of the service this adapter represents
    #[must_use]
    pub fn new(service_name: String) -> Self {
        let mut endpoints = HashMap::new();
        let port = get_fallback_port(&service_name);
        endpoints.insert(
            service_name.clone(),
            SocketAddr::from(([127, 0, 0, 1], port)),
        );

        Self {
            service_name,
            config: NestGateCanonicalConfig::default(), // Placeholder, needs proper initialization
            discovery_manager: Arc::new(RwLock::new(())), // Placeholder
            stats: Arc::new(RwLock::new(AdapterStats::default())),
            endpoints,
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn discover_endpoint(&self, service: &str) -> Result<SocketAddr> {
        discover_endpoint(service)
    }

    /// Returns all configured endpoints for this adapter.
    ///
    /// # Returns
    /// A map of service names to their socket addresses
    #[must_use]
    pub fn all_endpoints(&self) -> HashMap<String, SocketAddr> {
        self.endpoints.clone()
    }

    /// Returns whether this adapter is operating in standalone mode.
    ///
    /// # Returns
    /// Always returns `true` for the standalone adapter
    #[must_use]
    pub fn is_standalone(&self) -> bool {
        true
    }
}

/// **DEPRECATED**: Use `UnifiedNetworkConfig` from `crate::config::canonical_primary` instead
/// `UnifiedNetworkConfig` helper methods
#[allow(deprecated)] // Helper methods for deprecated type during migration
impl UnifiedNetworkConfig {
    /// Convert to unified config (identity function now)
    #[must_use]
    pub fn to_unified(&self) -> Self {
        self.clone()
    }
}
