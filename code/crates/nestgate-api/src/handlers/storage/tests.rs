// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use axum::Json;

use super::StorageHandler;
use super::http_handlers::{
    get_storage_datasets, get_storage_metrics, get_storage_pools, get_storage_snapshots,
};
use super::probes;
use super::types::{
    StorageDataset, StorageDatasetInfo, StorageManager, StorageMetrics, StoragePool,
    StoragePoolInfo, StorageSnapshot, StorageSnapshotInfo,
};

#[test]
fn test_storage_handler_new_and_default() {
    let _ = StorageHandler::new();
    let _ = StorageHandler;
}

#[test]
fn test_storage_pool_serialization() {
    let pool = StoragePool {
        name: "main-pool".to_string(),
        status: "ONLINE".to_string(),
        size: 1_000_000_000,
        used: 400_000_000,
        available: 600_000_000,
        health: "HEALTHY".to_string(),
        pool_type: "raidz".to_string(),
    };
    let json = serde_json::to_string(&pool).unwrap();
    assert!(json.contains("main-pool"));
    let parsed: StoragePool = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.size, 1_000_000_000);
}

#[test]
fn test_storage_dataset_serialization() {
    let dataset = StorageDataset {
        name: "main-pool/data".to_string(),
        pool: "main-pool".to_string(),
        size: 500_000_000,
        used: 200_000_000,
        available: 300_000_000,
        mount_point: "/data".to_string(),
        compression: "lz4".to_string(),
    };
    let json = serde_json::to_string(&dataset).unwrap();
    assert!(json.contains("main-pool/data"));
}

#[test]
fn test_storage_snapshot_serialization() {
    let snapshot = StorageSnapshot {
        name: "pool@snap1".to_string(),
        dataset: "pool".to_string(),
        size: 100_000_000,
        created: "2024-01-15".to_string(),
        referenced: 80_000_000,
    };
    let json = serde_json::to_string(&snapshot).unwrap();
    assert!(json.contains("pool@snap1"));
}

#[test]
fn test_storage_metrics_serialization() {
    let metrics = StorageMetrics {
        total_pools: 2,
        total_datasets: 5,
        total_snapshots: 12,
        total_storage: 1_500_000_000_000,
        used_storage: 550_000_000_000,
        available_storage: 950_000_000_000,
        iops: 1250.0,
        bandwidth_mbps: 450.5,
        health_status: "healthy".to_string(),
    };
    let json = serde_json::to_string(&metrics).unwrap();
    assert!(json.contains("healthy"));
}

#[test]
fn test_storage_pool_info_serialization() {
    let info = StoragePoolInfo {
        name: "main-pool".to_string(),
        total_capacity_gb: 1000,
        used_capacity_gb: 400,
        available_capacity_gb: 600,
        health_status: "healthy".to_string(),
    };
    let json = serde_json::to_string(&info).unwrap();
    assert!(json.contains("main-pool"));
}

#[test]
fn test_storage_dataset_info_serialization() {
    let info = StorageDatasetInfo {
        name: "main-pool/data".to_string(),
        pool_name: "main-pool".to_string(),
        used_space_gb: 200,
        compression_ratio: 1.5,
        dedup_ratio: 1.2,
    };
    let json = serde_json::to_string(&info).unwrap();
    assert!(json.contains("main-pool/data"));
}

#[tokio::test]
async fn test_get_storage_pools() {
    let result = get_storage_pools().await;
    assert!(result.is_ok());
    let Json(pools) = result.unwrap();
    assert_eq!(pools.len(), 2);
    assert_eq!(pools[0].name, "main-pool");
}

#[tokio::test]
async fn test_get_storage_datasets() {
    let result = get_storage_datasets().await;
    assert!(result.is_ok());
    let Json(datasets) = result.unwrap();
    assert_eq!(datasets.len(), 2);
}

#[tokio::test]
async fn test_get_storage_snapshots() {
    let result = get_storage_snapshots().await;
    assert!(result.is_ok());
    let Json(snapshots) = result.unwrap();
    assert_eq!(snapshots.len(), 2);
}

#[tokio::test]
async fn test_get_storage_metrics() {
    let result = get_storage_metrics().await;
    assert!(result.is_ok());
    let Json(metrics) = result.unwrap();
    assert_eq!(metrics.total_pools, 2);
    assert_eq!(metrics.health_status, "healthy");
}

