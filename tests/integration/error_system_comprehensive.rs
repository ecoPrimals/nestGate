//! Comprehensive Error System Tests
//!
//! These tests provide extensive coverage of NestGate's error handling system,
//! focusing on all error variants, context handling, and error propagation.

use nestgate_core::{
    error::{ErrorContext, NestGateError, RetryInfo, UnifiedConfigSource},
    Result,
};
use std::time::Duration;

/// Test all NestGateError::Internal variants and functionality
#[test]
fn test_internal_error_comprehensive() {
    // Test minimal internal error
    let error = NestGateError::Internal {
        message: "Test internal error".to_string(),
        location: None,
        debug_info: None,
        is_bug: false,
    };

    // Test error display
    let display = format!("{}", error);
    assert!(display.contains("Test internal error"));

    // Test debug formatting
    let debug = format!("{:?}", error);
    assert!(debug.contains("Internal"));
    assert!(debug.contains("Test internal error"));

    // Test with all fields populated
    let full_error = NestGateError::Internal {
        message: "Complex internal error".to_string(),
        location: Some("test_module::test_function".to_string()),
        debug_info: Some("Additional debug context".to_string()),
        is_bug: true,
    };

    let full_display = format!("{}", full_error);
    assert!(full_display.contains("Complex internal error"));

    // Test is_bug flag
    if let NestGateError::Internal { is_bug, .. } = full_error {
        assert!(is_bug);
    } else {
        panic!("Expected Internal error variant");
    }
}

/// Test all NestGateError::Validation variants and functionality
#[test]
fn test_validation_error_comprehensive() {
    let error = NestGateError::Validation {
        field: "email".to_string(),
        message: "Invalid email format".to_string(),
        current_value: Some("invalid-email".to_string()),
        expected: Some("user@domain.com".to_string()),
        user_error: true,
    };

    // Test field access
    if let NestGateError::Validation {
        field,
        message,
        current_value,
        expected,
        user_error,
    } = &error
    {
        assert_eq!(field, "email");
        assert_eq!(message, "Invalid email format");
        assert_eq!(current_value.as_ref().unwrap(), "invalid-email");
        assert_eq!(expected.as_ref().unwrap(), "user@domain.com");
        assert!(*user_error);
    } else {
        panic!("Expected Validation error variant");
    }

    // Test display
    let display = format!("{}", error);
    assert!(display.contains("email"));
    assert!(display.contains("Invalid email format"));

    // Test various field types
    let numeric_error = NestGateError::Validation {
        field: "port".to_string(),
        message: "Port out of range".to_string(),
        current_value: Some("99999".to_string()),
        expected: Some("1-65535".to_string()),
        user_error: false,
    };

    let numeric_display = format!("{}", numeric_error);
    assert!(numeric_display.contains("port"));
    assert!(numeric_display.contains("Port out of range"));
}

/// Test all NestGateError::Configuration variants and functionality
#[test]
fn test_configuration_error_comprehensive() {
    // Test with file source
    let file_error = NestGateError::Configuration {
        message: "Invalid database configuration".to_string(),
        config_source: UnifiedConfigSource::File("database.toml".to_string()),
        field: Some("connection_pool.max_size".to_string()),
        suggested_fix: Some("Set max_size to a value between 1 and 100".to_string()),
    };

    // Test field access
    if let NestGateError::Configuration {
        message,
        config_source,
        field,
        suggested_fix,
    } = &file_error
    {
        assert_eq!(message, "Invalid database configuration");
        assert_eq!(
            *config_source,
            UnifiedConfigSource::File("database.toml".to_string())
        );
        assert_eq!(field.as_ref().unwrap(), "connection_pool.max_size");
        assert_eq!(
            suggested_fix.as_ref().unwrap(),
            "Set max_size to a value between 1 and 100"
        );
    } else {
        panic!("Expected Configuration error variant");
    }

    // Test different config sources
    let env_error = NestGateError::Configuration {
        message: "Missing environment variable".to_string(),
        config_source: UnifiedConfigSource::Environment,
        field: None,
        suggested_fix: Some("Set NESTGATE_API_KEY environment variable".to_string()),
    };

    let env_display = format!("{}", env_error);
    assert!(env_display.contains("Missing environment variable"));

    // Test all UnifiedConfigSource variants
    let sources = vec![
        UnifiedConfigSource::File("test.toml".to_string()),
        UnifiedConfigSource::Environment,
        UnifiedConfigSource::Defaults,
        UnifiedConfigSource::CommandLine,
        UnifiedConfigSource::Database,
        UnifiedConfigSource::UserProvided,
        UnifiedConfigSource::Runtime,
        UnifiedConfigSource::Builder("custom_builder".to_string()),
        UnifiedConfigSource::Validation("validation_rule".to_string()),
    ];

    for source in sources {
        let error = NestGateError::Configuration {
            message: "Test config error".to_string(),
            config_source: source.clone(),
            field: None,
            suggested_fix: None,
        };

        let display = format!("{}", error);
        assert!(display.contains("Test config error"));

        // Test that each source can be created and compared
        let error2 = NestGateError::Configuration {
            message: "Test config error".to_string(),
            config_source: source.clone(),
            field: None,
            suggested_fix: None,
        };

        // Both errors should have the same config source
        if let (
            NestGateError::Configuration {
                config_source: s1, ..
            },
            NestGateError::Configuration {
                config_source: s2, ..
            },
        ) = (&error, &error2)
        {
            assert_eq!(s1, s2);
        }
    }
}

