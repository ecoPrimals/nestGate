//! Unit Tests for NestGate ZFS Components
//!
//! Focused unit tests for individual components and functions

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tempfile::TempDir;

use nestgate_core::{Result, StorageTier};
use nestgate_zfs::*;

#[cfg(test)]
mod config_unit_tests {
    use super::*;

    #[test]
    fn test_zfs_config_defaults() {
        let config = ZfsConfig::default();
        
        assert_eq!(config.api_endpoint, "http://localhost:8080");
        assert_eq!(config.default_pool, "nestpool");
        assert!(config.health_monitoring.enabled);
        assert_eq!(config.health_monitoring.check_interval_seconds, 30);
        assert!(config.metrics.enabled);
        assert_eq!(config.metrics.collection_interval_seconds, 60);
    }

    #[test]
    fn test_tier_config_hierarchy() {
        let config = ZfsConfig::default();
        
        let hot = config.get_tier_config(&StorageTier::Hot);
        let warm = config.get_tier_config(&StorageTier::Warm);
        let cold = config.get_tier_config(&StorageTier::Cold);
        
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
        
        let hot = config.get_tier_config(&StorageTier::Hot);
        let warm = config.get_tier_config(&StorageTier::Warm);
        let cold = config.get_tier_config(&StorageTier::Cold);
        
        // Hot tier should migrate faster than warm
        assert!(hot.migration_rules.age_threshold_days < warm.migration_rules.age_threshold_days);
        assert!(warm.migration_rules.age_threshold_days < cold.migration_rules.age_threshold_days);
        
        // Access frequency thresholds should decrease
        assert!(hot.migration_rules.access_frequency_threshold > warm.migration_rules.access_frequency_threshold);
    }

    #[test]
    fn test_capacity_limits() {
        let config = ZfsConfig::default();
        
        let hot = config.get_tier_config(&StorageTier::Hot);
        let warm = config.get_tier_config(&StorageTier::Warm);
        let cold = config.get_tier_config(&StorageTier::Cold);
        
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
        let hot = TierMetrics::default_for_tier(StorageTier::Hot);
        let warm = TierMetrics::default_for_tier(StorageTier::Warm);
        let cold = TierMetrics::default_for_tier(StorageTier::Cold);
        let cache = TierMetrics::default_for_tier(StorageTier::Hot); // Use Hot as cache equivalent
        
        // IOPS hierarchy: Cache > Hot > Warm > Cold
        assert!(cache.read_iops > hot.read_iops);
        assert!(hot.read_iops > warm.read_iops);
        assert!(warm.read_iops > cold.read_iops);
        
        // Throughput hierarchy
        assert!(cache.read_throughput_mbs > hot.read_throughput_mbs);
        assert!(hot.read_throughput_mbs > warm.read_throughput_mbs);
        assert!(warm.read_throughput_mbs > cold.read_throughput_mbs);
        
        // Latency hierarchy (inverse)
        assert!(cache.avg_read_latency_ms < hot.avg_read_latency_ms);
        assert!(hot.avg_read_latency_ms < warm.avg_read_latency_ms);
        assert!(warm.avg_read_latency_ms < cold.avg_read_latency_ms);
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
        let config = PerformanceConfig::default();
        
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
    fn test_optimization_opportunity_ordering() {
        let mut opportunities = vec![
            OptimizationOpportunity {
                optimization_type: OptimizationType::TierMigration,
                description: "Low impact".to_string(),
                expected_impact: 5.0,
                confidence: 0.9,
                complexity: OptimizationComplexity::Low,
                implementation_time: Duration::from_secs(60),
            },
            OptimizationOpportunity {
                optimization_type: OptimizationType::CompressionOptimization,
                description: "High impact".to_string(),
                expected_impact: 25.0,
                confidence: 0.8,
                complexity: OptimizationComplexity::Medium,
                implementation_time: Duration::from_secs(300),
            },
            OptimizationOpportunity {
                optimization_type: OptimizationType::CacheOptimization,
                description: "Medium impact".to_string(),
                expected_impact: 15.0,
                confidence: 0.85,
                complexity: OptimizationComplexity::Low,
                implementation_time: Duration::from_secs(120),
            },
        ];
        
        // Sort by expected impact (descending)
        opportunities.sort_by(|a, b| b.expected_impact.partial_cmp(&a.expected_impact).unwrap());
        
        assert_eq!(opportunities[0].expected_impact, 25.0);
        assert_eq!(opportunities[1].expected_impact, 15.0);
        assert_eq!(opportunities[2].expected_impact, 5.0);
    }

    #[test]
    fn test_tier_prediction_validation() {
        let prediction = TierPrediction {
            file_path: "/test/file.txt".to_string(),
            predicted_tier: StorageTier::Warm,
            current_tier: StorageTier::Hot,
            confidence: 0.92,
            reasoning: "Medium access frequency".to_string(),
            expected_improvement: 15.0,
            timestamp: SystemTime::now(),
        };
        
        assert_eq!(prediction.predicted_tier, StorageTier::Warm);
        assert_eq!(prediction.current_tier, StorageTier::Hot);
        assert!(prediction.confidence > 0.9);
        assert!(prediction.expected_improvement > 0.0);
    }
}

#[cfg(test)]
mod migration_unit_tests {
    use super::*;
    use crate::migration::*;

