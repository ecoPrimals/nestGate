// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Unified storage type definitions for `NestGate`.
///
/// Covers consistent storage operations across all storage backends and services.
use serde::{Deserialize, Serialize};
/// Unified storage backend types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Types of `UnifiedStorage`
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
    /// Returns the default instance
    fn default() -> Self {
        Self::Local
    }
}

/// Storage operation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Storageoperation
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
/// Storagetier
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
    /// Returns the default instance
    fn default() -> Self {
        Self::Hot
    }
}

impl StorageTier {
    /// Get the expected access frequency for this tier
    #[must_use]
    pub const fn access_frequency(&self) -> &'static str {
        match self {
            Self::Hot => "multiple times per day",
            Self::Warm => "weekly to monthly",
            Self::Cool => "monthly to yearly",
            Self::Cold => "yearly or less",
            Self::Frozen => "archival only",
        }
    }

    /// Get the relative cost factor for this tier (Hot = 1.0)
    #[must_use]
    pub const fn cost_factor(&self) -> f32 {
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
/// Storagecapacity
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
    #[must_use]
    pub fn new(total_bytes: u64, used_bytes: u64) -> Self {
        let available_bytes = total_bytes.saturating_sub(used_bytes);
        let utilization_percent = if total_bytes > 0 {
            let bps = (u128::from(used_bytes).saturating_mul(10_000) / u128::from(total_bytes))
                .min(10_000) as u32;
            f64::from(bps) / 100.0
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
    #[must_use]
    pub fn is_nearly_full(&self) -> bool {
        self.utilization_percent > 90.0
    }

    /// Check if storage is critically full (>95% utilization)
    #[must_use]
    pub fn is_critically_full(&self) -> bool {
        self.utilization_percent > 95.0
    }
}

impl Default for StorageCapacity {
    /// Returns the default instance
    fn default() -> Self {
        Self::new(0, 0)
    }
}

#[cfg(test)]
mod storage_types_coverage_tests {
    use super::*;

    #[test]
    fn storage_capacity_new_and_flags() {
        let c = StorageCapacity::new(100, 50);
        assert_eq!(c.available_bytes, 50);
        assert!((c.utilization_percent - 50.0).abs() < 0.01);
        assert!(!c.is_nearly_full());
        let full = StorageCapacity::new(100, 92);
        assert!(full.is_nearly_full());
        assert!(!full.is_critically_full());
        let crit = StorageCapacity::new(100, 96);
        assert!(crit.is_critically_full());
    }

    #[test]
    fn storage_tier_access_and_cost() {
        assert_eq!(
            StorageTier::Hot.access_frequency(),
            "multiple times per day"
        );
        assert_eq!(StorageTier::Frozen.cost_factor(), 0.1);
    }

    #[test]
    fn unified_storage_type_roundtrip() {
        for v in [
            UnifiedStorageType::Local,
            UnifiedStorageType::Zfs,
            UnifiedStorageType::Custom("x".into()),
        ] {
            let j = serde_json::to_string(&v).unwrap();
            let back: UnifiedStorageType = serde_json::from_str(&j).unwrap();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn storage_operation_serde_roundtrip() {
        let op = StorageOperation::Sync;
        let j = serde_json::to_string(&op).unwrap();
        let back: StorageOperation = serde_json::from_str(&j).unwrap();
        assert_eq!(op, back);
    }
}

/// Storage capability flags for different storage backend features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Unifiedstoragecapability
pub enum UnifiedStorageCapability {
    /// Compression support
    Compression,
    /// Encryption support
    Encryption,
    /// Deduplication support
    Deduplication,
    /// Snapshot support
    Snapshots,
    /// Journaling support
    Journaling,
    /// Replication support
    Replication,
    /// Versioning support
    Versioning,
    /// Backup support
    Backup,
}
