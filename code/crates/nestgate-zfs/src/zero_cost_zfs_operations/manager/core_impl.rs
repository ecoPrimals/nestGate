// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Core construction, timeouts, and API compatibility helpers.

use super::ZeroCostZfsManager;
use nestgate_core::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::Duration;
use tokio::sync::RwLock;

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> Default for ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    /// Create new ZFS manager with compile-time configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            pools: Arc::new(RwLock::new(HashMap::with_capacity(MAX_POOLS))),
            datasets: Arc::new(RwLock::new(HashMap::with_capacity(MAX_DATASETS))),
            snapshots: Arc::new(RwLock::new(HashMap::with_capacity(MAX_SNAPSHOTS))),
            request_id_counter: AtomicU64::new(0),
        }
    }

    /// Get command timeout at compile-time
    #[must_use]
    pub const fn command_timeout() -> Duration {
        Duration::from_millis(COMMAND_TIMEOUT_MS)
    }

    /// Set dataset properties - API compatibility method
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &std::collections::HashMap<String, String>,
    ) -> Result<()> {
        // Implementation using ZFS set command
        for (key, value) in properties {
            self.execute_zfs_command(&["set", &format!("{key}={value}"), dataset_name])
                .await?;
        }
        Ok(())
    }

    /// Destroy snapshot - API compatibility method
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn destroy_snapshot(&self, snapshot_name: &str) -> Result<()> {
        // Implementation using ZFS destroy command
        self.execute_zfs_command(&["destroy", snapshot_name])
            .await?;
        Ok(())
    }
}
