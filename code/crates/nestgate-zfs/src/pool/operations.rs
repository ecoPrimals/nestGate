// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Mutating pool operations: create, destroy, scrub.

use tokio::process::Command as TokioCommand;
use tracing::{debug, info, warn};

use crate::error::{Result, ZfsOperation, create_zfs_error};
use crate::pool::types::PoolInfo;

use super::manager::ZfsPoolManager;

fn zpool_create_argv(name: &str, devices: &[String]) -> Vec<String> {
    let mut args = vec!["create".to_string(), name.to_string()];
    args.extend(devices.iter().cloned());
    args
}

fn zpool_destroy_argv(name: &str) -> [String; 3] {
    ["destroy".to_string(), "-f".to_string(), name.to_string()]
}

fn zpool_scrub_argv(name: &str) -> [String; 2] {
    ["scrub".to_string(), name.to_string()]
}

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

        let args = zpool_create_argv(name, devices);

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
            .args(zpool_destroy_argv(name))
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
        }
        debug!("Removed pool {} from discovered pools cache", name);

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
            .args(zpool_scrub_argv(name))
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

#[cfg(test)]
mod tests {
    use super::{zpool_create_argv, zpool_destroy_argv, zpool_scrub_argv};

    #[test]
    fn zpool_create_argv_orders_name_and_devices() {
        let argv = zpool_create_argv("tank", &["/dev/sda".into(), "/dev/sdb".into()]);
        assert_eq!(
            argv,
            vec![
                "create".to_string(),
                "tank".to_string(),
                "/dev/sda".to_string(),
                "/dev/sdb".to_string()
            ]
        );
    }

    #[test]
    fn zpool_create_argv_empty_devices() {
        let argv = zpool_create_argv("z", &[]);
        assert_eq!(argv, vec!["create".to_string(), "z".to_string()]);
    }

    #[test]
    fn zpool_destroy_argv_force_and_pool_name() {
        assert_eq!(
            zpool_destroy_argv("my-pool"),
            [
                "destroy".to_string(),
                "-f".to_string(),
                "my-pool".to_string()
            ]
        );
    }

    #[test]
    fn zpool_scrub_argv_pool_name() {
        assert_eq!(
            zpool_scrub_argv("tank"),
            ["scrub".to_string(), "tank".to_string()]
        );
    }
}
