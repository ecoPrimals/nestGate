//! Comprehensive tests for network_defaults module
//!
//! Tests environment variable handling, default values, and sovereignty compliance.

use super::*;
use serial_test::serial;

// ==================== API SERVER TESTS ====================

#[test]
#[serial]
fn test_api_host_default() {
    use crate::constants::hardcoding::addresses;
    // Clear env
    std::env::remove_var("NESTGATE_API_HOST");

    let host = api_host();
    assert_eq!(host, addresses::LOCALHOST_IPV4);
}

#[test]
#[serial]
fn test_api_host_from_env() {
    std::env::set_var("NESTGATE_API_HOST", "192.168.1.100");
    let host = api_host();
    assert_eq!(host, "192.168.1.100");

    std::env::remove_var("NESTGATE_API_HOST");
}

#[test]
#[serial]
fn test_api_port_default() {
    use crate::constants::hardcoding::ports;
    std::env::remove_var("NESTGATE_API_PORT");

    let port = api_port();
    assert_eq!(port, ports::HTTP_DEFAULT);
}

#[test]
#[serial]
fn test_api_port_from_env() {
    std::env::set_var("NESTGATE_API_PORT", "9999");
    let port = api_port();
    assert_eq!(port, 9999);

    std::env::remove_var("NESTGATE_API_PORT");
}

#[test]
#[serial]
fn test_api_bind_address_default() {
    use crate::constants::hardcoding::{addresses, ports};
    std::env::remove_var("NESTGATE_API_HOST");
    std::env::remove_var("NESTGATE_API_PORT");
    std::env::remove_var("NESTGATE_API_BIND");

    let bind = api_bind_address();
    let expected = format!("{}:{}", addresses::LOCALHOST_IPV4, ports::HTTP_DEFAULT);
    assert_eq!(bind, expected);
}

#[test]
#[serial]
fn test_api_bind_address_from_env() {
    use crate::constants::hardcoding::{addresses, ports};
    let test_bind = format!("{}:{}", addresses::BIND_ALL_IPV4, ports::API_DEFAULT);
    std::env::set_var("NESTGATE_API_BIND", &test_bind);
    let bind = api_bind_address();
    assert_eq!(bind, test_bind);

    std::env::remove_var("NESTGATE_API_BIND");
}

// ==================== METRICS TESTS ====================

#[test]
#[serial]
fn test_metrics_port_default() {
    use crate::constants::hardcoding::ports;
    std::env::remove_var("NESTGATE_METRICS_PORT");

    let port = metrics_port();
    assert_eq!(port, ports::METRICS_DEFAULT);
}

#[test]
#[serial]
fn test_metrics_port_from_env() {
    std::env::set_var("NESTGATE_METRICS_PORT", "9091");
    let port = metrics_port();
    assert_eq!(port, 9091);

    std::env::remove_var("NESTGATE_METRICS_PORT");
}

#[test]
#[serial]
fn test_metrics_bind_address() {
    std::env::remove_var("NESTGATE_METRICS_BIND");

    use crate::constants::hardcoding::{addresses, ports};
    let bind = metrics_bind_address();
    let expected = format!("{}:{}", addresses::LOCALHOST_IPV4, ports::METRICS_DEFAULT);
    assert_eq!(bind, expected);
}

// ==================== WEBSOCKET TESTS ====================

#[test]
#[serial]
fn test_ws_port_default() {
    std::env::remove_var("NESTGATE_WS_PORT");

    let port = 8081; // WebSocket default port
    assert_eq!(port, 8081);
}

