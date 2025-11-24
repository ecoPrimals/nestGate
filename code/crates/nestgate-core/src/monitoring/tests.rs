// **MONITORING TESTS**
//
// Comprehensive tests for monitoring, metrics, health checks, and alerts

use super::*;
use std::collections::HashMap;
use std::time::{Duration, Instant, SystemTime};

// ==================== HEALTH STATUS TESTS ====================

#[test]
fn test_health_status_healthy() {
    use crate::monitoring::health_checks::HealthStatus;
    let status = HealthStatus::Healthy;
    assert!(status.is_operational());
    assert_eq!(status.severity_level(), 0);
}

#[test]
fn test_health_status_degraded() {
    use crate::monitoring::health_checks::HealthStatus;
    let warnings = vec!["High CPU usage".to_string(), "Low memory".to_string()];
    let status = HealthStatus::Degraded { warnings: warnings.clone() };
    assert!(status.is_operational());
    assert_eq!(status.severity_level(), 1);
}

#[test]
fn test_health_status_unhealthy() {
    use crate::monitoring::health_checks::HealthStatus;
    let errors = vec!["Database connection failed".to_string()];
    let status = HealthStatus::Unhealthy { errors: errors.clone() };
    assert!(!status.is_operational());
    assert_eq!(status.severity_level(), 2);
}

#[test]
fn test_health_status_unknown() {
    use crate::monitoring::health_checks::HealthStatus;
    let status = HealthStatus::Unknown;
    assert!(!status.is_operational());
    assert_eq!(status.severity_level(), 3);
}

// ==================== COMPONENT HEALTH TESTS ====================

#[test]
fn test_component_health_creation() {
    use crate::monitoring::health_checks::{ComponentHealth, HealthStatus};
    
    let component = ComponentHealth {
        name: "api-server".to_string(),
        component_type: "api".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        check_duration: Duration::from_millis(50),
        details: HashMap::new(),
        history: vec![],
    };
    
    assert_eq!(component.name, "api-server");
    assert_eq!(component.component_type, "api");
    assert!(component.status.is_operational());
}

// ==================== HEALTH CHECK CONFIG TESTS ====================

#[test]
fn test_health_check_config_default() {
    use crate::monitoring::health_checks::HealthCheckConfig;
    
    let config = HealthCheckConfig::default();
    assert_eq!(config.check_interval, Duration::from_secs(30));
    assert_eq!(config.check_timeout, Duration::from_secs(10));
    assert_eq!(config.history_size, 10);
    assert!(!config.deep_checks_enabled);
    assert!(config.custom_endpoints.is_empty());
}

#[test]
fn test_health_check_config_custom() {
    use crate::monitoring::health_checks::HealthCheckConfig;
    
    let config = HealthCheckConfig {
        check_interval: Duration::from_secs(60),
        check_timeout: Duration::from_secs(5),
        history_size: 20,
        deep_checks_enabled: true,
        custom_endpoints: vec!["/health".to_string(), "/ready".to_string()],
    };
    
    assert_eq!(config.check_interval, Duration::from_secs(60));
    assert!(config.deep_checks_enabled);
    assert_eq!(config.custom_endpoints.len(), 2);
}

// ==================== SYSTEM METRICS TESTS ====================

#[test]
fn test_system_metrics_default() {
    use crate::monitoring::metrics::SystemMetrics;
    
    let metrics = SystemMetrics::default();
    assert_eq!(metrics.cpu_usage, 0.0);
    assert_eq!(metrics.memory_usage, 0);
    assert_eq!(metrics.active_connections, 0);
}

#[test]
fn test_system_metrics_creation() {
    use crate::monitoring::metrics::SystemMetrics;
    
    let metrics = SystemMetrics {
        cpu_usage: 45.5,
        memory_usage: 8_000_000_000, // 8 GB
        memory_available: 16_000_000_000, // 16 GB
        disk_usage: 500_000_000_000, // 500 GB
        disk_available: 1_000_000_000_000, // 1 TB
        network_rx_bytes: 1_000_000,
        network_tx_bytes: 500_000,
        active_connections: 150,
        uptime: Duration::from_secs(3600 * 24), // 1 day
        timestamp: SystemTime::now(),
    };
    
    assert_eq!(metrics.cpu_usage, 45.5);
    assert_eq!(metrics.active_connections, 150);
    assert!(metrics.memory_available > metrics.memory_usage);
}