/// Test RetryInfo functionality comprehensively
#[test]
fn test_retry_info_comprehensive() {
    // Test basic retry info
    let retry_info = RetryInfo {
        max_attempts: 3,
        base_delay: Duration::from_millis(500),
        max_delay: Duration::from_secs(10),
        exponential_backoff: false,
    };

    assert_eq!(retry_info.max_attempts, 3);
    assert_eq!(retry_info.base_delay, Duration::from_millis(500));
    assert_eq!(retry_info.max_delay, Duration::from_secs(10));
    assert!(!retry_info.exponential_backoff);

    // Test exponential backoff
    let exp_retry = RetryInfo {
        max_attempts: 10,
        base_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(60),
        exponential_backoff: true,
    };

    assert_eq!(exp_retry.max_attempts, 10);
    assert_eq!(exp_retry.base_delay, Duration::from_secs(1));
    assert_eq!(exp_retry.max_delay, Duration::from_secs(60));
    assert!(exp_retry.exponential_backoff);

    // Test clone functionality
    let cloned = retry_info.clone();
    assert_eq!(retry_info.max_attempts, cloned.max_attempts);
    assert_eq!(retry_info.base_delay, cloned.base_delay);
    assert_eq!(retry_info.max_delay, cloned.max_delay);
    assert_eq!(retry_info.exponential_backoff, cloned.exponential_backoff);

    // Test debug formatting
    let debug = format!("{:?}", retry_info);
    assert!(debug.contains("max_attempts"));
    assert!(debug.contains("base_delay"));
    assert!(debug.contains("max_delay"));
    assert!(debug.contains("exponential_backoff"));
}

/// Test ErrorContext functionality comprehensively
#[test]
fn test_error_context_comprehensive() {
    use std::collections::HashMap;
    use std::time::SystemTime;

    // Test minimal context
    let minimal_context = ErrorContext {
        operation: "test_operation".to_string(),
        component: "test_component".to_string(),
        metadata: HashMap::new(),
        timestamp: SystemTime::now(),
    };

    assert_eq!(minimal_context.operation, "test_operation");
    assert_eq!(minimal_context.component, "test_component");
    assert!(minimal_context.metadata.is_empty());

    // Test full context with metadata
    let mut metadata = HashMap::new();
    metadata.insert("user_id".to_string(), "user_12345".to_string());
    metadata.insert("request_id".to_string(), "req_abcdef".to_string());
    metadata.insert("correlation_id".to_string(), "corr_xyz789".to_string());

    let full_context = ErrorContext {
        operation: "database_query".to_string(),
        component: "storage_service".to_string(),
        metadata: metadata.clone(),
        timestamp: SystemTime::now(),
    };

    assert_eq!(full_context.operation, "database_query");
    assert_eq!(full_context.component, "storage_service");
    assert_eq!(full_context.metadata.len(), 3);
    assert_eq!(full_context.metadata.get("user_id").unwrap(), "user_12345");
    assert_eq!(
        full_context.metadata.get("request_id").unwrap(),
        "req_abcdef"
    );
    assert_eq!(
        full_context.metadata.get("correlation_id").unwrap(),
        "corr_xyz789"
    );

    // Test clone functionality
    let cloned_context = full_context.clone();
    assert_eq!(full_context.operation, cloned_context.operation);
    assert_eq!(full_context.component, cloned_context.component);
    assert_eq!(full_context.metadata, cloned_context.metadata);

    // Test debug formatting
    let debug = format!("{:?}", full_context);
    assert!(debug.contains("database_query"));
    assert!(debug.contains("storage_service"));
    assert!(debug.contains("user_12345"));
}

