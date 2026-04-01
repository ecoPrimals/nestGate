// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Compile-time-limited capacity checks against in-memory maps.

use super::ZeroCostZfsManager;

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    /// Check if we can create more pools
    pub(super) async fn can_create_more_pools(&self) -> bool {
        let pools = self.pools.read().await;
        pools.len() < MAX_POOLS
    }

    /// Check if we can create more datasets
    pub(super) async fn can_create_more_datasets(&self) -> bool {
        let datasets = self.datasets.read().await;
        datasets.len() < MAX_DATASETS
    }

    /// Check if we can create more snapshots
    pub(super) async fn can_create_more_snapshots(&self) -> bool {
        let snapshots = self.snapshots.read().await;
        snapshots.len() < MAX_SNAPSHOTS
    }
}