#[test]
fn test_storage_manager_new() {
    let _ = StorageManager::new();
    let _ = StorageManager::default();
}

#[test]
fn parse_size_string_kb_mb_gb() {
    assert_eq!(probes::parse_size_string("100"), Some(100));
    assert_eq!(probes::parse_size_string("-"), Some(0));
    assert!(probes::parse_size_string("2M").unwrap() >= 2 * 1024 * 1024);
}

#[test]
fn parse_size_string_terabyte_and_petabyte_units() {
    let one_t = probes::parse_size_string("1T").expect("1T");
    assert_eq!(one_t, 1024_u64.pow(4));
    let one_tb = probes::parse_size_string("1TB").expect("1TB");
    assert_eq!(one_tb, 1024_u64.pow(4));
    let two_p = probes::parse_size_string("2P").expect("2P");
    assert_eq!(two_p, 2 * 1024_u64.pow(5));
}

#[test]
fn parse_size_string_invalid_number_is_none() {
    assert!(probes::parse_size_string("abcG").is_none());
}

#[test]
fn parse_size_string_trims_whitespace() {
    assert_eq!(probes::parse_size_string("  10K  "), Some(10 * 1024));
}

#[test]
fn should_include_filesystem_filters_pseudo_fs() {
    assert!(!probes::should_include_filesystem("tmpfs", "tmpfs", "/tmp"));
    assert!(!probes::should_include_filesystem(
        "/dev/sda1",
        "ext4",
        "/proc/foo"
    ));
    assert!(probes::should_include_filesystem("/dev/sda1", "ext4", "/"));
    assert!(probes::should_include_filesystem(
        "/dev/sdb1",
        "xfs",
        "/home/user"
    ));
    assert!(probes::should_include_filesystem(
        "/dev/sdc1",
        "ext4",
        "/mnt/backup"
    ));
    assert!(probes::should_include_filesystem(
        "/dev/sdd1",
        "ext4",
        "/media/usb"
    ));
}

#[test]
fn parse_bandwidth_unit_basic() {
    assert_eq!(probes::parse_bandwidth_unit("-"), Some(0.0));
    assert_eq!(probes::parse_bandwidth_unit(""), Some(0.0));
    assert!(probes::parse_bandwidth_unit("10M").unwrap() > 0.0);
    let k = probes::parse_bandwidth_unit("1024K").expect("K");
    assert!(k > 0.0);
    let g = probes::parse_bandwidth_unit("1G").expect("G");
    assert!(g > 100.0);
    let raw = probes::parse_bandwidth_unit("100").expect("raw");
    assert!(raw >= 0.0);
}

#[test]
fn create_fallback_root_pool_has_name() {
    let p = probes::create_fallback_root_pool();
    assert!(p.name.contains("root"));
}

// --- Round 6: `should_include_filesystem` / `parse_size_string` branches ---

#[test]
fn r6_should_include_btrfs_on_root() {
    assert!(probes::should_include_filesystem(
        "/dev/xvda1",
        "btrfs",
        "/"
    ));
}

#[test]
fn r6_should_include_xfs_under_media() {
    assert!(probes::should_include_filesystem(
        "/dev/sdc1",
        "xfs",
        "/media/usbstick"
    ));
}

#[test]
fn r6_should_exclude_sysfs_mount() {
    assert!(!probes::should_include_filesystem(
        "sysfs",
        "sysfs",
        "/sys/fs/cgroup"
    ));
}

#[test]
fn r6_should_exclude_devtmpfs_under_dev() {
    assert!(!probes::should_include_filesystem(
        "devtmpfs", "devtmpfs", "/dev"
    ));
}

#[test]
fn r6_parse_size_lowercase_kb() {
    let v = probes::parse_size_string("512kb").expect("kb");
    assert!(v >= 512 * 1024);
}

#[test]
fn r6_parse_size_uppercase_mb() {
    let v = probes::parse_size_string("3MB").expect("mb");
    assert_eq!(v, 3 * 1024 * 1024);
}

#[test]
fn r6_parse_size_no_unit_is_bytes() {
    assert_eq!(probes::parse_size_string("42"), Some(42));
}

#[test]
fn r6_parse_bandwidth_gigabit_style() {
    let v = probes::parse_bandwidth_unit("2G").expect("g");
    assert!(v > 1000.0);
}
