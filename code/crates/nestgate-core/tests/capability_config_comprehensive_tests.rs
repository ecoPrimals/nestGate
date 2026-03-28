// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive test suite for capability-based configuration
//!
//! Tests runtime discovery, error handling, fallback modes, and sovereignty compliance.

#[cfg(test)]
mod capability_config_tests {
    use nestgate_core::config::capability_based::{CapabilityConfigBuilder, FallbackMode};
    use nestgate_core::universal_traits::types::PrimalCapability;
    use std::time::Duration;

    #[test]
    fn test_builder_default_values() {
        let config = CapabilityConfigBuilder::new().build().unwrap();
        assert_eq!(config.discovery_timeout(), Duration::from_secs(5));
        assert_eq!(config.retry_attempts(), 3);
    }

    #[test]
    fn test_builder_custom_timeout() {
        let timeout = Duration::from_secs(10);
        let config = CapabilityConfigBuilder::new()
            .with_discovery_timeout(timeout)
            .build()
            .unwrap();
        assert_eq!(config.discovery_timeout(), timeout);
    }

    #[test]
    fn test_builder_custom_retries() {
        let config = CapabilityConfigBuilder::new()
            .with_retry_attempts(5)
            .build()
            .unwrap();
        assert_eq!(config.retry_attempts(), 5);
    }

    #[test]
    fn test_builder_zero_retries_rejected() {
        let result = CapabilityConfigBuilder::new()
            .with_retry_attempts(0)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_builder_all_fallback_modes() {
        let fail_fast = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::FailFast)
            .build()
            .unwrap();
        assert_eq!(fail_fast.fallback_mode(), FallbackMode::FailFast);

        let graceful = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::GracefulDegradation)
            .build()
            .unwrap();
        assert_eq!(graceful.fallback_mode(), FallbackMode::GracefulDegradation);

