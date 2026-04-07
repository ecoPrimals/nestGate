// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for performance metric types
//!
//! Tests all performance structures, defaults, serialization, and edge cases.

#[cfg(test)]
#[expect(
    clippy::float_cmp,
    reason = "performance metric tests assert exact Default() zeros and fixed thresholds"
)]
mod performance_types_tests {
    use super::super::types::*;
    use std::collections::HashMap;
    use std::time::{Duration, SystemTime};

    // ==================== SystemPerformanceMetrics Tests ====================

    #[test]
    fn test_system_performance_metrics_default() {
        let metrics = SystemPerformanceMetrics::default();

        assert_eq!(metrics.memory_utilization_percent, 0.0);
        assert_eq!(metrics.cpu_utilization_percent, 0.0);
        assert_eq!(metrics.disk_queue_depth, 0);
        assert_eq!(metrics.network_throughput_mbs, 0.0);
        assert_eq!(metrics.system_load_average, 0.0);
    }

    #[test]
    fn test_system_performance_metrics_creation() {
        let metrics = SystemPerformanceMetrics {
            memory_utilization_percent: 75.5,
            cpu_utilization_percent: 60.0,
            disk_queue_depth: 10,
            network_throughput_mbs: 125.5,
            system_load_average: 2.5,
        };

        assert_eq!(metrics.memory_utilization_percent, 75.5);
        assert_eq!(metrics.cpu_utilization_percent, 60.0);
    }

    #[test]
    fn test_system_performance_metrics_clone() {
        let metrics1 = SystemPerformanceMetrics::default();
        let metrics2 = metrics1.clone();

        assert_eq!(
            metrics1.cpu_utilization_percent,
            metrics2.cpu_utilization_percent
        );
    }

    #[test]
    fn test_system_performance_metrics_serialization() {
        let metrics = SystemPerformanceMetrics::default();
        let json = serde_json::to_string(&metrics).expect("Failed to serialize");

        assert!(json.contains("memory_utilization_percent"));
        assert!(json.contains("cpu_utilization_percent"));
    }

    #[test]
    fn test_system_performance_metrics_deserialization() {
        let json = r#"{
            "memory_utilization_percent": 50.0,
            "cpu_utilization_percent": 30.0,
            "disk_queue_depth": 5,
            "network_throughput_mbs": 100.0,
            "system_load_average": 1.5
        }"#;

        let metrics: SystemPerformanceMetrics =
            serde_json::from_str(json).expect("Failed to deserialize");

