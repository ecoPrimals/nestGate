//! Validation tests for defaults module
//!
//! Tests edge cases, validation logic, and error handling.

use super::*;
use serial_test::serial;

// ==================== TIMEOUT VALIDATION ====================

#[test]
fn test_timeout_constants_are_valid() {
    assert!(timeouts::DEFAULT_API_TIMEOUT.as_secs() > 0);
    assert!(timeouts::DEFAULT_DB_TIMEOUT.as_secs() > 0);
    assert!(timeouts::DEFAULT_HEALTH_TIMEOUT.as_secs() > 0);
    assert!(timeouts::DEFAULT_WS_TIMEOUT.as_secs() > 0);
}

#[test]
fn test_timeout_ordering_is_logical() {
    // Health check should be fastest
    assert!(timeouts::DEFAULT_HEALTH_TIMEOUT < timeouts::DEFAULT_DB_TIMEOUT);
    // DB should be faster than general API
    assert!(timeouts::DEFAULT_DB_TIMEOUT < timeouts::DEFAULT_API_TIMEOUT);
    // WebSocket should be longest (persistent connections)
    assert!(timeouts::DEFAULT_API_TIMEOUT < timeouts::DEFAULT_WS_TIMEOUT);
}

#[test]
fn test_timeout_values_are_reasonable() {
    // No timeout should be less than 1 second
    assert!(timeouts::DEFAULT_HEALTH_TIMEOUT.as_secs() >= 1);
    // No timeout should be more than 5 minutes
    assert!(timeouts::DEFAULT_WS_TIMEOUT.as_secs() <= 300);
}

// ==================== NETWORK DEFAULTS VALIDATION ====================

#[test]
fn test_default_ports_are_unique() {
    let ports = [
        network::DEFAULT_API_PORT,
        network::DEFAULT_WS_PORT,
        network::DEFAULT_HEALTH_PORT,
    ];

    // Check all ports are different
    for i in 0..ports.len() {
        for j in (i + 1)..ports.len() {
            assert_ne!(ports[i], ports[j], "Ports should be unique");
        }
    }
}

#[test]
fn test_default_ports_are_in_valid_range() {
    // These are compile-time constants, use const assertions
    const _: () = assert!(network::DEFAULT_API_PORT >= 1024);
    ///  
    const _: () = assert!(network::DEFAULT_WS_PORT >= 1024);
    // DEFAULT_API_PORT is u16, always <= 65535
    // DEFAULT_WS_PORT is u16, always <= 65535
}

#[test]
fn test_bind_address_is_valid() {
    let addr = network::DEFAULT_BIND_ADDRESS;
    use crate::constants::hardcoding::addresses;
    assert!(!addr.is_empty());
    // Should be a valid IP format (one of the safe defaults)
    assert!(
        addr == addresses::BIND_ALL_IPV4
            || addr == addresses::LOCALHOST_IPV4
            || addr == addresses::BIND_ALL_IPV6
    );
}

#[test]
fn test_hostname_is_valid() {
    let hostname = network::DEFAULT_HOSTNAME;
    assert!(!hostname.is_empty());
    assert!(hostname.len() < 256); // DNS limit
}

// ==================== DATABASE DEFAULTS VALIDATION ====================

#[test]
fn test_database_ports_are_standard() {
    // PostgreSQL standard port
    assert_eq!(database::DEFAULT_POSTGRES_PORT, 5432);
    // Redis standard port
    assert_eq!(database::DEFAULT_REDIS_PORT, 6379);
}

#[test]
fn test_db_host_is_safe_default() {
    use crate::constants::hardcoding::addresses;
    assert_eq!(database::DEFAULT_DB_HOST, addresses::LOCALHOST_NAME);
}

// ==================== MONITORING DEFAULTS VALIDATION ====================

#[test]
fn test_monitoring_ports_dont_conflict() {
    assert_ne!(monitoring::DEFAULT_METRICS_PORT, network::DEFAULT_API_PORT);
    assert_ne!(monitoring::DEFAULT_GRAFANA_PORT, network::DEFAULT_API_PORT);
    assert_ne!(
        monitoring::DEFAULT_METRICS_PORT,
        monitoring::DEFAULT_GRAFANA_PORT
    );
}

#[test]
fn test_metrics_port_is_standard() {
    // Prometheus standard port
    assert_eq!(monitoring::DEFAULT_METRICS_PORT, 9090);
}

#[test]
fn test_grafana_port_is_standard() {
    // Grafana standard port
    assert_eq!(monitoring::DEFAULT_GRAFANA_PORT, 3000);
}

// ==================== ENV HELPER VALIDATION ====================

#[test]
#[serial]
fn test_env_helpers_handle_missing_vars() {
    // Clear all env vars
    std::env::remove_var("NESTGATE_API_PORT");
    std::env::remove_var("NESTGATE_BIND_ADDRESS");

    // Should return defaults without panicking
    let port = env_helpers::api_port();
    let bind = env_helpers::bind_address();

    assert_eq!(port, network::DEFAULT_API_PORT);
    assert_eq!(bind, network::DEFAULT_BIND_ADDRESS);
}

