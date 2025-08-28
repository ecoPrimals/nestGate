//
// Data structures for the NestGate REST API, designed for clean JSON serialization
// and consumption by biomeOS and other management systems.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== SECTION ====================
// ZFS DATASET MODELS
// ==================== SECTION ====================

/// ZFS Dataset representation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dataset {
    /// Dataset name (e.g., "tank/data")
    pub name: String,
    /// Dataset type (filesystem, volume)
    pub dataset_type: DatasetType,
    /// Storage backend type
    pub backend: StorageBackendType,
    /// Dataset properties
    pub properties: DatasetProperties,
    /// Current statistics
    pub stats: DatasetStats,
    /// Creation timestamp
    pub created: DateTime<Utc>,
    /// Last modified timestamp
    pub modified: DateTime<Utc>,
    /// Dataset status
    pub status: DatasetStatus,
    /// Available snapshots count
    pub snapshot_count: u32,
}

/// Dataset type enumeration
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DatasetType {
    /// Standard filesystem dataset for file storage
    Filesystem,
    /// Block volume dataset for raw storage
    Volume,
}

/// Dataset status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DatasetStatus {
    /// Dataset is healthy and accessible
    Online,
    /// Dataset is not accessible
    Offline,
    /// Dataset is accessible but with reduced performance/reliability
    Degraded,
    /// Dataset has critical errors and may be corrupted
    Faulted,
}

/// Dataset properties
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetProperties {
    /// Compression enabled
    pub compression: bool,
    /// Compression type
    pub compression_type: Option<CompressionType>,
    /// Checksumming enabled
    pub checksum: bool,
    /// Checksum type
    pub checksum_type: Option<ChecksumType>,
    /// Deduplication enabled
    pub deduplication: bool,
    /// Encryption enabled
    pub encryption: bool,
    /// Read-only mode
    pub readonly: bool,
    /// Custom properties
    pub custom: HashMap<String, String>,
}

/// Compression types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CompressionType {
    /// Fast LZ4 compression algorithm
    Lz4,
    /// High-ratio Zstandard compression
    Zstd,
    /// Standard Gzip compression
    Gzip,
    /// No compression applied
    None,
}

/// Checksum types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChecksumType {
    /// SHA-256 cryptographic hash
    Sha256,
    /// BLAKE3 cryptographic hash (fast and secure)
    Blake3,
    /// CRC-32 cyclic redundancy check (fast, non-cryptographic)
    Crc32,
    /// No checksum verification
    None,
}

/// Dataset statistics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatasetStats {
    /// Total bytes used
    pub used_bytes: u64,
    /// Total bytes available
    pub available_bytes: u64,
    /// Files written
    pub files_written: u64,
    /// Files read
    pub files_read: u64,
    /// COW operations performed
    pub cow_operations: u64,
    /// Blocks copied due to COW
    pub blocks_copied: u64,
    /// Compression ratio (if enabled)
    pub compression_ratio: Option<f64>,
    /// Space saved by compression (bytes)
    pub compression_space_saved: Option<u64>,
    /// Checksums computed
    pub checksums_computed: u64,
    /// Checksums verified
    pub checksums_verified: u64,
    /// Read throughput (bytes/sec)
    pub read_throughput: f64,
    /// Write throughput (bytes/sec)
    pub write_throughput: f64,
    /// Average latency (milliseconds)
    pub avg_latency_ms: f64,
}

/// Request to create a new dataset
#[derive(Debug, Deserialize)]
pub struct CreateDatasetRequest {
    /// Dataset name
    pub name: String,
    /// Storage backend type
    pub backend: StorageBackendType,
    /// Backend-specific configuration
    pub backend_config: Option<serde_json::Value>,
    /// Dataset properties
    pub properties: Option<DatasetProperties>,
    /// Base path for filesystem backends
    pub path: Option<String>,
}

/// Request to update dataset properties
#[derive(Debug, Deserialize)]
pub struct UpdateDatasetRequest {
    /// Properties to update
    pub properties: DatasetProperties,
}

// ==================== SECTION ====================
// ZFS SNAPSHOT MODELS
// ==================== SECTION ====================

/// ZFS Snapshot representation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Snapshot {
    /// Snapshot ID
    pub id: String,
    /// Snapshot name (e.g., "backup-2025-01-30")
    pub name: String,
    /// Parent dataset name
    pub dataset: String,
    /// Creation timestamp
    pub created: DateTime<Utc>,
    /// Snapshot size in bytes
    pub size_bytes: u64,
    /// Unique data size (not shared with other snapshots)
    pub unique_bytes: u64,
    /// Number of files at snapshot time
    pub file_count: u64,
    /// Snapshot status
    pub status: SnapshotStatus,
    /// User-provided description
    pub description: Option<String>,
    /// Tags for organization
    pub tags: Vec<String>,
}

