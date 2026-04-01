// SPDX-License-Identifier: AGPL-3.0-only
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

//! Comprehensive tests for NestGateError types
//! Tests error creation, formatting, and type conversions

use nestgate_core::error::NestGateError;
use std::hint::black_box;

#[test]
fn test_internal_error_creation() {
    let error = NestGateError::internal("test error");
    let msg = format!("{}", error);
    assert!(!msg.is_empty());
}

#[test]
fn test_network_error_creation() {
    let error = NestGateError::network_error("connection failed");
    let msg = format!("{}", error);
    assert!(msg.contains("network") || msg.contains("connection"));
}

#[test]
fn test_storage_error_creation() {
    let error = NestGateError::storage_error("disk full");
    let msg = format!("{}", error);
    assert!(!msg.is_empty());
}

#[test]
fn test_configuration_error_creation() {
    let error = NestGateError::configuration_error("invalid config", "config");
    let msg = format!("{}", error);
    assert!(msg.contains("config") || msg.contains("invalid"));
}

#[test]
fn test_not_implemented_error() {
    let error = NestGateError::not_implemented("feature X");
    let msg = format!("{}", error);
    assert!(msg.contains("not") || msg.contains("implemented"));
}

#[test]
fn test_error_debug_format() {
    let error = NestGateError::internal("debug test");
    let debug = format!("{:?}", error);
    assert!(!debug.is_empty());
}

#[test]
fn test_multiple_error_types() {
    let errors = vec![
        NestGateError::internal("error 1"),
        NestGateError::network_error("error 2"),
        NestGateError::storage_error("error 3"),
    ];

    for error in errors {
        let msg = format!("{}", error);
        assert!(!msg.is_empty());
    }
}

#[test]
fn test_error_context_preservation() {
    let error = NestGateError::configuration_error("missing key", "database");
    let msg = format!("{}", error);
    // Error should preserve context
    assert!(msg.len() > 5);
}

#[test]
fn test_result_ok_creation() {
    let result: nestgate_core::Result<i32> = Ok(42);
    assert!(result.is_ok());
    if let Ok(v) = result {
        assert_eq!(v, 42);
    } else {
        panic!("expected Ok");
    }
}

#[test]
fn test_result_err_creation() {
    let result: nestgate_core::Result<i32> = Err(NestGateError::internal("fail"));
    assert!(result.is_err());
}

#[test]
fn test_result_map_operation() {
    let result: nestgate_core::Result<i32> = Ok(5);
    let mapped = result.map(|x| x * 2);
    assert!(mapped.is_ok());
    assert_eq!(mapped.unwrap(), 10);
}

#[test]
fn test_result_and_then_operation() {
    let result: nestgate_core::Result<i32> = Ok(5);
    let chained = result.map(|x| x + 3);
    assert!(chained.is_ok());
    assert_eq!(chained.unwrap(), 8);
}

#[test]
fn test_result_or_else_recovery() {
    let result: nestgate_core::Result<i32> = Err(NestGateError::internal("fail"));
    let recovered: nestgate_core::Result<i32> = result.or(Ok(42));
    assert!(recovered.is_ok());
    assert_eq!(recovered.unwrap(), 42);
}

#[test]
fn test_error_from_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let nest_err: NestGateError = io_err.into();
    let msg = format!("{}", nest_err);
    assert!(!msg.is_empty());
}

#[test]
fn test_question_mark_operator() {
    fn inner() -> nestgate_core::Result<i32> {
        Err(NestGateError::internal("inner error"))
    }

    fn outer() -> nestgate_core::Result<i32> {
        let _value = inner()?;
        Ok(42)
    }

    let result = outer();
    assert!(result.is_err());
}

#[test]
fn test_error_chain_propagation() {
    fn level1() -> nestgate_core::Result<()> {
        Err(NestGateError::storage_error("level1"))
    }

    fn level2() -> nestgate_core::Result<()> {
        level1()?;
        Ok(())
    }

    fn level3() -> nestgate_core::Result<()> {
        level2()?;
        Ok(())
    }

    let result = level3();
    assert!(result.is_err());
}

