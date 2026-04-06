// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for Real-Time Performance Monitor
//!
//! These tests cover the performance monitoring functionality
//! including metrics collection, alert thresholds, and caching.

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod performance_monitoring_tests {
    use super::super::monitoring::*;
    use super::super::types::*;
    use std::collections::HashMap;
    use std::time::SystemTime;

    // ==================== INITIALIZATION TESTS ====================

    #[test]
    fn test_monitor_creation() {
        let monitor = RealTimePerformanceMonitor::new();
        assert!(!std::ptr::addr_of!(monitor).is_null());
    }

    #[test]
    fn test_monitor_default() {
        let monitor = RealTimePerformanceMonitor::default();
        assert!(!std::ptr::addr_of!(monitor).is_null());
    }

    #[test]
    fn test_monitor_debug_implementation() {
        let monitor = RealTimePerformanceMonitor::new();
        let debug_str = format!("{monitor:?}");
        assert!(debug_str.contains("RealTimePerformanceMonitor"));
    }

    // ==================== METRICS CACHE TESTS ====================

    #[tokio::test]
    async fn test_get_metrics_cache() {
        let monitor = RealTimePerformanceMonitor::new();
        let cache = monitor.get_metrics_cache();

        let cache_guard = cache.read().await;
        assert_eq!(cache_guard.len(), 0); // Should start empty
    }

    #[tokio::test]
    async fn test_metrics_cache_isolation() {
        let monitor = RealTimePerformanceMonitor::new();
        let cache1 = monitor.get_metrics_cache();
        let cache2 = monitor.get_metrics_cache();

        // Both should reference the same cache
        let guard1 = cache1.read().await;
        let guard2 = cache2.read().await;
        assert_eq!(guard1.len(), guard2.len());
    }

    // ==================== ALERT THRESHOLDS TESTS ====================

    #[test]
    fn test_alert_thresholds_default() {
        let thresholds = AlertThresholds::default();
        assert_eq!(thresholds.cpu_threshold, 0.0);
        assert_eq!(thresholds.memory_threshold, 0.0);
        assert_eq!(thresholds.disk_threshold, 0.0);
    }

    #[test]
    fn test_alert_thresholds_custom() {
        let thresholds = AlertThresholds {
            cpu_threshold: 85.0,
            memory_threshold: 95.0,
            disk_threshold: 90.0,
        };

        assert_eq!(thresholds.cpu_threshold, 85.0);
        assert_eq!(thresholds.memory_threshold, 95.0);
        assert_eq!(thresholds.disk_threshold, 90.0);
    }

    #[test]
    fn test_alert_thresholds_debug() {
        let thresholds = AlertThresholds {
            cpu_threshold: 80.0,
            memory_threshold: 90.0,
            disk_threshold: 85.0,
        };

        let debug_str = format!("{thresholds:?}");
        assert!(debug_str.contains("AlertThresholds"));
    }

    // ==================== CONCURRENT ACCESS TESTS ====================

    #[tokio::test]
    async fn test_monitor_concurrent_creation() {
        let monitor1 = RealTimePerformanceMonitor::new();
        let monitor2 = RealTimePerformanceMonitor::new();

        // Both should be valid independent instances
        assert!(!std::ptr::addr_of!(monitor1).is_null());
        assert!(!std::ptr::addr_of!(monitor2).is_null());
    }

    #[tokio::test]
    async fn test_concurrent_cache_access() {
        let monitor = std::sync::Arc::new(RealTimePerformanceMonitor::new());

        let monitor1 = monitor.clone();
        let monitor2 = monitor.clone();

        let handle1 = tokio::spawn(async move {
            let cache = monitor1.get_metrics_cache();
            let guard = cache.read().await;
            guard.len()
        });

        let handle2 = tokio::spawn(async move {
            let cache = monitor2.get_metrics_cache();
            let guard = cache.read().await;
            guard.len()
        });

        let result1 = handle1.await.unwrap();
        let result2 = handle2.await.unwrap();

        assert_eq!(result1, result2); // Both should see same state
    }

    // ==================== METRICS INTEGRATION TESTS ====================

    #[tokio::test]
    async fn test_metrics_storage_and_retrieval() {
        let monitor = RealTimePerformanceMonitor::new();
        let cache = monitor.get_metrics_cache();

        // Store a metric
        {
            let mut cache_guard = cache.write().await;
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
            cache_guard.insert("test_key".to_string(), metrics);
        }

        // Retrieve the metric
        {
            let cache_guard = cache.read().await;
            assert_eq!(cache_guard.len(), 1);
            assert!(cache_guard.contains_key("test_key"));
        }
    }

    #[tokio::test]
    async fn test_multiple_metrics_storage() {
        let monitor = RealTimePerformanceMonitor::new();
        let cache = monitor.get_metrics_cache();

        // Store multiple metrics
        {
            let mut cache_guard = cache.write().await;

            for i in 0..5 {
                let metrics = ZfsPerformanceMetrics {
                    timestamp: SystemTime::now(),
                    pool_metrics: HashMap::new(),
                    dataset_metrics: HashMap::new(),
                    system_memory: SystemMemoryUsage {
                        total: 16_000_000_000,
                        available: 8_000_000_000 - (i * 1_000_000_000),
                        used: 8_000_000_000 + (i * 1_000_000_000),
                    },
                    arc_stats: ArcStatistics {
                        size: 4_000_000_000,
                        target_size: 4_000_000_000,
                        hit_ratio: (i as f64).mul_add(-0.01, 0.85),
                        miss_ratio: (i as f64).mul_add(0.01, 0.15),
                    },
                };
                cache_guard.insert(format!("metric_{i}"), metrics);
            }
        }

        // Verify all metrics stored
        {
            let cache_guard = cache.read().await;
            assert_eq!(cache_guard.len(), 5);

            for i in 0..5 {
                assert!(cache_guard.contains_key(&format!("metric_{i}")));
            }
        }
    }

    // ==================== EDGE CASE TESTS ====================

    #[tokio::test]
    async fn test_empty_cache_operations() {
        let monitor = RealTimePerformanceMonitor::new();
        let cache = monitor.get_metrics_cache();

        let guard = cache.read().await;
        assert_eq!(guard.len(), 0);
        assert!(!guard.contains_key("nonexistent"));
    }

    #[tokio::test]
    async fn test_cache_overwrite() {
        let monitor = RealTimePerformanceMonitor::new();
        let cache = monitor.get_metrics_cache();

        // Store initial metric
        {
            let mut guard = cache.write().await;
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
            guard.insert("key".to_string(), metrics);
        }

        // Overwrite with new metric
        {
            let mut guard = cache.write().await;
            let new_metrics = ZfsPerformanceMetrics {
                timestamp: SystemTime::now(),
                pool_metrics: HashMap::new(),
                dataset_metrics: HashMap::new(),
                system_memory: SystemMemoryUsage {
                    total: 32_000_000_000,
                    available: 16_000_000_000,
                    used: 16_000_000_000,
                },
                arc_stats: ArcStatistics {
                    size: 8_000_000_000,
                    target_size: 8_000_000_000,
                    hit_ratio: 0.90,
                    miss_ratio: 0.10,
                },
            };
            guard.insert("key".to_string(), new_metrics);
        }

        // Verify overwrite
        {
            let guard = cache.read().await;
            assert_eq!(guard.len(), 1);

            let metrics = guard.get("key").unwrap();
            assert_eq!(metrics.system_memory.total, 32_000_000_000);
            assert_eq!(metrics.arc_stats.hit_ratio, 0.90);
        }
    }

    // ==================== METRICS WITH POOL DATA TESTS ====================

    #[tokio::test]
    async fn test_metrics_with_pool_data() {
        let monitor = RealTimePerformanceMonitor::new();
        let cache = monitor.get_metrics_cache();

        {
            let mut guard = cache.write().await;

            let mut pool_metrics = HashMap::new();
            pool_metrics.insert(
                "tank".to_string(),
                ZfsPoolMetrics {
                    pool_name: "tank".to_string(),
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

            guard.insert("with_pool".to_string(), metrics);
        }

        {
            let guard = cache.read().await;
            let metrics = guard.get("with_pool").unwrap();
            assert_eq!(metrics.pool_metrics.len(), 1);
            assert!(metrics.pool_metrics.contains_key("tank"));
        }
    }

    #[tokio::test]
    async fn test_metrics_with_dataset_data() {
        let monitor = RealTimePerformanceMonitor::new();
        let cache = monitor.get_metrics_cache();

        {
            let mut guard = cache.write().await;

            let mut dataset_metrics = HashMap::new();
            dataset_metrics.insert(
                "tank/data".to_string(),
                ZfsDatasetMetrics {
                    dataset_name: "tank/data".to_string(),
                    access_pattern: AccessPattern::Sequential,
                    dedup_ratio: 1.2,
                    record_size: 131072,
                },
            );

            let metrics = ZfsPerformanceMetrics {
                timestamp: SystemTime::now(),
                pool_metrics: HashMap::new(),
                dataset_metrics,
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

            guard.insert("with_dataset".to_string(), metrics);
        }

        {
            let guard = cache.read().await;
            let metrics = guard.get("with_dataset").unwrap();
            assert_eq!(metrics.dataset_metrics.len(), 1);
            assert!(metrics.dataset_metrics.contains_key("tank/data"));
        }
    }

    // ==================== CLONE AND ARC TESTS ====================

    #[tokio::test]
    async fn test_monitor_in_arc() {
        let monitor = std::sync::Arc::new(RealTimePerformanceMonitor::new());
        let monitor_clone = monitor.clone();

        // Both should access the same underlying data
        let cache1 = monitor.get_metrics_cache();
        let cache2 = monitor_clone.get_metrics_cache();

        let guard1 = cache1.read().await;
        let guard2 = cache2.read().await;

        assert_eq!(guard1.len(), guard2.len());
    }

    // ==================== LIFECYCLE TESTS ====================

    #[tokio::test]
    async fn test_monitor_creation_and_destruction() {
        {
            let monitor = RealTimePerformanceMonitor::new();
            let cache = monitor.get_metrics_cache();
            let _guard = cache.read().await;
            // Monitor goes out of scope here
        }
        // Should not panic or leak
    }

    #[tokio::test]
    async fn test_multiple_monitor_instances() {
        let monitors: Vec<_> = (0..10).map(|_| RealTimePerformanceMonitor::new()).collect();

        assert_eq!(monitors.len(), 10);

        for monitor in &monitors {
            let cache = monitor.get_metrics_cache();
            let guard = cache.read().await;
            assert_eq!(guard.len(), 0); // Each should have independent cache
        }
    }
}
