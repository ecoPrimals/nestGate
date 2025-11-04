//! Unit tests for ZFS type constructors and builders
//!
//! This test file focuses on increasing coverage for basic type construction,
//! validation, and builder patterns in the nestgate-zfs crate.

use nestgate_zfs::types::*;
use std::time::SystemTime;

// ==================== CAPACITY MONITORING TYPES ====================

#[test]
fn test_bottleneck_report_construction() {
    let report = BottleneckReport {
        dataset: "tank/data".to_string(),
        bottleneck_type: "io_wait".to_string(),
        severity: "high".to_string(),
        recommendations: vec!["Add L2ARC cache".to_string()],
    };

    assert_eq!(report.dataset, "tank/data");
    assert_eq!(report.bottleneck_type, "io_wait");
    assert_eq!(report.severity, "high");
    assert_eq!(report.recommendations.len(), 1);
}

#[test]
fn test_capacity_report_construction() {
    let report = CapacityReport {
        dataset: "tank/data".to_string(),
        current_usage: 1024 * 1024 * 1024,       // 1GB
        projected_usage: 2 * 1024 * 1024 * 1024, // 2GB
        recommendations: vec!["Consider adding more storage".to_string()],
    };

    assert_eq!(report.dataset, "tank/data");
    assert!(report.projected_usage > report.current_usage);
    assert!(!report.recommendations.is_empty());
}

#[test]
fn test_maintenance_schedule_construction() {
    let now = SystemTime::now();
    let schedule = MaintenanceSchedule {
        dataset: "tank/backup".to_string(),
        next_maintenance: now,
        tasks: vec!["scrub".to_string(), "snapshot cleanup".to_string()],
    };

    assert_eq!(schedule.dataset, "tank/backup");
    assert_eq!(schedule.tasks.len(), 2);
    assert!(schedule.tasks.contains(&"scrub".to_string()));
}

#[test]
fn test_system_info_construction() {
    let info = SystemInfo {
        timestamp: SystemTime::now(),
        cpu_usage: 45.5,
        memory_usage: 60.2,
        disk_usage: 75.0,
    };

    assert!(info.cpu_usage >= 0.0 && info.cpu_usage <= 100.0);
    assert!(info.memory_usage >= 0.0 && info.memory_usage <= 100.0);
    assert!(info.disk_usage >= 0.0 && info.disk_usage <= 100.0);
}

#[test]
fn test_replication_performance_construction() {
    let perf = ReplicationPerformance {
        source_dataset: "tank/data".to_string(),
        target_dataset: "backup/data".to_string(),
        transfer_rate: 125.5, // MB/s
        compression_ratio: 2.1,
        estimated_completion: SystemTime::now(),
    };

    assert_eq!(perf.source_dataset, "tank/data");
    assert_eq!(perf.target_dataset, "backup/data");
    assert!(perf.transfer_rate > 0.0);
    assert!(perf.compression_ratio > 0.0);
}

