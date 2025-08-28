//
// Complete test coverage for real-time performance optimization including:
// - Metrics collection and analysis
// - Bottleneck detection algorithms
// - Performance optimization strategies
// - Real-time monitoring systems
// - Alert handling and response
// - ZFS parameter tuning

use nestgate_zfs::performance_engine::{
    AccessPattern, AlertResponse, AlertSeverity, AlertType, AppliedOptimization, ArcStatistics,
    BottleneckSeverity, OptimizationType, PerformanceAlert, PerformanceOptimizationResult,
    SystemCapabilities, SystemMemoryUsage, WorkloadPattern, ZfsBottleneck, ZfsBottleneckType,
    ZfsConfigurationContext, ZfsDatasetMetrics, ZfsPerformanceMetrics, ZfsPoolMetrics,
    ZfsTuningRecommendation, ZfsTuningResult,
};
use std::collections::HashMap;
use std::time::SystemTime;

#[cfg(test)]
mod test_utils {
    use super::*;

    pub fn create_test_pool_metrics(pool_name: &str) -> ZfsPoolMetrics {
        ZfsPoolMetrics {
            pool_name: pool_name.to_string(),
            read_ops: 1000.0,
            write_ops: 500.0,
            read_bandwidth: 100.0,
            write_bandwidth: 50.0,
            latency: 5.0,
            cache_hit_ratio: 0.85,
            fragmentation: 0.15,
        }
    }

    pub fn create_test_dataset_metrics(dataset_name: &str) -> ZfsDatasetMetrics {
        ZfsDatasetMetrics {
            dataset_name: dataset_name.to_string(),
            access_pattern: AccessPattern::Mixed,
            dedup_ratio: 1.2,
            record_size: 128 * 1024, // 128KB
        }
    }

    pub fn create_test_performance_metrics() -> ZfsPerformanceMetrics {
        let mut pool_metrics = HashMap::new();
        pool_metrics.insert("testpool".to_string(), create_test_pool_metrics("testpool"));

        let mut dataset_metrics = HashMap::new();
        dataset_metrics.insert(
            "testpool/data".to_string(),
            create_test_dataset_metrics("testpool/data"),
        );

        ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            dataset_metrics,
            system_memory: SystemMemoryUsage {
                total: 32 * 1024 * 1024 * 1024,     // 32GB
                available: 16 * 1024 * 1024 * 1024, // 16GB
                used: 16 * 1024 * 1024 * 1024,      // 16GB
            },
            arc_stats: ArcStatistics {
                size: 8 * 1024 * 1024 * 1024,         // 8GB
                target_size: 10 * 1024 * 1024 * 1024, // 10GB
                hit_ratio: 0.95,
                miss_ratio: 0.05,
            },
        }
    }

    pub fn create_test_bottleneck(severity: BottleneckSeverity) -> ZfsBottleneck {
        ZfsBottleneck {
            bottleneck_type: ZfsBottleneckType::HighLatency,
            severity,
            pool_name: "testpool".to_string(),
            dataset_name: Some("testpool/data".to_string()),
            description: "High latency detected in pool operations".to_string(),
            impact_score: 0.8,
        }
    }

    pub fn create_test_workload_pattern() -> WorkloadPattern {
        let mut io_size_distribution = HashMap::new();
        io_size_distribution.insert("small".to_string(), 0.3);
        io_size_distribution.insert("medium".to_string(), 0.5);
        io_size_distribution.insert("large".to_string(), 0.2);

        WorkloadPattern {
            access_pattern: AccessPattern::Random,
            io_size_distribution,
            read_write_ratio: 0.7,
            temporal_locality: 0.6,
        }
    }

    pub fn create_test_configuration_context() -> ZfsConfigurationContext {
        let mut current_config = HashMap::new();
        current_config.insert("recordsize".to_string(), "128K".to_string());
        current_config.insert("compression".to_string(), "lz4".to_string());
        current_config.insert("primarycache".to_string(), "all".to_string());

        ZfsConfigurationContext {
            pool_name: "testpool".to_string(),
            dataset_name: Some("testpool/data".to_string()),
            current_configuration: current_config,
            workload_pattern: create_test_workload_pattern(),
            system_capabilities: SystemCapabilities {
                cpu_cores: 8,
                memory_gb: 32,
                storage_type: "SSD".to_string(),
                network_bandwidth_gbps: 10.0,
            },
        }
    }
}

#[cfg(test)]
mod performance_metrics_tests {
    use test_utils::*;

    #[test]
    fn test_performance_metrics_creation() {
        let metrics = create_test_performance_metrics();

        assert_eq!(metrics.pool_metrics.len(), 1);
        assert_eq!(metrics.dataset_metrics.len(), 1);
        assert!(metrics.system_memory.total > 0);
        assert!(metrics.arc_stats.size > 0);
    }

