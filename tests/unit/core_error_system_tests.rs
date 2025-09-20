//! **COMPREHENSIVE CORE ERROR SYSTEM TESTS**
//!
//! Unit tests for the canonical error system to achieve 50% coverage target

use nestgate_core::error::{NestGateError, ValidationError, NetworkError, StorageError};
use nestgate_core::{Result, validation_error, network_error};
use std::collections::HashMap;

/// **ERROR CREATION TESTS**

#[cfg(test)]
mod error_creation_tests {
    use super::*;

    #[test]
    fn test_simple_error_creation() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateError::simple("Test error message");
        assert!(error.to_string().contains("Test error message"));
    Ok(())
    }

    #[test]
    fn test_validation_error_creation() -> Result<(), Box<dyn std::error::Error>> {
        let error = validation_error!("Invalid input: {}", "test_value");
        match error {
            NestGateError::Validation(ve) => {
                assert!(ve.message.contains("Invalid input"));
                assert!(ve.message.contains("test_value"));
    Ok(())
            }
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Test assertion failed"))),
    Ok(())
        }
    Ok(())
    }

    #[test]
    fn test_network_error_creation() -> Result<(), Box<dyn std::error::Error>> {
        let error = network_error!("Connection failed to {}", "example.com");
        match error {
            NestGateError::Network(ne) => {
                assert!(ne.message.contains("Connection failed"));
                assert!(ne.message.contains("example.com"));
    Ok(())
            }
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Test assertion failed"))),
    Ok(())
        }
    Ok(())
    }

    #[test]
    fn test_error_with_context() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateError::simple("Base error")
            .with_context("Additional context");
        assert!(error.to_string().contains("Base error"));
        assert!(error.to_string().contains("Additional context"));
    Ok(())
}
}

/// **ERROR CONVERSION TESTS**
#[cfg(test)]
mod error_conversion_tests {
    use super::*;
    use std::io;

    #[test]
    fn test_from_io_error() -> Result<(), Box<dyn std::error::Error>> {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let nestgate_error: NestGateError = io_error.into();
        assert!(nestgate_error.to_string().contains("File not found"));
    Ok(())
    }

    #[test]
    fn test_from_string_error() -> Result<(), Box<dyn std::error::Error>> {
        let string_error = "String error message".to_string();
        let nestgate_error: NestGateError = string_error.into();
        assert!(nestgate_error.to_string().contains("String error message"));
    Ok(())
    }

    #[test]
    fn test_validation_error_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let validation_error = ValidationError {
            message: "Validation failed".to_string(),
            field: Some("test_field".to_string()),
            code: Some("INVALID_VALUE".to_string()),
            context: HashMap::new(),
        };
        let nestgate_error: NestGateError = validation_error.into();
        assert!(nestgate_error.to_string().contains("Validation failed"));
    Ok(())
}
}

/// **ERROR CONTEXT TESTS**
#[cfg(test)]
mod error_context_tests {
    use super::*;

    #[test]
    fn test_error_context_chaining() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateError::simple("Original error")
            .with_context("First context")
            .with_context("Second context");
        
        let error_string = error.to_string();
        assert!(error_string.contains("Original error"));
        assert!(error_string.contains("First context"));
        assert!(error_string.contains("Second context"));
    Ok(())
    }

    #[test]
    fn test_validation_error_with_field() -> Result<(), Box<dyn std::error::Error>> {
        let mut error = ValidationError::new("Invalid value");
        error.field = Some("username".to_string());
        error.code = Some("INVALID_FORMAT".to_string());
        
        let nestgate_error = NestGateError::Validation(error);
        let error_string = nestgate_error.to_string();
        assert!(error_string.contains("Invalid value"));
        assert!(error_string.contains("username"));
    Ok(())
    }

    #[test]
    fn test_network_error_with_details() -> Result<(), Box<dyn std::error::Error>> {
        let error = NetworkError {
            message: "Connection timeout".to_string(),
            endpoint: Some("https://api.example.com".to_string()),
            status_code: Some(408),
            retry_after: Some(std::time::Duration::from_secs(30)),
        };
        
        let nestgate_error = NestGateError::Network(error);
        let error_string = nestgate_error.to_string();
        assert!(error_string.contains("Connection timeout"));
        assert!(error_string.contains("api.example.com"));
    Ok(())
}
}

/// **RESULT TYPE TESTS**
#[cfg(test)]
mod result_type_tests {
    use super::*;

