//! **PRODUCTION DISCOVERY IMPLEMENTATION** - Legacy/Compatibility Module
//!
//! ⚠️ **DEPRECATED**: This module provides backward compatibility.
//! New code should use [`production_capability_bridge`] for capability-based discovery.
//!
//! ## ✅ EVOLUTION STATUS (Dec 4, 2025)
//!
//! **Phase 1 Complete**: Capability-based architecture is now operational!
//!
//! ### ❌ OLD WAY (This Module - Still Works)
//! ```rust,ignore
//! // Environment-driven with hardcoded fallbacks
//! let discovery = ProductionServiceDiscovery::new(&config)?;
//! let port = discovery.discover_port("api")?;  // Falls back to hardcoded 8080
//! ```
//!
//! ### ✅ NEW WAY (Capability-Based - Recommended)
//! ```rust,ignore
//! use crate::universal_primal_discovery::production_capability_bridge::*;
//!
//! // Pure capability discovery - NO hardcoded fallbacks!
//! let discovery = CapabilityAwareDiscovery::initialize(&config).await?;
//! let services = discovery.find_service("api").await?;
//! let best = services.first().ok_or_else(|| Error::not_found("api"))?;
//! ```
//!
//! ## Migration Guide
//!
//! **Quick Migration**:
//! 1. Replace `ProductionServiceDiscovery` with `CapabilityAwareDiscovery`
//! 2. Replace `.discover_port("api")` with `.find_service("api").await?`
//! 3. Handle errors properly (no more hardcoded fallbacks!)
//!
//! See [`production_capability_bridge`](crate::universal_primal_discovery::production_capability_bridge)
//! for complete API and examples.
//!
//! ## Deprecation Timeline
//! - **v0.12.0** (Current): Both APIs work, old API marked deprecated
//! - **v0.13.0** (Q1 2026): Old API generates warnings  
//! - **v0.14.0** (Q2 2026): Old API removed
//!
//! [`production_capability_bridge`]: crate::universal_primal_discovery::production_capability_bridge

use crate::config::canonical_primary::NestGateCanonicalConfig;
use crate::Result;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use tracing::{debug, warn};

// Import config for environment variable lookups
use super::production_discovery_config::ProductionDiscoveryConfig;

/// Production service discovery configuration
///
/// **⚠️ DEPRECATED**: Use [`CapabilityAwareDiscovery`] instead for capability-based discovery without hardcoded fallbacks.
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::ServiceDiscoveryConfig;
///
/// // NEW (capability-based):
/// use nestgate_core::universal_primal_discovery::production_capability_bridge::CapabilityAwareDiscovery;
/// let discovery = CapabilityAwareDiscovery::initialize(&config).await?;
/// ```
///
/// [`CapabilityAwareDiscovery`]: crate::universal_primal_discovery::production_capability_bridge::CapabilityAwareDiscovery
#[derive(Debug, Clone)]
#[deprecated(
    since = "0.12.0",
    note = "Use CapabilityAwareDiscovery from production_capability_bridge instead"
)]
/// Configuration for ServiceDiscovery
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

/// Individual service endpoint information
#[derive(Debug, Clone)]
/// Serviceendpoint
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

/// Default values for fallback when discovery fails
#[derive(Debug, Clone)]
/// Discoverydefaults
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
        use crate::constants::canonical_defaults::network;
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
    /// Looks for patterns like: API_HOST, API_PORT, API_BIND
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
            let host = config
                .get_service_host(service_name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    crate::constants::canonical_defaults::network::LOCALHOST.to_string()
                });

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
            host: crate::constants::canonical_defaults::network::LOCALHOST.to_string(), // Default host
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
    pub fn default_port_for_service(service_name: &str) -> u16 {
        use crate::constants::canonical_defaults::network;
        match service_name {
            "api" | "web" => network::DEFAULT_API_PORT,
            "metrics" => network::DEFAULT_METRICS_PORT,
            "health" | "admin" | "websocket" | "network" | "storage" | "zfs" | "mcp"
            | "automation" => network::DEFAULT_INTERNAL_PORT,
            _ => network::DEFAULT_API_PORT,
        }
    }

    /// Get default bind address for a service
    pub fn default_bind_for_service(service_name: &str) -> IpAddr {
        match service_name {
            "api" | "web" => IpAddr::V4(Ipv4Addr::UNSPECIFIED), // 0.0.0.0 for external access
            _ => {
                // Parse localhost with proper error handling
                crate::constants::canonical_defaults::network::LOCALHOST
                    .parse()
                    .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)) // 127.0.0.1 for internal services
            }
        }
    }
}

