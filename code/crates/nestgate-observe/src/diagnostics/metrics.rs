// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// System Metrics Collection
/// This module contains structures for collecting and representing system metrics.
use nestgate_types::unified_enums::UnifiedServiceState as ServiceStatus;
use serde::{Deserialize, Serialize};
/// System-wide metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemmetrics
pub struct SystemMetrics {
    /// CPU usage percentage (0.0 to 100.0)
    #[allow(dead_code)]
    pub cpu_usage: f64,
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
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
/// Diskmetrics
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
/// Networkmetrics
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn system_metrics_default_json_roundtrip() -> Result<()> {
        let m = SystemMetrics::default();
        let j = serde_json::to_string(&m)?;
        let back: SystemMetrics = serde_json::from_str(&j)?;
        assert_eq!(back.memory_total, m.memory_total);
        assert_eq!(back.storage_total, m.storage_total);
        assert_eq!(back.uptime_seconds, m.uptime_seconds);
        Ok(())
    }

    #[test]
    fn disk_metrics_roundtrip() -> Result<()> {
        let d = DiskMetrics {
            device: "/dev/sda1".to_string(),
            mount_point: "/".to_string(),
            filesystem: "ext4".to_string(),
            total_bytes: 1024,
            used_bytes: 512,
            available_bytes: 512,
            usage_percent: 50.0,
            read_ops_per_sec: 1.0,
            write_ops_per_sec: 2.0,
            read_bytes_per_sec: 100,
            write_bytes_per_sec: 200,
        };
        let j = serde_json::to_string(&d)?;
        let back: DiskMetrics = serde_json::from_str(&j)?;
        assert_eq!(back.device, d.device);
        assert!((back.usage_percent - 50.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn network_metrics_roundtrip() -> Result<()> {
        let n = NetworkMetrics {
            interface: "eth0".to_string(),
            rx_bytes: 1,
            tx_bytes: 2,
            rx_packets: 3,
            tx_packets: 4,
            rx_errors: 0,
            tx_errors: 0,
            rx_drops: 0,
            tx_drops: 0,
            status: "up".to_string(),
            speed_mbps: 1000,
            duplex: "full".to_string(),
        };
        let j = serde_json::to_string(&n)?;
        let back: NetworkMetrics = serde_json::from_str(&j)?;
        assert_eq!(back.interface, n.interface);
        assert_eq!(back.speed_mbps, 1000);
        Ok(())
    }

    #[test]
    fn service_info_serializes_status() -> Result<()> {
        let s = ServiceInfo {
            name: "nestgate".to_string(),
            status: ServiceStatus::Running,
            pid: Some(42),
            cpu_percent: 1.5,
            memory_bytes: 4096,
            start_time: None,
            description: None,
            dependencies: vec![],
            command_line: None,
        };
        let j = serde_json::to_string(&s)?;
        assert!(j.contains("Running"));
        Ok(())
    }
}
/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceinfo
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
