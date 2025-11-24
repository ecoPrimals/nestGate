//! **CANONICAL STORAGE CONFIGURATION TYPES**
//!
//! Storage backend, connection, performance, security, replication,
//! tiering, protocol, and monitoring configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackendType {
    Filesystem,
    Memory,
    Zfs,
    Remote,
    Hybrid,
    Cloud,
    Distributed,
}

impl Default for StorageBackendType {
    fn default() -> Self {
        Self::Filesystem
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageConnectionConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageConnectionConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StorageConnectionConfig {
    pub connection_string: Option<String>,
    pub pool_size: usize,
    pub timeout: Duration,
    pub retry_attempts: usize,
    pub retry_delay: Duration,
    pub keep_alive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StoragePerformanceConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StoragePerformanceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StoragePerformanceConfig {
    pub cache_size_mb: usize,
    pub io_threads: usize,
    pub batch_size: usize,
    pub prefetch_enabled: bool,
    pub compression_enabled: bool,
    pub max_iops: Option<u64>,
    pub max_throughput_mbps: Option<u64>,
    pub latency_target_ms: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageSecurityConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageSecurityConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StorageSecurityConfig {
    pub encryption_enabled: bool,
    pub encryption_algorithm: String,
    pub key_rotation_interval: Duration,
    pub access_logging: bool,
    pub permission_checks: bool,
    pub secure_deletion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageReplicationConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageReplicationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StorageReplicationConfig {
    pub enabled: bool,
    pub replica_count: usize,
    pub sync_mode: ReplicationSyncMode,
    pub conflict_resolution: ConflictResolutionStrategy,
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationSyncMode {
    Synchronous,
    Asynchronous,
    SemiSynchronous,
}

impl Default for ReplicationSyncMode {
    fn default() -> Self {
        Self::Asynchronous
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    LastWriteWins,
    FirstWriteWins,
    Manual,
    Versioned,
}

impl Default for ConflictResolutionStrategy {
    fn default() -> Self {
        Self::LastWriteWins
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageTierConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageTierConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StorageTierConfig {
    pub hot_tier: TierSettings,
    pub warm_tier: TierSettings,
    pub cold_tier: TierSettings,
    pub archive_tier: TierSettings,
    pub auto_tiering: bool,
    pub migration_policies: Vec<TierMigrationPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TierSettings {
    pub storage_type: String,
    pub capacity_gb: Option<u64>,
    pub performance_class: String,
    pub migration_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMigrationPolicy {
    pub from_tier: String,
    pub to_tier: String,
    pub conditions: Vec<MigrationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationCondition {
    pub metric: String,
    pub operator: String,
    pub threshold: f64,
    pub duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageProtocolsConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageProtocolsConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StorageProtocolsConfig {
    pub nfs: Option<NfsProtocolConfig>,
    pub smb: Option<SmbProtocolConfig>,
    pub iscsi: Option<IscsiProtocolConfig>,
    pub s3: Option<S3ProtocolConfig>,
    pub ftp: Option<FtpProtocolConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsProtocolConfig {
    pub version: String,
    pub allowed_clients: Vec<String>,
    pub mount_options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbProtocolConfig {
    pub version: String,
    pub share_name: String,
    pub workgroup: String,
    pub authentication: SmbAuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbAuthConfig {
    pub method: String,
    pub domain: Option<String>,
    pub guest_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IscsiProtocolConfig {
    pub target_name: String,
    pub portal_endpoint: String,
    pub portal_port: u16,
    pub authentication: IscsiAuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IscsiAuthConfig {
    pub chap_enabled: bool,
    pub username: Option<String>,
    pub mutual_chap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3ProtocolConfig {
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub use_ssl: bool,
    pub path_style: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FtpProtocolConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub passive_mode: bool,
    pub ssl_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageMonitoringConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageMonitoringConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StorageMonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_interval: Duration,
    pub alert_thresholds: HashMap<String, f64>,
    pub performance_tracking: bool,
    pub usage_reporting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageResourceConfig {
    pub max_memory_mb: Option<u64>,
    pub max_disk_gb: Option<u64>,
    pub max_connections: usize,
    pub max_concurrent_operations: usize,
    pub resource_limits: HashMap<String, u64>,
}
