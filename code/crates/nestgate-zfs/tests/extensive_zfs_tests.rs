// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    dead_code,
    unused_doc_comments,
    unused_imports,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    deprecated,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
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
    clippy::cast_lossless,
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
    clippy::manual_midpoint,
    clippy::suboptimal_flops,
    clippy::items_after_statements,
    clippy::items_after_test_module,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::unreadable_literal,
    clippy::redundant_clone,
    clippy::useless_vec,
    clippy::field_reassign_with_default,
    clippy::cmp_null,
    clippy::bool_assert_comparison,
    clippy::used_underscore_items,
    clippy::needless_raw_string_hashes,
    clippy::ref_as_ptr,
    clippy::no_effect_underscore_binding,
    clippy::needless_collect,
    clippy::module_inception,
    clippy::default_trait_access,
    clippy::wildcard_in_or_patterns,
    clippy::or_fun_call,
    clippy::manual_string_new,
    clippy::unnecessary_literal_unwrap,
    clippy::unnecessary_debug_formatting,
    clippy::assigning_clones,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_map_or,
    clippy::unnecessary_lazy_evaluations,
    clippy::similar_names,
    clippy::needless_continue,
    clippy::collection_is_never_read,
    clippy::char_lit_as_u8,
    clippy::ptr_eq,
    clippy::uninlined_format_args,
    clippy::absurd_extreme_comparisons,
    clippy::match_wild_err_arm,
    clippy::single_match_else,
    clippy::derive_partial_eq_without_eq,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_const_for_fn,
    clippy::used_underscore_binding,
    clippy::ignored_unit_patterns,
    unused_comparisons,
    clippy::format_push_string
)]

//! Extensive unit tests for ZFS functionality
//! Targeting improved coverage for nestgate-zfs crate

use nestgate_zfs::ZfsPoolManager;
use nestgate_zfs::config::ZfsConfig;
use nestgate_zfs::error::ZfsError;
use nestgate_zfs::types::*;
use std::collections::HashMap;
use std::time::SystemTime;

// ==================== POOL STATE TESTS ====================

#[test]
fn test_pool_state_online() {
    let state = PoolState::Online;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Online"));
}

#[test]
fn test_pool_state_all_variants() {
    let states = [
        PoolState::Online,
        PoolState::Offline,
        PoolState::Degraded,
        PoolState::Faulted,
        PoolState::Removed,
        PoolState::Unavailable,
    ];

    assert_eq!(states.len(), 6);
}

#[test]
fn test_pool_state_serialization() {
    let state = PoolState::Online;
    let json = serde_json::to_string(&state).expect("Failed to serialize");
    let deserialized: PoolState = serde_json::from_str(&json).expect("Failed to deserialize");

    // Both should be Online variant
    assert!(matches!(deserialized, PoolState::Online));
}

// ==================== POOL HEALTH TESTS ====================

#[test]
fn test_pool_health_variants() {
    let health_states = [
        PoolHealth::Healthy,
        PoolHealth::Warning,
        PoolHealth::Critical,
        PoolHealth::Unknown,
    ];

    assert_eq!(health_states.len(), 4);
}

#[test]
fn test_pool_health_serialization() {
    let health = PoolHealth::Healthy;
    let json = serde_json::to_string(&health).expect("Failed to serialize");
    assert!(!json.is_empty());
}

// ==================== POOL CAPACITY TESTS ====================

#[test]
fn test_pool_capacity_creation() {
    let capacity = PoolCapacity {
        total: 1024 * 1024 * 1024,
        total_bytes: 1024 * 1024 * 1024, // 1GB
        used: 512 * 1024 * 1024,
        used_bytes: 512 * 1024 * 1024, // 512MB
        available: 512 * 1024 * 1024,
        available_bytes: 512 * 1024 * 1024, // 512MB
        utilization_percent: 50.0,
        fragmentation_percent: 5.0,
        deduplication_ratio: 1.2,
    };

    assert_eq!(capacity.total_bytes, 1024 * 1024 * 1024);
    assert_eq!(capacity.used_bytes, 512 * 1024 * 1024);
    assert_eq!(capacity.available_bytes, 512 * 1024 * 1024);
}

#[test]
fn test_pool_capacity_ratios() {
    let capacity = PoolCapacity {
        total: 1000,
        total_bytes: 1000,
        used: 300,
        used_bytes: 300,
        available: 700,
        available_bytes: 700,
        utilization_percent: 30.0,
        fragmentation_percent: 10.0,
        deduplication_ratio: 1.5,
    };

    // Verify deduplication ratio
    assert!(capacity.deduplication_ratio > 1.0);

    // Verify fragmentation
    assert!(capacity.fragmentation_percent >= 0.0);
    assert!(capacity.fragmentation_percent <= 100.0);
}

#[test]
fn test_pool_capacity_zero_fragmentation() {
    let capacity = PoolCapacity {
        total: 1000,
        total_bytes: 1000,
        used: 500,
        used_bytes: 500,
        available: 500,
        available_bytes: 500,
        utilization_percent: 50.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
    };

    assert_eq!(capacity.fragmentation_percent, 0.0);
    assert_eq!(capacity.deduplication_ratio, 1.0);
}

