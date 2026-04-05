// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for Zero-Cost ZFS Manager
//!
//! This test suite covers:
//! - Manager initialization and configuration
//! - Type aliases and const generics
//! - Data structures (PoolInfo, DatasetInfo, SnapshotInfo)
//! - Capacity limits and bounds checking
//! - Timeout functionality
//! - Property parsing
//! - Serialization/deserialization
//! - Concurrent access patterns

use super::manager::{
    DevelopmentZfsManager, EnterpriseZfsManager, HighPerformanceZfsManager, ProductionZfsManager,
    TestingZfsManager, ZeroCostZfsManager,
};
use super::types::{ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo};
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

// ==================== HELPER FUNCTIONS ====================

/// Creates  Test Pool Info
fn create_test_pool_info(name: &str) -> ZeroCostPoolInfo {
    let mut properties = HashMap::new();
    properties.insert("health".to_string(), "ONLINE".to_string());
    properties.insert("size".to_string(), "1000000".to_string());

    ZeroCostPoolInfo {
        name: name.to_string(),
        size: 1_000_000,
        used: 250_000,
        available: 750_000,
        health: "ONLINE".to_string(),
        properties,
        created_at: SystemTime::now(),
    }
}

/// Creates  Test Dataset Info
fn create_test_dataset_info(name: &str, pool: &str, tier: StorageTier) -> ZeroCostDatasetInfo {
    ZeroCostDatasetInfo {
        name: name.to_string(),
        pool: pool.to_string(),
        tier,
        size: 500_000,
        used: 100_000,
        properties: HashMap::new(),
        mount_point: Some(PathBuf::from(format!("/mnt/{}", name))),
        created_at: SystemTime::now(),
    }
}

/// Creates  Test Snapshot Info
fn create_test_snapshot_info(name: &str, dataset: &str) -> ZeroCostSnapshotInfo {
    ZeroCostSnapshotInfo {
        name: name.to_string(),
        dataset: dataset.to_string(),
        size: 10_000,
        created_at: SystemTime::now(),
        properties: HashMap::new(),
    }
}

// ==================== MANAGER CREATION TESTS ====================

#[test]
fn test_zero_cost_manager_new() {
    let _manager: TestingZfsManager = ZeroCostZfsManager::new();
    // Manager should be created with default values
    assert!(TestingZfsManager::command_timeout().as_millis() > 0);
}

#[test]
fn test_zero_cost_manager_default() {
    let _manager: TestingZfsManager = ZeroCostZfsManager::default();
    // Default should be equivalent to new()
    assert_eq!(TestingZfsManager::command_timeout().as_millis(), 5000);
}

// ==================== TYPE ALIAS TESTS ====================

#[test]
fn test_development_manager_timeout() {
    let timeout = DevelopmentZfsManager::command_timeout();
    assert_eq!(timeout, Duration::from_millis(10_000));
}

#[test]
fn test_production_manager_timeout() {
    let timeout = ProductionZfsManager::command_timeout();
    assert_eq!(timeout, Duration::from_millis(30_000));
}

#[test]
fn test_high_performance_manager_timeout() {
    let timeout = HighPerformanceZfsManager::command_timeout();
    assert_eq!(timeout, Duration::from_millis(45_000));
}

#[test]
fn test_testing_manager_timeout() {
    let timeout = TestingZfsManager::command_timeout();
    assert_eq!(timeout, Duration::from_millis(5_000));
}

#[test]
fn test_enterprise_manager_timeout() {
    let timeout = EnterpriseZfsManager::command_timeout();
    assert_eq!(timeout, Duration::from_millis(60_000));
}

// ==================== DATA STRUCTURE TESTS ====================

#[test]
fn test_zero_cost_pool_info_creation() {
    let pool = create_test_pool_info("test_pool");

    assert_eq!(pool.name, "test_pool");
    assert_eq!(pool.size, 1_000_000);
    assert_eq!(pool.used, 250_000);
    assert_eq!(pool.available, 750_000);
    assert_eq!(pool.health, "ONLINE");
    assert!(!pool.properties.is_empty());
}

