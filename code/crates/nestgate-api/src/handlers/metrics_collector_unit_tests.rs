// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unit tests for metrics collector
//!
//! These tests cover metrics data structures and validation logic
//! without requiring actual system metrics collection.

#[cfg(test)]
mod tests {
    use super::super::metrics_collector::*;
    use std::time::SystemTime;

    // ==================== RealTimeMetrics Tests ====================

    #[test]
    fn test_real_time_metrics_creation() {
        let metrics = RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![],
            system_metrics: create_test_system_metrics(),
            arc_hit_ratio: 0.85,
            l2arc_hit_ratio: 0.75,
            compression_ratio: 2.5,
            total_throughput: 1_000_000.0,
            average_read_latency: 5.2,
            average_write_latency: 8.3,
        };

        assert!((metrics.arc_hit_ratio - 0.85).abs() < 0.01);
        assert!((metrics.compression_ratio - 2.5).abs() < 0.01);
    }

    #[test]
    fn test_real_time_metrics_clone() {
        let metrics1 = create_test_real_time_metrics();
        let metrics2 = metrics1.clone();

        assert_eq!(metrics1.arc_hit_ratio, metrics2.arc_hit_ratio);
        assert_eq!(metrics1.total_throughput, metrics2.total_throughput);
    }

    #[test]
    fn test_real_time_metrics_serialization() {
        let metrics = create_test_real_time_metrics();
        let json = serde_json::to_string(&metrics).expect("Failed to serialize");

        assert!(json.contains("arc_hit_ratio"));
        assert!(json.contains("compression_ratio"));
    }

    #[test]
    fn test_real_time_metrics_deserialization() {
        let metrics = create_test_real_time_metrics();
        let json = serde_json::to_string(&metrics).unwrap();

        let deserialized: RealTimeMetrics =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(metrics.arc_hit_ratio, deserialized.arc_hit_ratio);
    }

    // ==================== PoolMetrics Tests ====================

    #[test]
    fn test_pool_metrics_creation() {
        let pool = PoolMetrics {
            name: "testpool".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 75.5,
            total_capacity: 1_000_000_000,
            used_space: 755_000_000,
            available_space: 245_000_000,
            read_iops: 1500,
            write_iops: 800,
            read_throughput: 150_000_000.0,
            write_throughput: 80_000_000.0,
            fragmentation_level: 0.15,
            error_count: 0,
        };

        assert_eq!(pool.name, "testpool");
        assert_eq!(pool.health_status, "ONLINE");
        assert!((pool.utilization_percentage - 75.5).abs() < 0.01);
    }

    #[test]
    fn test_pool_metrics_capacity_validation() {
        let pool = PoolMetrics {
            name: "testpool".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 75.0,
            total_capacity: 1_000_000_000,
            used_space: 750_000_000,
            available_space: 250_000_000,
            read_iops: 1500,
            write_iops: 800,
            read_throughput: 150_000_000.0,
            write_throughput: 80_000_000.0,
            fragmentation_level: 0.15,
            error_count: 0,
        };

        assert_eq!(pool.used_space + pool.available_space, pool.total_capacity);
    }

    #[test]
    fn test_pool_metrics_clone() {
        let pool1 = create_test_pool_metrics();
        let pool2 = pool1.clone();

        assert_eq!(pool1.name, pool2.name);
        assert_eq!(pool1.total_capacity, pool2.total_capacity);
    }

    #[test]
    fn test_pool_metrics_serialization() {
        let pool = create_test_pool_metrics();
        let json = serde_json::to_string(&pool).expect("Failed to serialize");

        assert!(json.contains("testpool"));
        assert!(json.contains("health_status"));
    }

    // ==================== SystemMetrics Tests ====================

    #[test]
    fn test_system_metrics_creation() {
        let metrics = SystemMetrics {
            cpu_usage: 45.5,
            memory_usage: 65.0,
            memory_total: 16_000_000_000,
            memory_available: 5_600_000_000,
            network_io: create_test_network_io(),
            disk_io: create_test_disk_io(),
        };

        assert!((metrics.memory_usage - 65.0).abs() < 0.01);
        assert_eq!(metrics.memory_total, 16_000_000_000);
    }

    #[test]
    fn test_system_metrics_memory_calculation() {
        let metrics = create_test_system_metrics();
        let used_memory = metrics.memory_total - metrics.memory_available;
        let usage_percent = (used_memory as f64 / metrics.memory_total as f64) * 100.0;

        assert!((usage_percent - metrics.memory_usage).abs() < 1.0);
    }

    // ==================== NetworkIOMetrics Tests ====================

    #[test]
    fn test_network_io_metrics_creation() {
        let net_io = NetworkIOMetrics {
            bytes_sent: 1_000_000,
            bytes_received: 2_000_000,
            packets_sent: 5000,
            packets_received: 8000,
        };

        assert_eq!(net_io.bytes_sent, 1_000_000);
        assert_eq!(net_io.packets_received, 8000);
    }

    #[test]
    fn test_network_io_metrics_clone() {
        let net1 = create_test_network_io();
        let net2 = net1.clone();

        assert_eq!(net1.bytes_sent, net2.bytes_sent);
        assert_eq!(net1.packets_received, net2.packets_received);
    }

    #[test]
    fn test_network_io_metrics_serialization() {
        let net_io = create_test_network_io();
        let json = serde_json::to_string(&net_io).expect("Failed to serialize");

        assert!(json.contains("bytes_sent"));
        assert!(json.contains("packets_received"));
    }

    // ==================== DiskIOMetrics Tests ====================

    #[test]
    fn test_disk_io_metrics_creation() {
        let disk_io = DiskIOMetrics {
            read_bytes: 5_000_000,
            write_bytes: 3_000_000,
            read_operations: 1000,
            write_operations: 500,
        };

        assert_eq!(disk_io.read_bytes, 5_000_000);
        assert_eq!(disk_io.write_operations, 500);
    }

    #[test]
    fn test_disk_io_metrics_clone() {
        let disk1 = create_test_disk_io();
        let disk2 = disk1.clone();

        assert_eq!(disk1.read_bytes, disk2.read_bytes);
        assert_eq!(disk1.write_operations, disk2.write_operations);
    }

    #[test]
    fn test_disk_io_metrics_serialization() {
        let disk_io = create_test_disk_io();
        let json = serde_json::to_string(&disk_io).expect("Failed to serialize");

        assert!(json.contains("read_bytes"));
        assert!(json.contains("write_operations"));
    }

    // ==================== SystemSnapshot Tests ====================

    #[test]
    fn test_system_snapshot_creation() {
        let snapshot = SystemSnapshot {
            timestamp: SystemTime::now(),
            cpu_cores: 8,
            cpu_usage_percent: 45.5,
            memory_total_gb: 16,
            memory_used_gb: 10,
            disk_total_gb: 1000,
            disk_used_gb: 650,
            network_interfaces: vec!["eth0".to_string(), "eth1".to_string()],
        };

        assert_eq!(snapshot.cpu_cores, 8);
        assert_eq!(snapshot.memory_total_gb, 16);
        assert_eq!(snapshot.network_interfaces.len(), 2);
    }

    #[test]
    fn test_system_snapshot_clone() {
        let snap1 = create_test_system_snapshot();
        let snap2 = snap1.clone();

        assert_eq!(snap1.cpu_cores, snap2.cpu_cores);
        assert_eq!(snap1.disk_total_gb, snap2.disk_total_gb);
    }

    #[test]
    fn test_system_snapshot_serialization() {
        let snapshot = create_test_system_snapshot();
        let json = serde_json::to_string(&snapshot).expect("Failed to serialize");

        assert!(json.contains("cpu_cores"));
        assert!(json.contains("network_interfaces"));
    }

    #[test]
    fn test_system_snapshot_memory_utilization() {
        let snapshot = create_test_system_snapshot();
        let utilization =
            (f64::from(snapshot.memory_used_gb) / f64::from(snapshot.memory_total_gb)) * 100.0;

        assert!((0.0..=100.0).contains(&utilization));
    }

    #[test]
    fn test_system_snapshot_disk_utilization() {
        let snapshot = create_test_system_snapshot();
        let utilization = (snapshot.disk_used_gb as f64 / snapshot.disk_total_gb as f64) * 100.0;

        assert!((0.0..=100.0).contains(&utilization));
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_pool_metrics_zero_capacity() {
        let pool = PoolMetrics {
            name: "empty".to_string(),
            health_status: "OFFLINE".to_string(),
            utilization_percentage: 0.0,
            total_capacity: 0,
            used_space: 0,
            available_space: 0,
            read_iops: 0,
            write_iops: 0,
            read_throughput: 0.0,
            write_throughput: 0.0,
            fragmentation_level: 0.0,
            error_count: 0,
        };

        assert_eq!(pool.total_capacity, 0);
        assert_eq!(pool.utilization_percentage, 0.0);
    }

    #[test]
    fn test_pool_metrics_high_fragmentation() {
        let pool = PoolMetrics {
            name: "fragmented".to_string(),
            health_status: "DEGRADED".to_string(),
            utilization_percentage: 85.0,
            total_capacity: 1_000_000_000,
            used_space: 850_000_000,
            available_space: 150_000_000,
            read_iops: 500,
            write_iops: 200,
            read_throughput: 50_000_000.0,
            write_throughput: 20_000_000.0,
            fragmentation_level: 0.95,
            error_count: 15,
        };

        assert!((pool.fragmentation_level - 0.95).abs() < 0.01);
        assert_eq!(pool.error_count, 15);
    }

    #[test]
    fn test_network_io_zero_traffic() {
        let net_io = NetworkIOMetrics {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
        };

        assert_eq!(net_io.bytes_sent + net_io.bytes_received, 0);
        assert_eq!(net_io.packets_sent + net_io.packets_received, 0);
    }

    #[test]
    fn test_disk_io_zero_operations() {
        let disk_io = DiskIOMetrics {
            read_bytes: 0,
            write_bytes: 0,
            read_operations: 0,
            write_operations: 0,
        };

        assert_eq!(disk_io.read_operations + disk_io.write_operations, 0);
    }

    // ==================== Helper Functions ====================

    /// Creates  Test Real Time Metrics
    fn create_test_real_time_metrics() -> RealTimeMetrics {
        RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics: vec![create_test_pool_metrics()],
            system_metrics: create_test_system_metrics(),
            arc_hit_ratio: 0.85,
            l2arc_hit_ratio: 0.75,
            compression_ratio: 2.5,
            total_throughput: 1_000_000.0,
            average_read_latency: 5.2,
            average_write_latency: 8.3,
        }
    }

    /// Creates  Test Pool Metrics
    fn create_test_pool_metrics() -> PoolMetrics {
        PoolMetrics {
            name: "testpool".to_string(),
            health_status: "ONLINE".to_string(),
            utilization_percentage: 75.5,
            total_capacity: 1_000_000_000,
            used_space: 755_000_000,
            available_space: 245_000_000,
            read_iops: 1500,
            write_iops: 800,
            read_throughput: 150_000_000.0,
            write_throughput: 80_000_000.0,
            fragmentation_level: 0.15,
            error_count: 0,
        }
    }

    /// Creates  Test System Metrics
    fn create_test_system_metrics() -> SystemMetrics {
        SystemMetrics {
            cpu_usage: 45.5,
            memory_usage: 65.0,
            memory_total: 16_000_000_000,
            memory_available: 5_600_000_000,
            network_io: create_test_network_io(),
            disk_io: create_test_disk_io(),
        }
    }

    /// Creates  Test Network Io
    fn create_test_network_io() -> NetworkIOMetrics {
        NetworkIOMetrics {
            bytes_sent: 1_000_000,
            bytes_received: 2_000_000,
            packets_sent: 5000,
            packets_received: 8000,
        }
    }

    /// Creates  Test Disk Io
    fn create_test_disk_io() -> DiskIOMetrics {
        DiskIOMetrics {
            read_bytes: 5_000_000,
            write_bytes: 3_000_000,
            read_operations: 1000,
            write_operations: 500,
        }
    }

    /// Creates  Test System Snapshot
    fn create_test_system_snapshot() -> SystemSnapshot {
        SystemSnapshot {
            timestamp: SystemTime::now(),
            cpu_cores: 8,
            cpu_usage_percent: 45.5,
            memory_total_gb: 16,
            memory_used_gb: 10,
            disk_total_gb: 1000,
            disk_used_gb: 650,
            network_interfaces: vec!["eth0".to_string(), "eth1".to_string()],
        }
    }

    // ==================== Debug Tests ====================

    #[test]
    fn test_pool_metrics_debug_format() {
        let pool = create_test_pool_metrics();
        let debug_str = format!("{pool:?}");

        assert!(debug_str.contains("testpool"));
        assert!(debug_str.contains("ONLINE"));
    }

    #[test]
    fn test_system_metrics_debug_format() {
        let metrics = create_test_system_metrics();
        let debug_str = format!("{metrics:?}");

        assert!(debug_str.contains("memory_usage"));
    }

    #[test]
    fn test_network_io_debug_format() {
        let net_io = create_test_network_io();
        let debug_str = format!("{net_io:?}");

        assert!(debug_str.contains("bytes_sent"));
    }
}
