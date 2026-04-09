// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Core production discovery tests: instantiation, ports, addresses, endpoints, timeouts, naming,
//! edge cases, concurrency, and IPv4/IPv6 bind behavior.

use super::production_discovery::*;
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use std::env;

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
#[ignore = "IPv6 bind discovery is system-dependent; run manually with --ignored"]
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
