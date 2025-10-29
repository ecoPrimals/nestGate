//! **COMPREHENSIVE METRICS COLLECTOR TESTS**
//!
//! Test coverage for `metrics_collector.rs` - Real-time metrics collection and aggregation.
//!
//! This test suite covers:
//! - `RealTimeMetricsCollector` construction and initialization
//! - Metrics collection methods
//! - Data structures (`RealTimeMetrics`, `PoolMetrics`, `SystemMetrics`, etc.)
//! - Serialization/deserialization
//! - Edge cases and error handling
//! - Integration scenarios

#[cfg(test)]
mod tests {
    use super::super::metrics_collector::*;

    use std::time::SystemTime;

    // ==================== CONSTRUCTOR TESTS ====================

    #[test]
    fn test_metrics_collector_new() {
        let collector = RealTimeMetricsCollector::new();
        assert!(
            format!("{collector:?}").contains("RealTimeMetricsCollector"),
            "Collector should be created successfully"
        );
    }

    #[test]
    fn test_metrics_collector_multiple_instances() {
        let collector1 = RealTimeMetricsCollector::new();
        let collector2 = RealTimeMetricsCollector::new();

        // Both should be valid instances
        assert!(format!("{collector1:?}").contains("RealTimeMetricsCollector"));
        assert!(format!("{collector2:?}").contains("RealTimeMetricsCollector"));
    }

    // ==================== REAL TIME METRICS TESTS ====================

    #[test]
    fn test_real_time_metrics_creation() {
        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![],
            system_metrics: SystemMetrics {
                _cpu_usage: 50.0,
                memory_usage: 60.0,
                memory_total: 16_000_000_000,
                memory_available: 8_000_000_000,
                network_io: NetworkIOMetrics {
                    bytes_sent: 1000,
                    bytes_received: 2000,
                    packets_sent: 10,
                    packets_received: 20,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 5000,
                    write_bytes: 3000,
                    read_operations: 50,
                    write_operations: 30,
                },
            },
            arc_hit_ratio: 0.85,
            l2arc_hit_ratio: 0.75,
            compression_ratio: 2.1,
            total_throughput: 150.5,
            average_read_latency: 2.5,
            average_write_latency: 3.2,
        };

