/// Safe testing operations
/// Provides safe alternatives to testing operations that might panic
use crate::NestGateError;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// Create internal error (used by safe_operations functions)
pub fn internal_error(message: &str, context: &str) -> NestGateError {
    NestGateError::Internal {
        message: format!("{context}: {message}"),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some(format!("Context: {context}")),
        is_bug: false,
    }
}
