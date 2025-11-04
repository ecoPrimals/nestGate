//! **DATA COLLECTORS TESTS**
//!
//! Comprehensive tests for data collection components.

use super::collectors::*;
use super::metrics::*;

#[test]
fn test_data_collector_creation() {
    let collector = DataCollector::new(60);
    assert_eq!(collector.interval.as_secs(), 60);
}

#[test]
fn test_data_collector_custom_intervals() {
    let collector1 = DataCollector::new(30);
    let collector2 = DataCollector::new(120);
    let collector3 = DataCollector::new(300);

    assert_eq!(collector1.interval.as_secs(), 30);
    assert_eq!(collector2.interval.as_secs(), 120);
    assert_eq!(collector3.interval.as_secs(), 300);
}

#[tokio::test]
async fn test_collect_all_metrics() {
    let collector = DataCollector::new(60);
    let result = collector.collect_all_metrics().await;

    assert!(result.is_ok(), "Metrics collection should succeed");
    let metrics = result.expect("Test setup failed");

    assert!(metrics.cpu_usage_percent >= 0.0);
    assert!(metrics.memory_usage_bytes > 0);
}

#[tokio::test]
async fn test_get_latest_snapshot() {
    let collector = DataCollector::new(60);
    let result = collector.get_latest_snapshot().await;

    assert!(result.is_ok(), "Snapshot should be retrieved");
    let snapshot = result.expect("Test setup failed");

    assert!(snapshot.system_metrics.cpu_usage_percent >= 0.0);
    assert_eq!(snapshot.collector_id, "default");

    // Verify timestamp is recent
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(snapshot.collection_timestamp);
    assert!(duration.is_ok());
    assert!(duration.expect("Test setup failed").as_secs() < 2);
}

#[test]
fn test_metrics_snapshot_structure() {
    let snapshot = MetricsSnapshot {
        system_metrics: SystemMetrics {
            cpu_usage_percent: 50.0,
            memory_usage_bytes: 1024 * 1024 * 1024,
            disk_io_metrics: DiskIOMetrics {
                read_bytes_per_sec: 1024,
                write_bytes_per_sec: 512,
                read_ops_per_sec: 10,
                write_ops_per_sec: 5,
            },
            network_metrics: NetworkMetrics {
                rx_bytes_per_sec: 2048,
                tx_bytes_per_sec: 1024,
                rx_packets_per_sec: 20,
                tx_packets_per_sec: 10,
            },
            timestamp: std::time::SystemTime::now(),
        },
        collection_timestamp: std::time::SystemTime::now(),
        collector_id: "test-collector".to_string(),
    };

    assert_eq!(snapshot.system_metrics.cpu_usage_percent, 50.0);
    assert_eq!(snapshot.collector_id, "test-collector");
}

#[test]
fn test_metrics_snapshot_clone() {
    let original = MetricsSnapshot {
        system_metrics: SystemMetrics {
            cpu_usage_percent: 45.5,
            memory_usage_bytes: 1024 * 1024,
            disk_io_metrics: DiskIOMetrics {
                read_bytes_per_sec: 1000,
                write_bytes_per_sec: 500,
                read_ops_per_sec: 10,
                write_ops_per_sec: 5,
            },
            network_metrics: NetworkMetrics {
                rx_bytes_per_sec: 2000,
                tx_bytes_per_sec: 1000,
                rx_packets_per_sec: 20,
                tx_packets_per_sec: 10,
            },
            timestamp: std::time::SystemTime::now(),
        },
        collection_timestamp: std::time::SystemTime::now(),
        collector_id: "clone-test".to_string(),
    };

    let cloned = original.clone();
    assert_eq!(
        original.system_metrics.cpu_usage_percent,
        cloned.system_metrics.cpu_usage_percent
    );
    assert_eq!(original.collector_id, cloned.collector_id);
}

#[test]
fn test_batch_collector_creation() {
    let batch_collector = BatchCollector::new(5);
    assert_eq!(batch_collector.batch_size, 5);
    assert_eq!(batch_collector.collectors.len(), 5);
}