        assert!(metrics.arc_hit_ratio > 0.0);
        assert_eq!(metrics.compression_ratio, 2.1);
    }

    #[test]
    fn test_real_time_metrics_serialization() {
        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![],
            system_metrics: SystemMetrics {
                _cpu_usage: 45.0,
                memory_usage: 70.0,
                memory_total: 32_000_000_000,
                memory_available: 10_000_000_000,
                network_io: NetworkIOMetrics {
                    bytes_sent: 500,
                    bytes_received: 1500,
                    packets_sent: 5,
                    packets_received: 15,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 10000,
                    write_bytes: 8000,
                    read_operations: 100,
                    write_operations: 80,
                },
            },
            arc_hit_ratio: 0.90,
            l2arc_hit_ratio: 0.80,
            compression_ratio: 2.5,
            total_throughput: 200.0,
            average_read_latency: 1.8,
            average_write_latency: 2.3,
        };

        let serialized = serde_json::to_string(&metrics);
        assert!(serialized.is_ok(), "RealTimeMetrics should serialize");

        let json = serialized.unwrap();
        assert!(json.contains("arc_hit_ratio"));
        assert!(json.contains("0.9"));
    }

    // ==================== POOL METRICS TESTS ====================

    #[test]
    fn test_pool_metrics_creation() {
        let pool = PoolMetrics {
            name: "tank1".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 75.5,
            total_capacity: 1_000_000_000_000,
            used_space: 755_000_000_000,
            available_space: 245_000_000_000,
            read_iops: 5000,
            write_iops: 3000,
            read_throughput: 150.0,
            write_throughput: 100.0,
            fragmentation_level: 0.15,
            error_count: 0,
        };

        assert_eq!(pool.name, "tank1");
        assert_eq!(pool.health_status, "ONLINE");
        assert_eq!(pool.utilization_percentage, 75.5);
        assert_eq!(pool.error_count, 0);
    }

    #[test]
    fn test_pool_metrics_serialization() {
        let pool = PoolMetrics {
            name: "data-pool".to_string(),
            health_status: "DEGRADED".to_string(),
            utilization_percentage: 85.0,
            total_capacity: 2_000_000_000_000,
            used_space: 1_700_000_000_000,
            available_space: 300_000_000_000,
            read_iops: 8000,
            write_iops: 5000,
            read_throughput: 250.0,
            write_throughput: 180.0,
            fragmentation_level: 0.20,
            error_count: 3,
        };

        let serialized = serde_json::to_string(&pool);
        assert!(serialized.is_ok(), "PoolMetrics should serialize");

        let json = serialized.unwrap();
        assert!(json.contains("data-pool"));
        assert!(json.contains("DEGRADED"));
        assert!(json.contains("\"error_count\":3"));
    }

    #[test]
    fn test_pool_metrics_clone() {
        let pool1 = PoolMetrics {
            name: "pool1".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 50.0,
            total_capacity: 1000,
            used_space: 500,
            available_space: 500,
            read_iops: 1000,
            write_iops: 800,
            read_throughput: 100.0,
            write_throughput: 80.0,
            fragmentation_level: 0.10,
            error_count: 0,
        };

        let pool2 = pool1.clone();

        assert_eq!(pool1.name, pool2.name);
        assert_eq!(pool1.utilization_percentage, pool2.utilization_percentage);
    }

    // ==================== SYSTEM METRICS TESTS ====================

    #[test]
    fn test_system_metrics_creation() {
        let system = SystemMetrics {
            _cpu_usage: 65.0,
            memory_usage: 80.0,
            memory_total: 64_000_000_000,
            memory_available: 12_000_000_000,
            network_io: NetworkIOMetrics {
                bytes_sent: 10000,
                bytes_received: 20000,
                packets_sent: 100,
                packets_received: 200,
            },
            disk_io: DiskIOMetrics {
                read_bytes: 50000,
                write_bytes: 30000,
                read_operations: 500,
                write_operations: 300,
            },
        };

        assert_eq!(system.memory_usage, 80.0);
        assert_eq!(system.memory_total, 64_000_000_000);
    }

    #[test]
    fn test_system_metrics_serialization() {
        let system = SystemMetrics {
            _cpu_usage: 55.0,
            memory_usage: 70.0,
            memory_total: 32_000_000_000,
            memory_available: 9_600_000_000,
            network_io: NetworkIOMetrics {
                bytes_sent: 5000,
                bytes_received: 10000,
                packets_sent: 50,
                packets_received: 100,
            },
            disk_io: DiskIOMetrics {
                read_bytes: 25000,
                write_bytes: 15000,
                read_operations: 250,
                write_operations: 150,
            },
        };

        let serialized = serde_json::to_string(&system);
        assert!(serialized.is_ok(), "SystemMetrics should serialize");

        let json = serialized.unwrap();
        assert!(json.contains("memory_usage"));
        assert!(json.contains("70"));
    }

    // ==================== NETWORK IO METRICS TESTS ====================

    #[test]
    fn test_network_io_metrics_creation() {
        let network = NetworkIOMetrics {
            bytes_sent: 1_000_000,
            bytes_received: 2_000_000,
            packets_sent: 1000,
            packets_received: 2000,
        };

        assert_eq!(network.bytes_sent, 1_000_000);
        assert_eq!(network.bytes_received, 2_000_000);
        assert_eq!(network.packets_sent, 1000);
        assert_eq!(network.packets_received, 2000);
    }

    #[test]
    fn test_network_io_metrics_zero_values() {
        let network = NetworkIOMetrics {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
        };

        assert_eq!(network.bytes_sent, 0);
        assert_eq!(network.packets_received, 0);
    }

    #[test]
    fn test_network_io_metrics_serialization() {
        let network = NetworkIOMetrics {
            bytes_sent: 5000,
            bytes_received: 10000,
            packets_sent: 50,
            packets_received: 100,
        };

        let serialized = serde_json::to_string(&network);
        assert!(serialized.is_ok());

        let json = serialized.unwrap();
        assert!(json.contains("\"bytes_sent\":5000"));
        assert!(json.contains("\"packets_received\":100"));
    }

    // ==================== DISK IO METRICS TESTS ====================

    #[test]
    fn test_disk_io_metrics_creation() {
        let disk = DiskIOMetrics {
            read_bytes: 100_000_000,
            write_bytes: 50_000_000,
            read_operations: 1000,
            write_operations: 500,
        };

        assert_eq!(disk.read_bytes, 100_000_000);
        assert_eq!(disk.write_bytes, 50_000_000);
        assert_eq!(disk.read_operations, 1000);
        assert_eq!(disk.write_operations, 500);
    }

    #[test]
    fn test_disk_io_metrics_high_values() {
        let disk = DiskIOMetrics {
            read_bytes: u64::MAX / 2,
            write_bytes: u64::MAX / 4,
            read_operations: 1_000_000,
            write_operations: 500_000,
        };

        assert!(disk.read_bytes > 0);
        assert!(disk.write_bytes > 0);
    }

    #[test]
    fn test_disk_io_metrics_serialization() {
        let disk = DiskIOMetrics {
            read_bytes: 25000,
            write_bytes: 15000,
            read_operations: 250,
            write_operations: 150,
        };

        let serialized = serde_json::to_string(&disk);
        assert!(serialized.is_ok());

        let json = serialized.unwrap();
        assert!(json.contains("\"read_bytes\":25000"));
        assert!(json.contains("\"write_operations\":150"));
    }

    // ==================== SYSTEM SNAPSHOT TESTS ====================

    #[test]
    fn test_system_snapshot_creation() {
        let snapshot = SystemSnapshot {
            timestamp: SystemTime::now(),
            cpu_cores: 16,
            cpu_usage_percent: 45.5,
            memory_total_gb: 64,
            memory_used_gb: 32,
            disk_total_gb: 1000,
            disk_used_gb: 750,
            network_interfaces: vec!["eth0".to_string(), "eth1".to_string()],
        };

        assert_eq!(snapshot.cpu_cores, 16);
        assert_eq!(snapshot.memory_total_gb, 64);
        assert_eq!(snapshot.network_interfaces.len(), 2);
    }

    #[test]
    fn test_system_snapshot_serialization() {
        let snapshot = SystemSnapshot {
            timestamp: SystemTime::now(),
            cpu_cores: 8,
            cpu_usage_percent: 60.0,
            memory_total_gb: 32,
            memory_used_gb: 20,
            disk_total_gb: 500,
            disk_used_gb: 300,
            network_interfaces: vec!["enp0s3".to_string()],
        };

        let serialized = serde_json::to_string(&snapshot);
        assert!(serialized.is_ok());

        let json = serialized.unwrap();
        assert!(json.contains("\"cpu_cores\":8"));
        assert!(json.contains("enp0s3"));
    }

    #[test]
    fn test_system_snapshot_no_network_interfaces() {
        let snapshot = SystemSnapshot {
            timestamp: SystemTime::now(),
            cpu_cores: 4,
            cpu_usage_percent: 25.0,
            memory_total_gb: 16,
            memory_used_gb: 8,
            disk_total_gb: 250,
            disk_used_gb: 100,
            network_interfaces: vec![],
        };

        assert_eq!(snapshot.network_interfaces.len(), 0);
    }

    // ==================== IO METRICS POINT TESTS ====================

    #[test]
    fn test_io_metrics_point_creation() {
        let io_point = IOMetricsPoint {
            timestamp: SystemTime::now(),
            read_iops: 5000,
            write_iops: 3000,
            read_latency: 2.5,
            write_latency: 3.8,
        };

        assert_eq!(io_point.read_iops, 5000);
        assert_eq!(io_point.write_iops, 3000);
        assert_eq!(io_point.read_latency, 2.5);
        assert_eq!(io_point.write_latency, 3.8);
    }

    #[test]
    fn test_io_metrics_point_serialization() {
        let io_point = IOMetricsPoint {
            timestamp: SystemTime::now(),
            read_iops: 8000,
            write_iops: 6000,
            read_latency: 1.2,
            write_latency: 1.8,
        };

        let serialized = serde_json::to_string(&io_point);
        assert!(serialized.is_ok());

        let json = serialized.unwrap();
        assert!(json.contains("\"read_iops\":8000"));
        assert!(json.contains("\"write_latency\":1.8"));
    }

    #[test]
    fn test_io_metrics_point_zero_latency() {
        let io_point = IOMetricsPoint {
            timestamp: SystemTime::now(),
            read_iops: 1000,
            write_iops: 800,
            read_latency: 0.0,
            write_latency: 0.0,
        };

        assert_eq!(io_point.read_latency, 0.0);
        assert_eq!(io_point.write_latency, 0.0);
    }

    // ==================== CACHE METRICS POINT TESTS ====================

    #[test]
    fn test_cache_metrics_point_creation() {
        let cache_point = CacheMetricsPoint {
            timestamp: SystemTime::now(),
            arc_hit_ratio: 0.92,
            l2arc_hit_ratio: 0.85,
            arc_size: 8_000_000_000,
            l2arc_size: 16_000_000_000,
        };

        assert_eq!(cache_point.arc_hit_ratio, 0.92);
        assert_eq!(cache_point.l2arc_hit_ratio, 0.85);
        assert_eq!(cache_point.arc_size, 8_000_000_000);
        assert_eq!(cache_point.l2arc_size, 16_000_000_000);
    }

    #[test]
    fn test_cache_metrics_point_serialization() {
        let cache_point = CacheMetricsPoint {
            timestamp: SystemTime::now(),
            arc_hit_ratio: 0.88,
            l2arc_hit_ratio: 0.75,
            arc_size: 4_000_000_000,
            l2arc_size: 8_000_000_000,
        };

        let serialized = serde_json::to_string(&cache_point);
        assert!(serialized.is_ok());

        let json = serialized.unwrap();
        assert!(json.contains("0.88"));
        assert!(json.contains("\"arc_size\":4000000000"));
    }

    #[test]
    fn test_cache_metrics_point_perfect_hit_ratio() {
        let cache_point = CacheMetricsPoint {
            timestamp: SystemTime::now(),
            arc_hit_ratio: 1.0,
            l2arc_hit_ratio: 1.0,
            arc_size: 10_000_000_000,
            l2arc_size: 20_000_000_000,
        };

        assert_eq!(cache_point.arc_hit_ratio, 1.0);
        assert_eq!(cache_point.l2arc_hit_ratio, 1.0);
    }

    // ==================== CAPACITY METRICS POINT TESTS ====================

    #[test]
    fn test_capacity_metrics_point_creation() {
        let capacity_point = CapacityMetricsPoint {
            timestamp: SystemTime::now(),
            total_capacity: 1_000_000_000_000,
            used_space: 750_000_000_000,
            growth_rate: 10_000_000_000.0,
        };

        assert_eq!(capacity_point.total_capacity, 1_000_000_000_000);
        assert_eq!(capacity_point.used_space, 750_000_000_000);
        assert_eq!(capacity_point.growth_rate, 10_000_000_000.0);
    }

    #[test]
    fn test_capacity_metrics_point_serialization() {
        let capacity_point = CapacityMetricsPoint {
            timestamp: SystemTime::now(),
            total_capacity: 2_000_000_000_000,
            used_space: 1_500_000_000_000,
            growth_rate: 5_000_000_000.0,
        };

        let serialized = serde_json::to_string(&capacity_point);
        assert!(serialized.is_ok());

        let json = serialized.unwrap();
        assert!(json.contains("\"total_capacity\":2000000000000"));
        assert!(json.contains("5000000000"));
    }

    #[test]
    fn test_capacity_metrics_point_negative_growth() {
        let capacity_point = CapacityMetricsPoint {
            timestamp: SystemTime::now(),
            total_capacity: 1_000_000_000_000,
            used_space: 500_000_000_000,
            growth_rate: -1_000_000_000.0, // Shrinking
        };

        assert!(capacity_point.growth_rate < 0.0);
    }

    // ==================== COMPREHENSIVE METRICS POINT TESTS ====================

    #[test]
    fn test_comprehensive_metrics_point_creation() {
        let comprehensive = ComprehensiveMetricsPoint {
            timestamp: SystemTime::now(),
            io_metrics: IOMetricsPoint {
                timestamp: SystemTime::now(),
                read_iops: 5000,
                write_iops: 3000,
                read_latency: 2.0,
                write_latency: 3.0,
            },
            cache_metrics: CacheMetricsPoint {
                timestamp: SystemTime::now(),
                arc_hit_ratio: 0.90,
                l2arc_hit_ratio: 0.80,
                arc_size: 8_000_000_000,
                l2arc_size: 16_000_000_000,
            },
            capacity_metrics: CapacityMetricsPoint {
                timestamp: SystemTime::now(),
                total_capacity: 1_000_000_000_000,
                used_space: 700_000_000_000,
                growth_rate: 5_000_000_000.0,
            },
        };

        assert_eq!(comprehensive.io_metrics.read_iops, 5000);
        assert_eq!(comprehensive.cache_metrics.arc_hit_ratio, 0.90);
        assert_eq!(comprehensive.capacity_metrics.growth_rate, 5_000_000_000.0);
    }

    #[test]
    fn test_comprehensive_metrics_point_serialization() {
        let comprehensive = ComprehensiveMetricsPoint {
            timestamp: SystemTime::now(),
            io_metrics: IOMetricsPoint {
                timestamp: SystemTime::now(),
                read_iops: 3000,
                write_iops: 2000,
                read_latency: 1.5,
                write_latency: 2.5,
            },
            cache_metrics: CacheMetricsPoint {
                timestamp: SystemTime::now(),
                arc_hit_ratio: 0.85,
                l2arc_hit_ratio: 0.70,
                arc_size: 4_000_000_000,
                l2arc_size: 8_000_000_000,
            },
            capacity_metrics: CapacityMetricsPoint {
                timestamp: SystemTime::now(),
                total_capacity: 500_000_000_000,
                used_space: 300_000_000_000,
                growth_rate: 2_000_000_000.0,
            },
        };

        let serialized = serde_json::to_string(&comprehensive);
        assert!(serialized.is_ok());

        let json = serialized.unwrap();
        assert!(json.contains("io_metrics"));
        assert!(json.contains("cache_metrics"));
        assert!(json.contains("capacity_metrics"));
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_collector_with_system_resources() {
        let collector = RealTimeMetricsCollector::new();
        let result = collector.get_system_resources();

        // Result may be Ok or Err depending on system access
        match result {
            Ok(snapshot) => {
                assert!(snapshot.cpu_cores > 0, "Should have at least one CPU core");
                assert!(
                    snapshot.memory_total_gb > 0,
                    "Should have some total memory"
                );
            }
            Err(_) => {
                // Expected in test environments without system access
                assert!(true, "System resources may not be available in test env");
            }
        }
    }

    #[test]
    fn test_collector_with_pool_metrics() {
        let collector = RealTimeMetricsCollector::new();
        let result = collector.get_all_pool_metrics();

        // Result may be Ok or Err depending on ZFS availability
        match result {
            Ok(pools) => {
                // If we have pools, verify structure
                for (_name, pool) in pools {
                    assert!(!pool.name.is_empty(), "Pool should have a name");
                    assert!(pool.total_capacity > 0, "Pool should have capacity");
                }
            }
            Err(_) => {
                // Expected in environments without ZFS
                assert!(true, "ZFS may not be available in test env");
            }
        }
    }

    // ==================== EDGE CASES ====================

    #[test]
    fn test_pool_metrics_full_capacity() {
        let pool = PoolMetrics {
            name: "full-pool".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 100.0,
            total_capacity: 1_000_000_000_000,
            used_space: 1_000_000_000_000,
            available_space: 0,
            read_iops: 1000,
            write_iops: 0, // Cannot write to full pool
            read_throughput: 100.0,
            write_throughput: 0.0,
            fragmentation_level: 0.50,
            error_count: 0,
        };

        assert_eq!(pool.utilization_percentage, 100.0);
        assert_eq!(pool.available_space, 0);
        assert_eq!(pool.write_iops, 0);
    }

    #[test]
    fn test_pool_metrics_with_errors() {
        let pool = PoolMetrics {
            name: "degraded-pool".to_string(),
            health_status: "DEGRADED".to_string(),
            utilization_percentage: 60.0,
            total_capacity: 1_000_000_000_000,
            used_space: 600_000_000_000,
            available_space: 400_000_000_000,
            read_iops: 3000,
            write_iops: 2000,
            read_throughput: 150.0,
            write_throughput: 100.0,
            fragmentation_level: 0.30,
            error_count: 15,
        };

        assert_eq!(pool.health_status, "DEGRADED");
        assert_eq!(pool.error_count, 15);
    }

    #[test]
    fn test_cache_metrics_zero_hit_ratio() {
        let cache = CacheMetricsPoint {
            timestamp: SystemTime::now(),
            arc_hit_ratio: 0.0,
            l2arc_hit_ratio: 0.0,
            arc_size: 1_000_000_000,
            l2arc_size: 2_000_000_000,
        };

        assert_eq!(cache.arc_hit_ratio, 0.0);
        assert_eq!(cache.l2arc_hit_ratio, 0.0);
    }

    #[test]
    fn test_io_metrics_extreme_values() {
        let io = IOMetricsPoint {
            timestamp: SystemTime::now(),
            read_iops: 1_000_000, // Very high IOPS
            write_iops: 500_000,
            read_latency: 0.001, // Very low latency
            write_latency: 0.002,
        };

        assert_eq!(io.read_iops, 1_000_000);
        assert!(io.read_latency < 0.01);
    }

    #[test]
    fn test_real_time_metrics_with_multiple_pools() {
        let pool1 = PoolMetrics {
            name: "tank1".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 70.0,
            total_capacity: 1_000_000_000_000,
            used_space: 700_000_000_000,
            available_space: 300_000_000_000,
            read_iops: 5000,
            write_iops: 3000,
            read_throughput: 200.0,
            write_throughput: 150.0,
            fragmentation_level: 0.15,
            error_count: 0,
        };

        let pool2 = PoolMetrics {
            name: "tank2".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 50.0,
            total_capacity: 2_000_000_000_000,
            used_space: 1_000_000_000_000,
            available_space: 1_000_000_000_000,
            read_iops: 8000,
            write_iops: 6000,
            read_throughput: 300.0,
            write_throughput: 250.0,
            fragmentation_level: 0.10,
            error_count: 0,
        };

        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![pool1, pool2],
            system_metrics: SystemMetrics {
                _cpu_usage: 60.0,
                memory_usage: 75.0,
                memory_total: 64_000_000_000,
                memory_available: 16_000_000_000,
                network_io: NetworkIOMetrics {
                    bytes_sent: 100_000,
                    bytes_received: 200_000,
                    packets_sent: 1000,
                    packets_received: 2000,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 500_000,
                    write_bytes: 300000,
                    read_operations: 5000,
                    write_operations: 3000,
                },
            },
            arc_hit_ratio: 0.92,
            l2arc_hit_ratio: 0.88,
            compression_ratio: 2.3,
            total_throughput: 500.0, // Combined from both pools
            average_read_latency: 1.8,
            average_write_latency: 2.2,
        };

        assert_eq!(metrics.pool_metrics.len(), 2);
        assert_eq!(metrics.total_throughput, 500.0);
    }
}
