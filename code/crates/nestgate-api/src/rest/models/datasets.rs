// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dataset models and data structures for the NestGate REST API
//!
//! This module contains all data structures related to dataset management,
//! including dataset properties, statistics, creation requests, and _metadata.

use serde::{Deserialize, Serialize};

use super::types::{ChecksumType, CompressionType, DatasetStatus, DatasetType, StorageBackendType};

/// Request structure for creating a new dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `CreateDataset` operation
pub struct CreateDatasetRequest {
    /// Name of the dataset to create
    pub name: String,
    /// Type of dataset (filesystem, volume, etc.)
    pub dataset_type: DatasetType,
    /// Storage backend to use for the dataset
    pub backend: StorageBackendType,
    /// Optional dataset properties to set during creation
    pub properties: Option<DatasetProperties>,
    /// Optional size quota in bytes
    pub quota: Option<u64>,
    /// Optional description for the dataset
    pub description: Option<String>,
}

/// Request structure for updating an existing dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `UpdateDataset` operation
pub struct UpdateDatasetRequest {
    /// Optional updated properties for the dataset
    pub properties: Option<DatasetProperties>,
    /// Optional updated size quota in bytes
    pub quota: Option<u64>,
    /// Optional updated compression type
    pub compression_type: Option<CompressionType>,
    /// Optional updated checksum type
    pub checksum_type: Option<ChecksumType>,
}

/// Comprehensive dataset properties and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datasetproperties
pub struct DatasetProperties {
    /// Name of the dataset
    pub name: String,
    /// Optional mountpoint path for filesystem datasets
    pub mountpoint: Option<String>,
    /// Optional size quota in bytes
    pub quota: Option<u64>,
    /// Optional space reservation in bytes
    pub reservation: Option<u64>,

    // Compression settings
    /// Whether compression is enabled
    pub compression: bool,
    /// Type of compression algorithm to use
    pub compression_type: Option<CompressionType>,

    /// Whether checksumming is enabled
    pub checksum: bool,
    /// Type of checksum algorithm to use
    pub checksum_type: Option<ChecksumType>,

    /// Whether deduplication is enabled
    pub deduplication: bool,

    /// Whether encryption is enabled
    pub encryption: bool,

    /// Whether the dataset is read-only
    pub readonly: bool,

    /// Custom properties as key-value pairs
    pub custom: std::collections::HashMap<String, String>,
}

/// Statistical information about a dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datasetstats
pub struct DatasetStats {
    /// Name of the dataset
    pub name: String,
    /// Total size of the dataset in bytes
    pub size_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Number of files written to the dataset
    pub files_written: u64,
    /// Number of files read from the dataset
    pub files_read: u64,
    /// Number of copy-on-write operations performed
    pub cow_operations: u64,
    /// Number of blocks copied during COW operations
    pub blocks_copied: u64,
    /// Compression ratio achieved (if compression enabled)
    pub compression_ratio: Option<f64>,
    /// Space saved through compression in bytes
    pub compression_space_saved: Option<u64>,
    /// Deduplication ratio achieved
    pub deduplication_ratio: f64,
    /// Number of checksums computed
    pub checksums_computed: u64,
    /// Number of checksums verified
    pub checksums_verified: u64,
    /// Read throughput in bytes per second
    pub read_throughput: f64,
    /// Write throughput in bytes per second
    pub write_throughput: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Number of snapshots associated with this dataset
    pub snapshot_count: u32,
}

/// Complete dataset information including properties and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Dataset
pub struct Dataset {
    /// Unique name of the dataset
    pub name: String,
    /// Full path to the dataset
    pub path: String,
    /// Type of the dataset
    pub dataset_type: DatasetType,
    /// Storage backend used by the dataset
    pub backend: StorageBackendType,
    /// Total size of the dataset in bytes
    pub size_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Optional mountpoint path
    pub mountpoint: Option<String>,
    /// Dataset properties and configuration
    pub properties: DatasetProperties,
    /// Statistical information about the dataset
    pub stats: DatasetStats,
    /// Timestamp when the dataset was created
    pub created: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the dataset was last modified
    pub modified: chrono::DateTime<chrono::Utc>,
    /// Current status of the dataset
    pub status: DatasetStatus,
    /// Number of snapshots associated with this dataset
    pub snapshot_count: u32,
}