#[test]
fn test_batch_collector_different_sizes() {
    let small = BatchCollector::new(2);
    let medium = BatchCollector::new(10);
    let large = BatchCollector::new(50);

    assert_eq!(small.collectors.len(), 2);
    assert_eq!(medium.collectors.len(), 10);
    assert_eq!(large.collectors.len(), 50);
}

#[tokio::test]
async fn test_batch_collector_collect_batch() {
    let batch_collector = BatchCollector::new(3);
    let result = batch_collector.collect_batch().await;

    assert!(result.is_ok(), "Batch collection should succeed");
    let metrics_vec = result.expect("Test setup failed");

    assert_eq!(metrics_vec.len(), 3, "Should collect from all 3 collectors");

    for metrics in metrics_vec {
        assert!(metrics.cpu_usage_percent >= 0.0);
        assert!(metrics.memory_usage_bytes > 0);
    }
}

#[tokio::test]
async fn test_batch_collector_parallel_collection() {
    let batch_collector = BatchCollector::new(10);
    let result = batch_collector.collect_batch().await;

    assert!(result.is_ok(), "Parallel batch collection should succeed");
    let metrics_vec = result.expect("Test setup failed");

    assert_eq!(metrics_vec.len(), 10, "Should collect from all collectors");
}

#[tokio::test]
async fn test_multiple_snapshots_sequential() {
    let collector = DataCollector::new(1);

    let snapshot1 = collector.get_latest_snapshot().await.expect("Test setup failed");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let snapshot2 = collector.get_latest_snapshot().await.expect("Test setup failed");

    // Both should succeed and have different timestamps
    assert!(snapshot2.collection_timestamp >= snapshot1.collection_timestamp);
}

#[tokio::test]
async fn test_data_collector_system_collector_arc() {
    let collector = DataCollector::new(60);

    // Test that Arc sharing works
    let system_collector_clone = collector.system_collector.clone();
    let result = system_collector_clone.collect_metrics().await;

    assert!(result.is_ok(), "Cloned collector should work");
}

#[tokio::test]
async fn test_collector_with_zero_interval() {
    // Even with interval 0, collector should still work
    let collector = DataCollector::new(0);
    let result = collector.collect_all_metrics().await;

    assert!(result.is_ok(), "Should work even with 0 interval");
}

#[test]
fn test_batch_collector_empty() {
    let batch_collector = BatchCollector::new(0);
    assert_eq!(batch_collector.collectors.len(), 0);
    assert_eq!(batch_collector.batch_size, 0);
}

#[tokio::test]
async fn test_batch_collector_empty_collect() {
    let batch_collector = BatchCollector::new(0);
    let result = batch_collector.collect_batch().await;

    assert!(result.is_ok(), "Empty batch should succeed");
    let metrics_vec = result.expect("Test setup failed");
    assert_eq!(metrics_vec.len(), 0, "Empty batch should return no metrics");
}

#[tokio::test]
async fn test_collector_metrics_consistency() {
    let collector = DataCollector::new(30);

    let snapshot = collector.get_latest_snapshot().await.expect("Test setup failed");
    let direct_metrics = collector.collect_all_metrics().await.expect("Test setup failed");

    // Both should return valid metrics
    assert!(snapshot.system_metrics.cpu_usage_percent >= 0.0);
    assert!(direct_metrics.cpu_usage_percent >= 0.0);
}

#[test]
fn test_batch_collector_single_collector() {
    let batch_collector = BatchCollector::new(1);
    assert_eq!(batch_collector.collectors.len(), 1);
}

#[tokio::test]
async fn test_batch_collector_single_collect() {
    let batch_collector = BatchCollector::new(1);
    let result = batch_collector.collect_batch().await;

    assert!(result.is_ok());
    let metrics_vec = result.expect("Test setup failed");
    assert_eq!(metrics_vec.len(), 1);
}

#[tokio::test]
async fn test_large_batch_collection() {
    let batch_collector = BatchCollector::new(100);
    let result = batch_collector.collect_batch().await;

    assert!(result.is_ok(), "Large batch should succeed");
    let metrics_vec = result.expect("Test setup failed");
    assert_eq!(metrics_vec.len(), 100);
}
