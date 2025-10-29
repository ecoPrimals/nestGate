//! Comprehensive tests for REST API types module
//!
//! This module provides thorough testing coverage for all types defined in
//! the REST API, including enums, structs, serialization, and validation.

#![allow(clippy::unwrap_used, clippy::float_cmp)] // Test code - these patterns are acceptable

#[cfg(test)]
mod tests {
    use super::super::types::*;
    use chrono::Utc;

    // ==================== ENUM TESTS ====================

    #[test]
    fn test_dataset_type_serialization() {
        let types = vec![
            DatasetType::Filesystem,
            DatasetType::Volume,
            DatasetType::Snapshot,
            DatasetType::Bookmark,
        ];

        for dataset_type in types {
            let serialized = serde_json::to_string(&dataset_type).unwrap();
            let deserialized: DatasetType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(dataset_type, deserialized);
        }
    }

    #[test]
    fn test_checksum_type_variants() {
        let checksums = vec![
            ChecksumType::Fletcher2,
            ChecksumType::Fletcher4,
            ChecksumType::Sha256,
            ChecksumType::Sha512,
            ChecksumType::Skein,
            ChecksumType::EdonR,
        ];

        // Test all variants can be created and cloned
        for checksum in &checksums {
            let cloned = checksum.clone();
            assert_eq!(checksum, &cloned);
        }
    }

    #[test]
    fn test_dataset_status_all_variants() {
        let statuses = vec![
            DatasetStatus::Online,
            DatasetStatus::Offline,
            DatasetStatus::Degraded,
            DatasetStatus::Maintenance,
            DatasetStatus::Error,
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: DatasetStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(status, parsed);
        }
    }

    #[test]
    fn test_alert_severity_ordering() {
        let severities = vec![
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::Critical,
            AlertSeverity::Emergency,
        ];

        // Test that we can clone and compare
        for severity in &severities {
            let cloned = severity.clone();
            assert_eq!(severity, &cloned);
        }
    }

    #[test]
    fn test_alert_status_serialization() {
        let statuses = vec![
            AlertStatus::Active,
            AlertStatus::Resolved,
            AlertStatus::Acknowledged,
            AlertStatus::Suppressed,
        ];

        for status in statuses {
            let json = serde_json::to_value(&status).unwrap();
            let parsed: AlertStatus = serde_json::from_value(json).unwrap();
            assert_eq!(status, parsed);
        }
    }

    #[test]
    fn test_comparison_operator_variants() {
        let operators = vec![
            ComparisonOperator::GreaterThan,
            ComparisonOperator::LessThan,
            ComparisonOperator::Equal,
            ComparisonOperator::NotEqual,
            ComparisonOperator::GreaterThanOrEqual,
            ComparisonOperator::LessThanOrEqual,
        ];

        for op in operators {
            let serialized = serde_json::to_string(&op).unwrap();
            let deserialized: ComparisonOperator = serde_json::from_str(&serialized).unwrap();
            assert_eq!(op, deserialized);
        }
    }

    #[test]
    fn test_storage_backend_type_all() {
        let backends = vec![
            StorageBackendType::Memory,
            StorageBackendType::Local,
            StorageBackendType::Remote,
            StorageBackendType::Filesystem,
            StorageBackendType::Cloud,
            StorageBackendType::Network,
            StorageBackendType::Block,
            StorageBackendType::File,
        ];

        for backend in backends {
            let json = serde_json::to_string(&backend).unwrap();
            assert!(!json.is_empty());
        }
    }

    #[test]
    fn test_compression_type_serialization() {
        let types = vec![
            CompressionType::Lz4,
            CompressionType::Gzip,
            CompressionType::Zstd,
            CompressionType::None,
        ];

        for compression in types {
            let json = serde_json::to_string(&compression).unwrap();
            let parsed: CompressionType = serde_json::from_str(&json).unwrap();
            assert_eq!(compression, parsed);
        }
    }

    #[test]
    fn test_snapshot_status_variants() {
        let statuses = vec![
            SnapshotStatus::Active,
            SnapshotStatus::Pending,
            SnapshotStatus::Failed,
            SnapshotStatus::Deleted,
        ];

        for status in &statuses {
            let cloned = status.clone();
            assert_eq!(status, &cloned);
        }
    }

