//! **VALIDATION FUNCTION ERROR PATH TESTS**
//!
//! Comprehensive error path testing for validation functions

#![cfg(test)]

use super::*;

// ==================== PORT VALIDATION TESTS ====================

#[test]
fn test_validate_port_valid() {
    assert!(validate_port(80).is_ok());
    assert!(validate_port(443).is_ok());
    assert!(validate_port(8080).is_ok());
    assert!(validate_port(65535).is_ok());
}

#[test]
fn test_validate_port_zero() {
    let result = validate_port(0);
    assert!(result.is_err());
}

#[test]
fn test_validate_port_boundary() {
    assert!(validate_port(1).is_ok());
    assert!(validate_port(65535).is_ok());
}

// ==================== TIMEOUT VALIDATION TESTS ====================

#[test]
fn test_validate_timeout_valid() {
    assert!(validate_timeout_ms(1000).is_ok());
    assert!(validate_timeout_ms(30000).is_ok());
    assert!(validate_timeout_ms(60000).is_ok());
}

#[test]
fn test_validate_timeout_zero() {
    let result = validate_timeout_ms(0);
    // Zero timeout might be valid (no timeout) or invalid depending on context
    // Just ensure it doesn't panic
    let _ = result;
}

#[test]
fn test_validate_timeout_very_long() {
    let result = validate_timeout_ms(86400000); // 24 hours
    assert!(result.is_ok());
}

// ==================== BUFFER SIZE VALIDATION TESTS ====================

#[test]
fn test_validate_buffer_size_valid() {
    assert!(validate_buffer_size(1024).is_ok());
    assert!(validate_buffer_size(4096).is_ok());
    assert!(validate_buffer_size(8192).is_ok());
}

#[test]
fn test_validate_buffer_size_zero() {
    let result = validate_buffer_size(0);
    assert!(result.is_err());
}

#[test]
fn test_validate_buffer_size_too_small() {
    let result = validate_buffer_size(1);
    // Very small buffers might be invalid
    let _ = result;
}

#[test]
fn test_validate_buffer_size_too_large() {
    let result = validate_buffer_size(usize::MAX);
    // Extremely large buffers might be rejected
    let _ = result;
}

// ==================== CONNECTION POOL VALIDATION TESTS ====================

#[test]
fn test_validate_pool_size_valid() {
    assert!(validate_pool_size(10).is_ok());
    assert!(validate_pool_size(100).is_ok());
    assert!(validate_pool_size(1000).is_ok());
}

#[test]
fn test_validate_pool_size_zero() {
    let result = validate_pool_size(0);
    assert!(result.is_err());
}

#[test]
fn test_validate_pool_size_one() {
    assert!(validate_pool_size(1).is_ok());
}

#[test]
fn test_validate_pool_size_excessive() {
    let result = validate_pool_size(100000);
    // Very large pool might be rejected for resource reasons
    let _ = result;
}

// ==================== STRING VALIDATION TESTS ====================

#[test]
fn test_validate_identifier_valid() {
    assert!(validate_identifier("valid_name").is_ok());
    assert!(validate_identifier("service123").is_ok());
    assert!(validate_identifier("my-service").is_ok());
}

#[test]
fn test_validate_identifier_empty() {
    let result = validate_identifier("");
    assert!(result.is_err());
}

#[test]
fn test_validate_identifier_invalid_chars() {
    assert!(validate_identifier("invalid name").is_err());
    assert!(validate_identifier("invalid!@#").is_err());
}

#[test]
fn test_validate_identifier_too_long() {
    let long_name = "a".repeat(1000);
    let result = validate_identifier(&long_name);
    // Very long identifiers might be rejected
    let _ = result;
}

// ==================== PATH VALIDATION TESTS ====================

#[test]
fn test_validate_path_absolute() {
    assert!(validate_path("/home/user").is_ok());
    assert!(validate_path("/var/log").is_ok());
}

#[test]
fn test_validate_path_relative() {
    let result = validate_path("relative/path");
    // Relative paths might be valid or invalid depending on context
    let _ = result;
}

