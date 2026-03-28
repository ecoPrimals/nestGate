// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **CANONICAL ZFS SERVICE TRAIT - COMPREHENSIVE UNIFICATION**
//
// This trait provides the complete canonical interface for all ZFS backend implementations.
// It unifies all methods from native, remote, and fail-safe backends into a single consistent API.

//! Traits module

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsResult,
};

use crate::handlers::zfs::universal_zfs::backends::native::core::NativeZfsService;
use crate::handlers::zfs::universal_zfs::fail_safe::core::FailSafeZfsService;

/// **CANONICAL UNIVERSAL ZFS SERVICE TRAIT**
///
/// This trait defines the complete interface that all ZFS backend implementations must provide.
/// It includes all methods from `native_real`, remote, and `fail_safe` implementations.
/// **CANONICAL MODERNIZATION**: Dyn-compatible explicit future boxing for `Arc<dyn UniversalZfsService>`
pub trait UniversalZfsService: Send + Sync {
    // ==================== CORE SERVICE METHODS ====================
    /// Get the service name
    fn service_name(&self) -> &str;

    /// Get the service version  
    fn service_version(&self) -> &str;

    /// Perform a health check on the service
    fn health_check(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<HealthStatus>> + Send + '_>>;

    /// Get service metrics
    fn get_metrics(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<ServiceMetrics>> + Send + '_>>;

    /// Check if the service is available
    fn is_available(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>>;

    /// Shutdown the service gracefully
    fn shutdown(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>>;

    // ==================== POOL OPERATIONS ====================

    /// List all pools
    fn list_pools(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<PoolInfo>>> + Send + '_>>;

    /// Create a new pool
    fn create_pool(
        &self,
        config: &PoolConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<PoolInfo>> + Send + '_>>;

    /// Get information about a specific pool
    fn get_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<PoolInfo>>> + Send + '_>>;

    /// Destroy a pool
    fn destroy_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>>;

    /// Scrub a pool (data integrity check)
    fn scrub_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>>;

    /// Get pool status information
    fn get_pool_status(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>>;

    // ==================== DATASET OPERATIONS ====================

    /// List all datasets
    fn list_datasets(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<DatasetInfo>>> + Send + '_>>;

    /// Create a new dataset
    fn create_dataset(
        &self,
        config: &DatasetConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<DatasetInfo>> + Send + '_>>;

    /// Get information about a specific dataset
    fn get_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<DatasetInfo>>> + Send + '_>>;

    /// Destroy a dataset
    fn destroy_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>>;

    /// Set dataset properties
    fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>>;

    /// Get dataset properties
    fn get_dataset_properties(
        &self,
        dataset_name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<HashMap<String, String>>> + Send + '_>>;

    // ==================== SNAPSHOT OPERATIONS ====================

    /// List all snapshots
    fn list_snapshots(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<SnapshotInfo>>> + Send + '_>>;

    /// Create a snapshot
    fn create_snapshot(
        &self,
        config: &SnapshotConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<SnapshotInfo>> + Send + '_>>;

    /// List snapshots for a specific dataset
    fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<SnapshotInfo>>> + Send + '_>>;

    /// Destroy a snapshot
    fn destroy_snapshot(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>>;

    // ==================== OPTIMIZATION & CONFIGURATION ====================

    /// Optimize ZFS configuration
    fn optimize(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>>;

    /// Get optimization analytics
    fn get_optimization_analytics(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<serde_json::Value>> + Send + '_>>;

    /// Predict optimal tier for data
    fn predict_tier(
        &self,
        file_path: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>>;

    /// Get current configuration
    fn get_configuration(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<serde_json::Value>> + Send + '_>>;

    /// Update configuration
    fn update_configuration(
        &self,
        config: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>>;
}

/// **DYN-COMPATIBLE ZFS SERVICE WRAPPER**
/// Wrapper enum for dynamic dispatch of ZFS services
#[derive(Debug)]
/// Dynzfsservice
pub enum DynZfsService {
    /// Native ZFS service implementation
    Native(NativeZfsService),
    /// Fail-safe ZFS service with circuit breaker
    FailSafe(FailSafeZfsService),
}
impl DynZfsService {
    /// Service name
    #[must_use]
    pub fn service_name(&self) -> &str {
        match self {
            Self::Native(service) => service.service_name(),
            Self::FailSafe(service) => service.service_name(),
        }
    }

    /// Service version
    #[must_use]
    pub fn service_version(&self) -> &str {
        match self {
            Self::Native(service) => service.service_version(),
            Self::FailSafe(service) => service.service_version(),
        }
    }

    /// Health check
    pub async fn health_check(&self) -> UniversalZfsResult<HealthStatus> {
        match self {
            Self::Native(service) => service.health_check().await,
            Self::FailSafe(service) => service.health_check().await,
        }
    }

    /// Get metrics
    pub async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics> {
        match self {
            Self::Native(service) => service.get_metrics().await,
            Self::FailSafe(service) => service.get_metrics().await,
        }
    }

    /// Check if available
    pub async fn is_available(&self) -> bool {
        match self {
            Self::Native(service) => service.is_available().await,
            Self::FailSafe(service) => service.is_available().await,
        }
    }

    /// Shutdown
    pub async fn shutdown(&self) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.shutdown().await,
            Self::FailSafe(service) => service.shutdown().await,
        }
    }

    /// Create pool
    pub async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        match self {
            Self::Native(service) => service.create_pool(config).await,
            Self::FailSafe(service) => service.create_pool(config).await,
        }
    }

    /// Get pool
    pub async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        match self {
            Self::Native(service) => service.get_pool(name).await,
            Self::FailSafe(service) => service.get_pool(name).await,
        }
    }

    /// Destroy pool
    pub async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_pool(name).await,
            Self::FailSafe(service) => service.destroy_pool(name).await,
        }
    }

    /// Create snapshot
    pub async fn create_snapshot(
        &self,
        config: &SnapshotConfig,
    ) -> UniversalZfsResult<SnapshotInfo> {
        match self {
            Self::Native(service) => service.create_snapshot(config).await,
            Self::FailSafe(service) => service.create_snapshot(config).await,
        }
    }

    /// Destroy snapshot
    pub async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_snapshot(name).await,
            Self::FailSafe(service) => service.destroy_snapshot(name).await,
        }
    }

    /// List datasets
    pub async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>> {
        match self {
            Self::Native(service) => service.list_datasets().await,
            Self::FailSafe(service) => service.list_datasets().await,
        }
    }

    /// Scrub pool
    pub async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.scrub_pool(name).await,
            Self::FailSafe(service) => service.scrub_pool(name).await,
        }
    }

    /// Get pool status
    pub async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.get_pool_status(name).await,
            Self::FailSafe(service) => service.get_pool_status(name).await,
        }
    }

    /// Create dataset
    pub async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        match self {
            Self::Native(service) => service.create_dataset(config).await,
            Self::FailSafe(service) => service.create_dataset(config).await,
        }
    }

    /// Get dataset
    pub async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        match self {
            Self::Native(service) => service.get_dataset(name).await,
            Self::FailSafe(service) => service.get_dataset(name).await,
        }
    }

    /// Destroy dataset
    pub async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_dataset(name).await,
            Self::FailSafe(service) => service.destroy_dataset(name).await,
        }
    }

    /// Set dataset properties
    pub async fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => {
                service
                    .set_dataset_properties(dataset_name, properties)
                    .await
            }
            Self::FailSafe(service) => {
                service
                    .set_dataset_properties(dataset_name, properties)
                    .await
            }
        }
    }

