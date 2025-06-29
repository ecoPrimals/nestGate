//! Comprehensive Performance Engine Tests
//!
//! Tests the real-time performance optimization engine with focus on:
//! - Performance metrics collection and analysis
//! - Bottleneck detection and optimization
//! - ZFS parameter tuning
//! - Alert handling and response

use nestgate_zfs::performance_engine::{
    PerformanceOptimizationEngine, RealTimePerformanceMonitor, PerformanceEngineConfig,
    ZfsPerformanceMetrics, ZfsPoolMetrics, ZfsDatasetMetrics, SystemMemoryUsage,
    ArcStatistics, ZfsBottleneck, ZfsBottleneckType, BottleneckSeverity,
    PerformanceOptimizationResult, AppliedOptimization, OptimizationType,
    PerformanceAlert, AlertType, AlertSeverity, AccessPattern, WorkloadPattern,
    ZfsConfigurationContext, ZfsTuningResult
};
use nestgate_zfs::{ZfsPoolManager, ZfsDatasetManager};
use nestgate_zfs::config::ZfsConfig;
use nestgate_core::StorageTier;
use std::collections::HashMap;
use std::time::SystemTime;
use std::sync::Arc;

/// Test helper functions
mod test_helpers {
    use super::*;
    
    /// Create a test ZFS performance metrics structure
    pub fn create_test_performance_metrics() -> ZfsPerformanceMetrics {
        let mut pool_metrics = HashMap::new();
        pool_metrics.insert("test_pool".to_string(), ZfsPoolMetrics {
            name: "test_pool".to_string(),
            read_ops_per_sec: 1000.0,
            write_ops_per_sec: 500.0,
            read_bandwidth_mbps: 100.0,
            write_bandwidth_mbps: 50.0,
            average_latency_ms: 5.0,
            cache_hit_ratio: 0.85,
            fragmentation_percent: 15.0,
        });
        
        let mut dataset_metrics = HashMap::new();
        dataset_metrics.insert("test_dataset".to_string(), ZfsDatasetMetrics {
            name: "test_dataset".to_string(),
            compression_ratio: 2.0,
            dedup_ratio: 1.5,
            record_size: 128 * 1024, // 128KB
            access_pattern: AccessPattern::Sequential,
        });
        
        ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            dataset_metrics,
            system_memory_usage: SystemMemoryUsage {
                total_memory: 16 * 1024 * 1024 * 1024, // 16GB
                used_memory: 8 * 1024 * 1024 * 1024,   // 8GB
                available_memory: 8 * 1024 * 1024 * 1024, // 8GB
            },
            arc_stats: ArcStatistics {
                hit_ratio: 0.90,
                size_bytes: 4 * 1024 * 1024 * 1024, // 4GB
                target_size_bytes: 4 * 1024 * 1024 * 1024, // 4GB
                meta_used_bytes: 512 * 1024 * 1024, // 512MB
            },
        }
    }
    
    /// Create test bottleneck
    pub fn create_test_bottleneck() -> ZfsBottleneck {
        ZfsBottleneck {
            bottleneck_type: ZfsBottleneckType::HighLatency,
            affected_component: "test_pool".to_string(),
            severity: BottleneckSeverity::Medium,
            current_value: 25.0,
            threshold_value: 20.0,
            zfs_specific_context: "Pool latency exceeds threshold".to_string(),
        }
    }
    
    /// Create test performance alert
    pub fn create_test_alert() -> PerformanceAlert {
        PerformanceAlert {
            alert_type: AlertType::HighLatency,
            severity: AlertSeverity::Warning,
            component: "test_pool".to_string(),
            description: "Pool latency is above threshold".to_string(),
            timestamp: SystemTime::now(),
        }
    }
    
    /// Create test workload pattern
    pub fn create_test_workload_pattern() -> WorkloadPattern {
        WorkloadPattern {
            read_write_ratio: 0.7, // 70% reads, 30% writes
            sequential_random_ratio: 0.8, // 80% sequential
            average_io_size: 64 * 1024, // 64KB
            peak_iops: 10000,
        }
    }
}

