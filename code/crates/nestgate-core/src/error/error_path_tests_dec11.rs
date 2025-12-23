//! Strategic Error Path Tests - December 11, 2025
//!
//! Tests for error handling edge cases and propagation scenarios

#[cfg(test)]
mod error_strategic_tests {
    use crate::error::NestGateError;
    use std::io;

    // ==================== Error Construction Tests ====================

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error = NestGateError::from(io_err);

        // Verify error was created successfully
        let display = format!("{}", error);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_error_display_formatting() {
        let error = NestGateError::configuration_error("test_field", "test error message");
        let display_str = format!("{}", error);
        assert!(!display_str.is_empty());
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = NestGateError::configuration_error("test_field", "test error");
        let debug_str = format!("{:?}", error);
        assert!(!debug_str.is_empty());
    }

    // ==================== Error Propagation Tests ====================

    #[test]
    fn test_result_chain_ok() {
        fn inner() -> Result<i32, NestGateError> {
            Ok(42)
        }

        fn outer() -> Result<i32, NestGateError> {
            let value = inner()?;
            Ok(value * 2)
        }

        let result = outer();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 84);
    }

    #[test]
    fn test_result_chain_error_propagation() {
        fn inner() -> Result<i32, NestGateError> {
            Err(NestGateError::configuration_error("inner", "inner error"))
        }

        fn outer() -> Result<i32, NestGateError> {
            let value = inner()?;
            Ok(value * 2)
        }

        let result = outer();
        assert!(result.is_err());
    }

    // ==================== Error Type Variants Tests ====================

    #[test]
    fn test_configuration_error_creation() {
        let error = NestGateError::configuration_error("field", "config error");
        let display = format!("{}", error);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_multiple_error_types() {
        let errors = [
            NestGateError::configuration_error("config", "config error"),
            NestGateError::from(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "access denied",
            )),
        ];

        assert_eq!(errors.len(), 2);
    }

    // ==================== Error Message Tests ====================

    #[test]
    fn test_error_message_with_special_characters() {
        let messages = vec![
            "error: special!@#$%^&*()",
            "error with\nnewline",
            "error with\ttab",
            "error with 'quotes'",
            "error with \"double quotes\"",
        ];

        for msg in messages {
            let error = NestGateError::configuration_error("field", msg);
            let display = format!("{}", error);
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn test_error_message_unicode() {
        let messages = vec![
            "错误信息",  // Chinese
            "エラー",    // Japanese
            "오류",      // Korean
            "ошибка",    // Russian
            "Fehler 🔥", // Emoji
        ];

        for msg in messages {
            let error = NestGateError::configuration_error("field", msg);
            let display = format!("{}", error);
            assert!(!display.is_empty());
        }
    }

    // ==================== Error Context Tests ====================

    #[test]
    fn test_nested_error_contexts() {
        fn level3() -> Result<(), NestGateError> {
            Err(NestGateError::configuration_error(
                "level3",
                "level 3 error",
            ))
        }

        fn level2() -> Result<(), NestGateError> {
            level3()?;
            Ok(())
        }

        fn level1() -> Result<(), NestGateError> {
            level2()?;
            Ok(())
        }

        let result = level1();
        assert!(result.is_err());
    }

    // ==================== Option and Result Interop Tests ====================

    #[test]
    fn test_option_to_result_conversion() {
        fn get_value() -> Option<i32> {
            None
        }

        fn process() -> Result<i32, NestGateError> {
            let value = get_value()
                .ok_or_else(|| NestGateError::configuration_error("value", "value not found"))?;
            Ok(value * 2)
        }

        let result = process();
        assert!(result.is_err());
    }

    #[test]
    fn test_result_ok_or_conversion() {
        let maybe_value: Result<i32, NestGateError> = Ok(42);

        match maybe_value {
            Ok(v) => assert_eq!(v, 42),
            Err(_) => panic!("Expected Ok"),
        }
    }

    // ==================== Error Recovery Tests ====================

    #[test]
    fn test_error_recovery_with_or_else() {
        fn fallible() -> Result<i32, NestGateError> {
            Err(NestGateError::configuration_error("field", "error"))
        }

        let result: Result<i32, NestGateError> = fallible().or(Ok(100));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn test_error_recovery_with_unwrap_or() {
        fn fallible() -> Result<i32, NestGateError> {
            Err(NestGateError::configuration_error("field", "error"))
        }

        let value = fallible().unwrap_or(42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_error_recovery_with_unwrap_or_else() {
        fn fallible() -> Result<i32, NestGateError> {
            Err(NestGateError::configuration_error("field", "error"))
        }

        let value = fallible().unwrap_or(99);
        assert_eq!(value, 99);
    }

    // ==================== Error Collection Tests ====================

    #[test]
    fn test_collect_results_all_ok() {
        let results: Vec<Result<i32, NestGateError>> = vec![Ok(1), Ok(2), Ok(3)];
        let collected: Result<Vec<_>, _> = results.into_iter().collect();

        assert!(collected.is_ok());
        assert_eq!(collected.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_collect_results_with_error() {
        let results: Vec<Result<i32, NestGateError>> = vec![
            Ok(1),
            Err(NestGateError::configuration_error("field", "error")),
            Ok(3),
        ];
        let collected: Result<Vec<_>, _> = results.into_iter().collect();

        assert!(collected.is_err());
    }

    // ==================== Error Comparison Tests ====================

    #[test]
    fn test_error_string_representation() {
        let error = NestGateError::configuration_error("test", "test error");
        let as_string = format!("{}", error);
        assert!(!as_string.is_empty());
    }

    #[test]
    fn test_error_from_io_variants() {
        let io_errors = vec![
            io::Error::new(io::ErrorKind::NotFound, "not found"),
            io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
            io::Error::new(io::ErrorKind::ConnectionRefused, "connection refused"),
            io::Error::new(io::ErrorKind::TimedOut, "timed out"),
        ];

        for io_err in io_errors {
            let nestgate_err = NestGateError::from(io_err);
            let display = format!("{}", nestgate_err);
            assert!(!display.is_empty());
        }
    }
}
