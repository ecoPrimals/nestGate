// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::types::{DatasetInfo, ZfsDatasetManager};
use super::validation::{
    parse_zfs_dataset_list_line, parse_zfs_list_datasets_row, parse_zfs_snapshot_list_line,
    tier_hint_from_dataset_name,
};
use crate::config::ZfsConfig;
use crate::pool::ZfsPoolManager;
use nestgate_core::canonical_types::StorageTier as CoreStorageTier;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn dataset_info_serde_roundtrip() {
    let info = DatasetInfo {
        name: "pool/ds".to_string(),
        used_space: 1,
        available_space: 2,
        file_count: Some(3),
        compression_ratio: Some(1.25),
        mount_point: "/mnt/pool/ds".to_string(),
        tier: CoreStorageTier::Warm,
        properties: HashMap::from([("a".into(), "b".into())]),
    };
    let json = serde_json::to_string(&info).expect("test: serialize DatasetInfo");
    let back: DatasetInfo = serde_json::from_str(&json).expect("test: deserialize DatasetInfo");
    assert_eq!(back.name, info.name);
    assert_eq!(back.tier, info.tier);
    assert_eq!(back.properties.get("a"), Some(&"b".to_string()));
}

#[tokio::test]
async fn new_for_testing_builds_manager() {
    let m = ZfsDatasetManager::new_for_testing();
    // delete/destroy now run real `zfs destroy` — expect failure on non-existent dataset
    let err = m.delete_dataset("nonexistent-test-dataset").await;
    assert!(err.is_err(), "delete of nonexistent dataset should fail");
    let err = m.destroy_dataset("nonexistent-test-dataset").await;
    assert!(err.is_err(), "destroy of nonexistent dataset should fail");
}

#[test]
fn with_shared_config_and_new_construct_managers() {
    let cfg = Arc::new(ZfsConfig::default());
    let pm = Arc::new(ZfsPoolManager::new_for_testing());
    let _shared = ZfsDatasetManager::with_shared_config(Arc::clone(&cfg), Arc::clone(&pm));

    let owned = ZfsConfig::default();
    let _from_owned = ZfsDatasetManager::new(owned, pm);
}

#[tokio::test]
async fn create_fallback_dataset_info_populates_expected_fields() {
    let m = ZfsDatasetManager::new_for_testing();
    let info = m
        .create_fallback_dataset_info("tank/fs")
        .expect("test: fallback dataset info");
    assert_eq!(info.name, "tank/fs");
    assert_eq!(info.mount_point, "/tank/fs");
    assert_eq!(info.tier, CoreStorageTier::Warm);
    assert!(info.compression_ratio.is_some());
}

#[tokio::test]
#[ignore = "Requires real ZFS"]
async fn create_dataset_runs_zfs_create() {
    let m = ZfsDatasetManager::new_for_testing();
    let _ = m
        .create_dataset("tmpds", "nonexistent_pool_xyz", CoreStorageTier::Warm)
        .await;
}

#[test]
fn dataset_info_serde_all_storage_tiers() {
    for tier in [
        CoreStorageTier::Hot,
        CoreStorageTier::Warm,
        CoreStorageTier::Cold,
        CoreStorageTier::Cache,
        CoreStorageTier::Archive,
    ] {
        let info = DatasetInfo {
            name: "p/d".into(),
            used_space: 1,
            available_space: 2,
            file_count: None,
            compression_ratio: None,
            mount_point: "/m".into(),
            tier: tier.clone(),
            properties: HashMap::new(),
        };
        let json = serde_json::to_string(&info).expect("serialize tier");
        let back: DatasetInfo = serde_json::from_str(&json).expect("deserialize tier");
        assert_eq!(back.tier, tier);
    }
}

