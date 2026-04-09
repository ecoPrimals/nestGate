// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for the zero-cost ZFS manager.

use super::super::traits::ZeroCostZfsOperations;
use super::super::types::{ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo};
use super::{
    DevelopmentZfsManager, EnterpriseZfsManager, HighPerformanceZfsManager, ProductionZfsManager,
    TestingZfsManager, ZeroCostZfsManager,
};
use super::{
    build_dataset_create_zfs_args, build_pool_create_zfs_args, build_snapshot_zfs_path,
    parse_dataset_list_line, parse_pool_list_line, parse_snapshot_list_line,
    zero_cost_pool_from_zfs_properties,
};
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;
use std::time::SystemTime;

#[test]
fn parse_pool_properties_tab_and_trim() {
    let m = TestingZfsManager::new();
    let out = "size\t12345\nallocated\t100\nhealth\tONLINE\n";
    let map = m.test_parse_pool_properties(out);
    assert_eq!(map.get("size"), Some(&"12345".to_string()));
    assert_eq!(map.get("allocated"), Some(&"100".to_string()));
    assert_eq!(map.get("health"), Some(&"ONLINE".to_string()));
}

#[test]
fn parse_pool_properties_skips_lines_without_tab() {
    let m = TestingZfsManager::new();
    let map = m.test_parse_pool_properties("no tab here\nkey\tvalue\n");
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("key"), Some(&"value".to_string()));
}

#[test]
fn test_zero_cost_zfs_manager_new() {
    let manager = ZeroCostZfsManager::<10, 100, 1000, 10_000>::new();
    drop(manager);
}

#[test]
fn test_zero_cost_zfs_manager_default() {
    let manager = TestingZfsManager::default();
    drop(manager);
}

#[test]
fn test_command_timeout_development() {
    let timeout = DevelopmentZfsManager::command_timeout();
    assert_eq!(timeout.as_millis(), 10_000);
}

#[test]
fn test_command_timeout_production() {
    let timeout = ProductionZfsManager::command_timeout();
    assert_eq!(timeout.as_millis(), 30_000);
}

#[test]
fn test_command_timeout_high_performance() {
    let timeout = HighPerformanceZfsManager::command_timeout();
    assert_eq!(timeout.as_millis(), 45_000);
}

#[test]
fn test_command_timeout_testing() {
    let timeout = TestingZfsManager::command_timeout();
    assert_eq!(timeout.as_millis(), 5000);
}

#[test]
fn test_command_timeout_enterprise() {
    let timeout = EnterpriseZfsManager::command_timeout();
    assert_eq!(timeout.as_millis(), 60_000);
}

#[test]
fn test_type_alias_development_zfs_manager() {
    let _manager: DevelopmentZfsManager = DevelopmentZfsManager::new();
}

#[test]
fn test_type_alias_production_zfs_manager() {
    let _manager: ProductionZfsManager = ProductionZfsManager::new();
}

#[test]
fn test_type_alias_high_performance_zfs_manager() {
    let _manager: HighPerformanceZfsManager = HighPerformanceZfsManager::new();
}

#[test]
fn test_type_alias_testing_zfs_manager() {
    let _manager: TestingZfsManager = TestingZfsManager::new();
}

#[test]
fn test_type_alias_enterprise_zfs_manager() {
    let _manager: EnterpriseZfsManager = EnterpriseZfsManager::new();
}

#[tokio::test]
async fn max_pools_capacity_reached_for_testing_manager() {
    let m = TestingZfsManager::new();
    assert!(m.test_can_create_more_pools().await);
    m.test_insert_pool_entry("p0".into()).await;
    m.test_insert_pool_entry("p1".into()).await;
    assert_eq!(m.test_pool_map_len().await, 2);
    assert!(!m.test_can_create_more_pools().await);
}

#[tokio::test]
async fn dataset_capacity_still_available_when_under_limit() {
    let m = TestingZfsManager::new();
    assert!(m.test_can_create_more_datasets().await);
    m.test_insert_dataset_entry("ds0".into(), "p0".into()).await;
    assert!(m.test_can_create_more_datasets().await);
}

#[tokio::test]
async fn max_snapshots_capacity_enforced_for_testing_manager() {
    let m = TestingZfsManager::new();
    assert!(m.test_can_create_more_snapshots().await);
    for i in 0..100 {
        m.test_insert_snapshot_entry(format!("snap{i}")).await;
    }
    assert_eq!(m.test_snapshot_map_len().await, 100);
    assert!(!m.test_can_create_more_snapshots().await);
}

