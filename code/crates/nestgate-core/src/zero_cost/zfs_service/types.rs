//
// Data structures and configuration types for zero-cost ZFS operations.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Default pool information implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPoolInfo {
    pub name: String,
    pub status: String,
    pub capacity: u64,
    pub used: u64,
    pub available: u64,
    pub health: String,
}

/// Default dataset information implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultDatasetInfo {
    pub name: String,
    pub pool: String,
    pub used: u64,
    pub available: u64,
    pub compression: String,
    pub mountpoint: String,
}

/// Default snapshot information implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSnapshotInfo {
    pub name: String,
    pub dataset: String,
    pub created: SystemTime,
    pub used: u64,
}

/// Default health status implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultHealthStatus {
    pub status: String,
    pub pools_healthy: u32,
    pub datasets_healthy: u32,
    pub snapshots_healthy: u32,
    pub overall_health_percentage: f64,
    pub last_check: SystemTime,
    pub issues: Vec<String>,
}

/// Default service metrics implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultServiceMetrics {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub average_operation_time: Duration,
    pub uptime: Duration,
    pub pools_managed: u32,
    pub datasets_managed: u32,
    pub snapshots_managed: u32,
}

/// ZFS operation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsOperationStats {
    pub pool_operations: OperationTypeStats,
    pub dataset_operations: OperationTypeStats,
    pub snapshot_operations: OperationTypeStats,
    pub cache_stats: ZfsCacheStats,
}

/// Statistics for specific operation types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperationTypeStats {
    pub total: u64,
    pub successful: u64,
    pub failed: u64,
    pub average_duration_ms: f64,
}

/// ZFS cache statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsCacheStats {
    pub hit_rate: f64,
    pub miss_rate: f64,
    pub total_requests: u64,
    pub cache_size_bytes: u64,
    pub evictions: u64,
}

/// Default pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultPoolConfig {
    pub name: String,
    pub devices: Vec<String>,
    pub raid_level: String,
}

/// Default dataset configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultDatasetConfig {
    pub name: String,
    pub pool: String,
    pub compression: String,
}

/// Default snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSnapshotConfig {
    pub dataset: String,
    pub name: String,
    pub recursive: bool,
}
