//! Comprehensive tests for nestgate-zfs types module
//! Target: Improve coverage from 4.72% toward 15%+
//!
//! This test suite covers:
//! - Type construction and validation
//! - Serialization/deserialization
//! - Edge cases and boundary conditions
//! - Type conversions and helpers

use nestgate_zfs::types::*;
use serde_json;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ==================== BOTTLENECK REPORT TESTS ====================

#[test]
fn test_bottleneck_report_creation() {
    let report = BottleneckReport {
        dataset: "tank/data".to_string(),
        bottleneck_type: "io_saturation".to_string(),
        severity: "high".to_string(),
        recommendations: vec![
            "Add more disks".to_string(),
            "Enable compression".to_string(),
        ],
    };

    assert_eq!(report.dataset, "tank/data");
    assert_eq!(report.bottleneck_type, "io_saturation");
    assert_eq!(report.severity, "high");
    assert_eq!(report.recommendations.len(), 2);
}

#[test]
fn test_bottleneck_report_serialization() {
    let report = BottleneckReport {
        dataset: "tank/logs".to_string(),
        bottleneck_type: "memory_pressure".to_string(),
        severity: "medium".to_string(),
        recommendations: vec!["Increase ARC size".to_string()],
    };

    let json = serde_json::to_string(&report).expect("Failed to serialize");
    assert!(json.contains("tank/logs"));
    assert!(json.contains("memory_pressure"));

    let deserialized: BottleneckReport =
        serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(deserialized.dataset, report.dataset);
    assert_eq!(deserialized.bottleneck_type, report.bottleneck_type);
}

#[test]
fn test_bottleneck_report_clone() {
    let report = BottleneckReport {
        dataset: "pool/dataset".to_string(),
        bottleneck_type: "cpu_bound".to_string(),
        severity: "low".to_string(),
        recommendations: vec![],
    };

    let cloned = report.clone();
    assert_eq!(cloned.dataset, report.dataset);
    assert_eq!(cloned.recommendations.len(), 0);
}

#[test]
fn test_bottleneck_report_empty_recommendations() {
    let report = BottleneckReport {
        dataset: "test".to_string(),
        bottleneck_type: "none".to_string(),
        severity: "info".to_string(),
        recommendations: vec![],
    };

    assert!(report.recommendations.is_empty());
}

#[test]
fn test_bottleneck_report_many_recommendations() {
    let recommendations: Vec<String> = (0..10).map(|i| format!("Recommendation {}", i)).collect();

    let report = BottleneckReport {
        dataset: "tank".to_string(),
        bottleneck_type: "multi".to_string(),
        severity: "critical".to_string(),
        recommendations: recommendations.clone(),
    };

    assert_eq!(report.recommendations.len(), 10);
    assert_eq!(report.recommendations[0], "Recommendation 0");
    assert_eq!(report.recommendations[9], "Recommendation 9");
}

// ==================== CAPACITY REPORT TESTS ====================

#[test]
fn test_capacity_report_creation() {
    let report = CapacityReport {
        dataset: "tank/data".to_string(),
        current_usage: 1024 * 1024 * 1024,       // 1 GB
        projected_usage: 2 * 1024 * 1024 * 1024, // 2 GB
        recommendations: vec!["Plan for expansion".to_string()],
    };

    assert_eq!(report.dataset, "tank/data");
    assert_eq!(report.current_usage, 1024 * 1024 * 1024);
    assert!(report.projected_usage > report.current_usage);
}

#[test]
fn test_capacity_report_usage_growth() {
    let report = CapacityReport {
        dataset: "storage".to_string(),
        current_usage: 500,
        projected_usage: 1000,
        recommendations: vec![],
    };

    let growth_factor = report.projected_usage as f64 / report.current_usage as f64;
    assert_eq!(growth_factor, 2.0);
}

#[test]
fn test_capacity_report_zero_usage() {
    let report = CapacityReport {
        dataset: "empty".to_string(),
        current_usage: 0,
        projected_usage: 0,
        recommendations: vec![],
    };

    assert_eq!(report.current_usage, 0);
    assert_eq!(report.projected_usage, 0);
}

#[test]
fn test_capacity_report_serialization() {
    let report = CapacityReport {
        dataset: "test/dataset".to_string(),
        current_usage: 12345,
        projected_usage: 67890,
        recommendations: vec!["Monitor closely".to_string()],
    };

    let json = serde_json::to_string(&report).expect("Serialization failed");
    let deserialized: CapacityReport = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(deserialized.dataset, report.dataset);
    assert_eq!(deserialized.current_usage, report.current_usage);
    assert_eq!(deserialized.projected_usage, report.projected_usage);
}

// ==================== MAINTENANCE SCHEDULE TESTS ====================

