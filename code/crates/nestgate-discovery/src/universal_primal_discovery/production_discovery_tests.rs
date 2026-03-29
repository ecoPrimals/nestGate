// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for Production Discovery Implementation
//!
//! This module provides extensive test coverage for the production service discovery
//! system, ensuring environment-driven configuration works correctly.

use super::production_discovery::*;
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

// ==================== BASIC INSTANTIATION TESTS ====================

#[test]
fn test_production_service_discovery_new() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config);
    assert!(discovery.is_ok());
}

#[test]
fn test_production_service_discovery_default() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config);
    assert!(discovery.is_ok());
}

#[test]
fn test_multiple_production_discovery_instances() {
    let config = NestGateCanonicalConfig::default();
    let discovery1 = ProductionServiceDiscovery::new(&config);
    let discovery2 = ProductionServiceDiscovery::new(&config);

    assert!(discovery1.is_ok());
    assert!(discovery2.is_ok());
}

// ==================== PORT DISCOVERY TESTS ====================

#[test]
fn test_discover_port_from_env() {
    let orig = env::var("NESTGATE_TEST_SERVICE_PORT").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_TEST_SERVICE_PORT", "9876");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("test_service");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_TEST_SERVICE_PORT", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_TEST_SERVICE_PORT"),
    }
    assert!(result.is_ok());
}

#[test]
fn test_discover_port_fallback() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    // Service not in environment - should use fallback
    let result = discovery.discover_port("unknown_service");
    assert!(result.is_ok());

    let port = result.unwrap();
    assert!(port >= 1024); // u16 is always <= 65535
}

#[test]
fn test_discover_port_invalid_env() {
    let orig = env::var("NESTGATE_INVALID_PORT").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_INVALID_PORT", "not_a_number");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("invalid");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_INVALID_PORT", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_INVALID_PORT"),
    }
    assert!(result.is_ok());
}

#[test]
fn test_discover_port_out_of_range() {
    let orig = env::var("NESTGATE_OOR_PORT").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_OOR_PORT", "99999");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("oor");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_OOR_PORT", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_OOR_PORT"),
    }
    assert!(result.is_ok());
}

// ==================== ADDRESS DISCOVERY TESTS ====================

#[test]
fn test_discover_bind_address_from_env() {
    let orig = env::var("NESTGATE_API_HOST").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_API_HOST", "192.168.1.100");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_bind_address("api");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_API_HOST", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_API_HOST"),
    }
    assert!(result.is_ok());
}

#[test]
fn test_discover_bind_address_localhost_fallback() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_bind_address("unknown");
    assert!(result.is_ok());

    let addr = result.unwrap();
    // Should fallback to localhost
    assert!(addr.is_loopback());
}

#[test]
fn test_discover_bind_address_invalid_ip() {
    let orig = env::var("NESTGATE_INVALID_HOST").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_INVALID_HOST", "not.an.ip.address");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_bind_address("invalid");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_INVALID_HOST", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_INVALID_HOST"),
    }
    assert!(result.is_ok());
}

// ==================== SOCKET ADDRESS DISCOVERY TESTS ====================

#[test]
fn test_discover_endpoint_success() {
    let orig_host = env::var("NESTGATE_WEB_HOST").ok();
    let orig_port = env::var("NESTGATE_WEB_PORT").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_WEB_HOST", "127.0.0.1");
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_WEB_PORT", "8080");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_endpoint("web");
    match orig_host {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_WEB_HOST", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_WEB_HOST"),
    }
    match orig_port {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_WEB_PORT", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_WEB_PORT"),
    }
    assert!(result.is_ok());
    let socket_addr = result.unwrap();
    assert_eq!(socket_addr.port(), 8080);
}

#[test]
fn test_discover_endpoint_partial_config() {
    let orig = env::var("NESTGATE_PARTIAL_PORT").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_PARTIAL_PORT", "3000");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_endpoint("partial");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_PARTIAL_PORT", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_PARTIAL_PORT"),
    }
    assert!(result.is_ok());
    let socket_addr = result.unwrap();
    assert!(socket_addr.ip().is_loopback());
}

// ==================== TIMEOUT DISCOVERY TESTS ====================

#[test]
fn test_discover_timeout_from_env() {
    let orig = env::var("NESTGATE_TIMEOUT_CONNECT_MS").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_TIMEOUT_CONNECT_MS", "5000");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_timeout("connect");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_TIMEOUT_CONNECT_MS", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_TIMEOUT_CONNECT_MS"),
    }
    assert!(result.is_ok());
}

