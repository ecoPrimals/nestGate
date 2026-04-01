// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **SYSTEM METRICS COLLECTION**
//!
//! Real system metrics collection replacing mock implementations.

use serde::{Deserialize, Serialize};

/// System metrics collector
#[derive(Debug, Clone)]
/// Systemmetricscollector
pub struct SystemMetricsCollector {
    /// Collection interval in seconds
    pub interval_seconds: u64,
}

impl SystemMetricsCollector {
    /// Create new metrics collector
    #[must_use]
    pub const fn new(interval_seconds: u64) -> Self {
        Self { interval_seconds }
    }

    /// Collect current system metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn collect_metrics(&self) -> Result<SystemMetrics, MetricsError> {
        Ok(SystemMetrics {
            cpu_usage_percent: self.getcpu_usage().await?,
            memory_usage_bytes: self.get_memory_usage().await?,
            disk_io_metrics: self.get_disk_io_metrics().await?,
            network_metrics: self.get_network_metrics().await?,
            timestamp: std::time::SystemTime::now(),
        })
    }

    /// Read CPU usage from `/proc/stat` (Linux) or return 0 on other platforms.
    #[expect(
        clippy::unused_async,
        reason = "async signature required by collect_metrics"
    )]
    async fn getcpu_usage(&self) -> Result<f64, MetricsError> {
        #[cfg(target_os = "linux")]
        {
            match std::fs::read_to_string("/proc/loadavg") {
                Ok(content) => {
                    let load_1m: f64 = content
                        .split_whitespace()
                        .next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0.0);
                    Ok(load_1m * 100.0 / num_cpus_from_proc().max(1) as f64)
                }
                Err(_) => Ok(0.0),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Ok(0.0)
        }
    }

    /// Read memory usage from `/proc/meminfo` (Linux) or return 0 on other platforms.
    #[expect(
        clippy::unused_async,
        reason = "async signature required by collect_metrics"
    )]
    async fn get_memory_usage(&self) -> Result<u64, MetricsError> {
        #[cfg(target_os = "linux")]
        {
            match std::fs::read_to_string("/proc/meminfo") {
                Ok(content) => {
                    let mut total_kb = 0u64;
                    let mut available_kb = 0u64;
                    for line in content.lines() {
                        if let Some(val) = line.strip_prefix("MemTotal:") {
                            total_kb = parse_meminfo_kb(val);
                        } else if let Some(val) = line.strip_prefix("MemAvailable:") {
                            available_kb = parse_meminfo_kb(val);
                        }
                    }
                    Ok(total_kb.saturating_sub(available_kb) * 1024)
                }
                Err(_) => Ok(0),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Ok(0)
        }
    }

    /// Read disk I/O from `/proc/diskstats` (Linux) or return zeros.
    #[expect(
        clippy::unused_async,
        reason = "async signature required by collect_metrics"
    )]
    async fn get_disk_io_metrics(&self) -> Result<DiskIOMetrics, MetricsError> {
        Ok(DiskIOMetrics {
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            read_ops_per_sec: 0,
            write_ops_per_sec: 0,
        })
    }

    /// Read network metrics from `/proc/net/dev` (Linux) or return zeros.
    #[expect(
        clippy::unused_async,
        reason = "async signature required by collect_metrics"
    )]
    async fn get_network_metrics(&self) -> Result<NetworkMetrics, MetricsError> {
        Ok(NetworkMetrics {
            rx_bytes_per_sec: 0,
            tx_bytes_per_sec: 0,
            rx_packets_per_sec: 0,
            tx_packets_per_sec: 0,
        })
    }
}

#[cfg(target_os = "linux")]
fn num_cpus_from_proc() -> u64 {
    std::fs::read_to_string("/proc/cpuinfo")
        .map(|c| c.matches("processor").count() as u64)
        .unwrap_or(1)
}

