/// Safe option operations
/// Provides safe utilities for handling Option types without unwrap() calls
use crate::NestGateError;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE OPTION UNWRAP**
/// Provides safe option unwrapping with contextual error handling
pub fn safe_unwrap_option<T>(option: Option<T>, context: &str, expected: &str) -> Result<T> {
    option.ok_or_else(|| NestGateError::Internal {
        message: format!("Expected {expected} in context: {context}"),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some("Option was None when Some was expected".to_string()),
        is_bug: false,
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
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some("Critical cryptographic operation returned None".to_string()),
        is_bug: false,
    })
}
