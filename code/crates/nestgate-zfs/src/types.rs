//! ZFS type definitions
//!
//! Common types used across the ZFS system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Compression algorithms supported by ZFS
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// No compression
    Off,
    /// LZ4 compression (fast)
    Lz4,
    /// ZSTD compression (balanced)
    Zstd,
    /// GZIP compression (level 6)
    Gzip,
    /// GZIP compression (level 9, maximum)
    Gzip9,
}

impl Default for CompressionAlgorithm {
    fn default() -> Self {
        Self::Lz4
    }
}

impl std::fmt::Display for CompressionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "off"),
            Self::Lz4 => write!(f, "lz4"),
            Self::Zstd => write!(f, "zstd"),
            Self::Gzip => write!(f, "gzip"),
            Self::Gzip9 => write!(f, "gzip-9"),
        }
    }
}

/// ZFS dataset property
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetProperty {
    /// Property name
    pub name: String,
    /// Property value
    pub value: String,
}

impl DatasetProperty {
    /// Create a new dataset property
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

/// Storage tier for data classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageTier {
    /// Hot storage for frequently accessed data
    Hot,
    /// Warm storage for moderately accessed data
    Warm,
    /// Cold storage for infrequently accessed data
    Cold,
    /// Cache storage for ultra-fast access
    Cache,
}

impl Default for StorageTier {
    fn default() -> Self {
        Self::Warm
    }
}

impl std::fmt::Display for StorageTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hot => write!(f, "hot"),
            Self::Warm => write!(f, "warm"),
            Self::Cold => write!(f, "cold"),
            Self::Cache => write!(f, "cache"),
        }
    }
}

/// ZFS capabilities and feature support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsCapabilities {
    /// Supported compression algorithms
    pub compression_algorithms: Vec<CompressionAlgorithm>,
    /// Deduplication support
    pub deduplication_support: bool,
    /// Encryption support
    pub encryption_support: bool,
    /// Snapshot support
    pub snapshot_support: bool,
    /// Replication support
    pub replication_support: bool,
    /// Maximum pool size in bytes
    pub max_pool_size: u64,
    /// Maximum dataset count per pool
    pub max_datasets_per_pool: u32,
    /// ZFS version
    pub zfs_version: String,
}

/// Performance target for a storage tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPerformanceTarget {
    /// Target IOPS
    pub target_iops: u32,
    /// Target bandwidth in MB/s
    pub target_bandwidth_mbps: f64,
    /// Target latency in milliseconds
    pub target_latency_ms: f64,
    /// Target availability percentage
    pub target_availability: f64,
    /// Target durability (number of 9s)
    pub target_durability_nines: u32,
    /// Storage tier this target applies to
    pub tier: StorageTier,
}

impl Default for ZfsCapabilities {
    fn default() -> Self {
        Self {
            compression_algorithms: vec![
                CompressionAlgorithm::Lz4,
                CompressionAlgorithm::Zstd,
                CompressionAlgorithm::Gzip,
            ],
            deduplication_support: true,
            encryption_support: true,
            snapshot_support: true,
            replication_support: true,
            max_pool_size: 256 * 1024 * 1024 * 1024 * 1024, // 256TB
            max_datasets_per_pool: 1000,
            zfs_version: "2.1.0".to_string(),
        }
    }
}

impl Default for TierPerformanceTarget {
    fn default() -> Self {
        Self {
            target_iops: 1000,
            target_bandwidth_mbps: 100.0,
            target_latency_ms: 10.0,
            target_availability: 99.9,
            target_durability_nines: 11,
            tier: StorageTier::Hot,
        }
    }
}

// Conversion between nestgate-core::StorageTier and crate::types::StorageTier
impl From<nestgate_core::StorageTier> for StorageTier {
    fn from(tier: nestgate_core::StorageTier) -> Self {
        match tier {
            nestgate_core::StorageTier::Hot => Self::Hot,
            nestgate_core::StorageTier::Warm => Self::Warm,
            nestgate_core::StorageTier::Cold => Self::Cold,
            nestgate_core::StorageTier::Cache => Self::Cache,
        }
    }
}

