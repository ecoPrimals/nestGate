/// **UNIFIED NETWORK CONFIGURATION EXTENSIONS**
/// Consolidates all fragmented network configuration structs into the StandardDomainConfig pattern.
/// 
/// **ELIMINATES**:
/// - NetworkConfig (lib.rs)
/// - OrchestrationConfig (orchestration_adapter.rs)
/// - UniversalOrchestrationConfig (universal_orchestration.rs)
/// - ProtocolConfig (protocol.rs, protocols.rs)
/// - VlanConfig (vlan.rs)
/// - NetworkVlanConfig (unified_network_config/network_settings.rs)
/// - SongbirdConfig (legacy compatibility)
/// 
/// **PROVIDES**:
/// - Single source of truth for all network configuration
/// - Consistent configuration patterns with base unified configs
/// - Extensible architecture for network-specific settings

use nestgate_core::unified_config_consolidation::StandardDomainConfig;
use nestgate_core::smart_abstractions::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

/// **UNIFIED NETWORK EXTENSIONS**
/// Consolidates all network-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedNetworkExtensions {
    /// Orchestration and service discovery settings
    pub orchestration: NetworkOrchestrationSettings,
    /// Protocol configuration settings
    pub protocols: NetworkProtocolSettings,
    /// VLAN and network segmentation settings
    pub vlan: NetworkVlanSettings,
    /// Connection management settings
    pub connections: NetworkConnectionSettings,
    /// Load balancing and routing settings
    pub routing: NetworkRoutingSettings,
    /// Quality of Service settings
    pub qos: NetworkQosSettings,
    /// Network security settings
    pub security: NetworkSecuritySettings,
}

/// Network orchestration and service discovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOrchestrationSettings {
    /// Enable universal orchestration discovery
    pub enable_orchestration: bool,
    /// Orchestration discovery timeout
    pub orchestration_timeout: Duration,
    /// Service discovery interval
    pub discovery_interval: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Maximum concurrent orchestration connections
    pub max_orchestration_connections: u32,
    /// Orchestration retry configuration
    pub retry_config: OrchestrationRetryConfig,
    /// Service registration settings
    pub service_registration: ServiceRegistrationSettings,
}

/// Orchestration retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationRetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Retry multiplier
    pub multiplier: f64,
    /// Enable exponential backoff
    pub exponential_backoff: bool,
}

/// Service registration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationSettings {
    /// Auto-register services
    pub auto_register: bool,
    /// Service TTL
    pub service_ttl: Duration,
    /// Registration retry attempts
    pub registration_retries: u32,
    /// Service metadata
    pub service_metadata: HashMap<String, String>,
}

/// Network protocol configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProtocolSettings {
    /// TCP protocol settings
    pub tcp: TcpProtocolSettings,
    /// UDP protocol settings
    pub udp: UdpProtocolSettings,
    /// HTTP protocol settings
    pub http: HttpProtocolSettings,
    /// WebSocket protocol settings
    pub websocket: WebSocketProtocolSettings,
    /// Custom protocol settings
    pub custom_protocols: HashMap<String, CustomProtocolSettings>,
}

/// TCP protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpProtocolSettings {
    /// Enable TCP keep-alive
    pub keep_alive: bool,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// TCP no-delay option
    pub no_delay: bool,
    /// Socket buffer sizes
    pub buffer_sizes: SocketBufferSettings,
    /// Connection timeout
    pub connection_timeout: Duration,
}

/// UDP protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpProtocolSettings {
    /// Enable UDP broadcast
    pub enable_broadcast: bool,
    /// Enable UDP multicast
    pub enable_multicast: bool,
    /// Multicast groups
    pub multicast_groups: Vec<IpAddr>,
    /// Socket buffer sizes
    pub buffer_sizes: SocketBufferSettings,
}

/// HTTP protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpProtocolSettings {
    /// HTTP version (1.1, 2.0)
    pub version: String,
    /// Request timeout
    pub request_timeout: Duration,
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Connection pool settings
    pub connection_pool: ConnectionPoolSettings,
    /// Compression settings
    pub compression: CompressionSettings,
}