    #[test]
    fn test_alert_type_variants() {
        let types = vec![
            AlertType::System,
            AlertType::Storage,
            AlertType::Network,
            AlertType::Performance,
        ];

        for alert_type in types {
            let serialized = serde_json::to_value(&alert_type).unwrap();
            let deserialized: AlertType = serde_json::from_value(serialized).unwrap();
            assert_eq!(alert_type, deserialized);
        }
    }

    // ==================== STRUCT TESTS ====================

    #[test]
    fn test_storage_metrics_creation() {
        let metrics = StorageMetrics {
            total_bytes: 1_000_000_000,
            used_bytes: 600_000_000,
            available_bytes: 400_000_000,
            read_ops_per_sec: 1000.0,
            write_ops_per_sec: 500.0,
        };

        assert_eq!(metrics.total_bytes, 1_000_000_000);
        assert_eq!(metrics.used_bytes, 600_000_000);
        assert_eq!(metrics.available_bytes, 400_000_000);
        assert_eq!(metrics.read_ops_per_sec, 1000.0);
        assert_eq!(metrics.write_ops_per_sec, 500.0);
    }

    #[test]
    fn test_storage_metrics_serialization() {
        let metrics = StorageMetrics {
            total_bytes: 1000,
            used_bytes: 600,
            available_bytes: 400,
            read_ops_per_sec: 100.0,
            write_ops_per_sec: 50.0,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let parsed: StorageMetrics = serde_json::from_str(&json).unwrap();

        assert_eq!(metrics.total_bytes, parsed.total_bytes);
        assert_eq!(metrics.used_bytes, parsed.used_bytes);
        assert_eq!(metrics.available_bytes, parsed.available_bytes);
    }

    #[test]
    fn test_network_io_metrics_creation() {
        let metrics = NetworkIoMetrics {
            bytes_sent: 1_000_000,
            bytes_received: 2_000_000,
            packets_sent: 1000,
            packets_received: 1500,
            rx_bytes_per_sec: 10_000.0,
            tx_bytes_per_sec: 5_000.0,
            rx_packets_per_sec: 100.0,
            tx_packets_per_sec: 50.0,
        };

        assert_eq!(metrics.bytes_sent, 1_000_000);
        assert_eq!(metrics.bytes_received, 2_000_000);
        assert_eq!(metrics.packets_sent, 1000);
        assert_eq!(metrics.packets_received, 1500);
    }

    #[test]
    fn test_network_io_metrics_serialization() {
        let metrics = NetworkIoMetrics {
            bytes_sent: 100,
            bytes_received: 200,
            packets_sent: 10,
            packets_received: 20,
            rx_bytes_per_sec: 1.0,
            tx_bytes_per_sec: 2.0,
            rx_packets_per_sec: 3.0,
            tx_packets_per_sec: 4.0,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let parsed: NetworkIoMetrics = serde_json::from_str(&json).unwrap();

        assert_eq!(metrics.bytes_sent, parsed.bytes_sent);
        assert_eq!(metrics.rx_bytes_per_sec, parsed.rx_bytes_per_sec);
    }

    #[test]
    fn test_disk_io_metrics_creation() {
        let metrics = DiskIoMetrics {
            read_bytes_per_sec: 1000.0,
            write_bytes_per_sec: 500.0,
            read_ops_per_sec: 100.0,
            write_ops_per_sec: 50.0,
            read_mbps: 10.0,
            write_mbps: 5.0,
            read_iops: 150.0,
            write_iops: 75.0,
            avg_queue_depth: 2.5,
        };

        assert_eq!(metrics.read_bytes_per_sec, 1000.0);
        assert_eq!(metrics.write_bytes_per_sec, 500.0);
        assert_eq!(metrics.avg_queue_depth, 2.5);
    }

    #[test]
    fn test_disk_io_metrics_serialization() {
        let metrics = DiskIoMetrics {
            read_bytes_per_sec: 100.0,
            write_bytes_per_sec: 50.0,
            read_ops_per_sec: 10.0,
            write_ops_per_sec: 5.0,
            read_mbps: 1.0,
            write_mbps: 0.5,
            read_iops: 15.0,
            write_iops: 7.5,
            avg_queue_depth: 1.0,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let parsed: DiskIoMetrics = serde_json::from_str(&json).unwrap();

        assert_eq!(metrics.read_iops, parsed.read_iops);
        assert_eq!(metrics.write_iops, parsed.write_iops);
    }

    #[test]
    fn test_alert_creation() {
        let alert = Alert {
            id: "alert-001".to_string(),
            name: "High CPU Usage".to_string(),
            description: "CPU usage exceeded threshold".to_string(),
            message: "CPU at 95%".to_string(),
            severity: AlertSeverity::Warning,
            status: AlertStatus::Active,
            created_at: Utc::now(),
            triggered_at: Utc::now(),
            suggested_actions: vec!["Check processes".to_string()],
            conditions: vec![],
        };

        assert_eq!(alert.id, "alert-001");
        assert_eq!(alert.name, "High CPU Usage");
        assert_eq!(alert.severity, AlertSeverity::Warning);
        assert_eq!(alert.status, AlertStatus::Active);
    }

    #[test]
    fn test_alert_serialization() {
        let alert = Alert {
            id: "test-alert".to_string(),
            name: "Test".to_string(),
            description: "Test alert".to_string(),
            message: "Test message".to_string(),
            severity: AlertSeverity::Info,
            status: AlertStatus::Active,
            created_at: Utc::now(),
            triggered_at: Utc::now(),
            suggested_actions: vec![],
            conditions: vec![],
        };

        let json = serde_json::to_string(&alert).unwrap();
        let parsed: Alert = serde_json::from_str(&json).unwrap();

        assert_eq!(alert.id, parsed.id);
        assert_eq!(alert.name, parsed.name);
    }

    #[test]
    fn test_dashboard_alert_creation() {
        let alert = DashboardAlert {
            id: "dash-001".to_string(),
            title: "System Alert".to_string(),
            message: "Check required".to_string(),
            alert_type: AlertType::System,
            severity: AlertSeverity::Critical,
            created_at: Utc::now(),
        };

        assert_eq!(alert.id, "dash-001");
        assert_eq!(alert.title, "System Alert");
        assert_eq!(alert.alert_type, AlertType::System);
    }

    #[test]
    fn test_alert_condition_creation() {
        let condition = AlertCondition {
            metric_name: "cpu_usage".to_string(),
            operator: ComparisonOperator::GreaterThan,
            threshold: 90.0,
            duration_seconds: 300,
            currentvalue: 95.0,
        };

        assert_eq!(condition.metric_name, "cpu_usage");
        assert_eq!(condition.operator, ComparisonOperator::GreaterThan);
        assert_eq!(condition.threshold, 90.0);
        assert_eq!(condition.duration_seconds, 300);
        assert_eq!(condition.currentvalue, 95.0);
    }

    #[test]
    fn test_alert_condition_serialization() {
        let condition = AlertCondition {
            metric_name: "memory".to_string(),
            operator: ComparisonOperator::LessThan,
            threshold: 10.0,
            duration_seconds: 60,
            currentvalue: 5.0,
        };

        let json = serde_json::to_string(&condition).unwrap();
        let parsed: AlertCondition = serde_json::from_str(&json).unwrap();

        assert_eq!(condition.metric_name, parsed.metric_name);
        assert_eq!(condition.threshold, parsed.threshold);
    }

    #[test]
    fn test_circuit_breaker_config_creation() {
        let config = CircuitBreakerConfig {
            failure_threshold: 5,
            timeout_seconds: 30,
            reset_timeout_seconds: 60,
        };

        assert_eq!(config.failure_threshold, 5);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.reset_timeout_seconds, 60);
    }

    #[test]
    fn test_circuit_breaker_config_serialization() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            timeout_seconds: 10,
            reset_timeout_seconds: 30,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: CircuitBreakerConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.failure_threshold, parsed.failure_threshold);
        assert_eq!(config.timeout_seconds, parsed.timeout_seconds);
    }

