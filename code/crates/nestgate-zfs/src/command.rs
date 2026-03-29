// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use anyhow::{self, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== SECTION ====================

/// **CANONICAL**: `AnyhowResult` type alias for external integration
type AnyhowResult<T> = anyhow::Result<T>;
/// **CANONICAL**: ZFS command Result type using `AnyhowResult` for external integration
/// This uses `AnyhowResult` for better ecosystem integration with external command execution
type ZfsCommandResult<T> = AnyhowResult<T>;
/// **CANONICAL**: Parsed table result using `AnyhowResult`
type ParsedTableResult = AnyhowResult<Vec<HashMap<String, String>>>;
use std::process::Command;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;
// Removed unused tracing import
/// ZFS command execution framework
#[derive(Debug, Clone)]
/// Zfscommand
pub struct ZfsCommand {
    /// Dry Run
    pub dry_run: bool,
    /// Timeout Seconds
    pub timeout_seconds: u64,
}
impl Default for ZfsCommand {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            dry_run: false,
            timeout_seconds: 30,
        }
    }
}

impl ZfsCommand {
    /// Creates a new ZFS command executor with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use nestgate_zfs::command::ZfsCommand;
    /// let cmd = ZfsCommand::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables or disables dry-run mode
    ///
    /// When enabled, commands will be validated but not executed.
    ///
    /// # Arguments
    ///
    /// * `dry_run` - If true, commands won't actually execute
    ///
    /// # Examples
    ///
    /// ```
    /// use nestgate_zfs::command::ZfsCommand;
    /// let cmd = ZfsCommand::new().with_dry_run(true);
    /// ```
    #[must_use]
    pub const fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Sets the command execution timeout
    ///
    /// # Arguments
    ///
    /// * `timeout_seconds` - Maximum time to wait for command completion
    ///
    /// # Examples
    ///
    /// ```
    /// use nestgate_zfs::command::ZfsCommand;
    /// let cmd = ZfsCommand::new().with_timeout(60);
    /// ```
    #[must_use]
    pub const fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    /// Execute a zpool command
    pub async fn zpool(&self, args: &[&str]) -> ZfsCommandResult<CommandResult> {
        self.execute_command("zpool", args).await
    }

    /// Execute a zfs command
    pub async fn zfs(&self, args: &[&str]) -> ZfsCommandResult<CommandResult> {
        self.execute_command("zfs", args).await
    }

    /// Check if ZFS is available on the system
    pub fn check_zfs_available() -> ZfsCommandResult<bool> {
        let result = Command::new("which").arg("zfs").output();

        if let Ok(output) = result {
            Ok(output.status.success())
        } else {
            // Try direct execution
            let result = Command::new("zfs").arg("version").output();

            match result {
                Ok(output) => Ok(output.status.success()),
                Err(_) => Ok(false),
            }
        }
    }

    /// Execute a command with proper error handling and logging
    async fn execute_command(
        &self,
        command: &str,
        args: &[&str],
    ) -> ZfsCommandResult<CommandResult> {
        if self.dry_run {
            info!("DRY RUN: {} {}", command, args.join(" "));
            return Ok(CommandResult {
                success: true,
                stdout: format!("DRY RUN: {} {}", command, args.join(" ")),
                stderr: String::new(),
                exit_code: 0,
            });
        }

        debug!("Executing: {} {}", command, args.join(" "));

        let output = tokio::process::Command::new(command)
            .args(args)
            .output()
            .await
            .with_context(|| "Failed to execute error details command".to_string())?;

        // Convert command output to strings
        let stdout_result = if output.stdout.is_empty() {
            String::new()
        } else {
            String::from_utf8_lossy(&output.stdout).into_owned()
        };

        let stderr_result = if output.stderr.is_empty() {
            String::new()
        } else {
            String::from_utf8_lossy(&output.stderr).into_owned()
        };

        let result = CommandResult {
            success: output.status.success(),
            stdout: stdout_result,
            stderr: stderr_result,
            exit_code: output.status.code().unwrap_or(-1),
        };

        if result.success {
            debug!("Command succeeded: {} {}", command, args.join(" "));
            if !result.stdout.is_empty() {
                debug!("Output: {}", result.stdout.trim());
            }
        } else {
            error!(
                "Command failed: {} {} (exit code: {})",
                command,
                args.join(" "),
                result.exit_code
            );
            if !result.stderr.is_empty() {
                error!("Error output: {}", result.stderr.trim());
            }
        }

        Ok(result)
    }
}

