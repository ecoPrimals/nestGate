/// Safe threading operations
/// Provides safe alternatives to thread operations that might panic
use crate::error::NestGateError;
use std::thread;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE THREAD JOIN**
/// Replaces crate::safe_operations::safe_thread_join(handle)? with proper error handling
pub fn safe_thread_join<T: std::fmt::Debug>(handle: thread::JoinHandle<T>) -> Result<T> {
    handle.join().map_err(|e| NestGateError::Internal {
        message: format!("Thread join failed: {e:?}"),
        component: "safe_operations_threading".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false,
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: "safe_thread_join".to_string(),
            component: "safe_threading".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("details".to_string(), "Thread panicked during execution".to_string());
                map.insert("error_debug".to_string(), format!("{:?}", e));
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Review thread code for panic conditions".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    })
}

/// Safe wrapper for joining async tasks
pub fn safe_task_join<T>(
    task: tokio::task::JoinHandle<T>,
    operation_name: &str,
) -> Result<impl std::future::Future<Output = Result<T>>>
where
    T: Send + 'static,
{
    let operation = operation_name.to_string();
    Ok(async move {
        task.await
            .map_err(|e| crate::error::NestGateError::Internal {
                message: format!("Task join failed for operation '{operation}': {e}"),
                component: "safe_operations_threading".to_string(),
                location: Some(std::panic::Location::caller().to_string()),
                is_bug: false,
                context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                    operation: operation.to_string(),
                    component: "safe_threading".to_string(),
                    metadata: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("details".to_string(), "Async task failed or was cancelled".to_string());
                        map
                    },
                    timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec![],
                    performance_metrics: None,
                    environment: None,
                }),
            })
    })
}