// ==================== PROVIDER METRICS TESTS ====================

#[test]
fn test_provider_metrics_new() {
    use crate::monitoring::metrics::ProviderMetrics;
    
    let metrics = ProviderMetrics::new("test-provider".to_string(), "genome_data".to_string());
    assert_eq!(metrics.provider_name, "test-provider");
    assert_eq!(metrics.provider_type, "genome_data");
    assert_eq!(metrics.total_requests, 0);
    assert_eq!(metrics.successful_requests, 0);
    assert_eq!(metrics.failed_requests, 0);
}

#[test]
fn test_provider_metrics_record_success() {
    use crate::monitoring::metrics::ProviderMetrics;
    
    let mut metrics = ProviderMetrics::new("test-provider".to_string(), "model_data".to_string());
    
    metrics.record_success(Duration::from_millis(100));
    assert_eq!(metrics.total_requests, 1);
    assert_eq!(metrics.successful_requests, 1);
    assert_eq!(metrics.avg_response_time_ms, 100.0);
    
    metrics.record_success(Duration::from_millis(200));
    assert_eq!(metrics.total_requests, 2);
    assert_eq!(metrics.successful_requests, 2);
    assert_eq!(metrics.avg_response_time_ms, 150.0);
}

#[test]
fn test_provider_metrics_record_failure() {
    use crate::monitoring::metrics::ProviderMetrics;
    
    let mut metrics = ProviderMetrics::new("test-provider".to_string(), "compute".to_string());
    
    metrics.record_failure("TimeoutError".to_string());
    assert_eq!(metrics.total_requests, 1);
    assert_eq!(metrics.failed_requests, 1);
    assert_eq!(metrics.error_counts.get("TimeoutError"), Some(&1));
    
    metrics.record_failure("TimeoutError".to_string());
    assert_eq!(metrics.error_counts.get("TimeoutError"), Some(&2));
}

// ==================== METRICS CONFIG TESTS ====================

#[test]
fn test_metrics_config_default() {
    use crate::monitoring::metrics::MetricsConfig;
    
    let config = MetricsConfig::default();
    assert_eq!(config.collection_interval, Duration::from_secs(30));
    assert_eq!(config.retention_period, Duration::from_secs(3600));
    assert!(config.export_enabled);
    assert!(!config.export_endpoints.is_empty());
}

#[test]
fn test_metrics_config_custom() {
    use crate::monitoring::metrics::MetricsConfig;
    
    let mut labels = HashMap::new();
    labels.insert("environment".to_string(), "production".to_string());
    labels.insert("region".to_string(), "us-west-1".to_string());
    
    let config = MetricsConfig {
        collection_interval: Duration::from_secs(10),
        retention_period: Duration::from_secs(7200),
        export_enabled: true,
        export_endpoints: vec!["http://prometheus:9090".to_string()],
        labels,
    };
    
    assert_eq!(config.collection_interval, Duration::from_secs(10));
    assert_eq!(config.labels.len(), 2);
    assert_eq!(config.labels.get("environment"), Some(&"production".to_string()));
}

// ==================== HEALTH SUMMARY TESTS ====================

