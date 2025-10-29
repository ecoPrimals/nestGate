//! **METRICS INTEGRATION TESTS**
//!
//! Tests for the metrics collection and monitoring system

use crate::common::*;
use nestgate_core::metrics::*;
use std::time::Duration;

/// Test basic metrics collection
#[test]
async fn test_basic_metrics_collection() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Test counter metrics
    collector.increment_counter("requests_total");
    collector.increment_counter("requests_total");
    collector.increment_counter("errors_total");
    
    assert_eq!(collector.get_counter("requests_total"), Some(2));
    assert_eq!(collector.get_counter("errors_total"), Some(1));
    assert_eq!(collector.get_counter("nonexistent"), None);
    Ok(())
}

/// Test gauge metrics
#[test]
async fn test_gauge_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Test setting gauge values
    collector.set_gauge("cpu_usage", 75.5);
    collector.set_gauge("memory_usage", 60.2);
    collector.set_gauge("disk_usage", 45.8);
    
    assert_eq!(collector.get_gauge("cpu_usage"), Some(75.5));
    assert_eq!(collector.get_gauge("memory_usage"), Some(60.2));
    assert_eq!(collector.get_gauge("disk_usage"), Some(45.8));
    
    // Test updating gauge values
    collector.set_gauge("cpu_usage", 80.0);
    assert_eq!(collector.get_gauge("cpu_usage"), Some(80.0));
    Ok(())
}

/// Test histogram metrics
#[test]
async fn test_histogram_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Test recording histogram values
    collector.record_histogram("response_time", 0.1);
    collector.record_histogram("response_time", 0.2);
    collector.record_histogram("response_time", 0.15);
    collector.record_histogram("response_time", 0.3);
    
    let histogram = collector.get_histogram("response_time");
    assert!(histogram.is_some());
    
    if let Some(hist) = histogram {
        assert_eq!(hist.count(), 4);
        assert_eq!(hist.sum(), 0.75);
        assert_eq!(hist.average(), 0.1875);
        assert_eq!(hist.min(), 0.1);
        assert_eq!(hist.max(), 0.3);
    Ok(())
}
}

/// Test metrics with labels
#[test]
async fn test_metrics_with_labels() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Test labeled metrics
    let labels = vec![("method", "GET"), ("status", "200")];
    collector.increment_counter_with_labels("http_requests_total", &labels);
    collector.increment_counter_with_labels("http_requests_total", &labels);
    
    let labels_404 = vec![("method", "GET"), ("status", "404")];
    collector.increment_counter_with_labels("http_requests_total", &labels_404);
    
    assert_eq!(collector.get_counter_with_labels("http_requests_total", &labels), Some(2));
    assert_eq!(collector.get_counter_with_labels("http_requests_total", &labels_404), Some(1));
    Ok(())
}

/// Test metrics timing utilities
#[test]
async fn test_metrics_timing() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Test timing a block of code
    let timer = collector.start_timer("operation_duration");
    
    // Simulate some work
    std::thread::sleep(Duration::from_millis(10));
    
    let duration = timer.stop();
    assert!(duration.as_millis() >= 10);
    
    // Check that the metric was recorded
    let histogram = collector.get_histogram("operation_duration");
    assert!(histogram.is_some());
    
    if let Some(hist) = histogram {
        assert_eq!(hist.count(), 1);
        assert!(hist.sum() >= 0.01); // At least 10ms in seconds
    Ok(())
}
}

/// Test metrics export formats
#[test]
async fn test_metrics_export() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Add various metrics
    collector.increment_counter("test_counter");
    collector.set_gauge("test_gauge", 42.0);
    collector.record_histogram("test_histogram", 1.5);
    
    // Test Prometheus format export
    let prometheus_output = collector.export_prometheus();
    assert!(prometheus_output.contains("test_counter"));
    assert!(prometheus_output.contains("test_gauge"));
    assert!(prometheus_output.contains("test_histogram"));
    
    // Test JSON format export
    let json_output = collector.export_json();
    assert!(json_output.contains("test_counter"));
    assert!(json_output.contains("test_gauge"));
    assert!(json_output.contains("test_histogram"));
    
    // Verify JSON is valid
    let parsed: serde_json::Value = serde_json::from_str(&json_output)?;
    assert!(parsed.is_object());
    Ok(())
}

