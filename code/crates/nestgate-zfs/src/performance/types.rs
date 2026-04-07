// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Types module

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{RwLock, mpsc};

use crate::snapshot::SnapshotPolicy;
use crate::tier::TierStats;

/// Type alias for tier-specific performance metrics map with thread-safe access.
pub type TierMetricsMap = Arc<RwLock<HashMap<StorageTier, TierPerformanceData>>>;
/// Type alias for Alertconditionsvec
pub type AlertConditionsVec = Arc<RwLock<Vec<AlertCondition>>>;
/// Type alias for Activealertsvec
pub type ActiveAlertsVec = Arc<RwLock<Vec<ActiveAlert>>>;
/// Type alias for Snapshotpolicymap
pub type SnapshotPolicyMap = Arc<RwLock<HashMap<String, SnapshotPolicy>>>;
/// Type alias for Tierstatsmap
pub type TierStatsMap = Arc<RwLock<HashMap<StorageTier, TierStats>>>;

use crate::types::StorageTier;
use crate::{dataset::ZfsDatasetManager, pool::ZfsPoolManager};
// Removed unused import: StorageTier as CoreStorageTier
use std::time::Duration;

/// System performance metrics snapshot
///
/// Captures system-wide resource utilization metrics for performance monitoring
/// and bottleneck detection. Used by the performance monitoring system to track
/// system health and identify resource constraints.
///
/// # Fields
///
/// * `memory_utilization_percent` - Memory usage as percentage (0.0-100.0)
/// * `cpu_utilization_percent` - CPU usage as percentage (0.0-100.0)
/// * `disk_queue_depth` - Number of pending I/O operations
/// * `network_throughput_mbs` - Network throughput in megabytes per second
/// * `system_load_average` - System load average (1-minute)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformanceMetrics {
    /// Memory utilization percentage (0.0-100.0)
    pub memory_utilization_percent: f64,
    /// CPU utilization percentage (0.0-100.0)
    pub cpu_utilization_percent: f64,
    /// Number of pending disk I/O operations
    pub disk_queue_depth: u64,
    /// Network throughput in megabytes per second
    pub network_throughput_mbs: f64,
    /// System load average over 1 minute
    pub system_load_average: f64,
}

/// Memory information and utilization
///
/// Detailed memory usage statistics for the system, tracking total,
/// available, and used memory in megabytes.
///
/// # Fields
///
/// * `utilization_percent` - Memory utilization as percentage (0.0-100.0)
/// * `total_mb` - Total system memory in megabytes
/// * `available_mb` - Available memory in megabytes
/// * `used_mb` - Used memory in megabytes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Memory utilization percentage (0.0-100.0)
    pub utilization_percent: f64,
    /// Total system memory in megabytes
    pub total_mb: u64,
    /// Available memory in megabytes
    pub available_mb: u64,
    /// Used memory in megabytes
    pub used_mb: u64,
}
impl Default for SystemPerformanceMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            memory_utilization_percent: 0.0,
            cpu_utilization_percent: 0.0,
            disk_queue_depth: 0,
            network_throughput_mbs: 0.0,
            system_load_average: 0.0,
        }
    }
}

impl Default for MemoryInfo {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            utilization_percent: 0.0,
            total_mb: 0,
            available_mb: 0,
            used_mb: 0,
        }
    }
}

/// Disk I/O statistics and performance metrics
///
/// Tracks disk I/O performance including queue depth, throughput, and IOPS
/// (Input/Output Operations Per Second) for read and write operations.
///
/// # Fields
///
/// * `queue_depth` - Number of pending I/O operations in the queue
/// * `throughput_mbs` - Disk throughput in megabytes per second
/// * `read_iops` - Read operations per second
/// * `write_iops` - Write operations per second
///
/// # Usage
///
/// Used for monitoring disk performance and identifying I/O bottlenecks
/// in the storage subsystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Diskiostats
pub struct DiskIoStats {
    /// Number of pending I/O operations in the queue
    pub queue_depth: u64,
    /// Disk throughput in megabytes per second
    pub throughput_mbs: f64,
    /// Read operations per second (IOPS)
    pub read_iops: u64,
    /// Write operations per second (IOPS)
    pub write_iops: u64,
}
impl Default for DiskIoStats {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            queue_depth: 0,
            throughput_mbs: 0.0,
            read_iops: 0,
            write_iops: 0,
        }
    }
}

