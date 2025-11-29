//
// Data structures and configuration types for zero-cost ZFS operations.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Default pool information implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Defaultpoolinfo
pub struct DefaultPoolInfo {
    /// Name
    pub name: String,
    /// Status
    pub status: String,
    /// Capacity
    pub capacity: u64,
    /// Used
    pub used: u64,
    /// Available
    pub available: u64,
    /// Health
    pub health: String,
}
/// Default dataset information implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Defaultdatasetinfo
pub struct DefaultDatasetInfo {
    /// Name
    pub name: String,
    /// Pool
    pub pool: String,
    /// Used
    pub used: u64,
    /// Available
    pub available: u64,
    /// Compression
    pub compression: String,
    /// Mountpoint
    pub mountpoint: String,
}
/// Default snapshot information implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Defaultsnapshotinfo
pub struct DefaultSnapshotInfo {
    /// Name
    pub name: String,
    /// Dataset
    pub dataset: String,
    /// Created
    pub created: SystemTime,
    /// Used
    pub used: u64,
}
/// Default health status implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Defaulthealthstatus
pub struct DefaultHealthStatus {
    /// Status
    pub status: String,
    /// Pools Healthy
    pub pools_healthy: u32,
    /// Datasets Healthy
    pub datasets_healthy: u32,
    /// Snapshots Healthy
    pub snapshots_healthy: u32,
    /// Overall Health Percentage
    pub overall_health_percentage: f64,
    /// Last Check
    pub last_check: SystemTime,
    /// Issues
    pub issues: Vec<String>,
}
/// Default service metrics implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Defaultservicemetrics
pub struct DefaultServiceMetrics {
    /// Total Operations
    pub total_operations: u64,
    /// Successful Operations
    pub successful_operations: u64,
    /// Failed Operations
    pub failed_operations: u64,
    /// Average Operation Time
    pub average_operation_time: Duration,
    /// Uptime
    pub uptime: Duration,
    /// Pools Managed
    pub pools_managed: u32,
    /// Datasets Managed
    pub datasets_managed: u32,
    /// Snapshots Managed
    pub snapshots_managed: u32,
}
/// ZFS operation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Zfsoperationstats
pub struct ZfsOperationStats {
    /// Pool Operations
    pub pool_operations: OperationTypeStats,
    /// Dataset Operations
    pub dataset_operations: OperationTypeStats,
    /// Snapshot Operations
    pub snapshot_operations: OperationTypeStats,
    /// Cache Stats
    pub cache_stats: ZfsCacheStats,
}
/// Statistics for specific operation types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Operationtypestats
pub struct OperationTypeStats {
    /// Total
    pub total: u64,
    /// Successful
    pub successful: u64,
    /// Failed
    pub failed: u64,
    /// Average Duration Ms
    pub average_duration_ms: f64,
}
/// ZFS cache statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Zfscachestats
pub struct ZfsCacheStats {
    /// Hit Rate
    pub hit_rate: f64,
    /// Miss Rate
    pub miss_rate: f64,
    /// Total Requests
    pub total_requests: u64,
    /// Cache Size Bytes
    pub cache_size_bytes: u64,
    /// Evictions
    pub evictions: u64,
}
/// Default pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DefaultPoolConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DefaultPoolConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for DefaultPool
pub struct DefaultPoolConfig {
    /// Name
    pub name: String,
    /// Devices
    pub devices: Vec<String>,
    /// Raid Level
    pub raid_level: String,
}
/// Default dataset configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for DefaultDataset
pub struct DefaultDatasetConfig {
    /// Name
    pub name: String,
    /// Pool
    pub pool: String,
    /// Compression
    pub compression: String,
}
/// Default snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for DefaultSnapshot
pub struct DefaultSnapshotConfig {
    /// Dataset
    pub dataset: String,
    /// Name
    pub name: String,
    /// Recursive
    pub recursive: bool,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Defaultpoolconfigcanonical
pub type DefaultPoolConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DefaultPoolConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

