//! **NETWORK TYPES AND CONFIGURATION**
//!
//! This module provides all the data structures, enums, and configuration
//! types used by the network service.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};

// Import the canonical configuration system
use nestgate_core::unified_config_consolidation::StandardDomainConfig;

// ==================== SECTION ====================

/// **MIGRATED**: Network service configuration now uses `StandardDomainConfig` pattern
/// This replaces the old fragmented `NetworkConfig` with unified configuration
pub type NetworkConfig = StandardDomainConfig<NetworkExtensions>;
/// Network-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkExtensions {
    /// Port range start for service allocation
    pub port_range_start: u16,
    /// Port range end for service allocation
    pub port_range_end: u16,
    /// Keep-alive timeout in seconds
    pub keep_alive_timeout_seconds: u64,
    /// Protocol-specific settings
    pub protocol_settings: HashMap<String, String>,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
    /// Circuit breaker settings
    pub circuit_breaker: CircuitBreakerConfig,
}
/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub algorithm: String,
    pub health_check_interval: Duration,
    pub max_failures: u32,
}
impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: "round_robin".to_string(),
            health_check_interval: Duration::from_secs(30),
            max_failures: 3,
        }
    }
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub timeout_duration: Duration,
    pub half_open_max_calls: u32,
}
impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout_duration: Duration::from_secs(60),
            half_open_max_calls: 10,
        }
    }
}

impl Default for NetworkExtensions {
    fn default() -> Self {
        Self {
            port_range_start: 9000,
            port_range_end: 9999,
            keep_alive_timeout_seconds: 60,
            protocol_settings: HashMap::new(),
            load_balancing: LoadBalancingConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
        }
    }
}

// ==================== SECTION ====================

/// Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: String,
    pub endpoint: SocketAddr,
    pub established_at: SystemTime,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub status: ConnectionStatus,
}
impl ConnectionInfo {
    /// Create a new connection info
    #[must_use]
    pub fn new(id: String, endpoint: SocketAddr) -> Self {
        Self {
            id,
            endpoint,
            established_at: SystemTime::now(),
            bytes_sent: 0,
            bytes_received: 0,
            status: ConnectionStatus::Active,
        }
    }

    /// Get connection ID
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get connection address
    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.endpoint
    }

    /// Get connection age
    #[must_use]
    pub fn age(&self) -> Duration {
        self.established_at.elapsed().unwrap_or_default()
    }

    /// Check if connection is active
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(self.status, ConnectionStatus::Active)
    }

    /// Get connection status
    #[must_use]
    pub fn status(&self) -> &ConnectionStatus {
        &self.status
    }

    /// Update bytes sent
    pub fn add_bytes_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }

    /// Update bytes received
    pub fn add_bytes_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }

    /// Set connection status
    pub fn set_status(&mut self, status: ConnectionStatus) {
        self.status = status;
    }
}

/// Connection status enumeration
#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    /// Connection is active
    Active,
    /// Connection is idle
    Idle,
    /// Connection is closing
    Closing,
    /// Connection is closed
    Closed,
}
/// Connection details for external reporting
#[derive(Debug, Clone)]
pub struct ConnectionDetails {
    pub id: String,
    pub endpoint: SocketAddr,
    pub age: Duration,
    pub is_active: bool,
    pub status: String,
}
// ==================== SECTION ====================

/// Service information
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub id: String,
    pub name: String,
    pub endpoint: SocketAddr,
    pub health_status: HealthStatus,
    pub registered_at: SystemTime,
    pub metadata: HashMap<String, String>,
}
impl ServiceInfo {
    /// Create a new service info
    #[must_use]
    pub fn new(id: String, name: String, endpoint: SocketAddr) -> Self {
        Self {
            id,
            name,
            endpoint,
            health_status: HealthStatus::Healthy,
            registered_at: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }

    /// Get service ID
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get service name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get service address
    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.endpoint
    }

    /// Get health status
    #[must_use]
    pub fn health_status(&self) -> &HealthStatus {
        &self.health_status
    }

    /// Get registration time
    #[must_use]
    pub fn registered_at(&self) -> SystemTime {
        self.registered_at
    }

    /// Get metadata
    #[must_use]
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Check if service is healthy
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        matches!(self.health_status, HealthStatus::Healthy)
    }

    /// Set health status
    pub fn set_health_status(&mut self, status: HealthStatus) {
        self.health_status = status;
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get service age
    #[must_use]
    pub fn age(&self) -> Duration {
        self.registered_at.elapsed().unwrap_or_default()
    }
}

