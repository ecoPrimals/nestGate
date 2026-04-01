// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **NETWORK TYPES AND CONFIGURATION**
//!
//! This module provides all the data structures, enums, and configuration
//! types used by the network service.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};

// Import the canonical configuration system
use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// ==================== SECTION ====================

/// **CANONICAL**: Network service configuration using `canonical_primary`
/// This is the unified `NetworkConfig` for the entire ecosystem
pub type NetworkConfig = CanonicalNetworkConfig;
/// Network-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
///
/// **ECOSYSTEM SOVEREIGNTY**: Load balancing and circuit breaking are delegated
/// to networking capabilities discovered at runtime. `NestGate` does NOT hardcode
/// any specific networking primal - it discovers "networking" capability providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkextensions
pub struct NetworkExtensions {
    /// Port range start for service allocation
    pub port_range_start: u16,
    /// Port range end for service allocation
    pub port_range_end: u16,
    /// Keep-alive timeout in seconds
    pub keep_alive_timeout_seconds: u64,
    /// Protocol-specific settings
    pub protocol_settings: HashMap<String, String>,
    // REMOVED: load_balancing and circuit_breaker configs
    // These are now discovered via capability system:
    // - Discover capability: "networking" or "load-balancing"
    // - Provider implements these features
    // - No hardcoded primal names or configuration
}

impl Default for NetworkExtensions {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            port_range_start: 9000,
            port_range_end: 9999,
            keep_alive_timeout_seconds: 60,
            protocol_settings: HashMap::new(),
        }
    }
}

// ==================== SECTION ====================

/// Connection information
#[derive(Debug, Clone)]
/// Connectioninfo
pub struct ConnectionInfo {
    /// Unique identifier
    pub id: String,
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Established At
    pub established_at: SystemTime,
    /// Bytes Sent
    pub bytes_sent: u64,
    /// Bytes Received
    pub bytes_received: u64,
    /// Status
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
    pub const fn address(&self) -> SocketAddr {
        self.endpoint
    }

    /// Get connection age
    #[must_use]
    pub fn age(&self) -> Duration {
        self.established_at.elapsed().unwrap_or_default()
    }

    /// Check if connection is active
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(self.status, ConnectionStatus::Active)
    }

    /// Get connection status
    #[must_use]
    pub const fn status(&self) -> &ConnectionStatus {
        &self.status
    }

    /// Update bytes sent
    pub const fn add_bytes_sent(&mut self, bytes: u64) {
        self.bytes_sent += bytes;
    }

    /// Update bytes received
    pub const fn add_bytes_received(&mut self, bytes: u64) {
        self.bytes_received += bytes;
    }

    /// Set connection status
    pub const fn set_status(&mut self, status: ConnectionStatus) {
        self.status = status;
    }
}

/// Connection status enumeration
#[derive(Debug, Clone)]
/// Status values for Connection
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
/// Connectiondetails
pub struct ConnectionDetails {
    /// Unique identifier
    pub id: String,
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Age
    pub age: Duration,
    /// Whether active
    pub is_active: bool,
    /// Status
    pub status: String,
}
// ==================== SECTION ====================

/// Service information
#[derive(Debug, Clone)]
/// Serviceinfo
pub struct ServiceInfo {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Health Status
    pub health_status: HealthStatus,
    /// Registered At
    pub registered_at: SystemTime,
    /// Additional metadata key-value pairs
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
    pub const fn address(&self) -> SocketAddr {
        self.endpoint
    }

    /// Get health status
    #[must_use]
    pub const fn health_status(&self) -> &HealthStatus {
        &self.health_status
    }

    /// Get registration time
    #[must_use]
    pub const fn registered_at(&self) -> SystemTime {
        self.registered_at
    }