// ==================== POOL INFO TESTS ====================

#[test]
fn test_pool_info_creation() {
    let mut properties = HashMap::new();
    properties.insert("compression".to_string(), "lz4".to_string());

    let capacity = PoolCapacity {
        total: 1024,
        total_bytes: 1024,
        used: 512,
        used_bytes: 512,
        available: 512,
        available_bytes: 512,
        utilization_percent: 50.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
    };

    let pool_info = PoolInfo {
        name: "testpool".to_string(),
        size: 1024,
        used: 512,
        available: 512,
        health: PoolHealth::Healthy,
        state: PoolState::Online,
        capacity,
        properties,
        created_at: SystemTime::now(),
    };

    assert_eq!(pool_info.name, "testpool");
    assert_eq!(pool_info.size, 1024);
}

#[test]
fn test_pool_info_with_properties() {
    let mut properties = HashMap::new();
    properties.insert("compression".to_string(), "lz4".to_string());
    properties.insert("atime".to_string(), "off".to_string());

    let capacity = PoolCapacity {
        total: 1024,
        total_bytes: 1024,
        used: 0,
        used_bytes: 0,
        available: 1024,
        available_bytes: 1024,
        utilization_percent: 0.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
    };

    let pool_info = PoolInfo {
        name: "testpool".to_string(),
        size: 1024,
        used: 0,
        available: 1024,
        health: PoolHealth::Healthy,
        state: PoolState::Online,
        capacity,
        properties: properties.clone(),
        created_at: SystemTime::now(),
    };

    assert_eq!(pool_info.properties.len(), 2);
    assert_eq!(
        pool_info.properties.get("compression"),
        Some(&"lz4".to_string())
    );
}

// ==================== DATASET INFO TESTS ====================

