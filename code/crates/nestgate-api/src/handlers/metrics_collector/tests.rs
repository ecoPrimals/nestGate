// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::broadcast;
use tokio::time::Duration;

use crate::handlers::dashboard_types::{DashboardEvent, DashboardTimeRange};

use super::{
    DiskIOMetrics, MetricsCollectorState, NetworkIOMetrics, RealTimeMetrics,
    RealTimeMetricsCollector, SystemMetrics,
};

#[test]
fn metrics_collector_state_default_interval() {
    let s = MetricsCollectorState::default();
    assert_eq!(s.collection_interval, Duration::from_secs(5));
}

#[test]
fn realtime_metrics_collector_new_and_helpers() {
    let c = RealTimeMetricsCollector::new();
    let range = DashboardTimeRange::last_hours(1);
    assert!(c.get_historical_data("pool-a", &range).unwrap().is_empty());
    assert!(c.get_all_pool_metrics().unwrap().is_empty());
    assert!(c.get_io_historical_data(&range).unwrap().is_empty());
    assert!(c.get_cache_metrics().unwrap().is_empty());
    assert!(c.get_comprehensive_historical_data().unwrap().is_empty());
    assert!(c.get_capacity_historical_data(&range).unwrap().is_empty());
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
            _cpu_usage: 1.0,
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

#[test]
fn start_collection_does_not_panic() {
    let c = RealTimeMetricsCollector::new();
    let (tx, _rx) = broadcast::channel::<DashboardEvent>(4);
    c.start_collection(Arc::new(tx));
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
