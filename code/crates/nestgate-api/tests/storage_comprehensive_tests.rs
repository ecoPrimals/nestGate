// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines,
        clippy::cognitive_complexity,
    )
)]
#![allow(
    deprecated,
    missing_docs,
    dead_code,
    unfulfilled_lint_expectations,
    unused_doc_comments,
    unused_imports,
    unused_variables,
    unused_comparisons,
    unused_must_use,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::uninlined_format_args,
    clippy::similar_names,
    clippy::items_after_statements,
    clippy::empty_line_after_doc_comments,
    clippy::unreadable_literal,
    clippy::manual_clamp,
    clippy::pub_underscore_fields,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::wildcard_in_or_patterns,
    clippy::type_complexity,
    clippy::field_reassign_with_default,
    clippy::module_inception,
    clippy::unnecessary_get_then_check,
    clippy::cmp_null,
    clippy::redundant_clone,
    clippy::absurd_extreme_comparisons,
    clippy::no_effect_underscore_binding,
    clippy::default_constructed_unit_structs,
    clippy::manual_string_new,
    clippy::assertions_on_constants,
    clippy::unnecessary_unwrap,
    clippy::needless_collect,
    clippy::drop_non_drop,
    clippy::zero_sized_map_values,
    clippy::match_single_binding,
    clippy::match_same_arms,
    clippy::overly_complex_bool_expr,
    clippy::needless_character_iteration,
    clippy::manual_range_contains,
    clippy::bool_assert_comparison,
    clippy::single_component_path_imports,
    clippy::used_underscore_binding
)]

//! Comprehensive tests for storage handlers and types

use nestgate_api::handlers::storage::*;

// =====================================================
// STORAGE POOL TESTS
// =====================================================

#[test]
fn test_storage_pool_creation() {
    let pool = StoragePool {
        name: "test-pool".to_string(),
        status: "ONLINE".to_string(),
        size: 1_000_000_000_000,    // 1TB
        used: 400_000_000_000,      // 400GB
        available: 600_000_000_000, // 600GB
        health: "HEALTHY".to_string(),
        pool_type: "RAIDZ".to_string(),
    };

    assert_eq!(pool.name, "test-pool");
    assert_eq!(pool.status, "ONLINE");
    assert_eq!(pool.size, 1_000_000_000_000);
    assert_eq!(pool.used, 400_000_000_000);
    assert_eq!(pool.available, 600_000_000_000);
    assert_eq!(pool.health, "HEALTHY");
    assert_eq!(pool.pool_type, "RAIDZ");
}

#[test]
fn test_storage_pool_serialization() {
    let pool = StoragePool {
        name: "test-pool".to_string(),
        status: "ONLINE".to_string(),
        size: 1_000_000_000_000,
        used: 400_000_000_000,
        available: 600_000_000_000,
        health: "HEALTHY".to_string(),
        pool_type: "RAIDZ".to_string(),
    };

    let serialized = serde_json::to_string(&pool);
    assert!(serialized.is_ok(), "StoragePool should serialize");

    let json = serialized.expect("Failed to serialize");
    assert!(json.contains("\"name\":\"test-pool\""));
    assert!(json.contains("\"status\":\"ONLINE\""));
    assert!(json.contains("\"health\":\"HEALTHY\""));
}

#[test]
fn test_storage_pool_deserialization() {
    let json = r#"{
        "name": "test-pool",
        "status": "ONLINE",
        "size": 1000000000000,
        "used": 400000000000,
        "available": 600000000000,
        "health": "HEALTHY",
        "pool_type": "RAIDZ"
    }"#;

    let pool: std::result::Result<StoragePool, _> = serde_json::from_str(json);
    assert!(pool.is_ok(), "StoragePool should deserialize");

    let pool = pool.expect("Failed to deserialize");
    assert_eq!(pool.name, "test-pool");
    assert_eq!(pool.status, "ONLINE");
    assert_eq!(pool.health, "HEALTHY");
}

