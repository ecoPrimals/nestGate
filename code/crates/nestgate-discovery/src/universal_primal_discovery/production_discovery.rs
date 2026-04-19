// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Discovery APIs use Result for forward-compatible error propagation"
)]

//! Production service discovery — environment and config-driven endpoint resolution.
//!
//! Provides a fallback strategy for resolving service endpoints when capability-based
//! IPC discovery is not yet available:
//! 1. Environment variables (`{SERVICE}_HOST`, `{SERVICE}_PORT`, `{SERVICE}_BIND`)
//! 2. Canonical config (`NestGateCanonicalConfig`)
//! 3. Network defaults from `nestgate_config::constants::canonical_defaults`
//!
//! For full capability-based discovery, see
//! [`production_capability_bridge`](crate::universal_primal_discovery::production_capability_bridge).

use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_types::error::Result;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use tracing::{debug, warn};

// Import config for environment variable lookups
use super::production_discovery_config::ProductionDiscoveryConfig;

/// Service discovery configuration — endpoints, limits, and timeouts
/// resolved from environment and canonical config.
#[derive(Debug, Clone)]
pub struct ServiceDiscoveryConfig {
    /// Service endpoints discovered from environment/config
    pub services: HashMap<String, ServiceEndpoint>,
    /// Resource limits from environment/config
    pub resource_limits: HashMap<String, usize>,
    /// Operation timeouts from environment/config
    pub operation_timeouts: HashMap<String, Duration>,
    /// Default values when discovery fails
    pub defaults: DiscoveryDefaults,
}

/// Individual service endpoint information.
#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    /// Name
    pub name: String,
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Bind Address
    pub bind_address: IpAddr,
}

/// Default values for fallback when discovery fails.
#[derive(Debug, Clone)]
pub struct DiscoveryDefaults {
    /// Default Host
    pub default_host: String,
    /// Default Bind
    pub default_bind: IpAddr,
    /// Default Port
    pub default_port: u16,
    /// Default Timeout
    pub default_timeout: Duration,
    /// Default Limit
    pub default_limit: usize,
}

impl Default for DiscoveryDefaults {
    /// Returns the default instance
    fn default() -> Self {
        use nestgate_config::constants::canonical_defaults::network;
        use std::net::Ipv4Addr;

        // Parse localhost address with safe fallback
        // Note: This unwrap_or is safe - we're parsing a known constant
        // and providing a guaranteed-valid fallback
        let default_bind = network::LOCALHOST
            .parse()
            .unwrap_or(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST));

        Self {
            default_host: network::LOCALHOST.to_string(),
            default_bind,
            default_port: network::DEFAULT_API_PORT,
            default_timeout: Duration::from_secs(30),
            default_limit: 100,
        }
    }
}

impl Default for ServiceDiscoveryConfig {
    /// Returns a default configuration with standard service endpoints
    fn default() -> Self {
        let defaults = DiscoveryDefaults::default();
        let mut services = HashMap::new();
        let mut resource_limits = HashMap::new();
        let mut operation_timeouts = HashMap::new();

        // Add standard service endpoints
        services.insert(
            "api".to_string(),
            ServiceEndpoint {
                name: "api".to_string(),
                host: defaults.default_host.clone(),
                port: defaults.default_port,
                bind_address: defaults.default_bind,
            },
        );

        // Add default resource limits
        resource_limits.insert("max_connections".to_string(), defaults.default_limit);
        resource_limits.insert("max_retries".to_string(), 3);

        // Add default timeouts
        operation_timeouts.insert("default".to_string(), defaults.default_timeout);

        Self {
            services,
            resource_limits,
            operation_timeouts,
            defaults,
        }
    }
}

impl ServiceDiscoveryConfig {
    /// Create configuration from environment and config files
    ///
    /// # Errors
    ///
    /// Returns error if environment parsing fails or config is invalid
    pub fn from_environment(config: &NestGateCanonicalConfig) -> Result<Self> {
        let mut services = HashMap::new();
        let mut resource_limits = HashMap::new();
        let mut operation_timeouts = HashMap::new();

        // Discover services from environment variables
        Self::discover_services_from_env(&mut services)?;

        // Discover from configuration
        Self::discover_services_from_config(config, &mut services)?;

        // Discover resource limits
        Self::discover_limits_from_env(&mut resource_limits)?;

        // Discover timeouts
        Self::discover_timeouts_from_env(&mut operation_timeouts)?;

        Ok(Self {
            services,
            resource_limits,
            operation_timeouts,
            defaults: DiscoveryDefaults::default(),
        })
    }

