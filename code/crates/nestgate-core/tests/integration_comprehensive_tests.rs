//! Integration tests for configuration and discovery systems
//!
//! Tests the integration between capability-based config and service discovery.

#[cfg(test)]
mod config_discovery_integration_tests {
    use nestgate_core::config::capability_based::{CapabilityConfigBuilder, FallbackMode};
    use nestgate_core::constants::port_defaults::{DEFAULT_API_PORT, DEFAULT_DEV_PORT};
    use nestgate_core::universal_traits::types::PrimalCapability;
    use std::time::Duration;

    #[tokio::test]
    async fn test_multi_service_discovery_workflow() {
        // Setup multiple services with centralized port constants
        std::env::set_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "10.0.0.1:9000");
        std::env::set_var(
            "NESTGATE_CAPABILITY_SECURITY_ENDPOINT",
            format!("10.0.0.2:{}", DEFAULT_DEV_PORT),
        );
        std::env::set_var(
            "NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT",
            format!("10.0.0.3:{}", DEFAULT_API_PORT),
        );

        let config = CapabilityConfigBuilder::new()
            .with_discovery_timeout(Duration::from_secs(10))
            .with_retry_attempts(3)
            .build()
            .unwrap();

        // Discover all services
        let storage = config.discover(PrimalCapability::Storage).await;
        let security = config.discover(PrimalCapability::Security).await;
        let orchestration = config.discover(PrimalCapability::Orchestration).await;

        assert!(storage.is_ok());
        assert!(security.is_ok());
        assert!(orchestration.is_ok());

        // Verify endpoints match our centralized constants
        assert_eq!(storage.unwrap().endpoint.port(), 9000);
        assert_eq!(security.unwrap().endpoint.port(), DEFAULT_DEV_PORT);
        assert_eq!(orchestration.unwrap().endpoint.port(), DEFAULT_API_PORT);

        // Cleanup
        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");
        std::env::remove_var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT");
        std::env::remove_var("NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT");
    }

    #[tokio::test]
    async fn test_fallback_mode_graceful_degradation() {
        std::env::remove_var("NESTGATE_CAPABILITY_MONITORING_ENDPOINT");

        let config = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::GracefulDegradation)
            .build()
            .unwrap();

        let result = config.discover(PrimalCapability::Monitoring).await;

        // Should fail gracefully with error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fallback_mode_local_fallback() {
        std::env::remove_var("NESTGATE_CAPABILITY_ANALYTICS_ENDPOINT");

        let config = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::LocalFallback)
            .build()
            .unwrap();

        let result = config.discover(PrimalCapability::Analytics).await;

        // Should succeed with local fallback
        assert!(result.is_ok());
        let service = result.unwrap();
        assert!(service.metadata.contains_key("mode"));
    }

    #[tokio::test]
    async fn test_discovery_caching_performance() {
        std::env::set_var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT", "192.168.1.100:7000");

        let config = CapabilityConfigBuilder::new().build().unwrap();

        // First discovery
        let start = std::time::Instant::now();
        let result1 = config.discover(PrimalCapability::Compute).await;
        let first_duration = start.elapsed();
        assert!(result1.is_ok());

        // Second discovery (cached)
        let start = std::time::Instant::now();
        let result2 = config.discover(PrimalCapability::Compute).await;
        let second_duration = start.elapsed();
        assert!(result2.is_ok());

        // Cached access should be faster (or at least not significantly slower)
        assert!(second_duration <= first_duration + Duration::from_millis(10));

        std::env::remove_var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT");
    }

    #[tokio::test]
    async fn test_concurrent_discovery_requests() {
        std::env::set_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "10.0.0.1:9000");
        std::env::set_var(
            "NESTGATE_CAPABILITY_SECURITY_ENDPOINT",
            format!("10.0.0.2:{}", DEFAULT_DEV_PORT),
        );

        let config = CapabilityConfigBuilder::new().build().unwrap();

        // Spawn concurrent discoveries
        let config1 = config.clone();
        let config2 = config.clone();

        let handle1 =
            tokio::spawn(async move { config1.discover(PrimalCapability::Storage).await });

        let handle2 =
            tokio::spawn(async move { config2.discover(PrimalCapability::Security).await });

        let (result1, result2) = tokio::join!(handle1, handle2);

        assert!(result1.unwrap().is_ok());
        assert!(result2.unwrap().is_ok());

        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");
        std::env::remove_var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT");
    }

    #[tokio::test]
    async fn test_discovery_with_invalid_port() {
        std::env::set_var(
            "NESTGATE_CAPABILITY_MACHINELEARNING_ENDPOINT",
            "10.0.0.1:99999",
        );

        let config = CapabilityConfigBuilder::new().build().unwrap();
        let result = config.discover(PrimalCapability::MachineLearning).await;

        // Should fail due to invalid port
        assert!(result.is_err());

        std::env::remove_var("NESTGATE_CAPABILITY_MACHINELEARNING_ENDPOINT");
    }

    #[tokio::test]
    async fn test_discovery_retry_mechanism() {
        // Don't set env var, let it retry and fail
        std::env::remove_var("NESTGATE_CAPABILITY_DATAPROCESSING_ENDPOINT");

        let config = CapabilityConfigBuilder::new()
            .with_retry_attempts(3)
            .build()
            .unwrap();

        let start = std::time::Instant::now();
        let result = config.discover(PrimalCapability::DataProcessing).await;
        let duration = start.elapsed();

        // Should fail after retries
        assert!(result.is_err());

        // Should take some time due to retries with backoff
        assert!(duration >= Duration::from_millis(100)); // At least first retry delay
    }

    #[tokio::test]
    async fn test_builder_validation() {
        // Zero retries should be rejected
        let result = CapabilityConfigBuilder::new()
            .with_retry_attempts(0)
            .build();
        assert!(result.is_err());

        // Valid configuration
        let result = CapabilityConfigBuilder::new()
            .with_retry_attempts(1)
            .with_discovery_timeout(Duration::from_secs(1))
            .build();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_capability_endpoint_format_variations() {
        // Test different valid formats using centralized constants
        std::env::set_var(
            "NESTGATE_CAPABILITY_STORAGE_ENDPOINT",
            format!("127.0.0.1:{}", DEFAULT_API_PORT),
        );
        let config = CapabilityConfigBuilder::new().build().unwrap();
        let result = config.discover(PrimalCapability::Storage).await;
        assert!(result.is_ok());
        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");

        // IPv6
        std::env::set_var(
            "NESTGATE_CAPABILITY_STORAGE_ENDPOINT",
            format!("[::1]:{}", DEFAULT_API_PORT),
        );
        let config = CapabilityConfigBuilder::new().build().unwrap();
        let result = config.discover(PrimalCapability::Storage).await;
        assert!(result.is_ok());
        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");

        // Hostname with port
        std::env::set_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "0.0.0.0:9000");
        let config = CapabilityConfigBuilder::new().build().unwrap();
        let result = config.discover(PrimalCapability::Storage).await;
        assert!(result.is_ok());
        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");
    }
}

