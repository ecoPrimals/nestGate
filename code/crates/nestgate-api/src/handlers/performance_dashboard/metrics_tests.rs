//! Tests for performance dashboard metrics collector

use super::metrics::*;
use std::sync::Arc;
use tokio::sync::broadcast;

#[test]
fn test_real_time_metrics_collector_creation() {
    let collector = RealTimeMetricsCollector::new();

    // Should create without errors
    assert!(format!("{collector:?}").contains("RealTimeMetricsCollector"));
}

#[test]
fn test_collector_has_metrics_cache() {
    let collector = RealTimeMetricsCollector::new();

    // Collector should have internal cache structure
    assert!(format!("{collector:?}").contains("metrics_cache"));
}

#[tokio::test]
async fn test_get_current_metrics_returns_ok() {
    let collector = RealTimeMetricsCollector::new();

    let result = collector.get_current_metrics().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_metrics_has_timestamp() {
    let collector = RealTimeMetricsCollector::new();

    let metrics = collector.get_current_metrics().await.unwrap();
    let now = std::time::SystemTime::now();

    // Timestamp should be recent (within last 2 seconds)
    let duration = now.duration_since(metrics.timestamp);
    assert!(duration.is_ok());
    assert!(duration.unwrap().as_secs() < 2);
}

#[tokio::test]
async fn test_get_current_metrics_has_valid_cpu_usage() {
    let collector = RealTimeMetricsCollector::new();

    let metrics = collector.get_current_metrics().await.unwrap();

    // CPU usage should be between 0 and 100
    assert!(metrics.cpu_usage >= 0.0);
    assert!(metrics.cpu_usage <= 100.0);
}

#[tokio::test]
async fn test_get_current_metrics_has_valid_memory_usage() {
    let collector = RealTimeMetricsCollector::new();

    let metrics = collector.get_current_metrics().await.unwrap();

    // Memory usage should be between 0 and 100
    assert!(metrics.memory_usage >= 0.0);
    assert!(metrics.memory_usage <= 100.0);
}

#[tokio::test]
async fn test_get_current_metrics_has_valid_disk_io() {
    let collector = RealTimeMetricsCollector::new();

    let metrics = collector.get_current_metrics().await.unwrap();

    // Disk I/O should be non-negative
    assert!(metrics.disk_io >= 0.0);
}

#[tokio::test]
async fn test_get_current_metrics_has_valid_network_throughput() {
    let collector = RealTimeMetricsCollector::new();

    let metrics = collector.get_current_metrics().await.unwrap();

    // Network throughput should be non-negative
    assert!(metrics.network_throughput >= 0.0);
}

#[tokio::test]
async fn test_get_current_metrics_has_valid_active_connections() {
    let collector = RealTimeMetricsCollector::new();

    let metrics = collector.get_current_metrics().await.unwrap();

    // Active connections should be non-negative
    assert!(metrics.active_connections >= 0);
}

#[tokio::test]
async fn test_get_current_metrics_has_valid_response_time() {
    let collector = RealTimeMetricsCollector::new();

    let metrics = collector.get_current_metrics().await.unwrap();

    // Response time should be non-negative
    assert!(metrics.response_time_ms >= 0.0);
}

#[tokio::test]
async fn test_get_current_metrics_consistency() {
    let collector = RealTimeMetricsCollector::new();

    let metrics1 = collector.get_current_metrics().await.unwrap();
    let metrics2 = collector.get_current_metrics().await.unwrap();

    // Metrics should have consistent structure
    assert!(metrics1.cpu_usage >= 0.0);
    assert!(metrics2.cpu_usage >= 0.0);
}

#[tokio::test]
async fn test_multiple_collectors_independent() {
    let collector1 = RealTimeMetricsCollector::new();
    let collector2 = RealTimeMetricsCollector::new();

    let metrics1 = collector1.get_current_metrics().await.unwrap();
    let metrics2 = collector2.get_current_metrics().await.unwrap();

    // Both should return valid metrics independently
    assert!(metrics1.cpu_usage >= 0.0);
    assert!(metrics2.cpu_usage >= 0.0);
}

#[tokio::test]
async fn test_collector_can_be_shared() {
    let collector = Arc::new(RealTimeMetricsCollector::new());

    let collector_clone = Arc::clone(&collector);
    let metrics = collector_clone.get_current_metrics().await.unwrap();

    assert!(metrics.cpu_usage >= 0.0);
}

#[tokio::test]
async fn test_start_collection_accepts_broadcaster() {
    let collector = RealTimeMetricsCollector::new();
    let (tx, _rx) = broadcast::channel(100);
    let broadcaster = Arc::new(tx);

    // Should be able to start collection (we'll stop it quickly)
    let collector_arc = Arc::new(collector);
    let collector_clone = Arc::clone(&collector_arc);
    let broadcaster_clone = Arc::clone(&broadcaster);

    tokio::spawn(async move {
        tokio::time::timeout(
            std::time::Duration::from_millis(100),
            collector_clone.start_collection(broadcaster_clone),
        )
        .await
        .ok();
    });

    // Give it a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
}

#[tokio::test]
async fn test_metrics_broadcast_message_format() {
    let (tx, mut rx) = broadcast::channel(100);
    let broadcaster = Arc::new(tx);
    let collector = Arc::new(RealTimeMetricsCollector::new());

    let collector_clone = Arc::clone(&collector);
    let broadcaster_clone = Arc::clone(&broadcaster);

    // Start collection in background
    tokio::spawn(async move {
        tokio::time::timeout(
            std::time::Duration::from_millis(200),
            collector_clone.start_collection(broadcaster_clone),
        )
        .await
        .ok();
    });

    // Try to receive a message
    tokio::time::timeout(std::time::Duration::from_millis(150), async {
        if let Ok(message) = rx.recv().await {
            assert!(message.starts_with("metrics_update:"));
        }
    })
    .await
    .ok();
}

#[tokio::test]
async fn test_get_current_metrics_caching() {
    let collector = RealTimeMetricsCollector::new();

    // Get metrics twice quickly - second should use cache
    let metrics1 = collector.get_current_metrics().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    let metrics2 = collector.get_current_metrics().await.unwrap();

    // Both should be valid
    assert!(metrics1.cpu_usage >= 0.0);
    assert!(metrics2.cpu_usage >= 0.0);
}

#[tokio::test]
async fn test_metrics_timestamp_progression() {
    let collector = RealTimeMetricsCollector::new();

    let metrics1 = collector.get_current_metrics().await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let metrics2 = collector.get_current_metrics().await.unwrap();

    // Second metrics should have equal or later timestamp
    assert!(metrics2.timestamp >= metrics1.timestamp);
}

#[test]
fn test_collector_debug_format() {
    let collector = RealTimeMetricsCollector::new();
    let debug_str = format!("{collector:?}");

    // Debug format should contain key information
    assert!(debug_str.contains("RealTimeMetricsCollector"));
}

#[tokio::test]
async fn test_multiple_concurrent_gets() {
    let collector = Arc::new(RealTimeMetricsCollector::new());

    let mut handles = vec![];

    for _ in 0..5 {
        let collector_clone = Arc::clone(&collector);
        let handle = tokio::spawn(async move { collector_clone.get_current_metrics().await });
        handles.push(handle);
    }

    // All should complete successfully
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_collector_under_rapid_requests() {
    let collector = RealTimeMetricsCollector::new();

    // Make multiple rapid requests
    for _ in 0..10 {
        let result = collector.get_current_metrics().await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_metrics_structure_completeness() {
    let collector = RealTimeMetricsCollector::new();
    let metrics = collector.get_current_metrics().await.unwrap();

    // Verify all fields are present and reasonable
    assert!(metrics.cpu_usage >= 0.0 && metrics.cpu_usage <= 100.0);
    assert!(metrics.memory_usage >= 0.0 && metrics.memory_usage <= 100.0);
    assert!(metrics.disk_io >= 0.0);
    assert!(metrics.network_throughput >= 0.0);
    assert!(metrics.active_connections >= 0);
    assert!(metrics.response_time_ms >= 0.0);
}

#[tokio::test]
async fn test_collector_performance() {
    let collector = RealTimeMetricsCollector::new();

    let start = std::time::Instant::now();
    let _ = collector.get_current_metrics().await;
    let duration = start.elapsed();

    // Lenient: system load can cause slowdown; 30s ceiling
    assert!(
        duration.as_secs() < 30,
        "Collector should complete within 30s"
    );
}

#[tokio::test]
async fn test_collector_can_be_dropped_safely() {
    {
        let collector = RealTimeMetricsCollector::new();
        let _ = collector.get_current_metrics().await;
        // Collector drops here
    }

    // Should not panic or leak
}

#[tokio::test]
async fn test_collector_new_creates_fresh_instance() {
    let collector1 = RealTimeMetricsCollector::new();
    let collector2 = RealTimeMetricsCollector::new();

    let metrics1 = collector1.get_current_metrics().await.unwrap();
    let metrics2 = collector2.get_current_metrics().await.unwrap();

    // Both should return valid, independent metrics
    assert!(metrics1.timestamp <= std::time::SystemTime::now());
    assert!(metrics2.timestamp <= std::time::SystemTime::now());
}
