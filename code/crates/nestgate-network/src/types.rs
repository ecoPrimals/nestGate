//! **NETWORK TYPES AND CONFIGURATION**
//!
//! This module provides all the data structures, enums, and configuration
//! types used by the network service.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};

// ==================== SECTION ====================

/// Network service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Maximum number of concurrent connections
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    /// Port range start for service allocation
    pub port_range_start: u16,
    /// Port range end for service allocation
    pub port_range_end: u16,
    /// Enable keep-alive
    pub keep_alive: bool,
    /// Keep-alive timeout in seconds
    pub keep_alive_timeout_seconds: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: nestgate_core::constants::canonical::network::DEFAULT_BIND_ADDRESS.to_string(),
            port: nestgate_core::constants::canonical::network::DEFAULT_API_PORT,
            max_connections: 1000,
            connection_timeout_seconds: 30,
            port_range_start: 9000,
            port_range_end: 9999,
            keep_alive: true,
            keep_alive_timeout_seconds: 60,
        }
    }
}

// ==================== SECTION ====================

/// Connection information
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub id: String,
    pub address: SocketAddr,
    pub established_at: SystemTime,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub status: ConnectionStatus,
}

impl ConnectionInfo {
    /// Create a new connection info
    pub fn new(id: String, address: SocketAddr) -> Self {
        Self {
            id,
            address,
            established_at: SystemTime::now(),
            bytes_sent: 0,
            bytes_received: 0,
            status: ConnectionStatus::Active,
        }
    }

    /// Get connection ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get connection address
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// Get connection age
    pub fn age(&self) -> Duration {
        self.established_at.elapsed().unwrap_or_default()
    }

    /// Check if connection is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, ConnectionStatus::Active)
    }

    /// Get connection status
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
    pub address: SocketAddr,
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
    pub address: SocketAddr,
    pub health_status: HealthStatus,
    pub registered_at: SystemTime,
    pub metadata: HashMap<String, String>,
}

impl ServiceInfo {
    /// Create a new service info
    pub fn new(id: String, name: String, address: SocketAddr) -> Self {
        Self {
            id,
            name,
            address,
            health_status: HealthStatus::Healthy,
            registered_at: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }

    /// Get service ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get service name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get service address
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// Get health status
    pub fn health_status(&self) -> &HealthStatus {
        &self.health_status
    }

    /// Get registration time
    pub fn registered_at(&self) -> SystemTime {
        self.registered_at
    }

    /// Get metadata
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Check if service is healthy
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
    pub address: SocketAddr,
    pub health_status: String,
    pub registered_at: SystemTime,
    pub metadata: HashMap<String, String>,
}

// ==================== SECTION ====================

/// Network statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatistics {
    pub active_connections: u32,
    pub registered_services: u32,
    pub allocated_ports: u32,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
}

impl Default for NetworkStatistics {
    fn default() -> Self {
        Self {
            active_connections: 0,
            registered_services: 0,
            allocated_ports: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
        }
    }
}

/// Service status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub running: bool,
    pub connections: u32,
    pub services: u32,
    pub uptime_seconds: u64,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self {
            running: false,
            connections: 0,
            services: 0,
            uptime_seconds: 0,
        }
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
    pub fn new() -> Self {
        Self {
            config: NetworkConfig::default(),
        }
    }

    /// Set host
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.config.host = host.into();
        self
    }

    /// Set port
    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    /// Set max connections
    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.config.max_connections = max_connections;
        self
    }

    /// Set connection timeout
    pub fn connection_timeout(mut self, timeout_seconds: u64) -> Self {
        self.config.connection_timeout_seconds = timeout_seconds;
        self
    }

    /// Set port range
    pub fn port_range(mut self, start: u16, end: u16) -> Self {
        self.config.port_range_start = start;
        self.config.port_range_end = end;
        self
    }

    /// Enable/disable keep-alive
    pub fn keep_alive(mut self, enabled: bool) -> Self {
        self.config.keep_alive = enabled;
        self
    }

    /// Set keep-alive timeout
    pub fn keep_alive_timeout(mut self, timeout_seconds: u64) -> Self {
        self.config.keep_alive_timeout_seconds = timeout_seconds;
        self
    }

    /// Build the configuration
    pub fn build(self) -> NetworkConfig {
        self.config
    }
}

impl Default for NetworkConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
