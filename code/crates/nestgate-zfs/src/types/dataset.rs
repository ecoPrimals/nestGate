//! Dataset-related types for ZFS
//!
//! This module contains all types related to ZFS datasets and snapshots.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

use super::common::StorageTier;

/// ZFS dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    /// Short dataset name (without pool prefix)
    pub name: String,
    /// Full dataset name including pool
    pub full_name: String,
    /// Parent pool name
    pub pool: String,
    /// Total dataset size in bytes
    pub size: u64,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
    /// Mount point path
    pub mountpoint: Option<PathBuf>,
    /// Storage tier classification
    pub tier: StorageTier,
    /// Compression enabled
    pub compression: bool,
    /// Deduplication enabled
    pub deduplication: bool,
    /// When the dataset was created
    pub created_at: SystemTime,
}

impl Default for DatasetInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            full_name: String::new(),
            pool: String::new(),
            size: 0,
            used: 0,
            available: 0,
            mountpoint: None,
            tier: StorageTier::Hot,
            compression: false,
            deduplication: false,
            created_at: SystemTime::now(),
        }
    }
}

/// ZFS snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot name
    pub name: String,
    /// Full snapshot name (dataset@snapshot)
    pub full_name: String,
    /// Parent dataset name
    pub dataset: String,
    /// Snapshot size in bytes
    pub size: u64,
    /// When the snapshot was created
    pub created_at: SystemTime,
}

impl Default for SnapshotInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            full_name: String::new(),
            dataset: String::new(),
            size: 0,
            created_at: SystemTime::now(),
        }
    }
}

/// Zero-cost dataset info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostDatasetInfo {
    /// Dataset name
    pub name: String,
    /// Parent pool
    pub pool: String,
    /// Total size in bytes
    pub size: u64,
    /// Used size in bytes
    pub used: u64,
    /// Available size in bytes
    pub available: u64,
    /// Mount point
    pub mountpoint: Option<String>,
    /// Storage tier
    pub tier: String,
}

/// Zero-cost snapshot info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostSnapshotInfo {
    /// Snapshot name
    pub name: String,
    /// Parent dataset
    pub dataset: String,
    /// Snapshot size in bytes
    pub size: u64,
    /// Creation timestamp (Unix epoch seconds)
    pub created_at: u64,
}

