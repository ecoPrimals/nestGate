// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// ZFS bottleneck detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsBottleneck {
    /// Type of bottleneck detected
    pub bottleneck_type: ZfsBottleneckType,
    /// Severity level of the bottleneck
    pub severity: BottleneckSeverity,
    /// Name of the affected ZFS pool
    pub pool_name: String,
    /// Name of the affected dataset (if applicable)
    pub dataset_name: Option<String>,
    /// Human-readable description of the bottleneck
    pub description: String,
    /// Impact score (0.0 to 1.0, higher = more severe)
    pub impact_score: f64,
}
/// Types of ZFS bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsBottleneckType {
    /// High latency detected
    HighLatency,
    /// Low throughput detected
    LowThroughput,
    /// High cache miss rate
    CacheMiss,
    /// High pool fragmentation
    Fragmentation,
    /// System memory pressure
    MemoryPressure,
    /// High CPU utilization
    CpuUtilization,
    /// Network bandwidth saturation
    NetworkBandwidth,
    /// Disk I/O saturation
    DiskIo,
}
/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    /// Low severity - minor impact
    Low,
    /// Medium severity - noticeable impact
    Medium,
    /// High severity - significant impact
    High,
    /// Critical severity - severe impact requiring immediate attention
    Critical,
}