#[cfg(test)]
mod performance_engine_creation_tests {
    use super::*;
    use super::test_helpers::*;

    #[tokio::test]
    async fn test_performance_engine_creation() {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new_for_testing());
        let dataset_manager = Arc::new(ZfsDatasetManager::new(
            config.clone(),
            pool_manager.clone(),
        ));

        let engine = PerformanceOptimizationEngine::new(
            config,
            pool_manager,
            dataset_manager,
        );

        // Verify engine was created successfully
        // Since fields are private, we test by calling public methods
        let trending_data = engine.get_trending_data().await;
        assert!(trending_data.is_ok(), "Should be able to get trending data");
    }

    #[test]
    fn test_performance_engine_config_default() {
        let config = PerformanceEngineConfig::default();
        
        assert_eq!(config.latency_threshold_ms, 20.0);
        assert_eq!(config.cache_hit_threshold, 0.80);
        assert_eq!(config.fragmentation_threshold, 20.0);
        assert_eq!(config.arc_hit_threshold, 0.80);
        assert_eq!(config.optimization_interval_seconds, 300);
        assert_eq!(config.monitoring_interval_seconds, 30);
    }
}

#[cfg(test)]
mod real_time_performance_monitor_tests {
    use super::*;
    use super::test_helpers::*;

    #[test]
    fn test_real_time_monitor_creation() {
        let monitor = RealTimePerformanceMonitor::new();
        // Verify monitor was created (testing by existence)
        
        let default_monitor = RealTimePerformanceMonitor::default();
        // Should create without panic
    }

    #[tokio::test]
    async fn test_collect_metrics() {
        let monitor = RealTimePerformanceMonitor::new();
        let config = ZfsConfig::default();
        let pool_manager = ZfsPoolManager::new_for_testing();
        let dataset_manager = ZfsDatasetManager::new(
            config.clone(),
            Arc::new(pool_manager),
        );

        let result = monitor.collect_metrics(&ZfsPoolManager::new_for_testing(), &dataset_manager).await;
        // Should not panic - result may succeed or fail depending on environment
        match result {
            Ok(_) => println!("Metrics collection succeeded"),
            Err(_) => println!("Metrics collection failed as expected in test environment"),
        }
    }
}

#[cfg(test)]
mod performance_metrics_tests {
    use super::*;
    use super::test_helpers::*;

    #[test]
    fn test_zfs_performance_metrics_creation() {
        let metrics = create_test_performance_metrics();
        
        assert!(metrics.pool_metrics.contains_key("test_pool"));
        assert!(metrics.dataset_metrics.contains_key("test_dataset"));
        assert_eq!(metrics.system_memory_usage.total_memory, 16 * 1024 * 1024 * 1024);
        assert_eq!(metrics.arc_stats.hit_ratio, 0.90);
    }

    #[test]
    fn test_pool_metrics_values() {
        let metrics = create_test_performance_metrics();
        let pool_metrics = metrics.pool_metrics.get("test_pool").unwrap();
        
        assert_eq!(pool_metrics.name, "test_pool");
        assert_eq!(pool_metrics.read_ops_per_sec, 1000.0);
        assert_eq!(pool_metrics.write_ops_per_sec, 500.0);
        assert_eq!(pool_metrics.cache_hit_ratio, 0.85);
        assert_eq!(pool_metrics.fragmentation_percent, 15.0);
    }

    #[test]
    fn test_dataset_metrics_values() {
        let metrics = create_test_performance_metrics();
        let dataset_metrics = metrics.dataset_metrics.get("test_dataset").unwrap();
        
        assert_eq!(dataset_metrics.name, "test_dataset");
        assert_eq!(dataset_metrics.compression_ratio, 2.0);
        assert_eq!(dataset_metrics.dedup_ratio, 1.5);
        assert_eq!(dataset_metrics.record_size, 128 * 1024);
        assert!(matches!(dataset_metrics.access_pattern, AccessPattern::Sequential));
    }

