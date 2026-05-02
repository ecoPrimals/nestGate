// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module contains all the shared types, request/response structures,
// and data models used by the ZFS API handlers.

//! Types module

#![expect(
    clippy::pub_underscore_fields,
    reason = "ZFS handler DTOs use `_devices`/`_metadata` as reserved wire-visible fields"
)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// Production: Use real ZFS manager
#[cfg(not(feature = "dev-stubs"))]
use nestgate_zfs::ProductionZfsManager;

// Development: Use stub manager
#[cfg(feature = "dev-stubs")]
use crate::dev_stubs::zfs::ProductionZfsManager;

use nestgate_core::canonical_types::StorageTier;

/// ZFS API state container
#[derive(Clone)]
/// Zfsapistate
pub struct ZfsApiState {
    /// ZFS manager instance
    pub zfs_manager: Arc<ProductionZfsManager>,
}
/// Pool creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `CreatePool` operation
pub struct CreatePoolRequest {
    /// Pool name
    pub name: String,
    /// Device paths
    pub _devices: Vec<String>,
    /// Pool configuration
    pub config: Option<PoolConfig>,
}
/// Pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Pool
pub struct PoolConfig {
    /// RAID level (mirror, raidz1, raidz2, raidz3)
    pub raid_level: Option<String>,
    /// Compression algorithm
    pub compression: Option<String>,
    /// Deduplication enabled
    pub dedup: Option<bool>,
    /// Encryption enabled
    pub encryption: Option<bool>,
}
/// Dataset creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `CreateDataset` operation
pub struct CreateDatasetRequest {
    /// Dataset name
    pub name: String,
    /// Parent pool or dataset
    pub parent: String,
    /// Storage tier
    pub tier: StorageTier,
    /// Dataset properties
    pub properties: Option<HashMap<String, String>>,
}
/// Snapshot creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `CreateSnapshot` operation
pub struct CreateSnapshotRequest {
    /// Snapshot name
    pub name: String,
    /// Dataset to snapshot
    pub dataset: String,
    /// Recursive snapshot
    pub recursive: Option<bool>,
    /// Snapshot properties
    pub properties: Option<HashMap<String, String>>,
}
/// Tier migration request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `TierMigration` operation
pub struct TierMigrationRequest {
    /// Dataset path
    pub dataset_path: String,
    /// Source tier
    pub source_tier: StorageTier,
    /// Target tier
    pub target_tier: StorageTier,
    /// Migration priority
    pub priority: Option<u8>,
    /// Force migration even if not recommended
    pub force: Option<bool>,
}
/// Query parameters for listing operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Listquery
pub struct ListQuery {
    /// Limit number of results
    pub limit: Option<usize>,
    /// Skip number of results
    pub skip: Option<usize>,
    /// Filter by status
    pub status: Option<String>,
    /// Filter by tier
    pub tier: Option<StorageTier>,
}
/// Re-export universal API response from nestgate-core to eliminate duplication
pub use nestgate_core::response::ApiResponse;
/// Tier prediction request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `TierPrediction` operation
pub struct TierPredictionRequest {
    /// Path to the file for tier prediction
    pub file_path: String,
}
/// Performance analytics request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `PerformanceAnalytics` operation
pub struct PerformanceAnalyticsRequest {
    /// Pool name to analyze
    pub pool_name: Option<String>,
    /// Dataset name to analyze
    pub dataset_name: Option<String>,
    /// Time range for analysis
    pub time_range: Option<String>,
}
/// Optimization request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Optimization operation
pub struct OptimizationRequest {
    /// Target to optimize (pool, dataset, or system)
    pub target: String,
    /// Optimization type
    pub optimization_type: String,
    /// Priority level
    pub priority: Option<u8>,
}
/// Manifest provisioning request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `ManifestProvisioning` operation
pub struct ManifestProvisioningRequest {
    /// Service manifest
    pub manifest: serde_json::Value,
    /// Provisioning options
    pub options: Option<HashMap<String, String>>,
}
/// Management volume request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `ManagementVolume` operation
pub struct ManagementVolumeRequest {
    /// Volume name
    pub name: String,
    /// Size in bytes
    pub size: u64,
    /// Volume type
    pub volume_type: String,
    /// Additional properties
    pub properties: Option<HashMap<String, String>>,
}
/// Agent runtime request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `AgentRuntime` operation
pub struct AgentRuntimeRequest {
    /// Agent name
    pub name: String,
    /// Agent type
    pub agent_type: String,
    /// Runtime configuration
    pub config: serde_json::Value,
}
/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for `HealthCheck` operation
pub struct HealthCheckResponse {
    /// Overall health status
    pub status: String,
    /// Individual component health
    pub components: HashMap<String, String>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
/// Status response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Status operation
pub struct StatusResponse {
    /// System status
    pub status: String,
    /// Uptime in seconds
    pub uptime: u64,
    /// Version information
    pub version: String,
    /// Additional _metadata
    pub _metadata: HashMap<String, serde_json::Value>,
}