#[test]
fn test_health_summary_creation() {
    use crate::monitoring::health_checks::HealthSummary;
    
    let summary = HealthSummary {
        total_components: 10,
        healthy_components: 7,
        degraded_components: 2,
        unhealthy_components: 1,
        unknown_components: 0,
    };
    
    assert_eq!(summary.total_components, 10);
    assert_eq!(summary.healthy_components, 7);
    assert_eq!(summary.degraded_components, 2);
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_health_status_serialization() {
    use crate::monitoring::health_checks::HealthStatus;
    use serde_json;
    
    let status = HealthStatus::Healthy;
    let serialized = serde_json::to_string(&status);
    assert!(serialized.is_ok());
    
    let deserialized: Result<HealthStatus, _> = serde_json::from_str(&serialized.unwrap());
    assert!(deserialized.is_ok());
}

#[test]
fn test_system_metrics_serialization() {
    use crate::monitoring::metrics::SystemMetrics;
    use serde_json;
    
    let metrics = SystemMetrics::default();
    let serialized = serde_json::to_string(&metrics);
    assert!(serialized.is_ok());
}

#[test]
fn test_provider_metrics_serialization() {
    use crate::monitoring::metrics::ProviderMetrics;
    use serde_json;
    
    let metrics = ProviderMetrics::new("test".to_string(), "type".to_string());
    let serialized = serde_json::to_string(&metrics);
    assert!(serialized.is_ok());
}

// ==================== EDGE CASES ====================

#[test]
fn test_provider_metrics_multiple_error_types() {
    use crate::monitoring::metrics::ProviderMetrics;
    
    let mut metrics = ProviderMetrics::new("test".to_string(), "type".to_string());
    
    metrics.record_failure("TimeoutError".to_string());
    metrics.record_failure("NetworkError".to_string());
    metrics.record_failure("TimeoutError".to_string());
    metrics.record_failure("ParseError".to_string());
    
    assert_eq!(metrics.error_counts.len(), 3);
    assert_eq!(metrics.error_counts.get("TimeoutError"), Some(&2));
    assert_eq!(metrics.error_counts.get("NetworkError"), Some(&1));
    assert_eq!(metrics.error_counts.get("ParseError"), Some(&1));
}

#[test]
fn test_health_status_degraded_multiple_warnings() {
    use crate::monitoring::health_checks::HealthStatus;
    
    let warnings = vec![
        "Warning 1".to_string(),
        "Warning 2".to_string(),
        "Warning 3".to_string(),
    ];
    
    let status = HealthStatus::Degraded { warnings: warnings.clone() };
    
    match status {
        HealthStatus::Degraded { warnings: w } => {
            assert_eq!(w.len(), 3);
        }
        _ => panic!("Expected Degraded status"),
    }
}

#[test]
fn test_system_metrics_zero_values() {
    use crate::monitoring::metrics::SystemMetrics;
    
    let metrics = SystemMetrics {
        cpu_usage: 0.0,
        memory_usage: 0,
        memory_available: 0,
        disk_usage: 0,
        disk_available: 0,
        network_rx_bytes: 0,
        network_tx_bytes: 0,
        active_connections: 0,
        uptime: Duration::from_secs(0),
        timestamp: SystemTime::now(),
    };
    
    assert_eq!(metrics.cpu_usage, 0.0);
    assert_eq!(metrics.active_connections, 0);
}

#[test]
fn test_system_metrics_high_values() {
    use crate::monitoring::metrics::SystemMetrics;
    
    let metrics = SystemMetrics {
        cpu_usage: 99.9,
        memory_usage: u64::MAX / 2,
        memory_available: u64::MAX / 4,
        disk_usage: u64::MAX / 2,
        disk_available: u64::MAX / 4,
        network_rx_bytes: u64::MAX / 2,
        network_tx_bytes: u64::MAX / 2,
        active_connections: 10_000,
        uptime: Duration::from_secs(365 * 24 * 3600), // 1 year
        timestamp: SystemTime::now(),
    };
    
    assert_eq!(metrics.cpu_usage, 99.9);
    assert_eq!(metrics.active_connections, 10_000);
}

// ==================== CONCURRENT OPERATIONS ====================

#[tokio::test]
async fn test_concurrent_provider_metrics_updates() {
    use crate::monitoring::metrics::ProviderMetrics;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    let metrics = Arc::new(Mutex::new(ProviderMetrics::new("test".to_string(), "type".to_string())));
    
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let metrics_clone = Arc::clone(&metrics);
            tokio::spawn(async move {
                let mut m = metrics_clone.lock().await;
                m.record_success(Duration::from_millis(100));
            })
        })
        .collect();
    
    for handle in handles {
        handle.await.expect("Task should complete");
    }
    
    let final_metrics = metrics.lock().await;
    assert_eq!(final_metrics.total_requests, 10);
    assert_eq!(final_metrics.successful_requests, 10);
}