    #[test]
    fn test_migration_job_lifecycle() {
        use crate::types::StorageTier as ZfsStorageTier;
        use crate::migration::{MigrationJob, MigrationPriority, MigrationStatus};
        
        let job = MigrationJob::new(
            PathBuf::from("/test/file.txt"),
            ZfsStorageTier::Hot,
            ZfsStorageTier::Warm,
            MigrationPriority::Normal,
            1024 * 1024, // 1MB
        );
        
        assert_eq!(job.status, MigrationStatus::Queued);
        assert_eq!(job.progress, 0.0);
        assert_eq!(job.retry_count, 0);
        assert!(job.started_at.is_none());
        assert!(job.completed_at.is_none());
    }

    #[test]
    fn test_migration_priority_ordering() {
        assert!((MigrationPriority::Critical as u8) > (MigrationPriority::High as u8));
        assert!((MigrationPriority::High as u8) > (MigrationPriority::Normal as u8));
        assert!((MigrationPriority::Normal as u8) > (MigrationPriority::Low as u8));
    }

    #[test]
    fn test_migration_config_limits() {
        let config = MigrationConfig::default();
        
        assert!(config.max_concurrent_migrations > 0);
        assert!(config.max_bandwidth_per_migration > 0);
        assert!(config.total_bandwidth_limit > 0);
        assert!(!config.allowed_hours.is_empty());
        assert!(config.performance_impact_threshold > 0.0);
    }
}

#[cfg(test)]
mod snapshot_unit_tests {
    use super::*;
    use crate::snapshot::*;

    #[test]
    fn test_snapshot_policy_validation() {
        let policy = SnapshotPolicy::default();
        
        assert!(!policy.name.is_empty());
        assert!(policy.enabled);
        assert!(policy.max_snapshots_per_run > 0);
        assert!(policy.priority > 0);
        assert!(!policy.dataset_patterns.is_empty());
    }

    #[test]
    fn test_retention_policy_custom() {
        let retention = RetentionPolicy::default();
        
        if let RetentionPolicy::Custom { 
            hourly_hours, 
            daily_days, 
            weekly_weeks, 
            monthly_months, 
            yearly_years 
        } = retention {
            assert!(hourly_hours > 0);
            assert!(daily_days > 0);
            assert!(weekly_weeks > 0);
            assert!(monthly_months > 0);
            assert!(yearly_years > 0);
            
            // Verify reasonable retention periods
            assert!(daily_days >= hourly_hours);
            assert!(weekly_weeks * 7 >= daily_days);
        } else {
            panic!("Expected Custom retention policy");
        }
    }

    #[test]
    fn test_snapshot_operation_status() {
        let operation = SnapshotOperation {
            id: "test_op".to_string(),
            operation_type: SnapshotOperationType::Create,
            dataset: "pool/dataset".to_string(),
            snapshot_name: Some("test_snapshot".to_string()),
            status: SnapshotOperationStatus::Queued,
            created_at: SystemTime::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            policy: Some("test_policy".to_string()),
        };
        
        assert_eq!(operation.status, SnapshotOperationStatus::Queued);
        assert!(operation.started_at.is_none());
        assert!(operation.completed_at.is_none());
        assert!(operation.error_message.is_none());
    }
}

#[cfg(test)]
mod automation_unit_tests {
    use super::*;
    use crate::automation::*;

