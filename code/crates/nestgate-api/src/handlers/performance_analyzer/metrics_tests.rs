// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **SYSTEM METRICS TESTS**
//!
//! Comprehensive tests for system metrics collection.

use super::metrics::*;

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(std::num::NonZero::get)
        .unwrap_or(1)
}

#[tokio::test]
async fn test_system_metrics_collector_creation() {
    let collector = SystemMetricsCollector::new(60);
    assert_eq!(collector.interval_seconds, 60);
}

#[tokio::test]
async fn test_collect_metrics_success() {
    let collector = SystemMetricsCollector::new(30);
    let result = collector.collect_metrics().await;

    assert!(result.is_ok(), "Metrics collection should succeed");
    let metrics = result.expect("Test setup failed");

    assert!(metrics.cpu_usage_percent >= 0.0);
    // Multi-core systems can report >100% (one core = 100%)
    assert!(
        metrics.cpu_usage_percent <= 100.0 * num_cpus() as f64,
        "CPU usage {:.1}% exceeds per-core maximum",
        metrics.cpu_usage_percent,
    );
    // On Linux, memory_usage_bytes reads /proc/meminfo; on other platforms it's 0.
    #[cfg(target_os = "linux")]
    assert!(
        metrics.memory_usage_bytes > 0,
        "Memory usage should be positive on Linux"
    );
}

#[tokio::test]
async fn test_system_metrics_structure() {
    let collector = SystemMetricsCollector::new(60);
    let metrics = collector
        .collect_metrics()
        .await
        .expect("Test setup failed");

    assert!(metrics.cpu_usage_percent >= 0.0);
    assert!(metrics.cpu_usage_percent <= 100.0 * num_cpus() as f64);
}

#[test]
fn test_disk_io_metrics_structure() {
    let metrics = DiskIOMetrics {
        read_bytes_per_sec: 1024 * 1024,
        write_bytes_per_sec: 512 * 1024,
        read_ops_per_sec: 100,
        write_ops_per_sec: 50,
    };

    assert_eq!(metrics.read_bytes_per_sec, 1024 * 1024);
    assert_eq!(metrics.write_bytes_per_sec, 512 * 1024);
    assert_eq!(metrics.read_ops_per_sec, 100);
    assert_eq!(metrics.write_ops_per_sec, 50);
}

#[test]
fn test_network_metrics_structure() {
    let metrics = NetworkMetrics {
        rx_bytes_per_sec: 1024 * 1024,
        tx_bytes_per_sec: 512 * 1024,
        rx_packets_per_sec: 1000,
        tx_packets_per_sec: 800,
    };

    assert_eq!(metrics.rx_bytes_per_sec, 1024 * 1024);
    assert_eq!(metrics.tx_bytes_per_sec, 512 * 1024);
    assert_eq!(metrics.rx_packets_per_sec, 1000);
    assert_eq!(metrics.tx_packets_per_sec, 800);
}

#[tokio::test]
async fn test_multiple_collections_consistency() {
    let collector = SystemMetricsCollector::new(10);

    let metrics1 = collector
        .collect_metrics()
        .await
        .expect("Test setup failed");
    let metrics2 = collector
        .collect_metrics()
        .await
        .expect("Test setup failed");

    // Both collections should succeed
    assert!(metrics1.cpu_usage_percent >= 0.0);
    assert!(metrics2.cpu_usage_percent >= 0.0);
    assert!(metrics1.memory_usage_bytes > 0);
    assert!(metrics2.memory_usage_bytes > 0);
}