/// Test metrics aggregation
#[test]
async fn test_metrics_aggregation() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector1 = MetricsCollector::new();
    let mut collector2 = MetricsCollector::new();
    
    // Add metrics to both collectors
    collector1.increment_counter("shared_counter");
    collector1.increment_counter("shared_counter");
    collector1.set_gauge("shared_gauge", 10.0);
    
    collector2.increment_counter("shared_counter");
    collector2.set_gauge("shared_gauge", 20.0);
    collector2.increment_counter("unique_counter");
    
    // Test merging collectors
    let merged = collector1.merge(&collector2);
    
    // Counters should be summed
    assert_eq!(merged.get_counter("shared_counter"), Some(3));
    assert_eq!(merged.get_counter("unique_counter"), Some(1));
    
    // Gauges should use the latest value (from collector2)
    assert_eq!(merged.get_gauge("shared_gauge"), Some(20.0));
    Ok(())
}

/// Test metrics thresholds and alerts
#[test]
async fn test_metrics_thresholds() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Set up threshold monitoring
    collector.add_threshold("cpu_usage", 80.0, AlertLevel::Warning);
    collector.add_threshold("cpu_usage", 95.0, AlertLevel::Critical);
    collector.add_threshold("error_rate", 0.05, AlertLevel::Warning);
    
    // Test values below thresholds
    collector.set_gauge("cpu_usage", 70.0);
    let alerts = collector.check_thresholds();
    assert!(alerts.is_empty());
    
    // Test values above warning threshold
    collector.set_gauge("cpu_usage", 85.0);
    let alerts = collector.check_thresholds();
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].level, AlertLevel::Warning);
    assert_eq!(alerts[0].metric, "cpu_usage");
    
    // Test values above critical threshold
    collector.set_gauge("cpu_usage", 97.0);
    let alerts = collector.check_thresholds();
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0].level, AlertLevel::Critical);
    Ok(())
}

/// Test metrics persistence and recovery
#[tokio::test]
async fn test_metrics_persistence() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    let mut collector = MetricsCollector::new();
    
    // Add some metrics
    collector.increment_counter("persistent_counter");
    collector.increment_counter("persistent_counter");
    collector.set_gauge("persistent_gauge", 123.45);
    
    // Test saving metrics to storage
    let storage = MockStorage::new();
    let save_result = collector.save_to_storage(&storage).await;
    assert!(save_result.is_ok());
    
    // Test loading metrics from storage
    let mut new_collector = MetricsCollector::new();
    let load_result = new_collector.load_from_storage(&storage).await;
    assert!(load_result.is_ok());
    
    // Verify metrics were restored
    assert_eq!(new_collector.get_counter("persistent_counter"), Some(2));
    assert_eq!(new_collector.get_gauge("persistent_gauge"), Some(123.45));
    Ok(())
}

/// Test concurrent metrics collection
#[tokio::test]
async fn test_concurrent_metrics() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    let collector = std::sync::Arc::new(std::sync::Mutex::new(MetricsCollector::new()));
    let mut handles = Vec::new();
    
    // Spawn multiple tasks incrementing counters
    for i in 0..10 {
        let collector_clone = collector.clone();
        let handle = tokio::spawn(async move {
            for _ in 0..100 {
                let mut c = collector_clone.lock()?;
                c.increment_counter(&format!("thread_{}_counter", i));
                c.set_gauge(&format!("thread_{}_gauge", i), i as f64 * 10.0);
    Ok(())
            }
        });
        handles.push(handle);
    Ok(())
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await?;
    Ok(())
    }
    
    // Verify all metrics were recorded correctly
    let final_collector = collector.lock()?;
    for i in 0..10 {
        assert_eq!(final_collector.get_counter(&format!("thread_{}_counter", i)), Some(100));
        assert_eq!(final_collector.get_gauge(&format!("thread_{}_gauge", i)), Some(i as f64 * 10.0));
    Ok(())
}
}

