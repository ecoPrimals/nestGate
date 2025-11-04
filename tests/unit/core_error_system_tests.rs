//! **COMPREHENSIVE CORE ERROR SYSTEM TESTS**
//!
//! Unit tests for the canonical error system to achieve 50% coverage target
//! 
//! **MODERNIZED**: Now using NestGateUnifiedError throughout

use nestgate_core::error::{
    NestGateError, NestGateUnifiedError,
    ValidationErrorDetails, NetworkErrorDetails, StorageErrorDetails,
};
use nestgate_core::Result;
use std::collections::HashMap;

/// **ERROR CREATION TESTS**

#[cfg(test)]
mod error_creation_tests {
    use super::*;

    #[test]
    fn test_simple_error_creation() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateUnifiedError::Internal(Box::new(
            nestgate_core::error::InternalErrorDetails {
                message: "Test error message".to_string(),
                component: "test".to_string(),
                location: None,
                is_bug: false,
                context: None,
            }
        ));
        assert!(error.to_string().contains("Test error message"));
        Ok(())
    }

    #[test]
    fn test_validation_error_creation() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateUnifiedError::Validation(Box::new(ValidationErrorDetails {
            message: format!("Invalid input: {}", "test_value"),
            field: Some("input".to_string()),
            code: None,
            context: HashMap::new(),
        }));
        
        match error {
            NestGateUnifiedError::Validation(ve) => {
                assert!(ve.message.contains("Invalid input"));
                assert!(ve.message.contains("test_value"));
            }
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Test assertion failed"))),
        }
        Ok(())
    }

    #[test]
    fn test_network_error_creation() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateUnifiedError::Network(Box::new(NetworkErrorDetails {
            message: format!("Connection failed to {}", "example.com"),
            endpoint: Some("example.com".to_string()),
            port: None,
            protocol: "HTTP".to_string(),
            network_data: None,
            context: None,
        }));
        
        match error {
            NestGateUnifiedError::Network(ne) => {
                assert!(ne.message.contains("Connection failed"));
                assert!(ne.message.contains("example.com"));
            }
            _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Test assertion failed"))),
        }
        Ok(())
    }

    #[test]
    fn test_error_with_context() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateUnifiedError::Internal(Box::new(
            nestgate_core::error::InternalErrorDetails {
                message: "Base error".to_string(),
                component: "test".to_string(),
                location: Some("test.rs:42".to_string()),
                is_bug: false,
                context: None,
            }
        ));
        assert!(error.to_string().contains("Base error"));
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
        let nestgate_error: NestGateUnifiedError = io_error.into();
        assert!(nestgate_error.to_string().contains("File not found"));
        Ok(())
    }

    #[test]
    fn test_from_string_error() -> Result<(), Box<dyn std::error::Error>> {
        let string_error = "String error message".to_string();
        let nestgate_error = NestGateUnifiedError::Internal(Box::new(
            nestgate_core::error::InternalErrorDetails {
                message: string_error,
                component: "string_conversion".to_string(),
                location: None,
                is_bug: false,
                context: None,
            }
        ));
        assert!(nestgate_error.to_string().contains("String error message"));
        Ok(())
    }

    #[test]
    fn test_validation_error_conversion() -> Result<(), Box<dyn std::error::Error>> {
        let validation_details = ValidationErrorDetails {
            message: "Validation failed".to_string(),
            field: Some("test_field".to_string()),
            code: Some("INVALID_VALUE".to_string()),
            context: HashMap::new(),
        };
        let nestgate_error = NestGateUnifiedError::Validation(Box::new(validation_details));
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
        let error = NestGateUnifiedError::Internal(Box::new(
            nestgate_core::error::InternalErrorDetails {
                message: "Original error".to_string(),
                component: "test".to_string(),
                location: Some("test.rs:100".to_string()),
                is_bug: false,
                context: None,
            }
        ));
        
        let error_string = error.to_string();
        assert!(error_string.contains("Original error"));
        Ok(())
    }

    #[test]
    fn test_validation_error_with_field() -> Result<(), Box<dyn std::error::Error>> {
        let validation_details = ValidationErrorDetails {
            message: "Invalid value".to_string(),
            field: Some("username".to_string()),
            code: Some("INVALID_FORMAT".to_string()),
            context: HashMap::new(),
        };
        
        let nestgate_error = NestGateUnifiedError::Validation(Box::new(validation_details));
        let error_string = nestgate_error.to_string();
        assert!(error_string.contains("Invalid value"));
        Ok(())
    }

    #[test]
    fn test_network_error_with_details() -> Result<(), Box<dyn std::error::Error>> {
        let network_details = NetworkErrorDetails {
            message: "Connection timeout".to_string(),
            endpoint: Some("https://api.example.com".to_string()),
            port: Some(443),
            protocol: "HTTPS".to_string(),
            network_data: None,
            context: None,
        };
        
        let nestgate_error = NestGateUnifiedError::Network(Box::new(network_details));
        let error_string = nestgate_error.to_string();
        assert!(error_string.contains("Connection timeout"));
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
        let error = NestGateUnifiedError::Internal(Box::new(
            nestgate_core::error::InternalErrorDetails {
                message: "Test error".to_string(),
                component: "test".to_string(),
                location: None,
                is_bug: false,
                context: None,
            }
        ));
        let result: Result<String> = Err(error);
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
                Err(NestGateUnifiedError::Internal(Box::new(
                    nestgate_core::error::InternalErrorDetails {
                        message: "Too small".to_string(),
                        component: "test".to_string(),
                        location: None,
                        is_bug: false,
                        context: None,
                    }
                )))
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
        let validation_details = ValidationErrorDetails {
            message: "Test validation error".to_string(),
            field: Some("test_field".to_string()),
            code: Some("TEST_CODE".to_string()),
            context: {
                let mut ctx = HashMap::new();
                ctx.insert("key1".to_string(), "value1".to_string());
                ctx
            },
        };

        let json = serde_json::to_string(&validation_details)?;
        let deserialized: ValidationErrorDetails = serde_json::from_str(&json)?;
        
        assert_eq!(validation_details.message, deserialized.message);
        assert_eq!(validation_details.field, deserialized.field);
        assert_eq!(validation_details.code, deserialized.code);
        Ok(())
    }

    #[test]
    fn test_network_error_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let network_details = NetworkErrorDetails {
            message: "Network failure".to_string(),
            endpoint: Some("https://test.example.com".to_string()),
            port: Some(443),
            protocol: "HTTPS".to_string(),
            network_data: None,
            context: None,
        };

        let json = serde_json::to_string(&network_details)?;
        let deserialized: NetworkErrorDetails = serde_json::from_str(&json)?;
        
        assert_eq!(network_details.message, deserialized.message);
        assert_eq!(network_details.endpoint, deserialized.endpoint);
        assert_eq!(network_details.port, deserialized.port);
        Ok(())
    }
}