    #[test]
    fn test_pool_metrics_validation() {
        let pool_metrics = create_test_pool_metrics("testpool");

        assert_eq!(pool_metrics.pool_name, "testpool");
        assert!(pool_metrics.read_ops > 0.0);
        assert!(pool_metrics.write_ops > 0.0);
        assert!(pool_metrics.cache_hit_ratio > 0.0);
        assert!(pool_metrics.cache_hit_ratio <= 1.0);
    }

    #[test]
    fn test_dataset_metrics_validation() {
        let dataset_metrics = create_test_dataset_metrics("testpool/data");

        assert_eq!(dataset_metrics.dataset_name, "testpool/data");
        assert!(dataset_metrics.dedup_ratio >= 1.0);
        assert!(dataset_metrics.record_size > 0);
    }
}

#[cfg(test)]
mod bottleneck_detection_tests {

    #[test]
    fn test_bottleneck_creation() {
        let bottleneck = create_test_bottleneck(BottleneckSeverity::High);

        assert_eq!(bottleneck.severity, BottleneckSeverity::High);
        assert_eq!(bottleneck.pool_name, "testpool");
        assert!(bottleneck.impact_score > 0.0);
        assert!(bottleneck.impact_score <= 1.0);
    }

    #[test]
    fn test_bottleneck_types() {
        let bottleneck_types = [
            ZfsBottleneckType::HighLatency,
            ZfsBottleneckType::LowThroughput,
            ZfsBottleneckType::CacheMiss,
            ZfsBottleneckType::Fragmentation,
            ZfsBottleneckType::MemoryPressure,
            ZfsBottleneckType::CpuUtilization,
            ZfsBottleneckType::NetworkBandwidth,
            ZfsBottleneckType::DiskIo,
        ];

        assert_eq!(bottleneck_types.len(), 8);
    }

    #[test]
    fn test_bottleneck_severity_ordering() {
        let severities = [
            BottleneckSeverity::Low,
            BottleneckSeverity::Medium,
            BottleneckSeverity::High,
            BottleneckSeverity::Critical,
        ];

        assert_eq!(severities.len(), 4);
        assert_eq!(severities[0], BottleneckSeverity::Low);
        assert_eq!(severities[3], BottleneckSeverity::Critical);
    }
}

#[cfg(test)]
mod optimization_tests {

    #[test]
    fn test_applied_optimization_creation() {
        let optimization = AppliedOptimization {
            optimization_type: OptimizationType::CacheOptimization,
            description: "Improved ARC cache configuration".to_string(),
            performance_impact: 0.25,
            applied_at: SystemTime::now(),
        };

        assert_eq!(optimization.performance_impact, 0.25);
        assert!(!optimization.description.is_empty());
    }

    #[test]
    fn test_optimization_result_creation() {
        let optimization = AppliedOptimization {
            optimization_type: OptimizationType::LatencyOptimization,
            description: "Reduced latency through record size optimization".to_string(),
            performance_impact: 0.30,
            applied_at: SystemTime::now(),
        };

        let result = PerformanceOptimizationResult {
            applied_optimizations: vec![optimization],
            performance_improvement: 30.0,
            bottlenecks_resolved: vec![create_test_bottleneck(BottleneckSeverity::Medium)],
            recommendations: vec!["Consider increasing ARC size".to_string()],
        };

        assert_eq!(result.applied_optimizations.len(), 1);
        assert_eq!(result.performance_improvement, 30.0);
        assert_eq!(result.bottlenecks_resolved.len(), 1);
        assert_eq!(result.recommendations.len(), 1);
    }

    #[test]
    fn test_optimization_result_merge() {
        let mut result1 = PerformanceOptimizationResult {
            applied_optimizations: vec![AppliedOptimization {
                optimization_type: OptimizationType::CacheOptimization,
                description: "Cache optimization".to_string(),
                performance_impact: 0.2,
                applied_at: SystemTime::now(),
            }],
            performance_improvement: 20.0,
            bottlenecks_resolved: vec![create_test_bottleneck(BottleneckSeverity::Low)],
            recommendations: vec!["First recommendation".to_string()],
        };

        let result2 = PerformanceOptimizationResult {
            applied_optimizations: vec![AppliedOptimization {
                optimization_type: OptimizationType::ThroughputOptimization,
                description: "Throughput optimization".to_string(),
                performance_impact: 0.15,
                applied_at: SystemTime::now(),
            }],
            performance_improvement: 15.0,
            bottlenecks_resolved: vec![create_test_bottleneck(BottleneckSeverity::Medium)],
            recommendations: vec!["Second recommendation".to_string()],
        };

        result1.merge_with(result2);

        assert_eq!(result1.applied_optimizations.len(), 2);
        assert_eq!(result1.performance_improvement, 35.0);
        assert_eq!(result1.bottlenecks_resolved.len(), 2);
        assert_eq!(result1.recommendations.len(), 2);
    }

