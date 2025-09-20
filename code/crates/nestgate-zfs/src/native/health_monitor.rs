use super::command_executor::NativeZfsCommandExecutor;
// Removed unused imports
use nestgate_core::Result;
use std::sync::Arc;

pub struct NativeZfsHealthMonitor {
    command_executor: Arc<NativeZfsCommandExecutor>,
}

impl NativeZfsHealthMonitor {
    pub const fn new(command_executor: Arc<NativeZfsCommandExecutor>) -> Self {
        Self { command_executor }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn check_system_health(&self) -> Result<bool>  {
        match self.command_executor.execute_command(&["status"]).await {
            Ok(result) => Ok(result.success),
            Err(_) => Ok(false),
        }
    }
}
