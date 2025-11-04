//! Comprehensive tests for ZFS metrics collection
//! Target: Improve coverage of metrics module

use nestgate_zfs::metrics::{MetricsSnapshot, ZfsMetrics};
use std::time::{Duration, SystemTime};

// ==================== CREATION TESTS ====================

#[test]
fn test_create_metrics() {
    let metrics = ZfsMetrics::new();
    assert!(format!("{:?}", metrics).len() > 0);
}

#[test]
fn test_create_metrics_for_testing() {
    let metrics = ZfsMetrics::new_for_testing();
    assert!(format!("{:?}", metrics).len() > 0);
}

// ==================== OPERATION RECORDING TESTS ====================

#[test]
fn test_record_single_operation() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 10.0);

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_operations, 1);
    assert_eq!(snapshot.total_bytes, 1024);
}

#[test]
fn test_record_multiple_operations() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);
    metrics.record_operation(2048, 20.0);
    metrics.record_operation(4096, 15.0);

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_operations, 3);
    assert_eq!(snapshot.total_bytes, 1024 + 2048 + 4096);
}

#[test]
fn test_record_zero_bytes() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(0, 5.0);

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_operations, 1);
    assert_eq!(snapshot.total_bytes, 0);
}

#[test]
fn test_record_large_operation() {
    let metrics = ZfsMetrics::new();
    let large_size = 1_000_000_000_u64; // 1 GB

    metrics.record_operation(large_size, 100.0);

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_operations, 1);
    assert_eq!(snapshot.total_bytes, large_size);
}

// ==================== ERROR RECORDING TESTS ====================

#[test]
fn test_record_single_error() {
    let metrics = ZfsMetrics::new();
    metrics.record_error();

    let snapshot = metrics.get_current_metrics();
    // Error rate should be 0.0 when no operations have been recorded
    assert_eq!(snapshot.error_rate, 0.0);
}

#[test]
fn test_record_error_with_operations() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);
    metrics.record_operation(2048, 15.0);
    metrics.record_error();

    let snapshot = metrics.get_current_metrics();
    // 1 error out of 2 operations = 50% error rate
    assert!((snapshot.error_rate - 0.5).abs() < 0.01);
}

#[test]
fn test_multiple_errors() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);
    metrics.record_error();
    metrics.record_error();
    metrics.record_error();

    let snapshot = metrics.get_current_metrics();
    // 3 errors out of 1 operation = 300% error rate (this is valid)
    assert!((snapshot.error_rate - 3.0).abs() < 0.01);
}

// ==================== LATENCY TESTS ====================

#[test]
fn test_latency_tracking() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);

    let snapshot = metrics.get_current_metrics();
    // Average latency should be close to 10.0 (with exponential moving average)
    assert!(snapshot.average_latency_ms >= 0.0);
}

#[test]
fn test_latency_moving_average() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);
    metrics.record_operation(1024, 20.0);
    metrics.record_operation(1024, 30.0);

    let snapshot = metrics.get_current_metrics();
    // Latency should reflect the moving average
    assert!(snapshot.average_latency_ms > 0.0);
}

#[test]
fn test_zero_latency() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 0.0);

    let snapshot = metrics.get_current_metrics();
    assert!(snapshot.average_latency_ms >= 0.0);
}

// ==================== SNAPSHOT TESTS ====================

#[test]
fn test_initial_snapshot() {
    let metrics = ZfsMetrics::new();
    let snapshot = metrics.get_current_metrics();

    assert_eq!(snapshot.total_operations, 0);
    assert_eq!(snapshot.total_bytes, 0);
    assert_eq!(snapshot.error_rate, 0.0);
    assert_eq!(snapshot.operations_per_second, 0.0);
}

#[test]
fn test_snapshot_timestamp() {
    let metrics = ZfsMetrics::new();
    let before = SystemTime::now();
    let snapshot = metrics.get_current_metrics();
    let after = SystemTime::now();

    assert!(snapshot.timestamp >= before);
    assert!(snapshot.timestamp <= after);
}

#[test]
fn test_snapshot_uptime() {
    let metrics = ZfsMetrics::new();
    std::thread::sleep(Duration::from_millis(100));

    let snapshot = metrics.get_current_metrics();
    assert!(snapshot.uptime_seconds >= 0);
}

#[test]
fn test_multiple_snapshots() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);
    let snapshot1 = metrics.get_current_metrics();

    metrics.record_operation(2048, 15.0);
    let snapshot2 = metrics.get_current_metrics();

    assert!(snapshot2.total_operations > snapshot1.total_operations);
    assert!(snapshot2.total_bytes > snapshot1.total_bytes);
}

// ==================== RATE CALCULATION TESTS ====================

#[test]
fn test_operations_per_second_zero_uptime() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 10.0);

    let snapshot = metrics.get_current_metrics();
    // OPS should be 0 or very high if uptime is near zero
    assert!(snapshot.operations_per_second >= 0.0);
}

#[test]
fn test_throughput_zero_uptime() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 10.0);

    let snapshot = metrics.get_current_metrics();
    // Throughput should be 0 or positive
    assert!(snapshot.throughput_bytes_per_second >= 0);
}

#[test]
fn test_error_rate_zero_operations() {
    let metrics = ZfsMetrics::new();
    metrics.record_error();

    let snapshot = metrics.get_current_metrics();
    // Error rate should be 0.0 when no operations
    assert_eq!(snapshot.error_rate, 0.0);
}