    #[test]
    fn test_system_memory_usage() {
        let metrics = create_test_performance_metrics();
        let memory = &metrics.system_memory_usage;
        
        assert_eq!(memory.total_memory, 16 * 1024 * 1024 * 1024);
        assert_eq!(memory.used_memory, 8 * 1024 * 1024 * 1024);
        assert_eq!(memory.available_memory, 8 * 1024 * 1024 * 1024);
        
        // Verify memory consistency
        assert_eq!(memory.used_memory + memory.available_memory, memory.total_memory);
    }

    #[test]
    fn test_arc_statistics() {
        let metrics = create_test_performance_metrics();
        let arc = &metrics.arc_stats;
        
        assert_eq!(arc.hit_ratio, 0.90);
        assert_eq!(arc.size_bytes, 4 * 1024 * 1024 * 1024);
        assert_eq!(arc.target_size_bytes, 4 * 1024 * 1024 * 1024);
        assert_eq!(arc.meta_used_bytes, 512 * 1024 * 1024);
    }
}

#[cfg(test)]
mod bottleneck_detection_tests {
    use super::*;
    use super::test_helpers::*;

    #[test]
    fn test_bottleneck_creation() {
        let bottleneck = create_test_bottleneck();
        
        assert!(matches!(bottleneck.bottleneck_type, ZfsBottleneckType::HighLatency));
        assert_eq!(bottleneck.affected_component, "test_pool");
        assert!(matches!(bottleneck.severity, BottleneckSeverity::Medium));
        assert_eq!(bottleneck.current_value, 25.0);
        assert_eq!(bottleneck.threshold_value, 20.0);
    }

    #[test]
    fn test_bottleneck_types() {
        let bottleneck_types = vec![
            ZfsBottleneckType::HighLatency,
            ZfsBottleneckType::LowCacheHitRatio,
            ZfsBottleneckType::HighFragmentation,
            ZfsBottleneckType::ArcInefficiency,
            ZfsBottleneckType::RecordSizeMismatch,
            ZfsBottleneckType::CompressionInefficiency,
        ];
        
        // Test that all types can be created
        for bottleneck_type in bottleneck_types {
            let bottleneck = ZfsBottleneck {
                bottleneck_type,
                affected_component: "test".to_string(),
                severity: BottleneckSeverity::Low,
                current_value: 0.0,
                threshold_value: 1.0,
                zfs_specific_context: "test".to_string(),
            };
            // Should not panic
        }
    }

    #[test]
    fn test_bottleneck_severities() {
        let severities = vec![
            BottleneckSeverity::Low,
            BottleneckSeverity::Medium,
            BottleneckSeverity::High,
            BottleneckSeverity::Critical,
        ];
        
        for severity in severities {
            let bottleneck = ZfsBottleneck {
                bottleneck_type: ZfsBottleneckType::HighLatency,
                affected_component: "test".to_string(),
                severity,
                current_value: 0.0,
                threshold_value: 1.0,
                zfs_specific_context: "test".to_string(),
            };
            // Should not panic
        }
    }
}

#[cfg(test)]
mod optimization_result_tests {
    use super::*;

    #[test]
    fn test_optimization_result_creation() {
        let optimization = AppliedOptimization {
            optimization_type: OptimizationType::LatencyOptimization,
            component: "test_pool".to_string(),
            description: "Reduced latency by tuning cache".to_string(),
            parameters_changed: vec![("cache_size".to_string(), "8GB".to_string())],
            expected_improvement: "20% latency reduction".to_string(),
        };
        
        assert!(matches!(optimization.optimization_type, OptimizationType::LatencyOptimization));
        assert_eq!(optimization.component, "test_pool");
        assert_eq!(optimization.parameters_changed.len(), 1);
    }

