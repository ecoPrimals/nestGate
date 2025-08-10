/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// Safe service call with error context
pub async fn safe_service_call<F, T>(
    operation_name: &str,
    _context: &str,
    service_call: F,
) -> Result<T>
where
    F: std::future::Future<
        Output = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>,
    >,
{
    service_call
        .await
        .map_err(|e| crate::error::NestGateError::Internal {
            message: format!("Service call failed for '{operation_name}': {e}"),
            location: Some(std::panic::Location::caller().to_string()),
            debug_info: Some(format!(
                "Service call error for operation: {operation_name}"
            )),
            is_bug: false,
        })
}
