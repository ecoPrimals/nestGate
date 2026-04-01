// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL STORAGE CONFIGURATION TYPES**
//!
//! Storage backend, connection, performance, security, replication,
//! tiering, protocol, and monitoring configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of StorageBackend
pub enum StorageBackendType {
    /// Filesystem
    Filesystem,
    /// Memory
    Memory,
    /// Zfs
    Zfs,
    /// Remote
    Remote,
    /// Hybrid
    Hybrid,
    /// Cloud
    Cloud,
    /// Distributed
    Distributed,
}

impl Default for StorageBackendType {
    /// Returns the default instance
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
/// Configuration for StorageConnection
pub struct StorageConnectionConfig {
    /// Connection String
    pub connection_string: Option<String>,
    /// Size of pool
    pub pool_size: usize,
    /// Timeout
    pub timeout: Duration,
    /// Retry Attempts
    pub retry_attempts: usize,
    /// Retry Delay
    pub retry_delay: Duration,
    /// Keep Alive
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
/// Configuration for StoragePerformance
pub struct StoragePerformanceConfig {
    /// Cache Size in megabytes
    pub cache_size_mb: usize,
    /// Io Threads
    pub io_threads: usize,
    /// Size of batch
    pub batch_size: usize,
    /// Prefetch Enabled
    pub prefetch_enabled: bool,
    /// Compression Enabled
    pub compression_enabled: bool,
    /// Max Iops
    pub max_iops: Option<u64>,
    /// Max Throughput Mbps
    pub max_throughput_mbps: Option<u64>,
    /// Latency Target Ms
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
/// Configuration for StorageSecurity
pub struct StorageSecurityConfig {
    /// Encryption Enabled
    pub encryption_enabled: bool,
    /// Encryption Algorithm
    pub encryption_algorithm: String,
    /// Key Rotation Interval
    pub key_rotation_interval: Duration,
    /// Access Logging
    pub access_logging: bool,
    /// Permission Checks
    pub permission_checks: bool,
    /// Secure Deletion
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
/// Configuration for StorageReplication
pub struct StorageReplicationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Count of replica
    pub replica_count: usize,
    /// Sync Mode
    pub sync_mode: ReplicationSyncMode,
    /// Conflict Resolution
    pub conflict_resolution: ConflictResolutionStrategy,
    /// Health Check Interval
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Replicationsyncmode
pub enum ReplicationSyncMode {
    /// Synchronous
    Synchronous,
    /// Asynchronous
    Asynchronous,
    /// Semisynchronous
    SemiSynchronous,
}

impl Default for ReplicationSyncMode {
    /// Returns the default instance
    fn default() -> Self {
        Self::Asynchronous
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Conflictresolutionstrategy
pub enum ConflictResolutionStrategy {
    /// Lastwritewins
    LastWriteWins,
    /// Firstwritewins
    FirstWriteWins,
    /// Manual
    Manual,
    /// Versioned
    Versioned,
}

impl Default for ConflictResolutionStrategy {
    /// Returns the default instance
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
/// Configuration for StorageTier
pub struct StorageTierConfig {
    /// Hot Tier
    pub hot_tier: TierSettings,
    /// Warm Tier
    pub warm_tier: TierSettings,
    /// Cold Tier
    pub cold_tier: TierSettings,
    /// Archive Tier
    pub archive_tier: TierSettings,
    /// Auto Tiering
    pub auto_tiering: bool,
    /// Migration Policies
    pub migration_policies: Vec<TierMigrationPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Tiersettings
pub struct TierSettings {
    /// Storage Type
    pub storage_type: String,
    /// Capacity in gigabytes
    pub capacity_gb: Option<u64>,
    /// Performance Class
    pub performance_class: String,
    /// Migration Threshold
    pub migration_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tiermigrationpolicy
pub struct TierMigrationPolicy {
    /// From Tier
    pub from_tier: String,
    /// To Tier
    pub to_tier: String,
    /// Conditions
    pub conditions: Vec<MigrationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Migrationcondition
pub struct MigrationCondition {
    /// Metric
    pub metric: String,
    /// Operator
    pub operator: String,
    /// Threshold
    pub threshold: f64,
    /// Duration
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
/// Configuration for StorageProtocols
pub struct StorageProtocolsConfig {
    /// Nfs
    pub nfs: Option<NfsProtocolConfig>,
    /// Smb
    pub smb: Option<SmbProtocolConfig>,
    /// Iscsi
    pub iscsi: Option<IscsiProtocolConfig>,
    /// S3
    pub s3: Option<S3ProtocolConfig>,
    /// Ftp
    pub ftp: Option<FtpProtocolConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for NfsProtocol
pub struct NfsProtocolConfig {
    /// Version
    pub version: String,
    /// Allowed Clients
    pub allowed_clients: Vec<String>,
    /// Mount Options
    pub mount_options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for SmbProtocol
pub struct SmbProtocolConfig {
    /// Version
    pub version: String,
    /// Share name
    pub share_name: String,
    /// Workgroup
    pub workgroup: String,
    /// Authentication
    pub authentication: SmbAuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for SmbAuth
pub struct SmbAuthConfig {
    /// Method
    pub method: String,
    /// Domain
    pub domain: Option<String>,
    /// Guest Access
    pub guest_access: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for IscsiProtocol
pub struct IscsiProtocolConfig {
    /// Target name
    pub target_name: String,
    /// Portal Endpoint
    pub portal_endpoint: String,
    /// Portal Port
    pub portal_port: u16,
    /// Authentication
    pub authentication: IscsiAuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for IscsiAuth
pub struct IscsiAuthConfig {
    /// Chap Enabled
    pub chap_enabled: bool,
    /// Username
    pub username: Option<String>,
    /// Mutual Chap
    pub mutual_chap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for S3Protocol
pub struct S3ProtocolConfig {
    /// Endpoint
    pub endpoint: String,
    /// Region
    pub region: String,
    /// Bucket
    pub bucket: String,
    /// Access Key identifier
    pub access_key_id: String,
    /// Secret Access Key
    pub secret_access_key: String,
    /// Use Ssl
    pub use_ssl: bool,
    /// Path Style
    pub path_style: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for FtpProtocol
pub struct FtpProtocolConfig {
    /// Host
    pub host: String,
    /// Port
    pub port: u16,
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Passive Mode
    pub passive_mode: bool,
    /// Ssl Enabled
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
/// Configuration for StorageMonitoring
pub struct StorageMonitoringConfig {
    /// Metrics Enabled
    pub metrics_enabled: bool,
    /// Metrics Interval
    pub metrics_interval: Duration,
    /// Alert Thresholds
    pub alert_thresholds: HashMap<String, f64>,
    /// Performance Tracking
    pub performance_tracking: bool,
    /// Usage Reporting
    pub usage_reporting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for StorageResource
pub struct StorageResourceConfig {
    /// Max Memory in megabytes
    pub max_memory_mb: Option<u64>,
    /// Max Disk in gigabytes
    pub max_disk_gb: Option<u64>,
    /// Max Connections
    pub max_connections: usize,
    /// Max Concurrent Operations
    pub max_concurrent_operations: usize,
    /// Resource Limits
    pub resource_limits: HashMap<String, u64>,
}
