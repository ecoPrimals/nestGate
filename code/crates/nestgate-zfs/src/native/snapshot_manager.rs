use super::command_executor::NativeZfsCommandExecutor;
use crate::types::SnapshotInfo;
use nestgate_core::Result;
use std::sync::Arc;

pub struct NativeZfsSnapshotManager {
    command_executor: Arc<NativeZfsCommandExecutor>,
}

impl NativeZfsSnapshotManager {
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
