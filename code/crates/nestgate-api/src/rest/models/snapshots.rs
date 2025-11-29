//! Snapshot models and data structures for the NestGate REST API
//!
//! This module contains all data structures related to snapshot management,
//! including snapshot creation requests, snapshot _metadata, and cloning operations.

use super::types::SnapshotStatus;
use serde::{Deserialize, Serialize};

/// Request structure for creating a new snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for CreateSnapshot operation
pub struct CreateSnapshotRequest {
    /// Name of the snapshot to create
    pub name: String,

    /// Optional description for the snapshot
    pub description: Option<String>,

    /// Whether to create snapshots recursively for child datasets
    pub recursive: bool,

    /// Custom properties to set on the snapshot
    pub properties: std::collections::HashMap<String, String>,

    /// Optional tags to associate with the snapshot
    pub tags: Option<Vec<String>>,
}

/// Complete snapshot information and _metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshot
pub struct Snapshot {
    /// Unique identifier for the snapshot
    pub id: String,
    /// Name of the snapshot
    pub name: String,
    /// Name of the parent dataset
    pub dataset: String,
    /// Timestamp when the snapshot was created
    pub created: chrono::DateTime<chrono::Utc>,
    /// Total size of the snapshot in bytes
    pub size_bytes: u64,
    /// Unique data size in bytes (not shared with other snapshots)
    pub unique_bytes: u64,
    /// Referenced data size in bytes (shared with other snapshots)
    pub referenced_bytes: u64,
    /// Number of files in the snapshot
    pub file_count: u64,
    /// Current status of the snapshot
    pub status: SnapshotStatus,
    /// Optional description of the snapshot
    pub description: Option<String>,
    /// Tags associated with the snapshot
    pub tags: Vec<String>,
}

/// Request structure for cloning a snapshot into a new dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for CloneSnapshot operation
pub struct CloneSnapshotRequest {
    /// Name of the target dataset to create from the clone
    pub target_dataset_name: String,
    /// Name for the cloned dataset
    pub clone_name: String,
    /// Optional properties to set on the cloned dataset
    pub properties: Option<crate::rest::models::datasets::DatasetProperties>,
    /// Optional description for the cloned dataset
    pub description: Option<String>,
}

/// Metadata about a snapshot for listing operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotmetadata
pub struct SnapshotMetadata {
    /// Name of the snapshot
    pub name: String,
    /// Name of the parent dataset
    pub dataset: String,
    /// Timestamp when the snapshot was created
    pub created: chrono::DateTime<chrono::Utc>,
    /// Total size of the snapshot in bytes
    pub size_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Referenced data size in bytes
    pub referenced_bytes: u64,
}
