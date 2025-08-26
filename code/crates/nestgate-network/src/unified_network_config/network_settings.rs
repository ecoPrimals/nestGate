/// Contains all individual network configuration setting structs.
/// Extracted from the large unified_network_config.rs to achieve file size compliance.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

// ==================== NETWORK SETTINGS STRUCTS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkVlanSettings {
    /// Enable VLAN support
    pub enabled: bool,
    /// VLAN configurations by VLAN ID
    pub vlans: HashMap<u16, NetworkVlanConfig>,
    /// Default VLAN ID for untagged traffic
    pub default_vlan_id: u16,
    /// Enable VLAN trunking
    pub enable_trunking: bool,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkVlanConfig {
    pub vlan_id: u16,
    pub name: String,
    pub description: Option<String>,
    pub network: String, // CIDR notation
    pub gateway: Option<IpAddr>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProtocolSettings {
    /// Enabled protocols
    pub enabled_protocols: Vec<crate::protocol::Protocol>,
    /// Protocol-specific timeouts
    pub protocol_timeouts: HashMap<String, Duration>,
    /// Maximum concurrent protocol handlers
    pub max_protocol_handlers: u32,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct NetworkPortSettings {
    /// Port allocation ranges
    pub port_ranges: Vec<PortRange>,
    /// Reserved ports
    pub reserved_ports: Vec<u16>,
    /// Dynamic port allocation enabled
    pub dynamic_allocation: bool,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
    pub description: String,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkApiSettings {
    /// API endpoints configuration
    pub endpoints: HashMap<String, ApiEndpoint>,
    /// Default API timeout
    pub default_timeout: Duration,
    /// Maximum request size
    pub max_request_size: u64,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub address: SocketAddr,
    pub path: String,
    pub enabled: bool,
    pub timeout: Option<Duration>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkFileSystemSettings {
    /// NFS settings
    pub nfs: NfsSettings,
    /// SMB settings
    pub smb: SmbSettings,
    /// File system timeout
    pub timeout: Duration,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsSettings {
    pub enabled: bool,
    pub version: String,
    pub export_paths: Vec<String>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbSettings {
    pub enabled: bool,
    pub version: String,
    pub share_paths: Vec<String>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLoadBalancingSettings {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check settings
    pub health_check: HealthCheckSettings,
    /// Backend servers
    pub backends: Vec<BackendServer>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckSettings {
    pub enabled: bool,
    pub interval: Duration,
    pub timeout: Duration,
    pub retries: u32,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendServer {
    pub address: SocketAddr,
    pub weight: u32,
    pub enabled: bool,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkQosSettings {
    /// Quality of Service enabled
    pub enabled: bool,
    /// Bandwidth limits by priority
    pub bandwidth_limits: HashMap<QosPriority, u64>,
    /// Traffic shaping rules
    pub traffic_shaping: Vec<TrafficShapingRule>,
    }

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum QosPriority {
    High,
    Medium,
    Low,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficShapingRule {
    pub name: String,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub protocol: Option<String>,
    pub bandwidth_limit: u64,
    }

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for NetworkVlanSettings {
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
    fn default() -> Self {
        Self {
            enabled_protocols: vec![crate::protocol::Protocol::Http, crate::protocol::Protocol::Tcp],
            protocol_timeouts: HashMap::new(),
            max_protocol_handlers: 100,
    }
    }
    }

impl Default for NetworkConnectionSettings {
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
    fn default() -> Self {
        Self {
            port_ranges: vec![
                PortRange {
                    start: 8000,
                    end: 9000,
                    description: "Application ports".to_string(),
    }
            ],
            reserved_ports: vec![22, 80, 443],
            dynamic_allocation: true,
    }
    }
    }

impl Default for NetworkApiSettings {
    fn default() -> Self {
        Self {
            endpoints: HashMap::new(),
            default_timeout: Duration::from_secs(30),
            max_request_size: 10 * 1024 * 1024, // 10MB
    }
    }
    }

impl Default for NetworkFileSystemSettings {
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
    fn default() -> Self {
        let mut bandwidth_limits = HashMap::new();
        bandwidth_limits.insert(QosPriority::High, 1_000_000_000); // 1Gbps
        bandwidth_limits.insert(QosPriority::Medium, 500_000_000); // 500Mbps
        bandwidth_limits.insert(QosPriority::Low, 100_000_000);    // 100Mbps

        Self {
            enabled: false,
            bandwidth_limits,
            traffic_shaping: Vec::new(),
    }
    }
} 