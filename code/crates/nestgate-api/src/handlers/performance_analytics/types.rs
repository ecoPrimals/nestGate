//! Performance Analytics Types
//!
//! Core data structures and enums for performance monitoring and analytics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Performance analytics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Collection interval in seconds
    pub collection_interval: u64,
    /// Retention period in days
    pub retention_days: u32,
    /// Enable predictive analytics
    pub predictive_enabled: bool,
    /// Alert thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Alert threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU usage threshold (percentage)
    pub cpu_threshold: f64,
    /// Memory usage threshold (percentage)
    pub memory_threshold: f64,
    /// Disk usage threshold (percentage)
    pub disk_threshold: f64,
    /// Network latency threshold (ms)
    pub network_latency_threshold: f64,
    /// ZFS pool health threshold
    pub zfs_health_threshold: f64,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Timestamp of collection
    pub timestamp: DateTime<Utc>,
    /// CPU metrics
    pub cpu: CpuMetrics,
    /// Memory metrics
    pub memory: MemoryMetrics,
    /// Disk I/O metrics
    pub disk: DiskMetrics,
    /// Network metrics
    pub network: NetworkMetrics,
    /// ZFS specific metrics
    pub zfs: ZfsMetrics,
    /// Application metrics
    pub application: ApplicationMetrics,
}

/// CPU performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    /// Overall CPU usage percentage
    pub usage_percent: f64,
    /// Per-core usage percentages
    pub core_usage: Vec<f64>,
    /// Load averages (1min, 5min, 15min)
    pub load_average: [f64; 3],
    /// Context switches per second
    pub context_switches: u64,
    /// Interrupts per second
    pub interrupts: u64,
    /// CPU frequency (MHz)
    pub frequency: f64,
    /// Temperature (Celsius)
    pub temperature: Option<f64>,
}

/// Memory performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Total memory (bytes)
    pub total: u64,
    /// Available memory (bytes)
    pub available: u64,
    /// Used memory (bytes)
    pub used: u64,
    /// Free memory (bytes)
    pub free: u64,
    /// Cached memory (bytes)
    pub cached: u64,
    /// Buffer memory (bytes)
    pub buffers: u64,
    /// Swap total (bytes)
    pub swap_total: u64,
    /// Swap used (bytes)
    pub swap_used: u64,
    /// Usage percentage
    pub usage_percent: f64,
}

/// Disk I/O performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// Per-device metrics
    pub devices: HashMap<String, DiskDeviceMetrics>,
    /// Overall I/O wait percentage
    pub io_wait_percent: f64,
    /// Total read operations per second
    pub total_reads_per_sec: u64,
    /// Total write operations per second
    pub total_writes_per_sec: u64,
    /// Total read throughput (bytes/sec)
    pub total_read_throughput: u64,
    /// Total write throughput (bytes/sec)
    pub total_write_throughput: u64,
}

/// Per-device disk metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskDeviceMetrics {
    /// Device name
    pub device: String,
    /// Total space (bytes)
    pub total_space: u64,
    /// Used space (bytes)
    pub used_space: u64,
    /// Available space (bytes)
    pub available_space: u64,
    /// Usage percentage
    pub usage_percent: f64,
    /// Read operations per second
    pub reads_per_sec: u64,
    /// Write operations per second
    pub writes_per_sec: u64,
    /// Read throughput (bytes/sec)
    pub read_throughput: u64,
    /// Write throughput (bytes/sec)
    pub write_throughput: u64,
    /// Average queue depth
    pub queue_depth: f64,
    /// Average latency (ms)
    pub latency_ms: f64,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Per-interface metrics
    pub interfaces: HashMap<String, NetworkInterfaceMetrics>,
    /// Total bytes received per second
    pub total_rx_bytes_per_sec: u64,
    /// Total bytes transmitted per second
    pub total_tx_bytes_per_sec: u64,
    /// Total packets received per second
    pub total_rx_packets_per_sec: u64,
    /// Total packets transmitted per second
    pub total_tx_packets_per_sec: u64,
    /// Network errors per second
    pub errors_per_sec: u64,
}

/// Per-interface network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceMetrics {
    /// Interface name
    pub interface: String,
    /// Bytes received per second
    pub rx_bytes_per_sec: u64,
    /// Bytes transmitted per second
    pub tx_bytes_per_sec: u64,
    /// Packets received per second
    pub rx_packets_per_sec: u64,
    /// Packets transmitted per second
    pub tx_packets_per_sec: u64,
    /// Receive errors
    pub rx_errors: u64,
    /// Transmit errors
    pub tx_errors: u64,
    /// MTU size
    pub mtu: u32,
    /// Link speed (Mbps)
    pub speed: u64,
}

/// ZFS specific performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMetrics {
    /// Per-pool metrics
    pub pools: HashMap<String, ZfsPoolMetrics>,
    /// ARC (Adaptive Replacement Cache) metrics
    pub arc: ZfsArcMetrics,
    /// L2ARC metrics
    pub l2arc: Option<ZfsL2ArcMetrics>,
    /// ZIL (ZFS Intent Log) metrics
    pub zil: ZfsZilMetrics,
}

