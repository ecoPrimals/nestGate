// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![allow(async_fn_in_trait)] // Intentional ergonomic async trait for adapter implementations

//! Async ZFS service trait for the `dev-stubs` native async adapter.

use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsResult,
};

/// Native async interface for Universal ZFS operations (dev-stubs / adapter builds).
pub trait NativeAsyncUniversalZfsService: Send + Sync {
    /// Service display name
    fn service_name(&self) -> &str;
    /// Service version string
    fn service_version(&self) -> &str;

    async fn health_check(&self) -> UniversalZfsResult<HealthStatus>;
    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics>;
    async fn is_available(&self) -> bool;

    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>>;
    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>>;
    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo>;
    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()>;

    async fn list_datasets(&self, pool_name: Option<&str>) -> UniversalZfsResult<Vec<DatasetInfo>>;
    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>>;
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo>;
    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()>;

    async fn list_snapshots(
        &self,
        dataset_name: Option<&str>,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>>;
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo>;
    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()>;
    async fn bulk_create_snapshots(
        &self,
        configs: &[SnapshotConfig],
    ) -> UniversalZfsResult<Vec<SnapshotInfo>>;

    async fn clone_dataset(
        &self,
        snapshot_name: &str,
        new_dataset_name: &str,
    ) -> UniversalZfsResult<DatasetInfo>;
}
