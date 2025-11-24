//! **ERROR PATH COVERAGE TESTS - NESTGATE CORE**
//!
//! Comprehensive tests for error handling paths, edge cases, and error propagation
//! to boost nestgate-core coverage from 72.34% toward 80%+

#[cfg(test)]
mod error_path_coverage {
    use crate::error::NestGateError;
    use crate::Result;

    // ==================== ERROR CONSTRUCTION TESTS ====================

    #[test]
    fn test_validation_error_creation() {
        let error = NestGateError::validation_error("Invalid input");
        assert!(error.to_string().contains("Validation"));
        assert!(error.to_string().contains("Invalid input"));
    }

    #[test]
    fn test_network_error_creation() {
        let error = NestGateError::network_error("Connection failed");
        assert!(error.to_string().contains("Network"));
    }

    #[test]
    fn test_storage_error_creation() {
        let error = NestGateError::storage_error("Disk full");
        assert!(error.to_string().contains("Storage"));
    }

    #[test]
    fn test_internal_error_creation() {
        let error = NestGateError::internal("System error");
        assert!(error.to_string().contains("Internal"));
    }

    #[test]
    fn test_configuration_error_creation() {
        let error = NestGateError::configuration_error("config_field", "Invalid config");
        assert!(
            error.to_string().contains("Configuration") || error.to_string().contains("config")
        );
    }

    #[test]
    fn test_security_error_creation() {
        let error = NestGateError::security_error("Unauthorized");
        assert!(
            error.to_string().contains("Security") || error.to_string().contains("Unauthorized")
        );
    }

    #[test]
    fn test_api_error_creation() {
        let error = NestGateError::api_error("Bad request");
        assert!(error.to_string().contains("API") || error.to_string().contains("Bad request"));
    }

    #[test]
    fn test_timeout_error_creation() {
        use std::time::Duration;
        let error = NestGateError::timeout_error("db_query", Duration::from_secs(30));
        assert!(
            error.to_string().contains("Timeout")
                || error.to_string().contains("timeout")
                || error.to_string().contains("db_query")
        );
    }

    // ==================== ERROR PROPAGATION TESTS ====================

    #[test]
    fn test_result_ok_propagation() {
        fn inner_ok() -> Result<i32> {
            Ok(42)
        }

        fn outer() -> Result<i32> {
            let value = inner_ok()?;
            Ok(value * 2)
        }

        assert_eq!(outer().unwrap(), 84);
    }

    #[test]
    fn test_result_err_propagation() {
        fn inner_err() -> Result<i32> {
            Err(NestGateError::validation_error("test"))
        }

        fn outer() -> Result<i32> {
            let _value = inner_err()?;
            Ok(100)
        }

        assert!(outer().is_err());
    }

    #[test]
    fn test_error_context_preservation() {
        let error = NestGateError::network_error("original error");
        let error_str = error.to_string();
        assert!(error_str.contains("original error"));
    }

    // ==================== ERROR CONVERSION TESTS ====================

    #[test]
    fn test_error_from_string() {
        let error: NestGateError = "test error".to_string().into();
        assert!(error.to_string().contains("test error"));
    }

    #[test]
    fn test_error_from_str() {
        let error: NestGateError = "test error".into();
        assert!(error.to_string().contains("test error"));
    }

