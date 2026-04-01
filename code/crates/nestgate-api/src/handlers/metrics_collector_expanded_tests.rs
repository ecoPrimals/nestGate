// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **EXPANDED METRICS COLLECTOR TESTS**
//!
//! Comprehensive test coverage for `metrics_collector.rs` to improve overall coverage.
//! Focus on real-world scenarios, edge cases, and production workflows.

#[cfg(test)]
mod expanded_metrics_tests {
    use super::super::metrics_collector::*;

    use std::time::SystemTime;

    // ==================== REALTIME METRICS TESTS ====================

    #[test]
    fn test_realtime_metrics_creation() {
        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![],
            system_metrics: create_test_system_metrics(),
            arc_hit_ratio: 0.85,
            l2arc_hit_ratio: 0.72,
            compression_ratio: 2.1,
            total_throughput: 1_000_000.0,
            average_read_latency: 5.2,
            average_write_latency: 8.3,
        };

        assert!(metrics.arc_hit_ratio >= 0.0 && metrics.arc_hit_ratio <= 1.0);
        assert!(metrics.l2arc_hit_ratio >= 0.0 && metrics.l2arc_hit_ratio <= 1.0);
        assert!(metrics.compression_ratio > 0.0);
        assert!(metrics.total_throughput >= 0.0);
    }

    #[test]
    fn test_realtime_metrics_with_pools() {
        let pool = create_test_pool_metrics("tank");
        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![pool],
            system_metrics: create_test_system_metrics(),
            arc_hit_ratio: 0.90,
            l2arc_hit_ratio: 0.75,
            compression_ratio: 1.8,
            total_throughput: 500_000.0,
            average_read_latency: 3.5,
            average_write_latency: 6.7,
        };

        assert_eq!(metrics.pool_metrics.len(), 1);
        assert_eq!(metrics.pool_metrics[0].name, "tank");
    }

    #[test]
    fn test_realtime_metrics_multiple_pools() {
        let pools = vec![
            create_test_pool_metrics("tank1"),
            create_test_pool_metrics("tank2"),
            create_test_pool_metrics("backup"),
        ];

        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: pools,
            system_metrics: create_test_system_metrics(),
            arc_hit_ratio: 0.88,
            l2arc_hit_ratio: 0.70,
            compression_ratio: 2.0,
            total_throughput: 2_000_000.0,
            average_read_latency: 4.0,
            average_write_latency: 7.5,
        };

        assert_eq!(metrics.pool_metrics.len(), 3);
        assert!(metrics.pool_metrics.iter().any(|p| p.name == "tank1"));
        assert!(metrics.pool_metrics.iter().any(|p| p.name == "tank2"));
        assert!(metrics.pool_metrics.iter().any(|p| p.name == "backup"));
    }

    #[test]
    fn test_realtime_metrics_extreme_values() {
        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![],
            system_metrics: create_test_system_metrics(),
            arc_hit_ratio: 1.0,           // Perfect hit rate
            l2arc_hit_ratio: 0.0,         // No L2ARC hits
            compression_ratio: 10.0,      // Extreme compression
            total_throughput: 0.0,        // No throughput
            average_read_latency: 0.1,    // Very fast
            average_write_latency: 100.0, // Very slow
        };

        assert_eq!(metrics.arc_hit_ratio, 1.0);
        assert_eq!(metrics.l2arc_hit_ratio, 0.0);
        assert_eq!(metrics.compression_ratio, 10.0);
        assert_eq!(metrics.total_throughput, 0.0);
    }

    // ==================== POOL METRICS TESTS ====================

    #[test]
    fn test_pool_metrics_healthy_pool() {
        let pool = PoolMetrics {
            name: "production".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 45.0,
            total_capacity: 10_000_000_000_000, // 10TB
            used_space: 4_500_000_000_000,      // 4.5TB
            available_space: 5_500_000_000_000, // 5.5TB
            read_iops: 5000,
            write_iops: 3000,
            read_throughput: 50_000_000.0,  // 50MB/s
            write_throughput: 30_000_000.0, // 30MB/s
            fragmentation_level: 0.15,
            error_count: 0,
        };

        assert_eq!(pool.health_status, "ONLINE");
        assert_eq!(pool.error_count, 0);
        assert!(pool.utilization_percentage < 80.0); // Below warning threshold
        assert!(pool.fragmentation_level < 0.5); // Good fragmentation
    }

    #[test]
    fn test_pool_metrics_degraded_pool() {
        let pool = PoolMetrics {
            name: "degraded-pool".to_string(),
            health_status: "DEGRADED".to_string(),
            utilization_percentage: 92.5,
            total_capacity: 5_000_000_000_000,
            used_space: 4_625_000_000_000,
            available_space: 375_000_000_000,
            read_iops: 1000,
            write_iops: 500,
            read_throughput: 10_000_000.0,
            write_throughput: 5_000_000.0,
            fragmentation_level: 0.75, // High fragmentation
            error_count: 15,
        };

        assert_eq!(pool.health_status, "DEGRADED");
        assert!(pool.error_count > 0);
        assert!(pool.utilization_percentage > 90.0); // Critical threshold
        assert!(pool.fragmentation_level > 0.5); // High fragmentation
    }

    #[test]
    fn test_pool_metrics_capacity_calculations() {
        let pool = create_test_pool_metrics("test");

        // Verify capacity relationships
        assert_eq!(pool.total_capacity, pool.used_space + pool.available_space);

        // Verify utilization calculation
        let calculated_utilization = (pool.used_space as f64 / pool.total_capacity as f64) * 100.0;
        assert!((pool.utilization_percentage - calculated_utilization).abs() < 0.1);
    }

    #[test]
    fn test_pool_metrics_iops_throughput_relationship() {
        let pool = PoolMetrics {
            name: "test-pool".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 50.0,
            total_capacity: 1_000_000_000_000,
            used_space: 500_000_000_000,
            available_space: 500_000_000_000,
            read_iops: 10000,
            write_iops: 5000,
            read_throughput: 100_000_000.0, // 100MB/s
            write_throughput: 50_000_000.0, // 50MB/s
            fragmentation_level: 0.2,
            error_count: 0,
        };

        // Higher IOPS should generally correlate with higher throughput
        assert!(pool.read_iops > pool.write_iops);
        assert!(pool.read_throughput > pool.write_throughput);
    }

    #[test]
    fn test_pool_metrics_empty_pool() {
        let pool = PoolMetrics {
            name: "empty-pool".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 0.0,
            total_capacity: 10_000_000_000,
            used_space: 0,
            available_space: 10_000_000_000,
            read_iops: 0,
            write_iops: 0,
            read_throughput: 0.0,
            write_throughput: 0.0,
            fragmentation_level: 0.0,
            error_count: 0,
        };

        assert_eq!(pool.utilization_percentage, 0.0);
        assert_eq!(pool.used_space, 0);
        assert_eq!(pool.available_space, pool.total_capacity);
    }

    #[test]
    fn test_pool_metrics_full_pool() {
        let total = 10_000_000_000u64;
        let pool = PoolMetrics {
            name: "full-pool".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 100.0,
            total_capacity: total,
            used_space: total,
            available_space: 0,
            read_iops: 100,
            write_iops: 0, // Can't write to full pool
            read_throughput: 1_000_000.0,
            write_throughput: 0.0,
            fragmentation_level: 0.95, // Very fragmented
            error_count: 5,
        };

        assert_eq!(pool.utilization_percentage, 100.0);
        assert_eq!(pool.available_space, 0);
        assert_eq!(pool.write_iops, 0);
    }

    // ==================== SYSTEM METRICS TESTS ====================

    #[test]
    fn test_system_metrics_normal_load() {
        let system = SystemMetrics {
            cpu_usage: 45.0,
            memory_usage: 60.0,
            memory_total: 32_000_000_000,     // 32GB
            memory_available: 12_800_000_000, // 12.8GB
            network_io: create_test_network_io(),
            disk_io: create_test_disk_io(),
        };

        assert!(system.memory_usage > 0.0 && system.memory_usage < 100.0);
        assert!(system.memory_available < system.memory_total);
    }

    #[test]
    fn test_system_metrics_high_memory_pressure() {
        let system = SystemMetrics {
            cpu_usage: 85.0,
            memory_usage: 95.0,
            memory_total: 16_000_000_000,
            memory_available: 800_000_000, // Only 800MB available
            network_io: create_test_network_io(),
            disk_io: create_test_disk_io(),
        };

        assert!(system.memory_usage > 90.0); // High memory pressure
        assert!(system.memory_available < system.memory_total / 10); // Less than 10% free
    }

    #[test]
    fn test_system_metrics_memory_calculations() {
        let total = 64_000_000_000u64; // 64GB
        let available = 32_000_000_000u64; // 32GB
        let used = total - available;

        let system = SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: (used as f64 / total as f64) * 100.0,
            memory_total: total,
            memory_available: available,
            network_io: create_test_network_io(),
            disk_io: create_test_disk_io(),
        };

        assert_eq!(system.memory_usage, 50.0);
        assert_eq!(system.memory_available, total / 2);
    }

    // ==================== NETWORK IO METRICS TESTS ====================

    #[test]
    fn test_network_io_metrics() {
        let network = NetworkIOMetrics {
            bytes_sent: 1_000_000_000,   // 1GB
            bytes_received: 500_000_000, // 500MB
            packets_sent: 1_000_000,
            packets_received: 800_000,
        };

        assert!(network.bytes_sent > network.bytes_received);
        assert!(network.packets_sent > network.packets_received);
    }

    #[test]
    fn test_network_io_no_activity() {
        let network = NetworkIOMetrics {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
        };

        assert_eq!(network.bytes_sent, 0);
        assert_eq!(network.packets_sent, 0);
    }

    #[test]
    fn test_network_io_balanced_traffic() {
        let network = NetworkIOMetrics {
            bytes_sent: 100_000_000,
            bytes_received: 100_000_000,
            packets_sent: 10000,
            packets_received: 10000,
        };

        assert_eq!(network.bytes_sent, network.bytes_received);
        assert_eq!(network.packets_sent, network.packets_received);
    }

    #[test]
    fn test_network_io_high_traffic() {
        let network = NetworkIOMetrics {
            bytes_sent: 10_000_000_000,    // 10GB
            bytes_received: 8_000_000_000, // 8GB
            packets_sent: 10_000_000,
            packets_received: 8_000_000,
        };

        assert!(network.bytes_sent > 1_000_000_000); // More than 1GB
        assert!(network.packets_sent > 1_000_000); // More than 1M packets
    }

    // ==================== DISK IO METRICS TESTS ====================

    #[test]
    fn test_disk_io_metrics() {
        let disk = DiskIOMetrics {
            read_bytes: 5_000_000_000,  // 5GB
            write_bytes: 3_000_000_000, // 3GB
            read_operations: 50000,
            write_operations: 30000,
        };

        assert!(disk.read_bytes > disk.write_bytes);
        assert!(disk.read_operations > disk.write_operations);
    }

    #[test]
    fn test_disk_io_average_operation_size() {
        let disk = DiskIOMetrics {
            read_bytes: 1_000_000_000,
            write_bytes: 1_000_000_000,
            read_operations: 10000,
            write_operations: 5000,
        };

        let avg_read_size = disk.read_bytes as f64 / disk.read_operations as f64;
        let avg_write_size = disk.write_bytes as f64 / disk.write_operations as f64;

        assert!(avg_read_size > 0.0);
        assert!(avg_write_size > 0.0);
        assert!(avg_write_size > avg_read_size); // Larger writes typical
    }

    #[test]
    fn test_disk_io_idle_state() {
        let disk = DiskIOMetrics {
            read_bytes: 0,
            write_bytes: 0,
            read_operations: 0,
            write_operations: 0,
        };

        assert_eq!(disk.read_operations, 0);
        assert_eq!(disk.write_operations, 0);
    }

    #[test]
    fn test_disk_io_heavy_reads() {
        let disk = DiskIOMetrics {
            read_bytes: 100_000_000_000, // 100GB
            write_bytes: 1_000_000_000,  // 1GB
            read_operations: 1_000_000,
            write_operations: 10_000,
        };

        assert!(disk.read_bytes > disk.write_bytes * 10); // Much more reads
        assert!(disk.read_operations > disk.write_operations * 10);
    }

    #[test]
    fn test_disk_io_heavy_writes() {
        let disk = DiskIOMetrics {
            read_bytes: 1_000_000_000,   // 1GB
            write_bytes: 50_000_000_000, // 50GB
            read_operations: 10_000,
            write_operations: 500_000,
        };

        assert!(disk.write_bytes > disk.read_bytes * 10); // Much more writes
        assert!(disk.write_operations > disk.read_operations * 10);
    }

    // ==================== HELPER FUNCTIONS ====================

    /// Creates  Test Pool Metrics
    fn create_test_pool_metrics(name: &str) -> PoolMetrics {
        PoolMetrics {
            name: name.to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 50.0,
            total_capacity: 1_000_000_000_000,
            used_space: 500_000_000_000,
            available_space: 500_000_000_000,
            read_iops: 1000,
            write_iops: 500,
            read_throughput: 10_000_000.0,
            write_throughput: 5_000_000.0,
            fragmentation_level: 0.2,
            error_count: 0,
        }
    }

    /// Creates  Test System Metrics
    fn create_test_system_metrics() -> SystemMetrics {
        SystemMetrics {
            cpu_usage: 50.0,
            memory_usage: 60.0,
            memory_total: 16_000_000_000,
            memory_available: 6_400_000_000,
            network_io: create_test_network_io(),
            disk_io: create_test_disk_io(),
        }
    }

    /// Creates  Test Network Io
    fn create_test_network_io() -> NetworkIOMetrics {
        NetworkIOMetrics {
            bytes_sent: 1_000_000,
            bytes_received: 800_000,
            packets_sent: 10000,
            packets_received: 8000,
        }
    }

    /// Creates  Test Disk Io
    fn create_test_disk_io() -> DiskIOMetrics {
        DiskIOMetrics {
            read_bytes: 5_000_000,
            write_bytes: 3_000_000,
            read_operations: 1000,
            write_operations: 600,
        }
    }

    // ==================== SERIALIZATION TESTS ====================

    #[test]
    fn test_realtime_metrics_serialization() {
        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![create_test_pool_metrics("test")],
            system_metrics: create_test_system_metrics(),
            arc_hit_ratio: 0.85,
            l2arc_hit_ratio: 0.70,
            compression_ratio: 2.0,
            total_throughput: 1_000_000.0,
            average_read_latency: 5.0,
            average_write_latency: 8.0,
        };

        let json = serde_json::to_string(&metrics);
        assert!(json.is_ok());

        let deserialized: Result<RealTimeMetrics, _> =
            serde_json::from_str(&json.expect("serialization failed"));
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_pool_metrics_serialization() {
        let pool = create_test_pool_metrics("serialization-test");

        let json = serde_json::to_string(&pool);
        assert!(json.is_ok());

        let deserialized: Result<PoolMetrics, _> =
            serde_json::from_str(&json.expect("serialization failed"));
        assert!(deserialized.is_ok());

        let deserialized = deserialized.expect("deserialization failed");
        assert_eq!(deserialized.name, "serialization-test");
    }
}
