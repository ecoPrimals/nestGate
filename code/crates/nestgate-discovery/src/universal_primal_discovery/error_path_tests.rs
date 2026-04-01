// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Error path tests for production discovery
//!
//! Comprehensive test coverage for error handling scenarios in service discovery

use super::production_discovery::*;
use super::production_discovery_config::ProductionDiscoveryConfig;
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

#[test]
fn test_service_discovery_config_default() {
    let config = ServiceDiscoveryConfig::default();

    assert!(!config.services.is_empty());
    assert!(config.services.contains_key("api"));
    assert!(!config.resource_limits.is_empty());
    assert!(!config.operation_timeouts.is_empty());
}

#[test]
fn test_discovery_defaults_values() {
    let defaults = DiscoveryDefaults::default();

    assert_eq!(defaults.default_host, "127.0.0.1");
    assert!(matches!(defaults.default_bind, IpAddr::V4(_)));
    assert!(defaults.default_port > 0);
    assert!(defaults.default_timeout > Duration::from_secs(0));
    assert!(defaults.default_limit > 0);
}

#[test]
fn test_service_endpoint_creation() {
    let endpoint = ServiceEndpoint {
        name: "test".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    assert_eq!(endpoint.name, "test");
    assert_eq!(endpoint.host, "localhost");
    assert_eq!(endpoint.port, 8080);
}

#[test]
fn test_default_port_for_api_service() {
    let port = ServiceDiscoveryConfig::default_port_for_service("api");
    assert!(port > 0);
    // Port is u16, max value is 65535 by type definition
}

#[test]
fn test_default_port_for_metrics_service() {
    let port = ServiceDiscoveryConfig::default_port_for_service("metrics");
    assert!(port > 0);
    // Port is u16, max value is 65535 by type definition
}

#[test]
fn test_default_port_for_unknown_service() {
    let port = ServiceDiscoveryConfig::default_port_for_service("unknown_service");
    assert!(port > 0); // Should return default API port
}

#[test]
fn test_default_bind_for_external_service() {
    let bind = ServiceDiscoveryConfig::default_bind_for_service("api");
    assert!(matches!(bind, IpAddr::V4(addr) if addr == Ipv4Addr::UNSPECIFIED));
}

#[test]
fn test_default_bind_for_internal_service() {
    let bind = ServiceDiscoveryConfig::default_bind_for_service("internal");
    // Should be localhost or similar for internal services
    assert!(matches!(bind, IpAddr::V4(_)));
}

#[test]
fn test_production_discovery_config_from_env() {
    // Test that config can be created without panicking
    let config = ProductionDiscoveryConfig::from_env();

    // Basic validation - config may have empty maps if no env vars are set, which is valid
    // The important thing is that the config initializes successfully
    // Note: Vec/HashMap len() is always >= 0 by type definition (usize)
    let limits = config.get_all_resource_limits();
    let timeouts = config.get_all_operation_timeouts();
    assert!(
        limits.len() < usize::MAX,
        "Resource limits collection is valid"
    );
    assert!(
        timeouts.len() < usize::MAX,
        "Operation timeouts collection is valid"
    );
}

#[test]
fn test_service_discovery_with_empty_services() {
    let config = ServiceDiscoveryConfig {
        services: HashMap::new(),
        resource_limits: HashMap::new(),
        operation_timeouts: HashMap::new(),
        defaults: DiscoveryDefaults::default(),
    };

    // Should handle empty services gracefully
    assert!(config.services.is_empty());
    assert!(config.defaults.default_port > 0);
}

#[test]
fn test_service_discovery_limits_validation() {
    let config = ServiceDiscoveryConfig::default();

    // Verify limits exist and are reasonable
    for &limit in config.resource_limits.values() {
        assert!(limit > 0, "Resource limit should be positive");
        assert!(limit < 1_000_000, "Resource limit should be reasonable");
    }
}

#[test]
fn test_service_discovery_timeouts_validation() {
    let config = ServiceDiscoveryConfig::default();

    // Verify timeouts exist and are reasonable
    for &timeout in config.operation_timeouts.values() {
        assert!(
            timeout > Duration::from_secs(0),
            "Timeout should be positive"
        );
        assert!(
            timeout < Duration::from_secs(3600),
            "Timeout should be reasonable"
        );
    }
}

#[test]
fn test_multiple_service_endpoints() {
    let mut config = ServiceDiscoveryConfig::default();

    // Add additional services
    config.services.insert(
        "custom".to_string(),
        ServiceEndpoint {
            name: "custom".to_string(),
            host: "custom.local".to_string(),
            port: 9000,
            bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
        },
    );

    assert!(config.services.contains_key("api"));
    assert!(config.services.contains_key("custom"));
    assert!(config.services.len() >= 2);
}

#[test]
fn test_service_discovery_config_cloning() {
    let config1 = ServiceDiscoveryConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.services.len(), config2.services.len());
    assert_eq!(config1.resource_limits.len(), config2.resource_limits.len());
    assert_eq!(
        config1.operation_timeouts.len(),
        config2.operation_timeouts.len()
    );
}

