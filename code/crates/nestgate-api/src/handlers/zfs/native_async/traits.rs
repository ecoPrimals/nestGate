// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(async_fn_in_trait)] // Intentional ergonomic async trait for adapter implementations

//! Async ZFS service trait for the `dev-stubs` native async adapter.

use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsResult,
};

/// Native async interface for Universal ZFS operations (dev-stubs / adapter builds).
pub trait NativeAsyncUniversalZfsService: Send + Sync {
    /// Service display name.
    fn service_name(&self) -> &str;
    /// Service version string.
    fn service_version(&self) -> &str;

    /// Run a health check against the ZFS subsystem.
    async fn health_check(&self) -> UniversalZfsResult<HealthStatus>;
    /// Collect current service metrics (pool counts, operation stats).
    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics>;
    /// Quick probe: is the ZFS backend reachable?
    async fn is_available(&self) -> bool;

    /// List all ZFS pools visible to the service.
    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>>;
    /// Retrieve a single pool by name, or `None` if it does not exist.
    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>>;
    /// Create a new ZFS pool from the given configuration.
    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo>;
    /// Destroy a ZFS pool by name.
    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()>;

    /// List datasets, optionally filtered to a single pool.
    async fn list_datasets(&self, pool_name: Option<&str>) -> UniversalZfsResult<Vec<DatasetInfo>>;
    /// Retrieve a single dataset by name, or `None` if it does not exist.
    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>>;
    /// Create a new dataset from the given configuration.
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo>;
    /// Destroy a dataset by name.
    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()>;

    /// List snapshots, optionally filtered to a single dataset.
    async fn list_snapshots(
        &self,
        dataset_name: Option<&str>,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>>;
    /// Create a snapshot from the given configuration.
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo>;
    /// Destroy a snapshot by name.
    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()>;
    /// Create multiple snapshots atomically.
    async fn bulk_create_snapshots(
        &self,
        configs: &[SnapshotConfig],
    ) -> UniversalZfsResult<Vec<SnapshotInfo>>;

    /// Clone a dataset from an existing snapshot.
    async fn clone_dataset(
        &self,
        snapshot_name: &str,
        new_dataset_name: &str,
    ) -> UniversalZfsResult<DatasetInfo>;
}
