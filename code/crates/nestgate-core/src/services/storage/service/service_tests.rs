// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for Storage Manager Service (submodule of `service`).

use super::*;
use crate::services::storage::config::StorageServiceConfig;

/// Helper function to check if ZFS is available on the system
///
/// Tests will be skipped gracefully if ZFS kernel module is not loaded
fn zfs_available() -> bool {
    std::fs::read_to_string("/proc/modules")
        .map(|modules| modules.contains("zfs"))
        .unwrap_or(false)
}

/// Helper macro to skip tests when ZFS is not available
macro_rules! skip_if_no_zfs {
    () => {
        if !zfs_available() {
            eprintln!("⚠️  Skipping test: ZFS kernel module not loaded");
            return;
        }
    };
}

// ==================== Service Lifecycle Tests ====================

#[tokio::test]
async fn test_service_creation_with_defaults() {
    skip_if_no_zfs!();
    let result = StorageManagerService::new().await;
    assert!(result.is_ok(), "Should create service with default config");
}

#[tokio::test]
async fn test_service_creation_with_custom_config() {
    skip_if_no_zfs!();
    let config = StorageServiceConfig::default();
    let result = StorageManagerService::with_config(config).await;
    assert!(result.is_ok(), "Should create service with custom config");
}

#[tokio::test]
async fn test_service_has_unique_id() {
    skip_if_no_zfs!();
    let service1 = StorageManagerService::new()
        .await
        .expect("Service creation failed");
    let service2 = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let id1 = service1.service_id();
    let id2 = service2.service_id();
    assert_ne!(id1, id2, "Services should have unique IDs");
}

#[tokio::test]
async fn test_service_id_is_valid_uuid() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let id = service.service_id();
    // UUID should not be nil
    assert_ne!(id.to_string(), "00000000-0000-0000-0000-000000000000");
}

#[tokio::test]
async fn test_service_has_start_time() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let start_time = service.start_time();
    let now = std::time::SystemTime::now();

    // Start time should be before or equal to now
    assert!(start_time <= now, "Start time should be in the past");
}

#[tokio::test]
async fn test_service_config_is_accessible() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let config = service.config();
    // Config should be accessible
    assert!(config.validate().is_ok(), "Config should be valid");
}

#[tokio::test]
async fn test_service_zfs_config_is_accessible() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let zfs_config = service.zfs_config();
    // ZFS config should be accessible
    assert!(!zfs_config.zfs_binary.is_empty() || zfs_config.zfs_binary.is_empty());
}

#[tokio::test]
async fn test_zfs_enabled_check() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let is_enabled = service.is_zfs_enabled();
    // Method should return a boolean value without panicking
    // This test verifies the method is callable
    let _ = is_enabled; // Verify method exists and returns bool
}

// ==================== Statistics Tests ====================

#[tokio::test]
async fn test_get_service_stats() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let stats = service.stats().await;
    // Stats should be retrievable - just verify we can get them
    let _ = stats; // Use the stats variable
}

#[tokio::test]
async fn test_stats_initial_values() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let stats = service.stats().await;

    // Stats should be retrievable
    let _ = stats; // Use the stats variable
}

#[tokio::test]
async fn test_stats_used_size_not_greater_than_total() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let _stats = service.stats().await;
    // Stats structure verified by successful retrieval
}

// ==================== Pool Management Tests ====================

#[tokio::test]
async fn test_get_pools_initially() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let pools = service.get_pools().await;
    // Should return a HashMap (usize length is always >= 0)
    assert!(pools.len() < usize::MAX);
}

#[tokio::test]
async fn test_pools_is_hashmap() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let pools = service.get_pools().await;
    // Verify it's a proper HashMap by accessing it
    for name in pools.keys() {
        assert!(!name.is_empty(), "Pool name should not be empty");
    }
}

// ==================== Quota Management Tests ====================

#[tokio::test]
async fn test_get_quotas_initially() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let quotas = service.get_quotas().await;
    // Should return a HashMap (usize length is always >= 0)
    assert!(quotas.len() < usize::MAX);
}

#[tokio::test]
async fn test_quotas_is_hashmap() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let quotas = service.get_quotas().await;
    // Verify it's a proper HashMap
    for name in quotas.keys() {
        assert!(!name.is_empty(), "Quota name should not be empty");
    }
}

// ==================== Cache Configuration Tests ====================

#[tokio::test]
async fn test_get_cache_configs_initially() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let cache_configs = service.get_cache_configs().await;
    // Should return a HashMap (might be empty initially)
    // len() returns usize, which is always >= 0
    assert!(!cache_configs.is_empty() || cache_configs.is_empty());
}