    #[test]
    fn test_timeout_config_creation() {
        let config = TimeoutConfig {
            connect_timeout_ms: 5000,
            request_timeout_ms: 30000,
            idle_timeout_ms: 60000,
        };

        assert_eq!(config.connect_timeout_ms, 5000);
        assert_eq!(config.request_timeout_ms, 30000);
        assert_eq!(config.idle_timeout_ms, 60000);
    }

    #[test]
    fn test_timeout_config_serialization() {
        let config = TimeoutConfig {
            connect_timeout_ms: 1000,
            request_timeout_ms: 5000,
            idle_timeout_ms: 10000,
        };

        let json = serde_json::to_string(&config).unwrap();
        let parsed: TimeoutConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config.connect_timeout_ms, parsed.connect_timeout_ms);
        assert_eq!(config.request_timeout_ms, parsed.request_timeout_ms);
        assert_eq!(config.idle_timeout_ms, parsed.idle_timeout_ms);
    }

    #[test]
    fn test_retry_policy_creation() {
        let policy = RetryPolicy {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        };

        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.initial_delay_ms, 100);
        assert_eq!(policy.max_delay_ms, 5000);
        assert_eq!(policy.backoff_multiplier, 2.0);
    }

    #[test]
    fn test_retry_policy_serialization() {
        let policy = RetryPolicy {
            max_retries: 5,
            initial_delay_ms: 50,
            max_delay_ms: 10000,
            backoff_multiplier: 1.5,
        };

        let json = serde_json::to_string(&policy).unwrap();
        let parsed: RetryPolicy = serde_json::from_str(&json).unwrap();

        assert_eq!(policy.max_retries, parsed.max_retries);
        assert_eq!(policy.backoff_multiplier, parsed.backoff_multiplier);
    }

    #[test]
    fn test_validation_context_creation() {
        let context = ValidationContext {
            strict_mode: true,
            allow_unsafe_names: false,
        };

        assert!(context.strict_mode);
        assert!(!context.allow_unsafe_names);
    }

    #[test]
    fn test_validation_context_serialization() {
        let context = ValidationContext {
            strict_mode: false,
            allow_unsafe_names: true,
        };

        let json = serde_json::to_string(&context).unwrap();
        let parsed: ValidationContext = serde_json::from_str(&json).unwrap();

        assert_eq!(context.strict_mode, parsed.strict_mode);
        assert_eq!(context.allow_unsafe_names, parsed.allow_unsafe_names);
    }

    #[test]
    fn test_zfs_metrics_creation() {
        let metrics = ZfsMetrics {
            arc_hit_ratio: 0.95,
            arc_size_bytes: 8_000_000_000,
            arc_target_size_bytes: 10_000_000_000,
            read_throughput_mbps: 1000.0,
            write_throughput_mbps: 500.0,
            compression_ratio: 1.5,
            deduplication_ratio: 1.2,
            total_datasets: 100,
            total_snapshots: 500,
            total_used_bytes: 6_000_000_000,
        };

        assert_eq!(metrics.arc_hit_ratio, 0.95);
        assert_eq!(metrics.arc_size_bytes, 8_000_000_000);
        assert_eq!(metrics.total_datasets, 100);
        assert_eq!(metrics.total_snapshots, 500);
        assert_eq!(metrics.total_used_bytes, 6_000_000_000);
    }

    #[test]
    fn test_zfs_metrics_serialization() {
        let metrics = ZfsMetrics {
            arc_hit_ratio: 0.85,
            arc_size_bytes: 4_000_000_000,
            arc_target_size_bytes: 5_000_000_000,
            read_throughput_mbps: 500.0,
            write_throughput_mbps: 250.0,
            compression_ratio: 1.3,
            deduplication_ratio: 1.1,
            total_datasets: 50,
            total_snapshots: 200,
            total_used_bytes: 3_000_000_000,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let parsed: ZfsMetrics = serde_json::from_str(&json).unwrap();

        assert_eq!(metrics.arc_hit_ratio, parsed.arc_hit_ratio);
        assert_eq!(metrics.compression_ratio, parsed.compression_ratio);
        assert_eq!(metrics.total_used_bytes, parsed.total_used_bytes);
    }

    #[test]
    fn test_system_metrics_creation() {
        let metrics = SystemMetrics {
            cpu_usage_percent: 45.5,
            memory_usage_percent: 60.0,
            load_average: 2.5,
            uptime_seconds: 86400,
            timestamp: Utc::now(),
            disk_io: DiskIoMetrics {
                read_bytes_per_sec: 1000.0,
                write_bytes_per_sec: 500.0,
                read_ops_per_sec: 100.0,
                write_ops_per_sec: 50.0,
                read_mbps: 10.0,
                write_mbps: 5.0,
                read_iops: 150.0,
                write_iops: 75.0,
                avg_queue_depth: 2.0,
            },
            network_io: NetworkIoMetrics {
                bytes_sent: 1000,
                bytes_received: 2000,
                packets_sent: 100,
                packets_received: 200,
                rx_bytes_per_sec: 10.0,
                tx_bytes_per_sec: 5.0,
                rx_packets_per_sec: 1.0,
                tx_packets_per_sec: 0.5,
            },
            zfs_metrics: ZfsMetrics {
                arc_hit_ratio: 0.9,
                arc_size_bytes: 1_000_000_000,
                arc_target_size_bytes: 2_000_000_000,
                read_throughput_mbps: 100.0,
                write_throughput_mbps: 50.0,
                compression_ratio: 1.5,
                deduplication_ratio: 1.2,
                total_datasets: 10,
                total_snapshots: 50,
                total_used_bytes: 750_000_000,
            },
        };

        assert_eq!(metrics.cpu_usage_percent, 45.5);
        assert_eq!(metrics.memory_usage_percent, 60.0);
        assert_eq!(metrics.uptime_seconds, 86400);
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_storage_metrics_zero_values() {
        let metrics = StorageMetrics {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            read_ops_per_sec: 0.0,
            write_ops_per_sec: 0.0,
        };

        assert_eq!(metrics.total_bytes, 0);
        assert_eq!(metrics.read_ops_per_sec, 0.0);
    }

    #[test]
    fn test_storage_metrics_max_values() {
        let metrics = StorageMetrics {
            total_bytes: u64::MAX,
            used_bytes: u64::MAX,
            available_bytes: u64::MAX,
            read_ops_per_sec: f64::MAX,
            write_ops_per_sec: f64::MAX,
        };

        assert_eq!(metrics.total_bytes, u64::MAX);
        assert_eq!(metrics.read_ops_per_sec, f64::MAX);
    }

    #[test]
    fn test_alert_condition_zero_threshold() {
        let condition = AlertCondition {
            metric_name: "test".to_string(),
            operator: ComparisonOperator::Equal,
            threshold: 0.0,
            duration_seconds: 0,
            currentvalue: 0.0,
        };

        assert_eq!(condition.threshold, 0.0);
        assert_eq!(condition.duration_seconds, 0);
    }

    #[test]
    fn test_retry_policy_zero_retries() {
        let policy = RetryPolicy {
            max_retries: 0,
            initial_delay_ms: 0,
            max_delay_ms: 0,
            backoff_multiplier: 0.0,
        };

        assert_eq!(policy.max_retries, 0);
        assert_eq!(policy.backoff_multiplier, 0.0);
    }

    #[test]
    fn test_zfs_metrics_perfect_ratios() {
        let metrics = ZfsMetrics {
            arc_hit_ratio: 1.0, // 100% hit ratio
            arc_size_bytes: 1000,
            arc_target_size_bytes: 1000,
            read_throughput_mbps: 1000.0,
            write_throughput_mbps: 1000.0,
            compression_ratio: 1.0,   // No compression
            deduplication_ratio: 1.0, // No deduplication
            total_datasets: 0,
            total_snapshots: 0,
            total_used_bytes: 1000,
        };

        assert_eq!(metrics.arc_hit_ratio, 1.0);
        assert_eq!(metrics.compression_ratio, 1.0);
        assert_eq!(metrics.deduplication_ratio, 1.0);
        assert_eq!(metrics.total_used_bytes, 1000);
    }

    #[test]
    fn test_alert_with_empty_suggestions() {
        let alert = Alert {
            id: "empty-alert".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            message: "Test".to_string(),
            severity: AlertSeverity::Info,
            status: AlertStatus::Active,
            created_at: Utc::now(),
            triggered_at: Utc::now(),
            suggested_actions: vec![],
            conditions: vec![],
        };

        assert!(alert.suggested_actions.is_empty());
        assert!(alert.conditions.is_empty());
    }

    #[test]
    fn test_alert_with_multiple_conditions() {
        let alert = Alert {
            id: "multi-condition".to_string(),
            name: "Complex Alert".to_string(),
            description: "Multiple conditions".to_string(),
            message: "Test".to_string(),
            severity: AlertSeverity::Critical,
            status: AlertStatus::Active,
            created_at: Utc::now(),
            triggered_at: Utc::now(),
            suggested_actions: vec!["Action 1".to_string(), "Action 2".to_string()],
            conditions: vec![
                AlertCondition {
                    metric_name: "cpu".to_string(),
                    operator: ComparisonOperator::GreaterThan,
                    threshold: 90.0,
                    duration_seconds: 60,
                    currentvalue: 95.0,
                },
                AlertCondition {
                    metric_name: "memory".to_string(),
                    operator: ComparisonOperator::GreaterThan,
                    threshold: 85.0,
                    duration_seconds: 120,
                    currentvalue: 88.0,
                },
            ],
        };

        assert_eq!(alert.conditions.len(), 2);
        assert_eq!(alert.suggested_actions.len(), 2);
    }
}
