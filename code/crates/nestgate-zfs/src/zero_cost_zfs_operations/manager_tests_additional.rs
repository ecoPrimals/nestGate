// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Additional comprehensive tests for ZeroCostZfsManager
//!
//! Tests focused on publicly accessible methods and type configurations

use super::{ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo, ZeroCostZfsManager};
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Standard test type alias
type TestManager = ZeroCostZfsManager<16, 128, 512, 5000>;

// ==================== MANAGER CREATION TESTS ====================

#[test]
fn test_manager_creation() {
    let _manager = TestManager::new();
    // Manager should be created successfully
}

#[test]
fn test_manager_default() {
    let _manager = TestManager::default();
    // Default trait should create valid manager
}

// ==================== TIMEOUT CONFIGURATION TESTS ====================

#[test]
fn test_command_timeout_duration() {
    let timeout = TestManager::command_timeout();
    assert_eq!(timeout.as_millis(), 5000);
}

#[test]
fn test_different_timeout_configurations() {
    /// Type alias for FastManager
    type FastManager = ZeroCostZfsManager<16, 128, 512, 1000>;
    /// Type alias for SlowManager
    type SlowManager = ZeroCostZfsManager<16, 128, 512, 10000>;

    let fast_timeout = FastManager::command_timeout();
    let slow_timeout = SlowManager::command_timeout();

    assert_eq!(fast_timeout.as_millis(), 1000);
    assert_eq!(slow_timeout.as_millis(), 10000);
    assert!(slow_timeout > fast_timeout);
}

#[test]
fn test_very_short_timeout() {
    /// Type alias for VeryFastManager
    type VeryFastManager = ZeroCostZfsManager<16, 128, 512, 100>;
    let timeout = VeryFastManager::command_timeout();
    assert_eq!(timeout.as_millis(), 100);
}

#[test]
fn test_very_long_timeout() {
    /// Type alias for VerySlowManager
    type VerySlowManager = ZeroCostZfsManager<16, 128, 512, 60000>;
    let timeout = VerySlowManager::command_timeout();
    assert_eq!(timeout.as_millis(), 60000);
}

// ==================== CAPACITY CONFIGURATION TESTS ====================

#[test]
fn test_small_capacity_manager() {
    /// Type alias for SmallManager
    type SmallManager = ZeroCostZfsManager<1, 10, 100, 5000>;
    let _manager = SmallManager::new();
    // Should create with small capacities
}

#[test]
fn test_large_capacity_manager() {
    /// Type alias for LargeManager
    type LargeManager = ZeroCostZfsManager<1000, 100000, 1000000, 5000>;
    let _manager = LargeManager::new();
    // Should create with large capacities
}

#[test]
fn test_different_capacity_configurations() {
    /// Type alias for Config1
    type Config1 = ZeroCostZfsManager<10, 100, 1000, 5000>;
    /// Type alias for Config2
    type Config2 = ZeroCostZfsManager<20, 200, 2000, 5000>;
    /// Type alias for Config3
    type Config3 = ZeroCostZfsManager<30, 300, 3000, 5000>;

    let _m1 = Config1::new();
    let _m2 = Config2::new();
    let _m3 = Config3::new();
    // All configurations should be valid
}

// ==================== TYPE INFO TESTS ====================