/// Health status enumeration
#[derive(Debug, Clone)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Service status is unknown
    Unknown,
}
/// Service details for external reporting
#[derive(Debug, Clone)]
pub struct ServiceDetails {
    pub id: String,
    pub name: String,
    pub endpoint: SocketAddr,
    pub health_status: String,
    pub registered_at: SystemTime,
    pub metadata: HashMap<String, String>,
}
// ==================== SECTION ====================

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkStatistics {
    pub active_connections: u32,
    pub registered_services: u32,
    pub allocated_ports: u32,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
}

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Error,
    Unknown,
    Healthy,
    Unhealthy,
    Starting,
    Stopping,
    Failed,
}
impl Default for ServiceStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

// ==================== SECTION ====================

/// Network operation result
// Use canonical NetworkResult from nestgate_core::error
pub use nestgate_core::error::NetworkResult;
// ==================== SECTION ====================

/// Network configuration builder
pub struct NetworkConfigBuilder {
    config: NetworkConfig,
}
impl NetworkConfigBuilder {
    /// Create a new configuration builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: NetworkConfig::default(),
        }
    }

    /// Set host
    #[must_use]
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.config.network.api.bind_address = host.into().parse().unwrap_or(
            std::env::var("NESTGATE_BIND_ADDRESS")
                .unwrap_or_else(|_| {
                    std::env::var("NESTGATE_HOSTNAME")
                        .unwrap_or_else(|_| "localhost".to_string())
                        .to_string()
                })
                .parse()
                .unwrap(),
        );
        self
    }
    /// Set port
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.config.network.api.port = port;
        self
    }

    /// Set max connections
    #[must_use]
    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.config.network.api.max_connections = max_connections;
        self
    }
    /// Set connection timeout
    #[must_use]
    pub fn connection_timeout(mut self, timeout_seconds: u64) -> Self {
        self.config.network.api.connection_timeout = Duration::from_secs(timeout_seconds);
        self
    }

    /// Set port range
    #[must_use]
    pub fn port_range(mut self, start: u16, end: u16) -> Self {
        self.config.extensions.port_range_start = start;
        self.config.extensions.port_range_end = end;
        self
    }
    /// Enable/disable keep-alive
    #[must_use]
    pub fn keep_alive(self, _enabled: bool) -> Self {
        // Note: keep_alive is not a direct field in NetworkApiConfig
        // This may need to be stored elsewhere or removed
        // self.config.network.api.keep_alive = enabled;
        self
    }

    /// Set keep-alive timeout
    #[must_use]
    pub fn keep_alive_timeout(mut self, timeout_seconds: u64) -> Self {
        self.config.extensions.keep_alive_timeout_seconds = timeout_seconds;
        self
    }

    /// Build the configuration
    #[must_use]
    pub fn build(self) -> NetworkConfig {
        self.config
    }
}