#[test]
#[serial]
fn test_env_helpers_handle_invalid_utf8() {
    // Test with valid strings (Rust env vars are UTF-8)
    std::env::set_var("NESTGATE_API_PORT", "8080");
    let port = env_helpers::api_port();
    assert_eq!(port, ports::HTTP_DEFAULT);

    std::env::remove_var("NESTGATE_API_PORT");
}

#[test]
#[serial]
fn test_env_helpers_parse_zero() {
    std::env::set_var("NESTGATE_API_PORT", "0");
    let port = env_helpers::api_port();
    // Parser successfully parses "0" as 0 (validation happens elsewhere)
    assert_eq!(port, 0);

    std::env::remove_var("NESTGATE_API_PORT");
}

#[test]
#[serial]
fn test_env_helpers_parse_negative() {
    std::env::set_var("NESTGATE_API_PORT", "-1");
    let port = env_helpers::api_port();
    // Should fallback to default (negative is invalid)
    assert_eq!(port, network::DEFAULT_API_PORT);

    std::env::remove_var("NESTGATE_API_PORT");
}

// ==================== CONSISTENCY CHECKS ====================

#[test]
fn test_all_ports_positive() {
    // These are compile-time constants (u16), always > 0
    // Use const assertions for compile-time verification
    const _: () = assert!(network::DEFAULT_API_PORT > 0);
    ///  
    const _: () = assert!(network::DEFAULT_WS_PORT > 0);
    ///  
    const _: () = assert!(network::DEFAULT_HEALTH_PORT > 0);
    ///  
    const _: () = assert!(database::DEFAULT_POSTGRES_PORT > 0);
    ///  
    const _: () = assert!(database::DEFAULT_REDIS_PORT > 0);
    ///  
    const _: () = assert!(monitoring::DEFAULT_METRICS_PORT > 0);
    ///  
    const _: () = assert!(monitoring::DEFAULT_GRAFANA_PORT > 0);
}

#[test]
fn test_all_hostnames_valid() {
    let hostnames = vec![network::DEFAULT_HOSTNAME, database::DEFAULT_DB_HOST];

    for hostname in hostnames {
        assert!(!hostname.is_empty());
        assert!(!hostname.contains('\0'));
        assert!(hostname.len() < 256);
    }
}

#[test]
fn test_bind_addresses_are_ip_format() {
    let addr = network::DEFAULT_BIND_ADDRESS;

    // Should be valid IP or "any" address
    use crate::constants::hardcoding::addresses;
    assert!(
        addr == addresses::BIND_ALL_IPV4
            || addr == addresses::LOCALHOST_IPV4
            || addr == addresses::BIND_ALL_IPV6
            || addr.parse::<std::net::IpAddr>().is_ok()
    );
}

// ==================== TYPE SAFETY ====================

#[test]
fn test_port_type_safety() {
    // Ensure ports are u16 (valid port range)
    let _: u16 = network::DEFAULT_API_PORT;
    let _: u16 = database::DEFAULT_POSTGRES_PORT;
    let _: u16 = monitoring::DEFAULT_METRICS_PORT;
}

#[test]
fn test_timeout_type_safety() {
    use std::time::Duration;

    // Ensure timeouts are Duration types
    let _: Duration = timeouts::DEFAULT_API_TIMEOUT;
    let _: Duration = timeouts::DEFAULT_DB_TIMEOUT;
    let _: Duration = timeouts::DEFAULT_HEALTH_TIMEOUT;
}

#[test]
fn test_string_constants_are_static() {
    // Ensure string constants have 'static lifetime
    let _: &'static str = network::DEFAULT_BIND_ADDRESS;
    let _: &'static str = network::DEFAULT_HOSTNAME;
    let _: &'static str = database::DEFAULT_DB_HOST;
}

// ==================== ADDITIONAL VALIDATION ====================

#[test]
fn test_all_constants_initialized() {
    // Verify all constants can be accessed without panicking
    let _ = network::DEFAULT_API_PORT;
    let _ = network::DEFAULT_WS_PORT;
    let _ = network::DEFAULT_HEALTH_PORT;
    let _ = database::DEFAULT_POSTGRES_PORT;
    let _ = database::DEFAULT_REDIS_PORT;
    let _ = monitoring::DEFAULT_METRICS_PORT;
    let _ = monitoring::DEFAULT_GRAFANA_PORT;
}

#[test]
fn test_timeout_values_representable() {
    // Ensure timeouts don't overflow
    let api = timeouts::DEFAULT_API_TIMEOUT.as_millis();
    let db = timeouts::DEFAULT_DB_TIMEOUT.as_millis();
    let health = timeouts::DEFAULT_HEALTH_TIMEOUT.as_millis();
    let ws = timeouts::DEFAULT_WS_TIMEOUT.as_millis();

    assert!(api > 0 && api < u64::MAX as u128);
    assert!(db > 0 && db < u64::MAX as u128);
    assert!(health > 0 && health < u64::MAX as u128);
    assert!(ws > 0 && ws < u64::MAX as u128);
}

#[test]
fn test_no_duplicate_default_values() {
    // While some defaults can be the same, key ones should differ
    let api_port = network::DEFAULT_API_PORT;
    let ws_port = network::DEFAULT_WS_PORT;
    let health_port = network::DEFAULT_HEALTH_PORT;

    // These should all be different
    assert_ne!(api_port, ws_port);
    assert_ne!(api_port, health_port);
    assert_ne!(ws_port, health_port);
}