/// Snapshot status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotStatus {
    /// Snapshot is active and available
    Active,
    /// Snapshot is being destroyed
    Destroying,
    /// Snapshot is in an error state
    Error,
}

/// Request to create a new snapshot
#[derive(Debug, Deserialize)]
pub struct CreateSnapshotRequest {
    /// Snapshot name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Optional tags
    pub tags: Option<Vec<String>>,
}

/// Request to clone a snapshot
#[derive(Debug, Deserialize)]
pub struct CloneSnapshotRequest {
    /// New dataset name for the clone
    pub clone_name: String,
    /// Optional properties for the clone
    pub properties: Option<DatasetProperties>,
}

// ==================== SECTION ====================
// STORAGE BACKEND MODELS
// ==================== SECTION ====================

/// Storage backend types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackendType {
    /// In-memory storage backend
    Memory,
    /// Local filesystem backend
    Filesystem,
    /// Cloud storage backend (S3, Azure, etc.)
    Cloud,
    /// Network-attached storage backend
    Network,
    /// Block device backend
    Block,
}

/// Available storage backend
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageBackend {
    /// Backend type
    pub backend_type: StorageBackendType,
    /// Backend name/identifier
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Available capacity in bytes
    pub available_bytes: u64,
    /// Total capacity in bytes
    pub total_bytes: u64,
    /// Backend capabilities
    pub capabilities: Vec<StorageCapability>,
    /// Performance characteristics
    pub performance: StoragePerformance,
    /// Backend status
    pub status: StorageBackendStatus,
    /// Configuration options
    pub config_schema: Option<serde_json::Value>,
}

/// Storage capabilities
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StorageCapability {
    /// Basic read/write operations
    BasicOperations,
    /// Data compression support
    Compression,
    /// Data encryption support
    Encryption,
    /// Snapshot creation and management
    Snapshots,
    /// Data replication across nodes
    Replication,
    /// Backup and restore functionality
    Backup,
    /// Data integrity checksumming
    Checksumming,
    /// Data deduplication to save space
    Deduplication,
    /// Horizontally scalable storage
    Scalable,
    /// Persistent, durable storage
    Durable,
    /// Temporary, volatile storage
    Volatile,
    /// Atomic write operations
    AtomicWrites,
}

/// Storage performance characteristics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoragePerformance {
    /// Read throughput (MB/s)
    pub read_throughput_mbps: f64,
    /// Write throughput (MB/s)
    pub write_throughput_mbps: f64,
    /// Average latency (milliseconds)
    pub avg_latency_ms: f64,
    /// IOPS (operations per second)
    pub iops: u64,
    /// Performance tier (high/medium/low)
    pub tier: PerformanceTier,
}

/// Performance tiers
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PerformanceTier {
    /// High-performance tier (low latency, high throughput)
    High,
    /// Medium-performance tier (balanced performance)
    Medium,
    /// Low-performance tier (high latency, lower cost)
    Low,
}

/// Storage backend status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackendStatus {
    /// Backend is online and operational
    Online,
    /// Backend is offline and unavailable
    Offline,
    /// Backend is online but with reduced functionality
    Degraded,
    /// Backend is under maintenance
    Maintenance,
}

/// Request to scan for available storage
#[derive(Debug, Deserialize)]
pub struct ScanStorageRequest {
    /// Base path to scan
    pub path: Option<String>,
    /// Include cloud storage
    pub include_cloud: Option<bool>,
    /// Include network storage
    pub include_network: Option<bool>,
    /// Include block devices
    pub include_block: Option<bool>,
}

/// Request to benchmark storage
#[derive(Debug, Deserialize)]
pub struct BenchmarkStorageRequest {
    /// Backend to benchmark
    pub backend: StorageBackendType,
    /// Backend configuration
    pub config: Option<serde_json::Value>,
    /// Test duration in seconds
    pub duration_seconds: Option<u64>,
    /// Test file size in MB
    pub test_size_mb: Option<u64>,
}

/// Storage benchmark results
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkResults {
    /// Backend that was tested
    pub backend: StorageBackendType,
    /// Test duration
    pub duration_seconds: u64,
    /// Test file size
    pub test_size_mb: u64,
    /// Read performance
    pub read_performance: PerformanceMetrics,
    /// Write performance
    pub write_performance: PerformanceMetrics,
    /// Mixed workload performance
    pub mixed_performance: Option<PerformanceMetrics>,
    /// Test timestamp
    pub timestamp: DateTime<Utc>,
}

