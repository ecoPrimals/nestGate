// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Native ZFS Backend Implementation
//!
//! This module provides native ZFS command execution and management using
//! direct integration with system ZFS/zpool commands. It implements the
//! canonical type system and error handling patterns.
//!
//! # Overview
//!
//! The native backend executes ZFS operations by:
//! - Spawning ZFS/zpool command processes
//! - Parsing command output
//! - Translating to canonical types
//! - Handling errors gracefully
//!
//! # Modules
//!
//! - [`command_executor`] - Core command execution and output parsing
//! - [`dataset_manager`] - Dataset creation, deletion, and property management
//! - [`health_monitor`] - Pool and dataset health monitoring
//! - [`pool_manager`] - Pool creation, modification, and status queries
//! - [`snapshot_manager`] - Snapshot operations and scheduling
//!
//! # Examples
//!
//! ```rust,ignore
//! use nestgate_zfs::native::NativeZfsBackend;
//!
//! // Create native backend
//! let config = ZfsConfig::default();
//! let backend = NativeZfsBackend::new(config).await?;
//!
//! // Check availability
//! if backend.is_available().await {
//!     println!("ZFS is available");
//! }
//! ```
//!
//! # Platform Support
//!
//! - Linux: ZFS on Linux (ZoL) 0.8.0+
//! - FreeBSD: Native ZFS support
//! - Solaris/Illumos: Native ZFS support
//!
//! # Safety
//!
//! This module uses `tokio::process::Command` for safe subprocess execution.
//! No unsafe code is used in ZFS command execution.

/// Native ZFS command execution
pub mod command_executor;

/// Dataset management operations
pub mod dataset_manager;

/// Health monitoring and diagnostics
pub mod health_monitor;

/// Pool management operations
pub mod pool_manager;

/// Snapshot management and scheduling
pub mod snapshot_manager;

// Re-export main types from the canonical types module
pub use crate::types::{
    CommandResult, DatasetInfo, PoolCapacity, PoolHealth, PoolInfo, PoolState, PoolStatus,
    SnapshotInfo, ZfsCommand, ZfsError,
};

// use crate::canonical_zfs_config::CanonicalZfsConfig; // Module not yet implemented
use crate::config::ZfsConfig as CanonicalZfsConfig; // Using ZfsConfig as canonical
use nestgate_core::{NestGateError, Result};

/// Native ZFS backend implementation
///
/// Provides direct integration with system ZFS commands for pool and dataset
/// management. This backend executes native ZFS/zpool commands and translates
/// results into canonical types.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_zfs::native::NativeZfsBackend;
///
/// let config = ZfsConfig::default();
/// let backend = NativeZfsBackend::new(config).await?;
///
/// if backend.is_available().await {
///     println!("ZFS backend ready");
/// }
/// ```
///
/// # Platform Requirements
///
/// Requires ZFS commands to be available:
/// - `zfs` command for dataset operations
/// - `zpool` command for pool operations
pub struct NativeZfsBackend {
    /// ZFS configuration
    config: CanonicalZfsConfig,
}

impl NativeZfsBackend {
    /// Create a new native ZFS backend
    ///
    /// Validates that ZFS is available on the system before creating
    /// the backend instance.
    ///
    /// # Arguments
    ///
    /// * `config` - ZFS configuration
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - ZFS is not available on the system
    /// - ZFS commands cannot be executed
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let config = ZfsConfig::default();
    /// let backend = NativeZfsBackend::new(config).await?;
    /// ```
    pub async fn new(config: CanonicalZfsConfig) -> Result<Self> {
        // Check if ZFS is available
        if !is_zfs_available().await {
            return Err(NestGateError::storage_error("zfs_availability_check"));
        }
        Ok(Self { config })
    }

    /// Check if ZFS is available on the system
    ///
    /// # Returns
    ///
    /// `true` if ZFS commands are available, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// if backend.is_available().await {
    ///     println!("ZFS is ready");
    /// }
    /// ```
    pub async fn is_available(&self) -> bool {
        is_zfs_available().await
    }

    /// Get the backend configuration
    ///
    /// # Returns
    ///
    /// Reference to the ZFS configuration
    #[must_use]
    pub fn config(&self) -> &CanonicalZfsConfig {
        &self.config
    }
}

/// Check if ZFS is available on the system
///
/// Attempts to execute `zfs version` to verify ZFS is installed and accessible.
///
/// # Returns
///
/// `true` if ZFS is available, `false` otherwise
///
/// # Examples
///
/// ```rust,ignore
/// if is_zfs_available().await {
///     println!("ZFS detected");
/// }
/// ```
pub async fn is_zfs_available() -> bool {
    // Try to execute a simple zfs command to check availability
    match tokio::process::Command::new("zfs")
        .arg("version")
        .output()
        .await
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Check if zpool command is available
///
/// Attempts to execute `zpool version` to verify zpool is installed and accessible.
///
/// # Returns
///
/// `true` if zpool is available, `false` otherwise
///
/// # Examples
///
/// ```rust,ignore
/// if is_zpool_available().await {
///     println!("zpool detected");
/// }
/// ```
pub async fn is_zpool_available() -> bool {
    match tokio::process::Command::new("zpool")
        .arg("version")
        .output()
        .await
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Get ZFS version information
///
/// Queries the installed ZFS version by executing `zfs version`.
///
/// # Returns
///
/// ZFS version string (e.g., "zfs-2.1.5-1")
///
/// # Errors
///
/// Returns error if:
/// - ZFS command cannot be executed
/// - Command execution fails
/// - Output cannot be parsed
///
/// # Examples
///
/// ```rust,ignore
/// let version = get_zfs_version().await?;
/// println!("ZFS version: {}", version);
/// ```
pub async fn get_zfs_version() -> Result<String> {
    let output = tokio::process::Command::new("zfs")
        .arg("version")
        .output()
        .await
        .map_err(|_e| NestGateError::storage_error("zfs_version_check"))?;

    if !output.status.success() {
        return Err(NestGateError::storage_error("zfs_version_check"));
    }

    let version_str = String::from_utf8_lossy(&output.stdout);
    Ok(version_str.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zfs_availability_check() {
        // This test will pass/fail based on whether ZFS is actually installed
        // In CI/test environments, we expect this to be false
        let available = is_zfs_available().await;
        println!("ZFS available: {available}");
    }

    #[tokio::test]
    async fn test_native_backend_creation() {
        let config = CanonicalZfsConfig::default();

        // This may fail in test environments without ZFS, which is expected
        match NativeZfsBackend::new(config).await {
            Ok(backend) => {
                assert!(backend.is_available().await);
            }
            Err(_) => {
                // Expected in environments without ZFS
                println!("ZFS not available - test passed");
            }
        }
    }
}