/// ZFS performance monitor
#[derive(Debug)]
/// Zfsperformancemonitor
pub struct ZfsPerformanceMonitor {
    // config removed - using shared ZfsConfig instead
    /// Pool manager for ZFS pool operations
    pub pool_manager: Arc<ZfsPoolManager>,
    /// Dataset Manager
    pub dataset_manager: Arc<ZfsDatasetManager>,
    /// Real-time metrics
    pub current_metrics: Arc<RwLock<CurrentPerformanceMetrics>>,
    /// Historical metrics
    pub metrics_history: Arc<RwLock<VecDeque<PerformanceSnapshot>>>,
    /// Tier-specific metrics
    pub tier_metrics: TierMetricsMap,
    /// Alert conditions
    pub alert_conditions: AlertConditionsVec,
    /// Active alerts
    pub active_alerts: ActiveAlertsVec,

    /// Background tasks
    pub collection_task: Option<tokio::task::JoinHandle<()>>,
    /// Analysis Task
    pub analysis_task: Option<tokio::task::JoinHandle<()>>,
    /// Alert Task
    pub alert_task: Option<tokio::task::JoinHandle<()>>,

    /// Alert notification channel
    pub alert_sender: Option<mpsc::Sender<Alert>>,
}

/// Current performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Currentperformancemetrics
pub struct CurrentPerformanceMetrics {
    /// Timestamp of last update
    pub timestamp: SystemTime,
    /// Pool-wide metrics
    pub pool_metrics: PoolPerformanceMetrics,
    /// Tier-specific metrics
    pub tier_metrics: HashMap<StorageTier, TierMetrics>,
    /// System resource metrics
    pub system_metrics: SystemResourceMetrics,
    /// I/O statistics
    pub io_statistics: IoStatistics,
    /// Performance trends
    pub trends: PerformanceTrends,
}
/// Pool performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolperformancemetrics
pub struct PoolPerformanceMetrics {
    /// Total IOPS across all datasets
    pub total_iops: f64,
    /// Total throughput in MB/s
    pub total_throughput_mbs: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Pool utilization percentage
    pub utilization_percent: f64,
    /// Pool fragmentation percentage
    pub fragmentation_percent: f64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Deduplication ratio
    pub dedup_ratio: f64,
}
/// Tier-specific performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tiermetrics
pub struct TierMetrics {
    /// Storage tier
    pub tier: StorageTier,
    /// Read IOPS
    pub read_iops: f64,
    /// Write IOPS
    pub write_iops: f64,
    /// Read throughput in MB/s
    pub read_throughput_mbs: f64,
    /// Write throughput in MB/s
    pub write_throughput_mbs: f64,
    /// Average read latency in milliseconds
    pub avg_read_latency_ms: f64,
    /// Average write latency in milliseconds
    pub avg_write_latency_ms: f64,
    /// Queue depth
    pub queue_depth: f64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Utilization percentage
    pub utilization_percent: f64,
    /// Performance targets
    pub targets: TierPerformanceTargets,
    /// SLA compliance
    pub sla_compliance: SlaCompliance,
}
/// System resource metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemresourcemetrics
pub struct SystemResourceMetrics {
    /// CPU utilization percentage
    pub cpu_utilization_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Available memory in bytes
    pub available_memory_bytes: u64,
    /// Network I/O in MB/s
    pub network_io_mbs: f64,
    /// I/O wait percentage
    pub io_wait_percent: f64,
    /// 1-minute load average
    pub load_average_1m: f64,
}
/// I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Iostatistics
pub struct IoStatistics {
    /// Total read operations
    pub total_reads: u64,
    /// Total write operations
    pub total_writes: u64,
    /// Total bytes read
    pub total_bytes_read: u64,
    /// Total bytes written
    pub total_bytes_written: u64,
    /// Average I/O size in bytes
    pub avg_io_size_bytes: u64,
    /// Read/write ratio
    pub read_write_ratio: f64,
}
/// Performance snapshot for historical analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancesnapshot
pub struct PerformanceSnapshot {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Metrics snapshot
    pub metrics: CurrentPerformanceMetrics,
    /// Performance score (0-100)
    pub performance_score: f64,
}
/// Performance trends analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancetrends
pub struct PerformanceTrends {
    /// IOPS trend (positive = increasing)
    pub iops_trend: f64,
    /// Throughput trend (positive = increasing)
    pub throughput_trend: f64,
    /// Latency trend (positive = increasing)
    pub latency_trend: f64,
    /// Utilization trend (positive = increasing)
    pub utilization_trend: f64,
    /// Prediction confidence (0-1)
    pub prediction_confidence: f64,
}
/// Tier performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierperformancedata
pub struct TierPerformanceData {
    /// Tier type
    pub tier: StorageTier,
    /// Current metrics
    pub current_metrics: TierMetrics,
    /// Historical data
    pub history: VecDeque<TierMetrics>,
    /// Trend analysis
    pub trends: PerformanceTrends,
}
/// Performance targets for a tier
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierperformancetargets
pub struct TierPerformanceTargets {
    /// Target IOPS
    pub target_iops: f64,
    /// Target throughput in MB/s
    pub target_throughput_mbs: f64,
    /// Target latency in milliseconds
    pub target_latency_ms: f64,
    /// Target utilization percentage
    pub target_utilization_percent: f64,
    /// Target availability percentage
    pub target_availability_percent: f64,
}
/// SLA compliance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Slacompliance
pub struct SlaCompliance {
    /// Latency compliance percentage
    pub latency_compliance: f64,
    /// Throughput compliance percentage
    pub throughput_compliance: f64,
    /// Availability percentage
    pub availability_percent: f64,
    /// Error rate compliance percentage
    pub error_rate_compliance: f64,
}
/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertcondition
pub struct AlertCondition {
    /// Unique identifier
    pub id: String,
    /// Alert name
    pub name: String,
    /// Alert description
    pub description: String,
    /// Metric to monitor
    pub metric: AlertMetric,
    /// Comparison operator
    pub operator: AlertOperator,
    /// Threshold value
    pub threshold: f64,
    /// Duration before triggering
    pub duration: Duration,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Whether the alert is enabled
    pub enabled: bool,
}
/// Alert metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertmetric
pub enum AlertMetric {
    /// IOPS
    Iops,
    /// Throughput
    Throughput,
    /// Latency
    Latency,
    /// Utilization
    Utilization,
    /// Memory usage
    MemoryUsage,
    /// CPU usage
    CpuUsage,
    /// Error rate
    ErrorRate,
    /// Availability
    Availability,
    /// Queue depth
    QueueDepth,
    /// Cache hit ratio
    CacheHitRatio,
}
/// Alert operators
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertoperator
pub enum AlertOperator {
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Equal to
    EqualTo,
    /// Greater than or equal to
    GreaterThanOrEqualTo,
    /// Less than or equal to
    LessThanOrEqualTo,
    /// Not equal to
    NotEqualTo,
}
/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertseverity
pub enum AlertSeverity {
    /// Critical alert
    Critical,
    /// Warning alert
    Warning,
    /// Information alert
    Info,
}
/// Active alert
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Activealert
pub struct ActiveAlert {
    /// Alert condition ID
    pub condition_id: String,
    /// Alert name
    pub name: String,
    /// Alert description
    pub description: String,
    /// Current value
    pub currentvalue: f64,
    /// Threshold value
    pub threshold: f64,
    /// Severity
    pub severity: AlertSeverity,
    /// Start time
    pub start_time: SystemTime,
    /// Last update time
    pub last_update: SystemTime,
    /// Acknowledgment status
    pub acknowledged: bool,
    /// Acknowledgment time
    pub ack_time: Option<SystemTime>,
    /// Acknowledgment user
    pub ack_user: Option<String>,
}
/// Alert notification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alert
pub struct Alert {
    /// Alert type
    pub alert_type: AlertType,
    /// Alert data
    pub data: ActiveAlert,
}
/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Alert
pub enum AlertType {
    /// New alert triggered
    Triggered,
    /// Alert resolved
    Resolved,
    /// Alert acknowledged
    Acknowledged,
}
/// I/O statistics summary
#[derive(Debug, Clone)]
pub(crate) struct IoStatsSummary {
    /// Read Ops
    pub read_ops: u64,
    /// Write Ops
    pub write_ops: u64,
    /// Read Throughput Mbs
    pub read_throughput_mbs: f64,
    /// Write Throughput Mbs
    pub write_throughput_mbs: f64,
    /// Read Latency Ms
    pub read_latency_ms: f64,
    /// Write Latency Ms
    pub write_latency_ms: f64,
}
/// Pool properties
#[derive(Debug, Clone)]
pub(crate) struct PoolProperties {
    /// Fragmentation
    pub fragmentation: f64,
    /// Compression Ratio
    pub compression_ratio: f64,
    /// Dedup Ratio
    pub dedup_ratio: f64,
}
#[cfg(test)]
pub(crate) struct LocalMemoryInfo {
    available_mb: u64,
    used_mb: u64,
}
#[cfg(test)]
impl LocalMemoryInfo {
    pub const fn new(available_mb: u64, used_mb: u64) -> Self {
        Self {
            available_mb,
            used_mb,
        }
    }

