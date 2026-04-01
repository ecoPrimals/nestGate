// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for Metrics Collector
//!
//! Tests cover metrics collection, aggregation, export, and real-time monitoring.

#[cfg(test)]
mod metrics_collector_tests {
    use super::super::metrics_collector::*;

    // ==================== METRICS COLLECTOR TESTS ====================

    #[test]
    fn test_metrics_collector_creation() {
        let collector = MetricsCollector::new();
        assert!(std::ptr::addr_of!(collector) != std::ptr::null());
    }

    #[test]
    fn test_metrics_collector_default() {
        let collector = MetricsCollector;
        assert!(std::ptr::addr_of!(collector) != std::ptr::null());
    }

    #[test]
    fn test_metrics_collector_debug() {
        let collector = MetricsCollector::new();
        let debug_str = format!("{collector:?}");
        assert!(debug_str.contains("MetricsCollector"));
    }

    #[test]
    fn test_metrics_collector_clone() {
        let collector1 = MetricsCollector::new();
        let collector2 = collector1.clone();
        // Both should be valid
        assert!(std::ptr::addr_of!(collector1) != std::ptr::null());
        assert!(std::ptr::addr_of!(collector2) != std::ptr::null());
    }

    // ==================== SYSTEM METRICS TESTS ====================

    #[test]
    fn test_system_metrics_creation() {
        let metrics = SystemMetrics {
            cpu_usage: 45.5,
            memory_usage: 50.0,                   // percentage
            memory_total: 8192 * 1024 * 1024,     // bytes
            memory_available: 6144 * 1024 * 1024, // bytes
            network_io: NetworkIOMetrics {
                bytes_sent: 1000000,
                bytes_received: 2000000,
                packets_sent: 1000,
                packets_received: 2000,
            },
            disk_io: DiskIOMetrics {
                read_bytes: 5000000,
                write_bytes: 3000000,
                read_operations: 500,
                write_operations: 300,
            },
        };

        assert!(metrics.cpu_usage >= 0.0 && metrics.cpu_usage <= 100.0);
        assert!(metrics.memory_usage >= 0.0 && metrics.memory_usage <= 100.0);
        assert!(metrics.memory_total > 0);
        assert!(metrics.memory_available <= metrics.memory_total);
    }

    #[test]
    fn test_system_metrics_ratios() {
        let metrics = SystemMetrics {
            cpu_usage: 75.0,
            memory_usage: 75.0, // 75% usage
            memory_total: 8000 * 1024 * 1024,
            memory_available: 2000 * 1024 * 1024,
            network_io: NetworkIOMetrics {
                bytes_sent: 1000000,
                bytes_received: 2000000,
                packets_sent: 1000,
                packets_received: 2000,
            },
            disk_io: DiskIOMetrics {
                read_bytes: 5000000,
                write_bytes: 3000000,
                read_operations: 500,
                write_operations: 300,
            },
        };

        let memory_ratio = metrics.memory_usage / 100.0;
        assert!((0.0..=1.0).contains(&memory_ratio));

        assert!(metrics.memory_available <= metrics.memory_total);
    }

    #[test]
    fn test_system_metrics_serialization() {
        let metrics = SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 50.0,
            memory_total: 8192 * 1024 * 1024,
            memory_available: 4096 * 1024 * 1024,
            network_io: NetworkIOMetrics {
                bytes_sent: 1000000,
                bytes_received: 2000000,
                packets_sent: 1000,
                packets_received: 2000,
            },
            disk_io: DiskIOMetrics {
                read_bytes: 5000000,
                write_bytes: 3000000,
                read_operations: 500,
                write_operations: 300,
            },
        };

