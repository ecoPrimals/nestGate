// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage API: pools, datasets, snapshots, and probe helpers.

mod http_handlers;
mod probes;
mod types;

#[cfg(test)]
mod tests;

/// **STORAGE HANDLER**
///
/// Main handler for storage operations and management.
#[derive(Debug, Clone)]
/// Handler for Storage requests
pub struct StorageHandler;

impl Default for StorageHandler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl StorageHandler {
    /// Create a new storage handler instance
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

pub use http_handlers::{
    get_storage_datasets, get_storage_metrics, get_storage_pools, get_storage_snapshots,
};
pub use types::{
    StorageDataset, StorageDatasetInfo, StorageManager, StorageMetrics, StoragePool,
    StoragePoolInfo, StorageSnapshot, StorageSnapshotInfo,
};
