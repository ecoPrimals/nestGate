//! Unit tests for utils module
//!
//! These tests cover configuration, metrics, and utility functions.

#[cfg(test)]
mod tests {
    use super::super::utils::*;
    use std::time::Duration;

    // ==================== Config Tests ====================

    #[test]
    fn test_config_creation() {
        let config = Config {
            enabled: true,
            timeout: Duration::from_secs(30),
            max_connections: 100,
            buffer_size: 8192,
        };

        assert!(config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.buffer_size, 8192);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();

        assert!(config.enabled);
        assert!(config.max_connections > 0);
        assert!(config.buffer_size > 0);
    }

    #[test]
    fn test_config_clone() {
        let config1 = Config::default();
        let config2 = config1.clone();

        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(config1.timeout, config2.timeout);
        assert_eq!(config1.max_connections, config2.max_connections);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).expect("Failed to serialize");
        
        assert!(json.contains("enabled"));
        assert!(json.contains("max_connections"));
    }

    #[test]
    fn test_config_deserialization() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        
        let deserialized: Config = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        
        assert_eq!(config.enabled, deserialized.enabled);
        assert_eq!(config.max_connections, deserialized.max_connections);
    }

    #[test]
    fn test_config_debug() {
        let config = Config::default();
        let debug_str = format!("{:?}", config);
        
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("enabled"));
    }

    // ==================== HealthStatus Tests ====================

    #[test]
    fn test_health_status_variants() {
        let healthy = HealthStatus::Healthy;
        let degraded = HealthStatus::Degraded;
        let unhealthy = HealthStatus::Unhealthy;

        assert_eq!(healthy, HealthStatus::Healthy);
        assert_ne!(healthy, degraded);
        assert_ne!(degraded, unhealthy);
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_health_status_clone() {
        let status1 = HealthStatus::Degraded;
        let status2 = status1.clone();
        
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_health_status_serialization() {
        let status = HealthStatus::Healthy;
        let json = serde_json::to_string(&status).expect("Failed to serialize");
        
        let deserialized: HealthStatus = serde_json::from_str(&json)
            .expect("Failed to deserialize");
        
        assert_eq!(status, deserialized);
    }

    #[test]
    fn test_health_status_debug() {
        let status = HealthStatus::Healthy;
        let debug_str = format!("{:?}", status);
        
        assert!(debug_str.contains("Healthy"));
    }

    // ==================== Metrics Tests ====================

    #[test]
    fn test_metrics_creation() {
        let metrics = Metrics {
            requests_processed: 1000,
            errors_encountered: 5,
            average_response_time: Duration::from_millis(150),
            memory_usage_bytes: 1024 * 1024,
        };

        assert_eq!(metrics.requests_processed, 1000);
        assert_eq!(metrics.errors_encountered, 5);
        assert_eq!(metrics.average_response_time, Duration::from_millis(150));
        assert_eq!(metrics.memory_usage_bytes, 1024 * 1024);
    }

    #[test]
    fn test_metrics_default() {
        let metrics = Metrics::default();

        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
        assert_eq!(metrics.average_response_time, Duration::from_millis(0));
        assert_eq!(metrics.memory_usage_bytes, 0);
    }

    #[test]
    fn test_metrics_clone() {
        let metrics1 = Metrics {
            requests_processed: 500,
            errors_encountered: 2,
            average_response_time: Duration::from_millis(200),
            memory_usage_bytes: 2048,
        };

        let metrics2 = metrics1.clone();

        assert_eq!(metrics1.requests_processed, metrics2.requests_processed);
        assert_eq!(metrics1.errors_encountered, metrics2.errors_encountered);
    }

    #[test]
    fn test_metrics_serialization() {
        let metrics = Metrics::default();
        let json = serde_json::to_string(&metrics).expect("Failed to serialize");
        
        assert!(json.contains("requests_processed"));
        assert!(json.contains("errors_encountered"));
    }

    #[test]
    fn test_metrics_deserialization() {
        let metrics = Metrics {
            requests_processed: 123,
            errors_encountered: 4,
            average_response_time: Duration::from_millis(100),
            memory_usage_bytes: 4096,
        };

        let json = serde_json::to_string(&metrics).unwrap();
        let deserialized: Metrics = serde_json::from_str(&json)
            .expect("Failed to deserialize");

        assert_eq!(metrics.requests_processed, deserialized.requests_processed);
        assert_eq!(metrics.errors_encountered, deserialized.errors_encountered);
    }

    #[test]
    fn test_metrics_debug() {
        let metrics = Metrics::default();
        let debug_str = format!("{:?}", metrics);
        
        assert!(debug_str.contains("Metrics"));
        assert!(debug_str.contains("requests_processed"));
    }

    // ==================== DefaultService Tests ====================

    #[test]
    fn test_default_service_creation() {
        let config = Config::default();
        let service = DefaultService::new(config);

        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("DefaultService"));
    }

    #[test]
    fn test_default_service_with_custom_config() {
        let config = Config {
            enabled: true,
            timeout: Duration::from_secs(60),
            max_connections: 200,
            buffer_size: 16384,
        };

        let service = DefaultService::new(config);
        assert!(std::mem::size_of_val(&service) > 0);
    }

    #[test]
    fn test_create_service_helper() {
        let service = create_service();
        assert!(std::mem::size_of_val(&service) > 0);
    }

    // ==================== Configuration Validation Tests ====================

    #[tokio::test]
    async fn test_validate_config_valid() {
        let config = Config::default();
        let result = validate_config(&config).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_config_zero_connections() {
        let config = Config {
            enabled: true,
            timeout: Duration::from_secs(30),
            max_connections: 0,
            buffer_size: 8192,
        };

        let result = validate_config(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_config_zero_buffer() {
        let config = Config {
            enabled: true,
            timeout: Duration::from_secs(30),
            max_connections: 100,
            buffer_size: 0,
        };

        let result = validate_config(&config).await;
        assert!(result.is_err());
    }

    // ==================== Async Service Tests ====================

    #[tokio::test]
    async fn test_service_get_metrics() {
        let service = create_service();
        let metrics = service.get_metrics().await;

        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_config_disabled() {
        let config = Config {
            enabled: false,
            timeout: Duration::from_secs(30),
            max_connections: 100,
            buffer_size: 8192,
        };

        assert!(!config.enabled);
    }

    #[test]
    fn test_config_large_values() {
        let config = Config {
            enabled: true,
            timeout: Duration::from_secs(3600),
            max_connections: 10000,
            buffer_size: 1024 * 1024,
        };

        assert_eq!(config.max_connections, 10000);
        assert_eq!(config.buffer_size, 1024 * 1024);
    }

    #[test]
    fn test_metrics_high_load() {
        let metrics = Metrics {
            requests_processed: 1_000_000,
            errors_encountered: 100,
            average_response_time: Duration::from_millis(500),
            memory_usage_bytes: 1024 * 1024 * 100,
        };

        assert_eq!(metrics.requests_processed, 1_000_000);
        assert!(metrics.errors_encountered < metrics.requests_processed);
    }

    #[test]
    fn test_metrics_error_rate_calculation() {
        let metrics = Metrics {
            requests_processed: 1000,
            errors_encountered: 10,
            average_response_time: Duration::from_millis(100),
            memory_usage_bytes: 1024,
        };

        let error_rate = (metrics.errors_encountered as f64 
            / metrics.requests_processed as f64) * 100.0;
        
        assert!((error_rate - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_health_status_all_variants() {
        let statuses = vec![
            HealthStatus::Healthy,
            HealthStatus::Degraded,
            HealthStatus::Unhealthy,
        ];

        assert_eq!(statuses.len(), 3);
        assert_eq!(statuses[0], HealthStatus::Healthy);
        assert_eq!(statuses[1], HealthStatus::Degraded);
        assert_eq!(statuses[2], HealthStatus::Unhealthy);
    }

    #[test]
    fn test_config_timeout_range() {
        let short_timeout = Config {
            enabled: true,
            timeout: Duration::from_millis(100),
            max_connections: 50,
            buffer_size: 4096,
        };

        let long_timeout = Config {
            enabled: true,
            timeout: Duration::from_secs(300),
            max_connections: 50,
            buffer_size: 4096,
        };

        assert!(short_timeout.timeout < long_timeout.timeout);
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_config_metrics_workflow() {
        let config = Config::default();
        let _service = DefaultService::new(config);
        
        let metrics = Metrics::default();
        assert_eq!(metrics.requests_processed, 0);
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let config = Config::default();
        assert!(validate_config(&config).await.is_ok());
        
        let service = DefaultService::new(config);
        let metrics = service.get_metrics().await;
        
        assert_eq!(metrics.errors_encountered, 0);
    }
}

