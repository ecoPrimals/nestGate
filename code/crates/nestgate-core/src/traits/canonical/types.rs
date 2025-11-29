//! **Canonical Supporting Types**
//!
//! All supporting types for the canonical trait system.
//!
//! **Extracted**: November 19, 2025 - From canonical_unified_traits.rs
//! **Lines**: ~165 (from original 1,100-line file)

use crate::unified_enums::service_types::UnifiedServiceType;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// ==================== SUPPORTING TYPES ====================

/// Service capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // PEDANTIC: Added Default derive
/// Servicecapabilities
pub struct ServiceCapabilities {
    /// Can Scale
    pub can_scale: bool,
    /// Can Migrate
    pub can_migrate: bool,
    /// Can Backup
    pub can_backup: bool,
    /// Supported Protocols
    pub supported_protocols: Vec<String>,
}
/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Default derive
/// Providerhealth
pub struct ProviderHealth {
    /// Whether healthy
    pub is_healthy: bool,
    /// Last Check
    pub last_check: SystemTime,
    /// Health
    pub health: String,
}
impl Default for ProviderHealth {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            is_healthy: false,
            last_check: SystemTime::now(),
            health: String::new(),
        }
    }
}

/// Provider capabilities
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
/// Providercapabilities
pub struct ProviderCapabilities {
    /// Supported Types
    pub supported_types: Vec<UnifiedServiceType>,
    /// Max Instances
    pub max_instances: Option<u32>,
}
/// Storage usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageusagestats
pub struct StorageUsageStats {
    /// Total storage capacity in bytes
    pub total_capacity: u64,
    /// Used storage in bytes
    pub used_space: u64,
    /// Available storage in bytes
    pub available_space: u64,
    /// Number of stored items
    pub item_count: u64,
    /// Last updated timestamp
    pub last_updated: std::time::SystemTime,
}
/// Connection handle
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// Connectionhandle
pub struct ConnectionHandle(pub u64);
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Connection
pub enum ConnectionStatus {
    /// Active
    Active,
    /// Idle
    Idle,
    /// Closed
    Closed,
    Error(String),
}
/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}
/// Security credentials
#[derive(Debug, Clone)]
/// Securitycredentials
pub struct SecurityCredentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}
/// Cron schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cronschedule
pub struct CronSchedule {
    /// Expression
    pub expression: String,
}
/// Schedule ID
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
/// Scheduleid
pub struct ScheduleId {
    /// Unique identifier
    pub id: String,
}
/// Schedule info
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Scheduleinfo
pub struct ScheduleInfo {
    /// Unique identifier
    pub id: ScheduleId,
    /// Schedule
    pub schedule: CronSchedule,
    /// Next Run
    pub next_run: Option<SystemTime>,
}

// ==================== CANONICAL STORAGE TYPES ====================

/// Storage backend type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of StorageBackend
pub enum StorageBackendType {
    /// Local filesystem storage
    FileSystem,
    /// In-memory storage
    Memory,
    /// Object storage (S3-compatible)
    ObjectStorage,
    /// Block storage
    BlockStorage,
    /// Network filesystem (NFS, SMB, etc.)
    NetworkFileSystem,
    /// ZFS storage
    Zfs,
    /// Distributed storage
    Distributed,
    /// Custom storage type
    Custom(String),
}

/// Storage capability enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Storagecapability
pub enum StorageCapability {
    /// Basic CRUD operations
    BasicOperations,
    /// Batch operations support
    BatchOperations,
    /// Metadata operations
    Metadata,
    /// Streaming data support
    Streaming,
    /// Snapshot support
    Snapshots,
    /// Atomic operations
    Atomic,
    /// Versioning support
    Versioning,
    /// Encryption support
    Encryption,
    /// Compression support
    Compression,
    /// Replication support
    Replication,
    /// Custom capability
    Custom(String),
}

/// Snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotinfo
pub struct SnapshotInfo {
    /// Snapshot identifier
    pub id: String,
    /// Snapshot name
    pub name: String,
    /// Creation timestamp
    pub created_at: std::time::SystemTime,
    /// Snapshot size in bytes
    pub size: u64,
}

/// Data stream for reading large objects
pub struct DataStream {
    // Implementation would contain actual stream
    _phantom: std::marker::PhantomData<()>,
}

/// Write stream for writing large objects
pub struct WriteStream {
    // Implementation would contain actual stream
    _phantom: std::marker::PhantomData<()>,
}

// Additional placeholder types for trait method signatures
/// Message metadata for network operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Messagemetadata
pub struct MessageMetadata {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Priority
    pub priority: u8,
}

impl Default for MessageMetadata {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            priority: 0,
        }
    }
}

/// Network connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Networkconnection
pub struct NetworkConnection {
    /// Connection identifier
    pub connection_id: u64,
    /// Remote Address
    pub remote_address: String,
}

/// Security policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securitypolicy
pub struct SecurityPolicy {
    /// Name
    pub name: String,
    /// Rules
    pub rules: Vec<String>,
}

/// Storage metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagemetadata
pub struct StorageMetadata {
    /// Size
    pub size: u64,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Modified At
    pub modified_at: SystemTime,
}

/// Stream handle for data streaming
#[derive(Debug, Clone, Copy)]
/// Streamhandle
pub struct StreamHandle(pub u64);
