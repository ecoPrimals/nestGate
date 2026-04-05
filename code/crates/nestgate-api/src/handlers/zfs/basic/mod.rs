// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module provides HTTP API endpoints for ZFS operations using the new
// canonical zero-cost architecture with compile-time dispatch.

mod datasets;
mod extras;
mod handler_impl;
mod health;
mod pools;
mod service;
mod snapshots;
mod types;

#[cfg(test)]
mod tests;

pub use datasets::{create_dataset, get_dataset, list_datasets};
pub use extras::{
    delete_dataset, delete_snapshot, get_dataset_properties, get_performance_analytics,
    get_pool_status, predict_tier, set_dataset_properties, trigger_optimization,
};
pub use handler_impl::ZfsHandlerImpl;
pub use health::get_zfs_health;
pub use pools::{create_pool, delete_pool, get_pool, list_pools};
pub use snapshots::{create_snapshot, list_snapshots};
pub use types::{
    CreateDatasetRequest, CreatePoolRequest, CreateSnapshotRequest, ZfsHealthResponse,
    evaluate_zfs_health,
};
