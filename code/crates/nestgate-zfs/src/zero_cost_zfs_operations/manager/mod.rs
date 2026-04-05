// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZERO-COST ZFS MANAGER**
//! Main implementation of zero-cost ZFS operations manager

mod aliases;
mod command;
mod core_impl;
mod dataset_ops;
mod pool_ops;
mod snapshot_ops;
mod trait_impl;
mod validation;

pub use dataset_ops::{build_dataset_create_zfs_args, parse_dataset_list_line};
pub use pool_ops::{
    build_pool_create_zfs_args, parse_pool_list_line, zero_cost_pool_from_zfs_properties,
};
pub use snapshot_ops::{build_snapshot_zfs_path, parse_snapshot_list_line};

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod tests;

pub use aliases::{
    DevelopmentZfsManager, EnterpriseZfsManager, HighPerformanceZfsManager, ProductionZfsManager,
    TestingZfsManager,
};

use super::types::{DatasetInfoMap, PoolInfoMap, SnapshotInfoMap};
use std::sync::atomic::AtomicU64;

/// Zero-cost ZFS operations manager with compile-time capacity limits
pub struct ZeroCostZfsManager<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> {
    pub(super) pools: PoolInfoMap,
    pub(super) datasets: DatasetInfoMap,
    pub(super) snapshots: SnapshotInfoMap,
    pub(super) request_id_counter: AtomicU64,
}
