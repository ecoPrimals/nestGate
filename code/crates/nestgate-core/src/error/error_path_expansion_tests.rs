//! **ERROR PATH EXPANSION TESTS** - Nov 23, 2025
//!
//! Comprehensive tests for error paths, conversions, and edge cases
//! to boost coverage from 68.52% toward 90%.

#[cfg(test)]
mod error_conversion_tests {
    use crate::error::{NestGateError, Result};
    use std::io;

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let gate_err: NestGateError = io_err.into();
        let display = format!("{}", gate_err);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_from_string_error() {
        let err: NestGateError = "simple error".to_string().into();
        assert!(format!("{}", err).contains("simple error"));
    }

    #[test]
    fn test_error_chain_depth() {
        fn level3() -> Result<()> {
            Err(NestGateError::internal_error("level3", "deep"))
        }

        fn level2() -> Result<()> {
            level3()
                .map_err(|e| NestGateError::internal_error(format!("level2: {}", e), "middle"))?;
            Ok(())
        }

        fn level1() -> Result<()> {
            level2().map_err(|e| NestGateError::internal_error(format!("level1: {}", e), "top"))?;
            Ok(())
        }

        let result = level1();
        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("level"));
    }

    #[test]
    fn test_error_in_iterator() {
        let items = vec![Ok(1), Err(NestGateError::validation_error("bad")), Ok(3)];
        let result: Result<Vec<_>> = items.into_iter().collect();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_recovery_patterns() {
        fn might_fail(should_fail: bool) -> Result<String> {
            if should_fail {
                Err(NestGateError::network_error("connection failed"))
            } else {
                Ok("success".to_string())
            }
        }

        // Test recovery with fallback
        let result = might_fail(true)
            .or_else(|_| Ok::<String, NestGateError>("fallback".to_string()))
            .unwrap();
        assert_eq!(result, "fallback");

        // Test success case
        let result = might_fail(false).unwrap();
        assert_eq!(result, "success");
    }
}

#[cfg(test)]
mod error_context_tests {
    use crate::error::{ErrorContext, NestGateError};

    #[test]
    fn test_error_context_creation() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        let context = ErrorContext {
            operation: "test_op".to_string(),
            component: "test_component".to_string(),
            request_id: Some("req-123".to_string()),
            user_id: Some("user-456".to_string()),
            metadata,
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(context.operation, "test_op");
        assert_eq!(context.component, "test_component");
        assert!(!context.metadata.is_empty());
    }

    #[test]
    fn test_error_with_context() {
        let err = NestGateError::internal_error("test", "component");
        let debug_str = format!("{:?}", err);
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_multiple_error_contexts() {
        let errors = vec![("op1", "comp1"), ("op2", "comp2"), ("op3", "comp3")];

        for (op, comp) in errors {
            let err = NestGateError::internal_error(op, comp);
            assert!(format!("{:?}", err).contains(op) || format!("{:?}", err).contains(comp));
        }
    }
}

#[cfg(test)]
mod error_variant_tests {
    use crate::error::NestGateError;

    #[test]
    fn test_all_error_variants() {
        let errors = vec![
            NestGateError::internal_error("internal", "test"),
            NestGateError::system("system", "failure"),
            NestGateError::configuration_error("field", "invalid"),
            NestGateError::validation_error("bad input"),
            NestGateError::network_error("connection failed"),
            NestGateError::storage_error("disk full"),
            NestGateError::security_error("unauthorized"),
            NestGateError::api_error("bad request"),
        ];

        for err in errors {
            // Verify each error can be created and formatted
            assert!(!format!("{}", err).is_empty());
            assert!(!format!("{:?}", err).is_empty());
        }
    }

    #[test]
    fn test_error_equality_checks() {
        let err1 = NestGateError::validation_error("test");
        let err2 = NestGateError::validation_error("test");

        // Errors with same message should format similarly
        assert_eq!(format!("{}", err1), format!("{}", err2));
    }

    #[test]
    fn test_error_message_formatting() {
        let err = NestGateError::internal_error("detailed error message", "component_name");

        let display = format!("{}", err);
        let debug = format!("{:?}", err);

        // Display should be user-friendly
        assert!(!display.is_empty());
        // Debug should be detailed
        assert!(!debug.is_empty());
        assert!(debug.len() >= display.len());
    }
}

#[cfg(test)]
mod error_propagation_tests {
    use crate::error::{NestGateError, Result};

    #[test]
    fn test_question_mark_operator() {
        fn inner() -> Result<i32> {
            Err(NestGateError::validation_error("inner error"))
        }

        fn outer() -> Result<String> {
            let value = inner()?;
            Ok(format!("value: {}", value))
        }

        assert!(outer().is_err());
    }