        assert_eq!(metrics.memory_utilization_percent, 50.0);
        assert_eq!(metrics.disk_queue_depth, 5);
    }

    // ==================== MemoryInfo Tests ====================

    #[test]
    fn test_memory_info_default() {
        let info = MemoryInfo::default();

        assert_eq!(info.utilization_percent, 0.0);
        assert_eq!(info.total_mb, 0);
        assert_eq!(info.available_mb, 0);
        assert_eq!(info.used_mb, 0);
    }

    #[test]
    fn test_memory_info_creation() {
        let info = MemoryInfo {
            utilization_percent: 75.0,
            total_mb: 16384,
            available_mb: 4096,
            used_mb: 12288,
        };

        assert_eq!(info.total_mb, 16384);
        assert_eq!(info.used_mb, 12288);
    }

    #[test]
    fn test_memory_info_serialization() {
        let info = MemoryInfo {
            utilization_percent: 50.0,
            total_mb: 8192,
            available_mb: 4096,
            used_mb: 4096,
        };

        let json = serde_json::to_string(&info).expect("Failed to serialize");
        let restored: MemoryInfo = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(info.total_mb, restored.total_mb);
    }

    // ==================== DiskIoStats Tests ====================

    #[test]
    fn test_disk_io_stats_default() {
        let stats = DiskIoStats::default();

        assert_eq!(stats.queue_depth, 0);
        assert_eq!(stats.throughput_mbs, 0.0);
        assert_eq!(stats.read_iops, 0);
        assert_eq!(stats.write_iops, 0);
    }

    #[test]
    fn test_disk_io_stats_creation() {
        let stats = DiskIoStats {
            queue_depth: 32,
            throughput_mbs: 500.0,
            read_iops: 10000,
            write_iops: 5000,
        };

        assert_eq!(stats.queue_depth, 32);
        assert_eq!(stats.read_iops, 10000);
    }

    #[test]
    fn test_disk_io_stats_serialization() {
        let stats = DiskIoStats::default();
        let json = serde_json::to_string(&stats).expect("Failed to serialize");

        assert!(json.contains("queue_depth"));
        assert!(json.contains("throughput_mbs"));
    }

    // ==================== PoolPerformanceMetrics Tests ====================

    #[test]
    fn test_pool_performance_metrics_creation() {
        let metrics = PoolPerformanceMetrics {
            total_iops: 15000.0,
            total_throughput_mbs: 750.0,
            avg_latency_ms: 5.5,
            utilization_percent: 75.0,
            fragmentation_percent: 12.0,
            compression_ratio: 1.5,
            dedup_ratio: 1.2,
        };

        assert_eq!(metrics.total_iops, 15000.0);
        assert_eq!(metrics.compression_ratio, 1.5);
    }

    #[test]
    fn test_pool_performance_metrics_clone() {
        let metrics1 = PoolPerformanceMetrics {
            total_iops: 1000.0,
            total_throughput_mbs: 100.0,
            avg_latency_ms: 10.0,
            utilization_percent: 50.0,
            fragmentation_percent: 5.0,
            compression_ratio: 1.0,
            dedup_ratio: 1.0,
        };

        let metrics2 = metrics1.clone();
        assert_eq!(metrics1.total_iops, metrics2.total_iops);
    }

    #[test]
    fn test_pool_performance_metrics_serialization() {
        let metrics = PoolPerformanceMetrics {
            total_iops: 5000.0,
            total_throughput_mbs: 250.0,
            avg_latency_ms: 3.0,
            utilization_percent: 60.0,
            fragmentation_percent: 8.0,
            compression_ratio: 1.3,
            dedup_ratio: 1.1,
        };

        let json = serde_json::to_string(&metrics).expect("Failed to serialize");
        let restored: PoolPerformanceMetrics =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(metrics.total_iops, restored.total_iops);
    }

    // ==================== SystemResourceMetrics Tests ====================

    #[test]
    fn test_system_resource_metrics_creation() {
        let metrics = SystemResourceMetrics {
            cpu_utilization_percent: 45.0,
            memory_usage_bytes: 8_000_000_000,
            available_memory_bytes: 8_000_000_000,
            network_io_mbs: 100.0,
            io_wait_percent: 5.0,
            load_average_1m: 2.5,
        };

        assert_eq!(metrics.cpu_utilization_percent, 45.0);
        assert_eq!(metrics.memory_usage_bytes, 8_000_000_000);
    }

    #[test]
    fn test_system_resource_metrics_serialization() {
        let metrics = SystemResourceMetrics {
            cpu_utilization_percent: 30.0,
            memory_usage_bytes: 4_000_000_000,
            available_memory_bytes: 12_000_000_000,
            network_io_mbs: 50.0,
            io_wait_percent: 2.0,
            load_average_1m: 1.5,
        };

        let json = serde_json::to_string(&metrics).expect("Failed to serialize");
        assert!(json.contains("cpu_utilization_percent"));
        assert!(json.contains("memory_usage_bytes"));
    }

    // ==================== IoStatistics Tests ====================

    #[test]
    fn test_io_statistics_creation() {
        let stats = IoStatistics {
            total_reads: 1_000_000,
            total_writes: 500_000,
            total_bytes_read: 10_000_000_000,
            total_bytes_written: 5_000_000_000,
            avg_io_size_bytes: 4096,
            read_write_ratio: 2.0,
        };

        assert_eq!(stats.total_reads, 1_000_000);
        assert_eq!(stats.read_write_ratio, 2.0);
    }

    #[test]
    fn test_io_statistics_serialization() {
        let stats = IoStatistics {
            total_reads: 100,
            total_writes: 50,
            total_bytes_read: 1000,
            total_bytes_written: 500,
            avg_io_size_bytes: 512,
            read_write_ratio: 2.0,
        };

        let json = serde_json::to_string(&stats).expect("Failed to serialize");
        let restored: IoStatistics = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(stats.total_reads, restored.total_reads);
    }

    // ==================== PerformanceTrends Tests ====================

    #[test]
    fn test_performance_trends_creation() {
        let trends = PerformanceTrends {
            iops_trend: 0.5,
            throughput_trend: 0.3,
            latency_trend: -0.1,
            utilization_trend: 0.2,
            prediction_confidence: 0.85,
        };

        assert_eq!(trends.iops_trend, 0.5);
        assert_eq!(trends.prediction_confidence, 0.85);
    }

    #[test]
    fn test_performance_trends_serialization() {
        let trends = PerformanceTrends {
            iops_trend: 1.0,
            throughput_trend: 0.5,
            latency_trend: 0.1,
            utilization_trend: 0.3,
            prediction_confidence: 0.9,
        };

        let json = serde_json::to_string(&trends).expect("Failed to serialize");
        assert!(json.contains("iops_trend"));
        assert!(json.contains("prediction_confidence"));
    }

    // ==================== AlertMetric Tests ====================

    #[test]
    fn test_alert_metric_all_variants() {
        let metrics = vec![
            AlertMetric::Iops,
            AlertMetric::Throughput,
            AlertMetric::Latency,
            AlertMetric::Utilization,
            AlertMetric::MemoryUsage,
            AlertMetric::CpuUsage,
            AlertMetric::ErrorRate,
            AlertMetric::Availability,
            AlertMetric::QueueDepth,
            AlertMetric::CacheHitRatio,
        ];

        for metric in metrics {
            let json = serde_json::to_string(&metric).expect("Failed to serialize");
            assert!(!json.is_empty());
        }
    }

    #[test]
    fn test_alert_metric_serialization() {
        let metric = AlertMetric::Iops;
        let json = serde_json::to_string(&metric).expect("Failed to serialize");
        let restored: AlertMetric = serde_json::from_str(&json).expect("Failed to deserialize");

        let json2 = serde_json::to_string(&restored).unwrap();
        assert_eq!(json, json2);
    }

    // ==================== AlertOperator Tests ====================

    #[test]
    fn test_alert_operator_all_variants() {
        let operators = vec![
            AlertOperator::GreaterThan,
            AlertOperator::LessThan,
            AlertOperator::EqualTo,
            AlertOperator::GreaterThanOrEqualTo,
            AlertOperator::LessThanOrEqualTo,
            AlertOperator::NotEqualTo,
        ];

        for op in operators {
            let json = serde_json::to_string(&op).expect("Failed to serialize");
            assert!(!json.is_empty());
        }
    }

    #[test]
    fn test_alert_operator_serialization() {
        let op = AlertOperator::GreaterThan;
        let json = serde_json::to_string(&op).expect("Failed to serialize");

        assert!(json.contains("GreaterThan"));
    }

    // ==================== AlertSeverity Tests ====================

    #[test]
    fn test_alert_severity_all_variants() {
        let severities = vec![
            AlertSeverity::Critical,
            AlertSeverity::Warning,
            AlertSeverity::Info,
        ];

        for severity in severities {
            let json = serde_json::to_string(&severity).expect("Failed to serialize");
            assert!(!json.is_empty());
        }
    }

    #[test]
    fn test_alert_severity_serialization() {
        let severity = AlertSeverity::Critical;
        let json = serde_json::to_string(&severity).expect("Failed to serialize");

        assert!(json.contains("Critical"));
    }

    // ==================== TierPerformanceTargets Tests ====================

    #[test]
    fn test_tier_performance_targets_creation() {
        let targets = TierPerformanceTargets {
            target_iops: 10000.0,
            target_throughput_mbs: 500.0,
            target_latency_ms: 5.0,
            target_utilization_percent: 80.0,
            target_availability_percent: 99.9,
        };

        assert_eq!(targets.target_iops, 10000.0);
        assert_eq!(targets.target_availability_percent, 99.9);
    }

    #[test]
    fn test_tier_performance_targets_serialization() {
        let targets = TierPerformanceTargets {
            target_iops: 5000.0,
            target_throughput_mbs: 250.0,
            target_latency_ms: 10.0,
            target_utilization_percent: 75.0,
            target_availability_percent: 99.5,
        };

        let json = serde_json::to_string(&targets).expect("Failed to serialize");
        let restored: TierPerformanceTargets =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(targets.target_iops, restored.target_iops);
    }

    // ==================== SlaCompliance Tests ====================

    #[test]
    fn test_sla_compliance_creation() {
        let sla = SlaCompliance {
            latency_compliance: 95.0,
            throughput_compliance: 98.0,
            availability_percent: 99.9,
            error_rate_compliance: 99.5,
        };

        assert_eq!(sla.latency_compliance, 95.0);
        assert_eq!(sla.availability_percent, 99.9);
    }

    #[test]
    fn test_sla_compliance_serialization() {
        let sla = SlaCompliance {
            latency_compliance: 90.0,
            throughput_compliance: 95.0,
            availability_percent: 99.0,
            error_rate_compliance: 98.0,
        };

        let json = serde_json::to_string(&sla).expect("Failed to serialize");
        assert!(json.contains("latency_compliance"));
        assert!(json.contains("availability_percent"));
    }

    // ==================== AlertCondition Tests ====================

    #[test]
    fn test_alert_condition_creation() {
        let condition = AlertCondition {
            id: "alert-001".to_string(),
            name: "High CPU".to_string(),
            description: "CPU usage above 90%".to_string(),
            metric: AlertMetric::CpuUsage,
            operator: AlertOperator::GreaterThan,
            threshold: 90.0,
            duration: Duration::from_secs(300),
            severity: AlertSeverity::Warning,
            enabled: true,
        };

        assert_eq!(condition.id, "alert-001");
        assert_eq!(condition.threshold, 90.0);
        assert!(condition.enabled);
    }

    #[test]
    fn test_alert_condition_serialization() {
        let condition = AlertCondition {
            id: "test-alert".to_string(),
            name: "Test".to_string(),
            description: "Test alert".to_string(),
            metric: AlertMetric::Iops,
            operator: AlertOperator::LessThan,
            threshold: 100.0,
            duration: Duration::from_secs(60),
            severity: AlertSeverity::Info,
            enabled: false,
        };

        let json = serde_json::to_string(&condition).expect("Failed to serialize");
        let restored: AlertCondition = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(condition.id, restored.id);
        assert_eq!(condition.enabled, restored.enabled);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_system_performance_metrics_extreme_values() {
        let metrics = SystemPerformanceMetrics {
            memory_utilization_percent: 100.0,
            cpu_utilization_percent: 100.0,
            disk_queue_depth: u64::MAX,
            network_throughput_mbs: f64::MAX,
            system_load_average: 999.99,
        };

        assert_eq!(metrics.memory_utilization_percent, 100.0);
        assert_eq!(metrics.disk_queue_depth, u64::MAX);
    }

    #[test]
    fn test_memory_info_zero_values() {
        let info = MemoryInfo {
            utilization_percent: 0.0,
            total_mb: 0,
            available_mb: 0,
            used_mb: 0,
        };

        assert_eq!(info.utilization_percent, 0.0);
    }

    #[test]
    fn test_performance_trends_negative_values() {
        let trends = PerformanceTrends {
            iops_trend: -0.5,
            throughput_trend: -0.3,
            latency_trend: 0.2,
            utilization_trend: -0.1,
            prediction_confidence: 0.0,
        };

        assert_eq!(trends.iops_trend, -0.5);
    }

    #[test]
    fn test_io_statistics_zero_operations() {
        let stats = IoStatistics {
            total_reads: 0,
            total_writes: 0,
            total_bytes_read: 0,
            total_bytes_written: 0,
            avg_io_size_bytes: 0,
            read_write_ratio: 0.0,
        };

        assert_eq!(stats.total_reads, 0);
    }

    // ==================== Complex Structures Tests ====================

    #[test]
    fn test_current_performance_metrics_creation() {
        let pool_metrics = PoolPerformanceMetrics {
            total_iops: 1000.0,
            total_throughput_mbs: 100.0,
            avg_latency_ms: 5.0,
            utilization_percent: 50.0,
            fragmentation_percent: 10.0,
            compression_ratio: 1.5,
            dedup_ratio: 1.0,
        };

        let system_metrics = SystemResourceMetrics {
            cpu_utilization_percent: 40.0,
            memory_usage_bytes: 4_000_000_000,
            available_memory_bytes: 8_000_000_000,
            network_io_mbs: 50.0,
            io_wait_percent: 3.0,
            load_average_1m: 1.5,
        };

        let io_stats = IoStatistics {
            total_reads: 1000,
            total_writes: 500,
            total_bytes_read: 1_000_000,
            total_bytes_written: 500_000,
            avg_io_size_bytes: 4096,
            read_write_ratio: 2.0,
        };

        let trends = PerformanceTrends {
            iops_trend: 0.1,
            throughput_trend: 0.2,
            latency_trend: -0.05,
            utilization_trend: 0.15,
            prediction_confidence: 0.8,
        };

        let metrics = CurrentPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            tier_metrics: HashMap::new(),
            system_metrics,
            io_statistics: io_stats,
            trends,
        };

        assert!(metrics.pool_metrics.total_iops > 0.0);
        assert!(metrics.tier_metrics.is_empty());
    }

    #[test]
    fn test_performance_snapshot_creation() {
        let pool_metrics = PoolPerformanceMetrics {
            total_iops: 5000.0,
            total_throughput_mbs: 250.0,
            avg_latency_ms: 3.0,
            utilization_percent: 60.0,
            fragmentation_percent: 8.0,
            compression_ratio: 1.3,
            dedup_ratio: 1.1,
        };

        let metrics = CurrentPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            tier_metrics: HashMap::new(),
            system_metrics: SystemResourceMetrics {
                cpu_utilization_percent: 30.0,
                memory_usage_bytes: 2_000_000_000,
                available_memory_bytes: 14_000_000_000,
                network_io_mbs: 25.0,
                io_wait_percent: 1.0,
                load_average_1m: 0.8,
            },
            io_statistics: IoStatistics {
                total_reads: 500,
                total_writes: 250,
                total_bytes_read: 500_000,
                total_bytes_written: 250_000,
                avg_io_size_bytes: 4096,
                read_write_ratio: 2.0,
            },
            trends: PerformanceTrends {
                iops_trend: 0.05,
                throughput_trend: 0.1,
                latency_trend: -0.02,
                utilization_trend: 0.08,
                prediction_confidence: 0.9,
            },
        };

        let snapshot = PerformanceSnapshot {
            timestamp: SystemTime::now(),
            metrics,
            performance_score: 85.0,
        };

        assert_eq!(snapshot.performance_score, 85.0);
    }
}
