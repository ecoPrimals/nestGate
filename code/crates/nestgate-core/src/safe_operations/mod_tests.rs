// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for safe_operations module functions
//!
//! Tests for safe adapter initialization and connection pool operations.

use super::*;
use crate::error::NestGateError;

// ==================== SAFE ADAPTER INIT TESTS ====================

#[test]
fn test_safe_adapter_init_success() {
    let init_result: Result<String> = Ok("adapter".to_string());
    let result = safe_adapter_init(init_result, "TestAdapter");

    assert!(result.is_ok());
    let adapter = result.unwrap();
    assert!(adapter.is_some());
    assert_eq!(adapter.unwrap(), "adapter");
}

#[test]
fn test_safe_adapter_init_failure_returns_none() {
    let init_result: Result<String> = Err(NestGateError::internal_error(
        "Initialization failed",
        "test",
    ));
    let result = safe_adapter_init(init_result, "TestAdapter");

    assert!(result.is_ok());
    let adapter = result.unwrap();
    assert!(adapter.is_none()); // Should return None on failure, not error
}

#[test]
fn test_safe_adapter_init_with_complex_type() {
    #[derive(Debug, Clone, PartialEq)]
    struct ComplexAdapter {
        id: u32,
        name: String,
    }

    let adapter = ComplexAdapter {
        id: 1,
        name: "test".to_string(),
    };
    let init_result: Result<ComplexAdapter> = Ok(adapter.clone());
    let result = safe_adapter_init(init_result, "ComplexAdapter");

    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap(), adapter);
}

#[test]
fn test_safe_adapter_init_multiple_adapters() {
    // Test initializing multiple adapters
    let adapter1 = safe_adapter_init(Ok("adapter1".to_string()), "Adapter1");
    let adapter2 = safe_adapter_init(Ok("adapter2".to_string()), "Adapter2");
    let adapter3 = safe_adapter_init::<String>(
        Err(NestGateError::internal_error("failed", "test")),
        "Adapter3",
    );

    assert!(adapter1.unwrap().is_some());
    assert!(adapter2.unwrap().is_some());
    assert!(adapter3.unwrap().is_none());
}