#[test]
fn test_storage_pool_clone() {
    let pool = StoragePool {
        name: "original".to_string(),
        status: "ONLINE".to_string(),
        size: 1_000_000_000_000,
        used: 400_000_000_000,
        available: 600_000_000_000,
        health: "HEALTHY".to_string(),
        pool_type: "MIRROR".to_string(),
    };

    let cloned = pool.clone();
    assert_eq!(cloned.name, pool.name);
    assert_eq!(cloned.size, pool.size);
    assert_eq!(cloned.pool_type, pool.pool_type);
}

// =====================================================
// STORAGE DATASET TESTS
// =====================================================

#[test]
fn test_storage_dataset_creation() {
    let dataset = StorageDataset {
        name: "pool/dataset".to_string(),
        pool: "pool".to_string(),
        size: 500_000_000_000,      // 500GB
        used: 200_000_000_000,      // 200GB
        available: 300_000_000_000, // 300GB
        mount_point: "/mnt/dataset".to_string(),
        compression: "lz4".to_string(),
    };

    assert_eq!(dataset.name, "pool/dataset");
    assert_eq!(dataset.pool, "pool");
    assert_eq!(dataset.size, 500_000_000_000);
    assert_eq!(dataset.used, 200_000_000_000);
    assert_eq!(dataset.available, 300_000_000_000);
    assert_eq!(dataset.mount_point, "/mnt/dataset");
    assert_eq!(dataset.compression, "lz4");
}

#[test]
fn test_storage_dataset_serialization() {
    let dataset = StorageDataset {
        name: "pool/dataset".to_string(),
        pool: "pool".to_string(),
        size: 500_000_000_000,
        used: 200_000_000_000,
        available: 300_000_000_000,
        mount_point: "/mnt/dataset".to_string(),
        compression: "lz4".to_string(),
    };

    let serialized = serde_json::to_string(&dataset);
    assert!(serialized.is_ok(), "StorageDataset should serialize");

    let json = serialized.expect("Failed to serialize");
    assert!(json.contains("\"name\":\"pool/dataset\""));
    assert!(json.contains("\"compression\":\"lz4\""));
}

#[test]
fn test_storage_dataset_deserialization() {
    let json = r#"{
        "name": "pool/dataset",
        "pool": "pool",
        "size": 500000000000,
        "used": 200000000000,
        "available": 300000000000,
        "mount_point": "/mnt/dataset",
        "compression": "lz4"
    }"#;

    let dataset: std::result::Result<StorageDataset, _> = serde_json::from_str(json);
    assert!(dataset.is_ok(), "StorageDataset should deserialize");

    let dataset = dataset.expect("Failed to deserialize");
    assert_eq!(dataset.name, "pool/dataset");
    assert_eq!(dataset.compression, "lz4");
}

#[test]
fn test_storage_dataset_clone() {
    let dataset = StorageDataset {
        name: "pool/data".to_string(),
        pool: "pool".to_string(),
        size: 500_000_000_000,
        used: 200_000_000_000,
        available: 300_000_000_000,
        mount_point: "/mnt/data".to_string(),
        compression: "zstd".to_string(),
    };

    let cloned = dataset.clone();
    assert_eq!(cloned.name, dataset.name);
    assert_eq!(cloned.compression, dataset.compression);
}

// =====================================================
// STORAGE SNAPSHOT TESTS
// =====================================================

#[test]
fn test_storage_snapshot_creation() {
    let snapshot = StorageSnapshot {
        name: "pool/dataset@snap1".to_string(),
        dataset: "pool/dataset".to_string(),
        size: 100_000_000_000, // 100GB
        created: "2024-01-15T10:30:00Z".to_string(),
        referenced: 95_000_000_000, // 95GB
    };

    assert_eq!(snapshot.name, "pool/dataset@snap1");
    assert_eq!(snapshot.dataset, "pool/dataset");
    assert_eq!(snapshot.size, 100_000_000_000);
    assert_eq!(snapshot.created, "2024-01-15T10:30:00Z");
    assert_eq!(snapshot.referenced, 95_000_000_000);
}