#[test]
fn test_zero_cost_dataset_info_creation() {
    let dataset = create_test_dataset_info("test_dataset", "test_pool", StorageTier::Hot);

    assert_eq!(dataset.name, "test_dataset");
    assert_eq!(dataset.pool, "test_pool");
    assert_eq!(dataset.tier, StorageTier::Hot);
    assert_eq!(dataset.size, 500_000);
    assert_eq!(dataset.used, 100_000);
    assert!(dataset.mount_point.is_some());
}

#[test]
fn test_zero_cost_snapshot_info_creation() {
    let snapshot = create_test_snapshot_info("snap1", "test_pool/test_dataset");

    assert_eq!(snapshot.name, "snap1");
    assert_eq!(snapshot.dataset, "test_pool/test_dataset");
    assert_eq!(snapshot.size, 10_000);
}

#[test]
fn test_pool_info_clone() {
    let pool1 = create_test_pool_info("pool1");
    let pool2 = pool1.clone();

    assert_eq!(pool1.name, pool2.name);
    assert_eq!(pool1.size, pool2.size);
    assert_eq!(pool1.used, pool2.used);
}

#[test]
fn test_dataset_info_clone() {
    let dataset1 = create_test_dataset_info("ds1", "pool1", StorageTier::Warm);
    let dataset2 = dataset1.clone();

    assert_eq!(dataset1.name, dataset2.name);
    assert_eq!(dataset1.pool, dataset2.pool);
    assert_eq!(dataset1.tier, dataset2.tier);
}