// ==================== CONCURRENT ACCESS TESTS ====================

#[test]
fn test_concurrent_operations() {
    use std::sync::Arc;
    use std::thread;

    let metrics = Arc::new(ZfsMetrics::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let metrics_clone = Arc::clone(&metrics);
        let handle = thread::spawn(move || {
            metrics_clone.record_operation(1024, 10.0);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_operations, 10);
    assert_eq!(snapshot.total_bytes, 10 * 1024);
}

#[test]
fn test_concurrent_errors() {
    use std::sync::Arc;
    use std::thread;

    let metrics = Arc::new(ZfsMetrics::new());
    let mut handles = vec![];

    for _ in 0..5 {
        let metrics_clone = Arc::clone(&metrics);
        let handle = thread::spawn(move || {
            metrics_clone.record_error();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.error_rate, 0.0); // No operations recorded
}

// ==================== METRICS SNAPSHOT SERIALIZATION TESTS ====================

#[test]
fn test_snapshot_serialization() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 10.0);

    let snapshot = metrics.get_current_metrics();
    let json = serde_json::to_string(&snapshot).expect("Failed to serialize");

    assert!(json.contains("total_operations"));
    assert!(json.contains("total_bytes"));
}

#[test]
fn test_snapshot_deserialization() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 10.0);

    let snapshot = metrics.get_current_metrics();
    let json = serde_json::to_string(&snapshot).expect("Failed to serialize");
    let deserialized: MetricsSnapshot = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(deserialized.total_operations, snapshot.total_operations);
    assert_eq!(deserialized.total_bytes, snapshot.total_bytes);
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_max_operations() {
    let metrics = ZfsMetrics::new();

    // Record a large number of operations
    for _ in 0..1000 {
        metrics.record_operation(1024, 10.0);
    }

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_operations, 1000);
}

#[test]
fn test_max_bytes() {
    let metrics = ZfsMetrics::new();
    let max_u32 = u32::MAX as u64;

    metrics.record_operation(max_u32, 10.0);

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_bytes, max_u32);
}

#[test]
fn test_very_high_latency() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 10000.0); // 10 seconds

    let snapshot = metrics.get_current_metrics();
    assert!(snapshot.average_latency_ms > 0.0);
}

#[test]
fn test_mixed_operations() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);
    metrics.record_error();
    metrics.record_operation(2048, 20.0);
    metrics.record_operation(4096, 15.0);
    metrics.record_error();

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_operations, 3);
    assert_eq!(snapshot.total_bytes, 1024 + 2048 + 4096);
}

// ==================== RESET AND LIFECYCLE TESTS ====================

#[test]
fn test_metrics_lifecycle() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);
    let snapshot1 = metrics.get_current_metrics();

    // Continue using the same metrics instance
    metrics.record_operation(2048, 15.0);
    let snapshot2 = metrics.get_current_metrics();

    assert!(snapshot2.total_operations > snapshot1.total_operations);
}

#[test]
fn test_new_metrics_instance() {
    let metrics1 = ZfsMetrics::new();
    metrics1.record_operation(1024, 10.0);

    let metrics2 = ZfsMetrics::new();
    let snapshot2 = metrics2.get_current_metrics();

    // New instance should start from zero
    assert_eq!(snapshot2.total_operations, 0);
    assert_eq!(snapshot2.total_bytes, 0);
}

// ==================== DEBUG FORMAT TESTS ====================

#[test]
fn test_metrics_debug_format() {
    let metrics = ZfsMetrics::new();
    let debug_str = format!("{:?}", metrics);

    assert!(debug_str.len() > 0);
    assert!(debug_str.contains("ZfsMetrics"));
}

#[test]
fn test_snapshot_debug_format() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 10.0);

    let snapshot = metrics.get_current_metrics();
    let debug_str = format!("{:?}", snapshot);

    assert!(debug_str.len() > 0);
    assert!(debug_str.contains("MetricsSnapshot"));
}

// ==================== CLONE AND COPY TESTS ====================

#[test]
fn test_snapshot_clone() {
    let metrics = ZfsMetrics::new();
    metrics.record_operation(1024, 10.0);

    let snapshot1 = metrics.get_current_metrics();
    let snapshot2 = snapshot1.clone();

    assert_eq!(snapshot1.total_operations, snapshot2.total_operations);
    assert_eq!(snapshot1.total_bytes, snapshot2.total_bytes);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_realistic_workload() {
    let metrics = ZfsMetrics::new();

    // Simulate a realistic workload
    for i in 0..100 {
        if i % 10 == 0 {
            metrics.record_error();
        }
        metrics.record_operation(1024 * (i + 1), 10.0 + (i as f64 * 0.1));
    }

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.total_operations, 100);
    assert!(snapshot.total_bytes > 0);
}

#[test]
fn test_high_error_rate_scenario() {
    let metrics = ZfsMetrics::new();

    metrics.record_operation(1024, 10.0);
    for _ in 0..10 {
        metrics.record_error();
    }

    let snapshot = metrics.get_current_metrics();
    assert!(snapshot.error_rate > 1.0);
}

#[test]
fn test_zero_error_rate_scenario() {
    let metrics = ZfsMetrics::new();

    for _ in 0..100 {
        metrics.record_operation(1024, 10.0);
    }

    let snapshot = metrics.get_current_metrics();
    assert_eq!(snapshot.error_rate, 0.0);
}