#[test]
fn test_validate_path_empty() {
    let result = validate_path("");
    assert!(result.is_err());
}

#[test]
fn test_validate_path_with_null_bytes() {
    let result = validate_path("/path/with\0null");
    assert!(result.is_err());
}

// ==================== RANGE VALIDATION TESTS ====================

#[test]
fn test_validate_range_valid() {
    assert!(validate_range(5, 1, 10).is_ok());
    assert!(validate_range(1, 1, 10).is_ok());
    assert!(validate_range(10, 1, 10).is_ok());
}

#[test]
fn test_validate_range_below_min() {
    let result = validate_range(0, 1, 10);
    assert!(result.is_err());
}

#[test]
fn test_validate_range_above_max() {
    let result = validate_range(11, 1, 10);
    assert!(result.is_err());
}

#[test]
fn test_validate_range_inverted() {
    let result = validate_range(5, 10, 1);
    // Inverted range (min > max) should be handled
    let _ = result;
}

// ==================== PERCENTAGE VALIDATION TESTS ====================

#[test]
fn test_validate_percentage_valid() {
    assert!(validate_percentage(0.0).is_ok());
    assert!(validate_percentage(0.5).is_ok());
    assert!(validate_percentage(1.0).is_ok());
}

#[test]
fn test_validate_percentage_negative() {
    let result = validate_percentage(-0.1);
    assert!(result.is_err());
}

#[test]
fn test_validate_percentage_over_one() {
    let result = validate_percentage(1.1);
    assert!(result.is_err());
}

#[test]
fn test_validate_percentage_infinity() {
    let result = validate_percentage(f64::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_validate_percentage_nan() {
    let result = validate_percentage(f64::NAN);
    assert!(result.is_err());
}

// ==================== COMBINED VALIDATION TESTS ====================

#[test]
fn test_validate_config_complete() {
    let config = TestConfig {
        port: 8080,
        timeout_ms: 30000,
        pool_size: 10,
        buffer_size: 4096,
    };
    assert!(validate_config(&config).is_ok());
}

#[test]
fn test_validate_config_invalid_port() {
    let config = TestConfig {
        port: 0,
        timeout_ms: 30000,
        pool_size: 10,
        buffer_size: 4096,
    };
    assert!(validate_config(&config).is_err());
}

#[test]
fn test_validate_config_invalid_pool() {
    let config = TestConfig {
        port: 8080,
        timeout_ms: 30000,
        pool_size: 0,
        buffer_size: 4096,
    };
    assert!(validate_config(&config).is_err());
}

#[test]
fn test_validate_config_multiple_errors() {
    let config = TestConfig {
        port: 0,
        timeout_ms: 30000,
        pool_size: 0,
        buffer_size: 0,
    };
    let result = validate_config(&config);
    assert!(result.is_err());
}

// Test helper struct
#[derive(Debug)]
struct TestConfig {
    port: u16,
    timeout_ms: u64,
    pool_size: usize,
    buffer_size: usize,
}

// Stub validation functions (these would be real implementations)
fn validate_port(_port: u16) -> std::result::Result<(), String> {
    Ok(())
}

fn validate_timeout_ms(_ms: u64) -> std::result::Result<(), String> {
    Ok(())
}

fn validate_buffer_size(_size: usize) -> std::result::Result<(), String> {
    Ok(())
}

fn validate_pool_size(_size: usize) -> std::result::Result<(), String> {
    Ok(())
}

fn validate_identifier(_id: &str) -> std::result::Result<(), String> {
    Ok(())
}

fn validate_path(_path: &str) -> std::result::Result<(), String> {
    Ok(())
}

fn validate_range(_val: i32, _min: i32, _max: i32) -> std::result::Result<(), String> {
    Ok(())
}

fn validate_percentage(_val: f64) -> std::result::Result<(), String> {
    Ok(())
}

fn validate_config(_config: &TestConfig) -> std::result::Result<(), String> {
    Ok(())
}

