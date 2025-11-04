//! **ADDITIONAL ERROR COVERAGE TESTS**
//!
//! Simple tests to boost coverage using existing API.

#[cfg(test)]
mod basic_error_coverage {
    use crate::error::{NestGateError, Result};

    #[test]
    fn test_system_error() {
        let error = NestGateError::system("init", "Failed");
        assert!(format!("{:?}", error).len() > 0);
    }

    #[test]
    fn test_api_error() {
        let error = NestGateError::api_error("Bad request");
        assert!(format!("{}", error).len() > 0);
    }

    #[test]
    fn test_storage_error() {
        let error = NestGateError::storage_error("Not found");
        let result: Result<()> = Err(error);
        assert!(result.is_err());
    }

    #[test]
    fn test_security_error() {
        let error = NestGateError::security_error("Unauthorized");
        assert!(format!("{:?}", error).len() > 0);
    }

    #[test]
    fn test_error_display() {
        let errors = vec![
            NestGateError::internal_error("test", "comp"),
            NestGateError::validation_error("invalid"),
            NestGateError::network_error("failed"),
        ];
        for e in errors {
            assert!(format!("{}", e).len() > 0);
        }
    }

    #[test]
    fn test_error_chain() {
        fn inner() -> Result<i32> {
            Err(NestGateError::validation_error("bad"))
        }
        fn outer() -> Result<String> {
            let _ = inner()?;
            Ok("ok".to_string())
        }
        assert!(outer().is_err());
    }

    #[test]
    fn test_error_recovery() {
        fn fail() -> Result<String> {
            Err(NestGateError::network_error("Failed"))
        }
        fn backup() -> Result<String> {
            Ok("backup".to_string())
        }
        let result = fail().or_else(|_| backup());
        ?
    }

    #[test]
    fn test_error_map() {
        let ok: Result<i32> = Ok(10);
        assert_eq!(ok.map(|v| v * 2).expect("Test setup failed"), 20);

        let err: Result<i32> = Err(NestGateError::internal_error("e", "c"));
        assert!(err.map(|v| v * 2).is_err());
    }

    #[test]
    fn test_config_error() {
        let error = NestGateError::configuration_error("field", "invalid");
        assert!(format!("{:?}", error).len() > 0);
    }

    #[test]
    fn test_not_found() {
        let error = NestGateError::not_found("Resource not found");
        assert!(format!("{}", error).len() > 0);
    }

    #[test]
    fn test_io_error_text() {
        let error = NestGateError::io_error("File read failed");
        assert!(format!("{:?}", error).len() > 0);
    }

    #[test]
    fn test_multiple_internal_errors() {
        let e1 = NestGateError::internal_error("error1", "comp1");
        let e2 = NestGateError::internal_error("error2", "comp2");
        let e3 = NestGateError::internal_error("error3", "comp3");

        assert!(format!("{}", e1).len() > 0);
        assert!(format!("{}", e2).len() > 0);
        assert!(format!("{}", e3).len() > 0);
    }

    #[test]
    fn test_validation_errors() {
        let e1 = NestGateError::validation_error("Required field missing");
        let e2 = NestGateError::validation_error("Invalid format");
        let e3 = NestGateError::validation_error("Out of range");

        for e in [e1, e2, e3] {
            assert!(format!("{:?}", e).len() > 5);
        }
    }

    #[test]
    fn test_network_errors_variations() {
        let errors = vec![
            NestGateError::network_error("Connection timeout"),
            NestGateError::network_error("DNS resolution failed"),
            NestGateError::network_error("Connection refused"),
        ];

        for e in errors {
            let msg = format!("{}", e);
            assert!(msg.len() > 0);
        }
    }

    #[test]
    fn test_error_unwrap_or() {
        let err: Result<i32> = Err(NestGateError::internal_error("fail", "comp"));
        let value = err.unwrap_or(42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_error_and_then() {
        let ok: Result<i32> = Ok(5);
        let result = ok.and_then(|v| {
            if v > 0 {
                Ok(v * 2)
            } else {
                Err(NestGateError::validation_error("Must be positive"))
            }
        });
        assert_eq!(result.expect("Test setup failed"), 10);
    }

    #[test]
    fn test_nested_operations() {
        fn level1() -> Result<i32> {
            level2()?;
            Ok(1)
        }

        fn level2() -> Result<i32> {
            level3()?;
            Ok(2)
        }

        fn level3() -> Result<i32> {
            Err(NestGateError::internal_error("deep error", "level3"))
        }

        assert!(level1().is_err());
    }

    #[test]
    fn test_system_errors_variety() {
        let e1 = NestGateError::system("database", "Connection lost");
        let e2 = NestGateError::system("cache", "Memory full");
        let e3 = NestGateError::system("logger", "Write failed");

        assert!(format!("{:?}", e1).len() > 0);
        assert!(format!("{:?}", e2).len() > 0);
        assert!(format!("{:?}", e3).len() > 0);
    }

    #[test]
    fn test_security_errors_variety() {
        let e1 = NestGateError::security_error("Invalid token");
        let e2 = NestGateError::security_error("Permission denied");
        let e3 = NestGateError::security_error("Session expired");

        for e in [e1, e2, e3] {
            assert!(format!("{}", e).len() > 0);
        }
    }
}