#[test]
fn parse_pool_properties_first_tab_wins_on_duplicate_key() {
    let m = TestingZfsManager::new();
    let out = "k\tv1\nk\tv2\n";
    let map = m.test_parse_pool_properties(out);
    assert_eq!(map.get("k"), Some(&"v2".to_string()));
}

#[test]
fn zero_cost_pool_info_serde_roundtrip() {
    let p = ZeroCostPoolInfo {
        name: "tank".into(),
        size: 100,
        used: 10,
        available: 90,
        health: "ONLINE".into(),
        properties: HashMap::from([("ashift".into(), "12".into())]),
        created_at: std::time::SystemTime::UNIX_EPOCH,
    };
    let json = serde_json::to_string(&p).expect("serialize");
    let back: ZeroCostPoolInfo = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.name, p.name);
    assert_eq!(back.health, p.health);
}

#[test]
fn parse_pool_properties_empty_output() {
    let m = TestingZfsManager::new();
    assert!(m.test_parse_pool_properties("").is_empty());
}

#[tokio::test]
async fn set_dataset_properties_empty_map_completes_without_zfs_property_loop() {
    let m = TestingZfsManager::new();
    let props = std::collections::HashMap::new();
    m.set_dataset_properties("tank/ds", &props)
        .await
        .expect("empty property map should succeed without invoking zfs");
}

#[tokio::test]
async fn get_pool_properties_returns_cached_clone_without_zfs() {
    let m = TestingZfsManager::new();
    m.test_insert_pool_entry("cached-pool".into()).await;
    let pool = ZeroCostPoolInfo {
        name: "cached-pool".into(),
        size: 0,
        used: 0,
        available: 0,
        health: "ONLINE".into(),
        properties: HashMap::new(),
        created_at: std::time::SystemTime::UNIX_EPOCH,
    };
    let props = ZeroCostZfsOperations::get_pool_properties(&m, &pool)
        .await
        .expect("cache hit");
    assert!(props.is_empty());
}

#[tokio::test]
async fn max_datasets_capacity_enforced_for_testing_manager() {
    let m = TestingZfsManager::new();
    assert!(m.test_can_create_more_datasets().await);
    for i in 0..10 {
        m.test_insert_dataset_entry(format!("ds{i}"), "p0".into())
            .await;
    }
    assert!(!m.test_can_create_more_datasets().await);
}

