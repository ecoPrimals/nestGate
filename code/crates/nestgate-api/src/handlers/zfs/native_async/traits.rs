/// Native Async ZFS Traits - Zero-Cost Abstractions
/// Extracted from native_async_zfs.rs to maintain file size compliance
/// Contains trait definitions for ZFS storage operations without Future boxing overhead

use std::future::Future;

use super::super::universal_zfs::types::{
    DatasetConfig, DatasetInfo, PoolConfig, PoolInfo, ServiceMetrics,
    SnapshotConfig, SnapshotInfo, UniversalZfsResult,
};

/// Native async universal ZFS service trait - replaces #[async_trait] UniversalZfsService
pub trait NativeAsyncUniversalZfsService<
    const MAX_POOLS: usize = 1000,
    const MAX_DATASETS: usize = 10000,
    const MAX_SNAPSHOTS: usize = 100000,
    const HEALTH_CHECK_TIMEOUT_SECS: u64 = 30,
>: Send + Sync
{
    type Pool: Clone + Send + Sync + 'static;
    type Dataset: Clone + Send + Sync + 'static;
    type Snapshot: Clone + Send + Sync + 'static;
    type Health: Clone + Send + Sync + 'static;
    type Metrics: Clone + Send + Sync + 'static;

    /// Service identification - synchronous methods
    fn service_name(&self) -> &str;
    fn service_version(&self) -> &str;

    /// Health and status operations - native async, no Future boxing
    fn health_check(&self) -> impl Future<Output = UniversalZfsResult<Self::Health>> + Send;
    fn get_metrics(&self) -> impl Future<Output = UniversalZfsResult<Self::Metrics>> + Send;
    fn is_available(&self) -> impl Future<Output = bool> + Send;

    /// Pool operations - direct async methods
    fn list_pools(&self) -> impl Future<Output = UniversalZfsResult<Vec<Self::Pool>>> + Send;
    fn get_pool(
        &self,
        name: &str,
    ) -> impl Future<Output = UniversalZfsResult<Option<Self::Pool>>> + Send;
    fn create_pool(
        &self,
        config: &PoolConfig,
    ) -> impl Future<Output = UniversalZfsResult<Self::Pool>> + Send;
    fn destroy_pool(
        &self,
        name: &str,
    ) -> impl Future<Output = UniversalZfsResult<()>> + Send;

    /// Dataset operations - zero-copy async methods
    fn list_datasets(
        &self,
        pool_name: Option<&str>,
    ) -> impl Future<Output = UniversalZfsResult<Vec<Self::Dataset>>> + Send;
    fn get_dataset(
        &self,
        name: &str,
    ) -> impl Future<Output = UniversalZfsResult<Option<Self::Dataset>>> + Send;
    fn create_dataset(
        &self,
        config: &DatasetConfig,
    ) -> impl Future<Output = UniversalZfsResult<Self::Dataset>> + Send;
    fn destroy_dataset(
        &self,
        name: &str,
    ) -> impl Future<Output = UniversalZfsResult<()>> + Send;

    /// Snapshot operations - efficient async methods
    fn list_snapshots(
        &self,
        dataset_name: Option<&str>,
    ) -> impl Future<Output = UniversalZfsResult<Vec<Self::Snapshot>>> + Send;
    fn create_snapshot(
        &self,
        config: &SnapshotConfig,
    ) -> impl Future<Output = UniversalZfsResult<Self::Snapshot>> + Send;
    fn destroy_snapshot(
        &self,
        name: &str,
    ) -> impl Future<Output = UniversalZfsResult<()>> + Send;

    /// Advanced operations - bulk and batch methods
    fn bulk_create_snapshots(
        &self,
        configs: &[SnapshotConfig],
    ) -> impl Future<Output = UniversalZfsResult<Vec<Self::Snapshot>>> + Send;
    fn clone_dataset(
        &self,
        snapshot_name: &str,
        new_dataset_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<Self::Dataset>> + Send;
    }

/// Native async ZFS pool management trait
pub trait NativeAsyncZfsPoolManager<const MAX_POOLS: usize = 1000>: Send + Sync {
    type PoolInfo: Clone + Send + Sync + 'static;
    type PoolHealth: Clone + Send + Sync + 'static;
    
    /// Pool lifecycle management
    fn create_pool(
        &self,
        config: &PoolConfig,
    ) -> impl Future<Output = UniversalZfsResult<Self::PoolInfo>> + Send;
    
    fn import_pool(
        &self,
        pool_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<Self::PoolInfo>> + Send;
    
    fn export_pool(
        &self,
        pool_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<()>> + Send;
    
    /// Pool health and maintenance
    fn scrub_pool(
        &self,
        pool_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<()>> + Send;
    
    fn get_pool_health(
        &self,
        pool_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<Self::PoolHealth>> + Send;
    }

/// Native async ZFS dataset management trait
pub trait NativeAsyncZfsDatasetManager<const MAX_DATASETS: usize = 10000>: Send + Sync {
    type DatasetInfo: Clone + Send + Sync + 'static;
    
    /// Dataset operations
    fn create_filesystem(
        &self,
        name: &str,
        properties: &std::collections::HashMap<String, String>,
    ) -> impl Future<Output = UniversalZfsResult<Self::DatasetInfo>> + Send;
    
    fn create_volume(
        &self,
        name: &str,
        size: u64,
        properties: &std::collections::HashMap<String, String>,
    ) -> impl Future<Output = UniversalZfsResult<Self::DatasetInfo>> + Send;
    
    fn set_property(
        &self,
        dataset_name: &str,
        property: &str,
        value: &str,
    ) -> impl Future<Output = UniversalZfsResult<()>> + Send;
    
    fn get_properties(
        &self,
        dataset_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<std::collections::HashMap<String, String>>> + Send;
    }

/// Native async ZFS snapshot management trait
pub trait NativeAsyncZfsSnapshotManager<const MAX_SNAPSHOTS: usize = 100000>: Send + Sync {
    type SnapshotInfo: Clone + Send + Sync + 'static;
    
    /// Snapshot operations
    fn create_recursive_snapshot(
        &self,
        dataset_name: &str,
        snapshot_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<Vec<Self::SnapshotInfo>>> + Send;
    
    fn rollback_to_snapshot(
        &self,
        snapshot_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<()>> + Send;
    
    fn send_snapshot(
        &self,
        snapshot_name: &str,
        destination: &str,
    ) -> impl Future<Output = UniversalZfsResult<()>> + Send;
    
    fn receive_snapshot(
        &self,
        source: &str,
        destination: &str,
    ) -> impl Future<Output = UniversalZfsResult<Self::SnapshotInfo>> + Send;
    }

/// Native async ZFS monitoring trait
pub trait NativeAsyncZfsMonitor: Send + Sync {
    type ServiceMetrics: Clone + Send + Sync + 'static;
    type PoolMetrics: Clone + Send + Sync + 'static;
    type DatasetMetrics: Clone + Send + Sync + 'static;
    
    /// Monitoring operations
    fn get_service_metrics(
        &self,
    ) -> impl Future<Output = UniversalZfsResult<Self::ServiceMetrics>> + Send;
    
    fn get_pool_metrics(
        &self,
        pool_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<Self::PoolMetrics>> + Send;
    
    fn get_dataset_metrics(
        &self,
        dataset_name: &str,
    ) -> impl Future<Output = UniversalZfsResult<Self::DatasetMetrics>> + Send;
    
    fn get_io_stats(
        &self,
        name: &str,
    ) -> impl Future<Output = UniversalZfsResult<std::collections::HashMap<String, u64>>> + Send;
} 