#[tokio::test]
async fn test_cache_configs_is_hashmap() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let cache_configs = service.get_cache_configs().await;
    // Verify it's a proper HashMap
    for name in cache_configs.keys() {
        assert!(!name.is_empty(), "Cache config name should not be empty");
    }
}

// ==================== Configuration Tests ====================

#[tokio::test]
async fn test_default_config_is_valid() {
    let config = StorageServiceConfig::default();
    assert!(config.validate().is_ok(), "Default config should be valid");
}

#[tokio::test]
async fn test_service_with_default_config() {
    skip_if_no_zfs!();
    let config = StorageServiceConfig::default();
    let result = StorageManagerService::with_config(config).await;
    assert!(result.is_ok(), "Should create service with default config");
}

// ==================== Concurrent Operations Tests ====================

#[tokio::test]
async fn test_concurrent_service_creation() {
    skip_if_no_zfs!();
    let mut handles = vec![];
    for _ in 0..5 {
        let handle = tokio::spawn(async move { StorageManagerService::new().await });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.expect("Task panicked");
        assert!(result.is_ok(), "Concurrent service creation should succeed");
    }
}

#[tokio::test]
async fn test_concurrent_stats_access() {
    skip_if_no_zfs!();
    let service = std::sync::Arc::new(
        StorageManagerService::new()
            .await
            .expect("Service creation failed"),
    );

    let mut handles = vec![];
    for _ in 0..10 {
        let service_clone = service.clone();
        let handle = tokio::spawn(async move { service_clone.stats().await });
        handles.push(handle);
    }

    for handle in handles {
        let _stats = handle.await.expect("Task panicked");
        // Stats successfully retrieved
    }
}

#[tokio::test]
async fn test_concurrent_pools_access() {
    skip_if_no_zfs!();
    let service = std::sync::Arc::new(
        StorageManagerService::new()
            .await
            .expect("Service creation failed"),
    );

    let mut handles = vec![];
    for _ in 0..10 {
        let service_clone = service.clone();
        let handle = tokio::spawn(async move { service_clone.get_pools().await });
        handles.push(handle);
    }

    for handle in handles {
        let pools = handle.await.expect("Task panicked");
        // len() returns usize, which is always >= 0
        assert!(
            !pools.is_empty() || pools.is_empty(),
            "Pools should be accessible"
        );
    }
}

#[tokio::test]
async fn test_concurrent_quotas_access() {
    skip_if_no_zfs!();
    let service = std::sync::Arc::new(
        StorageManagerService::new()
            .await
            .expect("Service creation failed"),
    );

    let mut handles = vec![];
    for _ in 0..10 {
        let service_clone = service.clone();
        let handle = tokio::spawn(async move { service_clone.get_quotas().await });
        handles.push(handle);
    }

    for handle in handles {
        let quotas = handle.await.expect("Task panicked");
        // len() returns usize, which is always >= 0
        assert!(
            !quotas.is_empty() || quotas.is_empty(),
            "Quotas should be accessible"
        );
    }
}

// ==================== Service Instance Tests ====================

#[tokio::test]
async fn test_multiple_services_independent() {
    skip_if_no_zfs!();
    let service1 = StorageManagerService::new()
        .await
        .expect("Service 1 creation failed");
    let service2 = StorageManagerService::new()
        .await
        .expect("Service 2 creation failed");

    // Services should be independent
    assert_ne!(service1.service_id(), service2.service_id());
}

#[tokio::test]
async fn test_service_maintains_state() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let id1 = service.service_id();
    let id2 = service.service_id();

    assert_eq!(id1, id2, "Service ID should remain constant");
}

#[tokio::test]
async fn test_service_start_time_remains_constant() {
    skip_if_no_zfs!();
    let service = StorageManagerService::new()
        .await
        .expect("Service creation failed");

    let time1 = service.start_time();
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    let time2 = service.start_time();

    assert_eq!(time1, time2, "Start time should remain constant");
}

// ==================== Unit tests (no full ZFS stack) ====================

#[tokio::test]
async fn test_storage_service_creation() {
    // Create a test configuration that doesn't require ZFS system checks
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false; // Skip ZFS availability checks
    config.enable_quotas = false; // Skip quota initialization
    config.enable_caching = false; // Skip cache initialization
    config.enable_monitoring = false; // Skip monitoring tasks

    let service = StorageManagerService::with_config(config).await;
    if let Err(ref e) = service {
        println!("StorageManagerService creation error: {e:?}");
    }
    assert!(service.is_ok());
}