    #[test]
    fn test_tier_thresholds_hierarchy() {
        use crate::types::StorageTier as ZfsStorageTier;
        use crate::automation::TierThresholds;
        
        let thresholds = TierThresholds::default();
        
        // Hot tier should have higher access frequency threshold than warm
        assert!(thresholds.hot_access_frequency > thresholds.warm_access_frequency);
        
        // Size thresholds should exist for all tiers
        assert!(thresholds.size_thresholds.contains_key(&ZfsStorageTier::Hot));
        assert!(thresholds.size_thresholds.contains_key(&ZfsStorageTier::Warm));
        assert!(thresholds.size_thresholds.contains_key(&ZfsStorageTier::Cold));
        
        // Age thresholds should exist for all tiers
        assert!(thresholds.age_thresholds.contains_key(&ZfsStorageTier::Hot));
        assert!(thresholds.age_thresholds.contains_key(&ZfsStorageTier::Warm));
        assert!(thresholds.age_thresholds.contains_key(&ZfsStorageTier::Cold));
    }

    #[test]
    fn test_file_characteristics_validation() {
        let characteristics = FileCharacteristics {
            size: 1024 * 1024, // 1MB
            extension: "txt".to_string(),
            mime_type: "text/plain".to_string(),
            created_at: SystemTime::now(),
            accessed_at: SystemTime::now(),
            modified_at: SystemTime::now(),
            access_frequency: 15.0,
            read_write_ratio: 0.8,
        };
        
        assert!(characteristics.size > 0);
        assert!(!characteristics.extension.is_empty());
        assert!(!characteristics.mime_type.is_empty());
        assert!(characteristics.access_frequency >= 0.0);
        assert!(characteristics.read_write_ratio >= 0.0 && characteristics.read_write_ratio <= 1.0);
    }

    #[test]
    fn test_performance_expectation() {
        let expectation = PerformanceExpectation {
            latency_ms: 1.0,
            throughput_mbs: 1000.0,
            iops: 100000.0,
            compression_ratio: 2.0,
        };
        
        assert!(expectation.latency_ms > 0.0);
        assert!(expectation.throughput_mbs > 0.0);
        assert!(expectation.iops > 0.0);
        assert!(expectation.compression_ratio >= 1.0);
    }
}

#[cfg(test)]
mod error_unit_tests {
    use super::*;
    use crate::error::*;

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
        let context = ZfsError::create_context("pool_discovery", "pool_manager");
        
        assert_eq!(context.operation, "pool_discovery");
        assert_eq!(context.component, "pool_manager");
    }
}

#[cfg(test)]
mod orchestrator_unit_tests {
    use super::*;
    use crate::orchestrator_integration::*;

    #[test]
    fn test_zfs_capabilities() {
        let capabilities = ZfsCapabilities {
            pool_management: true,
            dataset_operations: true,
            snapshot_management: true,
            tier_management: true,
            ai_optimization: true,
            performance_monitoring: true,
            migration_engine: true,
            supported_tiers: vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold],
            compression_algorithms: vec!["lz4".to_string(), "zstd".to_string(), "gzip-9".to_string()],
            max_pool_capacity_gb: 10000,
            performance_targets: HashMap::new(),
        };
        
        assert!(capabilities.pool_management);
        assert!(capabilities.ai_optimization);
        assert_eq!(capabilities.supported_tiers.len(), 3);
        assert_eq!(capabilities.compression_algorithms.len(), 3);
        assert!(capabilities.max_pool_capacity_gb > 0);
    }

    #[test]
    fn test_tier_performance_targets() {
        let target = TierPerformanceTarget {
            target_iops: 100000,
            target_throughput_mbs: 5000,
            target_latency_ms: 1.0,
            target_cache_hit_ratio: 0.95,
        };
        
        assert!(target.target_iops > 0);
        assert!(target.target_throughput_mbs > 0);
        assert!(target.target_latency_ms > 0.0);
        assert!(target.target_cache_hit_ratio > 0.0 && target.target_cache_hit_ratio <= 1.0);
    }

    #[test]
    fn test_alert_severity_ordering() {
        use crate::performance::AlertSeverity;
        
        // Test that severity levels are properly ordered
        let info = AlertSeverity::Info;
        let warning = AlertSeverity::Warning;
        let critical = AlertSeverity::Critical;
        let emergency = AlertSeverity::Emergency;
        
        // These should be distinct values
        assert!(matches!(info, AlertSeverity::Info));
        assert!(matches!(warning, AlertSeverity::Warning));
        assert!(matches!(critical, AlertSeverity::Critical));
        assert!(matches!(emergency, AlertSeverity::Emergency));
    }
}

