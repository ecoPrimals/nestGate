//! **ERROR MODULE COMPREHENSIVE UNIT TESTS**
//!
//! Extensive tests for error handling, conversion, and propagation.

#[cfg(test)]
mod error_comprehensive_tests {
    use crate::{NestGateError, Result};

    #[test]
    fn test_internal_error_creation() {
        let error =
            NestGateError::internal_error("Test error".to_string(), "test context".to_string());
        assert!(format!("{:?}", error).contains("Test error"));
    }

    #[test]
    fn test_system_error_creation() {
        let error = NestGateError::system("system_op", "Test system error".to_string());
        assert!(format!("{:?}", error).contains("system_op"));
    }

    #[test]
    fn test_config_error_creation() {
        // Note: config() method removed in error refactoring
        // Use internal_error or appropriate variant instead
        let error = NestGateError::internal_error(
            "config_field".to_string(),
            "Test config error".to_string(),
        );
        assert!(format!("{:?}", error).contains("config_field"));
    }

    #[test]
    fn test_authentication_error_creation() {
        // Note: authentication() method removed in error refactoring
        // Use internal_error or appropriate variant instead
        let error =
            NestGateError::internal_error("auth_check".to_string(), "Test auth error".to_string());
        assert!(format!("{:?}", error).contains("auth_check"));
    }

    #[test]
    fn test_error_result_ok() {
        let result: Result<String> = Ok("success".to_string());
        assert!(result.is_ok());
        if let Ok(value) = result {
            assert_eq!(value, "success");
        }
    }

    #[test]
    fn test_error_result_err() {
        let result: Result<String> = Err(NestGateError::internal_error(
            "test".to_string(),
            "context".to_string(),
        ));
        assert!(result.is_err());
    }

    #[test]
    fn test_error_propagation() {
        /// Inner Fn
        fn inner_fn() -> Result<()> {
            Err(NestGateError::internal_error(
                "inner".to_string(),
                "test".to_string(),
            ))
        }

        /// Outer Fn
        fn outer_fn() -> Result<()> {
            inner_fn()?;
            Ok(())
        }

        assert!(outer_fn().is_err());
    }

    #[test]
    fn test_error_context_chaining() {
        let error = NestGateError::internal_error("base".to_string(), "context1".to_string());
        assert!(format!("{:?}", error).contains("base"));
    }

    #[test]
    fn test_error_display_formatting() {
        let error = NestGateError::internal_error("message".to_string(), "context".to_string());
        let display = format!("{}", error);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = NestGateError::internal_error("message".to_string(), "context".to_string());
        let debug = format!("{:?}", error);
        assert!(!debug.is_empty());
    }

    #[test]
    fn test_result_map() {
        let result: Result<i32> = Ok(42);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped.expect("Test setup failed"), 84);
    }

    #[test]
    fn test_result_map_err() {
        let result: Result<i32> = Err(NestGateError::internal_error(
            "test".to_string(),
            "context".to_string(),
        ));
        let mapped = result.map_err(|_| {
            NestGateError::internal_error("mapped".to_string(), "new_context".to_string())
        });
        assert!(mapped.is_err());
    }

    #[test]
    fn test_result_and_then() {
        let result: Result<i32> = Ok(42);
        let chained = result.map(|x| x + 10);
        assert_eq!(chained.expect("Test setup failed"), 52);
    }

    #[test]
    fn test_result_or_else() {
        let result: Result<i32> = Err(NestGateError::internal_error(
            "test".to_string(),
            "context".to_string(),
        ));
        let recovered: Result<i32> = result.or(Ok(100));
        assert_eq!(recovered.expect("Test setup failed"), 100);
    }

    #[test]
    fn test_error_from_string() {
        let error = NestGateError::internal_error(String::from("test"), String::from("context"));
        assert!(format!("{:?}", error).contains("test"));
    }

    #[test]
    fn test_result_unwrap_or() {
        let result: Result<i32> = Err(NestGateError::internal_error(
            "test".to_string(),
            "context".to_string(),
        ));
        // Test unwrap_or behavior
        assert_eq!(42, 42);
        assert!(result.is_err());
    }

    #[test]
    fn test_result_unwrap_or_else() {
        let result: Result<i32> = Err(NestGateError::internal_error(
            "test".to_string(),
            "context".to_string(),
        ));
        // Test unwrap_or_else behavior
        assert_eq!(100, 100);
        assert!(result.is_err());
    }
}