    #[test]
    fn test_error_debug_display() {
        let error = NestGateError::internal("debug test");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.len() > 0);
    }

    // ==================== ERROR CHAINING TESTS ====================

    #[test]
    fn test_nested_error_propagation() {
        fn level1() -> Result<()> {
            Err(NestGateError::storage_error("level 1"))
        }

        fn level2() -> Result<()> {
            level1()?;
            Ok(())
        }

        fn level3() -> Result<()> {
            level2()?;
            Ok(())
        }

        assert!(level3().is_err());
    }

    #[test]
    fn test_multiple_error_types_chain() {
        fn operation1() -> Result<i32> {
            Err(NestGateError::network_error("network failed"))
        }

        fn operation2() -> Result<i32> {
            operation1().or_else(|_| Err(NestGateError::storage_error("storage failed")))
        }

        let result = operation2();
        assert!(result.is_err());
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_empty_error_message() {
        let error = NestGateError::internal("");
        let error_str = error.to_string();
        assert!(error_str.len() > 0); // Should still have error type
    }

    #[test]
    fn test_very_long_error_message() {
        let long_msg = "error ".repeat(1000);
        let error = NestGateError::internal(&long_msg);
        assert!(error.to_string().contains("error"));
    }

    #[test]
    fn test_error_with_special_characters() {
        let error = NestGateError::validation_error("Error: \n\t\r \"quoted\" 'text'");
        assert!(error.to_string().len() > 0);
    }

    #[test]
    fn test_error_with_unicode() {
        let error = NestGateError::internal("Error with 🚀 emoji and 中文");
        assert!(error.to_string().contains("🚀"));
    }

    // ==================== RESULT TYPE OPERATIONS ====================

    #[test]
    fn test_result_map_ok() {
        let result: Result<i32> = Ok(10);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped.unwrap(), 20);
    }

    #[test]
    fn test_result_map_err() {
        let result: Result<i32> = Err(NestGateError::internal("test"));
        let mapped = result.map(|x| x * 2);
        assert!(mapped.is_err());
    }

    #[test]
    fn test_result_and_then_ok() {
        let result: Result<i32> = Ok(10);
        let chained = result.and_then(|x| {
            if x > 5 {
                Ok(x * 2)
            } else {
                Err(NestGateError::validation_error("too small"))
            }
        });
        assert_eq!(chained.unwrap(), 20);
    }

    #[test]
    fn test_result_and_then_err() {
        let result: Result<i32> = Ok(3);
        let chained = result.and_then(|x| {
            if x > 5 {
                Ok(x * 2)
            } else {
                Err(NestGateError::validation_error("too small"))
            }
        });
        assert!(chained.is_err());
    }

    #[test]
    fn test_result_or_else_ok() {
        let result: Result<i32> = Ok(10);
        let recovered: Result<i32> = result.or_else(|_| Ok(0));
        assert_eq!(recovered.unwrap(), 10);
    }

    #[test]
    fn test_result_or_else_err() {
        let result: Result<i32> = Err(NestGateError::internal("test"));
        let recovered: Result<i32> = result.or_else(|_| Ok(42));
        assert_eq!(recovered.unwrap(), 42);
    }

    #[test]
    fn test_result_unwrap_or() {
        let result: Result<i32> = Err(NestGateError::internal("test"));
        let value = result.unwrap_or(100);
        assert_eq!(value, 100);
    }

    #[test]
    fn test_result_unwrap_or_else() {
        let result: Result<i32> = Err(NestGateError::internal("test"));
        let value = result.unwrap_or_else(|_| 200);
        assert_eq!(value, 200);
    }

    // ==================== ERROR MATCHING TESTS ====================

    #[test]
    fn test_error_type_matching() {
        let error = NestGateError::validation_error("test");
        match error {
            NestGateError::Validation(_) => assert!(true),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_multiple_error_type_handling() {
        fn handle_error(error: NestGateError) -> &'static str {
            match error {
                NestGateError::Validation(_) => "validation",
                NestGateError::Network(_) => "network",
                NestGateError::Storage(_) => "storage",
                NestGateError::Configuration(_) => "configuration",
                _ => "other",
            }
        }

        assert_eq!(
            handle_error(NestGateError::validation_error("test")),
            "validation"
        );
        assert_eq!(
            handle_error(NestGateError::network_error("test")),
            "network"
        );
        assert_eq!(
            handle_error(NestGateError::storage_error("test")),
            "storage"
        );
        assert_eq!(
            handle_error(NestGateError::configuration_error("field", "test")),
            "configuration"
        );
    }

    // ==================== CONCURRENT ERROR HANDLING ====================

    #[tokio::test]
    async fn test_concurrent_error_creation() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                tokio::spawn(async move {
                    let error = NestGateError::internal(&format!("error {}", i));
                    assert!(error.to_string().contains(&i.to_string()));
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_concurrent_result_operations() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                tokio::spawn(async move {
                    let result: Result<i32> = if i % 2 == 0 {
                        Ok(i)
                    } else {
                        Err(NestGateError::internal("odd"))
                    };

                    if i % 2 == 0 {
                        assert!(result.is_ok());
                    } else {
                        assert!(result.is_err());
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.await.unwrap();
        }
    }

    // ==================== ERROR RECOVERY PATTERNS ====================

    #[test]
    fn test_error_recovery_with_default() {
        fn operation_that_fails() -> Result<String> {
            Err(NestGateError::network_error("failed"))
        }

        let result = operation_that_fails().unwrap_or_else(|_| "default".to_string());
        assert_eq!(result, "default");
    }

    #[test]
    fn test_error_recovery_with_retry() {
        let mut attempt = 0;

        fn operation(attempt: &mut i32) -> Result<i32> {
            *attempt += 1;
            if *attempt < 3 {
                Err(NestGateError::network_error("retry"))
            } else {
                Ok(100)
            }
        }

        let result = operation(&mut attempt)
            .or_else(|_| operation(&mut attempt))
            .or_else(|_| operation(&mut attempt));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
    }

    // ==================== ERROR LOGGING TESTS ====================

    #[test]
    fn test_error_message_extraction() {
        let error = NestGateError::validation_error("field is required");
        let message = error.to_string();
        assert!(message.contains("field is required"));
    }

    #[test]
    fn test_error_type_identification() {
        let errors = vec![
            NestGateError::validation_error("v"),
            NestGateError::network_error("n"),
            NestGateError::storage_error("s"),
        ];

        for error in errors {
            assert!(error.to_string().len() > 0);
        }
    }
}
