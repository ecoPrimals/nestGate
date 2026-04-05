// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL NETWORK CONFIGURATION TYPES**
//!
//! Network interface, protocol, connection, security, performance,
//! load balancing, service discovery, and monitoring configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkInterfaceConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkInterfaceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkInterface
pub struct NetworkInterfaceConfig {
    /// Bind Addresses
    pub bind_addresses: Vec<String>,
    /// Primary Port
    pub primary_port: u16,
    /// Additional Ports
    pub additional_ports: Vec<u16>,
    /// Interface Preferences
    pub interface_preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkProtocolsConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkProtocolsConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkProtocols
pub struct NetworkProtocolsConfig {
    /// Http Enabled
    pub http_enabled: bool,
    /// Https Enabled
    pub https_enabled: bool,
    /// Websocket Enabled
    pub websocket_enabled: bool,
    /// Grpc Enabled
    pub grpc_enabled: bool,
    /// Tcp Enabled
    pub tcp_enabled: bool,
    /// Udp Enabled
    pub udp_enabled: bool,
    /// Protocol Versions
    pub protocol_versions: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkConnectionConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkConnectionConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkConnection
pub struct NetworkConnectionConfig {
    /// Max Connections
    pub max_connections: usize,
    /// Connection Timeout
    pub connection_timeout: Duration,
    /// Read Timeout
    pub read_timeout: Duration,
    /// Write Timeout
    pub write_timeout: Duration,
    /// Keep Alive
    pub keep_alive: bool,
    /// Keep Alive Timeout
    pub keep_alive_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkSecurityConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkSecurityConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkSecurity
pub struct NetworkSecurityConfig {
    /// Tls Enabled
    pub tls_enabled: bool,
    /// Client Auth Required
    pub client_auth_required: bool,
    /// Cipher Suites
    pub cipher_suites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkPerformanceConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkPerformanceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkPerformance
pub struct NetworkPerformanceConfig {
    /// Tcp Nodelay
    pub tcp_nodelay: bool,
    /// Tcp Keepalive
    pub tcp_keepalive: bool,
    /// Buffer Sizes
    pub buffer_sizes: NetworkBufferConfig,
    /// Congestion Control
    pub congestion_control: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkBufferConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkBufferConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkBuffer
pub struct NetworkBufferConfig {
    /// Size of send buffer
    pub send_buffer_size: Option<usize>,
    /// Size of recv buffer
    pub recv_buffer_size: Option<usize>,
    /// Size of socket buffer
    pub socket_buffer_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkLoadBalancingConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkLoadBalancingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkLoadBalancing
pub struct NetworkLoadBalancingConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Strategy
    pub strategy: LoadBalancingStrategy,
    /// Health Check Enabled
    pub health_check_enabled: bool,
    /// Backends
    pub backends: Vec<BackendConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadbalancingstrategy
pub enum LoadBalancingStrategy {
    /// Roundrobin
    RoundRobin,
    /// Leastconnections
    LeastConnections,
    /// Weightedroundrobin
    WeightedRoundRobin,
    /// Iphash
    IpHash,
    /// Random
    Random,
}

impl Default for LoadBalancingStrategy {
    /// Returns the default instance
    fn default() -> Self {
        Self::RoundRobin
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Backend
pub struct BackendConfig {
    /// Endpoint
    pub endpoint: String,
    /// Port
    pub port: u16,
    /// Weight
    pub weight: u32,
    /// Max Connections
    pub max_connections: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkServiceDiscoveryConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkServiceDiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkServiceDiscovery
pub struct NetworkServiceDiscoveryConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Discovery Method
    pub discovery_method: ServiceDiscoveryMethod,
    /// Service Registry
    pub service_registry: Option<String>,
    /// Discovery Interval
    pub discovery_interval: Duration,
    /// Registration Ttl
    pub registration_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicediscoverymethod
pub enum ServiceDiscoveryMethod {
    /// Dns
    Dns,
    /// Consul
    Consul,
    /// Etcd
    Etcd,
    /// Kubernetes
    Kubernetes,
    /// Static
    Static,
}

impl Default for ServiceDiscoveryMethod {
    /// Returns the default instance
    fn default() -> Self {
        Self::Dns
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkMonitoringConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkMonitoringConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkMonitoring
pub struct NetworkMonitoringConfig {
    /// Connection Tracking
    pub connection_tracking: bool,
    /// Bandwidth Monitoring
    pub bandwidth_monitoring: bool,
    /// Latency Monitoring
    pub latency_monitoring: bool,
    /// Error Rate Monitoring
    pub error_rate_monitoring: bool,
    /// Metrics Export
    pub metrics_export: bool,
}