/// Performance metrics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceMetrics {
    /// Throughput in MB/s
    pub throughput_mbps: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// 95th percentile latency
    pub p95_latency_ms: f64,
    /// 99th percentile latency
    pub p99_latency_ms: f64,
    /// IOPS
    pub iops: u64,
}

// ==================== SECTION ====================
// AUTO-CONFIGURATION MODELS
// ==================== SECTION ====================

/// Auto-configuration request
#[derive(Debug, Deserialize)]
pub struct AutoConfigRequest {
    /// Use case for the storage
    pub use_case: UseCase,
    /// Minimum capacity required (GB)
    pub min_capacity_gb: Option<u64>,
    /// Performance requirements
    pub performance_requirements: Option<PerformanceRequirements>,
    /// Budget constraints
    pub budget_constraints: Option<BudgetConstraints>,
    /// Redundancy requirements
    pub redundancy_level: Option<RedundancyLevel>,
    /// Preferred features
    pub preferred_features: Option<Vec<StorageCapability>>,
}

/// Use cases for auto-configuration
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UseCase {
    /// Development and testing workloads
    Development,
    /// Home NAS file storage
    HomeNas,
    /// Database storage with ACID requirements
    Database,
    /// Long-term archival storage
    Archive,
    /// Media streaming and content delivery
    MediaStreaming,
    /// Backup and disaster recovery
    Backup,
    /// High-performance computing workloads
    HighPerformance,
}

/// Performance requirements
#[derive(Debug, Deserialize)]
pub struct PerformanceRequirements {
    /// Minimum throughput (MB/s)
    pub min_throughput_mbps: Option<f64>,
    /// Maximum acceptable latency (ms)
    pub max_latency_ms: Option<f64>,
    /// Minimum IOPS
    pub min_iops: Option<u64>,
    /// Performance priority vs cost
    pub performance_priority: bool,
}

/// Budget constraints
#[derive(Debug, Deserialize)]
pub struct BudgetConstraints {
    /// Maximum monthly cost
    pub max_monthly_cost: Option<f64>,
    /// Cost optimization priority
    pub cost_optimization: bool,
}

/// Redundancy levels
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RedundancyLevel {
    /// No redundancy (single disk failure causes data loss)
    None,
    /// Mirror redundancy (can survive 1 disk failure)
    Mirror,
    /// RAID-Z1 with single parity (can survive 1 disk failure)
    RaidZ1,
    /// RAID-Z2 with double parity (can survive 2 disk failures)
    RaidZ2,
    /// RAID-Z3 with triple parity (can survive 3 disk failures)
    RaidZ3,
}

/// Auto-configuration result
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoConfigResult {
    /// Recommended storage configuration
    pub recommended_config: StorageConfiguration,
    /// Alternative configurations
    pub alternatives: Vec<StorageConfiguration>,
    /// Configuration rationale
    pub rationale: String,
    /// Estimated costs
    pub cost_estimate: CostEstimate,
    /// Performance projections
    pub performance_projection: PerformanceProjection,
}

/// Storage configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageConfiguration {
    /// Configuration name
    pub name: String,
    /// Storage tiers
    pub tiers: Vec<StorageTier>,
    /// Total capacity (GB)
    pub total_capacity_gb: u64,
    /// Redundancy configuration
    pub redundancy: RedundancyLevel,
    /// Enabled features
    pub features: Vec<StorageCapability>,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
}

/// Storage tier configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StorageTier {
    /// Tier name
    pub name: String,
    /// Backend type
    pub backend: StorageBackendType,
    /// Tier capacity (GB)
    pub capacity_gb: u64,
    /// Tier purpose
    pub purpose: String,
    /// Performance characteristics
    pub performance: StoragePerformance,
}

/// Cost estimation
#[derive(Debug, Serialize, Deserialize)]
pub struct CostEstimate {
    /// Initial setup cost
    pub setup_cost: f64,
    /// Monthly operational cost
    pub monthly_cost: f64,
    /// Cost per GB per month
    pub cost_per_gb_monthly: f64,
    /// Cost breakdown by component
    pub breakdown: HashMap<String, f64>,
}

/// Performance projection
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceProjection {
    /// Expected throughput (MB/s)
    pub expected_throughput_mbps: f64,
    /// Expected latency (ms)
    pub expected_latency_ms: f64,
    /// Expected IOPS
    pub expected_iops: u64,
    /// Scalability characteristics
    pub scalability: String,
}

// ==================== SECTION ====================
// MONITORING MODELS
// ==================== SECTION ====================