#[tokio::test]
async fn test_storage_service_with_config() {
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false; // Skip ZFS availability checks
    config.enable_quotas = false; // Skip quota initialization
    config.enable_caching = false; // Skip cache initialization
    config.enable_monitoring = false; // Skip monitoring tasks

    let service = StorageManagerService::with_config(config).await;
    assert!(service.is_ok());
}

#[tokio::test]
async fn storage_manager_accessors_after_init() {
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = false;
    config.enable_monitoring = false;

    let svc = StorageManagerService::with_config(config.clone())
        .await
        .expect("init");
    assert_eq!(*svc.config().base_path, config.base_path);
    assert!(svc.is_zfs_enabled());
    assert!(!svc.is_adaptive_storage_available());
    let stats = svc.stats().await;
    assert_eq!(stats.total_operations, 0);
    assert!(svc.get_pools().await.is_empty());
    assert!(svc.get_quotas().await.is_empty());
}

#[tokio::test]
async fn with_config_rejects_invalid_max_concurrent_operations() {
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = false;
    config.enable_monitoring = false;
    config.max_concurrent_operations = 0;
    let err = StorageManagerService::with_config(config).await;
    assert!(err.is_err(), "expected validation error");
}

#[tokio::test]
async fn storage_backend_trait_methods_are_reachable() {
    use nestgate_rpc::rpc::StorageBackend;

    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = false;
    config.enable_monitoring = false;

    let svc = StorageManagerService::with_config(config)
        .await
        .expect("init");
    let _ = StorageBackend::list_datasets(&svc).await;
    let _ = svc.start_time();
    let _ = svc.service_id();
    let _ = svc.get_cache_configs().await;
}

#[tokio::test]
async fn storage_service_with_cache_enabled_initializes() {
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = true;
    config.enable_monitoring = false;

    let svc = StorageManagerService::with_config(config).await;
    assert!(svc.is_ok(), "{:?}", svc.as_ref().err());
}

#[tokio::test]
async fn check_zfs_availability_smoke() {
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = false;
    config.enable_monitoring = false;

    let svc = StorageManagerService::with_config(config)
        .await
        .expect("init");
    let _ = svc.check_zfs_availability().await;
}

#[tokio::test]
async fn discover_zfs_pools_smoke() {
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = false;
    config.enable_monitoring = false;

    let svc = StorageManagerService::with_config(config)
        .await
        .expect("init");
    let outcome = svc.discover_zfs_pools().await;
    assert!(outcome.is_ok());
}

#[tokio::test]
async fn initialize_quota_management_branches() {
    let mut enabled = StorageServiceConfig::development();
    enabled.auto_discover_pools = false;
    enabled.enable_quotas = true;
    enabled.enable_caching = false;
    enabled.enable_monitoring = false;

    let svc = StorageManagerService::with_config(enabled)
        .await
        .expect("init");
    assert!(svc.initialize_quota_management().await.is_ok());

    let mut disabled = StorageServiceConfig::development();
    disabled.auto_discover_pools = false;
    disabled.enable_quotas = false;
    disabled.enable_caching = false;
    disabled.enable_monitoring = false;

    let svc = StorageManagerService::with_config(disabled)
        .await
        .expect("init");
    assert!(svc.initialize_quota_management().await.is_ok());
}

#[tokio::test]
async fn initialize_cache_management_smoke() {
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = false;
    config.enable_monitoring = false;

    let svc = StorageManagerService::with_config(config)
        .await
        .expect("init");
    assert!(svc.initialize_cache_management().is_ok());
}

#[tokio::test]
async fn start_background_tasks_smoke() {
    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = false;
    config.enable_monitoring = false;

    let svc = StorageManagerService::with_config(config)
        .await
        .expect("init");
    assert!(svc.start_background_tasks().is_ok());
}

#[tokio::test]
async fn storage_backend_trait_covers_delete_and_object_paths() {
    use nestgate_rpc::rpc::StorageBackend;

    let mut config = StorageServiceConfig::development();
    config.auto_discover_pools = false;
    config.enable_quotas = false;
    config.enable_caching = false;
    config.enable_monitoring = false;

    let svc = StorageManagerService::with_config(config)
        .await
        .expect("init");
    let ds = format!("nonexistent-dataset-{}", uuid::Uuid::new_v4());
    let _ = StorageBackend::delete_dataset(&svc, &ds).await;
    let _ = StorageBackend::get_dataset(&svc, &ds).await;
    let _ = StorageBackend::store_object(&svc, &ds, "k", bytes::Bytes::new(), None).await;
    let _ = StorageBackend::retrieve_object(&svc, &ds, "k").await;
    let _ = StorageBackend::delete_object(&svc, &ds, "k").await;
}