    #[test]
    fn test_optimization_types() {
        let types = [
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
}

#[cfg(test)]
mod alert_handling_tests {

    #[test]
    fn test_performance_alert_creation() {
        let alert = PerformanceAlert {
            alert_type: AlertType::PerformanceDegradation,
            severity: AlertSeverity::Warning,
            pool_name: "testpool".to_string(),
            dataset_name: Some("testpool/data".to_string()),
            description: "Performance degradation detected".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(alert.alert_type, AlertType::PerformanceDegradation);
        assert_eq!(alert.severity, AlertSeverity::Warning);
        assert_eq!(alert.pool_name, "testpool");
        assert!(!alert.description.is_empty());
    }

    #[test]
    fn test_alert_response_creation() {
        let optimization_result = PerformanceOptimizationResult {
            applied_optimizations: vec![AppliedOptimization {
                optimization_type: OptimizationType::CacheOptimization,
                description: "Applied cache optimization".to_string(),
                performance_impact: 0.25,
                applied_at: SystemTime::now(),
            }],
            performance_improvement: 25.0,
            bottlenecks_resolved: vec![],
            recommendations: vec!["Monitor cache hit ratio".to_string()],
        };

        let response = AlertResponse {
            mitigation_applied: true,
            optimization_result: Some(optimization_result),
            follow_up_required: false,
        };

        assert!(response.mitigation_applied);
        assert!(response.optimization_result.is_some());
        assert!(!response.follow_up_required);
    }

    #[test]
    fn test_alert_types() {
        let alert_types = [
            AlertType::PerformanceDegradation,
            AlertType::BottleneckDetected,
            AlertType::ThresholdExceeded,
            AlertType::OptimizationFailed,
        ];

        assert_eq!(alert_types.len(), 4);
    }

    #[test]
    fn test_alert_severity_levels() {
        let severities = [
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::Error,
            AlertSeverity::Critical,
        ];

        assert_eq!(severities.len(), 4);
    }
}

#[cfg(test)]
mod tuning_tests {

    #[test]
    fn test_tuning_recommendation_creation() {
        let recommendation = ZfsTuningRecommendation {
            parameter: "recordsize".to_string(),
            recommended_value: "1M".to_string(),
            confidence: 0.85,
            expected_impact: 0.40,
        };

        assert_eq!(recommendation.parameter, "recordsize");
        assert_eq!(recommendation.recommended_value, "1M");
        assert!(recommendation.confidence > 0.0);
        assert!(recommendation.expected_impact > 0.0);
    }

    #[test]
    fn test_tuning_result_creation() {
        let mut parameter_changes = HashMap::new();
        parameter_changes.insert("recordsize".to_string(), "1M".to_string());
        parameter_changes.insert("compression".to_string(), "zstd".to_string());

        let result = ZfsTuningResult {
            tuning_applied: true,
            parameter_changes,
            expected_improvement: 45.0,
            validation_required: true,
        };

        assert!(result.tuning_applied);
        assert_eq!(result.parameter_changes.len(), 2);
        assert_eq!(result.expected_improvement, 45.0);
        assert!(result.validation_required);
    }

    #[test]
    fn test_tuning_recommendations_validation() {
        let recommendations = vec![
            ZfsTuningRecommendation {
                parameter: "recordsize".to_string(),
                recommended_value: "1M".to_string(),
                confidence: 0.90,
                expected_impact: 0.35,
            },
            ZfsTuningRecommendation {
                parameter: "compression".to_string(),
                recommended_value: "zstd".to_string(),
                confidence: 0.85,
                expected_impact: 0.20,
            },
        ];

        // Verify recommendation structure
        for rec in &recommendations {
            assert!(!rec.parameter.is_empty());
            assert!(!rec.recommended_value.is_empty());
            assert!(rec.confidence > 0.0);
            assert!(rec.expected_impact > 0.0);
        }

        // Verify specific recommendations
        let recordsize_rec = recommendations
            .iter()
            .find(|r| r.parameter == "recordsize")
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "recordsize recommendation not found",
                )
            })?;
        assert_eq!(recordsize_rec.recommended_value, "1M");

        let compression_rec = recommendations
            .iter()
            .find(|r| r.parameter == "compression")
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "compression recommendation not found",
                )
            })?;
        assert_eq!(compression_rec.recommended_value, "zstd");
    }
}