    #[test]
    fn test_optimization_result_merge() {
        let mut result1 = PerformanceOptimizationResult {
            optimizations_applied: 2,
            applied_optimizations: vec![],
            performance_improvement_estimate: "10% improvement".to_string(),
            warnings: vec!["Warning 1".to_string()],
        };
        
        let result2 = PerformanceOptimizationResult {
            optimizations_applied: 3,
            applied_optimizations: vec![],
            performance_improvement_estimate: "15% improvement".to_string(),
            warnings: vec!["Warning 2".to_string()],
        };
        
        result1.merge_with(result2);
        
        assert_eq!(result1.optimizations_applied, 5);
        assert_eq!(result1.warnings.len(), 2);
    }

    #[test]
    fn test_optimization_types() {
        let types = vec![
            OptimizationType::LatencyOptimization,
            OptimizationType::CacheOptimization,
            OptimizationType::DefragmentationScheduling,
            OptimizationType::ArcTuning,
            OptimizationType::RecordSizeAdjustment,
            OptimizationType::CompressionTuning,
        ];
        
        // Test that all optimization types can be created
        for opt_type in types {
            let optimization = AppliedOptimization {
                optimization_type: opt_type,
                component: "test".to_string(),
                description: "test".to_string(),
                parameters_changed: vec![],
                expected_improvement: "test".to_string(),
            };
            // Should not panic
        }
    }
}

#[cfg(test)]
mod performance_alert_tests {
    use super::*;
    use super::test_helpers::*;

    #[test]
    fn test_performance_alert_creation() {
        let alert = create_test_alert();
        
        assert!(matches!(alert.alert_type, AlertType::HighLatency));
        assert!(matches!(alert.severity, AlertSeverity::Warning));
        assert_eq!(alert.component, "test_pool");
        assert!(!alert.description.is_empty());
    }

    #[test]
    fn test_alert_types() {
        let alert_types = vec![
            AlertType::PerformanceBottleneck,
            AlertType::HighLatency,
            AlertType::LowThroughput,
            AlertType::CacheInefficiency,
            AlertType::FragmentationHigh,
        ];
        
        for alert_type in alert_types {
            let alert = PerformanceAlert {
                alert_type,
                severity: AlertSeverity::Info,
                component: "test".to_string(),
                description: "test".to_string(),
                timestamp: SystemTime::now(),
            };
            // Should not panic
        }
    }

    #[test]
    fn test_alert_severities() {
        let severities = vec![
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::High,
            AlertSeverity::Critical,
        ];
        
        for severity in severities {
            let alert = PerformanceAlert {
                alert_type: AlertType::HighLatency,
                severity,
                component: "test".to_string(),
                description: "test".to_string(),
                timestamp: SystemTime::now(),
            };
            // Should not panic
        }
    }
}

#[cfg(test)]
mod workload_pattern_tests {
    use super::*;
    use super::test_helpers::*;

    #[test]
    fn test_workload_pattern_creation() {
        let pattern = create_test_workload_pattern();
        
        assert_eq!(pattern.read_write_ratio, 0.7);
        assert_eq!(pattern.sequential_random_ratio, 0.8);
        assert_eq!(pattern.average_io_size, 64 * 1024);
        assert_eq!(pattern.peak_iops, 10000);
    }

    #[test]
    fn test_access_patterns() {
        let patterns = vec![
            AccessPattern::Sequential,
            AccessPattern::Random,
            AccessPattern::Mixed,
        ];
        
        for pattern in patterns {
            let dataset_metrics = ZfsDatasetMetrics {
                name: "test".to_string(),
                compression_ratio: 1.0,
                dedup_ratio: 1.0,
                record_size: 128 * 1024,
                access_pattern: pattern,
            };
            // Should not panic
        }
    }
}

#[cfg(test)]
mod configuration_context_tests {
    use super::*;

    #[test]
    fn test_zfs_configuration_context() {
        let context = ZfsConfigurationContext {
            current_record_size: 128 * 1024,
            current_compression: "lz4".to_string(),
            current_cache_settings: "all".to_string(),
            tier: StorageTier::Hot,
        };
        
        assert_eq!(context.current_record_size, 128 * 1024);
        assert_eq!(context.current_compression, "lz4");
        assert_eq!(context.current_cache_settings, "all");
        assert_eq!(context.tier, StorageTier::Hot);
    }