    /// Get dataset properties
    pub async fn get_dataset_properties(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>> {
        match self {
            Self::Native(service) => service.get_dataset_properties(dataset_name).await,
            Self::FailSafe(service) => service.get_dataset_properties(dataset_name).await,
        }
    }

    /// List snapshots
    pub async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        match self {
            Self::Native(service) => service.list_snapshots().await,
            Self::FailSafe(service) => service.list_snapshots().await,
        }
    }

    /// List dataset snapshots
    pub async fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        match self {
            Self::Native(service) => service.list_dataset_snapshots(dataset_name).await,
            Self::FailSafe(service) => service.list_dataset_snapshots(dataset_name).await,
        }
    }

    /// Optimize
    pub async fn optimize(&self) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.optimize().await,
            Self::FailSafe(service) => service.optimize().await,
        }
    }

    /// Get optimization analytics
    pub async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value> {
        match self {
            Self::Native(service) => service.get_optimization_analytics().await,
            Self::FailSafe(service) => service.get_optimization_analytics().await,
        }
    }

    /// Predict tier
    pub async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.predict_tier(file_path).await,
            Self::FailSafe(service) => service.predict_tier(file_path).await,
        }
    }

    /// Get configuration
    pub async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value> {
        match self {
            Self::Native(service) => service.get_configuration().await,
            Self::FailSafe(service) => service.get_configuration().await,
        }
    }

    /// Update configuration
    pub async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.update_configuration(config).await,
            Self::FailSafe(service) => service.update_configuration(config).await,
        }
    }
}