#[test]
fn test_pool_info_creation() {
    let pool_info = ZeroCostPoolInfo {
        name: "test_pool".to_string(),
        size: 1_000_000,
        used: 250_000,
        available: 750_000,
        health: "ONLINE".to_string(),
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert_eq!(pool_info.name, "test_pool");
    assert_eq!(pool_info.size, 1_000_000);
    assert_eq!(pool_info.used, 250_000);
}

#[test]
fn test_dataset_info_creation() {
    let dataset_info = ZeroCostDatasetInfo {
        name: "pool/dataset".to_string(),
        pool: "pool".to_string(),
        size: 1000,
        used: 100,
        tier: StorageTier::Hot,
        mount_point: Some(PathBuf::from("/mnt/test")),
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert_eq!(dataset_info.name, "pool/dataset");
    assert_eq!(dataset_info.tier, StorageTier::Hot);
}

#[test]
fn test_snapshot_info_creation() {
    let snapshot_info = ZeroCostSnapshotInfo {
        name: "pool/dataset@snap".to_string(),
        dataset: "pool/dataset".to_string(),
        size: 1024,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert_eq!(snapshot_info.name, "pool/dataset@snap");
    assert_eq!(snapshot_info.size, 1024);
}

// ==================== PROPERTY MAP TESTS ====================

#[test]
fn test_pool_info_with_properties() {
    let mut properties = HashMap::new();
    properties.insert("compression".to_string(), "lz4".to_string());
    properties.insert("atime".to_string(), "off".to_string());

    let pool_info = ZeroCostPoolInfo {
        name: "pool".to_string(),
        size: 1000,
        used: 0,
        available: 1000,
        health: "ONLINE".to_string(),
        properties,
        created_at: SystemTime::now(),
    };

    assert_eq!(pool_info.properties.len(), 2);
    assert_eq!(
        pool_info.properties.get("compression"),
        Some(&"lz4".to_string())
    );
}

#[test]
fn test_dataset_info_with_properties() {
    let mut properties = HashMap::new();
    properties.insert("quota".to_string(), "10G".to_string());

    let dataset_info = ZeroCostDatasetInfo {
        name: "pool/ds".to_string(),
        pool: "pool".to_string(),
        size: 1000,
        used: 100,
        tier: StorageTier::Warm,
        mount_point: None,
        properties,
        created_at: SystemTime::now(),
    };

    assert_eq!(dataset_info.properties.len(), 1);
}

#[test]
fn test_snapshot_info_with_properties() {
    let mut properties = HashMap::new();
    properties.insert("used".to_string(), "512".to_string());

    let snapshot_info = ZeroCostSnapshotInfo {
        name: "snap".to_string(),
        dataset: "ds".to_string(),
        size: 512,
        properties,
        created_at: SystemTime::now(),
    };

    assert_eq!(snapshot_info.properties.len(), 1);
}

// ==================== TIER TESTS ====================

#[test]
fn test_all_storage_tiers() {
    let hot = ZeroCostDatasetInfo {
        name: "hot".to_string(),
        pool: "p".to_string(),
        size: 1000,
        used: 100,
        tier: StorageTier::Hot,
        mount_point: None,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    let warm = ZeroCostDatasetInfo {
        name: "warm".to_string(),
        pool: "p".to_string(),
        size: 1000,
        used: 100,
        tier: StorageTier::Warm,
        mount_point: None,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    let cold = ZeroCostDatasetInfo {
        name: "cold".to_string(),
        pool: "p".to_string(),
        size: 1000,
        used: 100,
        tier: StorageTier::Cold,
        mount_point: None,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert_eq!(hot.tier, StorageTier::Hot);
    assert_eq!(warm.tier, StorageTier::Warm);
    assert_eq!(cold.tier, StorageTier::Cold);
}

// ==================== MOUNT POINT TESTS ====================

#[test]
fn test_dataset_with_mount_point() {
    let dataset = ZeroCostDatasetInfo {
        name: "ds".to_string(),
        pool: "p".to_string(),
        size: 1000,
        used: 0,
        tier: StorageTier::Hot,
        mount_point: Some(PathBuf::from("/mnt/data")),
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert!(dataset.mount_point.is_some());
    assert_eq!(dataset.mount_point.unwrap(), PathBuf::from("/mnt/data"));
}

#[test]
fn test_dataset_without_mount_point() {
    let dataset = ZeroCostDatasetInfo {
        name: "ds".to_string(),
        pool: "p".to_string(),
        size: 1000,
        used: 0,
        tier: StorageTier::Hot,
        mount_point: None,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert!(dataset.mount_point.is_none());
}

// ==================== HEALTH STATUS TESTS ====================

#[test]
fn test_pool_health_online() {
    let pool = ZeroCostPoolInfo {
        name: "pool".to_string(),
        size: 1000,
        used: 0,
        available: 1000,
        health: "ONLINE".to_string(),
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert_eq!(pool.health, "ONLINE");
}

#[test]
fn test_pool_health_degraded() {
    let pool = ZeroCostPoolInfo {
        name: "pool".to_string(),
        size: 1000,
        used: 0,
        available: 1000,
        health: "DEGRADED".to_string(),
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert_eq!(pool.health, "DEGRADED");
}

// Total tests added: 20
// Focus areas:
// - Manager creation (2 tests)
// - Timeout configuration (5 tests)
// - Capacity configuration (3 tests)
// - Type info creation (3 tests)
// - Property maps (3 tests)
// - Storage tiers (1 test)
// - Mount points (2 tests)
// - Health status (2 tests)