/// **ERROR DEBUGGING TESTS**
#[cfg(test)]
mod error_debugging_tests {
    use super::*;
    use std::io;

    #[test]
    fn test_error_debug_output() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateUnifiedError::Internal(Box::new(
            nestgate_core::error::InternalErrorDetails {
                message: "Debug test error".to_string(),
                component: "test".to_string(),
                location: None,
                is_bug: false,
                context: None,
            }
        ));
        let debug_output = format!("{:?}", error);
        assert!(debug_output.contains("Debug test error"));
        Ok(())
    }

    #[test]
    fn test_error_display_output() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateUnifiedError::Internal(Box::new(
            nestgate_core::error::InternalErrorDetails {
                message: "Display test error".to_string(),
                component: "test".to_string(),
                location: None,
                is_bug: false,
                context: None,
            }
        ));
        let display_output = format!("{}", error);
        assert!(display_output.contains("Display test error"));
        Ok(())
    }

    #[test]
    fn test_error_source_chain() -> Result<(), Box<dyn std::error::Error>> {
        let root_cause = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
        let nestgate_error: NestGateUnifiedError = root_cause.into();
        
        // NestGateUnifiedError implements Error trait
        let error_string = nestgate_error.to_string();
        assert!(error_string.contains("Access denied"));
        Ok(())
    }
}

/// **UNIFIED ERROR SYSTEM TESTS**
#[cfg(test)]
mod unified_error_tests {
    use super::*;

    #[test]
    fn test_unified_error_variants() -> Result<(), Box<dyn std::error::Error>> {
        // Test various error variants
        let validation = NestGateUnifiedError::Validation(Box::new(ValidationErrorDetails {
            message: "Validation test".to_string(),
            field: Some("test".to_string()),
            code: None,
            context: HashMap::new(),
        }));
        assert!(validation.to_string().contains("Validation"));

        let network = NestGateUnifiedError::Network(Box::new(NetworkErrorDetails {
            message: "Network test".to_string(),
            endpoint: None,
            port: None,
            protocol: "HTTP".to_string(),
            network_data: None,
            context: None,
        }));
        assert!(network.to_string().contains("Network"));

        let storage = NestGateUnifiedError::Storage(Box::new(StorageErrorDetails {
            message: "Storage test".to_string(),
            resource: None,
            storage_data: None,
            operation: None,
            context: None,
        }));
        assert!(storage.to_string().contains("Storage"));

        Ok(())
    }

    #[test]
    fn test_error_type_matching() -> Result<(), Box<dyn std::error::Error>> {
        let error = NestGateUnifiedError::Validation(Box::new(ValidationErrorDetails {
            message: "Test validation".to_string(),
            field: Some("test_field".to_string()),
            code: Some("TEST".to_string()),
            context: HashMap::new(),
        }));

        match error {
            NestGateUnifiedError::Validation(details) => {
                assert_eq!(details.field, Some("test_field".to_string()));
                assert_eq!(details.code, Some("TEST".to_string()));
            }
            _ => return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Expected Validation error"
            ))),
        }

        Ok(())
    }
} 