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
            component: "safe_operations".to_string(),
            location: Some(std::panic::Location::caller().to_string()),
            is_bug: false,
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: format!("service_call_{}", operation_name),
                component: "safe_operations".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("operation_name".to_string(), operation_name.to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                retry_info: Some(crate::error::context::RetryInfo {
                    attempt: 1,
                    max_attempts: 3,
                    retry_delay: std::time::Duration::from_millis(100),
                    base_delay: std::time::Duration::from_millis(100), // PEDANTIC: Added missing field
                    exponential_backoff: true, // PEDANTIC: Added missing field
                    backoff_multiplier: 2.0, // PEDANTIC: Added missing field
                    max_delay: std::time::Duration::from_secs(30), // PEDANTIC: Added missing field
                    jitter_ms: Some(50), // PEDANTIC: Added missing field
                }),
                recovery_suggestions: vec![
                    "Check service health".to_string(),
                    "Verify configuration".to_string(),
                ],
                performance_metrics: None,
                environment: None,
            }),
        })
}