#[cfg(test)]
mod safe_operations_edge_cases {
    use nestgate_core::safe_operations::{
        parse_env_var, parse_env_var_optional, SafeCollectionExt, SafeStringExt,
    };

    #[test]
    fn test_parse_env_var_whitespace() {
        std::env::set_var("TEST_WHITESPACE", "  42  ");
        let result: nestgate_core::Result<String> = parse_env_var("TEST_WHITESPACE");
        assert!(result.is_ok());
        std::env::remove_var("TEST_WHITESPACE");
    }

    #[test]
    fn test_safe_get_empty_vec() {
        let empty: Vec<i32> = vec![];
        let result = empty.safe_get(0);
        assert!(result.is_err());

        let result = empty.safe_first();
        assert!(result.is_err());

        let result = empty.safe_last();
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_get_boundary_indices() {
        let data = vec![10, 20, 30];

        // Valid boundaries
        assert_eq!(*data.safe_get(0).unwrap(), 10);
        assert_eq!(*data.safe_get(2).unwrap(), 30);

        // Out of bounds
        assert!(data.safe_get(3).is_err());
        assert!(data.safe_get(100).is_err());
    }

    #[test]
    fn test_safe_parse_edge_cases() {
        // Empty string
        let result: nestgate_core::Result<i32> = "".safe_parse();
        assert!(result.is_err());

        // Negative numbers
        let result: nestgate_core::Result<i32> = "-42".safe_parse();
        assert_eq!(result.unwrap(), -42);

        // Max values
        let result: nestgate_core::Result<i32> = "2147483647".safe_parse();
        assert_eq!(result.unwrap(), 2147483647);
    }

    #[test]
    fn test_parse_optional_with_empty_string() {
        std::env::set_var("EMPTY_VAR", "");
        let result: nestgate_core::Result<Option<String>> = parse_env_var_optional("EMPTY_VAR");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(String::new()));
        std::env::remove_var("EMPTY_VAR");
    }

    #[test]
    fn test_safe_collection_with_single_element() {
        let single = vec![42];

        assert_eq!(*single.safe_first().unwrap(), 42);
        assert_eq!(*single.safe_last().unwrap(), 42);
        assert_eq!(*single.safe_get(0).unwrap(), 42);
        assert!(single.safe_get(1).is_err());
    }
}
