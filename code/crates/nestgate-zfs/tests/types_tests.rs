// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
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
// Types tests moved from src/types.rs to comply with 1000-line limit

//! Types Tests module

use nestgate_zfs::types::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

mod tests {
    use super::*;

    #[test]
    fn test_pool_info_creation() {
        let pool_info = PoolInfo::default();
        assert_eq!(pool_info.name, "");
        assert_eq!(pool_info.size, 0);
        assert!(matches!(pool_info.health, PoolHealth::Unknown));
    }

    #[test]
    fn test_dataset_info_creation() {
        let dataset_info = DatasetInfo::default();
        assert_eq!(dataset_info.compression, "lz4");
        assert_eq!(dataset_info.checksum, "sha256");
        assert!(matches!(dataset_info.tier, StorageTier::Warm));
    }

    #[test]
    fn test_pool_status_conversion() {
        let health: PoolHealth = PoolStatus::Online.into();
        assert!(matches!(health, PoolHealth::Healthy));

        let state: PoolState = PoolStatus::Degraded.into();
        assert!(matches!(state, PoolState::Degraded));
    }

    #[test]
    fn test_zfs_error_conversion() {
        let zfs_err = ZfsError::PoolError {
            message: "Pool creation failed".to_string(),
        };
        let nestgate_err: nestgate_core::NestGateError =
            nestgate_core::NestGateError::from(format!("{:?}", zfs_err));
        assert!(nestgate_err.to_string().contains("PoolError"));
    }

    #[test]
    fn test_pool_capacity_default() {
        let capacity = PoolCapacity::default();
        assert_eq!(capacity.total_bytes, 0);
        assert_eq!(capacity.used_bytes, 0);
        assert_eq!(capacity.available_bytes, 0);
        assert_eq!(capacity.fragmentation_percent, 0.0);
        assert_eq!(capacity.deduplication_ratio, 1.0);
    }

