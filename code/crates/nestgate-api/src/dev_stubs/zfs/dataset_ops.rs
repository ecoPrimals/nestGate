// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ZFS Dataset Operations (Development Stubs)**
//!
//! Mock implementations for ZFS dataset operations.
//!
//! **Extracted**: November 19, 2025 - From dev_stubs/zfs.rs
//! **Lines**: ~350 (from original 1,015-line file)

#![cfg(feature = "dev-stubs")]

use super::config::ProductionZfsManager;
use super::types::{ZeroCostDatasetInfo, ZfsError};
use nestgate_core::canonical_types::StorageTier;

/// DatasetOperations trait
pub trait DatasetOperations {
    /// Creates  Dataset
    fn create_dataset(&self, name: &str) -> Result<(), ZfsError>;
    /// List Datasets
    fn list_datasets(&self, pool: &str) -> Result<Vec<ZeroCostDatasetInfo>, ZfsError>;
    /// Creates  Dataset With Tier
    fn create_dataset_with_tier(
        &self,
        pool: &str,
        name: &str,
        tier: StorageTier,
    ) -> Result<ZeroCostDatasetInfo, ZfsError>;
}

impl DatasetOperations for ProductionZfsManager {
    /// Creates  Dataset
    fn create_dataset(&self, _name: &str) -> Result<(), ZfsError> {
        Ok(())
    }

    /// List Datasets
    fn list_datasets(&self, pool: &str) -> Result<Vec<ZeroCostDatasetInfo>, ZfsError> {
        Ok(vec![ZeroCostDatasetInfo {
            name: format!("{pool}/data"),
            used: 100_000_000_000,
            available: 900_000_000_000,
            referenced: 100_000_000_000,
            mountpoint: format!("/{pool}/data"),
            mounted: true,
        }])
    }

    /// Creates  Dataset With Tier
    fn create_dataset_with_tier(
        &self,
        pool: &str,
        name: &str,
        _tier: StorageTier,
    ) -> Result<ZeroCostDatasetInfo, ZfsError> {
        Ok(ZeroCostDatasetInfo {
            name: format!("{pool}/{name}"),
            used: 0,
            available: 1_000_000_000,
            referenced: 0,
            mountpoint: format!("/{pool}/{name}"),
            mounted: true,
        })
    }
}