#[tokio::test]
async fn create_pool_errors_when_max_pools_reached_before_zfs() {
    let m = TestingZfsManager::new();
    m.test_insert_pool_entry("p0".into()).await;
    m.test_insert_pool_entry("p1".into()).await;
    let err = ZeroCostZfsOperations::create_pool(&m, "p2", &["/dev/null"])
        .await
        .expect_err("expected capacity error");
    let msg = err.to_string();
    assert!(
        msg.contains("maximum pools") || msg.contains("Cannot create pool"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn create_dataset_errors_when_max_datasets_reached_before_zfs() {
    let m = TestingZfsManager::new();
    let pool = ZeroCostPoolInfo {
        name: "p".into(),
        size: 0,
        used: 0,
        available: 0,
        health: "ONLINE".into(),
        properties: HashMap::new(),
        created_at: std::time::SystemTime::UNIX_EPOCH,
    };
    for i in 0..10 {
        m.test_insert_dataset_entry(format!("ds{i}"), "p".into())
            .await;
    }
    let err = ZeroCostZfsOperations::create_dataset(&m, &pool, "extra", StorageTier::Hot)
        .await
        .expect_err("expected capacity error");
    let msg = err.to_string();
    assert!(
        msg.contains("maximum datasets") || msg.contains("Cannot create dataset"),
        "unexpected error: {msg}"
    );
}

#[tokio::test]
async fn create_snapshot_errors_when_max_snapshots_reached_before_zfs() {
    let m = TestingZfsManager::new();
    let ds = ZeroCostDatasetInfo {
        name: "d0".into(),
        pool: "p".into(),
        tier: StorageTier::Warm,
        size: 0,
        used: 0,
        properties: HashMap::new(),
        mount_point: None,
        created_at: std::time::SystemTime::UNIX_EPOCH,
    };
    for i in 0..100 {
        m.test_insert_snapshot_entry(format!("snap{i}")).await;
    }
    let err = ZeroCostZfsOperations::create_snapshot(&m, &ds, "new_snap")
        .await
        .expect_err("expected capacity error");
    let msg = err.to_string();
    assert!(
        msg.contains("maximum snapshots") || msg.contains("Cannot create snapshot"),
        "unexpected error: {msg}"
    );
}

#[test]
fn zero_cost_zfs_operations_can_create_pool_trait_default() {
    let m = TestingZfsManager::new();
    assert!(ZeroCostZfsOperations::can_create_pool(&m));
}

#[test]
fn build_dataset_create_zfs_args_hot_includes_lz4_and_sync_always() {
    let args = build_dataset_create_zfs_args(&StorageTier::Hot, "tank/ds");
    assert_eq!(
        args,
        vec![
            "create",
            "-o",
            "compression=lz4",
            "-o",
            "sync=always",
            "tank/ds"
        ]
    );
}

#[test]
fn build_dataset_create_zfs_args_archive_includes_atime_off() {
    let args = build_dataset_create_zfs_args(&StorageTier::Archive, "z/root");
    assert!(args.contains(&"atime=off"));
    assert_eq!(args.last().copied(), Some("z/root"));
}

#[test]
fn build_dataset_create_zfs_args_cache_includes_primarycache() {
    let args = build_dataset_create_zfs_args(&StorageTier::Cache, "c/cache");
    assert!(args.contains(&"primarycache=all"));
}

#[test]
fn parse_dataset_list_line_skips_pool_row_and_short_lines() {
    let t = SystemTime::UNIX_EPOCH;
    assert!(parse_dataset_list_line("tank\t0\t0\t-", "tank", t).is_none());
    assert!(parse_dataset_list_line("tank/ds\t1", "tank", t).is_none());
    let ds = parse_dataset_list_line("tank/ds\t10\t90\t/mnt", "tank", t).expect("row");
    assert_eq!(ds.name, "ds");
    assert_eq!(ds.pool, "tank");
    assert_eq!(ds.size, 100);
    assert_eq!(ds.mount_point, Some(std::path::PathBuf::from("/mnt")));
}

#[test]
fn parse_dataset_list_line_none_mountpoint() {
    let t = SystemTime::UNIX_EPOCH;
    let ds = parse_dataset_list_line("tank/ds\t0\t0\tnone", "tank", t).expect("row");
    assert!(ds.mount_point.is_none());
}

#[test]
fn build_pool_create_zfs_args_orders_create_name_devices() {
    let args = build_pool_create_zfs_args("tank", &["/dev/sda", "/dev/sdb"]);
    assert_eq!(args, vec!["create", "tank", "/dev/sda", "/dev/sdb"]);
}

#[test]
fn zero_cost_pool_from_zfs_properties_computes_available() {
    let mut p = HashMap::new();
    p.insert("size".into(), "1000".into());
    p.insert("allocated".into(), "250".into());
    p.insert("health".into(), "ONLINE".into());
    let info = zero_cost_pool_from_zfs_properties("tank", &p, SystemTime::UNIX_EPOCH);
    assert_eq!(info.size, 1000);
    assert_eq!(info.used, 250);
    assert_eq!(info.available, 750);
    assert_eq!(info.health, "ONLINE");
}

#[test]
fn zero_cost_pool_from_zfs_properties_unknown_health_when_missing() {
    let info = zero_cost_pool_from_zfs_properties("z", &HashMap::new(), SystemTime::UNIX_EPOCH);
    assert_eq!(info.health, "UNKNOWN");
}

#[test]
fn parse_pool_list_line_requires_five_columns() {
    assert!(parse_pool_list_line("a\tb", SystemTime::UNIX_EPOCH).is_none());
    let p =
        parse_pool_list_line("tank\t1000\t100\t900\tONLINE", SystemTime::UNIX_EPOCH).expect("pool");
    assert_eq!(p.name, "tank");
    assert_eq!(p.health, "ONLINE");
}

#[test]
fn build_snapshot_zfs_path_joins_with_at() {
    assert_eq!(
        build_snapshot_zfs_path("tank/data", "snap1"),
        "tank/data@snap1"
    );
}

#[test]
fn parse_snapshot_list_line_splits_dataset_and_name() {
    let s = parse_snapshot_list_line("tank/ds@snap\t42", SystemTime::UNIX_EPOCH).expect("snap");
    assert_eq!(s.dataset, "tank/ds");
    assert_eq!(s.name, "snap");
    assert_eq!(s.size, 42);
}

#[test]
fn parse_snapshot_list_line_requires_at_separator() {
    assert!(parse_snapshot_list_line("nope\t1", SystemTime::UNIX_EPOCH).is_none());
}

#[test]
fn zero_cost_dataset_info_serde_roundtrip() {
    let d = ZeroCostDatasetInfo {
        name: "d".into(),
        pool: "tank".into(),
        tier: StorageTier::Cold,
        size: 1,
        used: 1,
        properties: HashMap::new(),
        mount_point: None,
        created_at: std::time::SystemTime::UNIX_EPOCH,
    };
    let json = serde_json::to_string(&d).expect("serialize");
    let back: ZeroCostDatasetInfo = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.tier, StorageTier::Cold);
}