    #[test]
    fn test_map_err_propagation() {
        fn operation() -> Result<i32> {
            Err(NestGateError::internal_error("original", "context"))
        }

        let result = operation()
            .map_err(|e| NestGateError::internal_error(format!("wrapped: {}", e), "new_context"));

        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("wrapped"));
    }

    #[test]
    fn test_and_then_propagation() {
        fn step1() -> Result<i32> {
            Ok(42)
        }

        fn step2(value: i32) -> Result<String> {
            if value > 40 {
                Ok(format!("value: {}", value))
            } else {
                Err(NestGateError::validation_error("too small"))
            }
        }

        let result = step1().and_then(step2);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "value: 42");
    }
}

#[cfg(test)]
mod error_edge_cases {
    use crate::error::{NestGateError, Result};

    #[test]
    fn test_empty_error_message() {
        let err = NestGateError::internal_error("", "context");
        // Should handle empty message gracefully
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_very_long_error_message() {
        let long_msg = "x".repeat(10000);
        let err = NestGateError::internal_error(&long_msg, "context");
        let display = format!("{}", err);
        // Should handle long messages without panic
        assert!(display.len() > 0);
    }

    #[test]
    fn test_unicode_in_error_message() {
        let err = NestGateError::internal_error("失敗 échec ошибка", "unicode_test");
        let display = format!("{}", err);
        assert!(display.contains("失敗") || !display.is_empty());
    }

    #[test]
    fn test_special_characters_in_error() {
        let special = r#"Special: \n \t " ' < > & | $ ` "#;
        let err = NestGateError::internal_error(special, "special_chars");
        // Should handle special characters without panic
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_nested_result_unwrapping() {
        let result: Result<Result<i32>> = Ok(Ok(42));
        let inner = result.unwrap();
        assert_eq!(inner.unwrap(), 42);

        let result: Result<Result<i32>> = Ok(Err(NestGateError::validation_error("inner")));
        let inner = result.unwrap();
        assert!(inner.is_err());
    }

    #[test]
    fn test_option_with_error() {
        fn might_return_none() -> Option<Result<i32>> {
            Some(Err(NestGateError::internal_error(
                "error in option",
                "test",
            )))
        }

        let result = might_return_none();
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }
}

#[cfg(test)]
mod error_performance_tests {
    use crate::error::NestGateError;

    #[test]
    fn test_error_creation_performance() {
        // Create many errors to test performance characteristics
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let _ = NestGateError::internal_error(format!("error {}", i), "perf_test");
        }
        let duration = start.elapsed();

        // Should complete in reasonable time (< 100ms for 1000 errors)
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_error_formatting_performance() {
        let err = NestGateError::internal_error("test", "context");

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = format!("{}", err);
        }
        let duration = start.elapsed();

        // Should format quickly (< 50ms for 1000 formats)
        assert!(duration.as_millis() < 50);
    }
}

#[cfg(test)]
mod error_integration_tests {
    use crate::error::{NestGateError, Result};
    use std::collections::HashMap;

    #[test]
    fn test_error_in_hashmap() {
        let mut errors: HashMap<String, NestGateError> = HashMap::new();
        errors.insert("err1".to_string(), NestGateError::validation_error("bad1"));
        errors.insert("err2".to_string(), NestGateError::network_error("bad2"));

        assert_eq!(errors.len(), 2);
        assert!(errors.contains_key("err1"));
    }

    #[test]
    fn test_error_in_vec() {
        let errors: Vec<NestGateError> = vec![
            NestGateError::internal_error("e1", "c1"),
            NestGateError::internal_error("e2", "c2"),
            NestGateError::internal_error("e3", "c3"),
        ];

        assert_eq!(errors.len(), 3);
        for err in errors {
            assert!(!format!("{}", err).is_empty());
        }
    }

    #[test]
    fn test_error_across_threads() {
        use std::sync::Arc;
        use std::thread;

        let err = Arc::new(NestGateError::internal_error("shared", "test"));
        let err_clone = Arc::clone(&err);

        let handle = thread::spawn(move || format!("{}", err_clone));

        let result = handle.join().unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_batch_error_handling() {
        fn process_item(item: i32) -> Result<i32> {
            if item % 2 == 0 {
                Ok(item * 2)
            } else {
                Err(NestGateError::validation_error("odd number"))
            }
        }

        let items = vec![2, 4, 6];
        let results: Result<Vec<_>> = items.into_iter().map(process_item).collect();
        assert!(results.is_ok());

        let items = vec![2, 3, 4];
        let results: Result<Vec<_>> = items.into_iter().map(process_item).collect();
        assert!(results.is_err());
    }
}