    /// Discover services from environment variables
    ///
    /// Looks for patterns like: `API_HOST`, `API_PORT`, `API_BIND`
    fn discover_services_from_env(services: &mut HashMap<String, ServiceEndpoint>) -> Result<()> {
        let service_names = [
            "API",
            "WEB",
            "METRICS",
            "HEALTH",
            "ADMIN",
            "WEBSOCKET",
            "NETWORK",
            "STORAGE",
            "ZFS",
            "MCP",
            "AUTOMATION",
        ];

        // Use config to get environment variables
        let config = ProductionDiscoveryConfig::from_env();

        for service_name in &service_names {
            let name_lower = service_name.to_lowercase();

            // Discover host (via config)
            let host = config.get_service_host(service_name).map_or_else(
                || nestgate_config::constants::canonical_defaults::network::LOCALHOST.to_string(),
                std::string::ToString::to_string,
            );

            // Discover port (via config, with safe fallback)
            let port = config
                .get_service_port(service_name)
                .unwrap_or_else(|| Self::default_port_for_service(&name_lower));

            // Discover bind address (via config, with safe fallback)
            let bind_address = config
                .get_service_bind(service_name)
                .and_then(|addr| IpAddr::from_str(addr).ok())
                .unwrap_or_else(|| Self::default_bind_for_service(&name_lower));

            services.insert(
                name_lower.clone(),
                ServiceEndpoint {
                    name: name_lower,
                    host,
                    port,
                    bind_address,
                },
            );
        }

        debug!("Discovered {} services from environment", services.len());
        Ok(())
    }

    /// Discover services from configuration
    fn discover_services_from_config(
        config: &NestGateCanonicalConfig,
        services: &mut HashMap<String, ServiceEndpoint>,
    ) -> Result<()> {
        // Use API config as primary service
        let api_service = ServiceEndpoint {
            name: "api".to_string(),
            host: nestgate_config::constants::canonical_defaults::network::LOCALHOST.to_string(), // Default host
            port: config.network.api.port,
            bind_address: IpAddr::V4(Ipv4Addr::UNSPECIFIED), // Bind to all by default
        };
        services.insert("api".to_string(), api_service);

        debug!("Discovered services from configuration");
        Ok(())
    }

    /// Discover resource limits from environment
    fn discover_limits_from_env(limits: &mut HashMap<String, usize>) -> Result<()> {
        // Use config to get environment variables
        let config = ProductionDiscoveryConfig::from_env();

        // Copy all discovered limits from config
        for (key, value) in config.get_all_resource_limits() {
            limits.insert(key.clone(), *value);
        }

        debug!(
            "Discovered {} resource limits from environment",
            limits.len()
        );
        Ok(())
    }

    /// Discover operation timeouts from environment
    fn discover_timeouts_from_env(timeouts: &mut HashMap<String, Duration>) -> Result<()> {
        // Use config to get environment variables
        let config = ProductionDiscoveryConfig::from_env();

        // Copy all discovered timeouts from config (convert seconds to Duration)
        for (key, value_secs) in config.get_all_operation_timeouts() {
            timeouts.insert(key.clone(), Duration::from_secs(*value_secs));
        }

        debug!(
            "Discovered {} operation timeouts from environment",
            timeouts.len()
        );
        Ok(())
    }

    /// Get default port for a service (used when env var not set)
    #[must_use]
    pub fn default_port_for_service(service_name: &str) -> u16 {
        use nestgate_config::constants::canonical_defaults::network;
        match service_name {
            "metrics" => network::DEFAULT_METRICS_PORT,
            "health" | "admin" | "websocket" | "network" | "storage" | "zfs" | "mcp"
            | "automation" => network::DEFAULT_INTERNAL_PORT,
            _ => network::DEFAULT_API_PORT,
        }
    }

    /// Get default bind address for a service
    #[must_use]
    pub fn default_bind_for_service(service_name: &str) -> IpAddr {
        match service_name {
            "api" | "web" => IpAddr::V4(Ipv4Addr::UNSPECIFIED), // 0.0.0.0 for external access
            _ => {
                // Parse localhost with proper error handling
                nestgate_config::constants::canonical_defaults::network::LOCALHOST
                    .parse()
                    .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)) // 127.0.0.1 for internal services
            }
        }
    }
}

/// Production service discovery implementation.
///
/// Resolves endpoints, ports, resource limits, and timeouts from environment
/// variables and canonical config, with documented defaults as fallback.
pub struct ProductionServiceDiscovery {
    config: ServiceDiscoveryConfig,
}

impl ProductionServiceDiscovery {
    /// Create new production service discovery
    ///
    /// # Errors
    ///
    /// Returns error if configuration loading fails
    pub fn new(nestgate_config: &NestGateCanonicalConfig) -> Result<Self> {
        let config = ServiceDiscoveryConfig::from_environment(nestgate_config)?;

        Ok(Self { config })
    }

    /// Discover bind address for a service
    ///
    /// Priority: Environment variable → Config → Default
    ///
    /// # Errors
    ///
    /// Returns error only if discovery fails catastrophically (rare)
    pub fn discover_bind_address(&self, service_name: &str) -> Result<IpAddr> {
        // Try to get from discovered services
        if let Some(service) = self.config.services.get(service_name) {
            return Ok(service.bind_address);
        }

        // Fall back to default
        warn!(
            "Service '{}' not found in discovery, using default bind address",
            service_name
        );
        Ok(self.config.defaults.default_bind)
    }

