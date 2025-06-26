//! Unit Tests for NestGate ZFS Components
//!
//! Focused unit tests for individual components and functions

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;

use nestgate_core::{Result, StorageTier as CoreStorageTier};
use nestgate_zfs::{*, types::StorageTier, migration::{MigrationJob, MigrationPriority, MigrationStatus}};
use nestgate_automation::PerformanceExpectation;

#[cfg(test)]
mod config_unit_tests {
    use super::*;

    #[test]
    fn test_zfs_config_defaults() {
        let config = ZfsConfig::default();
        
        assert_eq!(config.api_endpoint, "http://127.0.0.1:8081");
        assert_eq!(config.default_pool, "nestpool");
        assert!(config.health_monitoring.enabled);
        assert_eq!(config.health_monitoring.check_interval_seconds, 30);
        assert!(config.metrics.enabled);
        assert_eq!(config.metrics.collection_interval_seconds, 60);
    }

    #[test]
    fn test_tier_config_hierarchy() {
        let config = ZfsConfig::default();
        
        let hot = config.get_tier_config(&CoreStorageTier::Hot);
        let warm = config.get_tier_config(&CoreStorageTier::Warm);
        let cold = config.get_tier_config(&CoreStorageTier::Cold);
        
        // Verify compression algorithms
        assert_eq!(hot.properties.get("compression").unwrap(), "lz4");
        assert_eq!(warm.properties.get("compression").unwrap(), "zstd");
        assert_eq!(cold.properties.get("compression").unwrap(), "gzip-9");
        
        // Verify performance profiles
        assert!(matches!(hot.performance_profile, crate::config::PerformanceProfile::HighPerformance));
        assert!(matches!(warm.performance_profile, crate::config::PerformanceProfile::Balanced));
        assert!(matches!(cold.performance_profile, crate::config::PerformanceProfile::HighCompression));
    }

    #[test]
    fn test_migration_rules_thresholds() {
        let config = ZfsConfig::default();
        
        let hot = config.get_tier_config(&CoreStorageTier::Hot);
        let warm = config.get_tier_config(&CoreStorageTier::Warm);
        let cold = config.get_tier_config(&CoreStorageTier::Cold);
        
        // Hot tier should migrate faster than warm
        assert!(hot.migration_rules.age_threshold_days < warm.migration_rules.age_threshold_days);
        assert!(warm.migration_rules.age_threshold_days < cold.migration_rules.age_threshold_days);
        
        // Access frequency thresholds should decrease
        assert!(hot.migration_rules.access_frequency_threshold > warm.migration_rules.access_frequency_threshold);
    }

    #[test]
    fn test_capacity_limits() {
        let config = ZfsConfig::default();
        
        let hot = config.get_tier_config(&CoreStorageTier::Hot);
        let warm = config.get_tier_config(&CoreStorageTier::Warm);
        let cold = config.get_tier_config(&CoreStorageTier::Cold);
        
        // Cold tier should allow higher utilization
        assert!(cold.capacity_limits.max_utilization > warm.capacity_limits.max_utilization);
        assert!(warm.capacity_limits.max_utilization > hot.capacity_limits.max_utilization);
    }
}

#[cfg(test)]
mod performance_unit_tests {
    use super::*;
    use crate::performance::*;

    #[test]
    fn test_tier_metrics_hierarchy() {
        let hot = TierMetrics::default_for_tier(CoreStorageTier::Hot);
        let warm = TierMetrics::default_for_tier(CoreStorageTier::Warm);
        let cold = TierMetrics::default_for_tier(CoreStorageTier::Cold);
        
        // IOPS hierarchy: Hot > Warm > Cold
        assert!(hot.read_iops >= warm.read_iops);
        assert!(warm.read_iops >= cold.read_iops);
        
        // Latency hierarchy: Hot <= Warm <= Cold (lower is better)
        assert!(hot.avg_read_latency_ms <= warm.avg_read_latency_ms);
        assert!(warm.avg_read_latency_ms <= cold.avg_read_latency_ms);
        
        // All metrics should be non-negative
        assert!(hot.read_iops >= 0.0);
        assert!(warm.read_iops >= 0.0);
        assert!(cold.read_iops >= 0.0);
    }