/// WebSocket protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketProtocolSettings {
    /// WebSocket ping interval
    pub ping_interval: Duration,
    /// WebSocket pong timeout
    pub pong_timeout: Duration,
    /// Maximum message size
    pub max_message_size: usize,
    /// Maximum frame size
    pub max_frame_size: usize,
    /// Enable compression
    pub compression_enabled: bool,
}

/// Custom protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProtocolSettings {
    /// Protocol name
    pub name: String,
    /// Protocol version
    pub version: String,
    /// Protocol configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Protocol enabled
    pub enabled: bool,
}

/// Socket buffer settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketBufferSettings {
    /// Send buffer size
    pub send_buffer_size: usize,
    /// Receive buffer size
    pub receive_buffer_size: usize,
}

/// Connection pool settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolSettings {
    /// Minimum connections
    pub min_connections: u32,
    /// Maximum connections
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
}

/// Compression settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionSettings {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: String,
    /// Compression level
    pub level: u8,
    /// Minimum size for compression
    pub min_size: usize,
}

/// Network VLAN and segmentation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkVlanSettings {
    /// Enable VLAN support
    pub enabled: bool,
    /// Default VLAN ID
    pub default_vlan_id: u16,
    /// VLAN configurations
    pub vlans: HashMap<u16, VlanConfiguration>,
    /// VLAN tagging settings
    pub tagging: VlanTaggingSettings,
}

/// VLAN configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlanConfiguration {
    /// VLAN ID
    pub vlan_id: u16,
    /// VLAN name
    pub name: String,
    /// VLAN subnet
    pub subnet: String,
    /// VLAN gateway
    pub gateway: Option<IpAddr>,
    /// VLAN enabled
    pub enabled: bool,
}

/// VLAN tagging settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VlanTaggingSettings {
    /// Enable VLAN tagging
    pub enabled: bool,
    /// Default tag behavior
    pub default_tag_behavior: String,
    /// Untagged VLAN
    pub untagged_vlan: Option<u16>,
}

/// Network connection management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnectionSettings {
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Keep-alive settings
    pub keep_alive: KeepAliveSettings,
    /// Connection retry settings
    pub retry_settings: ConnectionRetrySettings,
    /// Connection pooling settings
    pub pooling: ConnectionPoolingSettings,
}

/// Keep-alive settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeepAliveSettings {
    /// Enable keep-alive
    pub enabled: bool,
    /// Keep-alive interval
    pub interval: Duration,
    /// Keep-alive timeout
    pub timeout: Duration,
    /// Maximum keep-alive probes
    pub max_probes: u32,
}

/// Connection retry settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRetrySettings {
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Retry multiplier
    pub multiplier: f64,
}

/// Connection pooling settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolingSettings {
    /// Enable connection pooling
    pub enabled: bool,
    /// Pool size
    pub pool_size: u32,
    /// Pool timeout
    pub pool_timeout: Duration,
    /// Pool cleanup interval
    pub cleanup_interval: Duration,
}

/// Network routing and load balancing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRoutingSettings {
    /// Load balancing algorithm
    pub load_balancing_algorithm: String,
    /// Routing table settings
    pub routing_table: RoutingTableSettings,
    /// Failover settings
    pub failover: RoutingFailoverSettings,
    /// Health check settings
    pub health_checks: RoutingHealthCheckSettings,
}

/// Routing table settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingTableSettings {
    /// Enable dynamic routing
    pub dynamic_routing: bool,
    /// Static routes
    pub static_routes: HashMap<String, String>,
    /// Route priority
    pub route_priority: u32,
    /// Route timeout
    pub route_timeout: Duration,
}

/// Routing failover settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingFailoverSettings {
    /// Enable failover
    pub enabled: bool,
    /// Failover threshold
    pub failure_threshold: u32,
    /// Failover timeout
    pub failover_timeout: Duration,
    /// Recovery threshold
    pub recovery_threshold: u32,
}