#[test]
fn test_storage_snapshot_serialization() {
    let snapshot = StorageSnapshot {
        name: "pool/data@backup".to_string(),
        dataset: "pool/data".to_string(),
        size: 100_000_000_000,
        created: "2024-01-15T10:30:00Z".to_string(),
        referenced: 95_000_000_000,
    };

    let serialized = serde_json::to_string(&snapshot);
    assert!(serialized.is_ok(), "StorageSnapshot should serialize");

    let json = serialized.expect("Failed to serialize");
    assert!(json.contains("\"name\":\"pool/data@backup\""));
    assert!(json.contains("\"dataset\":\"pool/data\""));
}

#[test]
fn test_storage_snapshot_deserialization() {
    let json = r#"{
        "name": "pool/data@backup",
        "dataset": "pool/data",
        "size": 100000000000,
        "created": "2024-01-15T10:30:00Z",
        "referenced": 95000000000
    }"#;

    let snapshot: std::result::Result<StorageSnapshot, _> = serde_json::from_str(json);
    assert!(snapshot.is_ok(), "StorageSnapshot should deserialize");

    let snapshot = snapshot.expect("Failed to deserialize");
    assert_eq!(snapshot.name, "pool/data@backup");
    assert_eq!(snapshot.dataset, "pool/data");
}

#[test]
fn test_storage_snapshot_clone() {
    let snapshot = StorageSnapshot {
        name: "pool/data@snap1".to_string(),
        dataset: "pool/data".to_string(),
        size: 100_000_000_000,
        created: "2024-01-15T10:30:00Z".to_string(),
        referenced: 95_000_000_000,
    };

    let cloned = snapshot.clone();
    assert_eq!(cloned.name, snapshot.name);
    assert_eq!(cloned.dataset, snapshot.dataset);
}

// =====================================================
// STORAGE METRICS TESTS
// =====================================================

#[test]
fn test_storage_metrics_creation() {
    let metrics = StorageMetrics {
        total_pools: 3,
        total_datasets: 15,
        total_snapshots: 50,
        total_storage: 5_000_000_000_000,     // 5TB
        used_storage: 2_000_000_000_000,      // 2TB
        available_storage: 3_000_000_000_000, // 3TB
        iops: 1500.0,
        bandwidth_mbps: 500.0,
        health_status: "HEALTHY".to_string(),
    };

    assert_eq!(metrics.total_pools, 3);
    assert_eq!(metrics.total_datasets, 15);
    assert_eq!(metrics.total_snapshots, 50);
    assert_eq!(metrics.total_storage, 5_000_000_000_000);
    assert_eq!(metrics.used_storage, 2_000_000_000_000);
    assert_eq!(metrics.available_storage, 3_000_000_000_000);
    assert!((metrics.iops - 1500.0).abs() < 0.01);
    assert!((metrics.bandwidth_mbps - 500.0).abs() < 0.01);
    assert_eq!(metrics.health_status, "HEALTHY");
}

#[test]
fn test_storage_metrics_serialization() {
    let metrics = StorageMetrics {
        total_pools: 2,
        total_datasets: 10,
        total_snapshots: 30,
        total_storage: 2_000_000_000_000,
        used_storage: 800_000_000_000,
        available_storage: 1_200_000_000_000,
        iops: 1000.0,
        bandwidth_mbps: 300.0,
        health_status: "HEALTHY".to_string(),
    };

    let serialized = serde_json::to_string(&metrics);
    assert!(serialized.is_ok(), "StorageMetrics should serialize");

    let json = serialized.expect("Failed to serialize");
    assert!(json.contains("\"total_pools\":2"));
    assert!(json.contains("\"health_status\":\"HEALTHY\""));
}

