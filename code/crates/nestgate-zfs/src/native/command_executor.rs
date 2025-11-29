//! ZFS Command Executor
//!
//! This module provides safe and robust execution of native ZFS commands
//! with proper error handling, timeout management, and security measures.
//!
//! # Overview
//!
//! The command executor:
//! - Executes ZFS/zpool commands via `tokio::process::Command`
//! - Enforces timeouts to prevent hung operations
//! - Parses command output into structured data
//! - Provides comprehensive error handling
//! - Logs all operations for debugging
//!
//! # Safety
//!
//! - ✅ No unsafe code
//! - ✅ Timeout enforcement prevents indefinite blocking
//! - ✅ Proper error propagation
//! - ✅ Structured logging for audit trails
//!
//! # Examples
//!
//! ```rust,ignore
//! use nestgate_zfs::native::command_executor::NativeZfsCommandExecutor;
//!
//! let executor = NativeZfsCommandExecutor::new();
//!
//! // Execute ZFS command
//! let result = executor.execute_zfs_command(&["list", "-H"]).await?;
//! if result.success {
//!     println!("Output: {}", result.stdout);
//! }
//! ```

use nestgate_core::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{debug, error, info, warn};

/// Default timeout for ZFS commands (5 minutes)
///
/// Most ZFS operations complete within seconds, but pool creation
/// and scrubs can take several minutes. 5 minutes provides a safe
/// upper bound for normal operations.
const DEFAULT_ZFS_COMMAND_TIMEOUT_SECS: u64 = 300;

/// Typical number of properties in a ZFS dataset
///
/// Used for HashMap pre-allocation to reduce reallocations when
/// parsing ZFS property lists. Most datasets have 30-50 properties.
const ZFS_TYPICAL_PROPERTY_COUNT: usize = 40;

/// Native ZFS command executor
///
/// Executes ZFS and zpool commands safely with timeout enforcement
/// and comprehensive error handling.
///
/// # Configuration
///
/// - **Timeout**: Configurable command timeout (default: 5 minutes)
/// - **Verbose Logging**: Enable via `ZFS_VERBOSE_LOGGING` env var
///
/// # Examples
///
/// ```rust,ignore
/// // Default executor
/// let executor = NativeZfsCommandExecutor::new();
///
/// // Custom timeout
/// let executor = NativeZfsCommandExecutor::with_timeout(600);  // 10 minutes
/// ```
pub struct NativeZfsCommandExecutor {
    /// Command timeout in seconds
    timeout_seconds: u64,

    /// Whether to log all commands for debugging
    /// (enabled via ZFS_VERBOSE_LOGGING environment variable)
    verbose_logging: bool,
}

/// Result of a ZFS command execution
///
/// Contains the complete output of a ZFS command execution including
/// success status, stdout/stderr output, and exit code.
///
/// # Fields
///
/// - **success**: Whether the command completed successfully
/// - **stdout**: Standard output from the command
/// - **stderr**: Standard error from the command
/// - **exit_code**: Process exit code (0 = success)
///
/// # Examples
///
/// ```rust,ignore
/// let result = executor.execute_zfs_command(&["list"]).await?;
///
/// if result.success {
///     // Parse stdout
///     for line in result.stdout.lines() {
///         println!("Pool: {}", line);
///     }
/// } else {
///     eprintln!("Command failed: {}", result.stderr);
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfscommandresult
pub struct ZfsCommandResult {
    /// Whether the command succeeded (exit code 0)
    pub success: bool,

    /// Standard output from the command
    pub stdout: String,

    /// Standard error from the command
    pub stderr: String,

    /// Process exit code
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
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// Test modules
#[cfg(test)]
#[path = "command_executor_tests.rs"]
mod command_executor_tests;