/// Routing health check settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingHealthCheckSettings {
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Health check endpoint
    pub endpoint: String,
    /// Expected response codes
    pub expected_response_codes: Vec<u16>,
}

/// Network Quality of Service settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkQosSettings {
    /// Enable QoS
    pub enabled: bool,
    /// Traffic shaping settings
    pub traffic_shaping: TrafficShapingSettings,
    /// Bandwidth limits
    pub bandwidth_limits: BandwidthLimitSettings,
    /// Priority settings
    pub priority: QosPrioritySettings,
}

/// Traffic shaping settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficShapingSettings {
    /// Enable traffic shaping
    pub enabled: bool,
    /// Shaping algorithm
    pub algorithm: String,
    /// Burst size
    pub burst_size: usize,
    /// Rate limit
    pub rate_limit: u64,
}

/// Bandwidth limit settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthLimitSettings {
    /// Upload limit (bytes per second)
    pub upload_limit: Option<u64>,
    /// Download limit (bytes per second)
    pub download_limit: Option<u64>,
    /// Per-connection limits
    pub per_connection_limits: bool,
}

/// QoS priority settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QosPrioritySettings {
    /// Default priority
    pub default_priority: u8,
    /// Priority classes
    pub priority_classes: HashMap<String, u8>,
    /// Priority scheduling algorithm
    pub scheduling_algorithm: String,
}

/// Network security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecuritySettings {
    /// Firewall settings
    pub firewall: FirewallSettings,
    /// Rate limiting settings
    pub rate_limiting: RateLimitingSettings,
    /// DDoS protection settings
    pub ddos_protection: DdosProtectionSettings,
    /// Network access control
    pub access_control: NetworkAccessControlSettings,
}

/// Firewall settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallSettings {
    /// Enable firewall
    pub enabled: bool,
    /// Default policy (allow, deny)
    pub default_policy: String,
    /// Firewall rules
    pub rules: Vec<FirewallRule>,
    /// Log blocked connections
    pub log_blocked: bool,
}

/// Firewall rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    /// Rule name
    pub name: String,
    /// Source IP/CIDR
    pub source: Option<String>,
    /// Destination IP/CIDR
    pub destination: Option<String>,
    /// Port range
    pub ports: Option<String>,
    /// Protocol (tcp, udp, icmp)
    pub protocol: Option<String>,
    /// Action (allow, deny)
    pub action: String,
    /// Rule enabled
    pub enabled: bool,
}

/// Rate limiting settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingSettings {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per second limit
    pub requests_per_second: u32,
    /// Burst size
    pub burst_size: u32,
    /// Rate limiting algorithm
    pub algorithm: String,
}

/// DDoS protection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DdosProtectionSettings {
    /// Enable DDoS protection
    pub enabled: bool,
    /// Connection threshold
    pub connection_threshold: u32,
    /// Request threshold
    pub request_threshold: u32,
    /// Block duration
    pub block_duration: Duration,
    /// Whitelist IPs
    pub whitelist: Vec<IpAddr>,
}

/// Network access control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAccessControlSettings {
    /// Enable access control
    pub enabled: bool,
    /// Allowed IP ranges
    pub allowed_ip_ranges: Vec<String>,
    /// Blocked IP ranges
    pub blocked_ip_ranges: Vec<String>,
    /// Geo-blocking settings
    pub geo_blocking: GeoBlockingSettings,
}

/// Geo-blocking settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoBlockingSettings {
    /// Enable geo-blocking
    pub enabled: bool,
    /// Allowed countries
    pub allowed_countries: Vec<String>,
    /// Blocked countries
    pub blocked_countries: Vec<String>,
    /// GeoIP database path
    pub geoip_database: Option<String>,
}

/// **UNIFIED NETWORK CONFIGURATION**
/// The single source of truth for all network configuration across the system
pub type UnifiedNetworkConfig = StandardDomainConfig<UnifiedNetworkExtensions>;

impl UnifiedNetworkConfig {
    /// Create development configuration optimized for local development
    pub fn development() -> Self {
        Self::create_for_environment("development")
    }