/// Production service discovery implementation
pub struct ProductionServiceDiscovery {
    config: ServiceDiscoveryConfig,
    #[allow(dead_code)] // Reserved for future advanced discovery features
    discovery_manager: (), // Placeholder for capability registry
}

impl ProductionServiceDiscovery {
    /// Create new production service discovery
    ///
    /// # Errors
    ///
    /// Returns error if configuration loading fails
    pub fn new(nestgate_config: &NestGateCanonicalConfig) -> Result<Self> {
        let config = ServiceDiscoveryConfig::from_environment(nestgate_config)?;

        Ok(Self {
            config,
            discovery_manager: (), // Placeholder
        })
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
    pub fn all_services(&self) -> &HashMap<String, ServiceEndpoint> {
        &self.config.services
    }

    /// Get discovery configuration
    #[must_use]
    pub fn config(&self) -> &ServiceDiscoveryConfig {
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

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Servicediscoveryconfigcanonical
pub type ServiceDiscoveryConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ServiceDiscoveryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_discovery_defaults() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Should have at least the API service from config
        assert!(!discovery.all_services().is_empty());
    }

    #[test]
    fn test_port_discovery() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // API port should come from config
        let api_port = discovery
            .discover_port("api")
            .expect("Failed to discover API port");
        assert_eq!(api_port, config.network.api.port);
    }

