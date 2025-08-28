/// Safe option operations
/// Provides safe utilities for handling Option types without unwrap() calls
use crate::error::NestGateError;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE OPTION UNWRAP**
/// Provides safe option unwrapping with contextual error handling
pub fn safe_unwrap_option<T>(option: Option<T>, context: &str, expected: &str) -> Result<T> {
    option.ok_or_else(|| NestGateError::Internal {
        message: format!("Expected {expected} in context: {context}"),
        component: "safe_operations_options".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: "safe_unwrap_option".to_string(),
            component: "safe_options".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("context".to_string(), context.to_string());
                map.insert("expected".to_string(), expected.to_string());
                map.insert("details".to_string(), "Option was None when Some was expected".to_string());
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Ensure the Option contains a value before unwrapping".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    })
}

/// **SAFE OPTION UNWRAP WITH RECOVERY**
/// Provides automatic recovery value when possible
pub fn safe_unwrap_or_default<T: Default>(option: Option<T>, context: &str) -> T {
    match option {
        Some(value) => value,
        None => {
            // Log the fallback for debugging but don't error
            tracing::debug!(
                "Using default value for missing option in context: {}",
                context
            );
            T::default()
        }
    }
}

/// **SAFE CRYPTO OPERATIONS**
/// For cryptographic operations that should not fail
pub fn safe_crypto_unwrap<T>(option: Option<T>, operation: &str) -> Result<T> {
    option.ok_or_else(|| NestGateError::Internal {
        message: format!("Cryptographic operation failed: {operation}"),
        component: "safe_operations_options".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: operation.to_string(),
            component: "safe_crypto".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("details".to_string(), "Critical cryptographic operation returned None".to_string());
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Verify cryptographic operation preconditions".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    })
}