    /// Create production configuration optimized for high-load production
    pub fn production() -> Self {
        Self::create_for_environment("production")
    }

    /// Create configuration for specific network workload
    pub fn for_workload(workload: &str) -> Self {
        Self::create_for_workload(workload)
    }
}

// Default implementations for all settings structs
impl SmartDefault for UnifiedNetworkExtensions {
    fn smart_default() -> Self {
        Self {
            orchestration: NetworkOrchestrationSettings::smart_default(),
            protocols: NetworkProtocolSettings::smart_default(),
            vlan: NetworkVlanSettings::smart_default(),
            connections: NetworkConnectionSettings::smart_default(),
            routing: NetworkRoutingSettings::smart_default(),
            qos: NetworkQosSettings::smart_default(),
            security: NetworkSecuritySettings::smart_default(),
        }
    }
}

impl Default for UnifiedNetworkExtensions {
    fn default() -> Self {
        Self::smart_default()
    }
}

impl SmartDefault for NetworkOrchestrationSettings {
    fn smart_default() -> Self {
        Self {
            enable_orchestration: true,
            orchestration_timeout: Duration::from_secs(10),
            discovery_interval: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(10),
            max_orchestration_connections: 100,
            retry_config: OrchestrationRetryConfig::smart_default(),
            service_registration: ServiceRegistrationSettings::smart_default(),
        }
    }
}

impl Default for NetworkOrchestrationSettings {
    fn default() -> Self {
        Self::smart_default()
    }
}

impl Default for OrchestrationRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            exponential_backoff: true,
        }
    }
}

impl Default for ServiceRegistrationSettings {
    fn default() -> Self {
        Self {
            auto_register: true,
            service_ttl: Duration::from_secs(60),
            registration_retries: 3,
            service_metadata: HashMap::new(),
        }
    }
}

impl Default for NetworkProtocolSettings {
    fn default() -> Self {
        Self {
            tcp: TcpProtocolSettings::default(),
            udp: UdpProtocolSettings::default(),
            http: HttpProtocolSettings::default(),
            websocket: WebSocketProtocolSettings::default(),
            custom_protocols: HashMap::new(),
        }
    }
}

impl Default for TcpProtocolSettings {
    fn default() -> Self {
        Self {
            keep_alive: true,
            keep_alive_timeout: Duration::from_secs(30),
            no_delay: true,
            buffer_sizes: SocketBufferSettings::default(),
            connection_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for UdpProtocolSettings {
    fn default() -> Self {
        Self {
            enable_broadcast: false,
            enable_multicast: false,
            multicast_groups: vec![],
            buffer_sizes: SocketBufferSettings::default(),
        }
    }
}

impl Default for HttpProtocolSettings {
    fn default() -> Self {
        Self {
            version: "1.1".to_string(),
            request_timeout: Duration::from_secs(30),
            max_connections: 1000,
            connection_pool: ConnectionPoolSettings::default(),
            compression: CompressionSettings::default(),
        }
    }
}

impl Default for WebSocketProtocolSettings {
    fn default() -> Self {
        Self {
            ping_interval: Duration::from_secs(30),
            pong_timeout: Duration::from_secs(10),
            max_message_size: 64 * 1024 * 1024, // 64MB
            max_frame_size: 16 * 1024 * 1024,   // 16MB
            compression_enabled: true,
        }
    }
}

impl Default for CustomProtocolSettings {
    fn default() -> Self {
        Self {
            name: "custom".to_string(),
            version: "1.0".to_string(),
            config: HashMap::new(),
            enabled: false,
        }
    }
}

impl Default for SocketBufferSettings {
    fn default() -> Self {
        Self {
            send_buffer_size: 65536,    // 64KB
            receive_buffer_size: 65536, // 64KB
        }
    }
}

impl Default for ConnectionPoolSettings {
    fn default() -> Self {
        Self {
            min_connections: 1,
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
        }
    }
}

impl Default for CompressionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: "gzip".to_string(),
            level: 6,
            min_size: 1024, // 1KB
        }
    }
}

