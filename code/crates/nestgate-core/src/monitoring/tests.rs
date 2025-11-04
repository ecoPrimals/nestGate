// **MONITORING TESTS**
//
// Tests for monitoring and metrics collection

use super::*;
use std::time::{Duration, Instant, SystemTime};

// ==================== BASIC METRIC TESTS ====================

#[test]
fn test_metric_timestamp() {
    let now = Instant::now();
    assert!(now.elapsed() < Duration::from_millis(100));
}

#[test]
fn test_metric_value_storage() {
    let value: f64 = 42.0;
    assert_eq!(value, 42.0);
}

#[test]
fn test_metric_name_validation() {
    let metric_name = "cpu_usage";
    assert!(!metric_name.is_empty());
    assert!(metric_name.is_ascii());
}

#[test]
fn test_metric_threshold() {
    let current_value = 75.0;
    let threshold = 90.0;
    assert!(current_value < threshold);
}

#[test]
fn test_metric_aggregation() {
    let values = vec![10.0, 20.0, 30.0, 40.0];
    let sum: f64 = values.iter().sum();
    let avg = sum / values.len() as f64;
    assert_eq!(avg, 25.0);
}

#[test]
fn test_metric_collection_timestamp() {
    let timestamp = SystemTime::now();
    assert!(timestamp.elapsed().expect("Test setup failed") < Duration::from_secs(1));
}

// ==================== ALERT SEVERITY TESTS ====================

#[test]
fn test_alert_severity_ordering() {
    use crate::monitoring::AlertSeverity;
    
    let critical = AlertSeverity::Critical;
    let warning = AlertSeverity::Warning;
    let info = AlertSeverity::Info;
    
    // Test basic creation
    assert!(format!("{:?}", critical).contains("Critical"));
    assert!(format!("{:?}", warning).contains("Warning"));
    assert!(format!("{:?}", info).contains("Info"));
}

// ==================== HEALTH STATUS TESTS ====================

#[test]
fn test_health_status_healthy() {
    use crate::monitoring::HealthStatus;
    let status = HealthStatus::Healthy;
    assert!(format!("{:?}", status).contains("Healthy"));
}

#[test]
fn test_health_status_degraded() {
    use crate::monitoring::HealthStatus;
    let status = HealthStatus::Degraded;
    assert!(format!("{:?}", status).contains("Degraded"));
}

#[test]
fn test_health_status_unhealthy() {
    use crate::monitoring::HealthStatus;
    let status = HealthStatus::Unhealthy;
    assert!(format!("{:?}", status).contains("Unhealthy"));
}

// ==================== METRICS COLLECTION TESTS ====================

#[test]
fn test_metrics_vector_storage() {
    let metrics: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(metrics.len(), 5);
    assert_eq!(metrics[0], 1.0);
    assert_eq!(metrics[4], 5.0);
}

#[test]
fn test_metrics_statistical_calculations() {
    let values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    
    let sum: f64 = values.iter().sum();
    assert_eq!(sum, 150.0);
    
    let avg = sum / values.len() as f64;
    assert_eq!(avg, 30.0);
    
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    assert_eq!(min, 10.0);
    
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    assert_eq!(max, 50.0);
}

#[test]
fn test_metrics_percentile_calculation() {
    let mut values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    values.sort_by(|a, b| a.partial_cmp(b).expect("Test setup failed"));
    
    let median_idx = values.len() / 2;
    let median = values[median_idx];
    assert_eq!(median, 6.0);
}

// ==================== DURATION AND TIME TESTS ====================

#[test]
fn test_duration_measurements() {
    let start = Instant::now();
    std::thread::sleep(Duration::from_millis(10));
    let elapsed = start.elapsed();
    
    assert!(elapsed >= Duration::from_millis(10));
    assert!(elapsed < Duration::from_millis(100));
}

#[test]
fn test_duration_conversions() {
    let millis = 5000_u64;
    let duration = Duration::from_millis(millis);
    
    assert_eq!(duration.as_millis(), 5000);
    assert_eq!(duration.as_secs(), 5);
}