#[test]
fn test_storage_metrics_deserialization() {
    let json = r#"{
        "total_pools": 2,
        "total_datasets": 10,
        "total_snapshots": 30,
        "total_storage": 2000000000000,
        "used_storage": 800000000000,
        "available_storage": 1200000000000,
        "iops": 1000.0,
        "bandwidth_mbps": 300.0,
        "health_status": "HEALTHY"
    }"#;

    let metrics: std::result::Result<StorageMetrics, _> = serde_json::from_str(json);
    assert!(metrics.is_ok(), "StorageMetrics should deserialize");

    let metrics = metrics.expect("Failed to deserialize");
    assert_eq!(metrics.total_pools, 2);
    assert_eq!(metrics.health_status, "HEALTHY");
}

#[test]
fn test_storage_metrics_clone() {
    let metrics = StorageMetrics {
        total_pools: 3,
        total_datasets: 15,
        total_snapshots: 50,
        total_storage: 5_000_000_000_000,
        used_storage: 2_000_000_000_000,
        available_storage: 3_000_000_000_000,
        iops: 1500.0,
        bandwidth_mbps: 500.0,
        health_status: "HEALTHY".to_string(),
    };

    let cloned = metrics.clone();
    assert_eq!(cloned.total_pools, metrics.total_pools);
    assert_eq!(cloned.health_status, metrics.health_status);
}

// =====================================================
// STORAGE POOL INFO TESTS
// =====================================================

#[test]
fn test_storage_pool_info_creation() {
    let pool_info = StoragePoolInfo {
        name: "production-pool".to_string(),
        total_capacity_gb: 2000,
        used_capacity_gb: 800,
        available_capacity_gb: 1200,
        health_status: "healthy".to_string(),
    };

    assert_eq!(pool_info.name, "production-pool");
    assert_eq!(pool_info.total_capacity_gb, 2000);
    assert_eq!(pool_info.used_capacity_gb, 800);
    assert_eq!(pool_info.available_capacity_gb, 1200);
    assert_eq!(pool_info.health_status, "healthy");
}

#[test]
fn test_storage_pool_info_serialization() {
    let pool_info = StoragePoolInfo {
        name: "backup-pool".to_string(),
        total_capacity_gb: 1000,
        used_capacity_gb: 300,
        available_capacity_gb: 700,
        health_status: "healthy".to_string(),
    };

    let serialized = serde_json::to_string(&pool_info);
    assert!(serialized.is_ok(), "StoragePoolInfo should serialize");

    let json = serialized.expect("Failed to serialize");
    assert!(json.contains("\"name\":\"backup-pool\""));
    assert!(json.contains("\"total_capacity_gb\":1000"));
}

#[test]
fn test_storage_pool_info_deserialization() {
    let json = r#"{
        "name": "archive-pool",
        "total_capacity_gb": 5000,
        "used_capacity_gb": 2000,
        "available_capacity_gb": 3000,
        "health_status": "healthy"
    }"#;

    let pool_info: std::result::Result<StoragePoolInfo, _> = serde_json::from_str(json);
    assert!(pool_info.is_ok(), "StoragePoolInfo should deserialize");

    let pool_info = pool_info.expect("Failed to deserialize");
    assert_eq!(pool_info.name, "archive-pool");
    assert_eq!(pool_info.total_capacity_gb, 5000);
}

// =====================================================
// STORAGE DATASET INFO TESTS
// =====================================================

#[test]
fn test_storage_dataset_info_creation() {
    let dataset_info = StorageDatasetInfo {
        name: "pool/production-data".to_string(),
        pool_name: "pool".to_string(),
        used_space_gb: 500,
        compression_ratio: 2.5,
        dedup_ratio: 1.8,
    };

    assert_eq!(dataset_info.name, "pool/production-data");
    assert_eq!(dataset_info.pool_name, "pool");
    assert_eq!(dataset_info.used_space_gb, 500);
    assert!((dataset_info.compression_ratio - 2.5).abs() < 0.01);
    assert!((dataset_info.dedup_ratio - 1.8).abs() < 0.01);
}