        let local = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::LocalFallback)
            .build()
            .unwrap();
        assert_eq!(local.fallback_mode(), FallbackMode::LocalFallback);
    }

    #[tokio::test]
    async fn test_discovery_from_env_var() {
        let orig = std::env::var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT").ok();
        nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "127.0.0.1:9000");

        let config = CapabilityConfigBuilder::new().build().unwrap();
        let result = config.discover(PrimalCapability::Storage).await;

        match orig {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT"),
        }
        assert!(result.is_ok());
        let service = result.unwrap();
        assert_eq!(service.capability, PrimalCapability::Storage);
        assert_eq!(service.endpoint.port(), 9000);
    }

    #[tokio::test]
    async fn test_discovery_missing_env_var_fail_fast() {
        let orig = std::env::var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT").ok();
        nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT");

        let config = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::FailFast)
            .build()
            .unwrap();

        let result = config.discover(PrimalCapability::Compute).await;
        if let Some(v) = orig {
            nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT", v);
        }
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discovery_invalid_endpoint_format() {
        let orig = std::env::var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT").ok();
        nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT", "invalid_format");

        let config = CapabilityConfigBuilder::new().build().unwrap();
        let result = config.discover(PrimalCapability::Security).await;

        match orig {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT"),
        }
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discovery_caching() {
        let orig = std::env::var("NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT").ok();
        nestgate_core::env_process::set_var(
            "NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT",
            "192.168.1.1:8080",
        );

        let config = CapabilityConfigBuilder::new().build().unwrap();

        let result1 = config.discover(PrimalCapability::Orchestration).await;
        let result2 = config.discover(PrimalCapability::Orchestration).await;

        match orig {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_ORCHESTRATION_ENDPOINT"),
        }
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        let service1 = result1.unwrap();
        let service2 = result2.unwrap();
        assert_eq!(service1.endpoint, service2.endpoint);
    }

    #[tokio::test]
    async fn test_multiple_capabilities_discovery() {
        let orig_s = std::env::var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT").ok();
        let orig_sec = std::env::var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT").ok();
        let orig_m = std::env::var("NESTGATE_CAPABILITY_MONITORING_ENDPOINT").ok();
        nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "10.0.0.1:9000");
        nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT", "10.0.0.2:3000");
        nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_MONITORING_ENDPOINT", "10.0.0.3:9090");

        let config = CapabilityConfigBuilder::new().build().unwrap();

        let storage = config.discover(PrimalCapability::Storage).await;
        let security = config.discover(PrimalCapability::Security).await;
        let monitoring = config.discover(PrimalCapability::Monitoring).await;

        match orig_s {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT"),
        }
        match orig_sec {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_SECURITY_ENDPOINT"),
        }
        match orig_m {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_MONITORING_ENDPOINT", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_MONITORING_ENDPOINT"),
        }
        assert!(storage.is_ok());
        assert!(security.is_ok());
        assert!(monitoring.is_ok());
        assert_eq!(storage.unwrap().endpoint.port(), 9000);
        assert_eq!(security.unwrap().endpoint.port(), 3000);
        assert_eq!(monitoring.unwrap().endpoint.port(), 9090);
    }

    #[tokio::test]
    async fn test_local_fallback_mode() {
        let orig = std::env::var("NESTGATE_CAPABILITY_ANALYTICS_ENDPOINT").ok();
        nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_ANALYTICS_ENDPOINT");

        let config = CapabilityConfigBuilder::new()
            .with_fallback_mode(FallbackMode::LocalFallback)
            .build()
            .unwrap();

        let result = config.discover(PrimalCapability::Analytics).await;
        if let Some(v) = orig {
            nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_ANALYTICS_ENDPOINT", v);
        }
        assert!(result.is_ok());
        let service = result.unwrap();
        assert!(service.metadata.get("mode") == Some(&"local_fallback".to_string()));
    }

    #[test]
    fn test_retry_attempts_boundary_values() {
        // Minimum valid (1)
        let config = CapabilityConfigBuilder::new()
            .with_retry_attempts(1)
            .build();
        assert!(config.is_ok());

        // Large value
        let config = CapabilityConfigBuilder::new()
            .with_retry_attempts(100)
            .build();
        assert!(config.is_ok());

        // Zero (invalid)
        let config = CapabilityConfigBuilder::new()
            .with_retry_attempts(0)
            .build();
        assert!(config.is_err());
    }

    #[test]
    fn test_timeout_boundary_values() {
        // Very short timeout
        let config = CapabilityConfigBuilder::new()
            .with_discovery_timeout(Duration::from_millis(1))
            .build();
        assert!(config.is_ok());

        // Very long timeout
        let config = CapabilityConfigBuilder::new()
            .with_discovery_timeout(Duration::from_secs(3600))
            .build();
        assert!(config.is_ok());

        // Zero timeout (edge case - should work but not recommended)
        let config = CapabilityConfigBuilder::new()
            .with_discovery_timeout(Duration::from_secs(0))
            .build();
        assert!(config.is_ok());
    }

    #[tokio::test]
    async fn test_ipv6_endpoint() {
        let orig = std::env::var("NESTGATE_CAPABILITY_DATAPROCESSING_ENDPOINT").ok();
        nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_DATAPROCESSING_ENDPOINT", "[::1]:8080");

        let config = CapabilityConfigBuilder::new().build().unwrap();
        let result = config.discover(PrimalCapability::DataProcessing).await;

        match orig {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_DATAPROCESSING_ENDPOINT", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_DATAPROCESSING_ENDPOINT"),
        }
        assert!(result.is_ok());
        let service = result.unwrap();
        assert_eq!(service.endpoint.port(), 8080);
    }

    #[test]
    fn test_config_builder_chain() {
        let config = CapabilityConfigBuilder::new()
            .with_discovery_timeout(Duration::from_secs(15))
            .with_retry_attempts(7)
            .with_fallback_mode(FallbackMode::GracefulDegradation)
            .build()
            .unwrap();

        assert_eq!(config.discovery_timeout(), Duration::from_secs(15));
        assert_eq!(config.retry_attempts(), 7);
        assert_eq!(config.fallback_mode(), FallbackMode::GracefulDegradation);
    }

    #[tokio::test]
    #[ignore] // Requires network/socket for capability discovery
    async fn test_discovery_sovereignty_compliance() {
        let orig = std::env::var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT").ok();
        nestgate_core::env_process::set_var(
            "NESTGATE_CAPABILITY_COMPUTE_ENDPOINT",
            "discovered.service:7000",
        );

        let config = CapabilityConfigBuilder::new().build().unwrap();
        let result = config.discover(PrimalCapability::Compute).await;

        match orig {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT"),
        }
        assert!(result.is_ok());
        let service = result.unwrap();
        assert_eq!(service.capability, PrimalCapability::Compute);
        assert!(!service.metadata.contains_key("primal_name"));
    }
}

#[cfg(test)]
mod discovered_service_tests {
    use nestgate_core::config::capability_based::DiscoveredService;
    use nestgate_core::universal_traits::types::PrimalCapability;
    use std::collections::HashMap;

    #[test]
    fn test_discovered_service_creation() {
        let service = DiscoveredService {
            capability: PrimalCapability::Storage,
            endpoint: "127.0.0.1:9000".parse().unwrap(),
            metadata: HashMap::new(),
            discovered_at: std::time::Instant::now(),
        };

        assert_eq!(service.capability, PrimalCapability::Storage);
        assert_eq!(service.endpoint.port(), 9000);
    }

    #[test]
    fn test_discovered_service_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("region".to_string(), "us-west".to_string());

        let service = DiscoveredService {
            capability: PrimalCapability::Security,
            endpoint: "10.0.0.1:3000".parse().unwrap(),
            metadata: metadata.clone(),
            discovered_at: std::time::Instant::now(),
        };

        assert_eq!(service.metadata.get("version"), Some(&"1.0.0".to_string()));
        assert_eq!(service.metadata.get("region"), Some(&"us-west".to_string()));
    }

    #[test]
    fn test_discovered_service_clone() {
        let service1 = DiscoveredService {
            capability: PrimalCapability::Orchestration,
            endpoint: "192.168.1.1:8080".parse().unwrap(),
            metadata: HashMap::new(),
            discovered_at: std::time::Instant::now(),
        };

        let service2 = service1.clone();
        assert_eq!(service1.capability, service2.capability);
        assert_eq!(service1.endpoint, service2.endpoint);
    }
}