/// Test error conversion and Result integration
#[test]
fn test_error_result_integration() {
    // Test Result<T, NestGateError> usage
    fn example_operation(should_fail: bool) -> Result<String> {
        if should_fail {
            Err(NestGateError::Internal {
                message: "Operation failed".to_string(),
                location: Some("test_function".to_string()),
                debug_info: None,
                is_bug: false,
            })
        } else {
            Ok("Success".to_string())
        }
    }

    // Test success case
    let success = example_operation(false);
    assert!(success.is_ok());
    assert_eq!(success.unwrap(), "Success");

    // Test error case
    let error = example_operation(true);
    assert!(error.is_err());

    let err = error.unwrap_err();
    if let NestGateError::Internal {
        message, location, ..
    } = err
    {
        assert_eq!(message, "Operation failed");
        assert_eq!(location.as_ref().unwrap(), "test_function");
    } else {
        panic!("Expected Internal error variant");
    }
}

/// Test error serialization and deserialization
#[test]
fn test_error_serialization() {
    use std::collections::HashMap;
    use std::time::SystemTime;

    let retry_info = RetryInfo {
        max_attempts: 5,
        base_delay: Duration::from_millis(1000),
        max_delay: Duration::from_secs(30),
        exponential_backoff: true,
    };

    // Test RetryInfo serialization
    let retry_json = serde_json::to_string(&retry_info).unwrap();
    let deserialized_retry: RetryInfo = serde_json::from_str(&retry_json).unwrap();

    assert_eq!(retry_info.max_attempts, deserialized_retry.max_attempts);
    assert_eq!(retry_info.base_delay, deserialized_retry.base_delay);
    assert_eq!(retry_info.max_delay, deserialized_retry.max_delay);
    assert_eq!(
        retry_info.exponential_backoff,
        deserialized_retry.exponential_backoff
    );

    // Test ErrorContext serialization
    let mut metadata = HashMap::new();
    metadata.insert("test_key".to_string(), "test_value".to_string());
    metadata.insert("user_id".to_string(), "test_user".to_string());

    let context = ErrorContext {
        operation: "serialize_test".to_string(),
        component: "test_component".to_string(),
        metadata: metadata.clone(),
        timestamp: SystemTime::now(),
    };

    let context_json = serde_json::to_string(&context).unwrap();
    let deserialized_context: ErrorContext = serde_json::from_str(&context_json).unwrap();

    assert_eq!(context.operation, deserialized_context.operation);
    assert_eq!(context.component, deserialized_context.component);
    assert_eq!(context.metadata, deserialized_context.metadata);

    // Test UnifiedConfigSource serialization
    let config_sources = vec![
        UnifiedConfigSource::File("test.toml".to_string()),
        UnifiedConfigSource::Environment,
        UnifiedConfigSource::Defaults,
        UnifiedConfigSource::CommandLine,
        UnifiedConfigSource::Database,
        UnifiedConfigSource::UserProvided,
        UnifiedConfigSource::Runtime,
        UnifiedConfigSource::Builder("test_builder".to_string()),
        UnifiedConfigSource::Validation("test_validation".to_string()),
    ];

    for source in config_sources {
        let json = serde_json::to_string(&source).unwrap();
        let deserialized: UnifiedConfigSource = serde_json::from_str(&json).unwrap();
        assert_eq!(source, deserialized);
    }
}

/// Test error chain and nested error scenarios
#[test]
fn test_error_chaining() {
    // Test creating errors with context
    fn create_validation_error() -> Result<()> {
        Err(NestGateError::Validation {
            field: "username".to_string(),
            message: "Username cannot be empty".to_string(),
            current_value: Some("".to_string()),
            expected: Some("non-empty string".to_string()),
            user_error: true,
        })
    }

    fn create_config_error() -> Result<()> {
        Err(NestGateError::Configuration {
            message: "Invalid user configuration".to_string(),
            config_source: UnifiedConfigSource::UserProvided,
            field: Some("user.username".to_string()),
            suggested_fix: Some("Provide a valid username".to_string()),
        })
    }

    fn create_internal_error() -> Result<()> {
        Err(NestGateError::Internal {
            message: "User validation pipeline failed".to_string(),
            location: Some("user_service::validate".to_string()),
            debug_info: Some("Multiple validation errors occurred".to_string()),
            is_bug: false,
        })
    }

    // Test that different error types can be created and handled
    let validation_result = create_validation_error();
    let config_result = create_config_error();
    let internal_result = create_internal_error();

    assert!(validation_result.is_err());
    assert!(config_result.is_err());
    assert!(internal_result.is_err());

    // Test error type identification
    let validation_err = validation_result.unwrap_err();
    let config_err = config_result.unwrap_err();
    let internal_err = internal_result.unwrap_err();

    match validation_err {
        NestGateError::Validation { .. } => (), // Expected
        _ => panic!("Expected Validation error"),
    }

    match config_err {
        NestGateError::Configuration { .. } => (), // Expected
        _ => panic!("Expected Configuration error"),
    }

    match internal_err {
        NestGateError::Internal { .. } => (), // Expected
        _ => panic!("Expected Internal error"),
    }
}
