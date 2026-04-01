// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Concrete enum dispatch for `UniversalZfsService` implementations.
//!
//! Wraps `NativeZfsService` and `FailSafeZfsService` in a single enum
//! so callers can use trait-object-free dispatch without `dyn`.

use std::collections::HashMap;
use std::sync::Arc;

use crate::handlers::zfs::universal_zfs::backends::native::core::NativeZfsService;
use crate::handlers::zfs::universal_zfs::fail_safe::core::{
    self as fail_safe_core, FailSafeZfsService,
};
use crate::handlers::zfs::universal_zfs::fail_safe::{
    dataset_operations as fail_safe_dataset, optimization as fail_safe_optimization,
    pool_operations as fail_safe_pool, snapshot_operations as fail_safe_snapshot,
};
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
    async fn health_check(&self) -> UniversalZfsResult<HealthStatus> {
        match self {
            Self::Native(service) => service.health_check().await,
            Self::FailSafe(service) => fail_safe_core::health_check(service).await,
        }
    }

    /// Gets Metrics
    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics> {
        match self {
            Self::Native(service) => service.get_metrics().await,
            Self::FailSafe(service) => service.get_metrics().await,
        }
    }

    /// Creates  Pool
    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        match self {
            Self::Native(service) => service.create_pool(config).await,
            Self::FailSafe(service) => fail_safe_pool::create_pool(service, config).await,
        }
    }

    /// Gets Pool
    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        match self {
            Self::Native(service) => service.get_pool(name).await,
            Self::FailSafe(service) => fail_safe_pool::get_pool(service, name).await,
        }
    }

    /// Destroy Pool
    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_pool(name).await,
            Self::FailSafe(service) => fail_safe_pool::destroy_pool(service, name).await,
        }
    }

    /// Scrub Pool
    async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.scrub_pool(name).await,
            Self::FailSafe(service) => fail_safe_pool::scrub_pool(service, name).await,
        }
    }

    /// Gets Pool Status
    async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.get_pool_status(name).await,
            Self::FailSafe(service) => fail_safe_pool::get_pool_status(service, name).await,
        }
    }

    /// List Datasets
    async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>> {
        match self {
            Self::Native(service) => service.list_datasets().await,
            Self::FailSafe(service) => fail_safe_dataset::list_datasets(service).await,
        }
    }

    /// Creates  Dataset
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        match self {
            Self::Native(service) => service.create_dataset(config).await,
            Self::FailSafe(service) => fail_safe_dataset::create_dataset(service, config).await,
        }
    }

    /// Gets Dataset
    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        match self {
            Self::Native(service) => service.get_dataset(name).await,
            Self::FailSafe(service) => fail_safe_dataset::get_dataset(service, name).await,
        }
    }

    /// Destroy Dataset
    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_dataset(name).await,
            Self::FailSafe(service) => fail_safe_dataset::destroy_dataset(service, name).await,
        }
    }

    /// Sets Dataset Properties
    async fn set_dataset_properties(
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
                fail_safe_dataset::set_dataset_properties(service, dataset_name, properties).await
            }
        }
    }

    /// Gets Dataset Properties
    async fn get_dataset_properties(
        &self,
        name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>> {
        match self {
            Self::Native(service) => service.get_dataset_properties(name).await,
            Self::FailSafe(service) => {
                fail_safe_dataset::get_dataset_properties(service, name).await
            }
        }
    }

    /// List Snapshots
    async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        match self {
            Self::Native(service) => service.list_snapshots().await,
            Self::FailSafe(service) => fail_safe_snapshot::list_snapshots(service).await,
        }
    }

    /// Creates  Snapshot
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo> {
        match self {
            Self::Native(service) => service.create_snapshot(config).await,
            Self::FailSafe(service) => fail_safe_snapshot::create_snapshot(service, config).await,
        }
    }

    /// List Dataset Snapshots
    async fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        match self {
            Self::Native(service) => service.list_dataset_snapshots(dataset_name).await,
            Self::FailSafe(service) => {
                fail_safe_snapshot::list_dataset_snapshots(service, dataset_name).await
            }
        }
    }

    /// Destroy Snapshot
    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.destroy_snapshot(name).await,
            Self::FailSafe(service) => fail_safe_snapshot::destroy_snapshot(service, name).await,
        }
    }

    /// Optimize
    async fn optimize(&self) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.optimize().await,
            Self::FailSafe(service) => fail_safe_optimization::optimize(service).await,
        }
    }

    /// Gets Optimization Analytics
    async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value> {
        match self {
            Self::Native(service) => service.get_optimization_analytics().await,
            Self::FailSafe(service) => {
                fail_safe_optimization::get_optimization_analytics(service).await
            }
        }
    }

    /// Predict Tier
    async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String> {
        match self {
            Self::Native(service) => service.predict_tier(file_path).await,
            Self::FailSafe(service) => {
                fail_safe_optimization::predict_tier(service, file_path).await
            }
        }
    }

    /// Gets Configuration
    async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value> {
        match self {
            Self::Native(service) => service.get_configuration().await,
            Self::FailSafe(service) => fail_safe_optimization::get_configuration(service).await,
        }
    }

    /// Updates  Configuration
    async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.update_configuration(config).await,
            Self::FailSafe(service) => {
                fail_safe_optimization::update_configuration(service, config).await
            }
        }
    }

    /// Checks if Available
    async fn is_available(&self) -> bool {
        match self {
            Self::Native(service) => service.is_available().await,
            Self::FailSafe(service) => fail_safe_core::is_available_dispatch(service).await,
        }
    }

    /// Shutdown
    async fn shutdown(&self) -> UniversalZfsResult<()> {
        match self {
            Self::Native(service) => service.shutdown().await,
            Self::FailSafe(service) => fail_safe_optimization::shutdown(service).await,
        }
    }

    /// List Pools
    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        match self {
            Self::Native(service) => service.list_pools().await,
            Self::FailSafe(service) => fail_safe_pool::list_pools(service).await,
        }
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