#[test]
fn test_metrics_serialization() {
    let metrics = SystemMetrics {
        cpu_usage_percent: 45.5,
        memory_usage_bytes: 1024 * 1024 * 1024,
        disk_io_metrics: DiskIOMetrics {
            read_bytes_per_sec: 1024 * 1024,
            write_bytes_per_sec: 512 * 1024,
            read_ops_per_sec: 100,
            write_ops_per_sec: 50,
        },
        network_metrics: NetworkMetrics {
            rx_bytes_per_sec: 1024 * 1024,
            tx_bytes_per_sec: 512 * 1024,
            rx_packets_per_sec: 1000,
            tx_packets_per_sec: 800,
        },
        timestamp: std::time::SystemTime::now(),
    };

    let json = serde_json::to_string(&metrics);
    assert!(json.is_ok(), "Metrics should serialize to JSON");

    let json_str = json.expect("Test setup failed");
    assert!(json_str.contains("cpu_usage_percent"));
    assert!(json_str.contains("memory_usage_bytes"));
    assert!(json_str.contains("disk_io_metrics"));
    assert!(json_str.contains("network_metrics"));
}

#[test]
fn test_metrics_deserialization() {
    let json = r#"{
        "cpu_usage_percent": 45.5,
        "memory_usage_bytes": 1073741824,
        "disk_io_metrics": {
            "read_bytes_per_sec": 1048576,
            "write_bytes_per_sec": 524288,
            "read_ops_per_sec": 100,
            "write_ops_per_sec": 50
        },
        "network_metrics": {
            "rx_bytes_per_sec": 1048576,
            "tx_bytes_per_sec": 524288,
            "rx_packets_per_sec": 1000,
            "tx_packets_per_sec": 800
        },
        "timestamp": {
            "secs_since_epoch": 1697000000,
            "nanos_since_epoch": 0
        }
    }"#;

    let metrics: Result<SystemMetrics, _> = serde_json::from_str(json);
    assert!(metrics.is_ok(), "Should deserialize from JSON");

    let metrics = metrics.expect("Test setup failed");
    assert_eq!(metrics.cpu_usage_percent, 45.5);
    assert_eq!(metrics.memory_usage_bytes, 1_073_741_824);
}

#[test]
fn test_metrics_error_display() {
    let error = MetricsError::SystemRead("test error".to_string());
    let display = format!("{error}");
    assert!(display.contains("test error"));

    let error = MetricsError::Parse("parse error".to_string());
    let display = format!("{error}");
    assert!(display.contains("parse error"));
}

#[tokio::test]
async fn test_collector_intervals() {
    let collector1 = SystemMetricsCollector::new(1);
    let collector60 = SystemMetricsCollector::new(60);
    let collector300 = SystemMetricsCollector::new(300);

    assert_eq!(collector1.interval_seconds, 1);
    assert_eq!(collector60.interval_seconds, 60);
    assert_eq!(collector300.interval_seconds, 300);
}

#[test]
fn test_disk_io_metrics_clone() {
    let original = DiskIOMetrics {
        read_bytes_per_sec: 1000,
        write_bytes_per_sec: 500,
        read_ops_per_sec: 10,
        write_ops_per_sec: 5,
    };

    let cloned = original.clone();
    assert_eq!(original.read_bytes_per_sec, cloned.read_bytes_per_sec);
    assert_eq!(original.write_bytes_per_sec, cloned.write_bytes_per_sec);
}

#[test]
fn test_network_metrics_clone() {
    let original = NetworkMetrics {
        rx_bytes_per_sec: 2000,
        tx_bytes_per_sec: 1000,
        rx_packets_per_sec: 20,
        tx_packets_per_sec: 10,
    };

    let cloned = original.clone();
    assert_eq!(original.rx_bytes_per_sec, cloned.rx_bytes_per_sec);
    assert_eq!(original.tx_bytes_per_sec, cloned.tx_bytes_per_sec);
}

#[tokio::test]
async fn test_metrics_timestamp_validity() {
    let collector = SystemMetricsCollector::new(60);
    let metrics = collector
        .collect_metrics()
        .await
        .expect("Test setup failed");

    let now = std::time::SystemTime::now();
    let duration = now.duration_since(metrics.timestamp);

    // Timestamp should be very recent (within 1 second)
    assert!(duration.is_ok());
    let duration = duration.expect("Test setup failed");
    assert!(duration.as_secs() < 1, "Timestamp should be recent");
}