/// Enum wrapper for ZFS service implementations to enable dyn compatibility
#[derive(Debug)]
/// Universalzfsserviceenum
pub enum UniversalZfsServiceEnum {
    /// Native ZFS service implementation
    Native(NativeZfsService),
    /// Fail-safe ZFS service with circuit breaker
    FailSafe(FailSafeZfsService),
}

impl UniversalZfsServiceEnum {
    /// Create a new native ZFS service
    #[must_use]
    pub fn new_native() -> Self {
        Self::Native(NativeZfsService::new())
    }

    /// Create a new fail-safe ZFS service
    #[must_use]
    pub fn new_fail_safe(
        primary: Arc<Self>,
        config: crate::handlers::zfs::universal_zfs::config::FailSafeConfig,
    ) -> Self {
        Self::FailSafe(FailSafeZfsService::new(primary, config))
    }
}

impl UniversalZfsService for UniversalZfsServiceEnum {
    /// Service Name
    fn service_name(&self) -> &str {
        match self {
            Self::Native(service) => service.service_name(),
            Self::FailSafe(service) => service.service_name(),
        }
    }

    /// Service Version
    fn service_version(&self) -> &str {
        match self {
            Self::Native(service) => service.service_version(),
            Self::FailSafe(service) => service.service_version(),
        }
    }

