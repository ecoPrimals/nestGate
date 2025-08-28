/// Safe testing operations
/// Provides safe alternatives to testing operations that might panic
use crate::error::NestGateError;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// Create internal error (used by safe_operations functions)
pub fn internal_error(message: &str, context: &str) -> NestGateError {
    NestGateError::Internal {
        message: format!("{context}: {message}"),
        component: "safe_operations_testing".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: "test_error".to_string(),
            component: "safe_testing".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("context".to_string(), context.to_string());
                map.insert("message".to_string(), message.to_string());
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Review test conditions and retry".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    }
}
