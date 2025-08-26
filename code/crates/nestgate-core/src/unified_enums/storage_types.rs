///
/// Unified storage type definitions for consistent storage operations
/// across all NestGate storage backends and services.
use serde::{Deserialize, Serialize};

/// Unified storage backend types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnifiedStorageType {
    /// Local filesystem storage
    Local,
    /// ZFS storage system
    Zfs,
    /// Network-attached storage
    Network,
    /// Cloud storage (S3, Azure, etc.)
    Cloud,
    /// In-memory storage
    Memory,
    /// Database storage
    Database,
    /// Custom storage backend
    Custom(String),
}

impl Default for UnifiedStorageType {
    fn default() -> Self {
        Self::Local
    }
}

/// Storage operation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageOperation {
    /// Read data from storage
    Read,
    /// Write data to storage
    Write,
    /// Delete data from storage
    Delete,
    /// List storage contents
    List,
    /// Create storage location
    Create,
    /// Move/rename storage item
    Move,
    /// Copy storage item
    Copy,
    /// Sync storage contents
    Sync,
    /// Backup storage contents
    Backup,
    /// Restore from backup
    Restore,
}

/// Storage tier levels for data lifecycle management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageTier {
    /// Hot storage - frequently accessed data
    Hot = 0,
    /// Warm storage - occasionally accessed data
    Warm = 1,
    /// Cool storage - rarely accessed data
    Cool = 2,
    /// Cold storage - archival data
    Cold = 3,
    /// Frozen storage - long-term archival
    Frozen = 4,
}

impl Default for StorageTier {
    fn default() -> Self {
        Self::Hot
    }
}

impl StorageTier {
    /// Get the expected access frequency for this tier
    pub fn access_frequency(&self) -> &'static str {
        match self {
            Self::Hot => "multiple times per day",
            Self::Warm => "weekly to monthly",
            Self::Cool => "monthly to yearly",
            Self::Cold => "yearly or less",
            Self::Frozen => "archival only",
        }
    }

    /// Get the relative cost factor for this tier (Hot = 1.0)
    pub fn cost_factor(&self) -> f32 {
        match self {
            Self::Hot => 1.0,
            Self::Warm => 0.7,
            Self::Cool => 0.4,
            Self::Cold => 0.2,
            Self::Frozen => 0.1,
        }
    }
}

/// Storage capacity and usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapacity {
    /// Total storage capacity in bytes
    pub total_bytes: u64,
    /// Used storage in bytes
    pub used_bytes: u64,
    /// Available storage in bytes
    pub available_bytes: u64,
    /// Storage utilization percentage
    pub utilization_percent: f64,
}

impl StorageCapacity {
    /// Create a new storage capacity report
    pub fn new(total_bytes: u64, used_bytes: u64) -> Self {
        let available_bytes = total_bytes.saturating_sub(used_bytes);
        let utilization_percent = if total_bytes > 0 {
            (used_bytes as f64 / total_bytes as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total_bytes,
            used_bytes,
            available_bytes,
            utilization_percent,
        }
    }

    /// Check if storage is nearly full (>90% utilization)
    pub fn is_nearly_full(&self) -> bool {
        self.utilization_percent > 90.0
    }

    /// Check if storage is critically full (>95% utilization)
    pub fn is_critically_full(&self) -> bool {
        self.utilization_percent > 95.0
    }
}

impl Default for StorageCapacity {
    fn default() -> Self {
        Self::new(0, 0)
    }
}
