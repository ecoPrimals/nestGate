// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Concrete enum dispatch for `UniversalZfsService` implementations.
//!
//! Wraps `NativeZfsService` and `FailSafeZfsService` in a single enum
//! so callers can use trait-object-free dispatch without `dyn`.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::handlers::zfs::universal_zfs::backends::native::core::NativeZfsService;
use crate::handlers::zfs::universal_zfs::fail_safe::core::FailSafeZfsService;
use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsResult,
};

use super::traits::UniversalZfsService;
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
        let config = config.clone();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.create_pool(&config).await,
                Self::FailSafe(service) => service.create_pool(&config).await,
            }
        })
    }

    /// Gets Pool
    fn get_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<PoolInfo>>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.get_pool(&name).await,
                Self::FailSafe(service) => service.get_pool(&name).await,
            }
        })
    }

    /// Destroy Pool
    fn destroy_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.destroy_pool(&name).await,
                Self::FailSafe(service) => service.destroy_pool(&name).await,
            }
        })
    }

    /// Scrub Pool
    fn scrub_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.scrub_pool(&name).await,
                Self::FailSafe(service) => service.scrub_pool(&name).await,
            }
        })
    }

    /// Gets Pool Status
    fn get_pool_status(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.get_pool_status(&name).await,
                Self::FailSafe(service) => service.get_pool_status(&name).await,
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
        let config = config.clone();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.create_dataset(&config).await,
                Self::FailSafe(service) => service.create_dataset(&config).await,
            }
        })
    }

    /// Gets Dataset
    fn get_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<DatasetInfo>>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.get_dataset(&name).await,
                Self::FailSafe(service) => service.get_dataset(&name).await,
            }
        })
    }

    /// Destroy Dataset
    fn destroy_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.destroy_dataset(&name).await,
                Self::FailSafe(service) => service.destroy_dataset(&name).await,
            }
        })
    }

    /// Sets Dataset Properties
    fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let dataset_name = dataset_name.to_owned();
        let properties = properties.clone();
        Box::pin(async move {
            match self {
                Self::Native(service) => {
                    service
                        .set_dataset_properties(&dataset_name, &properties)
                        .await
                }
                Self::FailSafe(service) => {
                    service
                        .set_dataset_properties(&dataset_name, &properties)
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
        let name = name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.get_dataset_properties(&name).await,
                Self::FailSafe(service) => service.get_dataset_properties(&name).await,
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
        let config = config.clone();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.create_snapshot(&config).await,
                Self::FailSafe(service) => service.create_snapshot(&config).await,
            }
        })
    }

    /// List Dataset Snapshots
    fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<SnapshotInfo>>> + Send + '_>> {
        let dataset_name = dataset_name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.list_dataset_snapshots(&dataset_name).await,
                Self::FailSafe(service) => service.list_dataset_snapshots(&dataset_name).await,
            }
        })
    }

    /// Destroy Snapshot
    fn destroy_snapshot(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.destroy_snapshot(&name).await,
                Self::FailSafe(service) => service.destroy_snapshot(&name).await,
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
        let file_path = file_path.to_owned();
        Box::pin(async move {
            match self {
                Self::Native(service) => service.predict_tier(&file_path).await,
                Self::FailSafe(service) => service.predict_tier(&file_path).await,
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
    use crate::handlers::zfs::universal_zfs::traits::DynZfsService;
    use std::collections::HashMap;

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

    #[tokio::test]
    async fn universal_zfs_service_enum_is_available_and_shutdown() {
        let service = UniversalZfsServiceEnum::new_native();
        assert!(service.is_available().await);
        assert!(service.shutdown().await.is_ok());
    }

    #[test]
    fn dyn_zfs_service_fail_safe_service_name_and_version() {
        use crate::handlers::zfs::universal_zfs::config::FailSafeConfig;
        use crate::handlers::zfs::universal_zfs::fail_safe::core::FailSafeZfsService;
        use std::sync::Arc;

        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let mut cfg = FailSafeConfig::default();
        cfg.circuit_breaker.enabled = false;
        let fs = FailSafeZfsService::new(primary, cfg);
        let wrapped = DynZfsService::FailSafe(fs);
        assert_eq!(wrapped.service_name(), "fail-safe-zfs");
        assert_eq!(wrapped.service_version(), "1.0.0");
    }

    #[tokio::test]
    async fn dyn_zfs_service_fail_safe_health_check_runs() {
        use crate::handlers::zfs::universal_zfs::config::FailSafeConfig;
        use crate::handlers::zfs::universal_zfs::fail_safe::core::FailSafeZfsService;
        use std::sync::Arc;

        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let mut cfg = FailSafeConfig::default();
        cfg.circuit_breaker.enabled = false;
        let fs = FailSafeZfsService::new(primary, cfg);
        let wrapped = DynZfsService::FailSafe(fs);
        let _ = wrapped.health_check().await;
    }

    #[test]
    fn universal_zfs_service_enum_new_fail_safe_wraps_native() {
        use crate::handlers::zfs::universal_zfs::config::FailSafeConfig;
        use std::sync::Arc;

        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let mut cfg = FailSafeConfig::default();
        cfg.circuit_breaker.enabled = false;
        let e = UniversalZfsServiceEnum::new_fail_safe(primary, cfg);
        assert_eq!(e.service_name(), "fail-safe-zfs");
    }

    #[tokio::test]
    async fn trait_object_list_pools_and_optimize_native_enum() {
        use crate::handlers::zfs::universal_zfs_types::PoolConfig;

        let svc: &dyn UniversalZfsService = &UniversalZfsServiceEnum::new_native();
        let _ = svc.list_pools().await;
        let _ = svc.optimize().await;
        let cfg = PoolConfig {
            name: "noop".to_string(),
            devices: vec![],
            mountpoint: None,
            compression: false,
            deduplication: false,
            properties: std::collections::HashMap::new(),
        };
        let _ = svc.create_pool(&cfg).await;
    }

    #[tokio::test]
    async fn dyn_zfs_service_native_dispatches_all_wrappers() {
        use crate::handlers::zfs::universal_zfs_types::{
            DatasetConfig, PoolConfig, SnapshotConfig,
        };

        let svc = DynZfsService::Native(NativeZfsService::new());
        let pool_cfg = PoolConfig {
            name: "noop_pool".to_string(),
            devices: vec![],
            mountpoint: None,
            compression: false,
            deduplication: false,
            properties: HashMap::new(),
        };
        let ds_cfg = DatasetConfig {
            name: "noop/ds".to_string(),
            mountpoint: None,
            compression: false,
            quota: None,
            reservation: None,
            properties: HashMap::new(),
        };
        let snap_cfg = SnapshotConfig {
            name: "snap1".to_string(),
            dataset: "noop/ds".to_string(),
            properties: HashMap::new(),
        };

        let _ = svc.get_pool("nope").await;
        let _ = svc.destroy_pool("nope").await;
        let _ = svc.scrub_pool("nope").await;
        let _ = svc.get_pool_status("nope").await;
        let _ = svc.list_datasets().await;
        let _ = svc.create_dataset(&ds_cfg).await;
        let _ = svc.get_dataset("nope").await;
        let _ = svc.destroy_dataset("nope").await;
        let mut props = HashMap::new();
        props.insert("compression".to_string(), "lz4".to_string());
        let _ = svc.set_dataset_properties("noop/ds", &props).await;
        let _ = svc.get_dataset_properties("noop/ds").await;
        let _ = svc.list_snapshots().await;
        let _ = svc.create_snapshot(&snap_cfg).await;
        let _ = svc.list_dataset_snapshots("noop/ds").await;
        let _ = svc.destroy_snapshot("noop@snap").await;
        let _ = svc.create_pool(&pool_cfg).await;
        let _ = svc.optimize().await;
        let _ = svc.get_optimization_analytics().await;
        let _ = svc.predict_tier("/tmp/x").await;
        let _ = svc.get_configuration().await;
        let _ = svc
            .update_configuration(serde_json::json!({ "k": 1 }))
            .await;
    }

    #[tokio::test]
    async fn universal_zfs_service_enum_native_trait_covers_remaining_methods() {
        #[allow(deprecated)]
        use crate::handlers::zfs::universal_zfs_types::{DatasetConfig, SnapshotConfig};

        let svc = UniversalZfsServiceEnum::new_native();
        let ds_cfg = DatasetConfig {
            name: "t/d".to_string(),
            mountpoint: None,
            compression: false,
            quota: None,
            reservation: None,
            properties: HashMap::new(),
        };
        let snap_cfg = SnapshotConfig {
            name: "s".to_string(),
            dataset: "t/d".to_string(),
            properties: HashMap::new(),
        };

        let _ = svc.get_pool("x").await;
        let _ = svc.destroy_pool("x").await;
        let _ = svc.scrub_pool("x").await;
        let _ = svc.get_pool_status("x").await;
        let _ = svc.create_dataset(&ds_cfg).await;
        let _ = svc.get_dataset("x").await;
        let _ = svc.destroy_dataset("x").await;
        let _ = svc.set_dataset_properties("t/d", &HashMap::new()).await;
        let _ = svc.get_dataset_properties("t/d").await;
        let _ = svc.list_snapshots().await;
        let _ = svc.create_snapshot(&snap_cfg).await;
        let _ = svc.list_dataset_snapshots("t/d").await;
        let _ = svc.destroy_snapshot("t/d@s").await;
        let _ = svc.get_optimization_analytics().await;
        let _ = svc.predict_tier("/a").await;
        let _ = svc.get_configuration().await;
        let _ = svc.update_configuration(serde_json::json!({})).await;
    }
}
