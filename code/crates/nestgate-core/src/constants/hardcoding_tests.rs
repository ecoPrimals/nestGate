//! Tests for hardcoding constants and utilities
//!
//! Ensures hardcoded values are properly centralized and accessible.

use super::hardcoding::*;

#[test]
fn test_port_constants_valid() {
    assert!(ports::HTTP_DEFAULT > 0);
    assert!(ports::HTTP_DEFAULT < 65536);
    
    assert!(ports::HTTPS_DEFAULT > 0);
    assert!(ports::HTTPS_DEFAULT < 65536);
    
    assert!(ports::POSTGRES_DEFAULT > 0);
    assert!(ports::REDIS_DEFAULT > 0);
    assert!(ports::PROMETHEUS_DEFAULT > 0);
}

#[test]
fn test_address_constants() {
    assert_eq!(addresses::BIND_ALL_IPV4, "0.0.0.0");
    assert_eq!(addresses::BIND_ALL_IPV6, "::");
    assert_eq!(addresses::LOCALHOST_IPV4, "127.0.0.1");
    assert_eq!(addresses::LOCALHOST_IPV6, "::1");
    assert_eq!(addresses::LOCALHOST_NAME, "localhost");
}

#[test]
fn test_path_constants() {
    assert!(!paths::DEFAULT_CONFIG_DIR.is_empty());
    assert!(!paths::DEFAULT_DATA_DIR.is_empty());
    assert!(!paths::DEFAULT_LOG_DIR.is_empty());
}

#[test]
fn test_timeout_constants() {
    assert!(timeouts::DEFAULT_CONNECTION_TIMEOUT_MS > 0);
    assert!(timeouts::DEFAULT_REQUEST_TIMEOUT_MS > 0);
    assert!(timeouts::DEFAULT_IDLE_TIMEOUT_MS > 0);
}

#[test]
fn test_capacity_constants() {
    assert!(capacities::DEFAULT_BUFFER_SIZE > 0);
    assert!(capacities::DEFAULT_POOL_SIZE > 0);
    assert!(capacities::MAX_CONNECTIONS > 0);
}

#[test]
fn test_port_ranges() {
    // Ensure ports are in valid ranges
    assert!(ports::HTTP_DEFAULT >= 1024);
    assert!(ports::HTTP_DEFAULT <= 65535);
    
    // HTTPS should typically be 443
    assert_eq!(ports::HTTPS_DEFAULT, 443);
}

#[test]
fn test_timeout_ordering() {
    // Connection timeout should be less than request timeout
    assert!(timeouts::DEFAULT_CONNECTION_TIMEOUT_MS <= timeouts::DEFAULT_REQUEST_TIMEOUT_MS);
}

#[test]
fn test_capacity_sanity() {
    // Buffer size should be reasonable
    assert!(capacities::DEFAULT_BUFFER_SIZE >= 1024);
    assert!(capacities::DEFAULT_BUFFER_SIZE <= 1_048_576); // 1MB max
    
    // Pool size should be reasonable
    assert!(capacities::DEFAULT_POOL_SIZE >= 1);
    assert!(capacities::DEFAULT_POOL_SIZE <= 1000);
}

#[test]
fn test_address_format_ipv4() {
    // Verify IPv4 addresses are properly formatted
    assert!(addresses::LOCALHOST_IPV4.split('.').count() == 4);
    assert!(addresses::BIND_ALL_IPV4.split('.').count() == 4);
}

#[test]
fn test_address_format_ipv6() {
    // Verify IPv6 addresses are valid
    assert!(addresses::LOCALHOST_IPV6.contains(':'));
    assert!(!addresses::BIND_ALL_IPV6.is_empty());
}