#[test]
fn dataset_info_clone_and_debug() {
    let info = DatasetInfo {
        name: "z/a".into(),
        used_space: 10,
        available_space: 20,
        file_count: Some(100),
        compression_ratio: Some(2.0),
        mount_point: "/z/a".into(),
        tier: CoreStorageTier::Cold,
        properties: HashMap::new(),
    };
    let c = info.clone();
    assert_eq!(c.name, info.name);
    let dbg = format!("{info:?}");
    assert!(dbg.contains("DatasetInfo"));
    assert!(dbg.contains("z/a"));
}

#[test]
fn dataset_info_optional_fields_none() {
    let info = DatasetInfo {
        name: "p/x".into(),
        used_space: 0,
        available_space: u64::MAX,
        file_count: None,
        compression_ratio: None,
        mount_point: "-".into(),
        tier: CoreStorageTier::Archive,
        properties: HashMap::new(),
    };
    assert!(info.file_count.is_none());
    assert!(info.compression_ratio.is_none());
}

#[tokio::test]
async fn create_fallback_dataset_info_always_warm_tier() {
    let m = ZfsDatasetManager::new_for_testing();
    let a = m
        .create_fallback_dataset_info("tank/hot_data")
        .expect("fallback");
    let b = m
        .create_fallback_dataset_info("cold/store")
        .expect("fallback");
    assert_eq!(a.tier, CoreStorageTier::Warm);
    assert_eq!(b.tier, CoreStorageTier::Warm);
}

#[test]
fn tier_hint_from_dataset_name_all_variants() {
    assert_eq!(
        tier_hint_from_dataset_name("tank/hot_cache"),
        CoreStorageTier::Hot
    );
    assert_eq!(
        tier_hint_from_dataset_name("tank/cold_archive"),
        CoreStorageTier::Cold
    );
    assert_eq!(
        tier_hint_from_dataset_name("tank/warm_fs"),
        CoreStorageTier::Warm
    );
    assert_eq!(
        tier_hint_from_dataset_name("tank/data"),
        CoreStorageTier::Warm
    );
}

#[test]
fn parse_zfs_dataset_list_line_realistic_tab_output() {
    let line = "tank/app\t1048576\t1073741824\t/tank/app";
    let info = parse_zfs_dataset_list_line("tank/app", line).expect("line parses");
    assert_eq!(info.name, "tank/app");
    assert_eq!(info.used_space, 1048576);
    assert_eq!(info.available_space, 1073741824);
    assert_eq!(info.mount_point, "/tank/app");
    assert_eq!(info.tier, CoreStorageTier::Warm);
}

#[test]
fn parse_zfs_dataset_list_line_hot_and_cold_tiers() {
    let hot = parse_zfs_dataset_list_line("tank/hot/db", "tank/hot/db\t0\t0\t/mnt").expect("p");
    assert_eq!(hot.tier, CoreStorageTier::Hot);
    let cold = parse_zfs_dataset_list_line("store/cold/logs", "x\t1\t2\t-").expect("p");
    assert_eq!(cold.tier, CoreStorageTier::Cold);
}

#[test]
fn parse_zfs_dataset_list_line_short_line_returns_none() {
    assert!(parse_zfs_dataset_list_line("x", "only\t2").is_none());
}

#[test]
fn parse_zfs_list_datasets_row_sets_tier_from_name() {
    let row = parse_zfs_list_datasets_row("tank/hot/d\t100\t200\t/mnt").expect("row");
    assert_eq!(row.name, "tank/hot/d");
    assert_eq!(row.tier, CoreStorageTier::Hot);
}

#[test]
fn parse_zfs_snapshot_list_line_parses_at_sign() {
    let line = "tank/ds@s1\t4096\t8192\t1234567890";
    let s = parse_zfs_snapshot_list_line(line, "tank/ds").expect("snap");
    assert_eq!(s.full_name, "tank/ds@s1");
    assert_eq!(s.name, "s1");
    assert_eq!(s.dataset, "tank/ds");
    assert_eq!(s.size, 4096);
    assert_eq!(s.referenced_size, 8192);
}

#[test]
fn parse_zfs_snapshot_list_line_incomplete_returns_none() {
    assert!(parse_zfs_snapshot_list_line("bad", "ds").is_none());
}
