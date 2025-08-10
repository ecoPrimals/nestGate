/// Result handling utilities
/// Core utilities for handling results safely without unwrap() calls
use crate::NestGateError;
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
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some(format!(
            "Original error type: {}",
            std::any::type_name::<E>()
        )),
        is_bug: false,
    })
}
