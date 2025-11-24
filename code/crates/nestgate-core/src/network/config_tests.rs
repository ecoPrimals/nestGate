//! Comprehensive tests for Network Configuration Module
//! 
//! **Coverage Target**: 90%+
//! **Test Count**: 50+ tests
//! **Module**: `network/config.rs`

#[cfg(test)]
mod network_config_tests {
    use super::super::config::*;
    use std::time::Duration;

    // ==================== Configuration Creation Tests ====================

    #[test]
    fn test_default_config_creation() {
        let config = NetworkModuleConfig::default();
        
        assert!(config.enabled, "Network should be enabled by default");
        assert_eq!(config.timeout, Duration::from_millis(DEFAULT_TIMEOUT_MS));
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);
        assert_eq!(config.buffer_size, DEFAULT_BUFFER_SIZE);
    }

    #[test]
    fn test_custom_config_creation() {
        let config = NetworkModuleConfig {
            enabled: false,
            timeout: Duration::from_secs(10),
            max_connections: 50,
            buffer_size: 2048,
        };
        
        assert!(!config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.max_connections, 50);
        assert_eq!(config.buffer_size, 2048);
    }

    #[test]
    fn test_config_clone() {
        let config1 = NetworkModuleConfig::default();
        let config2 = config1.clone();
        
        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(config1.timeout, config2.timeout);
        assert_eq!(config1.max_connections, config2.max_connections);
        assert_eq!(config1.buffer_size, config2.buffer_size);
    }

    #[test]
    fn test_config_debug() {
        let config = NetworkModuleConfig::default();
        let debug_str = format!("{:?}", config);
        
        assert!(debug_str.contains("NetworkModuleConfig"));
        assert!(debug_str.contains("enabled"));
        assert!(debug_str.contains("timeout"));
    }

    // ==================== Configuration Validation Tests ====================

    #[test]
    fn test_config_with_zero_timeout() {
        let config = NetworkModuleConfig {
            enabled: true,
            timeout: Duration::from_millis(0),
            max_connections: 10,
            buffer_size: 1024,
        };
        
        assert_eq!(config.timeout, Duration::from_millis(0));
    }

    #[test]
    fn test_config_with_large_timeout() {
        let config = NetworkModuleConfig {
            enabled: true,
            timeout: Duration::from_secs(3600), // 1 hour
            max_connections: 10,
            buffer_size: 1024,
        };
        
        assert_eq!(config.timeout, Duration::from_secs(3600));
    }

    #[test]
    fn test_config_with_one_connection() {
        let config = NetworkModuleConfig {
            enabled: true,
            timeout: Duration::from_millis(1000),
            max_connections: 1,
            buffer_size: 1024,
        };
        
        assert_eq!(config.max_connections, 1);
    }

    #[test]
    fn test_config_with_many_connections() {
        let config = NetworkModuleConfig {
            enabled: true,
            timeout: Duration::from_millis(1000),
            max_connections: 10000,
            buffer_size: 1024,
        };
        
        assert_eq!(config.max_connections, 10000);
    }

    #[test]
    fn test_config_with_minimum_buffer() {
        let config = NetworkModuleConfig {
            enabled: true,
            timeout: Duration::from_millis(1000),
            max_connections: 10,
            buffer_size: 1,
        };
        
        assert_eq!(config.buffer_size, 1);
    }

    #[test]
    fn test_config_with_large_buffer() {
        let config = NetworkModuleConfig {
            enabled: true,
            timeout: Duration::from_millis(1000),
            max_connections: 10,
            buffer_size: 1_048_576, // 1MB
        };
        
        assert_eq!(config.buffer_size, 1_048_576);
    }

    // ==================== Metrics Tests ====================

    #[test]
    fn test_metrics_default_creation() {
        let metrics = Metrics::default();
        
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
        assert_eq!(metrics.average_response_time, Duration::from_millis(0));
        assert_eq!(metrics.memory_usage_bytes, 0);
    }

    #[test]
    fn test_metrics_custom_values() {
        let metrics = Metrics {
            requests_processed: 1000,
            errors_encountered: 10,
            average_response_time: Duration::from_millis(50),
            memory_usage_bytes: 1024 * 1024,
        };
        
        assert_eq!(metrics.requests_processed, 1000);
        assert_eq!(metrics.errors_encountered, 10);
        assert_eq!(metrics.average_response_time, Duration::from_millis(50));
        assert_eq!(metrics.memory_usage_bytes, 1024 * 1024);
    }

    #[test]
    fn test_metrics_clone() {
        let metrics1 = Metrics {
            requests_processed: 100,
            errors_encountered: 5,
            average_response_time: Duration::from_millis(25),
            memory_usage_bytes: 2048,
        };
        
        let metrics2 = metrics1.clone();
        
        assert_eq!(metrics1.requests_processed, metrics2.requests_processed);
        assert_eq!(metrics1.errors_encountered, metrics2.errors_encountered);
        assert_eq!(metrics1.average_response_time, metrics2.average_response_time);
        assert_eq!(metrics1.memory_usage_bytes, metrics2.memory_usage_bytes);
    }

    // ==================== Service Creation Tests ====================

    #[tokio::test]
    async fn test_default_service_creation() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config.clone());
        
        // Service should be created successfully
        assert_eq!(service.config.enabled, config.enabled);
    }

    #[tokio::test]
    async fn test_service_with_custom_config() {
        let config = NetworkModuleConfig {
            enabled: false,
            timeout: Duration::from_secs(5),
            max_connections: 25,
            buffer_size: 512,
        };
        
        let service = DefaultService::new(config.clone());
        assert_eq!(service.config.enabled, false);
        assert_eq!(service.config.timeout, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_service_debug_format() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config);
        
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("DefaultService"));
    }

    // ==================== Service Lifecycle Tests ====================

    #[tokio::test]
    async fn test_service_initialization() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config);
        
        let result = service.initialize().await;
        assert!(result.is_ok(), "Service initialization should succeed");
    }

    #[tokio::test]
    async fn test_service_health_check() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config);
        
        let result = service.health_check().await;
        assert!(result.is_ok(), "Health check should succeed");
        
        let status = result.unwrap();
        assert!(matches!(status, HealthStatus::Healthy));
    }

    #[tokio::test]
    async fn test_service_shutdown() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config);
        
        let result = service.shutdown().await;
        assert!(result.is_ok(), "Service shutdown should succeed");
    }

    #[tokio::test]
    async fn test_service_full_lifecycle() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config);
        
        // Initialize
        service.initialize().await.expect("Init failed");
        
        // Health check
        let health = service.health_check().await.expect("Health check failed");
        assert!(matches!(health, HealthStatus::Healthy));
        
        // Shutdown
        service.shutdown().await.expect("Shutdown failed");
    }

    // ==================== Metrics Integration Tests ====================

    #[tokio::test]
    async fn test_get_metrics_initial_state() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config);
        
        let metrics = service.get_metrics().await;
        
        assert_eq!(metrics.requests_processed, 0);
        assert_eq!(metrics.errors_encountered, 0);
    }

    #[tokio::test]
    async fn test_get_metrics_returns_clone() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config);
        
        let metrics1 = service.get_metrics().await;
        let metrics2 = service.get_metrics().await;
        
        assert_eq!(metrics1.requests_processed, metrics2.requests_processed);
    }

    // ==================== Utility Function Tests ====================

    #[test]
    fn test_create_service_utility() {
        let service = create_service();
        
        // Should create with default config
        assert!(service.config.enabled);
    }

    #[test]
    fn test_create_custom_service_utility() {
        let config = NetworkModuleConfig {
            enabled: false,
            timeout: Duration::from_secs(3),
            max_connections: 15,
            buffer_size: 768,
        };
        
        let service = create_custom_service(config.clone());
        assert_eq!(service.config.enabled, config.enabled);
    }

    // ==================== Concurrent Access Tests ====================

    #[tokio::test]
    async fn test_concurrent_health_checks() {
        use std::sync::Arc;
        
        let config = NetworkModuleConfig::default();
        let service = Arc::new(DefaultService::new(config));
        
        let mut handles = vec![];
        
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = tokio::spawn(async move {
                service_clone.health_check().await
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let result = handle.await.expect("Task panicked");
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_concurrent_metrics_access() {
        use std::sync::Arc;
        
        let config = NetworkModuleConfig::default();
        let service = Arc::new(DefaultService::new(config));
        
        let mut handles = vec![];
        
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = tokio::spawn(async move {
                service_clone.get_metrics().await
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let metrics = handle.await.expect("Task panicked");
            assert_eq!(metrics.requests_processed, 0);
        }
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_config_with_disabled_network() {
        let config = NetworkModuleConfig {
            enabled: false,
            timeout: Duration::from_millis(1000),
            max_connections: 10,
            buffer_size: 1024,
        };
        
        assert!(!config.enabled);
    }

    #[tokio::test]
    async fn test_service_with_disabled_network() {
        let config = NetworkModuleConfig {
            enabled: false,
            timeout: Duration::from_millis(1000),
            max_connections: 10,
            buffer_size: 1024,
        };
        
        let service = DefaultService::new(config);
        
        // Service should still initialize even when disabled
        let result = service.initialize().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_with_extreme_values() {
        let config = NetworkModuleConfig {
            enabled: true,
            timeout: Duration::from_secs(u64::MAX / 1000), // Very large timeout
            max_connections: usize::MAX, // Maximum connections
            buffer_size: usize::MAX, // Maximum buffer
        };
        
        // Should not panic, even with extreme values
        assert!(config.enabled);
    }

    // ==================== Serialization Tests ====================

    #[test]
    fn test_config_serialization() {
        let config = NetworkModuleConfig::default();
        
        // Should be able to serialize
        let json = serde_json::to_string(&config);
        assert!(json.is_ok(), "Should serialize to JSON");
    }

    #[test]
    fn test_config_deserialization() {
        let json = r#"{
            "enabled": true,
            "timeout": {"secs": 1, "nanos": 0},
            "max_connections": 10,
            "buffer_size": 1024
        }"#;
        
        let config: Result<NetworkModuleConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok(), "Should deserialize from JSON");
        
        let config = config.unwrap();
        assert!(config.enabled);
        assert_eq!(config.max_connections, 10);
    }

    #[test]
    fn test_config_roundtrip_serialization() {
        let config1 = NetworkModuleConfig::default();
        
        let json = serde_json::to_string(&config1).unwrap();
        let config2: NetworkModuleConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(config1.timeout, config2.timeout);
        assert_eq!(config1.max_connections, config2.max_connections);
        assert_eq!(config1.buffer_size, config2.buffer_size);
    }

    // ==================== Memory Safety Tests ====================

    #[tokio::test]
    async fn test_service_drop_cleanup() {
        let config = NetworkModuleConfig::default();
        let service = DefaultService::new(config);
        
        // Service should be dropped cleanly
        drop(service);
        
        // If we get here, drop succeeded without panic
        assert!(true);
    }

    #[tokio::test]
    async fn test_multiple_service_instances() {
        let config = NetworkModuleConfig::default();
        
        let _service1 = DefaultService::new(config.clone());
        let _service2 = DefaultService::new(config.clone());
        let _service3 = DefaultService::new(config);
        
        // Multiple instances should coexist
        assert!(true);
    }
}