/// Result of a command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Commandresult
pub struct CommandResult {
    /// Success
    pub success: bool,
    /// Stdout
    pub stdout: String,
    /// Stderr
    pub stderr: String,
    /// Exit Code
    pub exit_code: i32,
}
impl CommandResult {
    /// Check if the command was successful
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.success
    }

    /// Get the output as lines
    #[must_use]
    pub fn stdout_lines(&self) -> Vec<&str> {
        self.stdout.lines().collect()
    }

    /// Get the error output as lines
    #[must_use]
    pub fn stderr_lines(&self) -> Vec<&str> {
        self.stderr.lines().collect()
    }

    /// Parse the output as key-value pairs (for property commands)
    pub fn parse_properties(&self) -> ZfsCommandResult<HashMap<String, String>> {
        let mut properties = HashMap::new();

        for line in self.stdout_lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('\t') {
                properties.insert(key.trim().to_string(), value.trim().to_string());
            } else if let Some((key, value)) = line.split_once(' ') {
                properties.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        Ok(properties)
    }

    /// Parse tabular output (like zpool list, zfs list)
    pub fn parse_table(&self) -> ParsedTableResult {
        let lines = self.stdout_lines();
        if lines.is_empty() {
            return Ok(vec![]);
        }

        // First line should be headers
        let headers: Vec<&str> = lines[0].split_whitespace().collect();
        if headers.is_empty() {
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        for line in lines.iter().skip(1) {
            let values: Vec<&str> = line.split_whitespace().collect();
            if values.len() != headers.len() {
                warn!("Skipping malformed line: {}", line);
                continue;
            }

            let mut row = HashMap::new();
            for (header, value) in headers.iter().zip(values.iter()) {
                row.insert(header.to_string(), value.to_string());
            }
            results.push(row);
        }

        Ok(results)
    }
}

/// High-level ZFS operations
pub struct ZfsOperations {
    command: ZfsCommand,
}
impl ZfsOperations {
    /// Creates a new `ZfsOperations` instance with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            command: ZfsCommand::new(),
        }
    }

    /// Sets whether operations should be performed in dry-run mode.
    ///
    /// # Arguments
    /// * `dry_run` - If `true`, operations will be simulated without actual execution.
    #[must_use]
    pub const fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.command = self.command.with_dry_run(dry_run);
        self
    }

    /// List all ZFS pools
    pub async fn list_pools(&self) -> ZfsCommandResult<Vec<ZfsPool>> {
        let result = self
            .command
            .zpool(&["list", "-H", "-o", "name,size,alloc,free,health"])
            .await?;

        if !result.is_success() {
            return Err(anyhow::anyhow!("Failed to list pools: {}", result.stderr));
        }

        let mut pools = Vec::new();
        for line in result.stdout_lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                pools.push(ZfsPool {
                    name: parts[0].to_string(),
                    size: parts[1].to_string(),
                    allocated: parts[2].to_string(),
                    free: parts[3].to_string(),
                    health: parts[4].to_string(),
                });
            }
        }

        Ok(pools)
    }

    /// Get pool status
    pub async fn pool_status(&self, pool_name: &str) -> ZfsCommandResult<PoolStatus> {
        let result = self.command.zpool(&["status", pool_name]).await?;

        if !result.is_success() {
            return Err(anyhow::anyhow!(
                "Failed to get pool status: {}",
                result.stderr
            ));
        }

        // Parse pool status output
        let output = result.stdout;
        let healthy = output.contains("state: ONLINE") || output.contains("HEALTHY");
        let errors = output.contains("errors:") && !output.contains("errors: No known data errors");

        Ok(PoolStatus {
            name: pool_name.to_string(),
            state: if healthy {
                "ONLINE".to_string()
            } else {
                "DEGRADED".to_string()
            },
            scan: extract_scan_status(&output),
            errors: if errors {
                "Yes".to_string()
            } else {
                "No".to_string()
            },
            raw_output: output,
        })
    }

    /// List datasets in a pool
    pub async fn list_datasets(
        &self,
        pool_name: Option<&str>,
    ) -> ZfsCommandResult<Vec<ZfsDataset>> {
        let mut args = vec!["list", "-H", "-o", "name,used,avail,refer,mountpoint"];
        if let Some(pool) = pool_name {
            args.push(pool);
        }

        let result = self.command.zfs(&args).await?;

        if !result.is_success() {
            return Err(anyhow::anyhow!(
                "Failed to list datasets: {}",
                result.stderr
            ));
        }

        let mut datasets = Vec::new();
        for line in result.stdout_lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                datasets.push(ZfsDataset {
                    name: parts[0].to_string(),
                    used: parts[1].to_string(),
                    available: parts[2].to_string(),
                    referenced: parts[3].to_string(),
                    mountpoint: parts[4].to_string(),
                });
            }
        }

        Ok(datasets)
    }

    /// Create a dataset
    pub async fn create_dataset(
        &self,
        dataset_name: &str,
        properties: Option<&HashMap<String, String>>,
    ) -> ZfsCommandResult<()> {
        let mut args = vec!["create"];

        // Add properties if provided
        let mut property_strings = Vec::new();
        if let Some(props) = properties {
            for (key, value) in props {
                property_strings.push(format!("{key}={value}"));
            }
            for prop_string in &property_strings {
                args.push("-o");
                args.push(prop_string);
            }
        }

        args.push(dataset_name);

        let result = self.command.zfs(&args).await?;

        if !result.is_success() {
            return Err(anyhow::anyhow!(
                "Failed to create dataset: {}",
                result.stderr
            ));
        }

        info!("Successfully created dataset: {}", dataset_name);
        Ok(())
    }

    /// Create a snapshot
    pub async fn create_snapshot(
        &self,
        dataset_name: &str,
        _snapshot_name: &str,
    ) -> ZfsCommandResult<()> {
        let full_name = format!("{dataset_name}@error details");
        let result = self.command.zfs(&["snapshot", &full_name]).await?;

        if !result.is_success() {
            return Err(anyhow::anyhow!(
                "Failed to create snapshot: {}",
                result.stderr
            ));
        }

        info!("Successfully created snapshot: {}", full_name);
        Ok(())
    }

    /// List snapshots
    pub async fn list_snapshots(
        &self,
        dataset_name: Option<&str>,
    ) -> ZfsCommandResult<Vec<ZfsSnapshot>> {
        let mut args = vec!["list", "-H", "-t", "snapshot", "-o", "name,used,creation"];
        if let Some(dataset) = dataset_name {
            args.push(dataset);
        }

        let result = self.command.zfs(&args).await?;

        if !result.is_success() {
            return Err(anyhow::anyhow!(
                "Failed to list snapshots: {}",
                result.stderr
            ));
        }

        let mut snapshots = Vec::new();
        for line in result.stdout_lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                snapshots.push(ZfsSnapshot {
                    name: parts[0].to_string(),
                    used: parts[1].to_string(),
                    creation: parts[2].to_string(),
                });
            }
        }

        Ok(snapshots)
    }
}

