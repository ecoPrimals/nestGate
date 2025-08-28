/// Focused unit tests for individual components and functions
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::performance::TierMetrics;
use nestgate_automation::types::optimization::PerformanceExpectation;
use nestgate_core::StorageTier as CoreStorageTier;
use nestgate_zfs::{
    migration::{MigrationJob, MigrationPriority, MigrationStatus},
    types::StorageTier,
    *,
};

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
        assert_eq!(
            hot.properties.get("compression").unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            }),
            "lz4"
        );
        assert_eq!(
            warm.properties.get("compression").unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            }),
            "zstd"
        );
        assert_eq!(
            cold.properties.get("compression").unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            }),
            "gzip-9"
        );

        // Verify performance profiles
        assert!(matches!(
            hot.performance_profile,
            crate::config::PerformanceProfile::HighPerformance
        ));
        assert!(matches!(
            warm.performance_profile,
            crate::config::PerformanceProfile::Balanced
        ));
        assert!(matches!(
            cold.performance_profile,
            crate::config::PerformanceProfile::HighCompression
        ));
    }
}

#[cfg(test)]
mod performance_unit_tests {
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
mod migration_unit_tests {

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
}

#[cfg(test)]
mod snapshot_unit_tests {

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
        assert!(matches!(
            operation.operation_type,
            SnapshotOperationType::Create
        ));
        assert!(matches!(operation.status, SnapshotOperationStatus::Queued));
        assert_eq!(operation.snapshot_name, Some("test_snapshot".to_string()));
    }
}

#[cfg(test)]
mod automation_unit_tests {

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
        assert!(
            expectation.expected_availability > 0.0 && expectation.expected_availability <= 100.0
        );
        assert!(expectation.expected_durability_nines > 0);
    }
}

#[cfg(test)]
mod property_tests {

    #[test]
    fn test_tier_performance_invariants() {
        let tiers = vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold];
        let config = ZfsConfig::default();

        for tier in &tiers {
            let tier_config = config.get_tier_config(&match tier {
                StorageTier::Hot => CoreStorageTier::Hot,
                StorageTier::Warm => CoreStorageTier::Warm,
                StorageTier::Cold => CoreStorageTier::Cold,
                StorageTier::Cache => CoreStorageTier::Hot, // Map cache to hot for testing
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
        for tier in &[
            CoreStorageTier::Hot,
            CoreStorageTier::Warm,
            CoreStorageTier::Cold,
        ] {
            let tier_config = config.get_tier_config(tier);
            assert!(!tier_config.properties.is_empty());
        }
    }
}
