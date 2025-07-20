//! Performance monitoring types and data structures

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::{mpsc, RwLock};

use crate::{ZfsDatasetManager, ZfsPoolManager};
use nestgate_core::StorageTier;

/// ZFS performance monitor
#[derive(Debug)]
#[allow(dead_code)] // Fields used in comprehensive performance monitoring
pub struct ZfsPerformanceMonitor {
    pub config: PerformanceConfig,
    pub pool_manager: Arc<ZfsPoolManager>,
    pub dataset_manager: Arc<ZfsDatasetManager>,

    /// Real-time metrics
    pub current_metrics: Arc<RwLock<CurrentPerformanceMetrics>>,
    /// Historical metrics
    pub metrics_history: Arc<RwLock<VecDeque<PerformanceSnapshot>>>,
    /// Tier-specific metrics
    pub tier_metrics: Arc<RwLock<HashMap<StorageTier, TierPerformanceData>>>,
    /// Alert conditions
    pub alert_conditions: Arc<RwLock<Vec<AlertCondition>>>,
    /// Active alerts
    pub active_alerts: Arc<RwLock<Vec<ActiveAlert>>>,

    /// Background tasks
    pub collection_task: Option<tokio::task::JoinHandle<()>>,
    pub analysis_task: Option<tokio::task::JoinHandle<()>>,
    pub alert_task: Option<tokio::task::JoinHandle<()>>,

    /// Alert notification channel
    pub alert_sender: Option<mpsc::Sender<Alert>>,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Metrics collection interval in seconds
    pub collection_interval: u64,
    /// Analysis interval in seconds
    pub analysis_interval: u64,
    /// Alert check interval in seconds
    pub alert_interval: u64,
    /// History retention period in hours
    pub history_retention_hours: u64,
    /// Maximum history entries to keep
    pub max_history_entries: usize,
    /// Enable real-time alerting
    pub enable_alerting: bool,
    /// Enable trend analysis
    pub enable_trend_analysis: bool,
    /// Prometheus metrics endpoint
    pub prometheus_endpoint: Option<String>,
}

/// Current performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct ActiveAlert {
    /// Alert condition ID
    pub condition_id: String,
    /// Alert name
    pub name: String,
    /// Alert description
    pub description: String,
    /// Current value
    pub current_value: f64,
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
pub struct Alert {
    /// Alert type
    pub alert_type: AlertType,
    /// Alert data
    pub data: ActiveAlert,
}

/// Alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub read_ops: u64,
    pub write_ops: u64,
    pub read_throughput_mbs: f64,
    pub write_throughput_mbs: f64,
    pub read_latency_ms: f64,
    pub write_latency_ms: f64,
}

/// Pool properties
#[derive(Debug, Clone)]
pub(crate) struct PoolProperties {
    pub fragmentation: f64,
    pub compression_ratio: f64,
    pub dedup_ratio: f64,
}

/// System memory information
#[derive(Debug)]
pub(crate) struct LocalMemoryInfo {
    pub available_mb: u64,
    pub used_mb: u64,
}

/// Pool I/O statistics
#[derive(Debug, Clone, Default)]
pub(crate) struct PoolIoStats {
    pub read_ops: u64,
    pub write_ops: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
}

/// Dataset performance statistics
#[derive(Debug, Clone)]
pub(crate) struct DatasetPerformanceStats {
    pub read_iops: f64,
    pub write_iops: f64,
    pub read_throughput_mbs: f64,
    pub write_throughput_mbs: f64,
    pub read_latency_ms: f64,
    pub write_latency_ms: f64,
    pub utilization_percent: f64,
    pub compression_effectiveness: f64,
    pub deduplication_effectiveness: f64,
}
