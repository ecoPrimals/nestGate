/// System Metrics Collection
/// This module contains structures for collecting and representing system metrics.
use crate::unified_enums::UnifiedServiceState as ServiceStatus;
use serde::{Deserialize, Serialize};
/// System-wide metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage (0.0 to 100.0)
    pub _cpu_usage: f64,
    /// Memory used in bytes
    pub memory_used: u64,
    /// Total memory in bytes
    pub memory_total: u64,
    /// Storage used in bytes
    pub storage_used: u64,
    /// Total storage in bytes
    pub storage_total: u64,
    /// Load average (1 minute)
    pub load_average_1m: f64,
    /// Load average (5 minutes)
    pub load_average_5m: f64,
    /// Load average (15 minutes)
    pub load_average_15m: f64,
    /// Number of running processes
    pub process_count: u32,
    /// Network I/O bytes received
    pub network_rx_bytes: u64,
    /// Network I/O bytes transmitted
    pub network_tx_bytes: u64,
    /// Disk I/O read bytes
    pub disk_read_bytes: u64,
    /// Disk I/O write bytes
    pub disk_write_bytes: u64,
    /// System uptime in seconds
    pub uptime_seconds: u64,
}
impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            _cpu_usage: 0.0,
            memory_used: 0,
            memory_total: 8 * 1024 * 1024 * 1024, // 8GB default
            storage_used: 0,
            storage_total: 500 * 1024 * 1024 * 1024, // 500GB default
            load_average_1m: 0.0,
            load_average_5m: 0.0,
            load_average_15m: 0.0,
            process_count: 0,
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            disk_read_bytes: 0,
            disk_write_bytes: 0,
            uptime_seconds: 0,
        }
    }
}

/// Disk-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// Device name (e.g., "/dev/sda1")
    pub device: String,
    /// Mount point (e.g., "/")
    pub mount_point: String,
    /// Filesystem type (e.g., "ext4")
    pub filesystem: String,
    /// Total space in bytes
    pub total_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Usage percentage (0.0 to 100.0)
    pub usage_percent: f64,
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
    /// Read bytes per second
    pub read_bytes_per_sec: u64,
    /// Write bytes per second
    pub write_bytes_per_sec: u64,
}
/// Network interface metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Interface name (e.g., "eth0")
    pub interface: String,
    /// Bytes received
    pub rx_bytes: u64,
    /// Bytes transmitted
    pub tx_bytes: u64,
    /// Packets received
    pub rx_packets: u64,
    /// Packets transmitted
    pub tx_packets: u64,
    /// Receive errors
    pub rx_errors: u64,
    /// Transmit errors
    pub tx_errors: u64,
    /// Receive drops
    pub rx_drops: u64,
    /// Transmit drops
    pub tx_drops: u64,
    /// Interface status (up/down)
    pub status: String,
    /// Link speed in Mbps
    pub speed_mbps: u32,
    /// Duplex mode (full/half)
    pub duplex: String,
}
/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name
    pub name: String,
    /// Service status
    pub status: ServiceStatus,
    /// Process ID (if running)
    pub pid: Option<u32>,
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Service start time
    pub start_time: Option<std::time::SystemTime>,
    /// Service description
    pub description: Option<String>,
    /// Service configuration path
    /// Service log path
    /// Service dependencies
    pub dependencies: Vec<String>,
    /// Service environment variables
    /// Service command line
    pub command_line: Option<String>,
}
