// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct MetricsCollector;

impl MetricsCollector {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[test]
fn test_realtime_metrics_collector_new_and_default() {
    let collector = RealTimeMetricsCollector::new();
    let _ = RealTimeMetricsCollector::default();
    let (sender, _) = broadcast::channel(100);
    collector.start_collection(Arc::new(sender));
}

#[test]
fn test_metrics_collector_state_default() {
    let state = MetricsCollectorState::default();
    assert_eq!(state.collection_interval, Duration::from_secs(5));
}

#[test]
fn test_pool_metrics_serialization() {
    let metrics = PoolMetrics {
        name: "main-pool".to_string(),
        health_status: "ONLINE".to_string(),
        utilization_percentage: 45.5,
        total_capacity: 1_000_000_000_000,
        used_space: 455_000_000_000,
        available_space: 545_000_000_000,
        read_iops: 1000,
        write_iops: 500,
        read_throughput: 100.0,
        write_throughput: 50.0,
        fragmentation_level: 0.1,
        error_count: 0,
    };
    let json = serde_json::to_string(&metrics).unwrap();
    assert!(json.contains("main-pool"));
}

#[test]
fn test_network_io_metrics_serialization() {
    let metrics = NetworkIOMetrics {
        bytes_sent: 1024,
        bytes_received: 2048,
        packets_sent: 10,
        packets_received: 20,
    };
    let json = serde_json::to_string(&metrics).unwrap();
    assert!(json.contains("1024"));
}

#[test]
fn test_disk_io_metrics_serialization() {
    let metrics = DiskIOMetrics {
        read_bytes: 1000,
        write_bytes: 2000,
        read_operations: 50,
        write_operations: 25,
    };
    let json = serde_json::to_string(&metrics).unwrap();
    assert!(json.contains("1000"));
}

#[test]
fn test_system_snapshot_serialization() {
    let snapshot = SystemSnapshot {
        timestamp: SystemTime::now(),
        cpu_cores: 8,
        cpu_usage_percent: 50.0,
        memory_total_gb: 16,
        memory_used_gb: 8,
        disk_total_gb: 1000,
        disk_used_gb: 500,
        network_interfaces: vec!["eth0".to_string()],
    };
    let json = serde_json::to_string(&snapshot).unwrap();
    assert!(json.contains("eth0"));
}

#[test]
fn test_io_metrics_point_serialization() {
    let point = IOMetricsPoint {
        timestamp: SystemTime::now(),
        read_iops: 100,
        write_iops: 50,
        read_latency: 1.5,
        write_latency: 2.0,
    };
    let json = serde_json::to_string(&point).unwrap();
    assert!(json.contains("100"));
}

#[test]
fn test_cache_metrics_point_serialization() {
    let point = CacheMetricsPoint {
        timestamp: SystemTime::now(),
        arc_hit_ratio: 0.9,
        l2arc_hit_ratio: 0.7,
        arc_size: 1_000_000_000,
        l2arc_size: 500_000_000,
    };
    let json = serde_json::to_string(&point).unwrap();
    assert!(json.contains("0.9"));
}

#[test]
fn test_capacity_metrics_point_serialization() {
    let point = CapacityMetricsPoint {
        timestamp: SystemTime::now(),
        total_capacity: 10_000_000_000_000,
        used_space: 5_000_000_000_000,
        growth_rate: 100.0,
    };
    let json = serde_json::to_string(&point).unwrap();
    assert!(json.contains("100"));
}

#[tokio::test]
async fn test_get_current_metrics() {
    let collector = RealTimeMetricsCollector::new();
    let result = collector.get_current_metrics().await;
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert!(metrics.total_throughput >= 0.0);
}

#[test]
fn test_get_system_resources() {
    let collector = RealTimeMetricsCollector::new();
    let result = collector.get_system_resources();
    assert!(result.is_ok());
    let snapshot = result.unwrap();
    assert!(snapshot.cpu_cores > 0 || cfg!(not(target_os = "linux")));
}

#[test]
fn test_get_all_pool_metrics() {
    let collector = RealTimeMetricsCollector::new();
    let result = collector.get_all_pool_metrics();
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_get_historical_data() {
    let collector = RealTimeMetricsCollector::new();
    let time_range = DashboardTimeRange::last_hours(24);
    let result = collector.get_historical_data("main-pool", &time_range);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_get_io_historical_data() {
    let collector = RealTimeMetricsCollector::new();
    let time_range = DashboardTimeRange::last_hours(24);
    let result = collector.get_io_historical_data(&time_range);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_get_cache_metrics() {
    let collector = RealTimeMetricsCollector::new();
    let result = collector.get_cache_metrics();
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_get_comprehensive_historical_data() {
    let collector = RealTimeMetricsCollector::new();
    let result = collector.get_comprehensive_historical_data();
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn test_get_capacity_historical_data() {
    let collector = RealTimeMetricsCollector::new();
    let time_range = DashboardTimeRange::last_hours(24);
    let result = collector.get_capacity_historical_data(&time_range);
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}