#[test]
fn test_concurrent_error_handling() {
    use std::sync::Arc;
    use std::thread;

    let error = Arc::new(NestGateError::network_error("shared error"));

    let handles: Vec<_> = (0..5)
        .map(|_| {
            let err = Arc::clone(&error);
            thread::spawn(move || format!("{}", err))
        })
        .collect();

    for handle in handles {
        let msg = handle.join().unwrap();
        assert!(!msg.is_empty());
    }
}

#[test]
fn test_error_equality_via_display() {
    let err1 = NestGateError::internal("same message");
    let err2 = NestGateError::internal("same message");

    // Same error types with same messages should display the same
    assert_eq!(format!("{}", err1), format!("{}", err2));
}

#[test]
fn test_error_inequality_different_types() {
    let err1 = NestGateError::internal("message");
    let err2 = NestGateError::network_error("message");

    // Different error types should display differently
    assert_ne!(format!("{}", err1), format!("{}", err2));
}

#[test]
fn test_result_unwrap_or_default() {
    let err = NestGateError::internal("fail");
    let result: nestgate_core::Result<Vec<String>> = Err(err);
    let value: Vec<String> = black_box(result).unwrap_or_default();
    assert!(value.is_empty());
}

#[test]
fn test_result_unwrap_or_else() {
    let err = NestGateError::internal("fail");
    let result: nestgate_core::Result<i32> = Err(err);
    let value: i32 = black_box(result).unwrap_or(99);
    assert_eq!(value, 99);
}

#[test]
fn test_multiple_sequential_operations() {
    fn op1() -> nestgate_core::Result<i32> {
        Ok(1)
    }
    fn op2(x: i32) -> nestgate_core::Result<i32> {
        Ok(x + 1)
    }
    fn op3(x: i32) -> nestgate_core::Result<i32> {
        Ok(x * 2)
    }

    let result = op1().and_then(op2).and_then(op3);
    assert_eq!(result.unwrap(), 4);
}

#[test]
fn test_error_stops_chain() {
    fn op1() -> nestgate_core::Result<i32> {
        Ok(1)
    }
    fn op2(_: i32) -> nestgate_core::Result<i32> {
        Err(NestGateError::internal("stop"))
    }
    fn op3(_: i32) -> nestgate_core::Result<i32> {
        Ok(999) // Should not execute
    }

    let result = op1().and_then(op2).and_then(op3);
    assert!(result.is_err());
}

#[test]
fn test_error_recovery_pattern() {
    let primary: nestgate_core::Result<i32> = Err(NestGateError::network_error("primary fail"));
    let fallback: nestgate_core::Result<i32> = Ok(42);

    let result = primary.or(fallback);
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_error_message_length() {
    let errors = vec![
        NestGateError::internal("short"),
        NestGateError::network_error("medium length error"),
        NestGateError::storage_error("a very long error message with lots of detail"),
    ];

    for error in errors {
        let msg = format!("{}", error);
        assert!(!msg.is_empty());
        assert!(msg.len() < 1000); // Reasonable max length
    }
}

#[test]
fn test_error_contains_context() {
    let error = NestGateError::configuration_error("missing API_KEY", "environment");
    let msg = format!("{}", error);

    // Error should contain useful context
    assert!(msg.len() > 10);
}

#[test]
fn test_result_type_inference() {
    // Test that Result type inference works
    fn returns_result() -> nestgate_core::Result<String> {
        Ok("success".to_string())
    }

    let result = returns_result();
    assert!(result.is_ok());
}

#[test]
fn test_nested_result_operations() {
    let result: nestgate_core::Result<nestgate_core::Result<i32>> = Ok(Ok(42));
    assert!(result.is_ok());
    if let Ok(inner) = result {
        assert!(inner.is_ok());
    }
}
