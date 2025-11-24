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
pub struct ServiceCapabilities {
    pub can_scale: bool,
    pub can_migrate: bool,
    pub can_backup: bool,
    pub supported_protocols: Vec<String>,
}
/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Default derive
pub struct ProviderHealth {
    pub is_healthy: bool,
    pub last_check: SystemTime,
    pub health: String,
}
impl Default for ProviderHealth {
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
pub struct ProviderCapabilities {
    pub supported_types: Vec<UnifiedServiceType>,
    pub max_instances: Option<u32>,
}
/// Storage usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct ConnectionHandle(pub u64);
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Active,
    Idle,
    Closed,
    Error(String),
}
/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
/// Security credentials
#[derive(Debug, Clone)]
pub struct SecurityCredentials {
    pub username: String,
    pub password: String,
}
/// Cron schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronSchedule {
    pub expression: String,
}
/// Schedule ID
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
pub struct ScheduleId {
    pub id: String,
}
/// Schedule info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInfo {
    pub id: ScheduleId,
    pub schedule: CronSchedule,
    pub next_run: Option<SystemTime>,
}

// ==================== CANONICAL STORAGE TYPES ====================

/// Storage backend type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
pub struct MessageMetadata {
    pub timestamp: SystemTime,
    pub priority: u8,
}

impl Default for MessageMetadata {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            priority: 0,
        }
    }
}

/// Network connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    pub connection_id: u64,
    pub remote_address: String,
}

/// Security policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub name: String,
    pub rules: Vec<String>,
}

/// Storage metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    pub size: u64,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
}

/// Stream handle for data streaming
#[derive(Debug, Clone, Copy)]
pub struct StreamHandle(pub u64);
