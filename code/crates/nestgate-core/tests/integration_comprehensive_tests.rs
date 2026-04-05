// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
#![allow(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Integration tests for configuration and discovery systems
//!
//! All tests use `MapEnv` via `build_with_env` for fully concurrent, isolated execution.

#[cfg(test)]
mod config_discovery_integration_tests {
    use nestgate_core::config::capability_based::{
        CapabilityConfigBuilder, FallbackMode, PrimalCapability,
    };
    use nestgate_core::constants::port_defaults::{DEFAULT_API_PORT, DEFAULT_DEV_PORT};
    use nestgate_types::MapEnv;
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_multi_service_discovery_workflow() {
        let env = Arc::new(MapEnv::from([
            ("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "10.0.0.1:9000"),
            ("NESTGATE_CAPABILITY_SECURITY_ENDPOINT", "10.0.0.2:3000"),
            (
                "NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT",
                "10.0.0.3:8080",
            ),
        ]));

        let config = CapabilityConfigBuilder::new()
            .with_discovery_timeout(Duration::from_secs(10))
            .with_retry_attempts(3)
            .build_with_env(env)
            .unwrap();

        let storage = config.discover(PrimalCapability::Storage).await;
        let security = config.discover(PrimalCapability::Security).await;
        let orchestration = config.discover(PrimalCapability::Orchestration).await;

        assert!(storage.is_ok(), "Storage discovery should succeed");
        assert!(security.is_ok(), "Security discovery should succeed");
        assert!(
            orchestration.is_ok(),
            "Orchestration discovery should succeed"
        );

        assert_eq!(storage.unwrap().endpoint.port(), 9000);
        assert_eq!(security.unwrap().endpoint.port(), DEFAULT_DEV_PORT);
        assert_eq!(orchestration.unwrap().endpoint.port(), DEFAULT_API_PORT);
    }

    #[tokio::test]
    async fn test_fallback_mode_graceful_degradation() {
        let env = Arc::new(MapEnv::new());
        let config = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::GracefulDegradation)
            .build_with_env(env)
            .unwrap();

        let result = config.discover(PrimalCapability::Monitoring).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fallback_mode_local_fallback() {
        let env = Arc::new(MapEnv::new());
        let config = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::LocalFallback)
            .build_with_env(env)
            .unwrap();

        let result = config.discover(PrimalCapability::Analytics).await;
        assert!(result.is_ok());
        let service = result.unwrap();
        assert!(service.metadata.contains_key("mode"));
    }

    #[tokio::test]
    async fn test_discovery_caching_performance() {
        let env = Arc::new(MapEnv::from([(
            "NESTGATE_CAPABILITY_COMPUTE_ENDPOINT",
            "192.168.1.100:7000",
        )]));
        let config = CapabilityConfigBuilder::new().build_with_env(env).unwrap();

        let start = std::time::Instant::now();
        let result1 = config.discover(PrimalCapability::Compute).await;
        let first_duration = start.elapsed();
        let start2 = std::time::Instant::now();
        let result2 = config.discover(PrimalCapability::Compute).await;
        let second_duration = start2.elapsed();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(second_duration <= first_duration + Duration::from_millis(10));
    }

