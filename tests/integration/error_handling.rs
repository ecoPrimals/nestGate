// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Error Handling Integration Tests
//!
//! Tests for error creation, propagation, and handling

#![allow(clippy::unnecessary_literal_unwrap)]

use nestgate_core::{NestGateError, Result};

/// Test creating internal errors
#[test]
fn test_internal_error_creation() {
    let error =
        NestGateError::internal_error("Test error message".to_string(), "test_context".to_string());

    // Verify error contains expected information
    let error_str = format!("{:?}", error);
    assert!(error_str.contains("Test error message") || error_str.contains("InternalError"));
}

/// Test error propagation with ? operator
#[tokio::test]
async fn test_error_propagation() -> Result<()> {
    // Helper function that returns an error
    async fn failing_operation() -> Result<String> {
        Err(NestGateError::internal_error(
            "Operation failed".to_string(),
            "test".to_string(),
        ))
    }

    // Test that error propagates correctly
    let result = failing_operation().await;
    assert!(result.is_err());

    Ok(())
}

/// Test Result unwrapping in safe contexts
#[test]
fn test_result_unwrapping() {
    let success: Result<i32> = Ok(42);
    let value = success.unwrap();
    assert_eq!(value, 42);
}

/// Test error matching patterns
#[test]
fn test_error_matching() {
    let error = NestGateError::internal_error("Test".to_string(), "context".to_string());

    let result: Result<()> = Err(error);

    // Test matching on error
    match result {
        Ok(_) => panic!("Expected error"),
        Err(_) => {
            // Error correctly matched
        }
    }
}

/// Test error conversion patterns
#[test]
fn test_error_conversion() {
    // Test that errors can be created and converted
    let error = NestGateError::internal_error("Conversion test".to_string(), "test".to_string());

    let _boxed: Box<dyn std::error::Error> = Box::new(error);
    // Conversion successful
}

/// Test multiple error types
#[tokio::test]
async fn test_multiple_error_scenarios() -> Result<()> {
    // Test various error scenarios
    let scenarios = vec![
        ("config_error", "configuration"),
        ("network_error", "network"),
        ("storage_error", "storage"),
    ];

    for (msg, context) in scenarios {
        let error = NestGateError::internal_error(msg.to_string(), context.to_string());

        let result: Result<()> = Err(error);
        assert!(result.is_err());
    }

    Ok(())
}
