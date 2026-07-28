// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use tokio::sync::broadcast;
use tokio::time::Duration as TokioDuration;

use crate::handlers::dashboard_types::{DashboardEvent, DashboardTimeRange};

use super::history::MetricsSnapshot;
use super::{
    DiskIOMetrics, MetricsCollectorState, NetworkIOMetrics, PoolMetrics, RealTimeMetrics,
    RealTimeMetricsCollector, SystemMetrics,
};

#[test]
fn metrics_collector_state_default_interval() {
    let s = MetricsCollectorState::default();
    assert_eq!(s.collection_interval, Duration::from_secs(5));
}

#[test]
fn realtime_metrics_collector_historicals_empty_before_collection() {
    let c = RealTimeMetricsCollector::new();
    let range = DashboardTimeRange::last_hours(1);
    assert!(c.get_historical_data("pool-a", &range).is_ok());
    assert!(
        c.get_historical_data("pool-a", &range)
            .expect("historical")
            .is_empty()
    );
    assert!(c.get_io_historical_data(&range).is_ok());
    assert!(c.get_io_historical_data(&range).expect("io").is_empty());
    assert!(c.get_comprehensive_historical_data().is_ok());
    assert!(
        c.get_comprehensive_historical_data()
            .expect("comprehensive")
            .is_empty()
    );
    assert!(c.get_capacity_historical_data(&range).is_ok());
    assert!(
        c.get_capacity_historical_data(&range)
            .expect("capacity")
            .is_empty()
    );
}

#[test]
fn realtime_metrics_collector_historicals_filter_ring_buffer() {
    let c = RealTimeMetricsCollector::new();
    let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1_000);
    let end = start + Duration::from_secs(300);
    let in_range = start + Duration::from_secs(60);
    let out_of_range = end + Duration::from_secs(60);

    let sample = |timestamp: SystemTime, used_space: u64| MetricsSnapshot {
        metrics: RealTimeMetrics {
            timestamp,
            pool_metrics: vec![PoolMetrics {
                name: "pool-a".into(),
                health_status: "ONLINE".into(),
                utilization_percentage: 50.0,
                total_capacity: 1_000,
                used_space,
                available_space: 1_000 - used_space,
                read_iops: 10,
                write_iops: 5,
                read_throughput: 1.0,
                write_throughput: 2.0,
                fragmentation_level: 0.1,
                error_count: 0,
            }],
            system_metrics: SystemMetrics {
                cpu_usage: 1.0,
                memory_usage: 2.0,
                memory_total: 100,
                memory_available: 50,
                network_io: NetworkIOMetrics {
                    bytes_sent: 1,
                    bytes_received: 2,
                    packets_sent: 3,
                    packets_received: 4,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 10,
                    write_bytes: 20,
                    read_operations: 30,
                    write_operations: 40,
                },
            },
            arc_hit_ratio: 0.9,
            l2arc_hit_ratio: 0.1,
            compression_ratio: 1.2,
            total_throughput: 3.0,
            average_read_latency: 1.0,
            average_write_latency: 2.0,
        },
        arc_size: 100,
        l2arc_size: 200,
    };

    c.push_test_snapshot(sample(out_of_range, 50));
    c.push_test_snapshot(sample(in_range, 100));
    c.push_test_snapshot(sample(in_range + Duration::from_secs(30), 150));

    let range = DashboardTimeRange::new(start, end, Duration::from_secs(5));
    let pool_history = c
        .get_historical_data("pool-a", &range)
        .expect("pool history");
    assert_eq!(pool_history.len(), 2);
    assert_eq!(pool_history[0].used_space, 100);
    assert_eq!(pool_history[1].used_space, 150);

    let io_history = c.get_io_historical_data(&range).expect("io history");
    assert_eq!(io_history.len(), 2);
    assert_eq!(io_history[0].read_iops, 10);

    let capacity_history = c
        .get_capacity_historical_data(&range)
        .expect("capacity history");
    assert_eq!(capacity_history.len(), 2);
    assert_eq!(capacity_history[0].used_space, 100);

    let comprehensive = c
        .get_comprehensive_historical_data()
        .expect("comprehensive history");
    assert_eq!(comprehensive.len(), 3);
}

