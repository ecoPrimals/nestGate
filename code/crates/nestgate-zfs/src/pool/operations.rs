// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Mutating pool operations: create, destroy, scrub.

use tokio::process::Command as TokioCommand;
use tracing::{debug, info, warn};

use crate::error::{Result, ZfsOperation, create_zfs_error};
use crate::pool::types::PoolInfo;

use super::manager::ZfsPoolManager;

impl ZfsPoolManager {
    /// Create a new ZFS pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_pool(&self, name: &str, devices: &[String]) -> Result<PoolInfo> {
        info!("Creating ZFS pool: {} with devices: {:?}", name, devices);

        // Build the zpool create command
        let mut args = vec!["create", name];
        for device in devices {
            args.push(device);
        }

        let output = TokioCommand::new("zpool")
            .args(&args)
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to execute zpool create: error details".to_string(),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zpool create failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        // Refresh the pool info and return it
        self.discover_single_pool(name).await?;
        self.get_pool_info(name).await
    }

    /// Destroy a ZFS pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn destroy_pool(&self, name: &str) -> Result<()> {
        warn!("Destroying ZFS pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["destroy", "-f", name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!(
                        "Failed to execute zpool destroy: {}",
                        "actual_error_details"
                    ),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zpool destroy failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        // Remove from discovered pools storage
        {
            let mut pools = self.discovered_pools.write().await;
            pools.remove(name);
            debug!("Removed pool {} from discovered pools cache", name);
        }

        info!("Successfully destroyed pool: {}", name);
        Ok(())
    }

    /// Start a scrub operation on a pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn scrub_pool(&self, name: &str) -> Result<()> {
        info!("Starting scrub for pool: {}", name);

        let output = TokioCommand::new("zpool")
            .args(["scrub", name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to execute zpool scrub: error details".to_string(),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zpool scrub failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        info!("Successfully started scrub for pool: {}", name);
        Ok(())
    }
}
