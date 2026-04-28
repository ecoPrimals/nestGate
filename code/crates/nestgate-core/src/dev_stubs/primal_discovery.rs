// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **PRIMAL DISCOVERY STUBS** - Development/Testing Only
//!
//! **WARNING**: This module contains hardcoded values and should NEVER be used in production!
//!
//! ## DO NOT USE IN PRODUCTION
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
//! ## Production Alternative
//!
//! ```rust,ignore
//! // NEVER DO THIS IN PRODUCTION:
//! use nestgate_core::dev_stubs::primal_discovery::get_fallback_port;
//! let port = get_fallback_port("api");  // Returns a default port constant
//!
//! // PRODUCTION CODE SHOULD USE:
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
use crate::config::canonical_primary::NestGateCanonicalConfig;
use crate::universal_adapter::stats::AdapterStats;
use nestgate_config::LOCALHOST_IPV4;
use nestgate_config::constants::hardcoding::addresses::BIND_ALL_IPV4 as BIND_ALL_IPV4_STR;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, RwLock};
use std::time::Duration;

fn stub_loopback_ip() -> IpAddr {
    std::env::var("NESTGATE_DEV_HOST")
        .or_else(|_| std::env::var("NESTGATE_BIND_ADDRESS"))
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(LOCALHOST_IPV4))
}

fn stub_bind_addr_for_service(service_name: &str) -> IpAddr {
    match service_name {
        "api" | "web" | "http" => crate::safe_operations::safe_parse_ip_with_fallback(
            BIND_ALL_IPV4_STR,
            std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
            "stub_service_discovery",
        ),
        "internal" | "database" | "cache" => {
            let loopback = stub_loopback_ip();
            crate::safe_operations::safe_parse_ip_with_fallback(
                &loopback.to_string(),
                loopback,
                "stub_internal_services",
            )
        }
        _ => {
            let loopback = stub_loopback_ip();
            crate::safe_operations::safe_parse_ip_with_fallback(
                &loopback.to_string(),
                loopback,
                "stub_default_fallback",
            )
        }
    }
}

/// Discover timeout for a service
pub fn discover_timeout(operation: &str) -> crate::Result<Duration> {
    match operation {
        "connect" => Ok(Duration::from_secs(10)),
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

/// Standalone network adapter for isolated deployments
pub struct StandaloneNetworkAdapter {
    _service_name: String,
    _config: NestGateCanonicalConfig,
    _discovery_manager: Arc<RwLock<()>>, // Placeholder for capability registry
    _stats: Arc<RwLock<AdapterStats>>,
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
            SocketAddr::new(stub_loopback_ip(), port),
        );

        Self {
            _service_name: service_name,
            _config: NestGateCanonicalConfig::default(), // Placeholder, needs proper initialization
            _discovery_manager: Arc::new(RwLock::new(())),
            _stats: Arc::new(RwLock::new(AdapterStats::default())),
            endpoints,
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn discover_endpoint(&self, service: &str) -> Result<SocketAddr> {
        let port = get_fallback_port(service);
        let addr = stub_bind_addr_for_service(service);
        Ok(SocketAddr::new(addr, port))
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
    pub const fn is_standalone(&self) -> bool {
        true
    }
}