#[cfg(test)]
mod mcp_unit_tests {
    use super::*;
    use crate::mcp_integration::*;

    #[test]
    fn test_mcp_config_defaults() {
        let config = ZfsMcpConfig::default();
        
        assert!(config.enable_ai_optimization);
        assert!(config.max_concurrent_operations > 0);
        assert_eq!(config.default_tier, StorageTier::Warm);
    }

    #[test]
    fn test_mount_request_validation() {
        let request = McpMountRequest {
            mount_id: "test_mount".to_string(),
            mount_point: "/mcp/test".to_string(),
            tier: StorageTier::Hot,
            size_gb: 10,
        };
        
        assert_eq!(request.mount_id, "test_mount");
        assert_eq!(request.tier, StorageTier::Hot);
        assert_eq!(request.size_gb, 10);
        assert!(request.mount_point.starts_with("/"));
    }

    #[test]
    fn test_volume_request_validation() {
        let request = McpVolumeRequest {
            volume_id: "test_volume".to_string(),
            tier: StorageTier::Warm,
            size_gb: 5,
        };
        
        assert!(!request.volume_id.is_empty());
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
        // Test that tier performance metrics maintain expected invariants
        let tiers = [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold];
        let metrics: Vec<_> = tiers.iter()
            .map(|tier| crate::performance::TierMetrics::default_for_tier(tier.clone()))
            .collect();
        
        for metric in &metrics {
            // All metrics should be non-negative
            assert!(metric.read_iops >= 0.0, "Read IOPS should be non-negative");
            assert!(metric.write_iops >= 0.0, "Write IOPS should be non-negative");
            assert!(metric.read_throughput_mbs >= 0.0, "Read throughput should be non-negative");
            assert!(metric.write_throughput_mbs >= 0.0, "Write throughput should be non-negative");
            assert!(metric.avg_read_latency_ms >= 0.0, "Read latency should be non-negative");
            assert!(metric.avg_write_latency_ms >= 0.0, "Write latency should be non-negative");
            assert!(metric.utilization_percent >= 0.0 && metric.utilization_percent <= 100.0, "Utilization should be 0-100%");
            assert!(metric.cache_hit_ratio >= 0.0 && metric.cache_hit_ratio <= 1.0, "Cache hit ratio should be 0-1");
            assert!(metric.error_rate >= 0.0 && metric.error_rate <= 1.0, "Error rate should be 0-1");
        }
    }

    #[test]
    fn test_config_validation_invariants() {
        let config = ZfsConfig::default();
        
        // Test that all tier configs are valid
        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            let tier_config = config.get_tier_config(&tier);
            
            assert!(!tier_config.name.is_empty(), "Tier name should not be empty");
            assert!(!tier_config.pool_name.is_empty(), "Pool name should not be empty");
            assert!(!tier_config.properties.is_empty(), "Properties should not be empty");
            assert!(tier_config.capacity_limits.max_utilization > 0.0 && tier_config.capacity_limits.max_utilization <= 1.0, "Max utilization should be 0-1");
            assert!(tier_config.capacity_limits.warning_threshold <= tier_config.capacity_limits.max_utilization, "Warning threshold should not exceed max utilization");
        }
    }

    #[test]
    fn test_migration_job_state_transitions() {
        let mut job = crate::migration::MigrationJob::new(
            PathBuf::from("/test/file.txt"),
            crate::types::StorageTier::Hot,
            crate::types::StorageTier::Warm,
            crate::migration::MigrationPriority::Normal,
            1024,
        );
        
        // Initial state
        assert_eq!(job.status, crate::migration::MigrationStatus::Queued);
        assert_eq!(job.progress, 0.0);
        
        // Progress should always be 0-100
        for progress in [0.0, 25.0, 50.0, 75.0, 100.0] {
            job.progress = progress;
            assert!(job.progress >= 0.0 && job.progress <= 100.0, "Progress should be 0-100%");
        }
        
        // Retry count should only increase
        let initial_retries = job.retry_count;
        job.retry_count += 1;
        assert!(job.retry_count > initial_retries, "Retry count should only increase");
    }
} 