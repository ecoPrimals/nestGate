// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for result types and extensions
//! Added: November 21, 2025 - Coverage Expansion
//!
//! Target: Complete coverage of Result types, extensions, and macros

#[cfg(test)]
mod result_types_tests {
    use crate::error::NestGateError;
    use crate::result_types::*;
    use crate::{network_result, storage_result, validation_result};
    use std::io;

    // ==================== Result Type Alias Tests ====================

    #[test]
    fn test_result_type_ok() {
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_type_err() {
        let result: Result<i32> = Err(NestGateError::validation("error"));
        assert!(result.is_err());
    }

    #[test]
    fn test_canonical_result_type() {
        let result: CanonicalResult<String> = Ok("success".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[test]
    fn test_void_result_ok() {
        let result: VoidResult = Ok(());
        assert!(result.is_ok());
    }

    #[test]
    fn test_void_result_err() {
        let result: VoidResult = Err(NestGateError::validation("failed"));
        assert!(result.is_err());
    }

    #[test]
    fn test_test_result_type() {
        let result: TestResult<String> = Ok("test passed".to_string());
        assert!(result.is_ok());
    }

    // ==================== Async Result Tests ====================

    #[tokio::test]
    async fn test_async_result_ok() {
        /// Async Operation
        async fn async_operation() -> Result<String> {
            Ok("async success".to_string())
        }

        let result = async_operation().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "async success");
    }

    #[tokio::test]
    async fn test_async_result_err() {
        /// Async Operation
        async fn async_operation() -> Result<String> {
            Err(NestGateError::validation("async error"))
        }

        let result = async_operation().await;
        assert!(result.is_err());
    }

    // ==================== Function Pointer Types ====================

    #[test]
    fn test_connection_factory_type() {
        use std::sync::Arc;

        let factory: ConnectionFactory<String> = Arc::new(|| Ok("connected".to_string()));

        let result = factory();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "connected");
    }

    #[test]
    fn test_connection_factory_failure() {
        use std::sync::Arc;

        let factory: ConnectionFactory<i32> =
            Arc::new(|| Err(NestGateError::network_error("connection failed")));

        let result = factory();
        assert!(result.is_err());
    }

    #[test]
    fn test_health_check_fn_type() {
        use std::sync::Arc;

        let health_check: HealthCheckFn<String> = Arc::new(|_s: &String| Ok(()));
        let test_data = String::from("test");
        let result = health_check(&test_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_health_check_fn_failure() {
        use std::sync::Arc;

        let health_check: HealthCheckFn<i32> = Arc::new(|n: &i32| {
            if *n < 0 {
                Err(NestGateError::validation("negative not allowed"))
            } else {
                Ok(())
            }
        });

        assert!(health_check(&10).is_ok());
        assert!(health_check(&-5).is_err());
    }

    #[test]
    fn test_validator_fn_type() {
        let validator: ValidatorFn<String> = Box::new(|s: &String| {
            if s.is_empty() {
                Err(NestGateError::validation("String cannot be empty"))
            } else {
                Ok(())
            }
        });

        assert!(validator(&"test".to_string()).is_ok());
        assert!(validator(&String::new()).is_err());
    }

    #[test]
    fn test_validator_fn_with_number() {
        let validator: ValidatorFn<i32> = Box::new(|n: &i32| {
            if *n >= 0 && *n <= 100 {
                Ok(())
            } else {
                Err(NestGateError::validation(
                    "Number must be between 0 and 100",
                ))
            }
        });

        assert!(validator(&50).is_ok());
        assert!(validator(&0).is_ok());
        assert!(validator(&100).is_ok());
        assert!(validator(&-1).is_err());
        assert!(validator(&101).is_err());
    }

    // ==================== ResultExt Trait Tests ====================

    #[test]
    fn test_to_canonical_from_io_error() {
        let io_result: std::result::Result<String, io::Error> =
            Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));

        let canonical: Result<String> = io_result.to_canonical();
        assert!(canonical.is_err());
    }

    #[test]
    fn test_to_canonical_success() {
        let ok_result: std::result::Result<i32, io::Error> = Ok(42);
        let canonical: Result<i32> = ok_result.to_canonical();
        assert!(canonical.is_ok());
        assert_eq!(canonical.unwrap(), 42);
    }

    #[test]
    fn test_with_context() {
        let io_result: std::result::Result<String, io::Error> = Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "access denied",
        ));

