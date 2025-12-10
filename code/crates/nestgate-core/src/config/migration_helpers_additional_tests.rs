//! Additional tests for configuration migration helpers

#![cfg(test)]

use super::*;

// ==================== ADDITIONAL EDGE CASE TESTS ====================

#[test]
fn test_get_port_with_invalid_env() {
    env::set_var("INVALID_PORT", "not_a_number");
    let port = get_port("INVALID_PORT", Some(8080), 3000);
    // Should fall back to config when env is invalid
    assert_eq!(port, 8080);
    env::remove_var("INVALID_PORT");
}

#[test]
fn test_get_port_with_zero() {
    let port = get_port("NONEXISTENT", Some(0), 3000);
    // Zero port is technically valid (means "any available port")
    assert_eq!(port, 0);
}

#[test]
fn test_get_port_with_max_value() {
    let port = get_port("NONEXISTENT", Some(65534), 3000);
    assert_eq!(port, 65534);
}

#[test]
fn test_get_host_with_env() {
    env::set_var("TEST_HOST", "192.168.1.1");
    let host = get_host("TEST_HOST", Some("localhost".to_string()), "127.0.0.1");
    assert_eq!(host, "192.168.1.1");
    env::remove_var("TEST_HOST");
}

#[test]
fn test_get_host_with_config() {
    let host = get_host("NONEXISTENT_HOST", Some("localhost".to_string()), "127.0.0.1");
    assert_eq!(host, "localhost");
}

#[test]
fn test_get_host_with_default() {
    let host = get_host("NONEXISTENT_HOST", None, "127.0.0.1");
    assert_eq!(host, "127.0.0.1");
}

#[test]
fn test_get_host_empty_string() {
    env::set_var("EMPTY_HOST", "");
    let host = get_host("EMPTY_HOST", Some("localhost".to_string()), "127.0.0.1");
    // Empty string from env should be used (it's what was set)
    assert_eq!(host, "");
    env::remove_var("EMPTY_HOST");
}

#[test]
fn test_build_address_with_ipv4() {
    let addr = build_address("192.168.1.1", 8080).unwrap();
    assert_eq!(addr, "192.168.1.1:8080");
}

#[test]
fn test_build_address_with_ipv6() {
    let addr = build_address("::1", 8080).unwrap();
    assert_eq!(addr, "::1:8080");
}

#[test]
fn test_build_address_with_hostname() {
    let addr = build_address("example.com", 443).unwrap();
    assert_eq!(addr, "example.com:443");
}

#[test]
fn test_build_address_with_port_zero() {
    let addr = build_address("localhost", 0).unwrap();
    assert_eq!(addr, "localhost:0");
}

#[test]
fn test_build_address_with_max_port() {
    let addr = build_address("localhost", 65535).unwrap();
    assert_eq!(addr, "localhost:65535");
}

#[test]
fn test_get_with_fallback_string() {
    let result = get_with_fallback("NONEXISTENT", Some("from_config".to_string()), "default".to_string());
    assert_eq!(result, "from_config");
}

#[test]
fn test_get_with_fallback_to_default() {
    let result: String = get_with_fallback("NONEXISTENT", None, "default".to_string());
    assert_eq!(result, "default");
}

#[test]
fn test_get_with_fallback_from_env() {
    env::set_var("FALLBACK_TEST", "from_env");
    let result = get_with_fallback("FALLBACK_TEST", Some("from_config".to_string()), "default".to_string());
    assert_eq!(result, "from_env");
    env::remove_var("FALLBACK_TEST");
}

#[test]
fn test_get_with_fallback_invalid_parse() {
    env::set_var("INVALID_NUMBER", "not_a_number");
    let result: u16 = get_with_fallback("INVALID_NUMBER", Some(123), 456);
    // Should fall back to config when env can't be parsed
    assert_eq!(result, 123);
    env::remove_var("INVALID_NUMBER");
}

// ==================== INTEGRATION-STYLE TESTS ====================

#[test]
fn test_multiple_fallback_levels() {
    // Test all three fallback levels work together
    env::remove_var("TEST_MULTI");
    
    // Level 3: Default only
    let result1: u16 = get_with_fallback("TEST_MULTI", None, 3000);
    assert_eq!(result1, 3000);
    
    // Level 2: Config fallback
    let result2: u16 = get_with_fallback("TEST_MULTI", Some(8080), 3000);
    assert_eq!(result2, 8080);
    
    // Level 1: Environment override
    env::set_var("TEST_MULTI", "9999");
    let result3: u16 = get_with_fallback("TEST_MULTI", Some(8080), 3000);
    assert_eq!(result3, 9999);
    
    env::remove_var("TEST_MULTI");
}

#[test]
fn test_address_building_workflow() {
    // Test complete workflow: get host, get port, build address
    let host = get_host("TEST_WORKFLOW_HOST", None, "localhost");
    let port = get_port("TEST_WORKFLOW_PORT", None, 8080);
        let addr = build_address(&host, port).expect("Should build address in concurrent test");
    
    assert_eq!(addr, "localhost:8080");
}

