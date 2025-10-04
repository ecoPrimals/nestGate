/// **⚠️ DEPRECATED - DO NOT USE**
///
/// This file is deprecated in favor of `domains/network/mod.rs`.
/// All NetworkConfig usage should migrate to `CanonicalNetworkConfig` from:
/// `use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig;`
///
/// **MIGRATION GUIDE**:
/// ```rust
/// // OLD:
/// use nestgate_core::config::canonical_master::network_config::NetworkConfig;
///
/// // NEW:
/// use nestgate_core::config::canonical_master::domains::network::CanonicalNetworkConfig;
/// ```
#[deprecated(
    since = "0.9.0",
    note = "Use canonical_master::domains::network::CanonicalNetworkConfig instead"
)]
/// **DEPRECATED**: Network and connectivity configuration with const generics for performance.
/// This module contains all network-related settings including API ports,
/// timeouts, load balancing, and service discovery.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

// ==================== SECTION ====================

/// Network configuration with const generics for performance
#[deprecated(
    since = "0.9.0",
    note = "Use canonical_master::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig<const API_PORT: u16 = 8080, const TIMEOUT_MS: u64 = 30000> {
    /// API server bind address
    pub bind_endpoint: IpAddr,
    /// API server port (compile-time optimized)
    pub port: u16,
    /// Request timeout (compile-time optimized)
    pub request_timeout: Duration,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// Enable TLS
    pub tls_enabled: bool,
    /// TLS certificate path
    /// TLS private key path
    /// Load balancer configuration
    pub load_balancer: LoadBalancerConfig,
    /// Service discovery configuration
    pub service_discovery: ServiceDiscoveryConfig,
    /// External network configuration
    pub external: ExternalNetworkConfig,
    /// Network-specific settings
    pub network_settings: HashMap<String, serde_json::Value>,
}
impl<const API_PORT: u16, const TIMEOUT_MS: u64> NetworkConfig<API_PORT, TIMEOUT_MS> {
    /// Get effective API port (compile-time optimized)
    #[must_use]
    pub fn api_port() -> u16 {
        API_PORT
    }

    /// Get effective timeout (compile-time optimized)
    #[must_use]
    pub fn timeout_ms() -> u64 {
        TIMEOUT_MS
    }

    /// Get timeout as Duration
    #[must_use]
    pub fn timeout_duration() -> Duration {
        Duration::from_millis(TIMEOUT_MS)
    }
}

/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Enable load balancing
    pub enabled: bool,
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Backend servers
    pub backends: Vec<BackendServer>,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
}
/// Load balancing algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
    Random,
}
/// Backend server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendServer {
    /// Server address
    pub endpoint: String,
    /// Server port
    pub port: u16,
    /// Server weight (for weighted algorithms)
    pub weight: u32,
    /// Server health status
    pub healthy: bool,
}
/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Health check endpoint
    pub endpoint: String,
    /// Expected HTTP status code
    pub expected_status: u16,
}
/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    /// Enable service discovery
    pub enabled: bool,
    /// Discovery method
    pub method: String,
    /// Discovery endpoints
    pub endpoints: Vec<String>,
    /// Discovery settings
    pub discovery_settings: HashMap<String, serde_json::Value>,
}
/// External network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalNetworkConfig {
    /// External host
    pub host: String,
    /// External port
    pub port: u16,
}
impl Default for ExternalNetworkConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

/// Service discovery protocols
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceDiscoveryProtocol {
    Consul,
    Etcd,
    Zookeeper,
    Kubernetes,
    Static,
}
/// Service registration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationConfig {
    /// Service name
    pub service_name: String,
    /// Service tags
    pub tags: Vec<String>,
    /// Registration interval
    pub registration_interval: Duration,
    /// Health check settings for registration
    pub health_check: Option<HealthCheckConfig>,
}
// ==================== SECTION ====================

impl<const API_PORT: u16, const TIMEOUT_MS: u64> Default for NetworkConfig<API_PORT, TIMEOUT_MS> {
    fn default() -> Self {
        Self {
            bind_endpoint: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: API_PORT,
            request_timeout: Duration::from_millis(TIMEOUT_MS),
            connection_timeout: Duration::from_secs(10),
            max_connections: 1000,
            keep_alive_timeout: Duration::from_secs(60),
            tls_enabled: false,
            load_balancer: LoadBalancerConfig::default(),
            service_discovery: ServiceDiscoveryConfig::default(),
            external: ExternalNetworkConfig::default(),
            network_settings: HashMap::new(),
        }
    }
}

impl Default for LoadBalancerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            backends: Vec::new(),
            health_check: HealthCheckConfig::default(),
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            endpoint: "/health".to_string(),
            expected_status: 200,
        }
    }
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: "static".to_string(),
            endpoints: Vec::new(),
            discovery_settings: HashMap::new(),
        }
    }
}

impl Default for ServiceRegistrationConfig {
    fn default() -> Self {
        Self {
            service_name: "nestgate".to_string(),
            tags: vec!["nestgate".to_string(), "storage".to_string()],
            registration_interval: Duration::from_secs(30),
            health_check: Some(HealthCheckConfig::default()),
        }
    }
}
