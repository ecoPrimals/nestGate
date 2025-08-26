//
// ZFS-related configuration including pools, datasets, snapshots, and performance.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// ZFS configuration (consolidates 10+ ZFS configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ZfsConfig {
    /// Pool configurations
    pub pools: Vec<PoolConfig>,
    /// Dataset configurations
    pub datasets: Vec<DatasetConfig>,
    /// Snapshot configurations
    pub snapshots: SnapshotConfig,
    /// Performance tuning
    pub performance: ZfsPerformanceConfig,
    /// Fail-safe configuration
    pub failsafe: FailSafeConfig,
    /// Tiering configuration
    pub tiering: TieringConfig,
}

/// Pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Pool name
    pub name: String,
    /// Pool type (mirror, raidz1, raidz2, raidz3)
    pub pool_type: String,
    /// Device paths
    pub devices: Vec<PathBuf>,
    /// Mount point
    pub mountpoint: PathBuf,
    /// Compression algorithm
    pub compression: String,
    /// Enable deduplication
    pub deduplication: bool,
    /// Enable encryption
    pub encryption: bool,
}

/// Dataset configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    /// Dataset name
    pub name: String,
    /// Parent pool
    pub pool: String,
    /// Mount point
    pub mountpoint: PathBuf,
    /// Quota (bytes)
    pub quota: Option<u64>,
    /// Reservation (bytes)
    pub reservation: Option<u64>,
    /// Compression algorithm
    pub compression: String,
    /// Record size
    pub record_size: String,
}

/// Snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    /// Enable automatic snapshots
    pub auto_snapshot: bool,
    /// Snapshot interval
    pub interval: Duration,
    /// Retention policy
    pub retention_days: u32,
}

/// ZFS performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceConfig {
    /// ARC size limit
    pub arc_max_size: Option<u64>,
    /// ZIL sync policy
    pub sync_policy: String,
    /// Compression algorithm
    pub compression: String,
}

/// Fail-safe configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailSafeConfig {
    /// Enable fail-safe mode
    pub enabled: bool,
    /// Fallback storage path
    pub fallback_path: PathBuf,
    /// Health check interval
    pub health_check_interval: Duration,
}

/// Tiering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieringConfig {
    /// Enable automatic tiering
    pub enabled: bool,
    /// Hot tier threshold
    pub hot_threshold: Duration,
    /// Cold tier threshold
    pub cold_threshold: Duration,
}


impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            auto_snapshot: true,
            interval: Duration::from_secs(3600), // Hourly
            retention_days: 7,
        }
    }
}

impl Default for ZfsPerformanceConfig {
    fn default() -> Self {
        Self {
            arc_max_size: None,
            sync_policy: "standard".to_string(),
            compression: "lz4".to_string(),
        }
    }
}

impl Default for FailSafeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            fallback_path: PathBuf::from("/tmp/nestgate-fallback"),
            health_check_interval: Duration::from_secs(30),
        }
    }
}

impl Default for TieringConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            hot_threshold: Duration::from_secs(86400), // 1 day
            cold_threshold: Duration::from_secs(2592000), // 30 days
        }
    }
} 