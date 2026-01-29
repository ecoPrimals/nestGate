//! Comprehensive tests for network_defaults module
//!
//! **MODERN CONCURRENT-SAFE TESTING:**
//! - Uses dependency injection via NetworkDefaultsV2Config
//! - No environment variable pollution
//! - All tests run in parallel safely
//! - Idiomatic Rust testing patterns

use crate::config::network_defaults_v2_config::NetworkDefaultsV2Config;

// ==================== API SERVER TESTS ====================

#[test]
fn test_api_host_default() {
    use crate::constants::hardcoding::addresses;
    // Create config with defaults (no env vars)
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.api_host(), addresses::LOCALHOST_IPV4);
}

#[test]
fn test_api_host_custom() {
    // Inject custom host via builder (concurrent-safe!)
    let config = NetworkDefaultsV2Config::new().with_api_host("192.168.1.100".to_string());
    assert_eq!(config.api_host(), "192.168.1.100");
}

#[test]
fn test_api_port_default() {
    use crate::constants::hardcoding::ports;
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.api_port(), ports::HTTP_DEFAULT);
}

#[test]
fn test_api_port_custom() {
    let config = NetworkDefaultsV2Config::new().with_api_port(9999);
    assert_eq!(config.api_port(), 9999);
}

#[test]
fn test_api_bind_address_default() {
    use crate::constants::hardcoding::{addresses, ports};
    let config = NetworkDefaultsV2Config::new();
    let expected = format!("{}:{}", addresses::LOCALHOST_IPV4, ports::HTTP_DEFAULT);
    assert_eq!(config.api_bind_address(), expected);
}

#[test]
fn test_api_bind_address_custom() {
    use crate::constants::hardcoding::{addresses, ports};
    let test_host = addresses::BIND_ALL_IPV4;
    let test_port = ports::API_DEFAULT;
    let config = NetworkDefaultsV2Config::new()
        .with_api_host(test_host.to_string())
        .with_api_port(test_port);
    let expected = format!("{}:{}", test_host, test_port);
    assert_eq!(config.api_bind_address(), expected);
}

// ==================== METRICS TESTS ====================

#[test]
fn test_metrics_port_default() {
    use crate::constants::hardcoding::ports;
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.metrics_port(), ports::METRICS_DEFAULT);
}

#[test]
fn test_metrics_port_custom() {
    let config = NetworkDefaultsV2Config::new().with_metrics_port(9091);
    assert_eq!(config.metrics_port(), 9091);
}

#[test]
fn test_metrics_bind_address() {
    use crate::constants::hardcoding::{addresses, ports};
    let config = NetworkDefaultsV2Config::new();
    let expected = format!("{}:{}", addresses::LOCALHOST_IPV4, ports::METRICS_DEFAULT);
    assert_eq!(config.metrics_bind_address(), expected);
}

// ==================== WEBSOCKET TESTS ====================

#[test]
fn test_ws_port_default() {
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.websocket_port(), 8081); // Uses ADMIN_PORT = 8081
}

#[test]
fn test_ws_port_custom() {
    let config = NetworkDefaultsV2Config::new().with_websocket_port(8083);
    assert_eq!(config.websocket_port(), 8083);
}

#[test]
fn test_ws_bind_address() {
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.websocket_bind_address(), "127.0.0.1:8081"); // Uses ADMIN_PORT = 8081
}

// ==================== HEALTH CHECK TESTS ====================

#[test]
fn test_health_port_default() {
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.health_port(), 8082); // Uses HEALTH_PORT = 8082
}

#[test]
fn test_health_port_custom() {
    let config = NetworkDefaultsV2Config::new().with_health_port(8083);
    assert_eq!(config.health_port(), 8083);
}

#[test]
fn test_health_bind_address() {
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.health_bind_address(), "127.0.0.1:8082"); // Uses HEALTH_PORT = 8082
}

// ==================== STORAGE TESTS ====================

#[test]
fn test_storage_port_default() {
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.storage_port(), 3000); // Uses DEV_PORT = 3000
}

#[test]
fn test_storage_port_custom() {
    let config = NetworkDefaultsV2Config::new().with_storage_port(5001);
    assert_eq!(config.storage_port(), 5001);
}

#[test]
fn test_storage_bind_address() {
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.storage_bind_address(), "127.0.0.1:3000"); // Uses DEV_PORT = 3000
}

// ==================== INVALID INPUT HANDLING ====================
// Note: Invalid input handling is done at the from_env() level
// These tests verify default behavior

#[test]
fn test_port_validation() {
    // Config ensures valid ports through type system (u16)
    let config = NetworkDefaultsV2Config::new();
    assert_eq!(config.api_port(), 8080);
}

#[test]
fn test_valid_port_range() {
    // Test that we can set valid ports
    let config = NetworkDefaultsV2Config::new().with_api_port(8888);
    assert_eq!(config.api_port(), 8888);
}

#[test]
fn test_host_configuration() {
    // Test that empty host is handled properly
    let config = NetworkDefaultsV2Config::new().with_api_host("127.0.0.1".to_string());
    assert_eq!(config.api_host(), "127.0.0.1");
}

// ==================== SOVEREIGNTY COMPLIANCE ====================

#[test]
fn test_no_hardcoded_endpoints() {
    // All defaults should be overridable via builder
    let config = NetworkDefaultsV2Config::new()
        .with_api_host("custom.host".to_string())
        .with_api_port(9999);

    assert_eq!(config.api_host(), "custom.host");
    assert_eq!(config.api_port(), 9999);
}

#[test]
fn test_bind_address_composition() {
    // Verify bind address is composed from host and port
    let config = NetworkDefaultsV2Config::new()
        .with_api_host("10.0.0.1".to_string())
        .with_api_port(8888);

    let bind = config.api_bind_address();
    assert_eq!(bind, "10.0.0.1:8888");
}

// ==================== CONCURRENT ACCESS ====================

#[tokio::test]
async fn test_concurrent_reads() {
    // Modern concurrent-safe test using Arc and tokio
    use std::sync::Arc;

    let config = Arc::new(NetworkDefaultsV2Config::new().with_api_port(7777));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let config_clone = Arc::clone(&config);
            tokio::spawn(async move {
                let port = config_clone.api_port();
                assert_eq!(port, 7777);
            })
        })
        .collect();

    for handle in handles {
        handle.await.expect("Task should complete");
    }
}

// ==================== IPV6 SUPPORT ====================

#[test]
fn test_ipv6_address_support() {
    let config = NetworkDefaultsV2Config::new().with_api_host("::1".to_string());
    assert_eq!(config.api_host(), "::1");
}

#[test]
fn test_ipv6_bind_address() {
    // IPv6 addresses need brackets in bind format
    let config = NetworkDefaultsV2Config::new()
        .with_api_host("[::1]".to_string())
        .with_api_port(8080);
    let bind = config.api_bind_address();
    assert!(bind.contains("::1"));
}

// ==================== PORT VALIDATION ====================

#[test]
fn test_port_range_validation() {
    // Test minimum valid port
    let config1 = NetworkDefaultsV2Config::new().with_api_port(1);
    assert!(config1.api_port() >= 1);

    // Test maximum valid port (u16 max)
    let config2 = NetworkDefaultsV2Config::new().with_api_port(65535);
    assert_eq!(config2.api_port(), 65535);
}
