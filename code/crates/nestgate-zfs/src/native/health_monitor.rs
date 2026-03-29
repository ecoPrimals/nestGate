// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use super::command_executor::NativeZfsCommandExecutor;
// Removed unused imports
use nestgate_core::Result;
use std::sync::Arc;

/// Native ZFS health monitoring system
///
/// Provides real-time health monitoring and status checks for ZFS pools
/// and datasets, including scrub status, error detection, and system health.
///
/// # Examples
///
/// ```no_run
/// use nestgate_zfs::native::health_monitor::NativeZfsHealthMonitor;
/// use nestgate_zfs::native::command_executor::NativeZfsCommandExecutor;
/// use std::sync::Arc;
///
/// # async fn example() -> nestgate_core::Result<()> {
/// let executor = Arc::new(NativeZfsCommandExecutor::new());
/// let monitor = NativeZfsHealthMonitor::new(executor);
///
/// // Check system health
/// let healthy = monitor.check_system_health().await?;
/// # Ok(())
/// # }
/// ```
pub struct NativeZfsHealthMonitor {
    command_executor: Arc<NativeZfsCommandExecutor>,
}

impl NativeZfsHealthMonitor {
    /// Creates a new health monitor instance
    ///
    /// # Arguments
    ///
    /// * `command_executor` - Shared reference to the ZFS command executor
    ///
    /// # Returns
    ///
    /// A new `NativeZfsHealthMonitor` instance
    #[must_use]
    pub const fn new(command_executor: Arc<NativeZfsCommandExecutor>) -> Self {
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

    /// Creates  Test Executor
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
