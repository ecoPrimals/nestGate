/// Safe threading operations
/// Provides safe alternatives to thread operations that might panic
use crate::NestGateError;
use std::thread;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE THREAD JOIN**
/// Replaces crate::safe_operations::safe_thread_join(handle)? with proper error handling
pub fn safe_thread_join<T: std::fmt::Debug>(handle: thread::JoinHandle<T>) -> Result<T> {
    handle.join().map_err(|e| NestGateError::Internal {
        message: format!("Thread join failed: {e:?}"),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some("Thread panicked during execution".to_string()),
        is_bug: false,
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
                location: Some(std::panic::Location::caller().to_string()),
                debug_info: Some("Async task failed or was cancelled".to_string()),
                is_bug: false,
            })
    })
}