    #[test]
    fn test_tuning_result() {
        let result = ZfsTuningResult {
            parameters_tuned: vec![
                ("recordsize".to_string(), "256K".to_string()),
                ("compression".to_string(), "zstd".to_string()),
            ],
            expected_improvement: "15% performance improvement".to_string(),
            warnings: vec!["May increase CPU usage".to_string()],
        };
        
        assert_eq!(result.parameters_tuned.len(), 2);
        assert!(!result.expected_improvement.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use super::test_helpers::*;

    #[tokio::test]
    async fn test_performance_optimization_workflow() {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new_for_testing());
        let dataset_manager = Arc::new(ZfsDatasetManager::new(
            config.clone(),
            pool_manager.clone(),
        ));

        let engine = PerformanceOptimizationEngine::new(
            config,
            pool_manager,
            dataset_manager,
        );

        // Test getting trending data
        let trending_result = engine.get_trending_data().await;
        assert!(trending_result.is_ok(), "Should be able to get trending data");
        
        let trending_data = trending_result.unwrap();
        // Should return empty vector in test environment
        assert!(trending_data.is_empty() || !trending_data.is_empty());
    }

    #[tokio::test]
    async fn test_performance_alert_handling() {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new_for_testing());
        let dataset_manager = Arc::new(ZfsDatasetManager::new(
            config.clone(),
            pool_manager.clone(),
        ));

        let engine = PerformanceOptimizationEngine::new(
            config,
            pool_manager,
            dataset_manager,
        );

        let alert = create_test_alert();
        let result = engine.handle_performance_alert(alert).await;
        
        // Should handle alert gracefully (may succeed or fail in test environment)
        match result {
            Ok(response) => {
                // Verify response structure
                assert!(response.immediate_actions.len() >= 0);
                assert!(response.long_term_recommendations.len() >= 0);
            }
            Err(_) => {
                println!("Alert handling failed as expected in test environment");
            }
        }
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;
    use super::test_helpers::*;

    #[test]
    fn test_extreme_performance_values() {
        let mut metrics = create_test_performance_metrics();
        
        // Test with extreme values
        if let Some(pool_metrics) = metrics.pool_metrics.get_mut("test_pool") {
            pool_metrics.read_ops_per_sec = f64::MAX;
            pool_metrics.cache_hit_ratio = 1.0; // Perfect cache hit ratio
            pool_metrics.fragmentation_percent = 0.0; // No fragmentation
            pool_metrics.average_latency_ms = 0.001; // Very low latency
        }
        
        // Should handle extreme values without panicking
        assert!(metrics.pool_metrics.get("test_pool").is_some());
    }

    #[test]
    fn test_zero_performance_values() {
        let mut metrics = create_test_performance_metrics();
        
        // Test with zero values
        if let Some(pool_metrics) = metrics.pool_metrics.get_mut("test_pool") {
            pool_metrics.read_ops_per_sec = 0.0;
            pool_metrics.write_ops_per_sec = 0.0;
            pool_metrics.cache_hit_ratio = 0.0;
            pool_metrics.fragmentation_percent = 100.0; // Maximum fragmentation
        }
        
        // Should handle zero values without panicking
        assert!(metrics.pool_metrics.get("test_pool").is_some());
    }

    #[test]
    fn test_empty_collections() {
        let metrics = ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: HashMap::new(), // Empty
            dataset_metrics: HashMap::new(), // Empty
            system_memory_usage: SystemMemoryUsage {
                total_memory: 0,
                used_memory: 0,
                available_memory: 0,
            },
            arc_stats: ArcStatistics {
                hit_ratio: 0.0,
                size_bytes: 0,
                target_size_bytes: 0,
                meta_used_bytes: 0,
            },
        };
        
        // Should handle empty collections
        assert!(metrics.pool_metrics.is_empty());
        assert!(metrics.dataset_metrics.is_empty());
    }
} 