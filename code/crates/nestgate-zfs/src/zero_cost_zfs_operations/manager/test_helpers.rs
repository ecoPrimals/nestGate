// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Test-only helpers for exercising internal manager behavior.

use super::ZeroCostZfsManager;
use crate::zero_cost_zfs_operations::types::{
    ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo,
};
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;
use std::time::SystemTime;

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    pub(crate) fn test_parse_pool_properties(&self, output: &str) -> HashMap<String, String> {
        self.parse_pool_properties(output)
    }

    pub(crate) async fn test_insert_pool_entry(&self, name: String) {
        let mut p = self.pools.write().await;
        p.insert(
            name.clone(),
            ZeroCostPoolInfo {
                name: name.clone(),
                size: 0,
                used: 0,
                available: 0,
                health: "ONLINE".into(),
                properties: HashMap::new(),
                created_at: SystemTime::UNIX_EPOCH,
            },
        );
    }

    pub(crate) async fn test_pool_map_len(&self) -> usize {
        self.pools.read().await.len()
    }

    pub(crate) async fn test_can_create_more_pools(&self) -> bool {
        self.can_create_more_pools().await
    }

    pub(crate) async fn test_can_create_more_datasets(&self) -> bool {
        self.can_create_more_datasets().await
    }

    pub(crate) async fn test_insert_dataset_entry(&self, name: String, pool: String) {
        let mut d = self.datasets.write().await;
        d.insert(
            name.clone(),
            ZeroCostDatasetInfo {
                name: name.clone(),
                pool,
                tier: StorageTier::Warm,
                size: 0,
                used: 0,
                properties: HashMap::new(),
                mount_point: None,
                created_at: SystemTime::UNIX_EPOCH,
            },
        );
    }

    pub(crate) async fn test_can_create_more_snapshots(&self) -> bool {
        self.can_create_more_snapshots().await
    }

    pub(crate) async fn test_insert_snapshot_entry(&self, name: String) {
        let mut s = self.snapshots.write().await;
        s.insert(
            name.clone(),
            ZeroCostSnapshotInfo {
                name: name.clone(),
                dataset: "pool/ds".into(),
                size: 0,
                created_at: SystemTime::UNIX_EPOCH,
                properties: HashMap::new(),
            },
        );
    }

    pub(crate) async fn test_snapshot_map_len(&self) -> usize {
        self.snapshots.read().await.len()
    }
}