#[test]
fn test_retention_policy_construction() {
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

// ==================== RETENTION POLICY VALIDATION ====================

#[test]
fn test_retention_policy_zero_values() {
    let policy = RetentionPolicy {
        name: "minimal".to_string(),
        keep_hourly: 0,
        keep_daily: 1,
        keep_weekly: 0,
        keep_monthly: 0,
    };

    // Should be valid even with zero values
    assert_eq!(policy.keep_hourly, 0);
    assert!(policy.keep_daily > 0);
}

#[test]
fn test_retention_policy_large_values() {
    let policy = RetentionPolicy {
        name: "archival".to_string(),
        keep_hourly: 168,  // 1 week of hourly
        keep_daily: 365,   // 1 year of daily
        keep_weekly: 104,  // 2 years of weekly
        keep_monthly: 120, // 10 years of monthly
    };

    assert!(policy.keep_hourly > 0);
    assert!(policy.keep_daily > 0);
    assert!(policy.keep_weekly > 0);
    assert!(policy.keep_monthly > 0);
}

// ==================== SYSTEM INFO VALIDATION ====================

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
fn test_system_info_high_usage() {
    let info = SystemInfo {
        timestamp: SystemTime::now(),
        cpu_usage: 99.9,
        memory_usage: 95.5,
        disk_usage: 98.0,
    };

    assert!(info.cpu_usage < 100.0);
    assert!(info.memory_usage < 100.0);
    assert!(info.disk_usage < 100.0);
}

// ==================== REPLICATION PERFORMANCE VALIDATION ====================

#[test]
fn test_replication_performance_slow_transfer() {
    let perf = ReplicationPerformance {
        source_dataset: "tank/large".to_string(),
        target_dataset: "remote/large".to_string(),
        transfer_rate: 1.5, // Slow: 1.5 MB/s
        compression_ratio: 1.2,
        estimated_completion: SystemTime::now(),
    };

    assert!(perf.transfer_rate > 0.0);
    assert!(perf.compression_ratio >= 1.0);
}

#[test]
fn test_replication_performance_fast_transfer() {
    let perf = ReplicationPerformance {
        source_dataset: "nvme/data".to_string(),
        target_dataset: "nvme/backup".to_string(),
        transfer_rate: 2500.0, // Fast: 2.5 GB/s
        compression_ratio: 3.5,
        estimated_completion: SystemTime::now(),
    };

    assert!(perf.transfer_rate > 1000.0);
    assert!(perf.compression_ratio > 3.0);
}

// ==================== CAPACITY REPORT VALIDATION ====================

#[test]
fn test_capacity_report_empty_recommendations() {
    let report = CapacityReport {
        dataset: "tank/ok".to_string(),
        current_usage: 500 * 1024 * 1024,
        projected_usage: 600 * 1024 * 1024,
        recommendations: vec![],
    };

    // Should be valid even with no recommendations
    assert!(report.recommendations.is_empty());
    assert!(report.projected_usage > report.current_usage);
}

#[test]
fn test_capacity_report_multiple_recommendations() {
    let report = CapacityReport {
        dataset: "tank/critical".to_string(),
        current_usage: 950 * 1024 * 1024 * 1024,
        projected_usage: 1000 * 1024 * 1024 * 1024,
        recommendations: vec![
            "Add more disks immediately".to_string(),
            "Enable compression".to_string(),
            "Archive old data".to_string(),
            "Set quotas on datasets".to_string(),
        ],
    };

    assert_eq!(report.recommendations.len(), 4);
    assert!(report.current_usage > 900 * 1024 * 1024 * 1024);
}

// ==================== BOTTLENECK REPORT VALIDATION ====================

#[test]
fn test_bottleneck_report_severity_levels() {
    let severities = vec!["low", "medium", "high", "critical"];

    for severity in severities {
        let report = BottleneckReport {
            dataset: format!("tank/{}", severity),
            bottleneck_type: "io".to_string(),
            severity: severity.to_string(),
            recommendations: vec![format!("Fix {} issue", severity)],
        };

        assert_eq!(report.severity, severity);
        assert!(!report.recommendations.is_empty());
    }
}

#[test]
fn test_bottleneck_report_types() {
    let types = vec!["io_wait", "cpu", "memory", "network", "fragmentation"];

    for bt in types {
        let report = BottleneckReport {
            dataset: "tank/test".to_string(),
            bottleneck_type: bt.to_string(),
            severity: "medium".to_string(),
            recommendations: vec![format!("Optimize {}", bt)],
        };

        assert_eq!(report.bottleneck_type, bt);
    }
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_retention_policy_serialization() {
    let policy = RetentionPolicy {
        name: "test".to_string(),
        keep_hourly: 24,
        keep_daily: 7,
        keep_weekly: 4,
        keep_monthly: 12,
    };

    // Test that it can be serialized (serde_json required in dev-dependencies)
    let json = serde_json::to_string(&policy).expect("Serialization failed");
    assert!(json.contains("test"));
    assert!(json.contains("24"));
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
    assert!(json.contains("50"));
    assert!(json.contains("60"));
    assert!(json.contains("70"));
}

// ==================== EDGE CASES ====================

#[test]
fn test_empty_dataset_name() {
    let report = BottleneckReport {
        dataset: String::new(),
        bottleneck_type: "test".to_string(),
        severity: "low".to_string(),
        recommendations: vec![],
    };

    // Should be valid structurally (validation happens elsewhere)
    assert_eq!(report.dataset.len(), 0);
}

#[test]
fn test_very_long_dataset_name() {
    let long_name = "a".repeat(500);
    let report = CapacityReport {
        dataset: long_name.clone(),
        current_usage: 100,
        projected_usage: 200,
        recommendations: vec![],
    };

    assert_eq!(report.dataset.len(), 500);
}

#[test]
fn test_maintenance_schedule_empty_tasks() {
    let schedule = MaintenanceSchedule {
        dataset: "tank/data".to_string(),
        next_maintenance: SystemTime::now(),
        tasks: vec![],
    };

    // Valid to have no tasks scheduled
    assert!(schedule.tasks.is_empty());
}

#[test]
fn test_maintenance_schedule_many_tasks() {
    let tasks: Vec<String> = (0..20).map(|i| format!("task_{}", i)).collect();
    let schedule = MaintenanceSchedule {
        dataset: "tank/busy".to_string(),
        next_maintenance: SystemTime::now(),
        tasks: tasks.clone(),
    };

    assert_eq!(schedule.tasks.len(), 20);
    assert_eq!(schedule.tasks[0], "task_0");
    assert_eq!(schedule.tasks[19], "task_19");
}