#[test]
fn test_discover_timeout_default() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_timeout("unknown_timeout");
    assert!(result.is_ok());

    let timeout = result.unwrap();
    assert!(timeout.as_secs() > 0);
}

#[test]
fn test_discover_timeout_invalid() {
    let orig = env::var("NESTGATE_TIMEOUT_INVALID_MS").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_TIMEOUT_INVALID_MS", "not_a_number");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_timeout("invalid");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_TIMEOUT_INVALID_MS", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_TIMEOUT_INVALID_MS"),
    }
    assert!(result.is_ok());
}

// ==================== SERVICE NAME VARIATIONS TESTS ====================

#[test]
fn test_service_name_with_hyphens() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("api-gateway");
    assert!(result.is_ok());
}

#[test]
fn test_service_name_with_underscores() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("web_server");
    assert!(result.is_ok());
}

#[test]
fn test_service_name_uppercase() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("API_SERVICE");
    assert!(result.is_ok());
}

#[test]
fn test_service_name_mixed_case() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("WebServer");
    assert!(result.is_ok());
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_empty_service_name() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    // Should handle empty name gracefully
    let result = discovery.discover_port("");
    assert!(result.is_ok());
}

#[test]
fn test_very_long_service_name() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let long_name = "a".repeat(1000);
    let result = discovery.discover_port(&long_name);
    assert!(result.is_ok());
}

#[test]
fn test_service_name_with_special_chars() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("service@#$%");
    assert!(result.is_ok());
}

// ==================== CONCURRENT ACCESS TESTS ====================

#[tokio::test]
async fn test_concurrent_port_discoveries() {
    let config = NestGateCanonicalConfig::default();
    let discovery = std::sync::Arc::new(ProductionServiceDiscovery::new(&config).unwrap());

    let discovery1 = discovery.clone();
    let discovery2 = discovery.clone();
    let discovery3 = discovery.clone();

    let handle1 = tokio::spawn(async move { discovery1.discover_port("service1") });

    let handle2 = tokio::spawn(async move { discovery2.discover_port("service2") });

    let handle3 = tokio::spawn(async move { discovery3.discover_port("service3") });

    let results = tokio::try_join!(handle1, handle2, handle3);
    assert!(results.is_ok());
}

#[tokio::test]
async fn test_concurrent_address_discoveries() {
    let config = NestGateCanonicalConfig::default();
    let discovery = std::sync::Arc::new(ProductionServiceDiscovery::new(&config).unwrap());

    let mut handles = vec![];

    for i in 0..10 {
        let disc = discovery.clone();
        let handle =
            tokio::spawn(async move { disc.discover_bind_address(&format!("service{}", i)) });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
    }
}

// ==================== IPv4 vs IPv6 TESTS ====================

#[test]
fn test_discover_ipv4_address() {
    let orig = env::var("NESTGATE_IPV4_HOST").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_IPV4_HOST", "192.168.1.1");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_bind_address("ipv4");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_IPV4_HOST", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_IPV4_HOST"),
    }
    assert!(result.is_ok());
    let addr = result.unwrap();
    assert!(addr.is_ipv4());
}

#[test]
#[ignore] // IPv6 support is system-dependent
fn test_discover_ipv6_address() {
    let orig = env::var("NESTGATE_IPV6_HOST").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_IPV6_HOST", "::1");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_bind_address("ipv6");
    match orig {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_IPV6_HOST", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_IPV6_HOST"),
    }
    if result.is_ok() {
        let addr = result.unwrap();
        assert!(addr.is_ipv6());
    }
}

// ==================== CONFIGURATION TESTS ====================

#[test]
fn test_production_config_from_env() {
    let config = NestGateCanonicalConfig::from_environment().unwrap_or_default();
    // Should create valid config
    assert!(ProductionServiceDiscovery::new(&config).is_ok());
}

#[test]
fn test_production_config_default() {
    let config = NestGateCanonicalConfig::default();
    // Should create valid config
    assert!(ProductionServiceDiscovery::new(&config).is_ok());
}

// ==================== ERROR HANDLING TESTS ====================

#[test]
fn test_graceful_error_handling() {
    let orig1 = env::var("NESTGATE_INVALID1_PORT").ok();
    let orig2 = env::var("NESTGATE_INVALID2_HOST").ok();
    let orig3 = env::var("NESTGATE_INVALID3_TIMEOUT_MS").ok();
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_INVALID1_PORT", "invalid");
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_INVALID2_HOST", "not-an-ip");
    // SAFETY: single-threaded test context.
    nestgate_platform::env_process::set_var("NESTGATE_INVALID3_TIMEOUT_MS", "bad");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config);

    match orig1 {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_INVALID1_PORT", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_INVALID1_PORT"),
    }
    match orig2 {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_INVALID2_HOST", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_INVALID2_HOST"),
    }
    match orig3 {
        Some(v) => nestgate_platform::env_process::set_var("NESTGATE_INVALID3_TIMEOUT_MS", v),
        None => nestgate_platform::env_process::remove_var("NESTGATE_INVALID3_TIMEOUT_MS"),
    }
    assert!(discovery.is_ok());
}

