use super::command_executor::NativeZfsCommandExecutor;
// Removed unused imports
use nestgate_core::Result;
use std::sync::Arc;

pub struct NativeZfsHealthMonitor {
    command_executor: Arc<NativeZfsCommandExecutor>,
}

impl NativeZfsHealthMonitor {
    #[must_use]
    pub fn new(command_executor: Arc<NativeZfsCommandExecutor>) -> Self {
        Self { command_executor }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn check_system_health(&self) -> Result<bool> {
        match self.command_executor.execute_command(&["status"]).await {
            Ok(result) => Ok(result.success),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_executor() -> Arc<NativeZfsCommandExecutor> {
        Arc::new(NativeZfsCommandExecutor::new())
    }

    #[test]
    fn test_health_monitor_creation() {
        let executor = create_test_executor();
        let monitor = NativeZfsHealthMonitor::new(executor);
        drop(monitor);
    }

    #[test]
    fn test_health_monitor_has_executor() {
        let executor = create_test_executor();
        let _monitor = NativeZfsHealthMonitor::new(Arc::clone(&executor));
        assert!(Arc::strong_count(&executor) > 1);
    }

    #[test]
    fn test_multiple_health_monitor_instances() {
        let executor = create_test_executor();
        let _monitor1 = NativeZfsHealthMonitor::new(Arc::clone(&executor));
        let _monitor2 = NativeZfsHealthMonitor::new(Arc::clone(&executor));
        let _monitor3 = NativeZfsHealthMonitor::new(Arc::clone(&executor));
        assert!(Arc::strong_count(&executor) > 3);
    }

    #[test]
    fn test_health_monitor_can_be_cloned_via_arc() {
        let executor = create_test_executor();
        let monitor1 = Arc::new(NativeZfsHealthMonitor::new(executor));
        let _monitor2 = Arc::clone(&monitor1);
        assert!(Arc::strong_count(&monitor1) == 2);
    }

    #[test]
    fn test_executor_shared_across_monitors() {
        let executor = create_test_executor();
        let initial_count = Arc::strong_count(&executor);
        let _monitor1 = NativeZfsHealthMonitor::new(Arc::clone(&executor));
        assert_eq!(Arc::strong_count(&executor), initial_count + 1);
    }
}