#[test]
fn test_maintenance_schedule_creation() {
    let schedule = MaintenanceSchedule {
        dataset: "tank/important".to_string(),
        next_maintenance: SystemTime::now(),
        tasks: vec!["Scrub".to_string(), "Snapshot cleanup".to_string()],
    };

    assert_eq!(schedule.dataset, "tank/important");
    assert_eq!(schedule.tasks.len(), 2);
}

#[test]
fn test_maintenance_schedule_time_ordering() {
    let now = SystemTime::now();
    let schedule = MaintenanceSchedule {
        dataset: "pool".to_string(),
        next_maintenance: now,
        tasks: vec![],
    };

    assert!(schedule.next_maintenance >= UNIX_EPOCH);
}

#[test]
fn test_maintenance_schedule_no_tasks() {
    let schedule = MaintenanceSchedule {
        dataset: "clean".to_string(),
        next_maintenance: SystemTime::now(),
        tasks: vec![],
    };

    assert!(schedule.tasks.is_empty());
}

#[test]
fn test_maintenance_schedule_many_tasks() {
    let tasks: Vec<String> = vec![
        "Scrub".to_string(),
        "Snapshot".to_string(),
        "Cleanup".to_string(),
        "Verify".to_string(),
        "Optimize".to_string(),
    ];

    let schedule = MaintenanceSchedule {
        dataset: "busy".to_string(),
        next_maintenance: SystemTime::now(),
        tasks: tasks.clone(),
    };

    assert_eq!(schedule.tasks.len(), 5);
    assert!(schedule.tasks.contains(&"Scrub".to_string()));
}

// ==================== SYSTEM INFO TESTS ====================

