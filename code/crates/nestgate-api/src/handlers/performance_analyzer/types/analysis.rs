// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Per-subsystem analysis rollups (CPU, memory, disk, network, ZFS).

use serde::{Deserialize, Serialize};

use super::trends::PerformanceTrend;

/// CPU analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cpuanalysis
pub struct CpuAnalysis {
    /// Current CPU usage
    pub current_usage: f64,
    /// Peak CPU usage in analysis period
    pub peak_usage: f64,
    /// Average CPU usage in analysis period
    pub average_usage: f64,
    /// CPU trend
    pub trend: PerformanceTrend,
}

/// Memory analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memoryanalysis
pub struct MemoryAnalysis {
    /// Current memory usage percentage
    pub current_usage_percent: f64,
    /// Peak memory usage in analysis period
    pub peak_usage_percent: f64,
    /// Average memory usage in analysis period
    pub average_usage_percent: f64,
    /// Memory trend
    pub trend: PerformanceTrend,
}

/// Disk analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Diskanalysis
pub struct DiskAnalysis {
    /// Current disk I/O utilization
    pub current_io_utilization: f64,
    /// Peak IOPS in analysis period
    pub peak_iops: f64,
    /// Average IOPS in analysis period
    pub average_iops: f64,
    /// Disk I/O trend
    pub trend: PerformanceTrend,
}

/// Network analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkanalysis
pub struct NetworkAnalysis {
    /// Current network utilization
    pub current_utilization: f64,
    /// Peak bandwidth usage in analysis period
    pub peak_bandwidth_mbps: f64,
    /// Average bandwidth usage in analysis period
    pub average_bandwidth_mbps: f64,
    /// Network trend
    pub trend: PerformanceTrend,
}

/// ZFS analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsanalysis
pub struct ZfsAnalysis {
    /// Current ARC hit ratio
    pub current_arc_hit_ratio: f64,
    /// Pool capacity usage
    pub pool_capacity_percent: f64,
    /// Pool health status
    pub pool_health: String,
    /// ZFS performance trend
    pub trend: PerformanceTrend,
}
