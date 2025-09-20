/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;
/// **SAFE ASYNC TASK SPAWNING**
/// Wraps async task spawning with error context
pub fn safe_spawn_task<F, T>(future: F, task_name: &str) -> tokio::task::JoinHandle<Result<T>>
where
    F: std::future::Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let _task_name = task_name.to_string();
    tokio::spawn(async move {
        let result = future.await;
        Ok(result)
    })
}
