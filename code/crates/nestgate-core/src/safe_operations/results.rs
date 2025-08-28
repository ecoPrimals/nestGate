/// Result handling utilities
/// Core utilities for handling results safely without unwrap() calls
use crate::error::NestGateError;
use std::fmt::Debug;

/// **UNIFIED**: Use the main Result type from parent module  
pub use super::Result;

/// **SAFE RESULT UNWRAP**
/// Provides safe result unwrapping with contextual error handling
pub fn safe_unwrap_result<T, E: Debug>(
    result: std::result::Result<T, E>,
    operation: &str,
    context: &str,
) -> Result<T> {
    result.map_err(|e| NestGateError::Internal {
        message: format!("Operation '{operation}' failed in context '{context}': {e:?}"),
        component: "safe_operations_results".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: operation.to_string(),
            component: "safe_results".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("context".to_string(), context.to_string());
                map.insert("original_error_type".to_string(), std::any::type_name::<E>().to_string());
                map.insert("error_debug".to_string(), format!("{:?}", e));
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Check operation preconditions and retry".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    })
}
