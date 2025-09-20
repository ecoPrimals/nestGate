// Safe service operations

use crate::error::NestGateError;
use crate::Result;

/// Safely call a service operation with error handling
pub fn safe_service_call<T, F>(operation_name: &str, operation: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    match operation() {
        Ok(result) => Ok(result),
        Err(e) => {
            eprintln!("Service operation '{operation_name}' failed: {e}");
            Err(e)
        }
    }
}
/// Create a service error with context
#[must_use]
pub const fn create_service_error(operation_name: &str, message: &str) -> NestGateError {
    NestGateError::internal_error(
        format!("Service operation '{operation_name}' failed: {message}"),
        "safe_operations",
    )
}