#[test]
fn test_dataset_info_creation() {
    let dataset = DatasetInfo {
        name: "dataset1".to_string(),
        full_name: "pool/dataset1".to_string(),
        pool: "pool".to_string(),
        size: 1024,
        used: 512,
        available: 512,
        mountpoint: Some(std::path::PathBuf::from("/mnt/dataset1")),
        mount_point: Some(std::path::PathBuf::from("/mnt/dataset1")),
        dataset_type: "filesystem".to_string(),
        compression: "lz4".to_string(),
        checksum: "sha256".to_string(),
        referenced: 512,
        compression_ratio: 1.0,
        tier: nestgate_core::canonical_types::StorageTier::Hot,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert_eq!(dataset.name, "dataset1");
    assert_eq!(dataset.full_name, "pool/dataset1");
    assert_eq!(dataset.pool, "pool");
}

#[test]
fn test_dataset_info_without_mount_point() {
    let dataset = DatasetInfo {
        name: "volume1".to_string(),
        full_name: "pool/volume1".to_string(),
        pool: "pool".to_string(),
        size: 1024,
        used: 512,
        available: 512,
        mountpoint: None,
        mount_point: None,
        dataset_type: "volume".to_string(),
        compression: "lz4".to_string(),
        checksum: "sha256".to_string(),
        referenced: 512,
        compression_ratio: 1.0,
        tier: nestgate_core::canonical_types::StorageTier::Cold,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    assert_eq!(dataset.mount_point, None);
}

// ==================== ERROR TESTS ====================

#[test]
fn test_zfs_error_pool_error() {
    let error = ZfsError::PoolError {
        message: "Pool not found".to_string(),
    };

    let error_str = format!("{}", error);
    assert!(error_str.contains("Pool not found"));
}

#[test]
fn test_zfs_error_dataset_error() {
    let error = ZfsError::DatasetError {
        message: "Dataset creation failed".to_string(),
    };

    assert!(format!("{}", error).contains("Dataset creation failed"));
}

#[test]
fn test_zfs_error_snapshot_error() {
    let error = ZfsError::SnapshotError {
        message: "Snapshot already exists".to_string(),
    };

    assert!(format!("{}", error).contains("Snapshot already exists"));
}

#[test]
fn test_zfs_error_command_error() {
    let error = ZfsError::CommandError {
        message: "Command execution timed out".to_string(),
    };

    assert!(format!("{}", error).contains("Command execution timed out"));
}

#[test]
fn test_zfs_error_config_error() {
    let error = ZfsError::ConfigError {
        message: "Invalid configuration".to_string(),
    };

    assert!(format!("{}", error).contains("Invalid configuration"));
}

#[test]
fn test_zfs_error_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let zfs_error = ZfsError::from(io_error);

    assert!(format!("{}", zfs_error).contains("file not found"));
}

// ==================== CONFIG TESTS ====================

#[test]
fn test_zfs_config_default() {
    let config = ZfsConfig::default();
    assert!(!format!("{:?}", config).is_empty());
}

// Health config tests removed - HealthCheckConfig not exported

// ==================== POOL MANAGER TESTS ====================

#[test]
fn test_pool_manager_creation() {
    let config = ZfsConfig::default();
    let _manager = ZfsPoolManager::new_production(config);

    // Manager created successfully - type checked at compile time
    // Test passes if creation succeeds without panic
}

// ==================== CAPACITY MONITORING TYPES TESTS ====================

#[test]
fn test_bottleneck_report_creation() {
    let report = BottleneckReport {
        dataset: "pool/dataset".to_string(),
        bottleneck_type: "io".to_string(),
        severity: "high".to_string(),
        recommendations: vec!["Add more disks".to_string()],
    };

    assert_eq!(report.dataset, "pool/dataset");
    assert_eq!(report.recommendations.len(), 1);
}

#[test]
fn test_capacity_report_creation() {
    let report = CapacityReport {
        dataset: "pool/dataset".to_string(),
        current_usage: 1024,
        projected_usage: 2048,
        recommendations: vec!["Plan for expansion".to_string()],
    };

    assert_eq!(report.current_usage, 1024);
    assert_eq!(report.projected_usage, 2048);
}

#[test]
fn test_retention_policy_creation() {
    let policy = RetentionPolicy {
        name: "standard".to_string(),
        keep_hourly: 24,
        keep_daily: 7,
        keep_weekly: 4,
        keep_monthly: 12,
    };

    assert_eq!(policy.name, "standard");
    assert_eq!(policy.keep_daily, 7);
}

#[test]
fn test_retention_policy_minimal() {
    let policy = RetentionPolicy {
        name: "minimal".to_string(),
        keep_hourly: 0,
        keep_daily: 1,
        keep_weekly: 0,
        keep_monthly: 0,
    };

    assert_eq!(policy.keep_hourly, 0);
    assert_eq!(policy.keep_daily, 1);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_pool_info_serialization() {
    let capacity = PoolCapacity {
        total: 1024,
        total_bytes: 1024,
        used: 512,
        used_bytes: 512,
        available: 512,
        available_bytes: 512,
        utilization_percent: 50.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
    };

    let pool_info = PoolInfo {
        name: "testpool".to_string(),
        size: 1024,
        used: 512,
        available: 512,
        health: PoolHealth::Healthy,
        state: PoolState::Online,
        capacity,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    let json = serde_json::to_string(&pool_info).expect("Failed to serialize");
    assert!(json.contains("testpool"));

    let deserialized: PoolInfo = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(deserialized.name, "testpool");
}

#[test]
fn test_dataset_info_serialization() {
    let dataset = DatasetInfo {
        name: "dataset1".to_string(),
        full_name: "pool/dataset1".to_string(),
        pool: "pool".to_string(),
        size: 1024,
        used: 512,
        available: 512,
        mountpoint: None,
        mount_point: None,
        dataset_type: "filesystem".to_string(),
        compression: "lz4".to_string(),
        checksum: "sha256".to_string(),
        referenced: 512,
        compression_ratio: 1.0,
        tier: nestgate_core::canonical_types::StorageTier::Hot,
        properties: HashMap::new(),
        created_at: SystemTime::now(),
    };

    let json = serde_json::to_string(&dataset).expect("Failed to serialize");
    assert!(json.contains("dataset1"));
}

// ==================== EDGE CASES ====================

#[test]
fn test_pool_capacity_full() {
    let capacity = PoolCapacity {
        total: 1000,
        total_bytes: 1000,
        used: 1000,
        used_bytes: 1000,
        available: 0,
        available_bytes: 0,
        utilization_percent: 100.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
    };

    assert_eq!(capacity.available_bytes, 0);
    assert_eq!(capacity.used_bytes, capacity.total_bytes);
}

#[test]
fn test_pool_capacity_empty() {
    let capacity = PoolCapacity {
        total: 1000,
        total_bytes: 1000,
        used: 0,
        used_bytes: 0,
        available: 1000,
        available_bytes: 1000,
        utilization_percent: 0.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
    };

    assert_eq!(capacity.used_bytes, 0);
    assert_eq!(capacity.available_bytes, capacity.total_bytes);
}

#[test]
fn test_empty_properties_map() {
    let properties: HashMap<String, String> = HashMap::new();
    assert_eq!(properties.len(), 0);
    assert_eq!(properties.get("anything"), None);
}

#[test]
fn test_multiple_properties() {
    let mut properties = HashMap::new();
    properties.insert("compression".to_string(), "lz4".to_string());
    properties.insert("atime".to_string(), "on".to_string());
    properties.insert("recordsize".to_string(), "128K".to_string());
    properties.insert("dedup".to_string(), "off".to_string());

    assert_eq!(properties.len(), 4);
    assert!(properties.contains_key("compression"));
    assert!(properties.contains_key("dedup"));
}

// ==================== STRING OPERATIONS ====================

#[test]
fn test_dataset_name_parsing() {
    let full_name = "pool/dataset/child";
    let parts: Vec<&str> = full_name.split('/').collect();

    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "pool");
    assert_eq!(parts[1], "dataset");
    assert_eq!(parts[2], "child");
}

#[test]
fn test_dataset_name_construction() {
    let pool = "mypool";
    let dataset = "mydataset";
    let full_name = format!("{}/{}", pool, dataset);

    assert_eq!(full_name, "mypool/mydataset");
}
