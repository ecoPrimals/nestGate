//! Core type definitions for the NestGate REST API
//!
//! This module contains all the fundamental data structures used across
//! the NestGate REST API, including storage backends, compression types,
//! metrics, alerts, and configuration structures.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents different types of datasets in the storage system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatasetType {
    /// Standard filesystem dataset
    Filesystem,
    /// Volume dataset for block storage
    Volume,
    /// Snapshot dataset
    Snapshot,
    /// Bookmark dataset
    Bookmark,
}

/// Checksum algorithms supported by the storage system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChecksumType {
    /// Fletcher checksum algorithm
    Fletcher2,
    /// Fletcher4 checksum algorithm (default)
    Fletcher4,
    /// SHA-256 cryptographic hash
    Sha256,
    /// SHA-512 cryptographic hash
    Sha512,
    /// Skein hash algorithm
    Skein,
    /// Edon-R hash algorithm
    EdonR,
}

/// Status of a dataset in the storage system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatasetStatus {
    /// Dataset is online and accessible
    Online,
    /// Dataset is offline or unavailable
    Offline,
    /// Dataset is in degraded state
    Degraded,
    /// Dataset is in maintenance mode
    Maintenance,
    /// Dataset has encountered an error
    Error,
}

/// Alert severity levels for system monitoring
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    /// Informational message
    Info,
    /// Warning condition
    Warning,
    /// Critical condition requiring attention
    Critical,
    /// Emergency condition requiring immediate action
    Emergency,
}

/// Status of an alert in the monitoring system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertStatus {
    /// Alert is active and firing
    Active,
    /// Alert condition has been resolved
    Resolved,
    /// Alert has been acknowledged by operator
    Acknowledged,
    /// Alert has been suppressed
    Suppressed,
}

/// Comparison operators for alert conditions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// Greater than comparison
    GreaterThan,
    /// Less than comparison
    LessThan,
    /// Equal to comparison
    Equal,
    /// Not equal to comparison
    NotEqual,
    /// Greater than or equal to comparison
    GreaterThanOrEqual,
    /// Less than or equal to comparison
    LessThanOrEqual,
}

/// Storage backend types supported by the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageBackendType {
    /// In-memory storage backend
    Memory,
    /// Local filesystem storage
    Local,
    /// Remote storage backend
    Remote,
    /// Filesystem-based storage
    Filesystem,
    /// Cloud storage backend
    Cloud,
    /// Network-attached storage
    Network,
    /// Block storage backend
    Block,
    /// File-based storage
    File,
}

/// Compression algorithms supported by the storage system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionType {
    /// LZ4 compression algorithm (fast)
    Lz4,
    /// Gzip compression algorithm
    Gzip,
    /// Zstandard compression algorithm (balanced)
    Zstd,
    /// No compression
    None,
}

/// Status of snapshots in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SnapshotStatus {
    /// Snapshot is active and available
    Active,
    /// Snapshot creation is pending
    Pending,
    /// Snapshot creation or operation failed
    Failed,
    /// Snapshot has been deleted
    Deleted,
}

/// Storage performance and capacity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total storage capacity in bytes
    pub total_bytes: u64,
    /// Used storage space in bytes
    pub used_bytes: u64,
    /// Available storage space in bytes
    pub available_bytes: u64,
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
}

/// Network I/O performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIoMetrics {
    /// Total bytes sent over network
    pub bytes_sent: u64,
    /// Total bytes received over network
    pub bytes_received: u64,
    /// Total packets sent over network
    pub packets_sent: u64,
    /// Total packets received over network
    pub packets_received: u64,
    /// Receive throughput in bytes per second
    pub rx_bytes_per_sec: f64,
    /// Transmit throughput in bytes per second
    pub tx_bytes_per_sec: f64,
    /// Receive packet rate per second
    pub rx_packets_per_sec: f64,
    /// Transmit packet rate per second
    pub tx_packets_per_sec: f64,
}

/// Disk I/O performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIoMetrics {
    /// Read throughput in bytes per second
    pub read_bytes_per_sec: f64,
    /// Write throughput in bytes per second
    pub write_bytes_per_sec: f64,
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
    /// Read throughput in megabytes per second
    pub read_mbps: f64,
    /// Write throughput in megabytes per second
    pub write_mbps: f64,
    /// Read IOPS (Input/Output Operations Per Second)
    pub read_iops: f64,
    /// Write IOPS (Input/Output Operations Per Second)
    pub write_iops: f64,
    /// Average I/O queue depth
    pub avg_queue_depth: f64,
}

