// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Wire types exchanged over both the tarpc and JSON-RPC protocols.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pool information returned by `list_pools`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    /// Pool name
    pub name: String,
    /// Total capacity in gigabytes
    pub total_capacity_gb: u64,
    /// Used capacity in gigabytes
    pub used_capacity_gb: u64,
    /// Available capacity in gigabytes
    pub available_capacity_gb: u64,
    /// Health status (ONLINE, DEGRADED, FAULTED, etc.)
    pub health_status: String,
    /// Backend type (zfs, ceph, etc.)
    pub backend: String,
}

/// Dataset information returned by `list_datasets`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    /// Dataset name
    pub name: String,
    /// Parent pool name
    pub pool_name: String,
    /// Used space in gigabytes
    pub used_space_gb: u64,
    /// Compression ratio (actual/logical)
    pub compression_ratio: f64,
    /// Deduplication ratio
    pub dedup_ratio: f64,
    /// Creation timestamp
    pub created_at: Option<String>,
}

/// Parameters for creating a new dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDatasetRequest {
    /// Target pool name
    pub pool: String,
    /// Dataset name
    pub name: String,
    /// ZFS properties to set
    pub properties: HashMap<String, String>,
}

/// Snapshot information for point-in-time copies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot name (e.g., "dataset@snapshot-1")
    pub name: String,
    /// Parent dataset name
    pub dataset: String,
    /// Creation timestamp in RFC3339 format
    pub created_at: String,
    /// Snapshot size in gigabytes
    pub size_gb: u64,
}

/// Result of a storage mutation operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// Whether the operation succeeded
    pub success: bool,
    /// Human-readable message describing the result
    pub message: String,
    /// Optional data payload for the result
    pub data: Option<serde_json::Value>,
}

/// Aggregated storage metrics across pools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total storage capacity in gigabytes
    pub total_capacity_gb: u64,
    /// Used storage capacity in gigabytes
    pub used_capacity_gb: u64,
    /// Available storage capacity in gigabytes
    pub available_capacity_gb: u64,
    /// Compression ratio (e.g., 1.7 means 1.7x compression)
    pub compression_ratio: f64,
    /// Deduplication ratio (e.g., 1.4 means 1.4x dedup)
    pub dedup_ratio: f64,
    /// Number of datasets in the storage system
    pub dataset_count: usize,
    /// Number of snapshots in the storage system
    pub snapshot_count: usize,
}

/// Health status for the storage system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Current health status (e.g., "healthy", "degraded", "unhealthy")
    pub status: String,
    /// System version string
    pub version: String,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Number of healthy storage pools
    pub pools_healthy: usize,
    /// Total number of storage pools
    pub pools_total: usize,
}

/// Version and capability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// System version string
    pub version: String,
    /// Protocol name (e.g., "tarpc", "grpc")
    pub protocol: String,
    /// List of supported capabilities
    pub capabilities: Vec<String>,
}
