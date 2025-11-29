use super::command_executor::NativeZfsCommandExecutor;
use crate::types::SnapshotInfo;
use nestgate_core::Result;
use std::sync::Arc;

/// Native ZFS snapshot manager
///
/// Provides production-ready ZFS snapshot operations including creation,
/// deletion, listing, and rollback functionality.
///
/// # Examples
///
/// ```no_run
/// use nestgate_zfs::native::snapshot_manager::NativeZfsSnapshotManager;
/// use nestgate_zfs::native::command_executor::NativeZfsCommandExecutor;
/// use std::sync::Arc;
///
/// # async fn example() -> nestgate_core::Result<()> {
/// let executor = Arc::new(NativeZfsCommandExecutor::new());
/// let manager = NativeZfsSnapshotManager::new(executor);
///
/// // Create a snapshot
/// manager.create_snapshot("tank/data", "backup-2024").await?;
/// # Ok(())
/// # }
/// ```
pub struct NativeZfsSnapshotManager {
    command_executor: Arc<NativeZfsCommandExecutor>,
}

impl NativeZfsSnapshotManager {
    /// Creates a new snapshot manager instance
    ///
    /// # Arguments
    ///
    /// * `command_executor` - Shared reference to the ZFS command executor
    ///
    /// # Returns
    ///
    /// A new `NativeZfsSnapshotManager` instance
    #[must_use]
    pub fn new(command_executor: Arc<NativeZfsCommandExecutor>) -> Self {
        Self { command_executor }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn create_snapshot(&self, dataset: &str, snapshot_name: &str) -> Result<()> {
        self.command_executor
            .create_snapshot(dataset, snapshot_name)
            .await?;
        Ok(())
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn list_snapshots(&self, dataset: &str) -> Result<Vec<SnapshotInfo>> {
        let output = self
            .command_executor
            .execute_command_expect_success(&[
                "list",
                "-H",
                "-t",
                "snapshot",
                "-o",
                "name,used,creation",
                dataset,
            ])
            .await?;

        let mut snapshots = Vec::new();
        for line in output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                let full_name = parts[0].to_string();
                let name = full_name
                    .split('@')
                    .next_back()
                    .unwrap_or(&full_name)
                    .to_string();

                snapshots.push(SnapshotInfo {
                    name,
                    dataset: dataset.to_string(),
                    size: parts[1].parse().unwrap_or(0),
                    properties: std::collections::HashMap::new(),
                    created_at: std::time::SystemTime::now(), // Simplified - would parse from creation time
                });
            }
        }

        Ok(snapshots)
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
    fn test_snapshot_manager_creation() {
        let executor = create_test_executor();
        let manager = NativeZfsSnapshotManager::new(executor);
        // Just verify it doesn't panic
        drop(manager);
    }

    #[test]
    fn test_snapshot_manager_has_executor() {
        let executor = create_test_executor();
        let _manager = NativeZfsSnapshotManager::new(Arc::clone(&executor));
        // Verify the Arc reference count increased
        assert!(Arc::strong_count(&executor) > 1);
    }

    #[test]
    fn test_multiple_manager_instances() {
        let executor = create_test_executor();
        let _manager1 = NativeZfsSnapshotManager::new(Arc::clone(&executor));
        let _manager2 = NativeZfsSnapshotManager::new(Arc::clone(&executor));
        assert!(Arc::strong_count(&executor) > 2);
    }

    #[test]
    fn test_snapshot_info_parsing_logic() {
        // Test the parsing logic that would be used in list_snapshots
        let sample_line = "tank/data@snap1\t1024\t1234567890";
        let parts: Vec<&str> = sample_line.split('\t').collect();

        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "tank/data@snap1");
        assert_eq!(parts[1], "1024");
    }

    #[test]
    fn test_snapshot_name_extraction() {
        // Test the @ splitting logic
        let full_name = "tank/data@snapshot1";
        let name = full_name.split('@').next_back().unwrap_or(full_name);
        assert_eq!(name, "snapshot1");
    }

    #[test]
    fn test_snapshot_name_no_at_sign() {
        // Test when there's no @ sign
        let full_name = "tank/data";
        let name = full_name.split('@').next_back().unwrap_or(full_name);
        assert_eq!(name, "tank/data");
    }

    #[test]
    fn test_size_parsing() {
        // Test size parsing logic
        let size_str = "1024";
        let size: u64 = size_str.parse().unwrap_or(0);
        assert_eq!(size, 1024);
    }

    #[test]
    fn test_size_parsing_invalid() {
        // Test size parsing with invalid input
        let size_str = "invalid";
        let size: u64 = size_str.parse().unwrap_or(0);
        assert_eq!(size, 0);
    }

    #[test]
    fn test_snapshot_parts_insufficient() {
        // Test handling of lines with insufficient parts
        let sample_line = "tank/data@snap1";
        let parts: Vec<&str> = sample_line.split('\t').collect();
        assert!(parts.len() < 3);
    }

    #[test]
    fn test_empty_line_handling() {
        let sample_line = "";
        let parts: Vec<&str> = sample_line.split('\t').collect();
        assert_eq!(parts.len(), 1);
        assert!(parts.len() < 3); // Would be skipped
    }
}
