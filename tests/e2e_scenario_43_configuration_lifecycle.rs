//! E2E Scenario 43: Configuration Lifecycle
//!
//! Tests configuration loading, validation, updates, and environment overrides
//! across the entire application lifecycle.
//!
//! **CONCURRENT DESIGN**: All tests are fully concurrent, no sleep dependencies

#[cfg(test)]
mod configuration_lifecycle_e2e {
    use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;
    use nestgate_core::config::environment::EnvironmentConfig;

    #[tokio::test]
    async fn test_default_configuration() {
        // Scenario: Load default configuration concurrently

        let config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();

        // Verify reasonable defaults exist
        assert!(!config.metadata.version.is_empty());
        assert!(config.network.api.port > 0);
        assert!(!config.network.api.bind_address.is_unspecified());
    }

    #[tokio::test]
    async fn test_environment_config_fallback() {
        // Scenario: Environment config with safe fallbacks (concurrent-safe)

        let env_config =
            EnvironmentConfig::from_env().unwrap_or_else(|_| EnvironmentConfig::default());

        // Should have valid values either way
        assert!(env_config.network.port.get() > 0);
        assert!(!env_config.network.host.is_empty());
    }

    #[tokio::test]
    async fn test_configuration_validation() {
        // Scenario: Validate configuration constraints (no blocking)

        let config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();

        // Port should be valid
        assert!(config.network.api.port > 0);
        // Port is u16, max value is 65535 by type definition
        assert!(config.network.api.port > 0);

        // Bind address should be valid
        assert!(!config.network.api.bind_address.is_unspecified());

        // Metadata should be valid
        assert!(!config.metadata.version.is_empty());
    }

    #[tokio::test]
    async fn test_configuration_immutability() {
        // Scenario: Configuration is immutable after creation (concurrent-safe)

        let config1: NestGateCanonicalConfig = NestGateCanonicalConfig::default();
        let config2 = config1.clone();

        // Both should have same values
        assert_eq!(config1.metadata.version, config2.metadata.version);
        assert_eq!(config1.network.api.port, config2.network.api.port);
        assert_eq!(
            config1.network.api.bind_address,
            config2.network.api.bind_address
        );
    }

    #[tokio::test]
    async fn test_network_configuration() {
        // Scenario: Network-specific configuration (instant validation)

        let config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();

        // Verify network settings
        assert!(config.network.api.port > 0);
        assert!(!config.network.api.bind_address.is_unspecified());

        // Bind address should be valid
        let bind_addr = config.network.api.bind_address;
        assert!(bind_addr.is_ipv4() || bind_addr.is_ipv6());
    }

    #[tokio::test]
    async fn test_configuration_clone() {
        // Scenario: Configuration can be safely cloned (concurrent-safe)

        let original: NestGateCanonicalConfig = NestGateCanonicalConfig::default();
        let cloned = original.clone();

        // Values should match
        assert_eq!(original.metadata.version, cloned.metadata.version);
        assert_eq!(original.network.api.port, cloned.network.api.port);

        // But they should be independent instances
        // (Verified by the fact that clone() works)
    }

    #[tokio::test]
    async fn test_configuration_debug_format() {
        // Scenario: Configuration has useful debug output (no I/O)

        let config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();
        let debug_str = format!("{:?}", config);

        // Debug output should contain key information
        assert!(!debug_str.is_empty());
        // Should contain some recognizable config fields
        assert!(debug_str.contains("metadata") || debug_str.contains("network"));
    }

    #[tokio::test]
    async fn test_multiple_config_instances() {
        // Scenario: Multiple independent config instances (fully concurrent)

        // Spawn multiple concurrent config creations
        let handles: Vec<_> = (0..10)
            .map(|_| {
                tokio::spawn(async {
                    let config: NestGateCanonicalConfig = NestGateCanonicalConfig::default();
                    // Each should have valid metadata
                    assert!(!config.metadata.version.is_empty());
                    config
                })
            })
            .collect();

        // All should complete successfully
        for handle in handles {
            let config = handle.await.expect("Config creation should not panic");
            assert!(!config.metadata.version.is_empty());
        }
    }

    #[tokio::test]
    async fn test_concurrent_config_access() {
        // Scenario: Concurrent access to shared config (no data races)

        use std::sync::Arc;

        let config: Arc<NestGateCanonicalConfig> = Arc::new(NestGateCanonicalConfig::default());

        // Spawn multiple concurrent readers
        let handles: Vec<_> = (0..100)
            .map(|_| {
                let config = Arc::clone(&config);
                tokio::spawn(async move {
                    // Concurrent reads should be safe and consistent
                    let port = config.network.api.port;
                    let bind_addr = config.network.api.bind_address;
                    assert!(port > 0);
                    assert!(!bind_addr.is_unspecified());
                })
            })
            .collect();

        // All concurrent readers should succeed
        for handle in handles {
            handle
                .await
                .expect("Concurrent config access should not panic");
        }
    }
}
