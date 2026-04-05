// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capacity monitoring and performance analysis types
//!
//! Domain: Monitoring, bottleneck detection, retention policies

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Bottleneck detection report for performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckReport {
    /// Dataset being analyzed
    pub dataset: String,
    /// Type of bottleneck detected (CPU, I/O, memory, etc.)
    pub bottleneck_type: String,
    /// Severity level of the bottleneck
    pub severity: String,
    /// Recommended actions to resolve the bottleneck
    pub recommendations: Vec<String>,
}

/// Capacity monitoring report for storage management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityReport {
    /// Dataset being monitored
    pub dataset: String,
    /// Current storage usage in bytes
    pub current_usage: u64,
    /// Projected future usage in bytes
    pub projected_usage: u64,
    /// Recommended capacity management actions
    pub recommendations: Vec<String>,
}

/// Maintenance scheduling information for automated operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    /// Dataset requiring maintenance
    pub dataset: String,
    /// Next scheduled maintenance time
    pub next_maintenance: SystemTime,
    /// List of maintenance tasks to perform
    pub tasks: Vec<String>,
}

/// System information for monitoring and diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Time when metrics were collected
    pub timestamp: SystemTime,
    /// CPU usage percentage (0.0-100.0)
    pub cpu_usage: f64,
    /// Memory usage percentage (0.0-100.0)
    pub memory_usage: f64,
    /// Disk usage percentage (0.0-100.0)
    pub disk_usage: f64,
}

/// Replication performance metrics for monitoring data transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPerformance {
    /// Source dataset being replicated from
    pub source_dataset: String,
    /// Target dataset being replicated to
    pub target_dataset: String,
    /// Current transfer rate in bytes per second
    pub transfer_rate: f64,
    /// Compression ratio achieved during replication
    pub compression_ratio: f64,
    /// Estimated time of completion
    pub estimated_completion: SystemTime,
}

/// Snapshot retention policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Name of the retention policy
    pub name: String,
    /// Number of hourly snapshots to keep
    pub keep_hourly: u32,
    /// Number of daily snapshots to keep
    pub keep_daily: u32,
    /// Number of weekly snapshots to keep
    pub keep_weekly: u32,
    /// Number of monthly snapshots to keep
    pub keep_monthly: u32,
}