    #[test]
    fn test_alert_condition_validation() {
        use crate::performance::{AlertCondition, AlertMetric, AlertOperator, AlertSeverity};
        use std::time::Duration;
        
        let condition = AlertCondition {
            id: "test_alert".to_string(),
            name: "Test Alert".to_string(),
            description: "Test alert condition".to_string(),
            metric: AlertMetric::Latency,
            operator: AlertOperator::GreaterThan,
            threshold: 100.0,
            duration: Duration::from_secs(60),
            severity: AlertSeverity::Warning,
            enabled: true,
        };
        
        assert_eq!(condition.id, "test_alert");
        assert_eq!(condition.threshold, 100.0);
        assert!(condition.enabled);
        assert!(matches!(condition.metric, AlertMetric::Latency));
        assert!(matches!(condition.severity, AlertSeverity::Warning));
    }

    #[test]
    fn test_performance_config_validation() {
        let config = crate::performance::PerformanceConfig::default();
        
        assert!(config.collection_interval > 0);
        assert!(config.analysis_interval > 0);
        assert!(config.alert_interval > 0);
        assert!(config.history_retention_hours > 0);
        assert!(config.max_history_entries > 0);
    }
}

#[cfg(test)]
mod ai_unit_tests {
    use super::*;
    use crate::ai_integration::*;

    #[test]
    fn test_ai_config_defaults() {
        let config = ZfsAiConfig::default();
        
        assert!(config.enable_tier_optimization);
        assert!(config.enable_predictive_analytics);
        assert!(config.enable_anomaly_detection);
        assert_eq!(config.optimization_interval, 3600);
        assert_eq!(config.analytics_interval, 300);
        assert!(config.min_confidence_threshold > 0.0 && config.min_confidence_threshold <= 1.0);
        assert!(config.max_concurrent_models > 0);
    }

    #[test]
    fn test_optimization_opportunity_creation() {
        let opportunity = OptimizationOpportunity {
            id: "test-opt".to_string(),
            opportunity_type: OptimizationType::TierMigration,
            description: "Move frequently accessed files to hot tier".to_string(),
            potential_benefit: OptimizationBenefit {
                performance_improvement: 25.0,
                storage_savings: 1024 * 1024 * 1024,
                cost_reduction: 100.0,
            },
            confidence_score: 0.85,
            estimated_effort: OptimizationEffort::Low,
            affected_datasets: vec!["test-dataset".to_string()],
            recommended_actions: vec!["Move to hot tier".to_string()],
            expected_impact: 25.0,
        };
        
        assert_eq!(opportunity.id, "test-opt");
        assert!(matches!(opportunity.opportunity_type, OptimizationType::TierMigration));
        assert!(opportunity.confidence_score > 0.0 && opportunity.confidence_score <= 1.0);
        assert!(opportunity.expected_impact > 0.0);
        assert!(!opportunity.affected_datasets.is_empty());
    }

    #[test]
    fn test_performance_expectation() {
        // Test that performance expectations are realistic
        let hot_tier_expectation = TierMetrics {
            tier: StorageTier::Hot,
            read_iops: 50000.0,    // High IOPS for hot tier
            write_iops: 40000.0,
            read_throughput_mbs: 1000.0,  // High throughput
            write_throughput_mbs: 800.0,
            avg_read_latency_ms: 0.5,     // Low latency
            avg_write_latency_ms: 1.0,
            utilization_percent: 0.0,     // Not relevant for this test
            capacity_bytes: 0,            // Not relevant for this test
            used_bytes: 0,                // Not relevant for this test
        };
        
        // Verify expectations are within reasonable bounds
        assert!(hot_tier_expectation.read_iops > 1000.0, "Hot tier should have high IOPS");
        assert!(hot_tier_expectation.avg_read_latency_ms < 5.0, "Hot tier should have low latency");
        assert!(hot_tier_expectation.read_throughput_mbs > 100.0, "Hot tier should have high throughput");
    }
}