    pub const fn total_mb(&self) -> u64 {
        self.available_mb + self.used_mb
    }

    pub fn usage_percentage(&self) -> f64 {
        if self.total_mb() == 0 {
            0.0
        } else {
            (self.used_mb as f64 / self.total_mb() as f64) * 100.0
        }
    }
}

#[cfg(test)]
#[expect(
    clippy::float_cmp,
    reason = "stats tests compare exact 0.0 defaults and integer-derived percentages as literals"
)]
mod local_pool_stats_tests {
    use super::{DatasetPerformanceStats, LocalMemoryInfo, PoolIoStats, PoolProperties};

    #[test]
    fn local_memory_info_total_and_usage() {
        let z = LocalMemoryInfo::new(0, 0);
        assert_eq!(z.total_mb(), 0);
        assert_eq!(z.usage_percentage(), 0.0);

        let m = LocalMemoryInfo::new(700, 300);
        assert_eq!(m.total_mb(), 1000);
        assert!((m.usage_percentage() - 30.0).abs() < 1e-9);
    }

    #[test]
    fn pool_io_stats_and_properties_default() {
        let p = PoolIoStats::default();
        assert_eq!(p.read_ops, 0);
        let props = PoolProperties::default();
        assert_eq!(props.fragmentation, 0.0);
        let d = DatasetPerformanceStats::default();
        assert_eq!(d.read_iops, 0.0);
    }
}

/// Pool I/O statistics
#[derive(Debug, Clone, Default)]
pub(crate) struct PoolIoStats {
    /// Read Ops
    pub read_ops: u64,
    /// Write Ops
    pub write_ops: u64,
    /// Bytes Read
    pub bytes_read: u64,
    /// Bytes Written
    pub bytes_written: u64,
}
/// Dataset performance statistics
#[derive(Debug, Clone)]
pub(crate) struct DatasetPerformanceStats {
    /// Read Iops
    pub read_iops: f64,
    /// Write Iops
    pub write_iops: f64,
    /// Read Throughput Mbs
    pub read_throughput_mbs: f64,
    /// Write Throughput Mbs
    pub write_throughput_mbs: f64,
    /// Read Latency Ms
    pub read_latency_ms: f64,
    /// Write Latency Ms
    pub write_latency_ms: f64,
    /// Utilization Percent
    pub utilization_percent: f64,
    /// Compression Effectiveness
    pub compression_effectiveness: f64,
    /// Deduplication Effectiveness
    pub deduplication_effectiveness: f64,
}