    #[test]
    fn test_pool_capacity_with_data() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
        };
        assert_eq!(capacity.total_bytes, 1_000_000_000);
        assert_eq!(
            capacity.used_bytes + capacity.available_bytes,
            capacity.total_bytes
        );
        assert!(capacity.fragmentation_percent >= 0.0);
        assert!(capacity.deduplication_ratio >= 1.0);
    }

    #[test]
    fn test_snapshot_info_default() {
        let snapshot = SnapshotInfo::default();
        assert!(snapshot.name.is_empty());
        assert!(snapshot.dataset.is_empty());
        assert_eq!(snapshot.size, 0);
        assert!(snapshot.properties.is_empty());
    }

    #[test]
    fn test_snapshot_info_with_data() {
        let mut properties = HashMap::new();
        properties.insert("creation".to_string(), "123456789".to_string());

        let snapshot = SnapshotInfo {
            name: "snap1".to_string(),
            dataset: "pool/dataset".to_string(),
            created_at: SystemTime::now(),
            used: 0,
            size: 1024 * 1024,
            referenced: 1024 * 1024,
            properties: properties.clone(),
        };

        assert_eq!(snapshot.name, "snap1");
        assert_eq!(snapshot.dataset, "pool/dataset");
        assert_eq!(snapshot.size, 1024 * 1024);
        assert_eq!(snapshot.properties.len(), 1);
    }

    #[test]
    fn test_bottleneck_report_creation() {
        let report = BottleneckReport {
            dataset: "pool/data".to_string(),
            bottleneck_type: "IOPS".to_string(),
            severity: "HIGH".to_string(),
            recommendations: vec!["Add more disks".to_string()],
        };

        assert_eq!(report.dataset, "pool/data");
        assert_eq!(report.bottleneck_type, "IOPS");
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_capacity_report_creation() {
        let report = CapacityReport {
            dataset: "pool/data".to_string(),
            current_usage: 8_000_000_000,
            projected_usage: 10_000_000_000,
            recommendations: vec!["Expand storage".to_string()],
        };

        assert!(report.projected_usage > report.current_usage);
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_retention_policy_creation() {
        let policy = RetentionPolicy {
            name: "default".to_string(),
            keep_hourly: 24,
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 12,
        };

        assert_eq!(policy.name, "default");
        assert!(policy.keep_hourly > 0);
        assert!(policy.keep_daily > 0);
    }

    #[test]
    fn test_command_result_success() {
        let result = CommandResult {
            success: true,
            stdout: "Pool created successfully".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
        };

        assert!(result.success);
        assert_eq!(result.exit_code, Some(0));
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn test_command_result_failure() {
        let result = CommandResult {
            success: false,
            stdout: String::new(),
            stderr: "Pool not found".to_string(),
            exit_code: Some(1),
        };

        assert!(!result.success);
        assert_eq!(result.exit_code, Some(1));
        assert!(!result.stderr.is_empty());
    }

    #[test]
    fn test_pool_status_all_variants() {
        let statuses = [
            PoolStatus::Online,
            PoolStatus::Degraded,
            PoolStatus::Faulted,
            PoolStatus::Offline,
            PoolStatus::Removed,
            PoolStatus::Unavailable,
        ];

        assert_eq!(statuses.len(), 6);
    }

    #[test]
    fn test_pool_health_from_status() {
        assert!(matches!(
            PoolHealth::from(PoolStatus::Online),
            PoolHealth::Healthy
        ));
        assert!(matches!(
            PoolHealth::from(PoolStatus::Degraded),
            PoolHealth::Warning
        ));
        assert!(matches!(
            PoolHealth::from(PoolStatus::Faulted),
            PoolHealth::Critical
        ));
        assert!(matches!(
            PoolHealth::from(PoolStatus::Offline),
            PoolHealth::Unknown
        ));
    }

    #[test]
    fn test_pool_state_from_status() {
        assert!(matches!(
            PoolState::from(PoolStatus::Online),
            PoolState::Online
        ));
        assert!(matches!(
            PoolState::from(PoolStatus::Degraded),
            PoolState::Degraded
        ));
        assert!(matches!(
            PoolState::from(PoolStatus::Faulted),
            PoolState::Faulted
        ));
        assert!(matches!(
            PoolState::from(PoolStatus::Removed),
            PoolState::Removed
        ));
    }

    #[test]
    fn test_pool_info_from_zfs_output() {
        let output = "size\t1000000000\nallocated\t500000000\nhealth\tONLINE\n";
        let result = pool_info_from_zfs_output("testpool", output);

        assert!(result.is_ok());
        let pool = result.expect("ZFS operation failed");
        assert_eq!(pool.name, "testpool");
        assert_eq!(pool.size, 1_000_000_000);
        assert_eq!(pool.used, 500_000_000);
        assert!(matches!(pool.health, PoolHealth::Healthy));
    }

    #[test]
    fn test_pool_info_from_zfs_output_degraded() {
        let output = "size\t2000000000\nallocated\t1000000000\nhealth\tDEGRADED\n";
        let result = pool_info_from_zfs_output("degraded_pool", output);

        assert!(result.is_ok());
        let pool = result.expect("ZFS operation failed");
        assert_eq!(pool.name, "degraded_pool");
        assert!(matches!(pool.health, PoolHealth::Warning));
        assert!(matches!(pool.state, PoolState::Degraded));
    }

    #[test]
    fn test_dataset_info_from_zfs_output() {
        let output = "name\tpool/dataset\nused\t1024\navailable\t2048\ncompression\tlz4\n";
        let result = dataset_info_from_zfs_output(output);

        assert!(result.is_ok());
        let dataset = result.expect("ZFS operation failed");
        assert_eq!(dataset.full_name, "pool/dataset");
        assert_eq!(dataset.pool, "pool");
        assert_eq!(dataset.compression, "lz4");
    }

    #[test]
    fn test_dataset_info_mount_point_parsing() {
        let output = "name\tpool/data\nused\t1024\navailable\t2048\nmountpoint\t/mnt/data\n";
        let result = dataset_info_from_zfs_output(output);

        assert!(result.is_ok());
        let dataset = result.expect("ZFS operation failed");
        assert!(dataset.mount_point.is_some());
        assert_eq!(
            dataset.mount_point.expect("ZFS operation failed"),
            PathBuf::from("/mnt/data")
        );
    }

    #[test]
    fn test_dataset_info_no_mount_point() {
        let output = "name\tpool/data\nused\t1024\navailable\t2048\nmountpoint\tnone\n";
        let result = dataset_info_from_zfs_output(output);

        assert!(result.is_ok());
        let dataset = result.expect("ZFS operation failed");
        assert!(dataset.mount_point.is_none());
    }

    #[test]
    fn test_zfs_error_types() {
        let pool_err = ZfsError::PoolError {
            message: "Pool failed".to_string(),
        };
        assert!(pool_err.to_string().contains("Pool operation failed"));

        let dataset_err = ZfsError::DatasetError {
            message: "Dataset failed".to_string(),
        };
        assert!(dataset_err.to_string().contains("Dataset operation failed"));

        let snapshot_err = ZfsError::SnapshotError {
            message: "Snapshot failed".to_string(),
        };
        assert!(
            snapshot_err
                .to_string()
                .contains("Snapshot operation failed")
        );
    }

    #[test]
    fn test_zero_cost_types_creation() {
        let pool = ZeroCostPoolInfo {
            name: "fast-pool".to_string(),
            size: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
            health: "ONLINE".to_string(),
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        };

        assert_eq!(pool.name, "fast-pool");
        assert_eq!(pool.size, 1_000_000_000);
    }

    #[test]
    fn test_replication_performance() {
        let perf = ReplicationPerformance {
            source_dataset: "pool1/data".to_string(),
            target_dataset: "pool2/data".to_string(),
            transfer_rate: 100.5,
            compression_ratio: 1.5,
            estimated_completion: SystemTime::now(),
        };

        assert_eq!(perf.source_dataset, "pool1/data");
        assert!(perf.transfer_rate > 0.0);
        assert!(perf.compression_ratio >= 1.0);
    }

    #[test]
    fn test_system_info_metrics() {
        let info = SystemInfo {
            timestamp: SystemTime::now(),
            cpu_usage: 45.5,
            memory_usage: 60.0,
            disk_usage: 75.0,
        };

        assert!(info.cpu_usage >= 0.0 && info.cpu_usage <= 100.0);
        assert!(info.memory_usage >= 0.0 && info.memory_usage <= 100.0);
        assert!(info.disk_usage >= 0.0 && info.disk_usage <= 100.0);
    }

    #[test]
    fn test_bottleneck_report_with_recommendations() {
        let report = BottleneckReport {
            dataset: "pool/data".to_string(),
            bottleneck_type: "io".to_string(),
            severity: "high".to_string(),
            recommendations: vec!["Add more IOPS".to_string(), "Enable caching".to_string()],
        };

        assert_eq!(report.dataset, "pool/data");
        assert_eq!(report.bottleneck_type, "io");
        assert_eq!(report.severity, "high");
        assert_eq!(report.recommendations.len(), 2);
    }

    #[test]
    fn test_capacity_report_with_projections() {
        let report = CapacityReport {
            dataset: "pool/archive".to_string(),
            current_usage: 5_000_000_000,
            projected_usage: 10_000_000_000,
            recommendations: vec!["Expand capacity".to_string()],
        };

        assert_eq!(report.dataset, "pool/archive");
        assert!(report.projected_usage > report.current_usage);
        assert!(!report.recommendations.is_empty());
    }

    #[test]
    fn test_maintenance_schedule() {
        let now = SystemTime::now();
        let schedule = MaintenanceSchedule {
            dataset: "pool/critical".to_string(),
            next_maintenance: now,
            tasks: vec!["Scrub".to_string(), "Backup".to_string()],
        };

        assert_eq!(schedule.dataset, "pool/critical");
        assert_eq!(schedule.tasks.len(), 2);
        assert!(schedule.tasks.contains(&"Scrub".to_string()));
    }

    #[test]
    fn test_retention_policy_defaults() {
        let policy = RetentionPolicy {
            name: "standard".to_string(),
            keep_hourly: 24,
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 12,
        };

        assert_eq!(policy.name, "standard");
        assert!(policy.keep_hourly > 0);
        assert!(policy.keep_daily > 0);
        assert!(policy.keep_weekly > 0);
        assert!(policy.keep_monthly > 0);
    }

    #[test]
    fn test_retention_policy_aggressive() {
        let policy = RetentionPolicy {
            name: "aggressive".to_string(),
            keep_hourly: 1,
            keep_daily: 1,
            keep_weekly: 0,
            keep_monthly: 0,
        };

        assert_eq!(policy.name, "aggressive");
        assert_eq!(policy.keep_weekly, 0);
        assert_eq!(policy.keep_monthly, 0);
    }

    #[test]
    fn test_pool_info_capacity_calculation() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 600_000_000,
            available_bytes: 400_000_000,
            utilization_percent: 60.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 600_000_000,
            available: 400_000_000,
        };

        let pool = PoolInfo {
            name: "test-pool".to_string(),
            size: 1_000_000_000,
            used: 600_000_000,
            available: 400_000_000,
            health: PoolHealth::Healthy,
            state: PoolState::Online,
            capacity,
            properties: HashMap::new(),
            created_at: SystemTime::now(),
        };

        assert_eq!(pool.used + pool.available, pool.size);
        assert!(pool.used <= pool.size);
        assert!(pool.available <= pool.size);
    }

    #[test]
    fn test_pool_health_variants() {
        let healthy = PoolHealth::Healthy;
        let warning = PoolHealth::Warning;
        let critical = PoolHealth::Critical;
        let unknown = PoolHealth::Unknown;

        // Test that all variants can be created
        assert!(matches!(healthy, PoolHealth::Healthy));
        assert!(matches!(warning, PoolHealth::Warning));
        assert!(matches!(critical, PoolHealth::Critical));
        assert!(matches!(unknown, PoolHealth::Unknown));
    }

    #[test]
    fn test_pool_state_variants() {
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
    fn test_replication_performance_transfer_rate() {
        let perf = ReplicationPerformance {
            source_dataset: "source/data".to_string(),
            target_dataset: "target/data".to_string(),
            transfer_rate: 1024.0, // MB/s
            compression_ratio: 2.0,
            estimated_completion: SystemTime::now(),
        };

        assert!(perf.transfer_rate > 0.0);
        assert!(perf.compression_ratio >= 1.0);
        assert!(perf.compression_ratio <= 10.0); // Realistic compression ratio
    }

    #[test]
    fn test_system_info_zero_usage() {
        let info = SystemInfo {
            timestamp: SystemTime::now(),
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
        };

        assert_eq!(info.cpu_usage, 0.0);
        assert_eq!(info.memory_usage, 0.0);
        assert_eq!(info.disk_usage, 0.0);
    }

    #[test]
    fn test_system_info_full_usage() {
        let info = SystemInfo {
            timestamp: SystemTime::now(),
            cpu_usage: 100.0,
            memory_usage: 100.0,
            disk_usage: 100.0,
        };

        assert_eq!(info.cpu_usage, 100.0);
        assert_eq!(info.memory_usage, 100.0);
        assert_eq!(info.disk_usage, 100.0);
    }

    #[test]
    fn test_bottleneck_report_empty_recommendations() {
        let report = BottleneckReport {
            dataset: "pool/ok".to_string(),
            bottleneck_type: "none".to_string(),
            severity: "low".to_string(),
            recommendations: vec![],
        };

        assert!(report.recommendations.is_empty());
        assert_eq!(report.severity, "low");
    }

    #[test]
    fn test_capacity_report_equal_usage() {
        let report = CapacityReport {
            dataset: "pool/stable".to_string(),
            current_usage: 5_000_000_000,
            projected_usage: 5_000_000_000,
            recommendations: vec![],
        };

        assert_eq!(report.current_usage, report.projected_usage);
    }

    #[test]
    fn test_maintenance_schedule_empty_tasks() {
        let schedule = MaintenanceSchedule {
            dataset: "pool/new".to_string(),
            next_maintenance: SystemTime::now(),
            tasks: vec![],
        };

        assert!(schedule.tasks.is_empty());
    }

    #[test]
    fn test_retention_policy_zero_retention() {
        let policy = RetentionPolicy {
            name: "ephemeral".to_string(),
            keep_hourly: 0,
            keep_daily: 0,
            keep_weekly: 0,
            keep_monthly: 0,
        };

        assert_eq!(
            policy.keep_hourly + policy.keep_daily + policy.keep_weekly + policy.keep_monthly,
            0
        );
    }

    #[test]
    fn test_pool_info_with_properties() {
        let mut props = HashMap::new();
        props.insert("compression".to_string(), "lz4".to_string());
        props.insert("atime".to_string(), "off".to_string());

        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 0,
            available_bytes: 1_000_000_000,
            utilization_percent: 0.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 0,
            available: 1_000_000_000,
        };

        let pool = PoolInfo {
            name: "fast-pool".to_string(),
            size: 1_000_000_000,
            used: 0,
            available: 1_000_000_000,
            health: PoolHealth::Healthy,
            state: PoolState::Online,
            capacity,
            properties: props.clone(),
            created_at: SystemTime::now(),
        };

        assert_eq!(pool.properties.len(), 2);
        assert_eq!(pool.properties.get("compression"), Some(&"lz4".to_string()));
        assert_eq!(pool.properties.get("atime"), Some(&"off".to_string()));
    }

    #[test]
    fn test_pool_capacity_fragmentation() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 750_000_000,
            available_bytes: 250_000_000,
            utilization_percent: 75.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 750_000_000,
            available: 250_000_000,
        };

        assert_eq!(capacity.total_bytes, 1_000_000_000);
        assert_eq!(capacity.used_bytes, 750_000_000);
        assert!(capacity.utilization_percent > 0.0);
    }

    #[test]
    fn test_pool_capacity_deduplication() {
        let capacity = PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 3.0, // High dedup ratio for space savings
            total: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
        };

        assert_eq!(capacity.deduplication_ratio, 3.0);
    }

    #[test]
    fn test_replication_performance_high_compression() {
        let perf = ReplicationPerformance {
            source_dataset: "source/compress".to_string(),
            target_dataset: "target/compress".to_string(),
            transfer_rate: 500.0,
            compression_ratio: 5.0, // High compression
            estimated_completion: SystemTime::now(),
        };

        assert_eq!(perf.compression_ratio, 5.0);
    }

    #[test]
    fn test_zfs_error_pool_error() {
        let err = ZfsError::PoolError {
            message: "Pool import failed".to_string(),
        };

        let error_string = err.to_string();
        assert!(error_string.contains("Pool operation failed"));
        assert!(error_string.contains("Pool import failed"));
    }

    #[test]
    fn test_zfs_error_dataset_error() {
        let err = ZfsError::DatasetError {
            message: "Dataset creation failed".to_string(),
        };

        assert!(err.to_string().contains("Dataset operation failed"));
    }

    #[test]
    fn test_zfs_error_command_error() {
        let err = ZfsError::CommandError {
            message: "Command execution timeout".to_string(),
        };

        assert!(err.to_string().contains("Command execution failed"));
    }
}