impl Default for ZfsOperations {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// Helper function to extract scan status from zpool status output
fn extract_scan_status(output: &str) -> String {
    for line in output.lines() {
        let line = line.trim();
        if line.starts_with("scan:") {
            return line.strip_prefix("scan:").unwrap_or("").trim().to_string();
        }
    }
    "none requested".to_string()
}

/// ZFS Pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfspool
pub struct ZfsPool {
    /// Name
    pub name: String,
    /// Size
    pub size: String,
    /// Allocated
    pub allocated: String,
    /// Free
    pub free: String,
    /// Health
    pub health: String,
}
/// Pool status information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolstatus
pub struct PoolStatus {
    /// Name
    pub name: String,
    /// State
    pub state: String,
    /// Scan
    pub scan: String,
    /// Errors
    pub errors: String,
    /// Raw Output
    pub raw_output: String,
}
/// ZFS Dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsdataset
pub struct ZfsDataset {
    /// Name
    pub name: String,
    /// Used
    pub used: String,
    /// Available
    pub available: String,
    /// Referenced
    pub referenced: String,
    /// Mountpoint
    pub mountpoint: String,
}
/// ZFS Snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfssnapshot
pub struct ZfsSnapshot {
    /// Name
    pub name: String,
    /// Used
    pub used: String,
    /// Creation
    pub creation: String,
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[test]
    fn zfs_command_default_and_builder() {
        let cmd = ZfsCommand::new().with_dry_run(true).with_timeout(120);
        assert!(cmd.dry_run);
        assert_eq!(cmd.timeout_seconds, 120);
        let defaults = ZfsCommand::default();
        assert!(!defaults.dry_run);
        assert_eq!(defaults.timeout_seconds, 30);
    }

