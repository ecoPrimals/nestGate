// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `ZeroCostZfsOperations` trait wiring for [`super::ZeroCostZfsManager`].

use super::ZeroCostZfsManager;
use crate::zero_cost_zfs_operations::traits::ZeroCostZfsOperations;
use crate::zero_cost_zfs_operations::types::{
    ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo,
};
use nestgate_core::Result;
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsOperations<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS>
    for ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    /// Type alias for Error
    type Error = nestgate_core::NestGateError;
    /// Type alias for Pool
    type Pool = ZeroCostPoolInfo;
    /// Type alias for Dataset
    type Dataset = ZeroCostDatasetInfo;
    /// Type alias for Snapshot
    type Snapshot = ZeroCostSnapshotInfo;
    /// Type alias for Properties
    type Properties = HashMap<String, String>;

    async fn create_pool(&self, name: &str, devices: &[&str]) -> Result<Self::Pool> {
        self.pool_create(name, devices).await
    }

    async fn create_dataset(
        &self,
        pool: &Self::Pool,
        name: &str,
        tier: StorageTier,
    ) -> Result<Self::Dataset> {
        self.dataset_create(pool, name, tier).await
    }

    async fn create_snapshot(&self, dataset: &Self::Dataset, name: &str) -> Result<Self::Snapshot> {
        self.snapshot_create(dataset, name).await
    }

    async fn get_pool_properties(&self, pool: &Self::Pool) -> Result<Self::Properties> {
        self.pool_get_properties(pool).await
    }

    async fn list_pools(&self) -> Result<Vec<Self::Pool>> {
        self.pool_list().await
    }

    async fn list_datasets(&self, pool: &Self::Pool) -> Result<Vec<Self::Dataset>> {
        self.dataset_list(pool).await
    }

    async fn list_snapshots(&self, dataset: &Self::Dataset) -> Result<Vec<Self::Snapshot>> {
        self.snapshot_list(dataset).await
    }
}