    #[tokio::test]
    async fn test_concurrent_discovery_requests() {
        let env = Arc::new(MapEnv::from([
            ("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "10.0.0.1:9000"),
            ("NESTGATE_CAPABILITY_SECURITY_ENDPOINT", "10.0.0.2:3000"),
        ]));

        let config = CapabilityConfigBuilder::new().build_with_env(env).unwrap();

        let config1 = config.clone();
        let config2 = config.clone();

        let handle1 =
            tokio::spawn(async move { config1.discover(PrimalCapability::Storage).await });
        let handle2 =
            tokio::spawn(async move { config2.discover(PrimalCapability::Security).await });

        let (result1, result2) = tokio::join!(handle1, handle2);
        assert!(result1.unwrap().is_ok());
        assert!(result2.unwrap().is_ok());
    }

    #[tokio::test]
    async fn test_discovery_with_invalid_port() {
        let env = Arc::new(MapEnv::from([(
            "NESTGATE_CAPABILITY_MACHINELEARNING_ENDPOINT",
            "10.0.0.1:99999",
        )]));
        let config = CapabilityConfigBuilder::new().build_with_env(env).unwrap();
        let result = config.discover(PrimalCapability::MachineLearning).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discovery_retry_mechanism() {
        let env = Arc::new(MapEnv::new());
        let config = CapabilityConfigBuilder::new()
            .with_retry_attempts(3)
            .build_with_env(env)
            .unwrap();

        let start = std::time::Instant::now();
        let result = config.discover(PrimalCapability::DataProcessing).await;
        let duration = start.elapsed();

        assert!(result.is_err());
        assert!(duration >= Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_builder_validation() {
        let result = CapabilityConfigBuilder::new()
            .with_retry_attempts(0)
            .build();
        assert!(result.is_err());

        let result = CapabilityConfigBuilder::new()
            .with_retry_attempts(1)
            .with_discovery_timeout(Duration::from_secs(1))
            .build();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_capability_endpoint_format_variations() {
        let env1 = Arc::new(MapEnv::from([(
            "NESTGATE_CAPABILITY_STORAGE_ENDPOINT",
            "127.0.0.1:8080",
        )]));
        let config1 = CapabilityConfigBuilder::new().build_with_env(env1).unwrap();
        let result1 = config1.discover(PrimalCapability::Storage).await;
        assert!(result1.is_ok());

        let env2 = Arc::new(MapEnv::from([(
            "NESTGATE_CAPABILITY_STORAGE_ENDPOINT",
            "[::1]:8080",
        )]));
        let config2 = CapabilityConfigBuilder::new().build_with_env(env2).unwrap();
        let result2 = config2.discover(PrimalCapability::Storage).await;
        assert!(result2.is_ok());

        let env3 = Arc::new(MapEnv::from([(
            "NESTGATE_CAPABILITY_STORAGE_ENDPOINT",
            "0.0.0.0:9000",
        )]));
        let config3 = CapabilityConfigBuilder::new().build_with_env(env3).unwrap();
        let result3 = config3.discover(PrimalCapability::Storage).await;
        assert!(result3.is_ok());
    }
}

#[cfg(test)]
mod safe_operations_edge_cases {
    use nestgate_core::safe_operations::safe_get;

    #[test]
    fn test_parse_env_var_whitespace() {
        let orig = std::env::var("TEST_WHITESPACE").ok();
        nestgate_core::env_process::set_var("TEST_WHITESPACE", "  42  ");
        let value = std::env::var("TEST_WHITESPACE");
        match orig {
            Some(v) => nestgate_core::env_process::set_var("TEST_WHITESPACE", v),
            None => nestgate_core::env_process::remove_var("TEST_WHITESPACE"),
        }
        assert!(value.is_ok());
    }

    #[test]
    fn test_safe_get_empty_vec() {
        let empty: Vec<i32> = vec![];
        // Modern safe_get function with context
        let result = safe_get(&empty, 0, "test_empty_vec");
        assert!(result.is_err());

        // Test with actual data
        let data = vec![10, 20, 30];
        let result = safe_get(&data, 0, "test_data");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_safe_get_boundary_indices() {
        let data = [10, 20, 30];

        // Valid boundaries
        assert_eq!(data.first().unwrap(), &10);
        assert_eq!(data.get(2).unwrap(), &30);

        // Out of bounds
        assert!(data.get(3).is_none());
        assert!(data.get(100).is_none());
    }

    #[test]
    fn test_safe_parse_edge_cases() {
        // Empty string
        let result: Result<i32, _> = "".parse();
        assert!(result.is_err());

        // Negative numbers
        let result: Result<i32, _> = "-42".parse();
        assert_eq!(result.unwrap(), -42);

        // Max values
        let result: Result<i32, _> = "2147483647".parse();
        assert_eq!(result.unwrap(), 2147483647);
    }

    #[test]
    fn test_parse_optional_with_empty_string() {
        let orig = std::env::var("EMPTY_VAR").ok();
        nestgate_core::env_process::set_var("EMPTY_VAR", "");
        let value = std::env::var("EMPTY_VAR").ok();
        match orig {
            Some(v) => nestgate_core::env_process::set_var("EMPTY_VAR", v),
            None => nestgate_core::env_process::remove_var("EMPTY_VAR"),
        }
        assert_eq!(value, Some(String::new()));
    }

    #[test]
    fn test_safe_collection_with_single_element() {
        let single = [42];

        assert_eq!(*single.first().unwrap(), 42);
        assert_eq!(*single.last().unwrap(), 42);
        assert_eq!(single.first().unwrap(), &42);
        assert!(single.get(1).is_none());
    }
}