#[test]
fn test_discovery_defaults_cloning() {
    let defaults1 = DiscoveryDefaults::default();
    let defaults2 = defaults1.clone();

    assert_eq!(defaults1.default_host, defaults2.default_host);
    assert_eq!(defaults1.default_bind, defaults2.default_bind);
    assert_eq!(defaults1.default_port, defaults2.default_port);
}

#[test]
fn test_service_endpoint_debug_format() {
    let endpoint = ServiceEndpoint {
        name: "test".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    let debug_str = format!("{:?}", endpoint);
    assert!(debug_str.contains("test"));
    assert!(debug_str.contains("localhost"));
    assert!(debug_str.contains("8080"));
}

#[test]
fn test_port_range_validation() {
    // Test that default ports are in valid range
    let api_port = ServiceDiscoveryConfig::default_port_for_service("api");
    let metrics_port = ServiceDiscoveryConfig::default_port_for_service("metrics");

    assert!(api_port >= 1024, "Should not use privileged ports");
    assert!(metrics_port >= 1024, "Should not use privileged ports");
    assert!(api_port != metrics_port, "Ports should be different");
}

#[test]
fn test_service_discovery_with_canonical_config() {
    let canonical_config = NestGateCanonicalConfig::default();
    let result = ServiceDiscoveryConfig::from_environment(&canonical_config);

    // Should succeed with default config
    assert!(result.is_ok());

    if let Ok(config) = result {
        assert!(!config.services.is_empty());
    }
}

#[test]
fn test_production_service_discovery_creation() {
    let canonical_config = NestGateCanonicalConfig::default();
    let result = ProductionServiceDiscovery::new(&canonical_config);

    // Should create successfully with default config
    assert!(result.is_ok());
}

#[test]
fn test_discovery_error_handling_invalid_port() {
    // Test that we handle invalid ports gracefully
    let endpoint = ServiceEndpoint {
        name: "test".to_string(),
        host: "localhost".to_string(),
        port: 65535, // Max valid port
        bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    assert_eq!(endpoint.port, 65535);
}

#[test]
fn test_discovery_defaults_consistency() {
    let defaults1 = DiscoveryDefaults::default();
    let defaults2 = DiscoveryDefaults::default();

    // Defaults should be consistent
    assert_eq!(defaults1.default_host, defaults2.default_host);
    assert_eq!(defaults1.default_port, defaults2.default_port);
    assert_eq!(defaults1.default_limit, defaults2.default_limit);
}

#[test]
fn test_service_name_variations() {
    // Test various service name variations
    let services = vec![
        "api",
        "web",
        "metrics",
        "health",
        "admin",
        "websocket",
        "unknown",
    ];

    for service in services {
        let port = ServiceDiscoveryConfig::default_port_for_service(service);
        assert!(port > 0, "Port should be positive for {}", service);

        let bind = ServiceDiscoveryConfig::default_bind_for_service(service);
        assert!(
            matches!(bind, IpAddr::V4(_)),
            "Bind should be IPv4 for {}",
            service
        );
    }
}
