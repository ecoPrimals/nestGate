// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Point-in-time resource metrics (CPU, memory, disk, network, ZFS).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Performance snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancesnapshot
pub struct PerformanceSnapshot {
    /// Timestamp of the snapshot
    pub timestamp: DateTime<Utc>,
    /// CPU metrics
    pub cpu: CpuMetrics,
    /// Memory metrics
    pub memory: MemoryMetrics,
    /// Disk metrics
    pub disk: DiskMetrics,
    /// Network metrics
    pub network: NetworkMetrics,
    /// ZFS metrics
    pub zfs: ZfsMetrics,
}

/// CPU performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cpumetrics
pub struct CpuMetrics {
    /// CPU usage percentage
    pub usage_percent: f64,
    /// Load average (1 minute)
    pub load_average_1m: f64,
    /// Load average (5 minutes)
    pub load_average_5m: f64,
    /// Load average (15 minutes)
    pub load_average_15m: f64,
    /// Number of CPU cores
    pub core_count: u32,
}

/// Memory performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memorymetrics
pub struct MemoryMetrics {
    /// Total memory in bytes
    pub total_bytes: u64,
    /// Used memory in bytes
    pub used_bytes: u64,
    /// Available memory in bytes
    pub available_bytes: u64,
    /// Memory usage percentage
    pub usage_percent: f64,
    /// Swap usage in bytes
    pub swap_used_bytes: u64,
}

/// Disk performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Diskmetrics
pub struct DiskMetrics {
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
    /// Read throughput in bytes per second
    pub read_bytes_per_sec: f64,
    /// Write throughput in bytes per second
    pub write_bytes_per_sec: f64,
    /// Average queue depth
    pub avg_queue_depth: f64,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkmetrics
pub struct NetworkMetrics {
    /// Bytes received per second
    pub rx_bytes_per_sec: f64,
    /// Bytes transmitted per second
    pub tx_bytes_per_sec: f64,
    /// Packets received per second
    pub rx_packets_per_sec: f64,
    /// Packets transmitted per second
    pub tx_packets_per_sec: f64,
}

/// ZFS performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsmetrics
pub struct ZfsMetrics {
    /// ARC hit ratio
    pub arc_hit_ratio: f64,
    /// ARC size in bytes
    pub arc_size_bytes: u64,
    /// L2ARC hit ratio
    pub l2arc_hit_ratio: f64,
    /// Pool capacity usage percentage
    pub pool_capacity_percent: f64,
    /// Pool health status
    pub pool_health: String,
    /// Scrub status
    pub scrub_status: String,
    /// Dataset count
    pub dataset_count: u32,
    /// Snapshot count
    pub snapshot_count: u32,
}
