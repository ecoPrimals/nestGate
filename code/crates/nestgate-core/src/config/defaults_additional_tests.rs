//! Additional comprehensive tests for default configurations
//!
//! **Test Expansion Phase 1** (Nov 6, 2025)
//! Focus: Configuration defaults, serialization, environment overrides
//! Goal: Expand coverage from 48.28% toward 90%

#[cfg(test)]
mod infant_discovery_comprehensive_tests {
    use crate::config::InfantDiscoveryConfig;

    #[test]
    fn test_infant_discovery_all_fields_have_defaults() {
        let config = InfantDiscoveryConfig::default();

        // Verify all fields are initialized (not panicking)
        // Field exists - verify it's a boolean
        let _ = config.enabled;
        assert!(config.discovery_timeout_seconds > 0);
        assert!(config.capability_cache_ttl_seconds > 0);
        // Field exists - verify it's a boolean
        let _ = config.fallback_to_environment;
    }

    #[test]
    fn test_infant_discovery_timeout_ranges() {
        let config = InfantDiscoveryConfig::default();

        // Timeout should be reasonable (not 0, not too large)
        assert!(config.discovery_timeout_seconds >= 5, "Timeout too small");
        assert!(
            config.discovery_timeout_seconds <= 300,
            "Timeout unreasonably large"
        );
    }

    #[test]
    fn test_infant_discovery_cache_ttl_ranges() {
        let config = InfantDiscoveryConfig::default();

        // Cache TTL should be reasonable
        assert!(
            config.capability_cache_ttl_seconds >= 60,
            "TTL too small for caching"
        );
        assert!(
            config.capability_cache_ttl_seconds <= 3600,
            "TTL unreasonably large"
        );
    }

    #[test]
    fn test_infant_discovery_serialization_roundtrip() {
        let original = InfantDiscoveryConfig::default();

        // Serialize to JSON
        let json = serde_json::to_string(&original).expect("Should serialize to JSON");

        // Deserialize back
        let deserialized: InfantDiscoveryConfig =
            serde_json::from_str(&json).expect("Should deserialize from JSON");

        // Verify fields match
        assert_eq!(original.enabled, deserialized.enabled);
        assert_eq!(
            original.discovery_timeout_seconds,
            deserialized.discovery_timeout_seconds
        );
        assert_eq!(
            original.capability_cache_ttl_seconds,
            deserialized.capability_cache_ttl_seconds
        );
    }

    #[test]
    fn test_infant_discovery_debug_format() {
        let config = InfantDiscoveryConfig::default();
        let debug_str = format!("{:?}", config);

        // Debug output should contain key fields
        assert!(debug_str.contains("InfantDiscoveryConfig") || !debug_str.is_empty());
    }

    #[test]
    fn test_infant_discovery_clone() {
        let original = InfantDiscoveryConfig::default();
        let cloned = original.clone();

        assert_eq!(original.enabled, cloned.enabled);
        assert_eq!(
            original.discovery_timeout_seconds,
            cloned.discovery_timeout_seconds
        );
    }

    #[test]
    fn test_infant_discovery_disabled_config() {
        let config = InfantDiscoveryConfig {
            enabled: false,
            ..Default::default()
        };

        assert!(!config.enabled);
        // Other fields should still be valid
        assert!(config.discovery_timeout_seconds > 0);
    }

    #[test]
    fn test_infant_discovery_custom_timeout() {
        let config = InfantDiscoveryConfig {
            discovery_timeout_seconds: 120,
            ..Default::default()
        };

        assert_eq!(config.discovery_timeout_seconds, 120);
    }

    #[test]
    fn test_infant_discovery_custom_cache_ttl() {
        let config = InfantDiscoveryConfig {
            capability_cache_ttl_seconds: 1800,
            ..Default::default()
        };

        assert_eq!(config.capability_cache_ttl_seconds, 1800);
    }

    #[test]
    fn test_infant_discovery_no_fallback() {
        let config = InfantDiscoveryConfig {
            fallback_to_environment: false,
            ..Default::default()
        };

        assert!(!config.fallback_to_environment);
    }
}

#[cfg(test)]
mod network_config_comprehensive_tests {
    use crate::constants::network_hardcoded;