    /// Get metadata
    #[must_use]
    pub const fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Check if service is healthy
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        matches!(self.health_status, HealthStatus::Healthy)
    }

    /// Set health status
    pub const fn set_health_status(&mut self, status: HealthStatus) {
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
/// Status values for Health
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
/// Servicedetails
pub struct ServiceDetails {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Health Status
    pub health_status: String,
    /// Registered At
    pub registered_at: SystemTime,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
// ==================== SECTION ====================

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Networkstatistics
pub struct NetworkStatistics {
    /// Active Connections
    pub active_connections: u32,
    /// Registered Services
    pub registered_services: u32,
    /// Allocated Ports
    pub allocated_ports: u32,
    /// Total Bytes Sent
    pub total_bytes_sent: u64,
    /// Total Bytes Received
    pub total_bytes_received: u64,
}

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for Service
pub enum ServiceStatus {
    /// Running
    Running,
    /// Stopped
    Stopped,
    /// Error
    Error,
    /// Unknown
    Unknown,
    /// Healthy
    Healthy,
    /// Unhealthy
    Unhealthy,
    /// Starting
    Starting,
    /// Stopping
    Stopping,
    /// Failed
    Failed,
}
impl Default for ServiceStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

// ==================== SECTION ====================

/// Network operation result
// Re-export from local error module
pub use crate::error::NetworkResult;
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
        use nestgate_core::constants::hardcoding::addresses;
        use std::net::IpAddr;

        // Safe: 127.0.0.1 is always a valid IP address (IPv4 localhost)
        const DEFAULT_LOCALHOST: IpAddr = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);

        // Development default when host parsing fails and bind env vars are unset.
        let localhost_addr = addresses::LOCALHOST_NAME
            .parse()
            .unwrap_or(DEFAULT_LOCALHOST);

        self.config.api.bind_address = host.into().parse().unwrap_or_else(|_| {
            std::env::var("NESTGATE_BIND_ADDRESS")
                .ok()
                .and_then(|s| s.parse().ok())
                .or_else(|| {
                    std::env::var("NESTGATE_HOSTNAME")
                        .ok()
                        .and_then(|s| s.parse().ok())
                })
                .unwrap_or(localhost_addr)
        });
        self
    }
    /// Set port
    #[must_use]
    pub const fn port(mut self, port: u16) -> Self {
        self.config.api.port = port;
        self
    }

    /// Set max connections
    #[must_use]
    pub const fn max_connections(mut self, max_connections: u32) -> Self {
        self.config.api.max_connections = max_connections;
        self
    }
    /// Set connection timeout
    #[must_use]
    pub const fn connection_timeout(mut self, timeout_seconds: u64) -> Self {
        self.config.api.connection_timeout = Duration::from_secs(timeout_seconds);
        self
    }

    /// Set port range
    #[must_use]
    pub const fn port_range(mut self, start: u16, end: u16) -> Self {
        self.config.api.port_range_start = start;
        self.config.api.port_range_end = end;
        self
    }
    /// Enable/disable keep-alive
    #[must_use]
    pub const fn keep_alive(self, _enabled: bool) -> Self {
        // Note: keep_alive is not a direct field in NetworkApiConfig
        // This may need to be stored elsewhere or removed
        // self.config.network.api.keep_alive = enabled;
        self
    }

    /// Set keep-alive timeout
    #[must_use]
    pub const fn keep_alive_timeout(mut self, timeout_seconds: u64) -> Self {
        self.config.performance.keep_alive_timeout_seconds = timeout_seconds;
        self
    }

    /// Build the configuration
    #[must_use]
    pub fn build(self) -> NetworkConfig {
        self.config
    }
}

impl Default for NetworkConfigBuilder {
    /// Returns the default instance
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
        use nestgate_core::constants::hardcoding::runtime_fallback_ports;
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            runtime_fallback_ports::HTTP,
        );
        let conn = ConnectionInfo::new("conn-123".to_string(), addr);

        assert_eq!(conn.id(), "conn-123");
        assert_eq!(conn.address(), addr);
        assert!(conn.is_active());
        assert_eq!(conn.bytes_sent, 0);
        assert_eq!(conn.bytes_received, 0);
    }

    #[test]
    fn test_connection_info_byte_tracking() {
        use nestgate_core::constants::hardcoding::runtime_fallback_ports;
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            runtime_fallback_ports::HTTP,
        );
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
        use nestgate_core::constants::hardcoding::runtime_fallback_ports;
        let addr = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            runtime_fallback_ports::HTTP,
        );
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

        assert_eq!(config.api.port, 8888);
        assert_eq!(config.api.max_connections, 100);
    }

    #[test]
    fn test_network_config_builder_port_range() {
        let config = NetworkConfigBuilder::new().port_range(10000, 20000).build();

        assert_eq!(config.api.port_range_start, 10000);
        assert_eq!(config.api.port_range_end, 20000);
    }

    #[test]
    fn test_network_config_builder_timeouts() {
        let config = NetworkConfigBuilder::new()
            .connection_timeout(120)
            .keep_alive_timeout(300)
            .build();

        assert_eq!(config.api.connection_timeout, Duration::from_secs(120));
        assert_eq!(config.performance.keep_alive_timeout_seconds, 300);
    }

    // ==================== LoadBalancing & CircuitBreaker Tests (3 tests) ====================

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

        let json = serialized.expect("Network operation failed");
        let deserialized: std::result::Result<NetworkStatistics, _> = serde_json::from_str(&json);
        assert!(
            deserialized.is_ok(),
            "Network statistics should deserialize"
        );

        let deserialized_stats = deserialized.expect("Network operation failed");
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
