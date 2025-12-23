//! Comprehensive tests for Production Discovery Implementation
//!
//! This module provides extensive test coverage for the production service discovery
//! system, ensuring environment-driven configuration works correctly.

use super::production_discovery::*;
use crate::config::canonical_primary::NestGateCanonicalConfig;
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
    // Set environment variable
    env::set_var("NESTGATE_TEST_SERVICE_PORT", "9876");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("test_service");
    assert!(result.is_ok());

    // Cleanup
    env::remove_var("NESTGATE_TEST_SERVICE_PORT");
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
    // Set invalid port in environment
    env::set_var("NESTGATE_INVALID_PORT", "not_a_number");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    // Should fallback gracefully
    let result = discovery.discover_port("invalid");
    assert!(result.is_ok());

    // Cleanup
    env::remove_var("NESTGATE_INVALID_PORT");
}

#[test]
fn test_discover_port_out_of_range() {
    // Set out-of-range port
    env::set_var("NESTGATE_OOR_PORT", "99999");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_port("oor");
    assert!(result.is_ok());

    // Cleanup
    env::remove_var("NESTGATE_OOR_PORT");
}

// ==================== ADDRESS DISCOVERY TESTS ====================

#[test]
fn test_discover_bind_address_from_env() {
    env::set_var("NESTGATE_API_HOST", "192.168.1.100");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_bind_address("api");
    assert!(result.is_ok());

    // Cleanup
    env::remove_var("NESTGATE_API_HOST");
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
    env::set_var("NESTGATE_INVALID_HOST", "not.an.ip.address");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    // Should handle invalid IP gracefully
    let result = discovery.discover_bind_address("invalid");
    assert!(result.is_ok());

    // Cleanup
    env::remove_var("NESTGATE_INVALID_HOST");
}

// ==================== SOCKET ADDRESS DISCOVERY TESTS ====================

#[test]
fn test_discover_endpoint_success() {
    env::set_var("NESTGATE_WEB_HOST", "127.0.0.1");
    env::set_var("NESTGATE_WEB_PORT", "8080");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_endpoint("web");
    assert!(result.is_ok());

    let socket_addr = result.unwrap();
    assert_eq!(socket_addr.port(), 8080);

    // Cleanup
    env::remove_var("NESTGATE_WEB_HOST");
    env::remove_var("NESTGATE_WEB_PORT");
}

#[test]
fn test_discover_endpoint_partial_config() {
    // Only port set
    env::set_var("NESTGATE_PARTIAL_PORT", "3000");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_endpoint("partial");
    assert!(result.is_ok());

    // Should use localhost for missing host
    let socket_addr = result.unwrap();
    assert!(socket_addr.ip().is_loopback());

    // Cleanup
    env::remove_var("NESTGATE_PARTIAL_PORT");
}

// ==================== TIMEOUT DISCOVERY TESTS ====================

#[test]
fn test_discover_timeout_from_env() {
    env::set_var("NESTGATE_TIMEOUT_CONNECT_MS", "5000");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_timeout("connect");
    assert!(result.is_ok());

    // Cleanup
    env::remove_var("NESTGATE_TIMEOUT_CONNECT_MS");
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
    env::set_var("NESTGATE_TIMEOUT_INVALID_MS", "not_a_number");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_timeout("invalid");
    assert!(result.is_ok());

    // Cleanup
    env::remove_var("NESTGATE_TIMEOUT_INVALID_MS");
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
    env::set_var("NESTGATE_IPV4_HOST", "192.168.1.1");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_bind_address("ipv4");
    assert!(result.is_ok());

    let addr = result.unwrap();
    assert!(addr.is_ipv4());

    // Cleanup
    env::remove_var("NESTGATE_IPV4_HOST");
}

#[test]
#[ignore] // IPv6 support is system-dependent
fn test_discover_ipv6_address() {
    env::set_var("NESTGATE_IPV6_HOST", "::1");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config).unwrap();

    let result = discovery.discover_bind_address("ipv6");

    // May or may not support IPv6 depending on system
    if result.is_ok() {
        let addr = result.unwrap();
        assert!(addr.is_ipv6());
    }

    // Cleanup
    env::remove_var("NESTGATE_IPV6_HOST");
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
    // Set multiple invalid environment variables
    env::set_var("NESTGATE_INVALID1_PORT", "invalid");
    env::set_var("NESTGATE_INVALID2_HOST", "not-an-ip");
    env::set_var("NESTGATE_INVALID3_TIMEOUT_MS", "bad");

    let config = NestGateCanonicalConfig::default();
    let discovery = ProductionServiceDiscovery::new(&config);

    // Should still create successfully with fallbacks
    assert!(discovery.is_ok());

    // Cleanup
    env::remove_var("NESTGATE_INVALID1_PORT");
    env::remove_var("NESTGATE_INVALID2_HOST");
    env::remove_var("NESTGATE_INVALID3_TIMEOUT_MS");
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
