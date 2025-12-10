//! Zero-cost optimized types for compile-time optimization
//!
//! Domain: Performance-optimized variants with reduced allocations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Re-export core storage tier
pub use nestgate_core::canonical_types::StorageTier;

/// Zero-cost pool information for compile-time optimization
///
/// Optimized variant with reduced allocations and static dispatch.
/// Use this when performance is critical and the overhead of the full
/// `PoolInfo` type is not acceptable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostPoolInfo {
    /// Pool name
    pub name: String,
    /// Total size in bytes
    pub size: u64,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
    /// Health status as string (optimized representation)
    pub health: String,
    /// Pool properties
    pub properties: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: SystemTime,
}

/// Zero-cost dataset information for compile-time optimization
///
/// Optimized variant with reduced allocations and static dispatch.
/// Prefer this over `DatasetInfo` in hot paths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostDatasetInfo {
    /// Dataset name
    pub name: String,
    /// Full qualified name
    pub full_name: String,
    /// Parent pool
    pub pool: String,
    /// Dataset size in bytes
    pub size: u64,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
    /// Mount point if applicable
    pub mount_point: Option<PathBuf>,
    /// Storage tier
    pub tier: StorageTier,
    /// Dataset properties
    pub properties: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: SystemTime,
}

/// Zero-cost snapshot information for compile-time optimization
///
/// Optimized variant with reduced allocations and static dispatch.
/// Use in performance-critical snapshot operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostSnapshotInfo {
    /// Snapshot name
    pub name: String,
    /// Parent dataset
    pub dataset: String,
    /// Snapshot size in bytes
    pub size: u64,
    /// Snapshot properties
    pub properties: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: SystemTime,
}
