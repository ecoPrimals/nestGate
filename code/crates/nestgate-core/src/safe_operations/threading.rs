/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;
/// Safe threading operations
/// Provides safe alternatives to thread operations that might panic
use crate::error::NestGateError;
use std::thread;
/// **SAFE THREAD JOIN**
/// Replaces `crate::safe_operations::safe_thread_join(handle)`? with proper error handling
pub fn safe_thread_join<T: std::fmt::Debug>(handle: thread::JoinHandle<T>) -> Result<T> {
    handle.join().map_err(|e| {
        NestGateError::internal_error(
            format!("Thread join failed: {e:?}"),
            "safe_operations_threading",
        )
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
        task.await.map_err(|e| {
            crate::error::NestGateError::internal_error(
                format!("Task join failed for operation '{operation}': {e}"),
                "safe_operations_threading",
            )
        })
    })
}