// ==================== MEMORY SAFETY TESTS ====================

#[test]
fn test_discovery_memory_safety() {
    let config = NestGateCanonicalConfig::default();
    let mut discoveries = vec![];

    for _ in 0..100 {
        discoveries.push(ProductionServiceDiscovery::new(&config).unwrap());
    }

    // All should be valid
    assert_eq!(discoveries.len(), 100);
}

#[test]
fn test_discovery_drop_safety() {
    let config = NestGateCanonicalConfig::default();

    for _ in 0..1000 {
        let discovery = ProductionServiceDiscovery::new(&config).unwrap();
        drop(discovery);
    }

    // Should not leak memory
}

// ==================== INLINE TESTS (moved from production_discovery.rs) ====================

#[test]
fn test_service_discovery_defaults() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    assert!(!discovery.all_services().is_empty());
}

#[test]
fn test_port_discovery_from_config() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let api_port = discovery
        .discover_port("api")
        .expect("Failed to discover API port");
    assert_eq!(api_port, config.network.api.port);
}

#[test]
fn test_port_discovery_unknown_service() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let port = discovery
        .discover_port("unknown_service_xyz")
        .expect("Failed to discover port");
    assert_eq!(port, discovery.config().defaults.default_port);
}

#[test]
fn test_bind_address_discovery() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let bind_addr = discovery
        .discover_bind_address("api")
        .expect("Failed to discover bind address");
    assert!(matches!(bind_addr, IpAddr::V4(_) | IpAddr::V6(_)));
}

#[test]
fn test_bind_address_unknown_service_fallback() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let bind_addr = discovery
        .discover_bind_address("nonexistent")
        .expect("Should fall back to default");
    assert_eq!(bind_addr, discovery.config().defaults.default_bind);
}

#[test]
fn test_endpoint_discovery_from_config() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let endpoint = discovery
        .discover_endpoint("api")
        .expect("Failed to discover endpoint");
    assert_eq!(endpoint.port(), config.network.api.port);
}

#[test]
fn test_endpoint_discovery_fallback() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let endpoint = discovery
        .discover_endpoint("missing_service")
        .expect("Should provide fallback endpoint");
    assert!(endpoint.port() > 0);
}

#[test]
fn test_limit_discovery_defaults() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let connections_limit = discovery
        .discover_limit("connections")
        .expect("Failed to discover limit");
    assert!(connections_limit > 0);
}

#[test]
fn test_limit_discovery_various_types() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let conn_limit = discovery
        .discover_limit("connections")
        .expect("connections");
    assert_eq!(conn_limit, 1000);

    let rps_limit = discovery
        .discover_limit("requests_per_second")
        .expect("requests_per_second");
    assert_eq!(rps_limit, 100);

    let mem_limit = discovery.discover_limit("memory_mb").expect("memory_mb");
    assert_eq!(mem_limit, 512);

    let disk_limit = discovery.discover_limit("disk_mb").expect("disk_mb");
    assert_eq!(disk_limit, 1024);
}

#[test]
fn test_limit_discovery_custom_type_fallback() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let limit = discovery
        .discover_limit("custom_resource_type")
        .expect("custom type");
    assert_eq!(limit, discovery.config().defaults.default_limit);
}

#[test]
fn test_timeout_discovery_defaults() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let timeout = discovery
        .discover_timeout("connect")
        .expect("Failed to discover timeout");
    assert!(timeout.as_secs() > 0);
}

#[test]
fn test_timeout_discovery_various_operations() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let connect_timeout = discovery.discover_timeout("connect").expect("connect");
    assert_eq!(connect_timeout, Duration::from_secs(10));

    let request_timeout = discovery.discover_timeout("request").expect("request");
    assert_eq!(request_timeout, Duration::from_secs(30));

    let health_timeout = discovery
        .discover_timeout("health_check")
        .expect("health_check");
    assert_eq!(health_timeout, Duration::from_secs(5));

    let discovery_timeout = discovery.discover_timeout("discovery").expect("discovery");
    assert_eq!(discovery_timeout, Duration::from_secs(15));
}