/// Per-ZFS pool metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolMetrics {
    /// Pool name
    pub pool: String,
    /// Pool health status
    pub health: String,
    /// Total capacity (bytes)
    pub capacity: u64,
    /// Used space (bytes)
    pub used: u64,
    /// Available space (bytes)
    pub available: u64,
    /// Usage percentage
    pub usage_percent: f64,
    /// Deduplication ratio
    pub dedup_ratio: f64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Read operations per second
    pub reads_per_sec: u64,
    /// Write operations per second
    pub writes_per_sec: u64,
    /// Read throughput (bytes/sec)
    pub read_throughput: u64,
    /// Write throughput (bytes/sec)
    pub write_throughput: u64,
    /// Fragmentation percentage
    pub fragmentation: f64,
}

/// ZFS ARC metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsArcMetrics {
    /// ARC size (bytes)
    pub size: u64,
    /// ARC target size (bytes)
    pub target_size: u64,
    /// ARC maximum size (bytes)
    pub max_size: u64,
    /// ARC hit ratio
    pub hit_ratio: f64,
    /// ARC miss ratio
    pub miss_ratio: f64,
    /// Recently used cache size
    pub mru_size: u64,
    /// Most frequently used cache size
    pub mfu_size: u64,
}

/// ZFS L2ARC metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsL2ArcMetrics {
    /// L2ARC size (bytes)
    pub size: u64,
    /// L2ARC hit ratio
    pub hit_ratio: f64,
    /// L2ARC miss ratio
    pub miss_ratio: f64,
    /// L2ARC reads per second
    pub reads_per_sec: u64,
    /// L2ARC writes per second
    pub writes_per_sec: u64,
}

/// ZFS ZIL metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsZilMetrics {
    /// ZIL commits per second
    pub commits_per_sec: u64,
    /// ZIL writes per second
    pub writes_per_sec: u64,
    /// ZIL sync writes per second
    pub sync_writes_per_sec: u64,
    /// ZIL throughput (bytes/sec)
    pub throughput: u64,
}

/// Application-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    /// NestGate API metrics
    pub api: ApiMetrics,
    /// Active connections
    pub active_connections: u64,
    /// Request rate (requests/sec)
    pub request_rate: f64,
    /// Error rate (errors/sec)
    pub error_rate: f64,
    /// Average response time (ms)
    pub avg_response_time: f64,
    /// Memory usage (bytes)
    pub memory_usage: u64,
    /// CPU usage percentage
    pub cpu_usage: f64,
}

/// API-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetrics {
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Active WebSocket connections
    pub websocket_connections: u64,
    /// Active SSE connections
    pub sse_connections: u64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Alert ID
    pub id: Uuid,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert message
    pub message: String,
    /// Metric that triggered the alert
    pub metric: String,
    /// Current value
    pub current_value: f64,
    /// Threshold value
    pub threshold_value: f64,
    /// Alert timestamp
    pub timestamp: DateTime<Utc>,
    /// Component that triggered the alert
    pub component: String,
    /// Suggested actions
    pub suggested_actions: Vec<String>,
}

/// Types of performance alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    /// Resource usage threshold exceeded
    ResourceUsage,
    /// Performance degradation detected
    PerformanceDegradation,
    /// System health issue
    HealthIssue,
    /// Predictive alert
    Predictive,
    /// Configuration optimization needed
    ConfigurationOptimization,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

/// Performance recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    /// Recommendation ID
    pub id: Uuid,
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Expected improvement
    pub expected_improvement: String,
    /// Implementation effort
    pub effort_level: EffortLevel,
    /// Priority
    pub priority: Priority,
    /// Actions to take
    pub actions: Vec<RecommendationAction>,
    /// Estimated impact
    pub estimated_impact: ImpactEstimate,
}

/// Types of performance recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    /// Hardware configuration
    Hardware,
    /// ZFS configuration
    ZfsConfiguration,
    /// Network optimization
    Network,
    /// Application tuning
    Application,
    /// Resource allocation
    ResourceAllocation,
}

/// Effort level for implementing recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
}

/// Recommendation priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Specific action in a recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationAction {
    /// Action description
    pub description: String,
    /// Command to execute (if applicable)
    pub command: Option<String>,
    /// Configuration changes required
    pub config_changes: Option<HashMap<String, String>>,
    /// Risk level
    pub risk_level: RiskLevel,
}

/// Risk level for recommendation actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Impact estimate for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactEstimate {
    /// Performance improvement percentage
    pub performance_improvement: f64,
    /// Resource usage reduction percentage
    pub resource_reduction: f64,
    /// Estimated timeframe for benefits
    pub timeframe: String,
    /// Confidence level (0-100)
    pub confidence: u8,
}

/// Query parameters for historical metrics
#[derive(Debug, Deserialize)]
pub struct HistoricalMetricsQuery {
    /// Hours of history to retrieve
    pub hours: Option<u32>,
    /// Specific metrics to include
    pub metrics: Option<String>,
    /// Aggregation interval (minutes)
    pub interval: Option<u32>,
} 