/// System alert with monitoring information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Unique identifier for the alert
    pub id: String,
    /// Human-readable name of the alert
    pub name: String,
    /// Detailed description of the alert condition
    pub description: String,
    /// Alert message text
    pub message: String,
    /// Severity level of the alert
    pub severity: AlertSeverity,
    /// Current status of the alert
    pub status: AlertStatus,
    /// Timestamp when alert was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when alert condition was triggered
    pub triggered_at: DateTime<Utc>,
    /// Recommended actions to resolve the alert
    pub suggested_actions: Vec<String>,
    /// Conditions that must be met for this alert
    pub conditions: Vec<AlertCondition>,
}

/// Dashboard-specific alert representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAlert {
    /// Unique identifier for the dashboard alert
    pub id: String,
    /// Title displayed in the dashboard
    pub title: String,
    /// Message content for the dashboard
    pub message: String,
    /// Type/category of the alert
    pub alert_type: AlertType,
    /// Severity level for dashboard display
    pub severity: AlertSeverity,
    /// Timestamp when alert was created
    pub created_at: DateTime<Utc>,
}

/// Condition that triggers an alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    /// Name of the metric being monitored
    pub metric_name: String,
    /// Comparison operator for the condition
    pub operator: ComparisonOperator,
    /// Threshold value for the condition
    pub threshold: f64,
    /// Duration in seconds the condition must persist
    pub duration_seconds: u32,
    /// Current value of the metric
    pub currentvalue: f64,
}

/// Categories of alerts in the monitoring system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertType {
    /// System-level alerts (CPU, memory, etc.)
    System,
    /// Storage-related alerts
    Storage,
    /// Network-related alerts
    Network,
    /// Performance-related alerts
    Performance,
}

/// Circuit breaker configuration for fault tolerance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening the circuit
    pub failure_threshold: u32,
    /// Timeout in seconds before attempting to close circuit
    pub timeout_seconds: u32,
    /// Reset timeout in seconds for circuit breaker
    pub reset_timeout_seconds: u32,
}

/// Timeout configuration for network operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Connection timeout in milliseconds
    pub connect_timeout_ms: u32,
    /// Request timeout in milliseconds
    pub request_timeout_ms: u32,
    /// Idle connection timeout in milliseconds
    pub idle_timeout_ms: u32,
}

/// Retry policy configuration for failed operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Initial delay in milliseconds before first retry
    pub initial_delay_ms: u32,
    /// Maximum delay in milliseconds between retries
    pub max_delay_ms: u32,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
}

/// Context for validation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationContext {
    /// Whether to enforce strict validation rules
    pub strict_mode: bool,
    /// Whether to allow potentially unsafe names
    pub allow_unsafe_names: bool,
}

/// System performance and resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage as a percentage (0.0 to 100.0)
    pub cpu_usage_percent: f64,
    /// Memory usage as a percentage (0.0 to 100.0)
    pub memory_usage_percent: f64,
    /// System load average
    pub load_average: f64,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Timestamp when metrics were collected
    pub timestamp: DateTime<Utc>,
    /// Disk I/O performance metrics
    pub disk_io: DiskIoMetrics,
    /// Network I/O performance metrics
    pub network_io: NetworkIoMetrics,
    /// ZFS-specific storage metrics
    pub zfs_metrics: ZfsMetrics,
}

/// ZFS-specific performance and status metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMetrics {
    /// ARC hit ratio as a percentage
    pub arc_hit_ratio: f64,
    /// ARC size in bytes
    pub arc_size_bytes: u64,
    /// ARC target size in bytes
    pub arc_target_size_bytes: u64,
    /// Read throughput in MB/s
    pub read_throughput_mbps: f64,
    /// Write throughput in MB/s
    pub write_throughput_mbps: f64,
    /// Compression ratio achieved
    pub compression_ratio: f64,
    /// Deduplication ratio achieved
    pub deduplication_ratio: f64,
    /// Total number of datasets
    pub total_datasets: u32,
    /// Total number of snapshots
    pub total_snapshots: u32,
    /// Total used space in bytes
    pub total_used_bytes: u64,
}
