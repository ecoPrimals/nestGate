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

#[cfg(test)]
mod snapshot_path_tests;
#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod test_zfs_stub;
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