impl Default for NetworkVlanSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            default_vlan_id: 1,
            vlans: HashMap::new(),
            tagging: VlanTaggingSettings::default(),
        }
    }
}

impl Default for VlanConfiguration {
    fn default() -> Self {
        Self {
            vlan_id: 1,
            name: "default".to_string(),
            subnet: "192.168.1.0/24".to_string(),
            gateway: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
            enabled: true,
        }
    }
}

impl Default for VlanTaggingSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            default_tag_behavior: "untagged".to_string(),
            untagged_vlan: Some(1),
        }
    }
}

impl Default for NetworkConnectionSettings {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            keep_alive: KeepAliveSettings::default(),
            retry_settings: ConnectionRetrySettings::default(),
            pooling: ConnectionPoolingSettings::default(),
        }
    }
}

impl Default for KeepAliveSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
            max_probes: 3,
        }
    }
}

impl Default for ConnectionRetrySettings {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

impl Default for ConnectionPoolingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            pool_size: 10,
            pool_timeout: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(300),
        }
    }
}

impl Default for NetworkRoutingSettings {
    fn default() -> Self {
        Self {
            load_balancing_algorithm: "round_robin".to_string(),
            routing_table: RoutingTableSettings::default(),
            failover: RoutingFailoverSettings::default(),
            health_checks: RoutingHealthCheckSettings::default(),
        }
    }
}

impl Default for RoutingTableSettings {
    fn default() -> Self {
        Self {
            dynamic_routing: true,
            static_routes: HashMap::new(),
            route_priority: 100,
            route_timeout: Duration::from_secs(300),
        }
    }
}

impl Default for RoutingFailoverSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 3,
            failover_timeout: Duration::from_secs(10),
            recovery_threshold: 2,
        }
    }
}

impl Default for RoutingHealthCheckSettings {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            endpoint: "/health".to_string(),
            expected_response_codes: vec![200],
        }
    }
}

impl Default for NetworkQosSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            traffic_shaping: TrafficShapingSettings::default(),
            bandwidth_limits: BandwidthLimitSettings::default(),
            priority: QosPrioritySettings::default(),
        }
    }
}

impl Default for TrafficShapingSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "token_bucket".to_string(),
            burst_size: 1024 * 1024, // 1MB
            rate_limit: 1024 * 1024,  // 1MB/s
        }
    }
}

impl Default for BandwidthLimitSettings {
    fn default() -> Self {
        Self {
            upload_limit: None,
            download_limit: None,
            per_connection_limits: false,
        }
    }
}

impl Default for QosPrioritySettings {
    fn default() -> Self {
        Self {
            default_priority: 5,
            priority_classes: HashMap::new(),
            scheduling_algorithm: "fair_queuing".to_string(),
        }
    }
}

impl Default for NetworkSecuritySettings {
    fn default() -> Self {
        Self {
            firewall: FirewallSettings::default(),
            rate_limiting: RateLimitingSettings::default(),
            ddos_protection: DdosProtectionSettings::default(),
            access_control: NetworkAccessControlSettings::default(),
        }
    }
}

impl Default for FirewallSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            default_policy: "allow".to_string(),
            rules: vec![],
            log_blocked: false,
        }
    }
}

impl Default for FirewallRule {
    fn default() -> Self {
        Self {
            name: "default_rule".to_string(),
            source: None,
            destination: None,
            ports: None,
            protocol: None,
            action: "allow".to_string(),
            enabled: true,
        }
    }
}

impl Default for RateLimitingSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_second: 100,
            burst_size: 10,
            algorithm: "token_bucket".to_string(),
        }
    }
}

impl Default for DdosProtectionSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            connection_threshold: 1000,
            request_threshold: 10000,
            block_duration: Duration::from_secs(300),
            whitelist: vec![],
        }
    }
}

impl Default for NetworkAccessControlSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_ip_ranges: vec!["0.0.0.0/0".to_string()],
            blocked_ip_ranges: vec![],
            geo_blocking: GeoBlockingSettings::default(),
        }
    }
}

impl Default for GeoBlockingSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_countries: vec![],
            blocked_countries: vec![],
            geoip_database: None,
        }
    }
} 