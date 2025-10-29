use std::collections::HashMap;
///
/// This module provides configuration for test environments including containers,
/// networks, and environment setup.
use serde::{Deserialize, Serialize};
use std::time::Duration;
// ==================== SECTION ====================

/// **Unified test environment configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestEnvironmentConfig {
    /// Container configuration
    pub containers: ContainerConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// Environment variables
    pub environment_variables: HashMap<String, String>,
    /// Resource limits
    pub resource_limits: ResourceLimits,
}
/// **Container configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerConfig {
    /// Enable containers
    pub enabled: bool,
    /// Container images
    pub images: HashMap<String, String>,
    /// Container ports
    pub ports: HashMap<String, u16>,
    /// Container volumes
    pub volumes: HashMap<String, String>,
}
/// **Network configuration**
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_master::domains::network`
#[deprecated(since = "0.9.0", note = "Use canonical_master::domains::network::CanonicalNetworkConfig instead")]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    /// Network mode
    pub mode: String,
    /// Network ports
    pub ports: Vec<u16>,
    /// Network timeouts
    pub timeouts: HashMap<String, Duration>,
    /// SSL/TLS configuration
    pub ssl_config: Option<SslConfig>,
}
/// **SSL configuration**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SslConfig {
    /// Enable SSL
    pub enabled: bool,
    /// Certificate path
    /// Key path
    /// CA path
}
/// **Resource limits**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    /// CPU limit
    pub cpu_limit: Option<f64>,
    /// Memory limit (MB)
    pub memory_limit_mb: Option<u64>,
    /// Disk limit (MB)
    pub disk_limit_mb: Option<u64>,
    /// Network bandwidth limit (Mbps)
    pub network_limit_mbps: Option<f64>,
}
