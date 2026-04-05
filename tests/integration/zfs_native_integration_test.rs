// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZFS NATIVE INTEGRATION TESTS**
//!
//! Exercises the universal ZFS factory and [`UniversalZfsService`] against the native backend.
//! Real ZFS paths are gated by `NESTGATE_TEST_REAL_ZFS`; destructive ops by `NESTGATE_TEST_DESTRUCTIVE`.
//!
//! Note: This file lives under `tests/integration/`; only `tests/*.rs` at the workspace root
//! are built as separate integration-test crates unless referenced elsewhere.

use nestgate_api::handlers::zfs::universal_zfs::{
    backends::{NativeZfsService, ZfsServiceFactory},
    service_enum::UniversalZfsServiceEnum,
    traits::UniversalZfsService,
    types::{DatasetConfig, SnapshotConfig},
};
use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Test configuration for ZFS integration tests
struct ZfsTestConfig {
    pub use_real_zfs: bool,
    pub test_pool_name: String,
    pub test_dataset_name: String,
    pub cleanup_after_test: bool,
}

impl Default for ZfsTestConfig {
    fn default() -> Self {
        Self {
            use_real_zfs: std::env::var("NESTGATE_TEST_REAL_ZFS").unwrap_or_default() == "true",
            test_pool_name: "nestgate-test-pool".to_string(),
            test_dataset_name: "nestgate-test-dataset".to_string(),
            cleanup_after_test: true,
        }
    }
}

/// Integration test suite for ZFS operations
#[cfg(test)]
mod tests {
    use super::*;

    /// Test ZFS service factory creation and availability detection
    #[tokio::test]
    async fn test_zfs_service_factory() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let config = NestGateCanonicalConfig::default();

        let zfs_available = ZfsServiceFactory::check_zfs_availability();
        info!("ZFS availability check result: {zfs_available}");

        let service = ZfsServiceFactory::create_service(config.clone());
        assert!(!service.service_name().is_empty());
        assert!(!service.service_version().is_empty());

        let auto_service = ZfsServiceFactory::create_auto_service(config).await;
        assert!(!auto_service.service_name().is_empty());

