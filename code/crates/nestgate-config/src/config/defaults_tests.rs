// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for configuration defaults
//!
//! This module provides extensive test coverage for all default configuration
//! implementations to ensure they meet expected standards.

#[cfg(test)]
mod config_defaults_tests {
    // Tests exercise deprecated `InfantDiscoveryConfig` for backward-compat coverage until
    // callers migrate to `CanonicalNetworkConfig`.
    #![expect(deprecated)]
    #![expect(clippy::panic)] // test assertions via `let ... else { panic!(...) }`

    use crate::config::*;

    // ==================== InfantDiscoveryConfig Tests ====================

    #[test]
    fn test_infant_discovery_config_default() {
        let config = InfantDiscoveryConfig::default();

        assert!(
            config.enabled,
            "Infant discovery should be enabled by default"
        );
        assert_eq!(
            config.discovery_timeout_seconds, 30,
            "Default timeout should be 30 seconds"
        );
        assert_eq!(
            config.capability_cache_ttl_seconds, 300,
            "Default cache TTL should be 300 seconds"
        );
        assert!(
            config.fallback_to_environment,
            "Should fallback to environment by default"
        );
    }

    #[test]
    fn test_infant_discovery_config_custom_values() {
        let config = InfantDiscoveryConfig {
            enabled: false,
            discovery_timeout_seconds: 60,
            capability_cache_ttl_seconds: 600,
            fallback_to_environment: false,
        };

        assert!(!config.enabled);
        assert_eq!(config.discovery_timeout_seconds, 60);
        assert_eq!(config.capability_cache_ttl_seconds, 600);
        assert!(!config.fallback_to_environment);
    }

    #[test]
    fn test_infant_discovery_config_serialization() {
        let config = InfantDiscoveryConfig::default();

        // Test serialization
        let Ok(serialized) = serde_json::to_string(&config) else {
            panic!("Should serialize successfully");
        };
        assert!(serialized.contains("enabled"));
        assert!(serialized.contains("discovery_timeout_seconds"));

        // Test deserialization
        let Ok(deserialized): Result<InfantDiscoveryConfig, _> = serde_json::from_str(&serialized)
        else {
            panic!("Should deserialize successfully");
        };
        assert_eq!(deserialized.enabled, config.enabled);
        assert_eq!(
            deserialized.discovery_timeout_seconds,
            config.discovery_timeout_seconds
        );
    }

    #[test]
    fn test_infant_discovery_config_clone() {
        let config = InfantDiscoveryConfig::default();
        let cloned = config.clone();

        assert_eq!(cloned.enabled, config.enabled);
        assert_eq!(
            cloned.discovery_timeout_seconds,
            config.discovery_timeout_seconds
        );
    }

    #[test]
    fn test_infant_discovery_config_debug() {
        let config = InfantDiscoveryConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("InfantDiscoveryConfig"));
        assert!(debug_str.contains("enabled"));
    }

    // ==================== Timeout and Duration Tests ====================

    #[test]
    fn test_discovery_timeout_range() {
        let config = InfantDiscoveryConfig {
            discovery_timeout_seconds: 1,
            ..Default::default()
        };
        assert_eq!(config.discovery_timeout_seconds, 1);

        let config = InfantDiscoveryConfig {
            discovery_timeout_seconds: 300,
            ..Default::default()
        };
        assert_eq!(config.discovery_timeout_seconds, 300);
    }

    #[test]
    fn test_cache_ttl_zero_is_valid() {
        // Zero TTL means no caching, which is valid
        let config = InfantDiscoveryConfig {
            capability_cache_ttl_seconds: 0,
            ..Default::default()
        };
        assert_eq!(config.capability_cache_ttl_seconds, 0);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_infant_discovery_disabled_with_timeout() {
        // Should be able to disable discovery even with timeout set
        let config = InfantDiscoveryConfig {
            enabled: false,
            discovery_timeout_seconds: 30,
            ..Default::default()
        };
        assert!(!config.enabled);
        assert_eq!(config.discovery_timeout_seconds, 30);
    }

    #[test]
    fn test_infant_discovery_no_fallback() {
        // Should be able to disable fallback
        let config = InfantDiscoveryConfig {
            fallback_to_environment: false,
            ..Default::default()
        };
        assert!(!config.fallback_to_environment);
    }

    // ==================== JSON Roundtrip Tests ====================

    #[test]
    fn test_infant_discovery_json_roundtrip() {
        let original = InfantDiscoveryConfig {
            enabled: true,
            discovery_timeout_seconds: 45,
            capability_cache_ttl_seconds: 500,
            fallback_to_environment: true,
        };

        let Ok(json) = serde_json::to_string(&original) else {
            panic!("Serialization should succeed");
        };
        let Ok(recovered): Result<InfantDiscoveryConfig, _> = serde_json::from_str(&json) else {
            panic!("Deserialization should succeed");
        };

        assert_eq!(recovered.enabled, original.enabled);
        assert_eq!(
            recovered.discovery_timeout_seconds,
            original.discovery_timeout_seconds
        );
        assert_eq!(
            recovered.capability_cache_ttl_seconds,
            original.capability_cache_ttl_seconds
        );
        assert_eq!(
            recovered.fallback_to_environment,
            original.fallback_to_environment
        );
    }

    #[test]
    fn test_infant_discovery_json_with_extra_fields() {
        // Should handle JSON with extra fields gracefully
        let json = r#"{
            "enabled": true,
            "discovery_timeout_seconds": 30,
            "capability_cache_ttl_seconds": 300,
            "fallback_to_environment": true,
            "unknown_field": "should_be_ignored"
        }"#;

        let Ok(config): Result<InfantDiscoveryConfig, _> = serde_json::from_str(json) else {
            panic!("Should deserialize with extra fields");
        };
        assert!(config.enabled);
    }

    // ==================== Config Comparison Tests ====================

    #[test]
    fn test_infant_discovery_configs_with_same_values_are_equal() {
        let config1 = InfantDiscoveryConfig::default();
        let config2 = InfantDiscoveryConfig::default();

        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(
            config1.discovery_timeout_seconds,
            config2.discovery_timeout_seconds
        );
    }

    #[test]
    fn test_infant_discovery_configs_with_different_values() {
        let config1 = InfantDiscoveryConfig::default();
        let config2 = InfantDiscoveryConfig {
            enabled: false,
            ..Default::default()
        };

        assert_ne!(config1.enabled, config2.enabled);
    }
}
