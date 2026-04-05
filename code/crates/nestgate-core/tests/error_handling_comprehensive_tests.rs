// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
#![expect(
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

//! Comprehensive error handling tests
//!
//! Tests for proper error propagation and handling patterns.

#[cfg(test)]
mod error_handling_comprehensive_tests {
    use nestgate_core::Result;
    use nestgate_core::error::NestGateError;

    #[test]
    fn test_error_creation_configuration() {
        let error = NestGateError::configuration_error("test_field", "test message");
        assert!(error.to_string().contains("test"));
    }

    #[test]
    fn test_error_creation_validation() {
        let error = NestGateError::validation_error("validation failed");
        assert!(error.to_string().contains("validation"));
    }

    #[test]
    fn test_error_propagation() {
        fn inner() -> Result<i32> {
            Err(NestGateError::network_error("connection failed"))
        }

        fn outer() -> Result<i32> {
            inner()?;
            Ok(42)
        }

        let result = outer();
        assert!(result.is_err());
    }

    #[test]
    fn test_result_chaining() {
        fn step1() -> Result<i32> {
            Ok(10)
        }

        fn step2(x: i32) -> Result<i32> {
            Ok(x * 2)
        }

        let result = step1().and_then(step2);
        assert_eq!(result.unwrap(), 20);
    }

    #[test]
    fn test_error_context_preservation() {
        let error = NestGateError::storage_error("disk full");
        let error_string = error.to_string();
        assert!(error_string.contains("disk") || error_string.contains("Storage"));
    }

    #[test]
    fn test_multiple_error_types() {
        let config_err = NestGateError::configuration_error("field", "invalid");
        let network_err = NestGateError::network_error("timeout");
        let storage_err = NestGateError::storage_error("not found");

        assert!(!config_err.to_string().is_empty());
        assert!(!network_err.to_string().is_empty());
        assert!(!storage_err.to_string().is_empty());
    }

    #[test]
    fn test_result_ok_case() {
        // Test Ok variant handling (using match to avoid clippy literal unwrap warning)
        let result: Result<String> = Ok("success".to_string());
        assert!(result.is_ok());
        match result {
            Ok(val) => assert_eq!(val, "success"),
            Err(_) => panic!("Expected Ok, got Err"),
        }
    }

    #[test]
    fn test_result_err_case() {
        let result: Result<i32> = Err(NestGateError::validation_error("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_error_from_string() {
        let error = NestGateError::from("simple error message");
        assert!(error.to_string().contains("error"));
    }

    #[test]
    fn test_option_to_result_conversion() {
        let some_value: Option<i32> = Some(42);
        let result: Result<i32> =
            some_value.ok_or_else(|| NestGateError::validation_error("value missing"));
        assert!(matches!(result, Ok(42)));

        let none_value: Option<i32> = None;
        let result: Result<i32> =
            none_value.ok_or_else(|| NestGateError::validation_error("value missing"));
        assert!(result.is_err());
    }

    #[test]
    fn test_map_err_transformation() {
        let result: std::result::Result<i32, String> = Err("parse error".to_string());
        let transformed: Result<i32> =
            result.map_err(|e| NestGateError::validation_error(format!("Failed: {}", e)));
        assert!(transformed.is_err());
    }

    #[test]
    fn test_result_unwrap_or() {
        // Test unwrap_or with dynamic results (not literal Ok/Err)
        fn get_ok_result() -> Result<i32> {
            Ok(42)
        }
        fn get_err_result() -> Result<i32> {
            Err(NestGateError::network_error("test"))
        }

        assert_eq!(get_ok_result().unwrap_or(0), 42);
        assert_eq!(get_err_result().unwrap_or(100), 100);
    }

    #[test]
    fn test_result_unwrap_or_else() {
        // Test unwrap_or with dynamic error
        fn get_error_result() -> Result<i32> {
            Err(NestGateError::validation_error("test"))
        }
        let value = get_error_result().unwrap_or(999);
        assert_eq!(value, 999);
    }

    #[test]
    fn test_early_return_pattern() {
        fn validate_and_process(value: i32) -> Result<i32> {
            if value < 0 {
                return Err(NestGateError::validation_error("negative value"));
            }
            if value > 100 {
                return Err(NestGateError::validation_error("value too large"));
            }
            Ok(value * 2)
        }

        assert!(validate_and_process(-1).is_err());
        assert!(validate_and_process(101).is_err());
        assert_eq!(validate_and_process(50).unwrap(), 100);
    }

    #[test]
    fn test_nested_results() {
        fn outer() -> Result<Result<i32>> {
            Ok(Ok(42))
        }

        let result = outer();
        assert!(result.is_ok());
        let inner = result.unwrap();
        assert_eq!(inner.unwrap(), 42);
    }

    #[test]
    fn test_collect_results() {
        let results: Vec<Result<i32>> = vec![Ok(1), Ok(2), Ok(3)];
        let collected: Result<Vec<i32>> = results.into_iter().collect();
        assert_eq!(collected.unwrap(), vec![1, 2, 3]);

        let results_with_err: Vec<Result<i32>> =
            vec![Ok(1), Err(NestGateError::validation_error("test")), Ok(3)];
        let collected: Result<Vec<i32>> = results_with_err.into_iter().collect();
        assert!(collected.is_err());
    }

    #[test]
    fn test_timeout_error() {
        use std::time::Duration;
        let error = NestGateError::timeout_error("operation", Duration::from_secs(5));
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_internal_error() {
        let error = NestGateError::internal_error("unexpected state", "test_module");
        assert!(error.to_string().contains("internal") || error.to_string().contains("Internal"));
    }

    #[test]
    fn test_security_error() {
        let error = NestGateError::security("authentication failed");
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_api_error() {
        let error = NestGateError::api_error("invalid request");
        assert!(error.to_string().contains("request") || error.to_string().contains("API"));
    }
}

// NOTE: Re-enable when utils::safe_operations is properly exported at crate root
// Currently blocked on module structure refactoring
// Tracking: These tests validate safe operations but module is not public
// Target: v0.2.0 - Complete utils module reorganization
/*
#[cfg(test)]
mod safe_operations_integration_tests {
    use nestgate_core::utils::safe_operations::*;
    use nestgate_core::Result;

    #[test]
    fn test_parse_env_var_integration() {
        nestgate_core::env_process::set_var("TEST_VALUE", "12345");
        let value: Result<i32> = parse_env_var("TEST_VALUE");
        assert_eq!(value.unwrap(), 12345);
        nestgate_core::env_process::remove_var("TEST_VALUE");
    }

    #[test]
    fn test_safe_collection_operations() {
        let data = vec![10, 20, 30, 40, 50];

        // Standard Rust methods are already safe
        assert_eq!(*data.get(0).unwrap(), 10);
        assert_eq!(*data.first().unwrap(), 10);
        assert_eq!(*data.last().unwrap(), 50);
        assert!(data.get(100).is_none());
    }

    #[test]
    fn test_safe_string_parsing() {
        // Test successful parsing
        let result: std::result::Result<u16, _> = "8080".parse();
        assert_eq!(result.unwrap(), 8080);

        // Test failed parsing
        let result: std::result::Result<u16, _> = "invalid".parse();
        assert!(result.is_err());
    }
}
*/
