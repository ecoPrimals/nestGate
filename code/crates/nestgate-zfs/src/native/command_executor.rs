//
// This module provides safe and robust execution of ZFS commands
// with proper error handling and security measures.

// Removed unused imports
use nestgate_core::{NestGateError, Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, error, info, warn};

/// Default timeout for ZFS commands (5 minutes)
const DEFAULT_ZFS_COMMAND_TIMEOUT_SECS: u64 = 300;

/// Typical number of properties in a ZFS dataset (used for HashMap pre-allocation)
const ZFS_TYPICAL_PROPERTY_COUNT: usize = 40;

/// Native ZFS command executor
pub struct NativeZfsCommandExecutor {
    /// Command timeout in seconds
    timeout_seconds: u64,
    /// Whether to log all commands (for debugging)
    verbose_logging: bool,
}
/// Result of a ZFS command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsCommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}
impl NativeZfsCommandExecutor {
    /// Create a new command executor
    #[must_use]
    pub fn new() -> Self {
        Self {
            timeout_seconds: DEFAULT_ZFS_COMMAND_TIMEOUT_SECS,
            verbose_logging: std::env::var("ZFS_VERBOSE_LOGGING").is_ok(),
        }
    }

    /// Create with custom timeout
    #[must_use]
    pub fn with_timeout(timeout_seconds: u64) -> Self {
        Self {
            timeout_seconds,
            verbose_logging: std::env::var("ZFS_VERBOSE_LOGGING").is_ok(),
        }
    }

    /// Execute a ZFS command safely
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn execute_command(&self, args: &[&str]) -> Result<ZfsCommandResult> {
        if self.verbose_logging {
            debug!("🔧 Executing ZFS command: zfs {}", args.join(" "));
        }

        // Security check: validate command arguments
        self.validate_command_args(args)?;

        let mut cmd = Command::new("zfs");
        cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());

        // Set timeout
        let output = match tokio::time::timeout(
            std::time::Duration::from_secs(self.timeout_seconds),
            cmd.output(),
        )
        .await
        {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                error!("ZFS command execution failed: {}", e);
                return Err(NestGateError::storage_error("zfs_command_execution"));
            }
            Err(_) => {
                error!(
                    "ZFS command timed out after {} seconds",
                    self.timeout_seconds
                );
                return Err(NestGateError::storage_error("zfs_command_timeout"));
            }
        };

        // PERFORMANCE OPTIMIZATION: Reduce allocations by using to_string() instead of into_owned()
        // This avoids double allocation when the data is already valid UTF-8
        let result = ZfsCommandResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        };

        if self.verbose_logging {
            debug!("📤 ZFS command result: {:?}", result);
        }

        if !result.success {
            warn!("ZFS command failed: {}", result.stderr);
        }

        Ok(result)
    }

    /// Execute a ZFS command and expect success
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn execute_command_expect_success(&self, args: &[&str]) -> Result<String> {
        let result = self.execute_command(args).await?;

        if !result.success {
            return Err(NestGateError::storage_error("zfs_command_failed"));
        }

        Ok(result.stdout)
    }

    /// Get ZFS pool list
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_pools(&self) -> Result<Vec<String>> {
        let output = self
            .execute_command_expect_success(&["list", "-H", "-o", "name", "-t", "filesystem"])
            .await?;

        Ok(output
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect())
    }

    /// Get ZFS dataset information
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_info(&self, dataset: &str) -> Result<HashMap<String, String>> {
        let output = self
            .execute_command_expect_success(&["get", "-H", "-p", "all", dataset])
            .await?;

        // PERFORMANCE OPTIMIZATION: Pre-allocate HashMap with typical property count
        // ZFS datasets typically have 30-50 properties, pre-allocating reduces rehashing
        let mut properties = HashMap::with_capacity(ZFS_TYPICAL_PROPERTY_COUNT);

        for line in output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 4 {
                let property = parts[1].to_string();
                let value = parts[2].to_string();
                properties.insert(property, value);
            }
        }

        Ok(properties)
    }

    /// Create a ZFS dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_dataset(
        &self,
        dataset: &str,
        properties: &HashMap<String, String>,
    ) -> Result<()> {
        let mut args = vec!["create"];

        // PERFORMANCE OPTIMIZATION: Pre-allocate Vec with known capacity
        // Each property needs 2 args (-o and key=value), plus final dataset arg
        let mut property_strings = Vec::with_capacity(properties.len());
        for (key, value) in properties {
            property_strings.push(format!("{key}={value}"));
        }

        // Pre-allocate args vector with exact size needed
        args.reserve(properties.len() * 2 + 1);

        // Add properties
        for prop_str in &property_strings {
            args.push("-o");
            args.push(prop_str);
        }

        args.push(dataset);

        self.execute_command_expect_success(&args).await?;

        info!("✅ Created ZFS dataset: {}", dataset);
        Ok(())
    }

    /// Create a ZFS snapshot
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_snapshot(&self, dataset: &str, snapshot_name: &str) -> Result<()> {
        let snapshot_full = format!("{dataset}@{snapshot_name}");
        self.execute_command_expect_success(&["snapshot", &snapshot_full])
            .await?;

        info!("✅ Created ZFS snapshot: {}", snapshot_full);
        Ok(())
    }

    /// Validate command arguments for security
    fn validate_command_args(&self, args: &[&str]) -> Result<()> {
        // PERFORMANCE OPTIMIZATION: Single-pass validation using chars()
        // Instead of 4 separate contains() calls, scan each character once
        for arg in args {
            for ch in arg.chars() {
                if matches!(ch, ';' | '&' | '|' | '`') {
                    return Err(NestGateError::security("Invalid command argument detected"));
                }
            }
        }

        // Validate that we only use safe ZFS commands
        if let Some(command) = args.first() {
            match *command {
                "list" | "get" | "set" | "create" | "destroy" | "snapshot" | "clone" | "send"
                | "receive" | "mount" | "unmount" | "share" | "unshare" | "upgrade"
                | "userspace" | "groupspace" | "projectspace" => {
                    // These are safe ZFS commands
                }
                _ => {
                    return Err(NestGateError::security(&format!(
                        "Unsafe ZFS command: {command}"
                    )));
                }
            }
        }

        Ok(())
    }
}

impl Default for NativeZfsCommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}

// Test modules
#[cfg(test)]
#[path = "command_executor_tests.rs"]
mod command_executor_tests;