#[test]
fn test_safe_adapter_init_with_unit_type() {
    let init_result: Result<()> = Ok(());
    let result = safe_adapter_init(init_result, "UnitAdapter");

    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

// ==================== SAFE CONNECTION POOL RETURN TESTS ====================

#[test]
fn test_safe_connection_pool_return_success() {
    let operation_result: Result<String> = Ok("connection".to_string());
    let result = safe_connection_pool_return(operation_result, "get_connection");

    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(inner.is_ok());
    assert_eq!(inner.unwrap(), "connection");
}

#[test]
fn test_safe_connection_pool_return_failure() {
    let operation_result: Result<String> =
        Err(NestGateError::internal_error("Connection failed", "pool"));
    let result = safe_connection_pool_return(operation_result, "get_connection");

    assert!(result.is_ok()); // Outer Result should be Ok
    let inner = result.unwrap();
    assert!(inner.is_err()); // Inner Result should be Err
}

#[test]
fn test_safe_connection_pool_return_with_complex_type() {
    #[derive(Debug, PartialEq)]
    struct Connection {
        id: u64,
        active: bool,
    }

    let conn = Connection {
        id: 123,
        active: true,
    };
    let operation_result: Result<Connection> = Ok(conn);
    let result = safe_connection_pool_return(operation_result, "acquire");

    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(inner.is_ok());
    assert_eq!(inner.unwrap().id, 123);
}

#[test]
fn test_safe_connection_pool_return_multiple_operations() {
    // Test multiple pool operations
    let op1 = safe_connection_pool_return(Ok(1), "op1");
    let op2 = safe_connection_pool_return(Ok(2), "op2");
    let op3 = safe_connection_pool_return::<i32>(
        Err(NestGateError::internal_error("failed", "test")),
        "op3",
    );

    assert!(op1.unwrap().is_ok());
    assert!(op2.unwrap().is_ok());
    assert!(op3.unwrap().is_err());
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_adapter_and_pool_together() {
    // Simulate adapter initialization
    let adapter = safe_adapter_init(Ok("test_adapter".to_string()), "TestAdapter");
    assert!(adapter.is_ok());

    // Simulate connection pool operation
    let connection = safe_connection_pool_return(Ok("connection".to_string()), "get_conn");
    assert!(connection.is_ok());
}

#[test]
fn test_graceful_degradation_pattern() {
    // Test that we can continue even if some adapters fail
    let critical_adapter = safe_adapter_init(Ok("critical".to_string()), "Critical");
    let optional_adapter = safe_adapter_init::<String>(
        Err(NestGateError::internal_error("optional failed", "test")),
        "Optional",
    );

    assert!(critical_adapter.unwrap().is_some());
    assert!(optional_adapter.unwrap().is_none()); // Optional can be None
}

// ==================== ERROR HANDLING TESTS ====================

#[test]
fn test_adapter_init_various_errors() {
    let errors = vec![
        NestGateError::internal_error("error1", "test"),
        NestGateError::system("error2", "test"),
        NestGateError::configuration_error("field", "error3"),
    ];

    for (i, error) in errors.into_iter().enumerate() {
        let result = safe_adapter_init::<String>(Err(error), &format!("Adapter{}", i));
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}

#[test]
fn test_pool_return_various_errors() {
    let errors = vec![
        NestGateError::internal_error("pool error 1", "test"),
        NestGateError::system("pool error 2", "test"),
    ];

    for (i, error) in errors.into_iter().enumerate() {
        let result = safe_connection_pool_return::<i32>(Err(error), &format!("op{}", i));
        assert!(result.is_ok());
        assert!(result.unwrap().is_err());
    }
}

// ==================== EDGE CASES ====================

#[test]
fn test_adapter_init_empty_name() {
    let result = safe_adapter_init(Ok(42), "");
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_pool_return_empty_operation() {
    let result = safe_connection_pool_return(Ok(42), "");
    assert!(result.is_ok());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_adapter_init_long_name() {
    let long_name = "A".repeat(1000);
    let result = safe_adapter_init(Ok(true), &long_name);
    assert!(result.is_ok());
}

#[test]
fn test_adapter_init_special_characters() {
    let result = safe_adapter_init(Ok(1), "Adapter-123_Test@v1.0");
    assert!(result.is_ok());
}

// ==================== TYPE VARIETY TESTS ====================

#[test]
fn test_adapter_init_with_option() {
    let result = safe_adapter_init(Ok(Some(42)), "OptionAdapter");
    assert!(result.is_ok());
    let outer = result.unwrap();
    assert!(outer.is_some());
    assert_eq!(outer.unwrap(), Some(42));
}

#[test]
fn test_adapter_init_with_vec() {
    let result = safe_adapter_init(Ok(vec![1, 2, 3]), "VecAdapter");
    assert!(result.is_ok());
    let outer = result.unwrap();
    assert!(outer.is_some());
    assert_eq!(outer.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_pool_return_with_tuple() {
    let result = safe_connection_pool_return(Ok((1, "data")), "tuple_op");
    assert!(result.is_ok());
    let inner = result.unwrap();
    assert!(inner.is_ok());
    assert_eq!(inner.unwrap(), (1, "data"));
}

// ==================== PERFORMANCE TESTS ====================

#[test]
fn test_adapter_init_performance() {
    for i in 0..100 {
        let result = safe_adapter_init(Ok(i), "PerformanceAdapter");
        assert!(result.is_ok());
    }
}

#[test]
fn test_pool_return_performance() {
    for i in 0..100 {
        let result = safe_connection_pool_return(Ok(i), "perf_op");
        assert!(result.is_ok());
    }
}

// ==================== REAL-WORLD SCENARIOS ====================

#[test]
fn test_database_adapter_scenario() {
    struct DatabaseAdapter {
        _connected: bool,
        _url: String,
    }

    use crate::constants::hardcoding::addresses;
    let db_url = format!("postgresql://{}:5432/db", addresses::LOCALHOST_NAME);
    let db_result: Result<DatabaseAdapter> = Ok(DatabaseAdapter {
        _connected: true,
        _url: db_url,
    });

    let adapter = safe_adapter_init(db_result, "DatabaseAdapter");
    assert!(adapter.is_ok());
    assert!(adapter.unwrap().is_some());
}

#[test]
fn test_cache_adapter_scenario() {
    let cache_result: Result<Vec<u8>> =
        Err(NestGateError::internal_error("Cache unavailable", "cache"));

    let adapter = safe_adapter_init(cache_result, "CacheAdapter");
    assert!(adapter.is_ok());
    // Cache is optional, so None is acceptable
    assert!(adapter.unwrap().is_none());
}

#[test]
fn test_connection_pool_scenario() {
    struct PoolConnection {
        _id: usize,
        _in_use: bool,
    }

    let pool_op: Result<PoolConnection> = Ok(PoolConnection {
        _id: 1,
        _in_use: true,
    });

    let result = safe_connection_pool_return(pool_op, "acquire_connection");
    assert!(result.is_ok());
    assert!(result.unwrap().is_ok());
}