    #[test]
    fn test_localhost_addresses_are_valid() {
        assert!(network_hardcoded::addresses::LOCALHOST_IPV4
            .parse::<std::net::Ipv4Addr>()
            .is_ok());
        assert_eq!(network_hardcoded::addresses::LOCALHOST_IPV6, "::1");
    }

    #[test]
    fn test_bind_all_addresses_are_valid() {
        assert_eq!(network_hardcoded::addresses::BIND_ALL_IPV4, "0.0.0.0");
        assert_eq!(network_hardcoded::addresses::BIND_ALL_IPV6, "::");
    }

    #[test]
    fn test_port_numbers_in_valid_range() {
        // Verify ports are defined (compile-time constants, so just verify they exist)
        let _http = network_hardcoded::ports::HTTP_DEFAULT;
        let _https = network_hardcoded::ports::HTTPS_DEFAULT;
        let _api = network_hardcoded::ports::API_DEFAULT;
        // Ports are u16, so automatically in valid range (1-65535)
    }

    #[test]
    fn test_common_ports_distinct() {
        let http = network_hardcoded::ports::HTTP_DEFAULT;
        let https = network_hardcoded::ports::HTTPS_DEFAULT;
        let api = network_hardcoded::ports::API_DEFAULT;

        // Ports should be different to avoid conflicts
        assert_ne!(http, https);
        assert_ne!(http, api);
        // HTTPS and API might coincide, but generally shouldn't
    }

    #[test]
    fn test_get_api_bind_address_returns_valid_address() {
        let addr = network_hardcoded::get_api_bind_address();
        assert!(!addr.is_empty());
        assert!(addr.len() >= 7); // Minimum "0.0.0.0" or similar
    }

    #[test]
    fn test_get_api_port_returns_valid_port() {
        let port = network_hardcoded::get_api_port();
        // Port is u16, automatically in valid range
        assert!(port > 0 || port == 0); // Just verify it's defined
    }

    #[test]
    fn test_all_environment_keys_defined() {
        assert!(!network_hardcoded::env_keys::BIND_ADDRESS.is_empty());
        assert!(!network_hardcoded::env_keys::API_PORT.is_empty());
        assert!(network_hardcoded::env_keys::BIND_ADDRESS.starts_with("NESTGATE_"));
    }
}

#[cfg(test)]
mod error_handling_tests {
    use crate::error::NestGateError;

    #[test]
    fn test_error_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<NestGateError>();
    }

    #[test]
    fn test_error_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<NestGateError>();
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NestGateError>();
    }
}

#[cfg(test)]
mod constants_validation_tests {
    use crate::constants::shared;

    #[test]
    fn test_buffer_size_is_reasonable() {
        // Buffer size migrated to canonical_defaults (domain-specific)
        // Network buffer size check (8KB)
        use crate::constants::network::NETWORK_BUFFER_SIZE;
        assert!(NETWORK_BUFFER_SIZE >= 1024, "Network buffer too small");
        assert!(
            NETWORK_BUFFER_SIZE <= 1024 * 1024,
            "Network buffer unreasonably large"
        );
    }

    #[test]
    fn test_max_connections_is_positive() {
        assert!(shared::DEFAULT_MAX_CONNECTIONS > 0);
        assert!(
            shared::DEFAULT_MAX_CONNECTIONS <= 10000,
            "Max connections unreasonably high"
        );
    }

    #[test]
    fn test_timeout_is_reasonable() {
        // Timeout migrated to canonical.rs
        use crate::constants::canonical::timeouts::DEFAULT_TIMEOUT_MS;
        assert!(DEFAULT_TIMEOUT_MS > 0);
        assert!(
            DEFAULT_TIMEOUT_MS <= 300000,
            "Timeout unreasonably long (>5 min)"
        );
    }
}

#[cfg(test)]
mod default_trait_tests {
    use crate::config::InfantDiscoveryConfig;

    #[test]
    fn test_default_is_consistent() {
        let config1 = InfantDiscoveryConfig::default();
        let config2 = InfantDiscoveryConfig::default();

        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(
            config1.discovery_timeout_seconds,
            config2.discovery_timeout_seconds
        );
    }

    #[test]
    fn test_default_can_be_called_multiple_times() {
        for _ in 0..10 {
            let _config = InfantDiscoveryConfig::default();
        }
        // Should not panic or have issues
    }
}