/// System metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Disk I/O metrics
    pub disk_io: DiskIoMetrics,
    /// Network I/O metrics
    pub network_io: NetworkIoMetrics,
    /// ZFS-specific metrics
    pub zfs_metrics: ZfsMetrics,
}

/// Disk I/O metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct DiskIoMetrics {
    /// Read throughput (MB/s)
    pub read_mbps: f64,
    /// Write throughput (MB/s)
    pub write_mbps: f64,
    /// Read IOPS
    pub read_iops: u64,
    /// Write IOPS
    pub write_iops: u64,
    /// Average queue depth
    pub avg_queue_depth: f64,
}

/// Network I/O metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkIoMetrics {
    /// Bytes received per second
    pub rx_bytes_per_sec: u64,
    /// Bytes transmitted per second
    pub tx_bytes_per_sec: u64,
    /// Packets received per second
    pub rx_packets_per_sec: u64,
    /// Packets transmitted per second
    pub tx_packets_per_sec: u64,
}

/// ZFS-specific metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct ZfsMetrics {
    /// Total datasets
    pub total_datasets: u32,
    /// Total snapshots
    pub total_snapshots: u32,
    /// Total space used (bytes)
    pub total_used_bytes: u64,
    /// Total space available (bytes)
    pub total_available_bytes: u64,
    /// Compression ratio across all datasets
    pub overall_compression_ratio: f64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
}

/// Alert definition
#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: String,
    /// Alert name
    pub name: String,
    /// Alert description
    pub description: String,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert status
    pub status: AlertStatus,
    /// When the alert was triggered
    pub triggered_at: DateTime<Utc>,
    /// Alert conditions
    pub conditions: Vec<AlertCondition>,
    /// Suggested actions
    pub suggested_actions: Vec<String>,
}

/// Alert severity levels
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AlertSeverity {
    /// Critical alert requiring immediate attention
    Critical,
    /// Warning alert indicating potential issues
    Warning,
    /// Informational alert for awareness
    Info,
}

/// Alert status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AlertStatus {
    /// Alert is active and requires attention
    Active,
    /// Alert has been acknowledged by an operator
    Acknowledged,
    /// Alert has been resolved
    Resolved,
}

/// Alert condition
#[derive(Debug, Serialize, Deserialize)]
pub struct AlertCondition {
    /// Metric name
    pub metric: String,
    /// Comparison operator
    pub operator: ComparisonOperator,
    /// Threshold value
    pub threshold: f64,
    /// Current value
    pub current_value: f64,
}

/// Comparison operators for alerts
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    /// Greater than comparison (>)
    GreaterThan,
    /// Less than comparison (<)
    LessThan,
    /// Equals comparison (==)
    Equals,
    /// Not equals comparison (!=)
    NotEquals,
    /// Greater than or equal comparison (>=)
    GreaterThanOrEqual,
    /// Less than or equal comparison (<=)
    LessThanOrEqual,
}

// ==================== SECTION ====================
// CONFIGURATION MODELS
// ==================== SECTION ====================

/// System configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Storage configuration
    pub storage: StorageConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Performance configuration
    pub performance: PerformanceConfig,
}

/// Storage configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Default backend type
    pub default_backend: StorageBackendType,
    /// Default compression
    pub default_compression: bool,
    /// Default checksum
    pub default_checksum: bool,
    /// Auto-snapshot configuration
    pub auto_snapshots: AutoSnapshotConfig,
}

/// Auto-snapshot configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct AutoSnapshotConfig {
    /// Enable auto-snapshots
    pub enabled: bool,
    /// Snapshot frequency
    pub frequency: SnapshotFrequency,
    /// Retention policy
    pub retention_days: u32,
    /// Naming pattern
    pub naming_pattern: String,
}

/// Snapshot frequency
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotFrequency {
    /// Take snapshots every hour
    Hourly,
    /// Take snapshots daily
    Daily,
    /// Take snapshots weekly
    Weekly,
    /// Take snapshots monthly
    Monthly,
}

/// Monitoring configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Metrics collection interval (seconds)
    pub metrics_interval_seconds: u64,
    /// Metrics retention days
    pub metrics_retention_days: u32,
    /// Alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

/// Security configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// JWT secret key
    pub jwt_secret: String,
    /// Token expiration time (hours)
    pub token_expiration_hours: u32,
    /// API rate limiting
    pub rate_limiting: RateLimitConfig,
}

/// Rate limiting configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
}

/// Performance configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Worker thread count
    pub worker_threads: u32,
    /// Connection pool size
    pub connection_pool_size: u32,
    /// Cache size (MB)
    pub cache_size_mb: u64,
}