    /// Health Check
    fn health_check(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<HealthStatus>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.health_check().await,
                Self::FailSafe(service) => service.health_check().await,
            }
        })
    }

    /// Gets Metrics
    fn get_metrics(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<ServiceMetrics>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.get_metrics().await,
                Self::FailSafe(service) => service.get_metrics().await,
            }
        })
    }

    /// Creates  Pool
    fn create_pool(
        &self,
        config: &PoolConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<PoolInfo>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.create_pool(config).await,
                Self::FailSafe(service) => service.create_pool(config).await,
            }
        })
    }

    /// Gets Pool
    fn get_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<PoolInfo>>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.get_pool(name).await,
                Self::FailSafe(service) => service.get_pool(name).await,
            }
        })
    }

    /// Destroy Pool
    fn destroy_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.destroy_pool(name).await,
                Self::FailSafe(service) => service.destroy_pool(name).await,
            }
        })
    }

    /// Scrub Pool
    fn scrub_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.scrub_pool(name).await,
                Self::FailSafe(service) => service.scrub_pool(name).await,
            }
        })
    }

    /// Gets Pool Status
    fn get_pool_status(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.get_pool_status(name).await,
                Self::FailSafe(service) => service.get_pool_status(name).await,
            }
        })
    }

    /// List Datasets
    fn list_datasets(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<DatasetInfo>>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.list_datasets().await,
                Self::FailSafe(service) => service.list_datasets().await,
            }
        })
    }

    /// Creates  Dataset
    fn create_dataset(
        &self,
        config: &DatasetConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<DatasetInfo>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.create_dataset(config).await,
                Self::FailSafe(service) => service.create_dataset(config).await,
            }
        })
    }

    /// Gets Dataset
    fn get_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<DatasetInfo>>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.get_dataset(name).await,
                Self::FailSafe(service) => service.get_dataset(name).await,
            }
        })
    }

    /// Destroy Dataset
    fn destroy_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.destroy_dataset(name).await,
                Self::FailSafe(service) => service.destroy_dataset(name).await,
            }
        })
    }

    /// Sets Dataset Properties
    fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => {
                    service
                        .set_dataset_properties(dataset_name, properties)
                        .await
                }
                Self::FailSafe(service) => {
                    service
                        .set_dataset_properties(dataset_name, properties)
                        .await
                }
            }
        })
    }

    /// Gets Dataset Properties
    fn get_dataset_properties(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<HashMap<String, String>>> + Send + '_>>
    {
        Box::pin(async {
            match self {
                Self::Native(service) => service.get_dataset_properties(name).await,
                Self::FailSafe(service) => service.get_dataset_properties(name).await,
            }
        })
    }

    /// List Snapshots
    fn list_snapshots(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<SnapshotInfo>>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.list_snapshots().await,
                Self::FailSafe(service) => service.list_snapshots().await,
            }
        })
    }

    /// Creates  Snapshot
    fn create_snapshot(
        &self,
        config: &SnapshotConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<SnapshotInfo>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.create_snapshot(config).await,
                Self::FailSafe(service) => service.create_snapshot(config).await,
            }
        })
    }

    /// List Dataset Snapshots
    fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<SnapshotInfo>>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.list_dataset_snapshots(dataset_name).await,
                Self::FailSafe(service) => service.list_dataset_snapshots(dataset_name).await,
            }
        })
    }

    /// Destroy Snapshot
    fn destroy_snapshot(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.destroy_snapshot(name).await,
                Self::FailSafe(service) => service.destroy_snapshot(name).await,
            }
        })
    }

    /// Optimize
    fn optimize(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.optimize().await,
                Self::FailSafe(service) => service.optimize().await,
            }
        })
    }

    /// Gets Optimization Analytics
    fn get_optimization_analytics(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<serde_json::Value>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.get_optimization_analytics().await,
                Self::FailSafe(service) => service.get_optimization_analytics().await,
            }
        })
    }

    /// Predict Tier
    fn predict_tier(
        &self,
        file_path: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.predict_tier(file_path).await,
                Self::FailSafe(service) => service.predict_tier(file_path).await,
            }
        })
    }

    /// Gets Configuration
    fn get_configuration(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<serde_json::Value>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.get_configuration().await,
                Self::FailSafe(service) => service.get_configuration().await,
            }
        })
    }

    /// Updates  Configuration
    fn update_configuration(
        &self,
        config: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.update_configuration(config).await,
                Self::FailSafe(service) => service.update_configuration(config).await,
            }
        })
    }

    /// Checks if Available
    fn is_available(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.is_available().await,
                Self::FailSafe(service) => service.is_available().await,
            }
        })
    }

    /// Shutdown
    fn shutdown(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.shutdown().await,
                Self::FailSafe(service) => service.shutdown().await,
            }
        })
    }

    /// List Pools
    fn list_pools(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<PoolInfo>>> + Send + '_>> {
        Box::pin(async {
            match self {
                Self::Native(service) => service.list_pools().await,
                Self::FailSafe(service) => service.list_pools().await,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dyn_zfs_service_native_service_name() {
        let service = DynZfsService::Native(NativeZfsService::new());
        assert_eq!(service.service_name(), "native-zfs");
    }

    #[test]
    fn test_dyn_zfs_service_native_service_version() {
        let service = DynZfsService::Native(NativeZfsService::new());
        assert_eq!(service.service_version(), "1.0.0");
    }

    #[test]
    fn test_universal_zfs_service_enum_new_native() {
        let service = UniversalZfsServiceEnum::new_native();
        assert_eq!(service.service_name(), "native-zfs");
    }

    #[test]
    fn test_universal_zfs_service_enum_native_is_available() {
        let service = UniversalZfsServiceEnum::new_native();
        // Is_available is async - we test the sync service_name/version
        assert_eq!(service.service_version(), "1.0.0");
    }

    #[tokio::test]
    async fn test_dyn_zfs_service_health_check() {
        let service = DynZfsService::Native(NativeZfsService::new());
        let result = service.health_check().await;
        // May succeed or fail depending on ZFS availability
        let _ = result;
    }

    #[tokio::test]
    async fn test_universal_zfs_service_enum_list_pools() {
        let service = UniversalZfsServiceEnum::new_native();
        let result = service.list_pools().await;
        // May succeed or fail depending on ZFS availability
        let _ = result;
    }

    #[tokio::test]
    async fn test_universal_zfs_service_enum_list_datasets() {
        let service = UniversalZfsServiceEnum::new_native();
        let result = service.list_datasets().await;
        let _ = result;
    }

    #[tokio::test]
    async fn test_universal_zfs_service_enum_get_metrics() {
        let service = UniversalZfsServiceEnum::new_native();
        let result = service.get_metrics().await;
        let _ = result;
    }
}