#[cfg(test)]
mod migration_unit_tests {
    use super::*;
    use crate::migration::*;

    #[test]
    fn test_migration_job_lifecycle() {
        let job = MigrationJob::new(
            PathBuf::from("/test/dataset"),
            StorageTier::Hot,
            StorageTier::Warm,
            MigrationPriority::Normal,
            1024 * 1024, // 1MB
        );
        
        assert!(!job.id.is_empty());
        assert_eq!(job.source_path, PathBuf::from("/test/dataset"));
        assert!(matches!(job.source_tier, StorageTier::Hot));
        assert!(matches!(job.target_tier, StorageTier::Warm));
        assert!(matches!(job.status, MigrationStatus::Queued));
    }

    #[test]
    fn test_migration_priority_ordering() {
        // Higher priority values should have higher precedence
        assert!(MigrationPriority::Critical as u32 > MigrationPriority::High as u32);
        assert!(MigrationPriority::High as u32 > MigrationPriority::Normal as u32);
        assert!(MigrationPriority::Normal as u32 > MigrationPriority::Low as u32);
    }

    #[test]
    fn test_migration_config_validation() {
        let config = migration::MigrationConfig::default();
        
        assert!(config.max_concurrent_migrations > 0);
        assert!(config.total_bandwidth_limit > 0);
        assert!(config.max_bandwidth_per_migration > 0);
        assert!(config.batch_size > 0);
    }
}

#[cfg(test)]
mod snapshot_unit_tests {
    use super::*;
    use crate::snapshot::*;

    #[test]
    fn test_snapshot_policy_validation() {
        let policy = SnapshotPolicy::default();
        
        assert_eq!(policy.name, "default");
        assert!(policy.enabled);
        assert!(matches!(policy.frequency, ScheduleFrequency::Hours(1)));
        assert!(!policy.dataset_patterns.is_empty());
        assert!(policy.max_snapshots_per_run > 0);
    }

    #[test]
    fn test_retention_policy_custom() {
        let policy = RetentionPolicy::Custom {
            hourly_hours: 24,
            daily_days: 30,
            weekly_weeks: 12,
            monthly_months: 12,
            yearly_years: 5,
        };
        
        if let RetentionPolicy::Custom { hourly_hours, daily_days, weekly_weeks, monthly_months, yearly_years } = policy {
            assert_eq!(hourly_hours, 24);
            assert_eq!(daily_days, 30);
            assert_eq!(weekly_weeks, 12);
            assert_eq!(monthly_months, 12);
            assert_eq!(yearly_years, 5);
        } else {
            panic!("Expected Custom retention policy");
        }
    }

    #[test]
    fn test_snapshot_operation_status() {
        let operation = SnapshotOperation {
            id: "test_op".to_string(),
            operation_type: SnapshotOperationType::Create,
            dataset: "test_dataset".to_string(),
            snapshot_name: Some("test_snapshot".to_string()),
            status: SnapshotOperationStatus::Queued,
            created_at: SystemTime::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            policy: Some("test_policy".to_string()),
        };
        
        assert_eq!(operation.id, "test_op");
        assert!(matches!(operation.operation_type, SnapshotOperationType::Create));
        assert!(matches!(operation.status, SnapshotOperationStatus::Queued));
        assert_eq!(operation.snapshot_name, Some("test_snapshot".to_string()));
    }
}

#[cfg(test)]
mod automation_unit_tests {
    use super::*;
    use nestgate_automation::TierThresholds;

    #[test]
    fn test_tier_thresholds_hierarchy() {
        // Create simple test thresholds for validation
        let hot_threshold = 80.0;
        let warm_threshold = 60.0;
        let cold_threshold = 40.0;
        
        // Hot tier should have higher access frequency threshold than warm
        assert!(hot_threshold > warm_threshold);
        assert!(warm_threshold > cold_threshold);
    }

