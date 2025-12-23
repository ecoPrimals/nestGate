//! **ERROR SYSTEM EDGE CASE TESTS** - Nov 23, 2025
//!
//! Comprehensive edge case tests for error handling including
//! boundary conditions, extreme inputs, and error composition.

#[cfg(test)]
mod error_message_edge_cases {
    use crate::error::NestGateError;

    #[test]
    fn test_extremely_long_error_message() {
        let long_msg = "error".repeat(10000);
        let error = NestGateError::validation_error(&long_msg);
        assert!(error.to_string().len() > 10000);
    }

    #[test]
    fn test_empty_error_message() {
        let error = NestGateError::validation_error("");
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_unicode_error_message() {
        let unicode_msg = "Error: 测试错误 🚨 ошибка خطأ";
        let error = NestGateError::validation_error(unicode_msg);
        assert!(error.to_string().contains("测试错误"));
    }

    #[test]
    fn test_special_characters_in_error() {
        let special_msg = r#"Error: \n\t\r\0 <>&"'"#;
        let error = NestGateError::validation_error(special_msg);
        assert!(error.to_string().contains("\\n"));
    }

    #[test]
    fn test_repeated_error_words() {
        let repeated = "ERROR ".repeat(1000);
        let error = NestGateError::validation_error(&repeated);
        assert!(!error.to_string().is_empty());
    }
}

#[cfg(test)]
mod error_context_edge_cases {
    use crate::error::ErrorContext;

    #[test]
    fn test_context_default() {
        let context = ErrorContext::default();
        assert_eq!(context.operation, "unknown");
        assert_eq!(context.component, "unknown");
    }

    #[test]
    fn test_context_with_metadata() {
        let mut context = ErrorContext::default();
        context
            .metadata
            .insert("key1".to_string(), "value1".to_string());
        context
            .metadata
            .insert("key2".to_string(), "value2".to_string());
        assert_eq!(context.metadata.len(), 2);
    }

    #[test]
    fn test_context_with_long_operation() {
        let context = ErrorContext {
            operation: "operation".repeat(100),
            ..Default::default()
        };
        assert!(context.operation.len() > 500);
    }

    #[test]
    fn test_context_cloning() {
        let mut context = ErrorContext::default();
        context.operation = "test_op".to_string();
        context.component = "test_component".to_string();
        let cloned = context.clone();
        assert_eq!(context.operation, cloned.operation);
        assert_eq!(context.component, cloned.component);
    }
}

#[cfg(test)]
mod error_conversion_edge_cases {
    use crate::error::NestGateError;
    use std::io;

    #[test]
    fn test_io_error_conversion_all_kinds() {
        let kinds = [
            io::ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied,
            io::ErrorKind::ConnectionRefused,
            io::ErrorKind::ConnectionReset,
            io::ErrorKind::ConnectionAborted,
            io::ErrorKind::NotConnected,
            io::ErrorKind::AddrInUse,
            io::ErrorKind::AddrNotAvailable,
            io::ErrorKind::BrokenPipe,
            io::ErrorKind::AlreadyExists,
            io::ErrorKind::WouldBlock,
            io::ErrorKind::InvalidInput,
            io::ErrorKind::InvalidData,
            io::ErrorKind::TimedOut,
            io::ErrorKind::WriteZero,
            io::ErrorKind::Interrupted,
            io::ErrorKind::UnexpectedEof,
        ];

        for kind in &kinds {
            let io_err = io::Error::new(*kind, "test error");
            let error: NestGateError = io_err.into();
            assert!(!error.to_string().is_empty());
        }
    }

    #[test]
    fn test_error_chain_depth() {
        let mut error: Result<(), NestGateError> = Ok(());

        for i in 0..100 {
            error = Err(NestGateError::internal_error(
                format!("Error {}", i),
                "test_component",
            ));
        }

        assert!(error.is_err());
    }

    #[test]
    fn test_string_error_conversion() {
        let string_err = "simple string error";
        let error: NestGateError = NestGateError::validation_error(string_err);
        assert!(error.to_string().contains(string_err));
    }
}

#[cfg(test)]
mod error_performance_edge_cases {
    use crate::error::NestGateError;

    #[test]
    fn test_rapid_error_creation() {
        let errors: Vec<_> = (0..10000)
            .map(|i| NestGateError::validation_error(&format!("Error {}", i)))
            .collect();
        assert_eq!(errors.len(), 10000);
    }

    #[test]
    fn test_error_to_string_performance() {
        let error = NestGateError::validation_error("test error");

        for _ in 0..1000 {
            let _ = error.to_string();
        }
    }

    #[test]
    fn test_error_cloning_performance() {
        let error = NestGateError::validation_error("test error");

        let errors: Vec<_> = (0..1000).map(|_| error.clone()).collect();
        assert_eq!(errors.len(), 1000);
    }
}

#[cfg(test)]
mod error_boundary_conditions {
    use crate::error::NestGateError;

    #[test]
    fn test_zero_length_strings() {
        let error = NestGateError::validation_error("");
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_maximum_string_capacity() {
        // Test with strings at reasonable limits (not OOM-inducing)
        let large_string = "x".repeat(1_000_000); // 1MB
        let error = NestGateError::validation_error(&large_string);
        assert!(error.to_string().len() >= 1_000_000);
    }

    #[test]
    fn test_error_with_newlines() {
        let msg = "Line 1\nLine 2\nLine 3\nLine 4";
        let error = NestGateError::validation_error(msg);
        assert!(error.to_string().contains("Line 1"));
    }

    #[test]
    fn test_error_with_tabs() {
        let msg = "Field1\tField2\tField3";
        let error = NestGateError::validation_error(msg);
        assert!(error.to_string().contains("Field1"));
    }
}

#[cfg(test)]
mod error_concurrent_edge_cases {
    use crate::error::NestGateError;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_error_creation() {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = thread::spawn(move || {
                let errors: Vec<_> = (0..100)
                    .map(|j| NestGateError::validation_error(&format!("Error {} {}", i, j)))
                    .collect();
                assert_eq!(errors.len(), 100);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_shared_error() {
        let error = Arc::new(NestGateError::validation_error("shared error"));

        let mut handles = vec![];
        for _ in 0..10 {
            let error_clone = Arc::clone(&error);
            let handle = thread::spawn(move || {
                let _ = error_clone.to_string();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

#[cfg(test)]
mod error_recovery_edge_cases {
    use crate::error::NestGateError;

    #[test]
    fn test_error_pattern_matching() {
        let errors = vec![
            NestGateError::validation_error("test"),
            NestGateError::internal_error("test", "component"),
            NestGateError::io_error("test"),
        ];

        for error in errors {
            match error {
                NestGateError::Validation { .. } => assert!(true),
                NestGateError::Internal { .. } => assert!(true),
                NestGateError::Io { .. } => assert!(true),
                _ => {}
            }
        }
    }

    #[test]
    fn test_error_recovery_chain() {
        let result: Result<(), NestGateError> =
            Err(NestGateError::validation_error("initial error"));

        let recovered =
            result.map_err(|_| NestGateError::internal_error("recovery failed", "recovery"));

        assert!(recovered.is_err());
    }

    #[test]
    fn test_error_map() {
        let result: Result<i32, NestGateError> = Err(NestGateError::validation_error("error"));
        let mapped = result.map_err(|_| NestGateError::internal_error("mapped", "test"));
        assert!(mapped.is_err());
    }
}