#[test]
fn test_timeout_discovery_custom_operation_fallback() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let timeout = discovery
        .discover_timeout("custom_operation")
        .expect("custom operation");
    assert_eq!(timeout, discovery.config().defaults.default_timeout);
}

#[test]
fn test_all_services_accessor() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let services = discovery.all_services();
    assert!(!services.is_empty());
    assert!(services.contains_key("api"));
}

#[test]
fn test_config_accessor() {
    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).expect("Failed to create discovery");

    let disc_config = discovery.config();
    assert!(!disc_config.services.is_empty());
}

#[test]
fn test_standalone_bind_address_discovery() {
    let config = NestGateCanonicalConfig::default();

    let bind_addr =
        discover_bind_address_standalone(&config, "api").expect("Failed to discover bind address");
    assert!(matches!(bind_addr, IpAddr::V4(_) | IpAddr::V6(_)));
}

#[test]
fn test_standalone_endpoint_discovery() {
    let config = NestGateCanonicalConfig::default();

    let endpoint =
        discover_endpoint_standalone(&config, "api").expect("Failed to discover endpoint");
    assert_eq!(endpoint.port(), config.network.api.port);
}

#[test]
fn test_standalone_port_discovery() {
    let config = NestGateCanonicalConfig::default();

    let port = discover_port_standalone(&config, "api").expect("Failed to discover port");
    assert_eq!(port, config.network.api.port);
}

#[test]
fn test_standalone_limit_discovery() {
    let config = NestGateCanonicalConfig::default();

    let limit =
        discover_limit_standalone(&config, "connections").expect("Failed to discover limit");
    assert!(limit > 0);
}

#[test]
fn test_standalone_timeout_discovery() {
    let config = NestGateCanonicalConfig::default();

    let timeout =
        discover_timeout_standalone(&config, "connect").expect("Failed to discover timeout");
    assert!(timeout.as_secs() > 0);
}

#[test]
fn test_create_production_discovery_function() {
    let config = NestGateCanonicalConfig::default();

    let discovery = create_production_discovery(&config).expect("Failed to create discovery");
    assert!(!discovery.all_services().is_empty());
}

#[test]
fn test_discovery_defaults_implementation() {
    let defaults = DiscoveryDefaults::default();

    assert_eq!(
        defaults.default_host,
        nestgate_config::constants::canonical_defaults::network::LOCALHOST
    );
    assert!(matches!(
        defaults.default_bind,
        IpAddr::V4(_) | IpAddr::V6(_)
    ));
    assert_eq!(
        defaults.default_port,
        nestgate_config::constants::canonical_defaults::network::DEFAULT_API_PORT
    );
    assert_eq!(defaults.default_timeout, Duration::from_secs(30));
    assert_eq!(defaults.default_limit, 100);
}

#[test]
fn test_service_endpoint_structure() {
    let endpoint = ServiceEndpoint {
        name: "test_service".to_string(),
        host: "localhost".to_string(),
        port: nestgate_config::constants::network_hardcoded::get_api_port(),
        bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
    };

    assert_eq!(endpoint.name, "test_service");
    assert_eq!(endpoint.host, "localhost");
    assert_eq!(endpoint.port, 8080);
    assert!(matches!(endpoint.bind_address, IpAddr::V4(_)));
}

#[test]
fn test_default_port_for_various_services() {
    let api_port = ServiceDiscoveryConfig::default_port_for_service("api");
    assert_eq!(
        api_port,
        nestgate_config::constants::canonical_defaults::network::DEFAULT_API_PORT
    );

    let metrics_port = ServiceDiscoveryConfig::default_port_for_service("metrics");
    assert_eq!(
        metrics_port,
        nestgate_config::constants::canonical_defaults::network::DEFAULT_METRICS_PORT
    );

    let health_port = ServiceDiscoveryConfig::default_port_for_service("health");
    assert_eq!(
        health_port,
        nestgate_config::constants::canonical_defaults::network::DEFAULT_INTERNAL_PORT
    );

    let unknown_port = ServiceDiscoveryConfig::default_port_for_service("unknown");
    assert_eq!(
        unknown_port,
        nestgate_config::constants::canonical_defaults::network::DEFAULT_API_PORT
    );
}

#[test]
fn test_default_bind_for_various_services() {
    let api_bind = ServiceDiscoveryConfig::default_bind_for_service("api");
    assert_eq!(api_bind, IpAddr::V4(Ipv4Addr::UNSPECIFIED));

    let internal_bind = ServiceDiscoveryConfig::default_bind_for_service("health");
    assert!(matches!(internal_bind, IpAddr::V4(_)));
}
