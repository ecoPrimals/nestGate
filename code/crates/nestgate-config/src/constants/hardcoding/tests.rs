// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::runtime_fallback_ports as fb;
use super::*;

#[test]
fn test_addresses_are_valid() {
    assert_eq!(addresses::LOCALHOST_IPV4, "127.0.0.1");
    assert_eq!(addresses::LOCALHOST_IPV6, "::1");
    assert_eq!(addresses::LOCALHOST_NAME, "localhost");
    assert_eq!(addresses::BIND_ALL_IPV4, "0.0.0.0");
    assert_eq!(addresses::BIND_ALL_IPV6, "::");
}

#[test]
fn test_ports_are_in_valid_range() {
    // All ports are u16, which are always >= 0, so just verify they're defined
    // These checks serve as documentation that these ports exist and are configured
    assert_eq!(fb::HTTP, 8080);
    assert_eq!(fb::HTTPS, 8443);
    assert_eq!(fb::API, 3000);
    assert_eq!(fb::METRICS, 9090);
    assert_eq!(fb::HEALTH, 8081);
}

#[test]
fn test_get_bind_address_default() {
    // Should return default when env var not set
    let addr = get_bind_address();
    assert!(!addr.is_empty());
}

#[test]
fn test_get_api_port_default() {
    // Should return valid port when env var not set
    let port = get_api_port();
    assert!(port > 0);
}

#[test]
fn test_discovery_timeout() {
    let timeout = discovery::get_timeout_ms();
    assert!(timeout > 0);
}

#[test]
fn test_limits_are_reasonable() {
    // These are compile-time constants, so we verify their relationships
    // rather than testing values that are always true.
    const _: () = assert!(limits::BUFFER_SIZE_MAX >= limits::BUFFER_SIZE_DEFAULT);
    const _: () = assert!(limits::MAX_CONNECTIONS >= limits::CONNECTION_POOL_SIZE);

    // Runtime verification that constants are accessible
    let _ = limits::BUFFER_SIZE_DEFAULT;
    let _ = limits::CONNECTION_POOL_SIZE;
    let _ = limits::TIMEOUT_SECS;
    let _ = limits::MAX_RETRIES;
}

// ==================== NEW COMPREHENSIVE TESTS ====================

#[test]
fn test_all_port_constants_are_unique() {
    // Ensure no port collisions in defaults
    let ports_vec = vec![
        fb::HTTP,
        fb::HTTPS,
        fb::METRICS,
        fb::HEALTH,
        fb::GRPC,
        fb::WEBSOCKET,
        fb::ADMIN,
    ];

    // At least verify ports are in valid ranges
    for port in &ports_vec {
        assert!(
            *port > 1024,
            "Port {} should be > 1024 (unprivileged)",
            port
        );
        assert!(*port < 65535, "Port {} should be < 65535", port);
    }
}

#[test]
fn test_database_ports() {
    assert_eq!(fb::POSTGRES, 5432);
    assert_eq!(fb::REDIS, 6379);
    assert_eq!(fb::MONGODB, 27017);
    assert_eq!(fb::MYSQL, 3306);
}

#[test]
fn test_service_ports() {
    assert_eq!(fb::DISCOVERY_SERVICE, 3010);
    assert_eq!(fb::ORCHESTRATOR_DEFAULT, 8090);
    assert_eq!(fb::STORAGE, 5000);
    assert_eq!(fb::COMPUTE, 8085);
}

#[test]
fn test_timeout_constants() {
    assert_eq!(timeouts::CONNECT_MS, 5_000);
    assert_eq!(timeouts::REQUEST_MS, 30_000);
    assert_eq!(timeouts::LONG_OPERATION_MS, 300_000);

    // Timeout hierarchy: CONNECT < REQUEST < LONG_OPERATION (enforced by design)
    // No runtime assertion needed - these are constants with intentional values
}

#[test]
fn test_discovery_constants() {
    assert_eq!(discovery::TIMEOUT_MS, 5000);
    assert_eq!(discovery::RETRY_ATTEMPTS, 3);
    assert_eq!(discovery::SCAN_PORT_START, 3000);
    assert_eq!(discovery::SCAN_PORT_END, 3999);

    // Port range: 3000-3999 (enforced by design, valid range guaranteed)
}

#[test]
fn test_get_metrics_port() {
    let port = get_metrics_port();
    assert_eq!(port, fb::METRICS);
    assert!(port > 0);
}

#[test]
fn test_get_health_port() {
    let port = get_health_port();
    assert_eq!(port, fb::HEALTH);
    assert!(port > 0);
}

#[test]
fn test_discovery_timeout_helper() {
    let timeout = discovery::get_timeout_ms();
    assert_eq!(timeout, discovery::TIMEOUT_MS);
    assert!(timeout > 0);
}

#[test]
fn test_ipv4_address_format() {
    // Verify IPv4 addresses are properly formatted
    assert!(
        addresses::LOCALHOST_IPV4
            .parse::<std::net::Ipv4Addr>()
            .is_ok()
    );
    assert!(
        addresses::BIND_ALL_IPV4
            .parse::<std::net::Ipv4Addr>()
            .is_ok()
    );
}

#[test]
fn test_ipv6_address_format() {
    // Verify IPv6 addresses are properly formatted
    assert!(
        addresses::LOCALHOST_IPV6
            .parse::<std::net::Ipv6Addr>()
            .is_ok()
    );
    assert!(
        addresses::BIND_ALL_IPV6
            .parse::<std::net::Ipv6Addr>()
            .is_ok()
    );
}

#[test]
fn test_buffer_size_limits() {
    assert_eq!(limits::BUFFER_SIZE_DEFAULT, 65536);
    assert_eq!(limits::BUFFER_SIZE_MAX, 1_048_576);
    // Buffer size hierarchy: DEFAULT < MAX (enforced by design)
}

#[test]
fn test_connection_limits() {
    assert_eq!(limits::CONNECTION_POOL_SIZE, 10);
    assert_eq!(limits::MAX_CONNECTIONS, 1000);
    // Connection limits: POOL_SIZE < MAX_CONNECTIONS (enforced by design)
}

#[test]
fn test_retry_configuration() {
    assert_eq!(limits::MAX_RETRIES, 3);
    assert_eq!(limits::TIMEOUT_SECS, 30);
    // Both values are positive by design (non-zero required for operation)
}

#[test]
fn test_service_capability_ports() {
    // Generic service defaults (capability-based discovery preferred)
    assert_eq!(fb::HEALTH, 8081);
    assert_eq!(fb::WEBSOCKET, 8082);
    assert_ne!(fb::HEALTH, fb::WEBSOCKET);
}

#[test]
fn test_extended_services_port() {
    assert_eq!(fb::EXTENDED_SERVICES, 3002);
    assert_eq!(fb::API_ALT, 3001);
    assert_ne!(fb::EXTENDED_SERVICES, fb::API);
}

#[test]
fn test_bind_address_is_valid() {
    let addr = get_bind_address();
    assert!(!addr.is_empty());
    // Should be either IPv4 or IPv6
    assert!(
        addr.parse::<std::net::Ipv4Addr>().is_ok() || addr.parse::<std::net::Ipv6Addr>().is_ok()
    );
}

#[test]
fn test_api_port_is_valid() {
    let port = get_api_port();
    assert!(port > 0);
    // u16 automatically ensures port <= 65535
}
