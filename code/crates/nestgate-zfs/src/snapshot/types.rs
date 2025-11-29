//
// Core data structures for snapshot information, operations, and statistics.

//! Types module

use crate::types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::operations::SnapshotOperationType;

/// Snapshot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotinfo
pub struct SnapshotInfo {
    /// Snapshot name
    pub name: String,
    /// Full snapshot path (dataset@snapshot)
    pub full_name: String,
    /// Dataset name
    pub dataset: String,
    /// Creation time
    pub created_at: SystemTime,
    /// Snapshot size in bytes
    pub size: u64,
    /// Referenced data size in bytes
    pub referenced_size: u64,
    /// Written data size in bytes
    pub written_size: u64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Snapshot properties
    pub properties: HashMap<String, String>,
    /// Associated policy name
    pub policy: Option<String>,
    /// Storage tier
    pub tier: StorageTier,
    /// Whether snapshot is protected from deletion
    pub protected: bool,
    /// Snapshot tags for organization
    pub tags: Vec<String>,
}
/// Snapshot operation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Status values for SnapshotOperation
pub enum SnapshotOperationStatus {
    /// Operation is queued
    Queued,
    /// Operation is running
    Running,
    /// Operation completed successfully
    Completed,
    /// Operation failed
    Failed(String),
    /// Operation was cancelled
    Cancelled,
}
/// Snapshot operation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotoperation
pub struct SnapshotOperation {
    /// Operation ID
    pub id: String,
    /// Operation type
    pub operation_type: SnapshotOperationType,
    /// Target dataset
    pub dataset: String,
    /// Snapshot name (for create/delete operations)
    pub snapshot_name: Option<String>,
    /// Operation status
    pub status: SnapshotOperationStatus,
    /// Created timestamp
    pub created_at: SystemTime,
    /// Started timestamp
    pub started_at: Option<SystemTime>,
    /// Completed timestamp
    pub completed_at: Option<SystemTime>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Associated policy
    pub policy: Option<String>,
}
/// Snapshot statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Snapshotstatistics
pub struct SnapshotStatistics {
    /// Total snapshots across all datasets
    pub total_snapshots: u64,
    /// Total snapshot data size in bytes
    pub total_size: u64,
    /// Total referenced data size in bytes
    pub total_referenced_size: u64,
    /// Total written data size in bytes
    pub total_written_size: u64,
    /// Average compression ratio
    pub average_compression_ratio: f64,
    /// Snapshots per tier
    pub snapshots_per_tier: HashMap<StorageTier, u64>,
    /// Size per tier
    pub size_per_tier: HashMap<StorageTier, u64>,
    /// Active policies
    pub active_policies: u32,
    /// Pending operations
    pub pending_operations: u32,
    /// Failed operations in last 24 hours
    pub recent_failures: u32,
}