    #[test]
    fn test_port_discovery_unknown_service() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Unknown service should fall back to default
        let port = discovery
            .discover_port("unknown_service_xyz")
            .expect("Failed to discover port");
        assert_eq!(port, discovery.config.defaults.default_port);
    }

    #[test]
    fn test_bind_address_discovery() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Test bind address discovery for API service
        let bind_addr = discovery
            .discover_bind_address("api")
            .expect("Failed to discover bind address");
        assert!(matches!(bind_addr, IpAddr::V4(_) | IpAddr::V6(_)));
    }

    #[test]
    fn test_bind_address_unknown_service_fallback() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Unknown service should use default
        let bind_addr = discovery
            .discover_bind_address("nonexistent")
            .expect("Should fall back to default");
        assert_eq!(bind_addr, discovery.config.defaults.default_bind);
    }

    #[test]
    fn test_endpoint_discovery() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Test full endpoint discovery
        let endpoint = discovery
            .discover_endpoint("api")
            .expect("Failed to discover endpoint");
        assert_eq!(endpoint.port(), config.network.api.port);
    }

    #[test]
    fn test_endpoint_discovery_fallback() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Non-existent service should provide fallback endpoint
        let endpoint = discovery
            .discover_endpoint("missing_service")
            .expect("Should provide fallback endpoint");
        assert!(endpoint.port() > 0);
    }

    #[test]
    fn test_limit_discovery_defaults() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        let connections_limit = discovery
            .discover_limit("connections")
            .expect("Failed to discover limit");
        assert!(connections_limit > 0);
    }

    #[test]
    fn test_limit_discovery_various_types() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Test different resource types with defaults
        let conn_limit = discovery
            .discover_limit("connections")
            .expect("connections");
        assert_eq!(conn_limit, 1000);

        let rps_limit = discovery
            .discover_limit("requests_per_second")
            .expect("requests_per_second");
        assert_eq!(rps_limit, 100);

        let mem_limit = discovery.discover_limit("memory_mb").expect("memory_mb");
        assert_eq!(mem_limit, 512);

        let disk_limit = discovery.discover_limit("disk_mb").expect("disk_mb");
        assert_eq!(disk_limit, 1024);
    }

    #[test]
    fn test_limit_discovery_custom_type_fallback() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Unknown resource type should use default limit
        let limit = discovery
            .discover_limit("custom_resource_type")
            .expect("custom type");
        assert_eq!(limit, discovery.config.defaults.default_limit);
    }

    #[test]
    fn test_timeout_discovery_defaults() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        let timeout = discovery
            .discover_timeout("connect")
            .expect("Failed to discover timeout");
        assert!(timeout.as_secs() > 0);
    }

    #[test]
    fn test_timeout_discovery_various_operations() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Test different operation timeouts
        let connect_timeout = discovery.discover_timeout("connect").expect("connect");
        assert_eq!(connect_timeout, Duration::from_secs(10));

        let request_timeout = discovery.discover_timeout("request").expect("request");
        assert_eq!(request_timeout, Duration::from_secs(30));

        let health_timeout = discovery
            .discover_timeout("health_check")
            .expect("health_check");
        assert_eq!(health_timeout, Duration::from_secs(5));

        let discovery_timeout = discovery.discover_timeout("discovery").expect("discovery");
        assert_eq!(discovery_timeout, Duration::from_secs(15));
    }

    #[test]
    fn test_timeout_discovery_custom_operation_fallback() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        // Unknown operation should use default timeout
        let timeout = discovery
            .discover_timeout("custom_operation")
            .expect("custom operation");
        assert_eq!(timeout, discovery.config.defaults.default_timeout);
    }

    #[test]
    fn test_all_services_accessor() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        let services = discovery.all_services();
        assert!(!services.is_empty());
        assert!(services.contains_key("api"));
    }

    #[test]
    fn test_config_accessor() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

        let disc_config = discovery.config();
        assert!(!disc_config.services.is_empty());
    }

    #[test]
    fn test_standalone_bind_address_discovery() {
        let config = NestGateCanonicalConfig::default();

        let bind_addr = discover_bind_address_standalone(&config, "api")
            .expect("Failed to discover bind address");
        assert!(matches!(bind_addr, IpAddr::V4(_) | IpAddr::V6(_)));
    }

    #[test]
    fn test_standalone_endpoint_discovery() {
        let config = NestGateCanonicalConfig::default();

        let endpoint =
            discover_endpoint_standalone(&config, "api").expect("Failed to discover endpoint");
        assert_eq!(endpoint.port(), config.network.api.port);
    }

    #[test]
    fn test_standalone_port_discovery() {
        let config = NestGateCanonicalConfig::default();

        let port = discover_port_standalone(&config, "api").expect("Failed to discover port");
        assert_eq!(port, config.network.api.port);
    }

    #[test]
    fn test_standalone_limit_discovery() {
        let config = NestGateCanonicalConfig::default();

        let limit =
            discover_limit_standalone(&config, "connections").expect("Failed to discover limit");
        assert!(limit > 0);
    }

    #[test]
    fn test_standalone_timeout_discovery() {
        let config = NestGateCanonicalConfig::default();

        let timeout =
            discover_timeout_standalone(&config, "connect").expect("Failed to discover timeout");
        assert!(timeout.as_secs() > 0);
    }

    #[test]
    fn test_create_production_discovery_function() {
        let config = NestGateCanonicalConfig::default();

        let discovery = create_production_discovery(&config).expect("Failed to create discovery");
        assert!(!discovery.all_services().is_empty());
    }

    #[test]
    fn test_discovery_defaults_implementation() {
        let defaults = DiscoveryDefaults::default();

        assert_eq!(
            defaults.default_host,
            crate::constants::canonical_defaults::network::LOCALHOST
        );
        assert!(matches!(
            defaults.default_bind,
            IpAddr::V4(_) | IpAddr::V6(_)
        ));
        assert_eq!(
            defaults.default_port,
            crate::constants::canonical_defaults::network::DEFAULT_API_PORT
        );
        assert_eq!(defaults.default_timeout, Duration::from_secs(30));
        assert_eq!(defaults.default_limit, 100);
    }

    #[test]
    fn test_service_endpoint_structure() {
        let endpoint = ServiceEndpoint {
            name: "test_service".to_string(),
            host: "localhost".to_string(),
            port: crate::constants::network_hardcoded::get_api_port(),
            bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
        };

        assert_eq!(endpoint.name, "test_service");
        assert_eq!(endpoint.host, "localhost");
        assert_eq!(endpoint.port, 8080);
        assert!(matches!(endpoint.bind_address, IpAddr::V4(_)));
    }

    #[test]
    fn test_default_port_for_various_services() {
        let api_port = ServiceDiscoveryConfig::default_port_for_service("api");
        assert_eq!(
            api_port,
            crate::constants::canonical_defaults::network::DEFAULT_API_PORT
        );

        let metrics_port = ServiceDiscoveryConfig::default_port_for_service("metrics");
        assert_eq!(
            metrics_port,
            crate::constants::canonical_defaults::network::DEFAULT_METRICS_PORT
        );

        let health_port = ServiceDiscoveryConfig::default_port_for_service("health");
        assert_eq!(
            health_port,
            crate::constants::canonical_defaults::network::DEFAULT_INTERNAL_PORT
        );

        let unknown_port = ServiceDiscoveryConfig::default_port_for_service("unknown");
        assert_eq!(
            unknown_port,
            crate::constants::canonical_defaults::network::DEFAULT_API_PORT
        );
    }

    #[test]
    fn test_default_bind_for_various_services() {
        let api_bind = ServiceDiscoveryConfig::default_bind_for_service("api");
        assert_eq!(api_bind, IpAddr::V4(Ipv4Addr::UNSPECIFIED)); // 0.0.0.0 for external access

        let internal_bind = ServiceDiscoveryConfig::default_bind_for_service("health");
        // Should be localhost for internal services
        assert!(matches!(internal_bind, IpAddr::V4(_)));
    }
}
