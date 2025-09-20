/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;
/// Safe option operations
/// Provides safe utilities for handling Option types without `unwrap()` calls
use crate::error::NestGateError;
/// **SAFE OPTION UNWRAP**
/// Provides safe option unwrapping with contextual error handling
pub fn safe_unwrap_option<T>(option: Option<T>, context: &str) -> Result<T> {
    option.ok_or_else(|| {
        NestGateError::internal_error(
            format!("Option was None in context: {context}"),
            "safe_operations_options",
        )
    })
}
/// **SAFE OPTION UNWRAP WITH RECOVERY**
/// Provides automatic recovery value when possible
pub fn safe_unwrap_or_default<T: Default>(option: Option<T>, context: &str) -> T {
    if let Some(value) = option {
        value
    } else {
        // Log the fallback for debugging but don't error
        tracing::debug!(
            "Using default value for missing option in context: {}",
            context
        );
        T::default()
    }
}
/// **SAFE CRYPTO OPERATIONS**
/// For cryptographic operations that should not fail
pub fn safe_crypto_unwrap<T>(option: Option<T>, operation: &str) -> Result<T> {
    option.ok_or_else(|| {
        NestGateError::internal_error(
            format!("Cryptographic operation failed: {operation}"),
            "safe_operations_options",
        )
    })
}