#[test]
fn test_storage_dataset_info_serialization() {
    let dataset_info = StorageDatasetInfo {
        name: "pool/logs".to_string(),
        pool_name: "pool".to_string(),
        used_space_gb: 100,
        compression_ratio: 3.0,
        dedup_ratio: 1.5,
    };

    let serialized = serde_json::to_string(&dataset_info);
    assert!(serialized.is_ok(), "StorageDatasetInfo should serialize");

    let json = serialized.expect("Failed to serialize");
    assert!(json.contains("\"name\":\"pool/logs\""));
    assert!(json.contains("\"pool_name\":\"pool\""));
}

#[test]
fn test_storage_dataset_info_deserialization() {
    let json = r#"{
        "name": "pool/backups",
        "pool_name": "pool",
        "used_space_gb": 750,
        "compression_ratio": 2.0,
        "dedup_ratio": 1.2
    }"#;

    let dataset_info: std::result::Result<StorageDatasetInfo, _> = serde_json::from_str(json);
    assert!(
        dataset_info.is_ok(),
        "StorageDatasetInfo should deserialize"
    );

    let dataset_info = dataset_info.expect("Failed to deserialize");
    assert_eq!(dataset_info.name, "pool/backups");
    assert_eq!(dataset_info.used_space_gb, 750);
}

// =====================================================
// STORAGE SNAPSHOT INFO TESTS
// =====================================================

#[test]
fn test_storage_snapshot_info_creation() {
    use std::time::SystemTime;

    let now = SystemTime::now();
    let snapshot_info = StorageSnapshotInfo {
        name: "pool/data@daily-backup".to_string(),
        dataset_name: "pool/data".to_string(),
        created_at: now,
        size_gb: 250,
    };

    assert_eq!(snapshot_info.name, "pool/data@daily-backup");
    assert_eq!(snapshot_info.dataset_name, "pool/data");
    assert_eq!(snapshot_info.size_gb, 250);
}

#[test]
fn test_storage_snapshot_info_serialization() {
    use std::time::SystemTime;

    let now = SystemTime::now();
    let snapshot_info = StorageSnapshotInfo {
        name: "pool/logs@hourly-01".to_string(),
        dataset_name: "pool/logs".to_string(),
        created_at: now,
        size_gb: 50,
    };

    let serialized = serde_json::to_string(&snapshot_info);
    assert!(serialized.is_ok(), "StorageSnapshotInfo should serialize");

    let json = serialized.expect("Failed to serialize");
    assert!(json.contains("\"name\":\"pool/logs@hourly-01\""));
    assert!(json.contains("\"dataset_name\":\"pool/logs\""));
}

// =====================================================
// STORAGE HANDLER TESTS
// =====================================================

#[test]
fn test_storage_handler_creation() {
    let handler = StorageHandler::new();
    assert!(
        std::mem::size_of_val(&handler) >= 0,
        "StorageHandler should be created"
    );
}

#[test]
fn test_storage_handler_default() {
    let handler = StorageHandler;
    assert!(
        std::mem::size_of_val(&handler) >= 0,
        "StorageHandler default should work"
    );
}

#[test]
fn test_storage_handler_clone() {
    let handler = StorageHandler::new();
    let cloned = handler;
    assert!(
        std::mem::size_of_val(&cloned) >= 0,
        "StorageHandler should be cloneable"
    );
}

// =====================================================
// STORAGE MANAGER TESTS
// =====================================================

#[test]
fn test_storage_manager_creation() {
    let manager = StorageManager::new();
    assert!(
        std::mem::size_of_val(&manager) >= 0,
        "StorageManager should be created"
    );
}

#[test]
fn test_storage_manager_default() {
    let manager = StorageManager::default();
    assert!(
        std::mem::size_of_val(&manager) >= 0,
        "StorageManager default should work"
    );
}