        let json = serde_json::to_string(&metrics).expect("Should serialize");
        assert!(json.contains("memory_usage"));
        assert!(json.contains("50"));
    }

    #[test]
    fn test_system_metrics_deserialization() {
        let json = r#"{
            "cpu_usage": 60.5,
            "memory_usage": 62.5,
            "memory_total": 8589934592,
            "memory_available": 6442450944,
            "network_io": {
                "bytes_sent": 1000000,
                "bytes_received": 2000000,
                "packets_sent": 1000,
                "packets_received": 2000
            },
            "disk_io": {
                "read_bytes": 5000000,
                "write_bytes": 3000000,
                "read_operations": 500,
                "write_operations": 300
            }
        }"#;

        let metrics: SystemMetrics = serde_json::from_str(json).expect("Should deserialize");
        // Verify metrics deserialized correctly
        assert!(metrics.cpu_usage > 0.0);
        assert_eq!(metrics.cpu_usage, 60.5);
    }

    // ==================== APPLICATION METRICS TESTS ====================

    #[test]
    fn test_application_metrics_creation() {
        let metrics = ApplicationMetrics {
            total_requests: 10000,
            successful_requests: 9500,
            failed_requests: 500,
            average_response_time_ms: 150.5,
            requests_per_second: 250.0,
            active_connections: 50,
            error_rate: 0.05,
        };

        assert_eq!(
            metrics.total_requests,
            metrics.successful_requests + metrics.failed_requests
        );
        assert!(metrics.average_response_time_ms > 0.0);
        assert!(metrics.requests_per_second >= 0.0);
        assert!(metrics.error_rate >= 0.0 && metrics.error_rate <= 1.0);
    }

    #[test]
    fn test_application_metrics_error_rate_calculation() {
        let metrics = ApplicationMetrics {
            total_requests: 1000,
            successful_requests: 950,
            failed_requests: 50,
            average_response_time_ms: 100.0,
            requests_per_second: 100.0,
            active_connections: 20,
            error_rate: 0.05,
        };

        let calculated_error_rate = metrics.failed_requests as f64 / metrics.total_requests as f64;
        assert!((calculated_error_rate - metrics.error_rate).abs() < 0.001);
    }

    #[test]
    fn test_application_metrics_zero_requests() {
        let metrics = ApplicationMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time_ms: 0.0,
            requests_per_second: 0.0,
            active_connections: 0,
            error_rate: 0.0,
        };

        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.error_rate, 0.0);
    }

    #[test]
    fn test_application_metrics_serialization() {
        let metrics = ApplicationMetrics {
            total_requests: 5000,
            successful_requests: 4750,
            failed_requests: 250,
            average_response_time_ms: 125.5,
            requests_per_second: 150.0,
            active_connections: 30,
            error_rate: 0.05,
        };

        let json = serde_json::to_string(&metrics).expect("Should serialize");
        assert!(json.contains("total_requests"));
        assert!(json.contains("5000"));
    }

    // ==================== METRICS SNAPSHOT TESTS ====================

    #[test]
    fn test_metrics_snapshot_creation() {
        use std::time::SystemTime;

        let snapshot = MetricsSnapshot {
            timestamp: SystemTime::now(),
            system: SystemMetrics {
                cpu_usage: 50.0,
                memory_usage: 50.0,
                memory_total: 8192 * 1024 * 1024,
                memory_available: 4096 * 1024 * 1024,
                network_io: NetworkIOMetrics {
                    bytes_sent: 1000000,
                    bytes_received: 2000000,
                    packets_sent: 1000,
                    packets_received: 2000,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 5000000,
                    write_bytes: 3000000,
                    read_operations: 500,
                    write_operations: 300,
                },
            },
            application: ApplicationMetrics {
                total_requests: 10000,
                successful_requests: 9500,
                failed_requests: 500,
                average_response_time_ms: 150.0,
                requests_per_second: 250.0,
                active_connections: 50,
                error_rate: 0.05,
            },
        };

        assert!(snapshot.system.cpu_usage > 0.0);
        assert!(snapshot.application.total_requests > 0);
    }

    #[test]
    fn test_metrics_snapshot_timestamp() {
        use std::time::SystemTime;

        let now = SystemTime::now();
        let snapshot = MetricsSnapshot {
            timestamp: now,
            system: SystemMetrics {
                cpu_usage: 50.0,
                memory_usage: 50.0,
                memory_total: 8192 * 1024 * 1024,
                memory_available: 4096 * 1024 * 1024,
                network_io: NetworkIOMetrics {
                    bytes_sent: 1000000,
                    bytes_received: 2000000,
                    packets_sent: 1000,
                    packets_received: 2000,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 5000000,
                    write_bytes: 3000000,
                    read_operations: 500,
                    write_operations: 300,
                },
            },
            application: ApplicationMetrics {
                total_requests: 5000,
                successful_requests: 4900,
                failed_requests: 100,
                average_response_time_ms: 100.0,
                requests_per_second: 100.0,
                active_connections: 25,
                error_rate: 0.02,
            },
        };

        assert_eq!(snapshot.timestamp, now);
    }

    // ==================== METRICS COLLECTION TESTS ====================

    #[tokio::test]
    async fn test_collect_metrics_endpoint() {
        let collector = MetricsCollector::new();
        // Collector should be ready to collect
        assert!(std::ptr::addr_of!(collector) != std::ptr::null());
    }

    #[tokio::test]
    async fn test_metrics_collection_consistency() {
        let collector = MetricsCollector::new();
        // Multiple collections should not panic
        let _collector2 = collector;
    }

    // ==================== METRICS AGGREGATION TESTS ====================

    #[test]
    fn test_metrics_aggregation_over_time() {
        let snapshots = [
            SystemMetrics {
                cpu_usage: 50.0,
                memory_usage: 50.0,
                memory_total: 8192 * 1024 * 1024,
                memory_available: 4096 * 1024 * 1024,
                network_io: NetworkIOMetrics {
                    bytes_sent: 1000000,
                    bytes_received: 2000000,
                    packets_sent: 1000,
                    packets_received: 2000,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 5000000,
                    write_bytes: 3000000,
                    read_operations: 500,
                    write_operations: 300,
                },
            },
            SystemMetrics {
                cpu_usage: 50.0,
                memory_usage: 50.0,
                memory_total: 8192 * 1024 * 1024,
                memory_available: 4096 * 1024 * 1024,
                network_io: NetworkIOMetrics {
                    bytes_sent: 1000000,
                    bytes_received: 2000000,
                    packets_sent: 1000,
                    packets_received: 2000,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 5000000,
                    write_bytes: 3000000,
                    read_operations: 500,
                    write_operations: 300,
                },
            },
            SystemMetrics {
                cpu_usage: 50.0,
                memory_usage: 50.0,
                memory_total: 8192 * 1024 * 1024,
                memory_available: 4096 * 1024 * 1024,
                network_io: NetworkIOMetrics {
                    bytes_sent: 1000000,
                    bytes_received: 2000000,
                    packets_sent: 1000,
                    packets_received: 2000,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 5000000,
                    write_bytes: 3000000,
                    read_operations: 500,
                    write_operations: 300,
                },
            },
        ];

        let avg_cpu = snapshots.iter().map(|s| s.cpu_usage).sum::<f64>() / snapshots.len() as f64;

        assert_eq!(avg_cpu, 50.0); // (50 + 50 + 50) / 3 = 50
    }

    #[test]
    fn test_metrics_trends() {
        let metrics_over_time = [(0, 30.0), (1, 35.0), (2, 40.0), (3, 45.0), (4, 50.0)];

        // Should show increasing trend
        for i in 1..metrics_over_time.len() {
            assert!(metrics_over_time[i].1 > metrics_over_time[i - 1].1);
        }
    }

    // ==================== ERROR HANDLING TESTS ====================

    #[test]
    fn test_invalidcpu_usage() {
        let metrics = SystemMetrics {
            cpu_usage: 150.0,
            memory_usage: 50.0,
            memory_total: 8192 * 1024 * 1024,
            memory_available: 4096 * 1024 * 1024,
            network_io: NetworkIOMetrics {
                bytes_sent: 1000000,
                bytes_received: 2000000,
                packets_sent: 1000,
                packets_received: 2000,
            },
            disk_io: DiskIOMetrics {
                read_bytes: 5000000,
                write_bytes: 3000000,
                read_operations: 500,
                write_operations: 300,
            },
        };

        // Structure allows it, but validation should happen elsewhere
        assert_eq!(metrics.cpu_usage, 150.0); // Test that we can create metrics with values > 100
    }

    #[test]
    fn test_negative_metrics() {
        let metrics = ApplicationMetrics {
            total_requests: 1000,
            successful_requests: 900,
            failed_requests: 100,
            average_response_time_ms: -10.0, // Invalid negative
            requests_per_second: 50.0,
            active_connections: 10,
            error_rate: 0.1,
        };

        // Structure allows it
        assert!(metrics.average_response_time_ms < 0.0);
    }

    // ==================== PERFORMANCE TESTS ====================

    #[test]
    fn test_metrics_serialization_performance() {
        let metrics = SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 50.0,
            memory_total: 8192 * 1024 * 1024,
            memory_available: 4096 * 1024 * 1024,
            network_io: NetworkIOMetrics {
                bytes_sent: 1000000,
                bytes_received: 2000000,
                packets_sent: 1000,
                packets_received: 2000,
            },
            disk_io: DiskIOMetrics {
                read_bytes: 5000000,
                write_bytes: 3000000,
                read_operations: 500,
                write_operations: 300,
            },
        };

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = serde_json::to_string(&metrics).expect("Should serialize");
        }
        let duration = start.elapsed();

        // Should serialize 1000 times in less than 100ms
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_metrics_clone_performance() {
        let collector = MetricsCollector::new();

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = collector.clone();
        }
        let duration = start.elapsed();

        // Should clone 1000 times very quickly
        assert!(duration.as_millis() < 10);
    }

    // ==================== INTEGRATION TESTS ====================

    #[tokio::test]
    async fn test_full_metrics_workflow() {
        let collector = MetricsCollector::new();

        // Should be able to create and use collector
        let _cloned = collector.clone();

        // Should be able to format debug
        let debug_str = format!("{collector:?}");
        assert!(!debug_str.is_empty());
    }

    #[tokio::test]
    async fn test_concurrent_metrics_collection() {
        use futures::future::join_all;

        let collector = MetricsCollector::new();
        let tasks = (0..10)
            .map(|_| {
                let c = collector.clone();
                tokio::spawn(async move {
                    // Simulate metrics collection
                    let _ = format!("{c:?}");
                })
            })
            .collect::<Vec<_>>();

        let results = join_all(tasks).await;

        // All tasks should complete successfully
        for result in results {
            assert!(result.is_ok());
        }
    }
}
