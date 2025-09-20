
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
pub struct VolumeInfo {
    pub id: String,
    pub name: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub tier: StorageTier,
    pub protocol: StorageProtocol,
    pub mount_path: Option<String>,
    pub created_at: SystemTime,
    pub metadata: HashMap<String, String>,
}
/// Mount information for storage volumes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountInfo {
    pub volume_id: String,
    pub mount_path: String,
    pub protocol: StorageProtocol,
    pub options: MountOptions,
    pub status: MountStatus,
    pub mounted_at: SystemTime,
}
/// Mount options for storage volumes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountOptions {
    pub read_only: bool,
    pub sync: bool,
    pub no_exec: bool,
    pub no_suid: bool,
    pub timeout_seconds: Option<u32>,
    pub custom_options: HashMap<String, String>,
}
/// Mount status for storage volumes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MountStatus {
    Mounting,
    Mounted,
    Unmounting,
    Unmounted,
    Error,
}
/// Mount request for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountRequest {
    pub volume_id: String,
    pub mount_path: String,
    pub options: MountOptions,
    pub force: bool,
}
/// Storage capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapacity {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub tier: StorageTier,
}
/// Storage metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub read_ops_per_sec: f64,
    pub write_ops_per_sec: f64,
    pub read_bytes_per_sec: f64,
    pub write_bytes_per_sec: f64,
    pub latency_ms: f64,
    pub error_rate: f64,
    pub capacity: StorageCapacity,
    pub timestamp: SystemTime,
}