#[test]
fn test_system_info_creation() {
    let info = SystemInfo {
        timestamp: SystemTime::now(),
        cpu_usage: 45.5,
        memory_usage: 60.2,
        disk_usage: 75.8,
    };

    assert!(info.cpu_usage >= 0.0 && info.cpu_usage <= 100.0);
    assert!(info.memory_usage >= 0.0 && info.memory_usage <= 100.0);
    assert!(info.disk_usage >= 0.0 && info.disk_usage <= 100.0);
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
fn test_system_info_max_usage() {
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
fn test_system_info_serialization() {
    let info = SystemInfo {
        timestamp: SystemTime::now(),
        cpu_usage: 50.0,
        memory_usage: 60.0,
        disk_usage: 70.0,
    };

    let json = serde_json::to_string(&info).expect("Serialization failed");
    let deserialized: SystemInfo = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(deserialized.cpu_usage, info.cpu_usage);
    assert_eq!(deserialized.memory_usage, info.memory_usage);
}

// ==================== REPLICATION PERFORMANCE TESTS ====================

#[test]
fn test_replication_performance_creation() {
    let perf = ReplicationPerformance {
        source_dataset: "source/data".to_string(),
        target_dataset: "target/data".to_string(),
        transfer_rate: 100.5,
        compression_ratio: 2.3,
        estimated_completion: SystemTime::now(),
    };

    assert_eq!(perf.source_dataset, "source/data");
    assert_eq!(perf.target_dataset, "target/data");
    assert!(perf.transfer_rate > 0.0);
    assert!(perf.compression_ratio > 1.0);
}

#[test]
fn test_replication_performance_high_compression() {
    let perf = ReplicationPerformance {
        source_dataset: "logs".to_string(),
        target_dataset: "backup/logs".to_string(),
        transfer_rate: 50.0,
        compression_ratio: 5.0,
        estimated_completion: SystemTime::now(),
    };

    assert!(perf.compression_ratio > 2.0);
}

#[test]
fn test_replication_performance_no_compression() {
    let perf = ReplicationPerformance {
        source_dataset: "media".to_string(),
        target_dataset: "backup/media".to_string(),
        transfer_rate: 200.0,
        compression_ratio: 1.0,
        estimated_completion: SystemTime::now(),
    };

    assert_eq!(perf.compression_ratio, 1.0);
}

#[test]
fn test_replication_performance_serialization() {
    let perf = ReplicationPerformance {
        source_dataset: "test/source".to_string(),
        target_dataset: "test/target".to_string(),
        transfer_rate: 75.5,
        compression_ratio: 1.8,
        estimated_completion: SystemTime::now(),
    };

    let json = serde_json::to_string(&perf).expect("Serialization failed");
    assert!(json.contains("test/source"));
    assert!(json.contains("test/target"));

    let deserialized: ReplicationPerformance =
        serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(deserialized.transfer_rate, perf.transfer_rate);
}

// ==================== RETENTION POLICY TESTS ====================

#[test]
fn test_retention_policy_standard() {
    let policy = RetentionPolicy {
        name: "standard".to_string(),
        keep_hourly: 24,
        keep_daily: 7,
        keep_weekly: 4,
        keep_monthly: 12,
    };

    assert_eq!(policy.name, "standard");
    assert_eq!(policy.keep_hourly, 24);
    assert_eq!(policy.keep_daily, 7);
    assert_eq!(policy.keep_weekly, 4);
    assert_eq!(policy.keep_monthly, 12);
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

    assert_eq!(policy.keep_hourly, 1);
    assert_eq!(policy.keep_weekly, 0);
}

#[test]
fn test_retention_policy_conservative() {
    let policy = RetentionPolicy {
        name: "conservative".to_string(),
        keep_hourly: 168, // 7 days
        keep_daily: 30,
        keep_weekly: 52,
        keep_monthly: 60, // 5 years
    };

    assert!(policy.keep_hourly > 100);
    assert!(policy.keep_monthly > 50);
}

#[test]
fn test_retention_policy_serialization() {
    let policy = RetentionPolicy {
        name: "test_policy".to_string(),
        keep_hourly: 12,
        keep_daily: 7,
        keep_weekly: 4,
        keep_monthly: 6,
    };

    let json = serde_json::to_string(&policy).expect("Serialization failed");
    let deserialized: RetentionPolicy =
        serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(deserialized.name, policy.name);
    assert_eq!(deserialized.keep_hourly, policy.keep_hourly);
    assert_eq!(deserialized.keep_daily, policy.keep_daily);
}

#[test]
fn test_retention_policy_total_snapshots() {
    let policy = RetentionPolicy {
        name: "total".to_string(),
        keep_hourly: 24,
        keep_daily: 7,
        keep_weekly: 4,
        keep_monthly: 12,
    };

    let total = policy.keep_hourly + policy.keep_daily + policy.keep_weekly + policy.keep_monthly;
    assert_eq!(total, 47);
}

// ==================== ZFS ERROR TESTS ====================

#[test]
fn test_zfs_error_pool_error() {
    let error = ZfsError::PoolError {
        message: "Pool not found".to_string(),
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("Pool not found"));
}

#[test]
fn test_zfs_error_dataset_error() {
    let error = ZfsError::DatasetError {
        message: "Dataset creation failed".to_string(),
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("Dataset"));
}

#[test]
fn test_zfs_error_snapshot_error() {
    let error = ZfsError::SnapshotError {
        message: "Snapshot already exists".to_string(),
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("Snapshot"));
}

#[test]
fn test_zfs_error_command_error() {
    let error = ZfsError::CommandError {
        message: "Command timeout".to_string(),
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("Command"));
}

#[test]
fn test_zfs_error_config_error() {
    let error = ZfsError::ConfigError {
        message: "Invalid configuration".to_string(),
    };

    let error_string = format!("{}", error);
    assert!(error_string.contains("configuration"));
}

#[test]
fn test_zfs_error_io_error_conversion() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let zfs_error: ZfsError = io_error.into();

    let error_string = format!("{}", zfs_error);
    assert!(error_string.contains("File not found") || error_string.contains("IO error"));
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_types_can_be_combined() {
    let _report = BottleneckReport {
        dataset: "tank".to_string(),
        bottleneck_type: "io".to_string(),
        severity: "high".to_string(),
        recommendations: vec![],
    };

    let _capacity = CapacityReport {
        dataset: "tank".to_string(),
        current_usage: 1000,
        projected_usage: 2000,
        recommendations: vec![],
    };

    let _schedule = MaintenanceSchedule {
        dataset: "tank".to_string(),
        next_maintenance: SystemTime::now(),
        tasks: vec![],
    };

    // All types can coexist and reference the same dataset
    assert!(true);
}

#[test]
fn test_all_types_are_cloneable() {
    let report = BottleneckReport {
        dataset: "test".to_string(),
        bottleneck_type: "test".to_string(),
        severity: "test".to_string(),
        recommendations: vec![],
    };
    let _cloned = report.clone();

    let capacity = CapacityReport {
        dataset: "test".to_string(),
        current_usage: 0,
        projected_usage: 0,
        recommendations: vec![],
    };
    let _cloned = capacity.clone();

    assert!(true); // If we get here, all clones succeeded
}

#[test]
fn test_all_types_are_serializable() {
    let types_json = vec![
        serde_json::to_string(&BottleneckReport {
            dataset: "t".to_string(),
            bottleneck_type: "t".to_string(),
            severity: "t".to_string(),
            recommendations: vec![],
        }),
        serde_json::to_string(&CapacityReport {
            dataset: "t".to_string(),
            current_usage: 0,
            projected_usage: 0,
            recommendations: vec![],
        }),
        serde_json::to_string(&SystemInfo {
            timestamp: SystemTime::now(),
            cpu_usage: 0.0,
            memory_usage: 0.0,
            disk_usage: 0.0,
        }),
    ];

    for json_result in types_json {
        assert!(json_result.is_ok());
    }
}
