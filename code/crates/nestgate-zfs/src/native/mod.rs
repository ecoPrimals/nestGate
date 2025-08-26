//
// This module provides native ZFS command execution and management
// using the canonical type system and error handling.

pub mod command_executor;
pub mod dataset_manager;
pub mod health_monitor;
pub mod pool_manager;
pub mod snapshot_manager;

// Re-export main types from the canonical types module
pub use crate::types::{
    CommandResult, DatasetInfo, PoolCapacity, PoolHealth, PoolInfo, PoolState, PoolStatus,
    SnapshotInfo, ZfsCommand, ZfsError, ZfsResult,
};

use crate::canonical_zfs_config::CanonicalZfsConfig;
use nestgate_core::{NestGateError, Result};

/// Native ZFS backend implementation
pub struct NativeZfsBackend {
    config: CanonicalZfsConfig,
}

impl NativeZfsBackend {
    /// Create a new native ZFS backend
    pub async fn new(config: CanonicalZfsConfig) -> Result<Self> {
        // Check if ZFS is available
        if !is_zfs_available().await {
            return Err(NestGateError::storage_error("zfs_availability_check", "ZFS is not available on this system", None));
        }

        Ok(Self { config })
    }

    /// Check if ZFS is available on the system
    pub async fn is_available(&self) -> bool {
        is_zfs_available().await
    }

    /// Get the configuration
    pub fn config(&self) -> &CanonicalZfsConfig {
        &self.config
    }
}

/// Check if ZFS is available on the system
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
pub async fn get_zfs_version() -> Result<String> {
    let output = tokio::process::Command::new("zfs")
        .arg("version")
        .output()
        .await
        .map_err(|e| NestGateError::storage_error("zfs_version_check", &format!("Failed to get ZFS version: {e}"), None))?;

    if !output.status.success() {
        return Err(NestGateError::storage_error("zfs_version_check", "Failed to get ZFS version", None));
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
        println!("ZFS available: {}", available);
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
