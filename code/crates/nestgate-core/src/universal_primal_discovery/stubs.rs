/// Universal Primal Discovery Stub Implementations
/// Provides fallback implementations for discovery operations when full discovery is unavailable.
/// These stubs ensure system stability and provide sensible defaults.
///
/// **⚠️ DEVELOPMENT ONLY**: This module is only available with `dev-stubs` feature
use crate::Result;
// **MIGRATED**: Using canonical config system instead of deprecated unified_types
use crate::capabilities::discovery::DiscoveryManager;
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
pub fn discover_endpoint(service_name: &str) -> Result<SocketAddr> {
    let port = get_fallback_port(service_name);
    let addr = discover_bind_address(service_name)?;
    Ok(SocketAddr::new(addr, port))
}
/// Discover limit for a service
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
    match service_name {
        "api" => 8080,
        "web" => 8080,
        "metrics" => 9090,
        "health" => 8081,
        "admin" => 8082,
        "websocket" => 8083,
        "network" => 8084,
        "storage" => 8085,
        "zfs" => 8086,
        "mcp" => 8087,
        "automation" => 8088,
        _ => 8080,
    }
}
/// Network configuration adapter for universal discovery
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_primary::domains::network`
#[deprecated(
    since = "0.9.0",
    note = "Use canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
#[allow(deprecated)]
pub struct NetworkConfigAdapter {
    #[allow(dead_code)]
    service_name: String,
    #[allow(dead_code)]
    config: NestGateCanonicalConfig,
    #[allow(dead_code)]
    discovery_manager: Arc<RwLock<DiscoveryManager>>,
    #[allow(dead_code)]
    stats: Arc<RwLock<AdapterStats>>,
}
#[allow(deprecated)]
impl NetworkConfigAdapter {
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
            discovery_manager: Arc::new(RwLock::new(DiscoveryManager::new())),
            stats: Arc::new(RwLock::new(AdapterStats::default())),
        }
    }

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
    discovery_manager: Arc<RwLock<DiscoveryManager>>,
    #[allow(dead_code)]
    stats: Arc<RwLock<AdapterStats>>,
    endpoints: HashMap<String, SocketAddr>,
}
impl StandaloneNetworkAdapter {
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
            discovery_manager: Arc::new(RwLock::new(DiscoveryManager::new())),
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

    #[must_use]
    pub fn all_endpoints(&self) -> HashMap<String, SocketAddr> {
        self.endpoints.clone()
    }

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