    #[test]
    fn test_result_ok_case() -> Result<(), Box<dyn std::error::Error>> {
        let result: Result<String> = Ok("Success".to_string());
        assert!(result.is_ok());
        assert_eq!(result?, "Success");
    Ok(())
    }

    #[test]
    fn test_result_error_case() -> Result<(), Box<dyn std::error::Error>> {
        let result: Result<String> = Err(NestGateError::simple("Test error"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Test error"));
    Ok(())
    }

    #[test]
    fn test_result_map_operations() -> Result<(), Box<dyn std::error::Error>> {
        let result: Result<i32> = Ok(42);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped?, 84);
    Ok(())
    }

    #[test]
    fn test_result_and_then_operations() -> Result<(), Box<dyn std::error::Error>> {
        let result: Result<i32> = Ok(10);
        let chained = result.and_then(|x| {
            if x > 5 {
                Ok(x * 3)
            } else {
                Err(NestGateError::simple("Too small"))
    Ok(())
            }
        });
        assert_eq!(chained?, 30);
    Ok(())
}
}

/// **ERROR SERIALIZATION TESTS**
#[cfg(test)]
mod error_serialization_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_validation_error_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let error = ValidationError {
            message: "Test validation error".to_string(),
            field: Some("test_field".to_string()),
            code: Some("TEST_CODE".to_string()),
            context: {
                let mut ctx = HashMap::new();
                ctx.insert("key1".to_string(), "value1".to_string());
                ctx
            },
        };

        let json = serde_json::to_string(&error)?;
        let deserialized: ValidationError = serde_json::from_str(&json)?;
        
        assert_eq!(error.message, deserialized.message);
        assert_eq!(error.field, deserialized.field);
        assert_eq!(error.code, deserialized.code);
    Ok(())
    }

    #[test]
    fn test_network_error_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let error = NetworkError {
            message: "Network failure".to_string(),
            endpoint: Some("https://test.example.com".to_string()),
            status_code: Some(500),
            retry_after: Some(std::time::Duration::from_secs(60)),
        };

        let json = serde_json::to_string(&error)?;
        let deserialized: NetworkError = serde_json::from_str(&json)?;
        
        assert_eq!(error.message, deserialized.message);
        assert_eq!(error.endpoint, deserialized.endpoint);
        assert_eq!(error.status_code, deserialized.status_code);
    Ok(())
}
}

/// **ERROR DEBUGGING TESTS**
#[cfg(test)]
mod error_debugging_tests {
    use super::*;

    #[test]
    fn test_error_debug_output() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateError::simple("Debug test error");
        let debug_output = format!("{:?}", error);
        assert!(debug_output.contains("Debug test error"));
    Ok(())
    }

    #[test]
    fn test_error_display_output() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateError::simple("Display test error");
        let display_output = format!("{}", error);
        assert!(display_output.contains("Display test error"));
    Ok(())
    }

    #[test]
    fn test_error_source_chain() -> Result<(), Box<dyn std::error::Error>> {
        let root_cause = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let nestgate_error: NestGateError = root_cause.into();
        
        // Test that we can access the source chain
        assert!(nestgate_error.source().is_some());
    Ok(())
}
}

/// **MACRO SYSTEM TESTS**
#[cfg(test)]
mod macro_system_tests {
    use super::*;

    #[test]
    fn test_validation_error_macro() -> Result<(), Box<dyn std::error::Error>> {
        let field = "username";
        let value = "invalid@";
        let error = validation_error!("Invalid {}: {}", field, value);
        
        match error {
            NestGateError::Validation(ve) => {
                assert!(ve.message.contains("Invalid username"));
                assert!(ve.message.contains("invalid@"));
    Ok(())
            }
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Test assertion failed"))),
    Ok(())
        }
    Ok(())
    }

    #[test]
    fn test_network_error_macro() -> Result<(), Box<dyn std::error::Error>> {
        let endpoint = "api.test.com";
        let error = network_error!("Failed to connect to {}", endpoint);
        
        match error {
            NestGateError::Network(ne) => {
                assert!(ne.message.contains("Failed to connect"));
                assert!(ne.message.contains("api.test.com"));
    Ok(())
            }
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Test assertion failed"))),
    Ok(())
        }
    Ok(())
    }

    #[test]
    fn test_error_macro_with_format() -> Result<(), Box<dyn std::error::Error>> {
        let error = nestgate_core::error!("Error code: {} - {}", 404, "Not Found");
        assert!(error.to_string().contains("Error code: 404"));
        assert!(error.to_string().contains("Not Found"));
    Ok(())
}
} 