        let with_context = io_result.with_context(|| "Failed to read config file".to_string());
        assert!(with_context.is_err());
    }

    #[test]
    fn test_with_context_success() {
        let ok_result: std::result::Result<i32, io::Error> = Ok(100);
        let with_context = ok_result.with_context(|| "Should not be called".to_string());
        assert!(with_context.is_ok());
        assert_eq!(with_context.unwrap(), 100);
    }

    // ==================== Macro Tests ====================

    #[test]
    fn test_validation_result_macro() {
        let result: std::result::Result<i32, io::Error> = Ok(42);
        let validated = validation_result!(result);
        assert!(validated.is_ok());
        assert_eq!(validated.unwrap(), 42);
    }

    #[test]
    fn test_validation_result_macro_error() {
        let result: std::result::Result<i32, io::Error> =
            Err(io::Error::new(io::ErrorKind::InvalidInput, "bad input"));
        let validated = validation_result!(result);
        assert!(validated.is_err());
    }

    #[test]
    fn test_network_result_macro() {
        let result: std::result::Result<String, io::Error> = Ok("data".to_string());
        let network_res = network_result!(result);
        assert!(network_res.is_ok());
        assert_eq!(network_res.unwrap(), "data");
    }

    #[test]
    fn test_network_result_macro_error() {
        let result: std::result::Result<String, io::Error> =
            Err(io::Error::new(io::ErrorKind::ConnectionRefused, "refused"));
        let network_res = network_result!(result);
        assert!(network_res.is_err());
    }

    #[test]
    fn test_storage_result_macro() {
        let result: std::result::Result<Vec<u8>, io::Error> = Ok(vec![1, 2, 3]);
        let storage_res = storage_result!(result);
        assert!(storage_res.is_ok());
        assert_eq!(storage_res.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_storage_result_macro_error() {
        let result: std::result::Result<Vec<u8>, io::Error> =
            Err(io::Error::new(io::ErrorKind::NotFound, "not found"));
        let storage_res = storage_result!(result);
        assert!(storage_res.is_err());
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_result_chain() {
        /// Step1
        fn step1() -> Result<i32> {
            Ok(10)
        }

        /// Step2
        fn step2(x: i32) -> Result<i32> {
            Ok(x * 2)
        }

        /// Step3
        fn step3(x: i32) -> Result<String> {
            Ok(format!("Result: {}", x))
        }

        let result = step1().and_then(step2).and_then(step3);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Result: 20");
    }

    #[test]
    fn test_result_chain_with_early_error() {
        /// Step1
        fn step1() -> Result<i32> {
            Ok(10)
        }

        /// Step2
        fn step2(_x: i32) -> Result<i32> {
            Err(NestGateError::validation("step2 failed"))
        }

        /// Step3
        fn step3(_x: i32) -> Result<String> {
            Ok("Should not reach".to_string())
        }

        let result = step1().and_then(step2).and_then(step3);

        assert!(result.is_err());
    }

    #[test]
    fn test_result_map() {
        let result: Result<i32> = Ok(42);
        let mapped = result.map(|x| x * 2);
        assert!(mapped.is_ok());
        assert_eq!(mapped.unwrap(), 84);
    }

    #[test]
    fn test_result_map_error() {
        let result: Result<i32> = Err(NestGateError::validation("error"));
        let mapped = result.map(|x| x * 2);
        assert!(mapped.is_err());
    }

    #[test]
    fn test_result_map_err() {
        let result: Result<i32> = Err(NestGateError::validation("original"));
        let mapped = result.map_err(|_| NestGateError::validation("mapped"));
        assert!(mapped.is_err());
    }

    #[test]
    fn test_result_or_else() {
        let result: Result<i32> = Err(NestGateError::validation("error"));
        let recovered: Result<i32> = result.or(Ok(100));
        assert!(recovered.is_ok());
        assert_eq!(recovered.unwrap(), 100);
    }

    #[test]
    fn test_result_unwrap_or() {
        let result: Result<i32> = Err(NestGateError::validation("error"));
        let value = result.unwrap_or(999);
        assert_eq!(value, 999);
    }

    #[test]
    fn test_result_unwrap_or_else() {
        let result: Result<i32> = Err(NestGateError::validation("error"));
        let value = result.unwrap_or(888);
        assert_eq!(value, 888);
    }

    // ==================== Complex Type Tests ====================

    #[test]
    fn test_result_with_vec() {
        let result: Result<Vec<String>> = Ok(vec!["a".to_string(), "b".to_string()]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_result_with_option() {
        let result: Result<Option<i32>> = Ok(Some(42));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(42));
    }

    #[test]
    fn test_result_with_nested_result() {
        let result: Result<Result<i32>> = Ok(Ok(42));
        assert!(result.is_ok());
        let inner = result.unwrap();
        assert!(inner.is_ok());
        assert_eq!(inner.unwrap(), 42);
    }

    #[test]
    fn test_canonical_result_with_custom_struct() {
        #[derive(Debug, PartialEq)]
        struct CustomData {
            value: i32,
            name: String,
        }

        let result: CanonicalResult<CustomData> = Ok(CustomData {
            value: 42,
            name: "test".to_string(),
        });

        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.value, 42);
        assert_eq!(data.name, "test");
    }

    // ==================== Error Conversion Tests ====================

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::other("test error");
        let result: Result<()> = Err(io_err.into());
        assert!(result.is_err());
    }

    #[test]
    fn test_string_error_conversion() {
        let str_err = "string error";
        let result: Result<()> = Err(NestGateError::validation(str_err));
        assert!(result.is_err());
    }

    #[test]
    fn test_nested_error_handling() {
        /// Inner Operation
        fn inner_operation() -> std::result::Result<i32, io::Error> {
            Err(io::Error::other("inner error"))
        }

        /// Outer Operation
        fn outer_operation() -> Result<i32> {
            inner_operation().to_canonical()
        }

        let result = outer_operation();
        assert!(result.is_err());
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_result_with_empty_string() {
        let result: Result<String> = Ok(String::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_result_with_zero() {
        let result: Result<i32> = Ok(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_result_with_none() {
        let result: Result<Option<i32>> = Ok(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_void_result_multiple_operations() {
        /// Op1
        fn op1() -> VoidResult {
            Ok(())
        }
        /// Op2
        fn op2() -> VoidResult {
            Ok(())
        }
        /// Op3
        fn op3() -> VoidResult {
            Ok(())
        }

        let result = op1().and_then(|_| op2()).and_then(|_| op3());
        assert!(result.is_ok());
    }

    #[test]
    fn test_void_result_with_early_failure() {
        /// Op1
        fn op1() -> VoidResult {
            Ok(())
        }
        /// Op2
        fn op2() -> VoidResult {
            Err(NestGateError::validation("failed"))
        }
        /// Op3
        fn op3() -> VoidResult {
            Ok(())
        }

        let result = op1().and_then(|_| op2()).and_then(|_| op3());
        assert!(result.is_err());
    }
}