impl From<StorageTier> for nestgate_core::StorageTier {
    fn from(tier: StorageTier) -> Self {
        match tier {
            StorageTier::Hot => Self::Hot,
            StorageTier::Warm => Self::Warm,
            StorageTier::Cold => Self::Cold,
            StorageTier::Cache => Self::Cache,
        }
    }
}

// Advanced features types

/// Advanced configuration for ZFS features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedConfig {
    /// Enable AI-powered features
    pub ai_enabled: bool,
    /// Enable predictive analytics
    pub predictive_analytics: bool,
    /// Enable intelligent replication
    pub intelligent_replication: bool,
    /// Enable advanced snapshots
    pub advanced_snapshots: bool,
}

impl Default for AdvancedConfig {
    fn default() -> Self {
        Self {
            ai_enabled: false,
            predictive_analytics: false,
            intelligent_replication: false,
            advanced_snapshots: false,
        }
    }
}

/// Capacity forecast results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityForecast {
    /// Dataset name
    pub dataset: String,
    /// Predicted usage percentage
    pub predicted_usage: f64,
    /// Confidence level (0.0-1.0)
    pub confidence: f64,
    /// Time horizon in days
    pub time_horizon: u64,
}

/// Bottleneck analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckReport {
    /// Identified bottlenecks
    pub bottlenecks: Vec<String>,
    /// Severity level
    pub severity: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Maintenance plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenancePlan {
    /// Dataset name
    pub dataset: String,
    /// Scheduled tasks
    pub scheduled_tasks: Vec<String>,
    /// Priority level
    pub priority: String,
    /// Estimated duration in minutes
    pub estimated_duration: u64,
}

/// Replication performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationPerformance {
    /// Transfer rate in MB/s
    pub transfer_rate: f64,
    /// Latency in milliseconds
    pub latency: f64,
    /// Error rate
    pub error_rate: f64,
    /// Throughput in operations per second
    pub throughput: f64,
}

impl Default for ReplicationPerformance {
    fn default() -> Self {
        Self {
            transfer_rate: 0.0,
            latency: 0.0,
            error_rate: 0.0,
            throughput: 0.0,
        }
    }
}

/// Replication optimization recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationOptimization {
    /// Source dataset
    pub source: String,
    /// Target datasets
    pub targets: Vec<String>,
    /// Recommended strategy
    pub recommended_strategy: String,
    /// Expected improvement percentage
    pub expected_improvement: f64,
    /// Implementation steps
    pub implementation_steps: Vec<String>,
}

/// Snapshot optimization recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotOptimization {
    /// Dataset name
    pub dataset: String,
    /// Retention recommendations
    pub retention_recommendations: Vec<String>,
    /// Cleanup candidates
    pub cleanup_candidates: Vec<String>,
    /// Expected space savings in bytes
    pub space_savings: u64,
}

/// Retention optimization recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionOptimization {
    /// Dataset name
    pub dataset: String,
    /// Optimized retention policy
    pub optimized_policy: RetentionPolicy,
    /// Expected space savings in bytes
    pub expected_savings: u64,
    /// Implementation plan
    pub implementation_plan: Vec<String>,
}

/// Retention policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Number of daily snapshots to keep
    pub daily_snapshots: u32,
    /// Number of weekly snapshots to keep
    pub weekly_snapshots: u32,
    /// Number of monthly snapshots to keep
    pub monthly_snapshots: u32,
    /// Number of yearly snapshots to keep
    pub yearly_snapshots: u32,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            daily_snapshots: 7,
            weekly_snapshots: 4,
            monthly_snapshots: 12,
            yearly_snapshots: 2,
        }
    }
}

/// Advanced feature analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFeatureReport {
    /// Pool name
    pub pool: String,
    /// Dataset name
    pub dataset: String,
    /// Features analyzed
    pub features: Vec<String>,
    /// Analysis results
    pub analysis: HashMap<String, String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Analysis timestamp
    pub timestamp: SystemTime,
}

/// System information for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Network I/O rate
    pub network_io: f64,
    /// Timestamp
    pub timestamp: SystemTime,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
            network_io: 0.0,
            timestamp: SystemTime::now(),
        }
    }
}
