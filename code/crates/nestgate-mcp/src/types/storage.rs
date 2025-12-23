
//! Storage module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

// Import the canonical StorageTier from the unified system
pub use nestgate_core::canonical_types::StorageTier;

// Note: StorageTier now comes from nestgate_core::types with these variants:
// - Hot: High-performance storage for frequently accessed data
// - Warm: Medium-performance storage for moderately accessed data
// - Cold: Low-performance storage for rarely accessed data
// - Cache: Fast cache storage for temporary data
//
// The Archive tier from the old definition maps to Cold tier in the unified system

/// Storage protocols supported by providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Storageprotocol
pub enum StorageProtocol {
    /// NFS protocol with version.
    Nfs(NfsVersion),
    /// SMB protocol with version.
    Smb(SmbVersion),
    /// iSCSI protocol.
    Iscsi,
    /// S3 protocol.
    S3,
    /// Custom protocol with name.
    Custom(String),
}
/// NFS protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Nfsversion
pub enum NfsVersion {
    /// NFS version 3.
    V3,
    /// NFS version 4.0.
    V4,
    /// NFS version 4.1.
    V41,
    /// NFS version 4.2.
    V42,
}
/// SMB protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Smbversion
pub enum SmbVersion {
    /// SMB version 2.
    V2,
    /// SMB version 3.0.
    V3,
    /// SMB version 3.1.
    V31,
}
/// Volume information for MCP storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Volumeinfo
pub struct VolumeInfo {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Size Bytes
    pub size_bytes: u64,
    /// Used Bytes
    pub used_bytes: u64,
    /// Tier
    pub tier: StorageTier,
    /// Protocol
    pub protocol: StorageProtocol,
    /// Mount Path
    pub mount_path: Option<String>,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Mount information for storage volumes
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mountinfo
pub struct MountInfo {
    /// Volume identifier
    pub volume_id: String,
    /// Mount Path
    pub mount_path: String,
    /// Protocol
    pub protocol: StorageProtocol,
    /// Options
    pub options: MountOptions,
    /// Status
    pub status: MountStatus,
    /// Mounted At
    pub mounted_at: SystemTime,
}
/// Mount options for storage volumes
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mountoptions
pub struct MountOptions {
    /// Read Only
    pub read_only: bool,
    /// Sync
    pub sync: bool,
    /// No Exec
    pub no_exec: bool,
    /// No Suid
    pub no_suid: bool,
    /// Timeout Seconds
    pub timeout_seconds: Option<u32>,
    /// Custom Options
    pub custom_options: HashMap<String, String>,
}
/// Mount status for storage volumes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for Mount
pub enum MountStatus {
    /// Mounting
    Mounting,
    /// Mounted
    Mounted,
    /// Unmounting
    Unmounting,
    /// Unmounted
    Unmounted,
    /// Error
    Error,
}
/// Mount request for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Mount operation
pub struct MountRequest {
    /// Volume identifier
    pub volume_id: String,
    /// Mount Path
    pub mount_path: String,
    /// Options
    pub options: MountOptions,
    /// Force
    pub force: bool,
}
/// Storage capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagecapacity
pub struct StorageCapacity {
    /// Total Bytes
    pub total_bytes: u64,
    /// Used Bytes
    pub used_bytes: u64,
    /// Available Bytes
    pub available_bytes: u64,
    /// Tier
    pub tier: StorageTier,
}
/// Storage metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagemetrics
pub struct StorageMetrics {
    /// Read Ops Per Sec
    pub read_ops_per_sec: f64,
    /// Write Ops Per Sec
    pub write_ops_per_sec: f64,
    /// Read Bytes Per Sec
    pub read_bytes_per_sec: f64,
    /// Write Bytes Per Sec
    pub write_bytes_per_sec: f64,
    /// Latency Ms
    pub latency_ms: f64,
    /// Error Rate
    pub error_rate: f64,
    /// Capacity
    pub capacity: StorageCapacity,
    /// Timestamp
    pub timestamp: SystemTime,
}
