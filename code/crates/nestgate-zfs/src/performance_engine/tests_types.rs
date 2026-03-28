// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Tests for Performance Engine Types
//!
//! These tests cover the type definitions and basic operations
//! for the performance optimization engine.

#[cfg(test)]
mod performance_types_tests {
    use super::super::types::*;
    use std::collections::HashMap;
    use std::time::SystemTime;

    // ==================== OPTIMIZATION STATE TESTS ====================

    #[test]
    fn test_optimization_state_default() {
        let state = OptimizationState::default();
        assert_eq!(state, OptimizationState::Idle);
    }

    #[test]
    fn test_optimization_state_transitions() {
        let states = vec![
            OptimizationState::Idle,
            OptimizationState::Collecting,
            OptimizationState::Analyzing,
            OptimizationState::Optimizing,
            OptimizationState::Validating,
            OptimizationState::Applied,
        ];
        assert_eq!(states.len(), 6);
    }

    #[test]
    fn test_optimization_state_equality() {
        assert_eq!(OptimizationState::Idle, OptimizationState::Idle);
        assert_ne!(OptimizationState::Idle, OptimizationState::Collecting);
    }

    #[test]
    fn test_optimization_state_clone() {
        let state = OptimizationState::Analyzing;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    // ==================== ZFS PERFORMANCE METRICS TESTS ====================

    #[test]
    fn test_zfs_performance_metrics_creation() {
        let metrics = ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: HashMap::new(),
            dataset_metrics: HashMap::new(),
            system_memory: SystemMemoryUsage {
                total: 16_000_000_000,
                available: 8_000_000_000,
                used: 8_000_000_000,
            },
            arc_stats: ArcStatistics {
                size: 4_000_000_000,
                target_size: 4_000_000_000,
                hit_ratio: 0.85,
                miss_ratio: 0.15,
            },
        };

        assert!(metrics.pool_metrics.is_empty());
        assert!(metrics.dataset_metrics.is_empty());
        assert_eq!(metrics.system_memory.total, 16_000_000_000);
        assert_eq!(metrics.arc_stats.hit_ratio, 0.85);
    }

    #[test]
    fn test_zfs_performance_metrics_with_pools() {
        let mut pool_metrics = HashMap::new();
        pool_metrics.insert(
            "test_pool".to_string(),
            ZfsPoolMetrics {
                pool_name: "test_pool".to_string(),
                read_ops: 1000.0,
                write_ops: 800.0,
                read_bandwidth: 100_000_000.0,
                write_bandwidth: 80_000_000.0,
                latency: 2.5,
                cache_hit_ratio: 0.85,
                fragmentation: 15.0,
            },
        );

        let metrics = ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            dataset_metrics: HashMap::new(),
            system_memory: SystemMemoryUsage {
                total: 16_000_000_000,
                available: 8_000_000_000,
                used: 8_000_000_000,
            },
            arc_stats: ArcStatistics {
                size: 4_000_000_000,
                target_size: 4_000_000_000,
                hit_ratio: 0.85,
                miss_ratio: 0.15,
            },
        };

