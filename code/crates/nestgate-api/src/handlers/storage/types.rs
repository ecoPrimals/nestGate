// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// Storage pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagepool
pub struct StoragePool {
    /// Pool name
    pub name: String,
    /// Pool status
    pub status: String,
    /// Total pool size in bytes
    pub size: u64,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
    /// Pool health status
    pub health: String,
    /// Pool type (raidz, mirror, etc.)
    pub pool_type: String,
}
/// Storage dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagedataset
pub struct StorageDataset {
    /// Dataset name
    pub name: String,
    /// Parent pool name
    pub pool: String,
    /// Dataset size in bytes
    pub size: u64,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
    /// Mount point path
    pub mount_point: String,
    /// Compression algorithm
    pub compression: String,
}
/// Storage snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagesnapshot
pub struct StorageSnapshot {
    /// Name of the storage volume
    pub name: String,
    /// Dataset path for this volume
    pub dataset: String,
    /// Size of the volume in bytes
    pub size: u64,
    /// Creation timestamp
    pub created: String,
    /// Referenced data size in bytes
    pub referenced: u64,
}
/// Storage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagemetrics
pub struct StorageMetrics {
    /// Total number of storage pools
    pub total_pools: u32,
    /// Total number of datasets
    pub total_datasets: u32,
    /// Total number of snapshots
    pub total_snapshots: u32,
    /// Total storage capacity in bytes
    pub total_storage: u64,
    /// Used storage space in bytes
    pub used_storage: u64,
    /// Available storage space in bytes
    pub available_storage: u64,
    /// Input/output operations per second
    pub iops: f64,
    /// Bandwidth in megabits per second
    pub bandwidth_mbps: f64,
    /// Overall health status of storage system
    pub health_status: String,
}

/// **STORAGE POOL INFO**
///
/// Information about a storage pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagepoolinfo
pub struct StoragePoolInfo {
    /// Pool name
    pub name: String,
    /// Total pool capacity in gigabytes
    pub total_capacity_gb: u64,
    /// Used capacity in gigabytes
    pub used_capacity_gb: u64,
    /// Available capacity in gigabytes
    pub available_capacity_gb: u64,
    /// Current health status
    pub health_status: String,
}

/// **STORAGE DATASET INFO**
///
/// Information about a storage dataset.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagedatasetinfo
pub struct StorageDatasetInfo {
    /// Dataset name
    pub name: String,
    /// Parent pool name
    pub pool_name: String,
    /// Used space in gigabytes
    pub used_space_gb: u64,
    /// Compression ratio achieved
    pub compression_ratio: f64,
    /// Deduplication ratio achieved
    pub dedup_ratio: f64,
}

/// **STORAGE SNAPSHOT INFO**
///
/// Information about a storage snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagesnapshotinfo
pub struct StorageSnapshotInfo {
    /// Snapshot name
    pub name: String,
    /// Parent dataset name
    pub dataset_name: String,
    /// Snapshot creation timestamp
    pub created_at: std::time::SystemTime,
    /// Snapshot size in gigabytes
    pub size_gb: u64,
}

/// Storage manager for storage operations
#[derive(Debug, Clone)]
/// Manager for Storage operations
pub struct StorageManager {
    // Placeholder fields
}

impl Default for StorageManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl StorageManager {
    /// Create a new storage manager instance
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