#[test]
fn test_snapshot_info_clone() {
    let snapshot1 = create_test_snapshot_info("snap1", "pool1/ds1");
    let snapshot2 = snapshot1.clone();

    assert_eq!(snapshot1.name, snapshot2.name);
    assert_eq!(snapshot1.dataset, snapshot2.dataset);
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_pool_info_serialization() {
    let pool = create_test_pool_info("serialize_pool");
    let serialized = serde_json::to_string(&pool).expect("Failed to serialize");
    assert!(serialized.contains("serialize_pool"));
    assert!(serialized.contains("ONLINE"));
}

#[test]
fn test_pool_info_deserialization() {
    let pool = create_test_pool_info("deserialize_pool");
    let serialized = serde_json::to_string(&pool).expect("Failed to serialize");
    let deserialized: ZeroCostPoolInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(pool.name, deserialized.name);
    assert_eq!(pool.size, deserialized.size);
    assert_eq!(pool.health, deserialized.health);
}

#[test]
fn test_dataset_info_serialization() {
    let dataset = create_test_dataset_info("serialize_ds", "pool1", StorageTier::Cold);
    let serialized = serde_json::to_string(&dataset).expect("Failed to serialize");
    assert!(serialized.contains("serialize_ds"));
    assert!(serialized.contains("pool1"));
}

#[test]
fn test_dataset_info_deserialization() {
    let dataset = create_test_dataset_info("deserialize_ds", "pool1", StorageTier::Cache);
    let serialized = serde_json::to_string(&dataset).expect("Failed to serialize");
    let deserialized: ZeroCostDatasetInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(dataset.name, deserialized.name);
    assert_eq!(dataset.pool, deserialized.pool);
    assert_eq!(dataset.tier, deserialized.tier);
}

#[test]
fn test_snapshot_info_serialization() {
    let snapshot = create_test_snapshot_info("serialize_snap", "pool1/ds1");
    let serialized = serde_json::to_string(&snapshot).expect("Failed to serialize");
    assert!(serialized.contains("serialize_snap"));
    assert!(serialized.contains("pool1/ds1"));
}

#[test]
fn test_snapshot_info_deserialization() {
    let snapshot = create_test_snapshot_info("deserialize_snap", "pool1/ds1");
    let serialized = serde_json::to_string(&snapshot).expect("Failed to serialize");
    let deserialized: ZeroCostSnapshotInfo =
        serde_json::from_str(&serialized).expect("Failed to deserialize");

    assert_eq!(snapshot.name, deserialized.name);
    assert_eq!(snapshot.dataset, deserialized.dataset);
}

// ==================== STORAGE TIER TESTS ====================

#[test]
fn test_dataset_with_hot_tier() {
    let dataset = create_test_dataset_info("hot_dataset", "pool1", StorageTier::Hot);
    assert_eq!(dataset.tier, StorageTier::Hot);
}

#[test]
fn test_dataset_with_warm_tier() {
    let dataset = create_test_dataset_info("warm_dataset", "pool1", StorageTier::Warm);
    assert_eq!(dataset.tier, StorageTier::Warm);
}

#[test]
fn test_dataset_with_cold_tier() {
    let dataset = create_test_dataset_info("cold_dataset", "pool1", StorageTier::Cold);
    assert_eq!(dataset.tier, StorageTier::Cold);
}

#[test]
fn test_dataset_with_cache_tier() {
    let dataset = create_test_dataset_info("cache_dataset", "pool1", StorageTier::Cache);
    assert_eq!(dataset.tier, StorageTier::Cache);
}

#[test]
fn test_dataset_with_archive_tier() {
    let dataset = create_test_dataset_info("archive_dataset", "pool1", StorageTier::Archive);
    assert_eq!(dataset.tier, StorageTier::Archive);
}

// ==================== MOUNT POINT TESTS ====================

#[test]
fn test_dataset_with_mount_point() {
    let dataset = create_test_dataset_info("mounted_ds", "pool1", StorageTier::Hot);
    assert!(dataset.mount_point.is_some());
    assert_eq!(
        dataset.mount_point.unwrap(),
        PathBuf::from("/mnt/mounted_ds")
    );
}

#[test]
fn test_dataset_without_mount_point() {
    let mut dataset = create_test_dataset_info("unmounted_ds", "pool1", StorageTier::Hot);
    dataset.mount_point = None;
    assert!(dataset.mount_point.is_none());
}

// ==================== PROPERTY MAP TESTS ====================

#[test]
fn test_pool_properties_map() {
    let pool = create_test_pool_info("prop_pool");
    assert!(pool.properties.contains_key("health"));
    assert_eq!(pool.properties.get("health"), Some(&"ONLINE".to_string()));
}

#[test]
fn test_pool_properties_mutation() {
    let mut pool = create_test_pool_info("mut_pool");
    pool.properties
        .insert("custom_prop".to_string(), "custom_value".to_string());
    assert_eq!(
        pool.properties.get("custom_prop"),
        Some(&"custom_value".to_string())
    );
}

#[test]
fn test_dataset_properties_empty() {
    let dataset = create_test_dataset_info("empty_prop_ds", "pool1", StorageTier::Warm);
    assert!(dataset.properties.is_empty());
}

#[test]
fn test_snapshot_properties_empty() {
    let snapshot = create_test_snapshot_info("empty_prop_snap", "pool1/ds1");
    assert!(snapshot.properties.is_empty());
}

// ==================== DEBUG TRAIT TESTS ====================

#[test]
fn test_pool_info_debug() {
    let pool = create_test_pool_info("debug_pool");
    let debug_str = format!("{:?}", pool);
    assert!(debug_str.contains("debug_pool"));
    assert!(debug_str.contains("ONLINE"));
}

#[test]
fn test_dataset_info_debug() {
    let dataset = create_test_dataset_info("debug_ds", "pool1", StorageTier::Hot);
    let debug_str = format!("{:?}", dataset);
    assert!(debug_str.contains("debug_ds"));
    assert!(debug_str.contains("pool1"));
}

#[test]
fn test_snapshot_info_debug() {
    let snapshot = create_test_snapshot_info("debug_snap", "pool1/ds1");
    let debug_str = format!("{:?}", snapshot);
    assert!(debug_str.contains("debug_snap"));
    assert!(debug_str.contains("pool1/ds1"));
}

// ==================== CONCURRENT ACCESS TESTS ====================

#[tokio::test]
async fn test_concurrent_pool_map_access() {
    let pool_map: Arc<RwLock<HashMap<String, ZeroCostPoolInfo>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let map1 = pool_map.clone();
    let map2 = pool_map.clone();

    let task1 = tokio::spawn(async move {
        let guard = map1.read().await;
        guard.len()
    });

    let task2 = tokio::spawn(async move {
        let guard = map2.read().await;
        guard.len()
    });

    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();

    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_concurrent_dataset_map_access() {
    let dataset_map: Arc<RwLock<HashMap<String, ZeroCostDatasetInfo>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let map1 = dataset_map.clone();
    let map2 = dataset_map.clone();

    let task1 = tokio::spawn(async move {
        let guard = map1.read().await;
        guard.len()
    });

    let task2 = tokio::spawn(async move {
        let guard = map2.read().await;
        guard.len()
    });

    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();

    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_concurrent_snapshot_map_access() {
    let snapshot_map: Arc<RwLock<HashMap<String, ZeroCostSnapshotInfo>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let map1 = snapshot_map.clone();
    let map2 = snapshot_map.clone();

    let task1 = tokio::spawn(async move {
        let guard = map1.read().await;
        guard.len()
    });

    let task2 = tokio::spawn(async move {
        let guard = map2.read().await;
        guard.len()
    });

    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();

    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_concurrent_pool_map_writes() {
    let pool_map: Arc<RwLock<HashMap<String, ZeroCostPoolInfo>>> =
        Arc::new(RwLock::new(HashMap::new()));

    let map1 = pool_map.clone();
    let map2 = pool_map.clone();

    let task1 = tokio::spawn(async move {
        let mut guard = map1.write().await;
        guard.insert("pool1".to_string(), create_test_pool_info("pool1"));
    });

    let task2 = tokio::spawn(async move {
        let mut guard = map2.write().await;
        guard.insert("pool2".to_string(), create_test_pool_info("pool2"));
    });

    task1.await.unwrap();
    task2.await.unwrap();

    let guard = pool_map.read().await;
    assert_eq!(guard.len(), 2);
    assert!(guard.contains_key("pool1"));
    assert!(guard.contains_key("pool2"));
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_pool_with_zero_size() {
    let mut pool = create_test_pool_info("zero_pool");
    pool.size = 0;
    pool.used = 0;
    pool.available = 0;

    assert_eq!(pool.size, 0);
    assert_eq!(pool.used, 0);
    assert_eq!(pool.available, 0);
}

#[test]
fn test_pool_with_full_utilization() {
    let mut pool = create_test_pool_info("full_pool");
    pool.used = pool.size;
    pool.available = 0;

    assert_eq!(pool.used, pool.size);
    assert_eq!(pool.available, 0);
}

#[test]
fn test_dataset_with_empty_name() {
    let mut dataset = create_test_dataset_info("normal_ds", "pool1", StorageTier::Hot);
    dataset.name = String::new();

    assert_eq!(dataset.name, "");
}

#[test]
fn test_snapshot_with_empty_name() {
    let mut snapshot = create_test_snapshot_info("normal_snap", "pool1/ds1");
    snapshot.name = String::new();

    assert_eq!(snapshot.name, "");
}

#[test]
fn test_pool_health_unhealthy() {
    let mut pool = create_test_pool_info("unhealthy_pool");
    pool.health = "DEGRADED".to_string();

    assert_eq!(pool.health, "DEGRADED");
}

#[test]
fn test_large_property_map() {
    let mut pool = create_test_pool_info("large_props_pool");

    for i in 0..1000 {
        pool.properties
            .insert(format!("prop_{}", i), format!("value_{}", i));
    }

    assert_eq!(pool.properties.len(), 1002); // 1000 + 2 from create_test_pool_info
}

// ==================== MANAGER TYPE CHECKS ====================

#[test]
fn test_all_manager_types_exist() {
    // Just verify all type aliases can be instantiated
    let _dev: DevelopmentZfsManager = ZeroCostZfsManager::new();
    let _prod: ProductionZfsManager = ZeroCostZfsManager::new();
    let _high_perf: HighPerformanceZfsManager = ZeroCostZfsManager::new();
    let _testing: TestingZfsManager = ZeroCostZfsManager::new();
    let _enterprise: EnterpriseZfsManager = ZeroCostZfsManager::new();
}