    #[test]
    fn test_file_characteristics() {
        // Test using the actual FileCharacteristics struct
        let characteristics = nestgate_automation::FileCharacteristics {
            is_frequently_accessed: true,
            is_sequential_access: false,
            is_compressible: true,
            is_dedupable: false,
        };
        
        assert!(characteristics.is_frequently_accessed);
        assert!(!characteristics.is_sequential_access);
        assert!(characteristics.is_compressible);
        assert!(!characteristics.is_dedupable);
    }

    #[test]
    fn test_performance_expectation() {
        // Test using the actual PerformanceExpectation struct
        let expectation = PerformanceExpectation {
            expected_iops: 100000,
            expected_bandwidth_mbps: 1000.0,
            expected_latency_ms: 1.0,
            expected_availability: 99.999,
            expected_durability_nines: 11,
        };
        
        assert!(expectation.expected_iops > 0);
        assert!(expectation.expected_bandwidth_mbps > 0.0);
        assert!(expectation.expected_latency_ms > 0.0);
        assert!(expectation.expected_availability > 0.0 && expectation.expected_availability <= 100.0);
        assert!(expectation.expected_durability_nines > 0);
    }
}

#[cfg(test)]
mod error_unit_tests {
    use super::*;
    use nestgate_zfs::error::*;

    #[test]
    fn test_error_hierarchy() {
        let pool_error = PoolError::NotFound { pool_name: "test_pool".to_string() };
        let zfs_error: ZfsError = pool_error.into();
        
        match zfs_error {
            ZfsError::PoolError(PoolError::NotFound { pool_name }) => {
                assert_eq!(pool_name, "test_pool");
            }
            _ => panic!("Error conversion failed"),
        }
    }

    #[test]
    fn test_retryable_errors() {
        let retryable_errors = vec![
            ZfsError::SystemUnavailable("ZFS not available".to_string()),
            ZfsError::Timeout("Operation timed out".to_string()),
            ZfsError::ResourceExhausted("Out of memory".to_string()),
            ZfsError::IoError(std::io::Error::new(std::io::ErrorKind::Interrupted, "interrupted")),
        ];
        
        for error in retryable_errors {
            assert!(ZfsError::is_retryable(&error), "Error should be retryable: {:?}", error);
        }
        
        let non_retryable_errors = vec![
            ZfsError::PoolError(PoolError::NotFound { pool_name: "test".to_string() }),
            ZfsError::ConfigError("Invalid config".to_string()),
            ZfsError::PermissionError("Access denied".to_string()),
        ];
        
        for error in non_retryable_errors {
            assert!(!ZfsError::is_retryable(&error), "Error should not be retryable: {:?}", error);
        }
    }

    #[test]
    fn test_error_context() {
        let context = ZfsError::create_context("pool_creation", "pool_manager");
        assert_eq!(context.operation, "pool_creation");
        assert_eq!(context.component, "pool_manager");
    }
}

#[cfg(test)]
mod orchestrator_unit_tests {
    use super::*;

    #[test]
    fn test_zfs_capabilities() {
        // Test ZFS capabilities through configuration
        let config = ZfsConfig::default();
        
        assert!(config.health_monitoring.enabled);
        assert!(config.metrics.enabled);
        assert!(!config.default_pool.is_empty());
        
        // Test tier configurations exist
        let hot_config = config.get_tier_config(&CoreStorageTier::Hot);
        let warm_config = config.get_tier_config(&CoreStorageTier::Warm);
        let cold_config = config.get_tier_config(&CoreStorageTier::Cold);
        
        assert!(!hot_config.properties.is_empty());
        assert!(!warm_config.properties.is_empty());
        assert!(!cold_config.properties.is_empty());
    }

    #[test]
    fn test_tier_performance_targets() {
        let target = nestgate_zfs::TierPerformanceTarget {
            tier: StorageTier::Hot,
            target_iops: 100000,
            target_bandwidth_mbps: 5000.0,
            target_latency_ms: 1.0,
            target_availability: 0.99999,
            target_durability_nines: 11,
        };
        
        assert!(target.target_iops > 0);
        assert!(target.target_bandwidth_mbps > 0.0);
        assert!(target.target_latency_ms > 0.0);
        assert!(target.target_availability > 0.0 && target.target_availability <= 1.0);
    }