impl Default for NetworkConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    // ==================== ConnectionInfo Tests (3 tests) ====================

    #[test]
    fn test_connection_info_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let conn = ConnectionInfo::new("conn-123".to_string(), addr);

        assert_eq!(conn.id(), "conn-123");
        assert_eq!(conn.address(), addr);
        assert!(conn.is_active());
        assert_eq!(conn.bytes_sent, 0);
        assert_eq!(conn.bytes_received, 0);
    }

    #[test]
    fn test_connection_info_byte_tracking() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut conn = ConnectionInfo::new("conn-123".to_string(), addr);

        conn.add_bytes_sent(1024);
        conn.add_bytes_received(2048);

        assert_eq!(conn.bytes_sent, 1024);
        assert_eq!(conn.bytes_received, 2048);

        // Add more bytes
        conn.add_bytes_sent(512);
        conn.add_bytes_received(1024);

        assert_eq!(conn.bytes_sent, 1536);
        assert_eq!(conn.bytes_received, 3072);
    }

    #[test]
    fn test_connection_status_transitions() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut conn = ConnectionInfo::new("conn-123".to_string(), addr);

        assert!(conn.is_active());

        conn.set_status(ConnectionStatus::Idle);
        assert!(!conn.is_active());

        conn.set_status(ConnectionStatus::Closing);
        assert!(!conn.is_active());

        conn.set_status(ConnectionStatus::Closed);
        assert!(!conn.is_active());
    }

    // ==================== ServiceInfo Tests (3 tests) ====================

    #[test]
    fn test_service_info_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
        let service = ServiceInfo::new("svc-456".to_string(), "test-service".to_string(), addr);

        assert_eq!(service.id(), "svc-456");
        assert_eq!(service.name(), "test-service");
        assert_eq!(service.address(), addr);
        assert!(service.is_healthy());
        assert_eq!(service.metadata().len(), 0);
    }

    #[test]
    fn test_service_info_metadata() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
        let mut service = ServiceInfo::new("svc-456".to_string(), "test-service".to_string(), addr);

        service.add_metadata("version".to_string(), "1.0.0".to_string());
        service.add_metadata("region".to_string(), "us-west".to_string());

        assert_eq!(service.metadata().len(), 2);
        assert_eq!(
            service.metadata().get("version"),
            Some(&"1.0.0".to_string())
        );
        assert_eq!(
            service.metadata().get("region"),
            Some(&"us-west".to_string())
        );
    }

    #[test]
    fn test_service_health_status() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
        let mut service = ServiceInfo::new("svc-456".to_string(), "test-service".to_string(), addr);

        assert!(service.is_healthy());

        service.set_health_status(HealthStatus::Degraded);
        assert!(!service.is_healthy());

        service.set_health_status(HealthStatus::Unhealthy);
        assert!(!service.is_healthy());

        service.set_health_status(HealthStatus::Healthy);
        assert!(service.is_healthy());
    }

    // ==================== NetworkConfig Builder Tests (3 tests) ====================

    #[test]
    fn test_network_config_builder_basic() {
        let config = NetworkConfigBuilder::new()
            .port(8888)
            .max_connections(100)
            .build();

        assert_eq!(config.network.api.port, 8888);
        assert_eq!(config.network.api.max_connections, 100);
    }

    #[test]
    fn test_network_config_builder_port_range() {
        let config = NetworkConfigBuilder::new().port_range(10000, 20000).build();

        assert_eq!(config.extensions.port_range_start, 10000);
        assert_eq!(config.extensions.port_range_end, 20000);
    }

    #[test]
    fn test_network_config_builder_timeouts() {
        let config = NetworkConfigBuilder::new()
            .connection_timeout(120)
            .keep_alive_timeout(300)
            .build();

        assert_eq!(
            config.network.api.connection_timeout,
            Duration::from_secs(120)
        );
        assert_eq!(config.extensions.keep_alive_timeout_seconds, 300);
    }

    // ==================== LoadBalancing & CircuitBreaker Tests (3 tests) ====================

    #[test]
    fn test_load_balancing_config_default() {
        let config = LoadBalancingConfig::default();

        assert_eq!(config.algorithm, "round_robin");
        assert_eq!(config.health_check_interval, Duration::from_secs(30));
        assert_eq!(config.max_failures, 3);
    }

    #[test]
    fn test_circuit_breaker_config_default() {
        let config = CircuitBreakerConfig::default();

        assert_eq!(config.failure_threshold, 5);
        assert_eq!(config.timeout_duration, Duration::from_secs(60));
        assert_eq!(config.half_open_max_calls, 10);
    }

    #[test]
    fn test_network_extensions_default() {
        let extensions = NetworkExtensions::default();

        assert_eq!(extensions.port_range_start, 9000);
        assert_eq!(extensions.port_range_end, 9999);
        assert_eq!(extensions.keep_alive_timeout_seconds, 60);
        assert_eq!(extensions.protocol_settings.len(), 0);
    }

    // ==================== Network Statistics Tests (3 tests) ====================

    #[test]
    fn test_network_statistics_default() {
        let stats = NetworkStatistics::default();

        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.registered_services, 0);
        assert_eq!(stats.allocated_ports, 0);
        assert_eq!(stats.total_bytes_sent, 0);
        assert_eq!(stats.total_bytes_received, 0);
    }

    #[test]
    fn test_network_statistics_serialization() {
        let stats = NetworkStatistics {
            active_connections: 10,
            registered_services: 5,
            allocated_ports: 20,
            total_bytes_sent: 1024,
            total_bytes_received: 2048,
        };

        let serialized = serde_json::to_string(&stats);
        assert!(serialized.is_ok(), "Network statistics should serialize");

        let json = serialized.unwrap();
        let deserialized: std::result::Result<NetworkStatistics, _> = serde_json::from_str(&json);
        assert!(
            deserialized.is_ok(),
            "Network statistics should deserialize"
        );

        let deserialized_stats = deserialized.unwrap();
        assert_eq!(deserialized_stats.active_connections, 10);
        assert_eq!(deserialized_stats.registered_services, 5);
    }

    #[test]
    fn test_service_status_variants() {
        let statuses = [
            ServiceStatus::Running,
            ServiceStatus::Stopped,
            ServiceStatus::Error,
            ServiceStatus::Unknown,
            ServiceStatus::Healthy,
            ServiceStatus::Unhealthy,
            ServiceStatus::Starting,
            ServiceStatus::Stopping,
            ServiceStatus::Failed,
        ];

        assert_eq!(statuses.len(), 9);
        assert_eq!(ServiceStatus::default(), ServiceStatus::Unknown);

        // Test equality
        assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
        assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
    }
}