/// Test metrics cleanup and rotation
#[test]
fn test_metrics_cleanup() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Add many metrics
    for i in 0..1000 {
        collector.increment_counter(&format!("counter_{}", i));
        collector.set_gauge(&format!("gauge_{}", i), i as f64);
    Ok(())
    }
    
    // Verify metrics count
    assert_eq!(collector.counter_count(), 1000);
    assert_eq!(collector.gauge_count(), 1000);
    
    // Test cleanup of old metrics
    let cutoff_time = std::time::SystemTime::now() - Duration::from_secs(3600); // 1 hour ago
    let cleaned_count = collector.cleanup_old_metrics(cutoff_time);
    
    // Since these are fresh metrics, none should be cleaned
    assert_eq!(cleaned_count, 0);
    assert_eq!(collector.counter_count(), 1000);
    
    // Test clearing all metrics
    collector.clear_all_metrics();
    assert_eq!(collector.counter_count(), 0);
    assert_eq!(collector.gauge_count(), 0);
    Ok(())
}

/// Comprehensive metrics integration test
#[test]
fn test_comprehensive_metrics_workflow() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = MetricsCollector::new();
    
    // Simulate a web server metrics scenario
    let endpoints = vec!["api/users", "api/orders", "api/products"];
    let methods = vec!["GET", "POST", "PUT", "DELETE"];
    let status_codes = vec!["200", "400", "404", "500"];
    
    // Generate realistic metrics
    for endpoint in &endpoints {
        for method in &methods {
            for status in &status_codes {
                let labels = vec![
                    ("endpoint", *endpoint),
                    ("method", *method),
                    ("status", *status),
                ];
                
                // Simulate different request patterns
                let count = match *status {
                    "200" => 100,
                    "400" => 10,
                    "404" => 5,
                    "500" => 2,
                    _ => 1,
                };
                
                for _ in 0..count {
                    collector.increment_counter_with_labels("http_requests_total", &labels);
                    
                    // Record response times
                    let response_time = match *status {
                        "200" => 0.1,
                        "400" => 0.05,
                        "404" => 0.02,
                        "500" => 2.0, // Errors are slow
                        _ => 0.1,
                    };
                    collector.record_histogram("http_response_duration", response_time);
    Ok(())
                }
    Ok(())
            }
    Ok(())
        }
    Ok(())
    }
    
    // Test that we have realistic metrics
    let success_labels = vec![("endpoint", "api/users"), ("method", "GET"), ("status", "200")];
    assert_eq!(collector.get_counter_with_labels("http_requests_total", &success_labels), Some(100));
    
    let error_labels = vec![("endpoint", "api/users"), ("method", "GET"), ("status", "500")];
    assert_eq!(collector.get_counter_with_labels("http_requests_total", &error_labels), Some(2));
    
    // Verify histogram data
    let response_histogram = collector.get_histogram("http_response_duration")?;
    assert!(response_histogram.count() > 0);
    assert!(response_histogram.average() > 0.0);
    
    // Test export functionality
    let prometheus_export = collector.export_prometheus();
    assert!(prometheus_export.contains("http_requests_total"));
    assert!(prometheus_export.contains("http_response_duration"));
    
    // Test summary statistics
    let summary = collector.get_summary();
    assert!(summary.total_counters > 0);
    assert!(summary.total_gauges >= 0);
    assert!(summary.total_histograms > 0);
} 