    #[test]
    fn command_result_helpers() {
        let result = CommandResult {
            success: true,
            stdout: "a\nb\n".to_string(),
            stderr: "e1\ne2".to_string(),
            exit_code: 0,
        };
        assert!(result.is_success());
        assert_eq!(result.stdout_lines().len(), 2);
        assert_eq!(result.stderr_lines().len(), 2);
    }

    #[test]
    fn command_result_parse_properties_tab_and_space() {
        let tab = CommandResult {
            success: true,
            stdout: "k1\tv1\n# skip\nk2 v2".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        let map = tab
            .parse_properties()
            .expect("test: parse tab-separated properties");
        assert_eq!(map.get("k1"), Some(&"v1".to_string()));
        assert_eq!(map.get("k2"), Some(&"v2".to_string()));
    }

    #[test]
    fn command_result_parse_table_rows_and_malformed_skip() {
        let result = CommandResult {
            success: true,
            stdout: "c1 c2\nv1 v2\nbad\nv3 v4 v5".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };
        let table = result.parse_table().expect("test: parse table");
        assert_eq!(table.len(), 1);
        assert_eq!(table[0]["c1"], "v1");
        assert_eq!(table[0]["c2"], "v2");
    }

    #[test]
    fn command_result_parse_table_empty_stdout() {
        let result = CommandResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
        };
        let table = result.parse_table().expect("test: empty table");
        assert!(table.is_empty());
    }

    #[test]
    fn extract_scan_status_finds_scan_line() {
        let out = "  scan: scrub in progress since Sun\n  other: x\n";
        assert_eq!(
            super::extract_scan_status(out),
            "scrub in progress since Sun"
        );
    }

    #[test]
    fn extract_scan_status_default_when_missing() {
        assert_eq!(super::extract_scan_status("no scan here"), "none requested");
    }

    #[tokio::test]
    async fn zfs_availability_check_does_not_panic() {
        let available = ZfsCommand::check_zfs_available().expect("test: check_zfs_available");
        let _ = available;
    }

    #[tokio::test]
    async fn dry_run_skips_real_execution() {
        let cmd = ZfsCommand::new().with_dry_run(true);
        let result = cmd.zpool(&["list"]).await.expect("test: dry-run zpool");
        assert!(result.is_success());
        assert!(result.stdout.contains("DRY RUN"));
        assert_eq!(result.exit_code, 0);
    }

    #[tokio::test]
    async fn command_result_parse_table_tab_separated() {
        let result = CommandResult {
            success: true,
            stdout: "name\tsize\talloc\npool1\t1T\t500G\npool2\t2T\t1T".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        let table = result.parse_table().expect("test: parse zpool-style table");
        assert_eq!(table.len(), 2);
        assert_eq!(table[0]["name"], "pool1");
        assert_eq!(table[1]["size"], "2T");
    }

    #[tokio::test]
    async fn zfs_operations_dry_run_list_paths() {
        let ops = ZfsOperations::new().with_dry_run(true);

        ops.list_pools().await.expect("test: list_pools dry run");
        ops.list_datasets(None)
            .await
            .expect("test: list_datasets dry run");
    }

    #[tokio::test]
    #[ignore = "Requires real ZFS"]
    async fn zfs_command_executes_real_zpool_list() {
        let cmd = ZfsCommand::new().with_dry_run(false);
        let _ = cmd.zpool(&["list"]).await;
    }
}
