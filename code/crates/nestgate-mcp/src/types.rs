//! Type definitions for the NestGate MCP system.
//!
//! This module has been restructured into focused sub-modules for better
//! maintainability and adherence to the 1000-line file size limit.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Import types from the types directory files
pub use self::auth_types::*;
pub use self::provider_types::*;
pub use self::storage_types::*;

// Include the individual type modules directly
#[path = "types/auth.rs"]
mod auth_types;

#[path = "types/provider.rs"]
mod provider_types;

#[path = "types/storage.rs"]
mod storage_types;

// Type aliases for convenience
pub type ProviderId = String;
pub type VolumeId = String;
pub type MountId = String;
pub type NodeId = String;

// Remaining types that need to be organized (simplified for now)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VolumeStatus {
    Creating,
    Available,
    InUse,
    Deleting,
    Error(String),
}

/// Mount options for volume mounting operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountOptions {
    /// File system type
    pub fs_type: Option<String>,
    /// Mount flags
    pub mount_flags: Vec<String>,
    /// Read-only mount
    pub read_only: bool,
    /// Additional mount parameters
    pub parameters: HashMap<String, String>,
}

impl Default for MountOptions {
    fn default() -> Self {
        Self {
            fs_type: None,
            mount_flags: vec!["rw".to_string()],
            read_only: false,
            parameters: HashMap::new(),
        }
    }
}

/// Disk I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIo {
    pub reads_completed: u64,
    pub writes_completed: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub io_time_ms: u64,
}

impl Default for DiskIo {
    fn default() -> Self {
        Self {
            reads_completed: 0,
            writes_completed: 0,
            bytes_read: 0,
            bytes_written: 0,
            io_time_ms: 0,
        }
    }
}

/// Storage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub total_capacity: u64,
    pub used_capacity: u64,
    pub available_capacity: u64,
    pub tier_metrics: HashMap<String, TierMetrics>,
}

/// Metrics for a specific storage tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMetrics {
    pub capacity: u64,
    pub used: u64,
    pub available: u64,
    pub performance_score: f64,
}

impl Default for StorageMetrics {
    fn default() -> Self {
        Self {
            total_capacity: 0,
            used_capacity: 0,
            available_capacity: 0,
            tier_metrics: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeInfo {
    pub id: VolumeId,
    pub name: String,
    pub size: u64,
    pub tier: StorageTier,
    pub status: VolumeStatus,
    pub provider_id: ProviderId,
    pub created_at: SystemTime,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeRequest {
    pub name: String,
    pub size: u64,
    pub tier: StorageTier,
    pub provider_id: ProviderId,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeResponse {
    pub volume: VolumeInfo,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeFilter {
    pub status: Option<VolumeStatus>,
    pub tier: Option<StorageTier>,
    pub provider_id: Option<ProviderId>,
}

// Mount-related types (these could be split into another module if needed)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountInfo {
    pub id: MountId,
    pub volume_id: VolumeId,
    pub node_id: NodeId,
    pub mount_point: PathBuf,
    pub protocol: StorageProtocol,
    pub created_at: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountRequest {
    pub volume_id: VolumeId,
    pub node_id: NodeId,
    pub mount_point: PathBuf,
    pub protocol: StorageProtocol,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountResponse {
    pub mount: MountInfo,
    pub message: String,
}