    /// Discover endpoint for a service
    ///
    /// # Errors
    ///
    /// Returns error if service discovery fails
    pub fn discover_endpoint(&self, service_name: &str) -> Result<SocketAddr> {
        if let Some(service) = self.config.services.get(service_name) {
            return Ok(SocketAddr::new(service.bind_address, service.port));
        }

        // Fall back to default
        warn!(
            "Service '{}' not found, using default endpoint",
            service_name
        );
        Ok(SocketAddr::new(
            self.config.defaults.default_bind,
            self.config.defaults.default_port,
        ))
    }

    /// Discover port for a service
    ///
    /// # Errors
    ///
    /// Returns error if discovery fails
    pub fn discover_port(&self, service_name: &str) -> Result<u16> {
        if let Some(service) = self.config.services.get(service_name) {
            return Ok(service.port);
        }

        warn!("Service '{}' port not found, using default", service_name);
        Ok(self.config.defaults.default_port)
    }

    /// Discover resource limit
    ///
    /// # Errors
    ///
    /// Returns error if discovery fails
    pub fn discover_limit(&self, resource_type: &str) -> Result<usize> {
        if let Some(&limit) = self.config.resource_limits.get(resource_type) {
            return Ok(limit);
        }

        // Default limits based on resource type
        let default_limit = match resource_type {
            "connections" => 1000,
            "requests_per_second" => 100,
            "memory_mb" => 512,
            "disk_mb" => 1024,
            _ => self.config.defaults.default_limit,
        };

        debug!(
            "Resource limit for '{}' not configured, using default: {}",
            resource_type, default_limit
        );
        Ok(default_limit)
    }

    /// Discover operation timeout
    ///
    /// # Errors
    ///
    /// Returns error if discovery fails
    pub fn discover_timeout(&self, operation: &str) -> Result<Duration> {
        if let Some(&timeout) = self.config.operation_timeouts.get(operation) {
            return Ok(timeout);
        }

        // Default timeouts based on operation type
        let default_timeout = match operation {
            "connect" => Duration::from_secs(10),
            "request" => Duration::from_secs(30),
            "health_check" => Duration::from_secs(5),
            "discovery" => Duration::from_secs(15),
            _ => self.config.defaults.default_timeout,
        };

        debug!(
            "Timeout for '{}' not configured, using default: {:?}",
            operation, default_timeout
        );
        Ok(default_timeout)
    }

    /// Get all discovered services
    #[must_use]
    pub const fn all_services(&self) -> &HashMap<String, ServiceEndpoint> {
        &self.config.services
    }

    /// Get discovery configuration
    #[must_use]
    pub const fn config(&self) -> &ServiceDiscoveryConfig {
        &self.config
    }
}

/// Create production discovery from canonical config
///
/// # Errors
///
/// Returns error if configuration is invalid
pub fn create_production_discovery(
    config: &NestGateCanonicalConfig,
) -> Result<ProductionServiceDiscovery> {
    ProductionServiceDiscovery::new(config)
}

/// Standalone function for backward compatibility with stub API
///
/// # Errors
///
/// Returns error if discovery fails
pub fn discover_bind_address_standalone(
    config: &NestGateCanonicalConfig,
    service_name: &str,
) -> Result<IpAddr> {
    let discovery = ProductionServiceDiscovery::new(config)?;
    discovery.discover_bind_address(service_name)
}

/// Standalone function for endpoint discovery
///
/// # Errors
///
/// Returns error if discovery fails
pub fn discover_endpoint_standalone(
    config: &NestGateCanonicalConfig,
    service_name: &str,
) -> Result<SocketAddr> {
    let discovery = ProductionServiceDiscovery::new(config)?;
    discovery.discover_endpoint(service_name)
}

/// Standalone function for port discovery
///
/// # Errors
///
/// Returns error if discovery fails
pub fn discover_port_standalone(
    config: &NestGateCanonicalConfig,
    service_name: &str,
) -> Result<u16> {
    let discovery = ProductionServiceDiscovery::new(config)?;
    discovery.discover_port(service_name)
}

/// Standalone function for limit discovery
///
/// # Errors
///
/// Returns error if discovery fails
pub fn discover_limit_standalone(
    config: &NestGateCanonicalConfig,
    resource_type: &str,
) -> Result<usize> {
    let discovery = ProductionServiceDiscovery::new(config)?;
    discovery.discover_limit(resource_type)
}

/// Standalone function for timeout discovery
///
/// # Errors
///
/// Returns error if discovery fails
pub fn discover_timeout_standalone(
    config: &NestGateCanonicalConfig,
    operation: &str,
) -> Result<Duration> {
    let discovery = ProductionServiceDiscovery::new(config)?;
    discovery.discover_timeout(operation)
}

/// Type alias to canonical network configuration for callers that import from this module.
pub type ServiceDiscoveryConfigCanonical =
    nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig;
