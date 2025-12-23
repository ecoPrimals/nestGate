/// Contains all individual network configuration setting structs.
/// Extracted from the large `unified_network_config.rs` to achieve file size compliance.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(deprecated)] // Using NetworkVlanConfig during transition period
/// Networkvlansettings
pub struct NetworkVlanSettings {
    /// Enable VLAN support
    pub enabled: bool,
    /// VLAN configurations by VLAN ID
    /// Migration in progress to CanonicalNetworkConfig (transition period until v0.12.0)
    pub vlans: HashMap<u16, NetworkVlanConfig>,
    /// Default VLAN ID for untagged traffic
    pub default_vlan_id: u16,
    /// Enable VLAN trunking
    pub enable_trunking: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkVlanConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkVlanConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkVlan
pub struct NetworkVlanConfig {
    /// Vlan identifier
    pub vlan_id: u16,
    /// Name
    pub name: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Network
    pub network: String, // CIDR notation
    /// Gateway
    pub gateway: Option<IpAddr>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkprotocolsettings
pub struct NetworkProtocolSettings {
    /// Enabled protocols
    pub enabled_protocols: Vec<crate::protocol::Protocol>,
    /// Protocol-specific timeouts
    pub protocol_timeouts: HashMap<String, Duration>,
    /// Maximum concurrent protocol handlers
    pub max_protocol_handlers: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkconnectionsettings
pub struct NetworkConnectionSettings {
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Keep-alive settings
    pub keep_alive: bool,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkportsettings
pub struct NetworkPortSettings {
    /// Port allocation ranges
    pub port_ranges: Vec<PortRange>,
    /// Reserved ports
    pub reserved_ports: Vec<u16>,
    /// Dynamic port allocation enabled
    pub dynamic_allocation: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Portrange
pub struct PortRange {
    /// Start
    pub start: u16,
    /// End
    pub end: u16,
    /// Human-readable description
    pub description: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkapisettings
pub struct NetworkApiSettings {
    /// API endpoints configuration
    pub endpoints: HashMap<String, ApiEndpoint>,
    /// Default API timeout
    pub default_timeout: Duration,
    /// Maximum request size
    pub max_request_size: u64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Apiendpoint
pub struct ApiEndpoint {
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Path
    pub path: String,
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Option<Duration>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkfilesystemsettings
pub struct NetworkFileSystemSettings {
    /// NFS settings
    pub nfs: NfsSettings,
    /// SMB settings
    pub smb: SmbSettings,
    /// File system timeout
    pub timeout: Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Nfssettings
pub struct NfsSettings {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Version
    pub version: String,
    /// Export Paths
    pub export_paths: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Smbsettings
pub struct SmbSettings {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Version
    pub version: String,
    /// Share Paths
    pub share_paths: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkloadbalancingsettings
pub struct NetworkLoadBalancingSettings {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check settings
    pub health_check: HealthCheckSettings,
    /// Backend servers
    pub backends: Vec<BackendServer>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadbalancingalgorithm
pub enum LoadBalancingAlgorithm {
    /// Roundrobin
    RoundRobin,
    /// Leastconnections
    LeastConnections,
    /// Weightedroundrobin
    WeightedRoundRobin,
    /// Iphash
    IpHash,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthchecksettings
pub struct HealthCheckSettings {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Interval
    pub interval: Duration,
    /// Timeout
    pub timeout: Duration,
    /// Retries
    pub retries: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Backendserver
pub struct BackendServer {
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Weight
    pub weight: u32,
    /// Whether this feature is enabled
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkqossettings
pub struct NetworkQosSettings {
    /// Quality of Service enabled
    pub enabled: bool,
    /// Bandwidth limits by priority
    pub bandwidth_limits: HashMap<QosPriority, u64>,
    /// Traffic shaping rules
    pub traffic_shaping: Vec<TrafficShapingRule>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
/// Qospriority
pub enum QosPriority {
    /// High
    High,
    /// Medium
    Medium,
    /// Low
    Low,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Trafficshapingrule
pub struct TrafficShapingRule {
    /// Name
    pub name: String,
    /// Source
    pub source: Option<String>,
    /// Destination
    pub destination: Option<String>,
    /// Protocol
    pub protocol: Option<String>,
    /// Bandwidth Limit
    pub bandwidth_limit: u64,
}

// ==================== SECTION ====================

impl Default for NetworkVlanSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            vlans: HashMap::new(),
            default_vlan_id: 1,
            enable_trunking: false,
        }
    }
}

impl Default for NetworkProtocolSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled_protocols: vec![
                crate::protocol::Protocol::Http,
                crate::protocol::Protocol::Tcp,
            ],
            protocol_timeouts: HashMap::new(),
            max_protocol_handlers: 100,
        }
    }
}

impl Default for NetworkConnectionSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            keep_alive: true,
            keep_alive_timeout: Duration::from_secs(300),
        }
    }
}

impl Default for NetworkPortSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            port_ranges: vec![PortRange {
                start: 8000,
                end: 9000,
                description: "Application ports".to_string(),
            }],
            reserved_ports: vec![22, 80, 443],
            dynamic_allocation: true,
        }
    }
}

impl Default for NetworkApiSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            endpoints: HashMap::new(),
            default_timeout: Duration::from_secs(30),
            max_request_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl Default for NetworkFileSystemSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            nfs: NfsSettings {
                enabled: false,
                version: "4.1".to_string(),
                export_paths: Vec::new(),
            },
            smb: SmbSettings {
                enabled: false,
                version: "3.0".to_string(),
                share_paths: Vec::new(),
            },
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for NetworkLoadBalancingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_check: HealthCheckSettings {
                enabled: true,
                interval: Duration::from_secs(30),
                timeout: Duration::from_secs(5),
                retries: 3,
            },
            backends: Vec::new(),
        }
    }
}

impl Default for NetworkQosSettings {
    /// Returns the default instance
    fn default() -> Self {
        let mut bandwidth_limits = HashMap::new();
        bandwidth_limits.insert(QosPriority::High, 1_000_000_000); // 1Gbps
        bandwidth_limits.insert(QosPriority::Medium, 500_000_000); // 500Mbps
        bandwidth_limits.insert(QosPriority::Low, 100_000_000); // 100Mbps

        Self {
            enabled: false,
            bandwidth_limits,
            traffic_shaping: Vec::new(),
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Networkvlanconfigcanonical
pub type NetworkVlanConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkVlanConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
