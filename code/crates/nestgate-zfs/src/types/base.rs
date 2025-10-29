//! Core ZFS types: Pool, Dataset, Snapshot, and basic structures

use crate::error::{create_zfs_error, ZfsOperation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Re-export core storage tier for convenience
pub use nestgate_core::canonical_types::StorageTier;

// Re-export canonical error types
pub use nestgate_core::error::{Result, ZfsResult};

// ==================== CANONICAL ERROR TYPES ====================

/// ZFS-specific error type for backward compatibility
#[derive(Debug, thiserror::Error)]
pub enum ZfsError {
    #[error("Pool operation failed: {message}")]
    PoolError { message: String },
    #[error("Dataset operation failed: {message}")]
    DatasetError { message: String },
    #[error("Snapshot operation failed: {message}")]
    SnapshotError { message: String },
    #[error("Command execution failed: {message}")]
    CommandError { message: String },
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// ==================== POOL TYPES ====================

/// ZFS pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub name: String,
    pub size: u64,
    pub allocated: u64,
    pub free: u64,
    pub health: PoolHealth,
    pub state: PoolState,
    pub capacity: PoolCapacity,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolHealth {
    Online,
    Degraded,
    Faulted,
    Offline,
    Removed,
    Unavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolState {
    Active,
    Exported,
    Destroyed,
    Spare,
    L2cache,
    Uninitialized,
    Unavailable,
    PotentiallyActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCapacity {
    pub size: u64,
    pub allocated: u64,
    pub free: u64,
}

// ==================== DATASET TYPES ====================

/// ZFS dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub name: String,
    pub used: u64,
    pub available: u64,
    pub referenced: u64,
    pub mountpoint: Option<PathBuf>,
    pub tier: StorageTier,
    pub properties: HashMap<String, String>,
    pub creation_time: SystemTime,
}

// ==================== SNAPSHOT TYPES ====================

/// ZFS snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    pub name: String,
    pub creation_time: SystemTime,
    pub used: u64,
    pub referenced: u64,
    pub properties: HashMap<String, String>,
}

// ==================== ZERO-COST TYPES ====================

/// Zero-cost pool info with compile-time optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostPoolInfo {
    pub name: String,
    pub size: u64,
    pub allocated: u64,
    pub free: u64,
    pub health: PoolHealth,
    pub state: PoolState,
    pub capacity_percent: f64,
    pub properties: HashMap<String, String>,
}

/// Zero-cost dataset info with compile-time optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostDatasetInfo {
    pub name: String,
    pub used: u64,
    pub available: u64,
    pub referenced: u64,
    pub mountpoint: Option<PathBuf>,
    pub tier: StorageTier,
    pub properties: HashMap<String, String>,
    pub last_modified: SystemTime,
}

/// Zero-cost snapshot info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostSnapshotInfo {
    pub name: String,
    pub creation_time: SystemTime,
    pub used: u64,
    pub referenced: u64,
}

// ==================== STATUS TYPES ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PoolStatus {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}