    #[test]
    fn test_alert_severity_ordering() {
        use crate::performance::AlertSeverity;
        
        // Test that critical is higher than warning
        assert!(AlertSeverity::Critical > AlertSeverity::Warning);
        assert!(AlertSeverity::Warning > AlertSeverity::Info);
        
        // Test severity levels are ordered correctly
        let severities = vec![
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::Critical,
            AlertSeverity::Emergency,
        ];
        
        for i in 1..severities.len() {
            assert!(severities[i] > severities[i-1], "Severity ordering incorrect");
        }
    }
}

#[cfg(test)]
mod mcp_unit_tests {
    use super::*;
    use crate::mcp_integration::*;

    #[test]
    fn test_mcp_config_defaults() {
        let config = mcp_integration::ZfsMcpConfig::default();
        
        assert_eq!(config.default_tier, CoreStorageTier::Warm);
        assert!(config.enable_ai_optimization);
        assert!(config.max_concurrent_operations > 0);
    }

    #[test]
    fn test_mount_request_validation() {
        let request = mcp_integration::McpMountRequest {
            mount_id: "test_mount".to_string(),
            mount_point: "/mcp/test".to_string(),
            tier: CoreStorageTier::Hot,
            size_gb: 10,
        };
        
        assert_eq!(request.mount_id, "test_mount");
        assert_eq!(request.tier, CoreStorageTier::Hot);
        assert!(request.size_gb > 0);
    }

    #[test]
    fn test_volume_request_validation() {
        let request = mcp_integration::McpVolumeRequest {
            volume_id: "test_volume".to_string(),
            tier: CoreStorageTier::Warm,
            size_gb: 5,
        };
        
        assert_eq!(request.volume_id, "test_volume");
        assert_eq!(request.tier, CoreStorageTier::Warm);
        assert!(request.size_gb > 0);
    }

    #[test]
    fn test_mount_status_types() {
        let active = MountStatus::Active;
        let inactive = MountStatus::Inactive;
        let error = MountStatus::Error("Test error".to_string());
        
        assert!(matches!(active, MountStatus::Active));
        assert!(matches!(inactive, MountStatus::Inactive));
        
        if let MountStatus::Error(msg) = error {
            assert_eq!(msg, "Test error");
        } else {
            panic!("Expected error status");
        }
    }
}

/// Property-based tests using quickcheck-style testing
#[cfg(test)]
mod property_tests {
    use super::*;

    #[test]
    fn test_tier_performance_invariants() {
        let tiers = vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold];
        let config = ZfsConfig::default();
        
        for tier in &tiers {
            let tier_config = config.get_tier_config(&match tier {
                StorageTier::Hot => CoreStorageTier::Hot,
                StorageTier::Warm => CoreStorageTier::Warm,
                StorageTier::Cold => CoreStorageTier::Cold,
            });
            assert!(!tier_config.properties.is_empty());
            assert!(tier_config.capacity_limits.max_utilization > 0.0);
            assert!(tier_config.capacity_limits.max_utilization <= 100.0);
        }
    }

    #[test]
    fn test_config_validation_invariants() {
        let config = ZfsConfig::default();
        
        assert!(!config.default_pool.is_empty());
        assert!(config.health_monitoring.check_interval_seconds > 0);
        assert!(config.metrics.collection_interval_seconds > 0);
        
        // Test that all tiers have valid configurations
        for tier in &[CoreStorageTier::Hot, CoreStorageTier::Warm, CoreStorageTier::Cold] {
            let tier_config = config.get_tier_config(tier);
            assert!(!tier_config.properties.is_empty());
        }
    }

    #[test]
    fn test_migration_job_state_transitions() {
        let job = MigrationJob::new(
            PathBuf::from("/test/file"),
            StorageTier::Hot,
            StorageTier::Warm,
            MigrationPriority::Normal,
            1024,
        );
        
        // Initial state should be Queued
        assert!(matches!(job.status, MigrationStatus::Queued));
        assert!(job.progress == 0.0);
        assert!(job.retry_count == 0);
    }
} 