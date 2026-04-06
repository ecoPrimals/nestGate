// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
#![allow(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Comprehensive error handling coverage tests
//!
//! These tests ensure all error variants are properly tested,
//! including construction, display, conversion, and handling.

use nestgate_core::error::{NestGateError, Result};

// ==================== ERROR CONSTRUCTION TESTS ====================

#[test]
fn test_error_invalid_operation() {
    let error = NestGateError::api_error("test_op: test reason");

    assert!(matches!(error, NestGateError::Api(_)));
    let error_string = format!("{}", error);
    assert!(error_string.contains("test_op") || error_string.contains("test reason"));
}

#[test]
fn test_error_not_found() {
    let error = NestGateError::storage_error("user 123 not found");

    assert!(matches!(error, NestGateError::Storage(_)));
    let error_string = format!("{}", error);
    assert!(error_string.contains("user") || error_string.contains("123"));
}

#[test]
fn test_error_configuration() {
    let error = NestGateError::configuration_error("database_url", "invalid format");

    assert!(matches!(error, NestGateError::Configuration(_)));
}

#[test]
fn test_error_network() {
    let error = NestGateError::network_error("connect to localhost:18080 failed");

    assert!(matches!(error, NestGateError::Network(_)));
}

// ==================== ERROR CONVERSION TESTS ====================

#[test]
fn test_error_from_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let nestgate_error: NestGateError = io_error.into();

    // IO errors are converted to Internal errors in the unified error system
    assert!(matches!(
        nestgate_error,
        NestGateError::Internal(_) | NestGateError::Io(_)
    ));
}

#[test]
fn test_error_from_string() {
    let error: NestGateError = "test error message".into();
    let error_string = format!("{}", error);
    assert!(error_string.contains("test error message"));
}

// ==================== RESULT TYPE TESTS ====================

#[test]
fn test_result_ok() {
    let result: Result<i32> = Ok(42);
    assert!(result.is_ok());
    // Use pattern matching instead of unwrap_or for test clarity
    match result {
        Ok(value) => assert_eq!(value, 42),
        Err(_) => panic!("Expected Ok, got Err"),
    }
}

#[test]
fn test_result_err() {
    let result: Result<i32> = Err(NestGateError::api_error("test: error"));

    assert!(result.is_err());
}

// ==================== ERROR PROPAGATION TESTS ====================

/// Function That Returns Result
fn function_that_returns_result() -> Result<String> {
    Ok("success".to_string())
}

/// Function That Returns Error
fn function_that_returns_error() -> Result<String> {
    Err(NestGateError::api_error("test: intentional error"))
}

#[test]
fn test_error_propagation_with_question_mark() {
    /// Caller
    fn caller() -> Result<String> {
        let value = function_that_returns_result()?;
        Ok(value)
    }

    assert!(caller().is_ok());
}

#[test]
fn test_error_propagation_failure() {
    /// Caller
    fn caller() -> Result<String> {
        let _value = function_that_returns_error()?;
        Ok("should not reach here".to_string())
    }

    assert!(caller().is_err());
}

// ==================== ERROR DISPLAY TESTS ====================

#[test]
fn test_error_display_format() {
    let error = NestGateError::api_error("delete: resource locked");

    let display = format!("{}", error);
    assert!(!display.is_empty());
    assert!(display.contains("delete") || display.contains("locked"));
}

#[test]
fn test_error_debug_format() {
    let error = NestGateError::storage_error("file test.txt not found");

    let debug = format!("{:?}", error);
    assert!(!debug.is_empty());
    assert!(debug.contains("Storage") || debug.contains("file"));
}

// ==================== ERROR CHAIN TESTS ====================

#[test]
fn test_error_with_source() {
    let error = NestGateError::network_error("open /etc/secure: access denied");

    assert!(matches!(error, NestGateError::Network(_)));
    // source() method testing would require std::error::Error trait
}

// ==================== ERROR EQUALITY TESTS ====================

#[test]
fn test_error_types_distinguishable() {
    let error1 = NestGateError::api_error("test: reason1");

    let error2 = NestGateError::storage_error("test: 123");

    // Different variants should have different discriminants
    assert!(std::mem::discriminant(&error1) != std::mem::discriminant(&error2));
}

// ==================== ERROR CONTEXT TESTS ====================

#[test]
fn test_error_with_empty_strings() {
    let error = NestGateError::api_error("");

    // Should still be valid, even with empty strings
    assert!(matches!(error, NestGateError::Api(_)));
}

#[test]
fn test_error_with_long_strings() {
    let long_string = "a".repeat(1000);
    let error = NestGateError::api_error(long_string);

    assert!(matches!(error, NestGateError::Api(_)));
}

// ==================== ERROR HANDLING PATTERNS TESTS ====================

#[test]
fn test_map_err_pattern() {
    let result: std::result::Result<(), String> = Err("string error".to_string());
    let mapped: Result<()> =
        result.map_err(|e| NestGateError::api_error(format!("conversion: {}", e)));

    assert!(mapped.is_err());
}

#[test]
fn test_or_else_pattern() {
    let result: Result<i32> = Err(NestGateError::storage_error("item 1 not found"));

    let recovered: Result<i32> = result.or(Ok(0));
    assert_eq!(recovered.expect("Test setup failed"), 0);
}

#[test]
fn test_and_then_pattern() {
    let result: Result<i32> = Ok(5);
    let chained = result.map(|x| x * 2);

    assert_eq!(chained.expect("Test setup failed"), 10);
}

// ==================== ERROR RECOVERY TESTS ====================

#[test]
fn test_error_recovery_with_default() {
    let error = NestGateError::storage_error("config default not found");

    // Test error recovery with unwrap_or on a real Result
    let fallback_fn = || -> Result<String> { Err(error) };
    let value = fallback_fn().unwrap_or_else(|_| "default_value".to_string());
    assert_eq!(value, "default_value");
}

#[test]
fn test_error_recovery_with_fallback() {
    /// Primary Source
    fn primary_source() -> Result<i32> {
        Err(NestGateError::network_error("fetch from primary failed"))
    }

    /// Fallback Source
    fn fallback_source() -> Result<i32> {
        Ok(42)
    }

    let value = primary_source().or_else(|_| fallback_source()).unwrap();
    assert_eq!(value, 42);
}