        assert_eq!(metrics.pool_metrics.len(), 1);
        assert!(metrics.pool_metrics.contains_key("test_pool"));
    }

    // ==================== ZFS POOL METRICS TESTS ====================

    #[test]
    fn test_zfs_pool_metrics_creation() {
        let pool = ZfsPoolMetrics {
            pool_name: "tank".to_string(),
            read_ops: 1500.0,
            write_ops: 1200.0,
            read_bandwidth: 150_000_000.0,
            write_bandwidth: 120_000_000.0,
            latency: 3.2,
            cache_hit_ratio: 0.90,
            fragmentation: 10.0,
        };

        assert_eq!(pool.pool_name, "tank");
        assert_eq!(pool.read_ops, 1500.0);
        assert!(pool.cache_hit_ratio >= 0.0 && pool.cache_hit_ratio <= 1.0);
    }

    #[test]
    fn test_zfs_pool_metrics_clone() {
        let pool = ZfsPoolMetrics {
            pool_name: "data".to_string(),
            read_ops: 2000.0,
            write_ops: 1800.0,
            read_bandwidth: 200_000_000.0,
            write_bandwidth: 180_000_000.0,
            latency: 1.8,
            cache_hit_ratio: 0.88,
            fragmentation: 12.5,
        };

        let cloned = pool.clone();
        assert_eq!(pool.pool_name, cloned.pool_name);
        assert_eq!(pool.read_ops, cloned.read_ops);
    }

    // ==================== ZFS DATASET METRICS TESTS ====================

    #[test]
    fn test_zfs_dataset_metrics_sequential() {
        let dataset = ZfsDatasetMetrics {
            dataset_name: "tank/data".to_string(),
            access_pattern: AccessPattern::Sequential,
            dedup_ratio: 1.2,
            record_size: 131072, // 128K
        };

        assert_eq!(dataset.dataset_name, "tank/data");
        assert_eq!(dataset.access_pattern, AccessPattern::Sequential);
        assert_eq!(dataset.record_size, 131072);
    }

    #[test]
    fn test_zfs_dataset_metrics_random() {
        let dataset = ZfsDatasetMetrics {
            dataset_name: "tank/logs".to_string(),
            access_pattern: AccessPattern::Random,
            dedup_ratio: 1.0,
            record_size: 8192, // 8K
        };

        assert_eq!(dataset.access_pattern, AccessPattern::Random);
        assert_eq!(dataset.record_size, 8192);
    }

    #[test]
    fn test_zfs_dataset_metrics_mixed() {
        let dataset = ZfsDatasetMetrics {
            dataset_name: "tank/mixed".to_string(),
            access_pattern: AccessPattern::Mixed,
            dedup_ratio: 1.5,
            record_size: 65536, // 64K
        };

        assert_eq!(dataset.access_pattern, AccessPattern::Mixed);
    }

    // ==================== ACCESS PATTERN TESTS ====================

    #[test]
    fn test_access_pattern_variants() {
        let patterns = vec![
            AccessPattern::Sequential,
            AccessPattern::Random,
            AccessPattern::Mixed,
        ];
        assert_eq!(patterns.len(), 3);
    }

    #[test]
    fn test_access_pattern_equality() {
        assert_eq!(AccessPattern::Sequential, AccessPattern::Sequential);
        assert_ne!(AccessPattern::Sequential, AccessPattern::Random);
    }

    #[test]
    fn test_access_pattern_clone() {
        let pattern = AccessPattern::Sequential;
        let cloned = pattern.clone();
        assert_eq!(pattern, cloned);
    }

    // ==================== SYSTEM MEMORY USAGE TESTS ====================

    #[test]
    fn test_system_memory_usage_creation() {
        let memory = SystemMemoryUsage {
            total: 32_000_000_000,
            available: 16_000_000_000,
            used: 16_000_000_000,
        };

        assert_eq!(memory.total, 32_000_000_000);
        assert_eq!(memory.available, 16_000_000_000);
        assert_eq!(memory.used, 16_000_000_000);
    }

    #[test]
    fn test_system_memory_usage_validation() {
        let memory = SystemMemoryUsage {
            total: 16_000_000_000,
            available: 8_000_000_000,
            used: 8_000_000_000,
        };

        // Used + available should approximately equal total
        assert!(memory.used + memory.available <= memory.total + 1000); // Allow small variance
    }

    // ==================== ARC STATISTICS TESTS ====================

    #[test]
    fn test_arc_statistics_creation() {
        let arc = ArcStatistics {
            size: 4_000_000_000,
            target_size: 4_000_000_000,
            hit_ratio: 0.85,
            miss_ratio: 0.15,
        };

        assert_eq!(arc.size, 4_000_000_000);
        assert_eq!(arc.target_size, 4_000_000_000);
        assert_eq!(arc.hit_ratio, 0.85);
        assert_eq!(arc.miss_ratio, 0.15);
    }

    #[test]
    fn test_arc_statistics_ratios() {
        let arc = ArcStatistics {
            size: 2_000_000_000,
            target_size: 4_000_000_000,
            hit_ratio: 0.90,
            miss_ratio: 0.10,
        };

        // Hit ratio + miss ratio should equal 1.0
        assert!((arc.hit_ratio + arc.miss_ratio - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_arc_statistics_clone() {
        let arc = ArcStatistics {
            size: 1_000_000_000,
            target_size: 2_000_000_000,
            hit_ratio: 0.75,
            miss_ratio: 0.25,
        };

        let cloned = arc.clone();
        assert_eq!(arc.size, cloned.size);
        assert_eq!(arc.hit_ratio, cloned.hit_ratio);
    }

    // ==================== ZFS BOTTLENECK TESTS ====================

    #[test]
    fn test_zfs_bottleneck_creation() {
        let bottleneck = ZfsBottleneck {
            bottleneck_type: ZfsBottleneckType::HighLatency,
            severity: BottleneckSeverity::High,
            pool_name: "tank".to_string(),
            dataset_name: Some("tank/data".to_string()),
            description: "High latency detected".to_string(),
            impact_score: 8.5,
        };

        assert_eq!(bottleneck.pool_name, "tank");
        assert_eq!(bottleneck.severity, BottleneckSeverity::High);
        assert_eq!(bottleneck.impact_score, 8.5);
    }

    #[test]
    fn test_zfs_bottleneck_no_dataset() {
        let bottleneck = ZfsBottleneck {
            bottleneck_type: ZfsBottleneckType::MemoryPressure,
            severity: BottleneckSeverity::Critical,
            pool_name: "system".to_string(),
            dataset_name: None,
            description: "Critical memory pressure".to_string(),
            impact_score: 9.8,
        };

        assert!(bottleneck.dataset_name.is_none());
        assert_eq!(bottleneck.severity, BottleneckSeverity::Critical);
    }

    // ==================== BOTTLENECK TYPE TESTS ====================

    #[test]
    fn test_bottleneck_type_variants() {
        let types = vec![
            ZfsBottleneckType::HighLatency,
            ZfsBottleneckType::LowThroughput,
            ZfsBottleneckType::CacheMiss,
            ZfsBottleneckType::Fragmentation,
            ZfsBottleneckType::MemoryPressure,
            ZfsBottleneckType::CpuUtilization,
            ZfsBottleneckType::NetworkBandwidth,
            ZfsBottleneckType::DiskIo,
        ];
        assert_eq!(types.len(), 8);
    }

    // ==================== BOTTLENECK SEVERITY TESTS ====================

    #[test]
    fn test_bottleneck_severity_variants() {
        let severities = vec![
            BottleneckSeverity::Low,
            BottleneckSeverity::Medium,
            BottleneckSeverity::High,
            BottleneckSeverity::Critical,
        ];
        assert_eq!(severities.len(), 4);
    }

    #[test]
    fn test_bottleneck_severity_equality() {
        assert_eq!(BottleneckSeverity::High, BottleneckSeverity::High);
        assert_ne!(BottleneckSeverity::Low, BottleneckSeverity::High);
    }

    // ==================== PERFORMANCE OPTIMIZATION RESULT TESTS ====================

    #[test]
    fn test_performance_optimization_result_default() {
        let result = PerformanceOptimizationResult::default();
        assert_eq!(result.applied_optimizations.len(), 0);
        assert_eq!(result.performance_improvement, 0.0);
        assert_eq!(result.bottlenecks_resolved.len(), 0);
        assert_eq!(result.recommendations.len(), 0);
    }

    #[test]
    fn test_performance_optimization_result_with_data() {
        let mut result = PerformanceOptimizationResult::default();
        result.performance_improvement = 15.5;
        result.recommendations.push("Increase ARC size".to_string());
        result
            .recommendations
            .push("Enable compression".to_string());

        assert_eq!(result.performance_improvement, 15.5);
        assert_eq!(result.recommendations.len(), 2);
    }

    #[test]
    fn test_performance_optimization_result_merge() {
        let mut result1 = PerformanceOptimizationResult::default();
        result1.performance_improvement = 10.0;
        result1
            .recommendations
            .push("First recommendation".to_string());

        let mut result2 = PerformanceOptimizationResult::default();
        result2.performance_improvement = 5.0;
        result2
            .recommendations
            .push("Second recommendation".to_string());

        result1.merge_with(result2);

        assert_eq!(result1.performance_improvement, 15.0);
        assert_eq!(result1.recommendations.len(), 2);
    }

    #[test]
    fn test_performance_optimization_result_clone() {
        let result = PerformanceOptimizationResult {
            applied_optimizations: vec![],
            performance_improvement: 20.0,
            bottlenecks_resolved: vec![],
            recommendations: vec!["Test".to_string()],
        };

        let cloned = result.clone();
        assert_eq!(
            result.performance_improvement,
            cloned.performance_improvement
        );
        assert_eq!(result.recommendations.len(), cloned.recommendations.len());
    }

    // ==================== APPLIED OPTIMIZATION TESTS ====================

    #[test]
    fn test_applied_optimization_creation() {
        let optimization = AppliedOptimization {
            optimization_type: OptimizationType::ArcTuning,
            description: "Increased ARC size to 8G".to_string(),
            performance_impact: 15.5,
            applied_at: SystemTime::now(),
        };

        assert_eq!(optimization.description, "Increased ARC size to 8G");
        assert_eq!(optimization.performance_impact, 15.5);
    }

    #[test]
    fn test_applied_optimization_clone() {
        let optimization = AppliedOptimization {
            optimization_type: OptimizationType::CompressionOptimization,
            description: "Enabled LZ4 compression".to_string(),
            performance_impact: 10.0,
            applied_at: SystemTime::now(),
        };

        let cloned = optimization.clone();
        assert_eq!(optimization.description, cloned.description);
        assert_eq!(optimization.performance_impact, cloned.performance_impact);
    }

    // ==================== OPTIMIZATION TYPE TESTS ====================

    #[test]
    fn test_optimization_type_variants() {
        let types = vec![
            OptimizationType::CacheOptimization,
            OptimizationType::LatencyOptimization,
            OptimizationType::ThroughputOptimization,
            OptimizationType::FragmentationDefrag,
            OptimizationType::ArcTuning,
            OptimizationType::RecordSizeOptimization,
            OptimizationType::CompressionOptimization,
        ];
        assert_eq!(types.len(), 7);
    }

    // ==================== ALERT TYPE TESTS ====================

    #[test]
    fn test_alert_type_variants() {
        let types = vec![
            AlertType::PerformanceDegradation,
            AlertType::BottleneckDetected,
            AlertType::ThresholdExceeded,
            AlertType::OptimizationFailed,
        ];
        assert_eq!(types.len(), 4);
    }

    // ==================== ALERT SEVERITY TESTS ====================

    #[test]
    fn test_alert_severity_variants() {
        let severities = vec![
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::Error,
            AlertSeverity::Critical,
        ];
        assert_eq!(severities.len(), 4);
    }

    #[test]
    fn test_alert_severity_equality() {
        assert_eq!(AlertSeverity::Critical, AlertSeverity::Critical);
        assert_ne!(AlertSeverity::Info, AlertSeverity::Critical);
    }

    // ==================== ALERT RESPONSE TESTS ====================

    #[test]
    fn test_alert_response_creation() {
        let response = AlertResponse {
            mitigation_applied: true,
            optimization_result: None,
            follow_up_required: false,
        };

        assert!(response.mitigation_applied);
        assert!(response.optimization_result.is_none());
        assert!(!response.follow_up_required);
    }

    #[test]
    fn test_alert_response_with_optimization() {
        let response = AlertResponse {
            mitigation_applied: true,
            optimization_result: Some(PerformanceOptimizationResult::default()),
            follow_up_required: true,
        };

        assert!(response.mitigation_applied);
        assert!(response.optimization_result.is_some());
        assert!(response.follow_up_required);
    }

    #[test]
    fn test_alert_response_default() {
        let response = AlertResponse::default();
        assert!(!response.mitigation_applied);
        assert!(!response.follow_up_required);
    }

    // ==================== PERFORMANCE ALERT TESTS ====================

    #[test]
    fn test_performance_alert_creation() {
        let alert = PerformanceAlert {
            alert_type: AlertType::PerformanceDegradation,
            severity: AlertSeverity::Warning,
            pool_name: "tank".to_string(),
            dataset_name: Some("tank/data".to_string()),
            description: "Performance degradation detected".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(alert.pool_name, "tank");
        assert_eq!(alert.severity, AlertSeverity::Warning);
        assert!(alert.dataset_name.is_some());
    }

    #[test]
    fn test_performance_alert_no_dataset() {
        let alert = PerformanceAlert {
            alert_type: AlertType::ThresholdExceeded,
            severity: AlertSeverity::Critical,
            pool_name: "system".to_string(),
            dataset_name: None,
            description: "Critical threshold exceeded".to_string(),
            timestamp: SystemTime::now(),
        };

        assert!(alert.dataset_name.is_none());
        assert_eq!(alert.severity, AlertSeverity::Critical);
    }

    // ==================== ZFS TUNING RESULT TESTS ====================

    #[test]
    fn test_zfs_tuning_result_success() {
        let mut param_changes = HashMap::new();
        param_changes.insert("arc_max".to_string(), "8G".to_string());

        let result = ZfsTuningResult {
            tuning_applied: true,
            parameter_changes: param_changes,
            expected_improvement: 20.0,
            validation_required: false,
        };

        assert!(result.tuning_applied);
        assert_eq!(result.expected_improvement, 20.0);
        assert_eq!(result.parameter_changes.len(), 1);
    }

    #[test]
    fn test_zfs_tuning_result_with_validation() {
        let mut param_changes = HashMap::new();
        param_changes.insert("compression".to_string(), "lz4".to_string());

        let result = ZfsTuningResult {
            tuning_applied: true,
            parameter_changes: param_changes,
            expected_improvement: 15.0,
            validation_required: true,
        };

        assert!(result.validation_required);
        assert_eq!(result.expected_improvement, 15.0);
    }
}