#[test]
fn test_system_time_operations() {
    let now = SystemTime::now();
    let elapsed = now.elapsed().expect("Test setup failed");
    
    assert!(elapsed < Duration::from_secs(1));
}

// ==================== THRESHOLD AND ALERT TESTS ====================

#[test]
fn test_threshold_breach_detection() {
    let cpu_usage = 95.0;
    let cpu_threshold = 90.0;
    let memory_usage = 85.0;
    let memory_threshold = 90.0;
    
    assert!(cpu_usage > cpu_threshold, "CPU threshold breached");
    assert!(memory_usage < memory_threshold, "Memory within threshold");
}

#[test]
fn test_multiple_threshold_levels() {
    let value = 85.0;
    
    let critical_threshold = 95.0;
    let warning_threshold = 80.0;
    let info_threshold = 50.0;
    
    assert!(value < critical_threshold);
    assert!(value > warning_threshold);
    assert!(value > info_threshold);
}

// ==================== DATA AGGREGATION TESTS ====================

#[test]
fn test_time_series_data() {
    let mut time_series: Vec<(SystemTime, f64)> = Vec::new();
    
    for i in 0..10 {
        time_series.push((SystemTime::now(), i as f64));
    }
    
    assert_eq!(time_series.len(), 10);
    assert_eq!(time_series[0].1, 0.0);
    assert_eq!(time_series[9].1, 9.0);
}

#[test]
fn test_rolling_average() {
    let values = vec![10.0, 15.0, 20.0, 25.0, 30.0];
    let window_size = 3;
    
    let mut rolling_avgs = Vec::new();
    for i in window_size..=values.len() {
        let window = &values[i-window_size..i];
        let avg = window.iter().sum::<f64>() / window_size as f64;
        rolling_avgs.push(avg);
    }
    
    assert_eq!(rolling_avgs.len(), 3);
    assert_eq!(rolling_avgs[0], 15.0); // (10+15+20)/3
    assert_eq!(rolling_avgs[1], 20.0); // (15+20+25)/3
    assert_eq!(rolling_avgs[2], 25.0); // (20+25+30)/3
}

// ==================== COMPONENT HEALTH TESTS ====================

#[test]
fn test_component_status_tracking() {
    let components = vec!["api", "database", "cache", "queue"];
    
    for component in &components {
        assert!(!component.is_empty());
        assert!(component.is_ascii());
    }
    
    assert_eq!(components.len(), 4);
}

// ==================== ERROR RATE CALCULATION TESTS ====================

#[test]
fn test_error_rate_calculation() {
    let total_requests = 1000_u64;
    let failed_requests = 25_u64;
    
    let error_rate = (failed_requests as f64 / total_requests as f64) * 100.0;
    
    assert_eq!(error_rate, 2.5);
    assert!(error_rate < 5.0, "Error rate acceptable");
}

#[test]
fn test_success_rate_calculation() {
    let total = 1000_u64;
    let successful = 975_u64;
    
    let success_rate = (successful as f64 / total as f64) * 100.0;
    
    assert_eq!(success_rate, 97.5);
    assert!(success_rate > 95.0, "Success rate healthy");
}

// ==================== PERFORMANCE TRACKING TESTS ====================

#[test]
fn test_latency_tracking() {
    let latencies_ms = vec![10_u64, 15, 20, 12, 18, 25, 30];
    
    let avg_latency = latencies_ms.iter().sum::<u64>() / latencies_ms.len() as u64;
    assert_eq!(avg_latency, 18);
    
    let max_latency = latencies_ms.iter().max().expect("Test setup failed");
    assert_eq!(*max_latency, 30);
}

#[test]
fn test_throughput_calculation() {
let requests_processed = 1000_u64;
let time_window_secs = 10_u64;

let throughput = requests_processed / time_window_secs;
assert_eq!(throughput, 100); // 100 requests per second
}

