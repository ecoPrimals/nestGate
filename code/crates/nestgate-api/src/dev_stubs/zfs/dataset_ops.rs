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

pub trait DatasetOperations {
    fn create_dataset(&self, name: &str) -> Result<(), ZfsError>;
    fn list_datasets(&self, pool: &str) -> Result<Vec<ZeroCostDatasetInfo>, ZfsError>;
    fn create_dataset_with_tier(
        &self,
        pool: &str,
        name: &str,
        tier: StorageTier,
    ) -> Result<ZeroCostDatasetInfo, ZfsError>;
}

impl DatasetOperations for ProductionZfsManager {
    fn create_dataset(&self, _name: &str) -> Result<(), ZfsError> {
        Ok(())
    }

    fn list_datasets(&self, pool: &str) -> Result<Vec<ZeroCostDatasetInfo>, ZfsError> {
        Ok(vec![ZeroCostDatasetInfo {
            name: format!("{}/data", pool),
            used: 100_000_000_000,
            available: 900_000_000_000,
            referenced: 100_000_000_000,
            mountpoint: format!("/{}/data", pool),
            mounted: true,
        }])
    }

    fn create_dataset_with_tier(
        &self,
        pool: &str,
        name: &str,
        _tier: StorageTier,
    ) -> Result<ZeroCostDatasetInfo, ZfsError> {
        Ok(ZeroCostDatasetInfo {
            name: format!("{}/{}", pool, name),
            used: 0,
            available: 1_000_000_000,
            referenced: 0,
            mountpoint: format!("/{}/{}", pool, name),
            mounted: true,
        })
    }
}