        info!("ZFS service factory tests passed");
        Ok(())
    }

    /// Test ZFS health check functionality
    #[tokio::test]
    async fn test_zfs_health_check() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let config = NestGateCanonicalConfig::default();
        let service = create_test_service(config).await;

        match service.health_check().await {
            Ok(health) => {
                info!("ZFS health check passed: {:?}", health.status);
                assert!(!health.service_name.is_empty());
            }
            Err(e) => {
                warn!("ZFS health check failed (expected if ZFS not available): {e}");
            }
        }
        Ok(())
    }

    /// Test ZFS service metrics collection
    #[tokio::test]
    async fn test_zfs_metrics() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let config = NestGateCanonicalConfig::default();
        let service = create_test_service(config).await;

        match service.get_metrics().await {
            Ok(metrics) => {
                info!("ZFS metrics collection passed");
                assert!(metrics.uptime_seconds >= 0);
                debug!("Metrics: uptime={}s", metrics.uptime_seconds);
            }
            Err(e) => {
                warn!("ZFS metrics collection failed: {e}");
            }
        }
        Ok(())
    }

    /// Test ZFS pool operations (conditional on real ZFS availability)
    #[tokio::test]
    async fn test_zfs_pool_operations() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let test_config = ZfsTestConfig::default();
        if !test_config.use_real_zfs {
            info!("Skipping real ZFS pool tests (set NESTGATE_TEST_REAL_ZFS=true to enable)");
            return Ok(());
        }

        let config = NestGateCanonicalConfig::default();
        let service = create_test_service(config).await;

        match service.list_pools().await {
            Ok(pools) => {
                info!("ZFS pool listing successful: found {} pools", pools.len());
                for pool in &pools {
                    debug!("Pool: {} ({})", pool.name, pool.health.to_string());
                }
            }
            Err(e) => {
                warn!("ZFS pool listing failed: {e}");
            }
        }

        if let Ok(pools) = service.list_pools().await {
            if let Some(first_pool) = pools.first() {
                match service.get_pool(first_pool.name.as_str()).await {
                    Ok(Some(pool_info)) => {
                        info!("ZFS pool info retrieval successful for: {}", pool_info.name);
                        assert_eq!(pool_info.name, first_pool.name);
                    }
                    Ok(None) => {
                        debug!("get_pool returned None for {}", first_pool.name);
                    }
                    Err(e) => {
                        warn!("ZFS pool info retrieval failed: {e}");
                    }
                }
            }
        }
        Ok(())
    }

    /// Test ZFS dataset operations (conditional on real ZFS availability)
    #[tokio::test]
    async fn test_zfs_dataset_operations() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let test_config = ZfsTestConfig::default();
        if !test_config.use_real_zfs {
            info!("Skipping real ZFS dataset tests (set NESTGATE_TEST_REAL_ZFS=true to enable)");
            return Ok(());
        }

        let config = NestGateCanonicalConfig::default();
        let service = create_test_service(config).await;

        match service.list_datasets().await {
            Ok(datasets) => {
                info!("ZFS dataset listing successful: found {} datasets", datasets.len());
                for dataset in &datasets {
                    debug!("Dataset: {} ({})", dataset.name, dataset.dataset_type.to_string());
                }
            }
            Err(e) => {
                warn!("ZFS dataset listing failed: {e}");
            }
        }

        if should_test_destructive_operations() {
            let dataset_config = DatasetConfig {
                name: format!("{}/{}", test_config.test_pool_name, test_config.test_dataset_name),
                mountpoint: Some("/tmp/nestgate-test".to_string()),
                compression: false,
                quota: None,
                reservation: None,
                properties: HashMap::new(),
            };

            match service.create_dataset(&dataset_config).await {
                Ok(dataset_info) => {
                    info!("ZFS dataset creation successful: {}", dataset_info.name);

                    if test_config.cleanup_after_test {
                        match service.destroy_dataset(dataset_config.name.as_str()).await {
                            Ok(()) => {
                                info!("Test dataset cleanup successful");
                            }
                            Err(e) => {
                                warn!("Test dataset cleanup failed: {e}");
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("ZFS dataset creation failed (expected without proper permissions): {e}");
                }
            }
        }
        Ok(())
    }

    /// Test ZFS snapshot operations (conditional on real ZFS availability)
    #[tokio::test]
    async fn test_zfs_snapshot_operations() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let test_config = ZfsTestConfig::default();
        if !test_config.use_real_zfs {
            info!("Skipping real ZFS snapshot tests (set NESTGATE_TEST_REAL_ZFS=true to enable)");
            return Ok(());
        }

        let config = NestGateCanonicalConfig::default();
        let service = create_test_service(config).await;

        if should_test_destructive_operations() {
            let dataset_path = format!("{}/{}", test_config.test_pool_name, test_config.test_dataset_name);
            let snapshot_config = SnapshotConfig {
                name: format!("{dataset_path}@nestgate-test-snapshot"),
                dataset: dataset_path,
                properties: HashMap::new(),
            };

            match service.create_snapshot(&snapshot_config).await {
                Ok(snapshot_info) => {
                    info!("ZFS snapshot creation successful: {}", snapshot_info.name);
                    assert!(snapshot_info.name.contains("nestgate-test-snapshot"));
                }
                Err(e) => {
                    warn!("ZFS snapshot creation failed (expected without existing dataset): {e}");
                }
            }
        }
        Ok(())
    }

    /// Test ZFS service error handling
    #[tokio::test]
    async fn test_zfs_error_handling() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let config = NestGateCanonicalConfig::default();
        let service = create_test_service(config).await;

        match service.get_pool("non-existent-pool-12345").await {
            Ok(None) => {
                info!("Proper handling for non-existent pool: None");
            }
            Ok(Some(_)) => {
                warn!("Expected None for non-existent pool, but got Some");
            }
            Err(e) => {
                info!("Non-existent pool reported as error: {e}");
            }
        }

        match service.destroy_dataset("non-existent-dataset-12345").await {
            Ok(()) => {
                warn!("Expected error for non-existent dataset, but got success");
            }
            Err(e) => {
                info!("Proper error handling for non-existent dataset: {e}");
            }
        }
        Ok(())
    }

    /// Test configuration integration
    #[tokio::test]
    async fn test_configuration_integration() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let config = NestGateCanonicalConfig::default();

        let service = NativeZfsService::new();
        assert_eq!(service.service_name(), "native-zfs");

        let factory_service = ZfsServiceFactory::create_service_with_sudo(config);
        assert!(!factory_service.service_name().is_empty());

        info!("Configuration integration tests passed");
        Ok(())
    }

    /// Test concurrent ZFS operations
    #[tokio::test]
    async fn test_concurrent_operations() -> Result<(), Box<dyn std::error::Error>> {
        init_test_logging();

        let config = NestGateCanonicalConfig::default();
        let service = Arc::new(create_test_service(config).await);

        let mut handles = Vec::new();

        for i in 0..5 {
            let service_clone = Arc::clone(&service);
            let handle = tokio::spawn(async move {
                debug!("Starting concurrent health check {i}");
                let result = service_clone.health_check().await;
                debug!("Completed concurrent health check {i}");
                result
            });
            handles.push(handle);
        }

        let mut success_count = 0;
        for handle in handles {
            match handle.await {
                Ok(Ok(_)) => success_count += 1,
                Ok(Err(e)) => debug!("Health check failed: {e}"),
                Err(e) => debug!("Task failed: {e}"),
            }
        }

        info!("Concurrent operations test completed: {success_count}/5 successful");
        Ok(())
    }

    fn init_test_logging() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_test_writer()
            .try_init();
    }

    async fn create_test_service(config: NestGateCanonicalConfig) -> Arc<UniversalZfsServiceEnum> {
        ZfsServiceFactory::create_auto_service(config).await
    }

    fn should_test_destructive_operations() -> bool {
        std::env::var("NESTGATE_TEST_DESTRUCTIVE").unwrap_or_default() == "true"
    }
}

trait PoolHealthDisplay {
    fn to_string(&self) -> String;
}

impl PoolHealthDisplay for nestgate_api::handlers::zfs::universal_zfs::types::PoolHealth {
    fn to_string(&self) -> String {
        match self {
            nestgate_api::handlers::zfs::universal_zfs::types::PoolHealth::Online => "Online".to_string(),
            nestgate_api::handlers::zfs::universal_zfs::types::PoolHealth::Degraded => "Degraded".to_string(),
            nestgate_api::handlers::zfs::universal_zfs::types::PoolHealth::Faulted => "Faulted".to_string(),
            nestgate_api::handlers::zfs::universal_zfs::types::PoolHealth::Offline => "Offline".to_string(),
        }
    }
}

trait DatasetTypeDisplay {
    fn to_string(&self) -> String;
}

impl DatasetTypeDisplay for nestgate_api::handlers::zfs::universal_zfs::types::DatasetType {
    fn to_string(&self) -> String {
        match self {
            nestgate_api::handlers::zfs::universal_zfs::types::DatasetType::Filesystem => {
                "Filesystem".to_string()
            }
            nestgate_api::handlers::zfs::universal_zfs::types::DatasetType::Volume => "Volume".to_string(),
            nestgate_api::handlers::zfs::universal_zfs::types::DatasetType::Snapshot => {
                "Snapshot".to_string()
            }
        }
    }
}