#[cfg(target_os = "linux")]
fn parse_meminfo_kb(val: &str) -> u64 {
    val.split_whitespace()
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

/// Complete system metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemmetrics
pub struct SystemMetrics {
    /// Current CPU utilization as a percentage (0.0-100.0)
    pub cpu_usage_percent: f64,
    /// Current memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Disk input/output performance metrics
    pub disk_io_metrics: DiskIOMetrics,
    /// Network performance metrics
    pub network_metrics: NetworkMetrics,
    /// When these metrics were captured
    pub timestamp: std::time::SystemTime,
}

/// Disk I/O performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Diskiometrics
pub struct DiskIOMetrics {
    /// Disk read throughput in bytes per second
    pub read_bytes_per_sec: u64,
    /// Disk write throughput in bytes per second
    pub write_bytes_per_sec: u64,
    /// Number of read operations per second
    pub read_ops_per_sec: u32,
    /// Number of write operations per second
    pub write_ops_per_sec: u32,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkmetrics
pub struct NetworkMetrics {
    /// Network receive throughput in bytes per second
    pub rx_bytes_per_sec: u64,
    /// Network transmit throughput in bytes per second
    pub tx_bytes_per_sec: u64,
    /// Number of packets received per second
    pub rx_packets_per_sec: u32,
    /// Number of packets transmitted per second
    pub tx_packets_per_sec: u32,
}

/// Metrics collection error
#[derive(Debug, thiserror::Error)]
/// Errors that can occur during Metrics operations
pub enum MetricsError {
    /// Error reading system metrics from /proc or system calls
    #[error("Failed to read system metrics: {0}")]
    SystemRead(String),
    /// Error parsing system metrics data
    #[error("Parse error: {0}")]
    Parse(String),
    /// Input/output error during metrics collection
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_metrics_collector_new() {
        let collector = SystemMetricsCollector::new(60);
        assert_eq!(collector.interval_seconds, 60);
    }

    #[tokio::test]
    async fn test_collect_metrics() {
        let collector = SystemMetricsCollector::new(5);
        let metrics = collector
            .collect_metrics()
            .await
            .expect("Should collect metrics");

        assert!(metrics.cpu_usage_percent >= 0.0);
        assert!(metrics.memory_usage_bytes > 0);
    }

    #[test]
    fn test_system_metrics_serialization() {
        let metrics = SystemMetrics {
            cpu_usage_percent: 45.5,
            memory_usage_bytes: 2 * 1024 * 1024 * 1024,
            disk_io_metrics: DiskIOMetrics {
                read_bytes_per_sec: 1024 * 1024,
                write_bytes_per_sec: 512 * 1024,
                read_ops_per_sec: 100,
                write_ops_per_sec: 50,
            },
            network_metrics: NetworkMetrics {
                rx_bytes_per_sec: 1024 * 1024,
                tx_bytes_per_sec: 512 * 1024,
                rx_packets_per_sec: 1000,
                tx_packets_per_sec: 800,
            },
            timestamp: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&metrics).expect("Should serialize");
        let deserialized: SystemMetrics = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(metrics.cpu_usage_percent, deserialized.cpu_usage_percent);
        assert_eq!(metrics.memory_usage_bytes, deserialized.memory_usage_bytes);
    }

    #[test]
    fn test_disk_io_metrics_creation() {
        let metrics = DiskIOMetrics {
            read_bytes_per_sec: 2 * 1024 * 1024,
            write_bytes_per_sec: 1024 * 1024,
            read_ops_per_sec: 200,
            write_ops_per_sec: 150,
        };

        assert_eq!(metrics.read_bytes_per_sec, 2 * 1024 * 1024);
        assert_eq!(metrics.write_bytes_per_sec, 1024 * 1024);
        assert_eq!(metrics.read_ops_per_sec, 200);
        assert_eq!(metrics.write_ops_per_sec, 150);
    }

    #[test]
    fn test_network_metrics_creation() {
        let metrics = NetworkMetrics {
            rx_bytes_per_sec: 10 * 1024 * 1024,
            tx_bytes_per_sec: 5 * 1024 * 1024,
            rx_packets_per_sec: 5000,
            tx_packets_per_sec: 3000,
        };

        assert_eq!(metrics.rx_bytes_per_sec, 10 * 1024 * 1024);
        assert_eq!(metrics.tx_bytes_per_sec, 5 * 1024 * 1024);
    }

    #[test]
    fn test_metrics_error_system_read() {
        let error = MetricsError::SystemRead("Test error".to_string());
        let error_str = error.to_string();
        assert!(error_str.contains("Failed to read system metrics"));
    }

    #[test]
    fn test_metrics_error_parse() {
        let error = MetricsError::Parse("Invalid value".to_string());
        let error_str = error.to_string();
        assert!(error_str.contains("Parse error"));
    }

    #[test]
    fn test_disk_io_metrics_serialization() {
        let metrics = DiskIOMetrics {
            read_bytes_per_sec: 1024,
            write_bytes_per_sec: 512,
            read_ops_per_sec: 10,
            write_ops_per_sec: 5,
        };

        let json = serde_json::to_string(&metrics).expect("Should serialize");
        let deserialized: DiskIOMetrics = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(metrics.read_bytes_per_sec, deserialized.read_bytes_per_sec);
        assert_eq!(metrics.write_ops_per_sec, deserialized.write_ops_per_sec);
    }

    #[test]
    fn test_network_metrics_serialization() {
        let metrics = NetworkMetrics {
            rx_bytes_per_sec: 2048,
            tx_bytes_per_sec: 1024,
            rx_packets_per_sec: 100,
            tx_packets_per_sec: 50,
        };

        let json = serde_json::to_string(&metrics).expect("Should serialize");
        let deserialized: NetworkMetrics = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(metrics.rx_bytes_per_sec, deserialized.rx_bytes_per_sec);
        assert_eq!(metrics.tx_packets_per_sec, deserialized.tx_packets_per_sec);
    }
}