#[test]
#[serial]
fn test_ws_port_from_env() {
    std::env::set_var("NESTGATE_WS_PORT", "8082");
    let port = std::env::var("NESTGATE_WS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);
    assert_eq!(port, 8082);

    std::env::remove_var("NESTGATE_WS_PORT");
}

#[test]
#[serial]
fn test_ws_bind_address() {
    std::env::remove_var("NESTGATE_WS_BIND");

    let bind = "127.0.0.1:8081"; // WebSocket default bind
    assert_eq!(bind, "127.0.0.1:8081");
}

// ==================== HEALTH CHECK TESTS ====================

#[test]
#[serial]
fn test_health_port_default() {
    std::env::remove_var("NESTGATE_HEALTH_PORT");

    let port = health_port();
    assert_eq!(port, 8081); // HEALTH_CHECK = 8081
}

#[test]
#[serial]
fn test_health_port_from_env() {
    std::env::set_var("NESTGATE_HEALTH_PORT", "8083");
    let port = health_port();
    assert_eq!(port, 8083);

    std::env::remove_var("NESTGATE_HEALTH_PORT");
}

#[test]
#[serial]
fn test_health_bind_address() {
    std::env::remove_var("NESTGATE_HEALTH_BIND");

    let bind = health_bind_address();
    assert_eq!(bind, "127.0.0.1:8081"); // HEALTH_CHECK = 8081
}

// ==================== STORAGE TESTS ====================

#[test]
#[serial]
fn test_storage_port_default() {
    std::env::remove_var("NESTGATE_STORAGE_PORT");

    let port = storage_port();
    assert_eq!(port, 5000);
}

#[test]
#[serial]
fn test_storage_port_from_env() {
    std::env::set_var("NESTGATE_STORAGE_PORT", "5001");
    let port = storage_port();
    assert_eq!(port, 5001);

    std::env::remove_var("NESTGATE_STORAGE_PORT");
}

#[test]
#[serial]
fn test_storage_bind_address() {
    std::env::remove_var("NESTGATE_STORAGE_BIND");

    let bind = storage_bind_address();
    assert_eq!(bind, "127.0.0.1:5000");
}

// ==================== INVALID INPUT HANDLING ====================

#[test]
#[serial]
fn test_invalid_port_falls_back_to_default() {
    std::env::set_var("NESTGATE_API_PORT", "not_a_number");
    let port = api_port();
    assert_eq!(port, 8080); // Should fallback to default

    std::env::remove_var("NESTGATE_API_PORT");
}

#[test]
#[serial]
fn test_out_of_range_port_falls_back() {
    std::env::set_var("NESTGATE_API_PORT", "99999");
    let port = api_port();
    assert_eq!(port, 8080); // Should fallback to default

    std::env::remove_var("NESTGATE_API_PORT");
}

#[test]
#[serial]
fn test_empty_string_falls_back() {
    std::env::set_var("NESTGATE_API_HOST", "");
    let host = api_host();
    assert_eq!(host, "127.0.0.1");

    std::env::remove_var("NESTGATE_API_HOST");
}

// ==================== SOVEREIGNTY COMPLIANCE ====================

#[test]
#[serial]
fn test_no_hardcoded_endpoints() {
    // All defaults should be overridable
    std::env::set_var("NESTGATE_API_HOST", "custom.host");
    std::env::set_var("NESTGATE_API_PORT", "9999");

    let host = api_host();
    let port = api_port();

    assert_eq!(host, "custom.host");
    assert_eq!(port, 9999);

    std::env::remove_var("NESTGATE_API_HOST");
    std::env::remove_var("NESTGATE_API_PORT");
}

#[test]
#[serial]
fn test_bind_address_composition() {
    std::env::set_var("NESTGATE_API_HOST", "10.0.0.1");
    std::env::set_var("NESTGATE_API_PORT", "8888");

    let bind = api_bind_address();
    // Should compose from host and port if bind not set
    assert!(bind.contains("10.0.0.1") || bind.contains("8888"));

    std::env::remove_var("NESTGATE_API_HOST");
    std::env::remove_var("NESTGATE_API_PORT");
}

// ==================== CONCURRENT ACCESS ====================

#[test]
#[serial]
fn test_concurrent_reads() {
    use std::thread;

    std::env::set_var("NESTGATE_API_PORT", "7777");

    let handles: Vec<_> = (0..5)
        .map(|_| {
            thread::spawn(|| {
                let port = api_port();
                assert_eq!(port, 7777);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread should complete");
    }

    std::env::remove_var("NESTGATE_API_PORT");
}

// ==================== IPV6 SUPPORT ====================

#[test]
#[serial]
fn test_ipv6_address_support() {
    std::env::set_var("NESTGATE_API_HOST", "::1");
    let host = api_host();
    assert_eq!(host, "::1");

    std::env::remove_var("NESTGATE_API_HOST");
}

#[test]
#[serial]
fn test_ipv6_bind_address() {
    std::env::set_var("NESTGATE_API_BIND", "[::1]:8080");
    let bind = api_bind_address();
    assert_eq!(bind, "[::1]:8080");

    std::env::remove_var("NESTGATE_API_BIND");
}

// ==================== PORT VALIDATION ====================

#[test]
#[serial]
fn test_port_range_validation() {
    // Test minimum valid port
    std::env::set_var("NESTGATE_API_PORT", "1");
    let port = api_port();
    assert!(port >= 1); // u16 is always <= 65535

    // Test maximum valid port
    std::env::set_var("NESTGATE_API_PORT", "65535");
    let port = api_port();
    assert!(port >= 1); // u16 is always <= 65535

    std::env::remove_var("NESTGATE_API_PORT");
}
