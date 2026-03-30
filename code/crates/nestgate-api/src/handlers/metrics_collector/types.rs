// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Real-time system and storage metrics collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeMetrics {
    /// Timestamp when these metrics were collected
    pub timestamp: SystemTime,
    /// Metrics for individual storage pools
    pub pool_metrics: Vec<PoolMetrics>,
    /// System-wide performance metrics
    pub system_metrics: SystemMetrics,
    /// ARC cache hit ratio (0.0 to 1.0)
    pub arc_hit_ratio: f64,
    /// L2ARC cache hit ratio (0.0 to 1.0)
    pub l2arc_hit_ratio: f64,
    /// Data compression ratio achieved
    pub compression_ratio: f64,
    /// Total system throughput in bytes per second
    pub total_throughput: f64,
    /// Average read latency in milliseconds
    pub average_read_latency: f64,
    /// Average write latency in milliseconds
    pub average_write_latency: f64,
}

/// Performance and utilization metrics for a storage pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    /// Name of the storage pool
    pub name: String,
    /// Current health status of the pool
    pub health_status: String,
    /// Pool capacity utilization as percentage (0.0 to 100.0)
    pub utilization_percentage: f64,
    /// Total pool capacity in bytes
    pub total_capacity: u64,
    /// Currently used space in bytes
    pub used_space: u64,
    /// Available free space in bytes
    pub available_space: u64,
    /// Read operations per second
    pub read_iops: u64,
    /// Write operations per second
    pub write_iops: u64,
    /// Read throughput in bytes per second
    pub read_throughput: f64,
    /// Write throughput in bytes per second
    pub write_throughput: f64,
    /// Pool fragmentation level (0.0 to 1.0)
    pub fragmentation_level: f64,
    /// Number of errors encountered
    pub error_count: u32,
}

/// System-wide performance and resource utilization metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage (currently unused, prefixed with _)
    pub _cpu_usage: f64,
    /// Memory usage percentage (0.0 to 100.0)
    pub memory_usage: f64,
    /// Total system memory in bytes
    pub memory_total: u64,
    /// Available memory in bytes
    pub memory_available: u64,
    /// Network I/O metrics
    pub network_io: NetworkIOMetrics,
    /// Disk I/O metrics
    pub disk_io: DiskIOMetrics,
}

/// Network input/output performance statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIOMetrics {
    /// Total bytes sent over network
    pub bytes_sent: u64,
    /// Total bytes received over network
    pub bytes_received: u64,
    /// Total packets sent
    pub packets_sent: u64,
    /// Total packets received
    pub packets_received: u64,
}

/// Disk input/output performance statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIOMetrics {
    /// Total bytes read from disk
    pub read_bytes: u64,
    /// Total bytes written to disk
    pub write_bytes: u64,
    /// Total read operations performed
    pub read_operations: u64,
    /// Total write operations performed
    pub write_operations: u64,
}

/// Point-in-time snapshot of system resource utilization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    /// Timestamp when this snapshot was taken
    pub timestamp: SystemTime,
    /// Number of CPU cores available
    pub cpu_cores: u32,
    /// Current CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Total system memory in gigabytes
    pub memory_total_gb: u32,
    /// Currently used memory in gigabytes
    pub memory_used_gb: u32,
    /// Total disk space in gigabytes
    pub disk_total_gb: u64,
    /// Currently used disk space in gigabytes
    pub disk_used_gb: u64,
    /// List of available network interfaces
    pub network_interfaces: Vec<String>,
}

/// Single data point for I/O performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOMetricsPoint {
    /// Timestamp for this measurement
    pub timestamp: SystemTime,
    /// Read operations per second at this time
    pub read_iops: u64,
    /// Write operations per second at this time
    pub write_iops: u64,
    /// Read latency in milliseconds
    pub read_latency: f64,
    /// Write latency in milliseconds
    pub write_latency: f64,
}

/// Single data point for cache performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetricsPoint {
    /// Timestamp for this measurement
    pub timestamp: SystemTime,
    /// ARC hit ratio at this time (0.0 to 1.0)
    pub arc_hit_ratio: f64,
    /// L2ARC hit ratio at this time (0.0 to 1.0)
    pub l2arc_hit_ratio: f64,
    /// ARC size in bytes
    pub arc_size: u64,
    /// L2ARC size in bytes
    pub l2arc_size: u64,
}

/// Complete metrics data point for time series analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveMetricsPoint {
    /// Timestamp for this comprehensive measurement
    pub timestamp: SystemTime,
    /// I/O performance metrics at this time
    pub io_metrics: IOMetricsPoint,
    /// Cache performance metrics at this time
    pub cache_metrics: CacheMetricsPoint,
    /// Capacity utilization metrics at this time
    pub capacity_metrics: CapacityMetricsPoint,
}

/// Single data point for capacity utilization metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityMetricsPoint {
    /// Timestamp for this measurement
    pub timestamp: SystemTime,
    /// Total system capacity in bytes
    pub total_capacity: u64,
    /// Currently used space in bytes
    pub used_space: u64,
    /// Rate of capacity growth in bytes per day
    pub growth_rate: f64,
}