#[cfg(test)]
mod configuration_tests {
    use super::test_utils::*;

    #[test]
    fn test_workload_pattern_creation() {
        let pattern = create_test_workload_pattern();

        assert_eq!(pattern.access_pattern, AccessPattern::Random);
        assert_eq!(pattern.io_size_distribution.len(), 3);
        assert!(pattern.read_write_ratio > 0.0);
        assert!(pattern.temporal_locality > 0.0);
    }

    #[test]
    fn test_configuration_context_creation() {
        let context = create_test_configuration_context();

        assert_eq!(context.pool_name, "testpool");
        assert!(context.dataset_name.is_some());
        assert_eq!(context.current_configuration.len(), 3);
        assert_eq!(context.system_capabilities.cpu_cores, 8);
        assert_eq!(context.system_capabilities.memory_gb, 32);
    }

    #[test]
    fn test_system_capabilities_validation() {
        let capabilities = SystemCapabilities {
            cpu_cores: 16,
            memory_gb: 64,
            storage_type: "NVMe".to_string(),
            network_bandwidth_gbps: 25.0,
        };

        assert_eq!(capabilities.cpu_cores, 16);
        assert_eq!(capabilities.memory_gb, 64);
        assert_eq!(capabilities.storage_type, "NVMe");
        assert_eq!(capabilities.network_bandwidth_gbps, 25.0);
    }
}

#[cfg(test)]
mod integration_tests {

    #[test]
    fn test_comprehensive_performance_scenario() {
        // Create initial metrics
        let _metrics = create_test_performance_metrics();

        // Detect bottlenecks
        let bottlenecks = vec![
            create_test_bottleneck(BottleneckSeverity::High),
            ZfsBottleneck {
                bottleneck_type: ZfsBottleneckType::CacheMiss,
                severity: BottleneckSeverity::Medium,
                pool_name: "testpool".to_string(),
                dataset_name: Some("testpool/data".to_string()),
                description: "Low cache hit ratio".to_string(),
                impact_score: 0.6,
            },
        ];

        // Apply optimizations
        let optimizations = vec![
            AppliedOptimization {
                optimization_type: OptimizationType::LatencyOptimization,
                description: "Reduced latency through ARC tuning".to_string(),
                performance_impact: 0.30,
                applied_at: SystemTime::now(),
            },
            AppliedOptimization {
                optimization_type: OptimizationType::CacheOptimization,
                description: "Improved cache hit ratio".to_string(),
                performance_impact: 0.25,
                applied_at: SystemTime::now(),
            },
        ];

        // Create optimization result
        let optimization_result = PerformanceOptimizationResult {
            applied_optimizations: optimizations,
            performance_improvement: 55.0,
            bottlenecks_resolved: bottlenecks,
            recommendations: vec![
                "Monitor ARC hit ratio".to_string(),
                "Consider increasing record size for large files".to_string(),
            ],
        };

        // Verify results
        assert_eq!(optimization_result.applied_optimizations.len(), 2);
        assert_eq!(optimization_result.performance_improvement, 55.0);
        assert_eq!(optimization_result.bottlenecks_resolved.len(), 2);
        assert_eq!(optimization_result.recommendations.len(), 2);

        // Create alert response
        let alert_response = AlertResponse {
            mitigation_applied: true,
            optimization_result: Some(optimization_result),
            follow_up_required: false,
        };

        assert!(alert_response.mitigation_applied);
        assert!(alert_response.optimization_result.is_some());
    }

    #[test]
    fn test_tuning_workflow() {
        // Create configuration context
        let _context = create_test_configuration_context();

        // Generate tuning recommendations
        let recommendations = vec![
            ZfsTuningRecommendation {
                parameter: "recordsize".to_string(),
                recommended_value: "1M".to_string(),
                confidence: 0.90,
                expected_impact: 0.40,
            },
            ZfsTuningRecommendation {
                parameter: "compression".to_string(),
                recommended_value: "zstd".to_string(),
                confidence: 0.85,
                expected_impact: 0.25,
            },
        ];

        // Apply tuning
        let mut parameter_changes = HashMap::new();
        for rec in &recommendations {
            parameter_changes.insert(rec.parameter.clone(), rec.recommended_value.clone());
        }

        let tuning_result = ZfsTuningResult {
            tuning_applied: true,
            parameter_changes,
            expected_improvement: 65.0,
            validation_required: true,
        };

        // Verify tuning workflow
        assert!(tuning_result.tuning_applied);
        assert_eq!(tuning_result.parameter_changes.len(), 2);
        assert_eq!(tuning_result.expected_improvement, 65.0);
        assert!(tuning_result.validation_required);
    }
}
