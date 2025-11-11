//! **PRODUCTION DISCOVERY IMPLEMENTATION**
//!
//! Environment-driven service discovery replacing hardcoded stubs.
//! Uses environment variables, configuration files, and runtime detection
//! to discover services dynamically without hardcoded values.

use crate::capabilities::discovery::DiscoveryManager;
use crate::config::canonical_primary::NestGateCanonicalConfig;
use crate::Result;
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use tracing::{debug, warn};

/// Production service discovery configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::ServiceDiscoveryConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ServiceDiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
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
pub struct ServiceEndpoint {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub bind_address: IpAddr,
}

/// Default values for fallback when discovery fails
#[derive(Debug, Clone)]
pub struct DiscoveryDefaults {
    pub default_host: String,
    pub default_bind: IpAddr,
    pub default_port: u16,
    pub default_timeout: Duration,
    pub default_limit: usize,
}

impl Default for DiscoveryDefaults {
    fn default() -> Self {
        use crate::constants::canonical_defaults::network;
        use std::net::Ipv4Addr;
        
        Self {
            default_host: network::LOCALHOST.to_string(),
            default_bind: network::LOCALHOST
                .parse()
                .unwrap_or(std::net::IpAddr::V4(Ipv4Addr::LOCALHOST)),
            default_port: network::DEFAULT_API_PORT,
            default_timeout: Duration::from_secs(30),
            default_limit: 100,
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

        for service_name in &service_names {
            let name_lower = service_name.to_lowercase();

            // Discover host
            let host = env::var(format!("{}_HOST", service_name)).unwrap_or_else(|_| {
                crate::constants::canonical_defaults::network::LOCALHOST.to_string()
            });

            // Discover port
            let port = env::var(format!("{}_PORT", service_name))
                .ok()
                .and_then(|p| p.parse::<u16>().ok())
                .unwrap_or_else(|| Self::default_port_for_service(&name_lower));

            // Discover bind address
            let bind_address = env::var(format!("{}_BIND", service_name))
                .ok()
                .and_then(|addr| IpAddr::from_str(&addr).ok())
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
        let limit_types = [
            ("CONNECTIONS", "connections"),
            ("REQUESTS_PER_SECOND", "requests_per_second"),
            ("MEMORY_MB", "memory_mb"),
            ("DISK_MB", "disk_mb"),
        ];

        for (env_suffix, limit_name) in &limit_types {
            if let Ok(value_str) = env::var(format!("NESTGATE_{}", env_suffix)) {
                if let Ok(value) = value_str.parse::<usize>() {
                    limits.insert(limit_name.to_string(), value);
                }
            }
        }

        debug!(
            "Discovered {} resource limits from environment",
            limits.len()
        );
        Ok(())
    }

    /// Discover operation timeouts from environment
    fn discover_timeouts_from_env(timeouts: &mut HashMap<String, Duration>) -> Result<()> {
        let timeout_types = [
            ("CONNECT", "connect"),
            ("REQUEST", "request"),
            ("HEALTH_CHECK", "health_check"),
            ("DISCOVERY", "discovery"),
        ];

        for (env_suffix, timeout_name) in &timeout_types {
            if let Ok(value_str) = env::var(format!("NESTGATE_{}_TIMEOUT", env_suffix)) {
                if let Ok(value_secs) = value_str.parse::<u64>() {
                    timeouts.insert(timeout_name.to_string(), Duration::from_secs(value_secs));
                }
            }
        }

        debug!(
            "Discovered {} operation timeouts from environment",
            timeouts.len()
        );
        Ok(())
    }

    /// Get default port for a service (used when env var not set)
    fn default_port_for_service(service_name: &str) -> u16 {
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
    fn default_bind_for_service(service_name: &str) -> IpAddr {
        match service_name {
            "api" | "web" => IpAddr::V4(Ipv4Addr::UNSPECIFIED), // 0.0.0.0 for external access
            _ => crate::constants::canonical_defaults::network::LOCALHOST
                .parse()
                .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)), // 127.0.0.1 for internal services
        }
    }
}

/// Production service discovery implementation
pub struct ProductionServiceDiscovery {
    config: ServiceDiscoveryConfig,
    #[allow(dead_code)] // Reserved for future advanced discovery features
    discovery_manager: DiscoveryManager,
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
            discovery_manager: DiscoveryManager::default(),
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
pub type ServiceDiscoveryConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

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
            .map_err(|e| NestGateError::Configuration {{ message: "Failed to create discovery".to_string(), source: Some(Box::new(e)) }})?

        // Should have at least the API service from config
        assert!(!discovery.all_services().is_empty());
    }

    #[test]
    fn test_port_discovery() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            .map_err(|e| NestGateError::Configuration {{ message: "Failed to create discovery".to_string(), source: Some(Box::new(e)) }})?

        // API port should come from config
        let api_port = discovery
            .discover_port("api")
            .map_err(|e| NestGateError::Configuration {{ message: "Failed to discover API port".to_string(), source: Some(Box::new(e)) }})?
        assert_eq!(api_port, config.network.api.port);
    }

    #[test]
    fn test_limit_discovery_defaults() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            .map_err(|e| NestGateError::Configuration {{ message: "Failed to create discovery".to_string(), source: Some(Box::new(e)) }})?

        let connections_limit = discovery
            .discover_limit("connections")
            .map_err(|e| NestGateError::Configuration {{ message: "Failed to discover limit".to_string(), source: Some(Box::new(e)) }})?
        assert!(connections_limit > 0);
    }

    #[test]
    fn test_timeout_discovery_defaults() {
        let config = NestGateCanonicalConfig::default();
        let discovery =
            .map_err(|e| NestGateError::Configuration {{ message: "Failed to create discovery".to_string(), source: Some(Box::new(e)) }})?

        let timeout = discovery
            .discover_timeout("connect")
            .map_err(|e| NestGateError::Configuration {{ message: "Failed to discover timeout".to_string(), source: Some(Box::new(e)) }})?
        assert!(timeout.as_secs() > 0);
    }
}