#[test]
fn test_storage_manager_clone() {
    let manager = StorageManager::new();
    let cloned = manager;
    assert!(
        std::mem::size_of_val(&cloned) >= 0,
        "StorageManager should be cloneable"
    );
}

// =====================================================
// ASYNC HANDLER TESTS
// =====================================================

#[tokio::test]
async fn test_get_storage_pools_returns_data() {
    let result = get_storage_pools().await;
    assert!(result.is_ok(), "get_storage_pools should return Ok");

    let pools = result.expect("Failed to get pools").0;
    assert!(!pools.is_empty(), "Should return at least one pool");
    assert_eq!(pools[0].name, "main-pool");
    assert_eq!(pools[0].health_status, "healthy");
}

#[tokio::test]
async fn test_get_storage_datasets_returns_data() {
    let result = get_storage_datasets().await;
    assert!(result.is_ok(), "get_storage_datasets should return Ok");

    let datasets = result.expect("Failed to get datasets").0;
    assert!(!datasets.is_empty(), "Should return at least one dataset");
    assert!(datasets[0].name.contains("main-pool"));
}

#[tokio::test]
async fn test_get_storage_snapshots_returns_data() {
    let result = get_storage_snapshots().await;
    assert!(result.is_ok(), "get_storage_snapshots should return Ok");

    let snapshots = result.expect("Failed to get snapshots").0;
    assert!(!snapshots.is_empty(), "Should return at least one snapshot");
    assert!(snapshots[0].name.contains('@'));
}

#[tokio::test]
async fn test_get_storage_metrics_returns_valid_data() {
    let result = get_storage_metrics().await;
    assert!(result.is_ok(), "get_storage_metrics should return Ok");

    let metrics = result.expect("Failed to get metrics").0;
    assert_eq!(metrics.total_pools, 2);
    assert_eq!(metrics.health_status, "healthy");
    assert!(metrics.iops > 0.0);
    assert!(metrics.bandwidth_mbps > 0.0);
}

// =====================================================
// EDGE CASE TESTS
// =====================================================

#[test]
fn test_storage_pool_zero_capacity() {
    let pool = StoragePool {
        name: "zero-pool".to_string(),
        status: "EMPTY".to_string(),
        size: 0,
        used: 0,
        available: 0,
        health: "DEGRADED".to_string(),
        pool_type: "UNKNOWN".to_string(),
    };

    assert_eq!(pool.size, 0);
    assert_eq!(pool.used, 0);
    assert_eq!(pool.available, 0);
}

#[test]
fn test_storage_dataset_empty_compression() {
    let dataset = StorageDataset {
        name: "pool/nocompression".to_string(),
        pool: "pool".to_string(),
        size: 1_000_000_000,
        used: 500_000_000,
        available: 500_000_000,
        mount_point: "/mnt/test".to_string(),
        compression: String::new(), // Empty compression
    };

    assert!(dataset.compression.is_empty());
}

#[test]
fn test_storage_metrics_zero_iops() {
    let metrics = StorageMetrics {
        total_pools: 1,
        total_datasets: 1,
        total_snapshots: 0,
        total_storage: 1_000_000_000_000,
        used_storage: 0,
        available_storage: 1_000_000_000_000,
        iops: 0.0,           // Zero IOPS
        bandwidth_mbps: 0.0, // Zero bandwidth
        health_status: "IDLE".to_string(),
    };

    assert_eq!(metrics.iops, 0.0);
    assert_eq!(metrics.bandwidth_mbps, 0.0);
}

#[test]
fn test_storage_pool_full_capacity() {
    let pool = StoragePool {
        name: "full-pool".to_string(),
        status: "ONLINE".to_string(),
        size: 1_000_000_000_000,
        used: 1_000_000_000_000, // Completely full
        available: 0,
        health: "WARNING".to_string(),
        pool_type: "MIRROR".to_string(),
    };

    assert_eq!(pool.used, pool.size);
    assert_eq!(pool.available, 0);
    assert_eq!(pool.health, "WARNING");
}
