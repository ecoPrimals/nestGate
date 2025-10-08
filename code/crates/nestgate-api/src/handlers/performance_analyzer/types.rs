//! Type definitions for performance analysis

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Performance trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTrend {
    /// Performance is improving
    Improving,
    /// Performance is stable
    Stable,
    /// Performance is degrading
    Degrading,
    /// Not enough data to determine trend
    Unknown,
}

/// Component performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentAnalysis {
    /// Component name
    pub component_name: String,
    /// Current usage percentage
    pub current_usage: f64,
    /// Performance trend
    pub trend: PerformanceTrend,
    /// Detected anomalies
    pub anomalies: Vec<String>,
    /// Performance recommendations
    pub recommendations: Vec<String>,
}

/// Performance snapshot at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Configuration for performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysisConfig {
    /// Enable CPU monitoring
    pub enable_cpu_monitoring: bool,
    /// Enable memory monitoring
    pub enable_memory_monitoring: bool,
    /// Enable disk monitoring
    pub enable_disk_monitoring: bool,
    /// Enable network monitoring
    pub enable_network_monitoring: bool,
    /// Enable ZFS monitoring
    pub enable_zfs_monitoring: bool,
    /// Analysis interval in seconds
    pub analysis_interval_seconds: u64,
    /// Maximum history entries to keep
    pub max_history_entries: usize,
}

impl Default for PerformanceAnalysisConfig {
    fn default() -> Self {
        Self {
            enable_cpu_monitoring: true,
            enable_memory_monitoring: true,
            enable_disk_monitoring: true,
            enable_network_monitoring: true,
            enable_zfs_monitoring: true,
            analysis_interval_seconds: 30,
            max_history_entries: 1000,
        }
    }
}

/// Performance analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalysisReport {
    /// Report generation timestamp
    pub generated_at: DateTime<Utc>,
    /// Overall system health score (0-100)
    pub overall_health_score: f64,
    /// Performance trends
    pub trends: PerformanceTrends,
    /// Component analyses
    pub component_analyses: Vec<ComponentAnalysis>,
    /// Performance recommendations
    pub recommendations: Vec<PerformanceRecommendation>,
    /// Critical issues detected
    pub critical_issues: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
    /// System uptime in seconds
    pub system_uptime_seconds: u64,
    /// Analysis period start
    pub analysis_period_start: DateTime<Utc>,
    /// Analysis period end
    pub analysis_period_end: DateTime<Utc>,
}

/// Performance trends analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    /// CPU usage trend
    pub cpu_trend: PerformanceTrend,
    /// Memory usage trend
    pub memory_trend: PerformanceTrend,
    /// Disk I/O trend
    pub disk_trend: PerformanceTrend,
    /// Network I/O trend
    pub network_trend: PerformanceTrend,
    /// ZFS performance trend
    pub zfs_trend: PerformanceTrend,
}

/// CPU analysis details
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Performance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    /// Recommendation category
    pub category: String,
    /// Recommendation description
    pub description: String,
    /// Priority level (1-10, 10 being highest)
    pub priority: u8,
    /// Estimated impact
    pub estimated_impact: String,
}

/// Performance analyzer state
#[derive(Debug, Clone, Default)]
pub struct PerformanceAnalyzerState {
    /// Whether analysis is currently running
    pub is_running: bool,
    /// Last analysis timestamp
    pub last_analysis: Option<DateTime<Utc>>,
    /// Total analyses performed
    pub total_analyses: u64,
    /// Configuration
    pub config: PerformanceAnalysisConfig,
}