#[test]
fn realtime_metrics_collector_default() {
    let _ = RealTimeMetricsCollector::default();
}

#[test]
fn metrics_types_serialize_roundtrip() {
    let m = RealTimeMetrics {
        timestamp: SystemTime::UNIX_EPOCH,
        pool_metrics: vec![],
        system_metrics: SystemMetrics {
            cpu_usage: 1.0,
            memory_usage: 50.0,
            memory_total: 8,
            memory_available: 4,
            network_io: NetworkIOMetrics {
                bytes_sent: 1,
                bytes_received: 2,
                packets_sent: 3,
                packets_received: 4,
            },
            disk_io: DiskIOMetrics {
                read_bytes: 10,
                write_bytes: 20,
                read_operations: 30,
                write_operations: 40,
            },
        },
        arc_hit_ratio: 0.9,
        l2arc_hit_ratio: 0.5,
        compression_ratio: 1.2,
        total_throughput: 100.0,
        average_read_latency: 1.0,
        average_write_latency: 2.0,
    };
    let s = serde_json::to_string(&m).unwrap();
    let _: RealTimeMetrics = serde_json::from_str(&s).unwrap();
}

#[test]
fn dashboard_time_range_last_hours_maps_to_variant() {
    let r = DashboardTimeRange::last_hours(24);
    let s = format!("{r:?}");
    assert!(!s.is_empty());
}

#[tokio::test]
async fn get_current_metrics_runs_on_linux_or_fallback() {
    let c = RealTimeMetricsCollector::new();
    let _ = c.get_current_metrics().await;
}

#[test]
fn get_system_resources_returns_snapshot() {
    let c = RealTimeMetricsCollector::new();
    let snap = c.get_system_resources().expect("snapshot");
    assert!(snap.cpu_cores >= 1);
}

#[tokio::test]
async fn start_collection_populates_history() {
    let c = RealTimeMetricsCollector::with_config(10, Duration::from_millis(50));
    let (tx, _rx) = broadcast::channel::<DashboardEvent>(4);
    c.start_collection(Arc::new(tx));

    tokio::time::sleep(TokioDuration::from_millis(200)).await;

    let range = DashboardTimeRange::last_hours(1);
    let history = c
        .get_comprehensive_historical_data()
        .expect("comprehensive history");
    assert!(
        !history.is_empty(),
        "background collection should store snapshots"
    );
    assert!(c.get_io_historical_data(&range).expect("io").len() >= history.len().saturating_sub(1));
}

#[test]
fn r6_metrics_collector_state_interval_30s() {
    let mut s = MetricsCollectorState::default();
    s.collection_interval = Duration::from_secs(30);
    assert_eq!(s.collection_interval, Duration::from_secs(30));
}

#[test]
fn r6_metrics_collector_state_interval_1s() {
    let mut s = MetricsCollectorState::default();
    s.collection_interval = Duration::from_secs(1);
    assert_eq!(s.collection_interval, Duration::from_secs(1));
}

#[test]
fn r6_metrics_collector_clone_preserves_interval() {
    let mut s = MetricsCollectorState::default();
    s.collection_interval = Duration::from_millis(750);
    let s2 = s.clone();
    assert_eq!(s.collection_interval, s2.collection_interval);
}

#[tokio::test]
async fn get_all_pool_metrics_from_collector() {
    let c = RealTimeMetricsCollector::new();
    assert!(c.get_all_pool_metrics().await.is_ok());
}

#[tokio::test]
async fn get_cache_metrics_from_collector() {
    let c = RealTimeMetricsCollector::new();
    assert!(c.get_cache_metrics().await.is_ok());
}
