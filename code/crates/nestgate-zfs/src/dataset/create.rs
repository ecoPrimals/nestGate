// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dataset create and destroy operations.

use crate::error::{ZfsOperation, create_zfs_error};
use nestgate_core::{Result, canonical_types::StorageTier as CoreStorageTier};
use std::collections::HashMap;
use tokio::process::Command;
use tracing::{info, warn};

use super::types::{DatasetInfo, ZfsDatasetManager};

/// `-o` argument pairs passed to `zfs create` for each [`CoreStorageTier`].
///
/// Exposed for unit tests so tier-specific command construction is covered without invoking `zfs`.
pub(super) const fn zfs_create_tier_property_args(
    tier: &CoreStorageTier,
) -> &'static [&'static str] {
    match tier {
        CoreStorageTier::Hot => &["-o", "compression=off", "-o", "recordsize=128K"],
        CoreStorageTier::Warm => &["-o", "compression=lz4", "-o", "recordsize=128K"],
        CoreStorageTier::Cold => &["-o", "compression=zstd", "-o", "recordsize=1M"],
        CoreStorageTier::Cache => &["-o", "compression=off", "-o", "recordsize=64K"],
        CoreStorageTier::Archive => &["-o", "compression=gzip-9", "-o", "recordsize=1M"],
    }
}

impl ZfsDatasetManager {
    /// Create a new ZFS dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_dataset(
        &self,
        name: &str,
        parent: &str,
        tier: CoreStorageTier,
    ) -> Result<DatasetInfo> {
        info!("Creating dataset: {}/{} on tier: {:?}", parent, name, tier);

        let dataset_path = format!("{parent}/{name}");

        // Execute ZFS create command
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args(["create"]);

        cmd.args(zfs_create_tier_property_args(&tier));

        cmd.arg(&dataset_path);

        let output = cmd.output().await.map_err(|e| {
            crate::error::ZfsErrorBuilder::command_error("zfs create", &e.to_string())
        })?;

        if !output.status.success() {
            return Err(crate::error::ZfsErrorBuilder::command_error(
                "zfs create",
                &String::from_utf8_lossy(&output.stderr),
            ));
        }

        // Return basic dataset info
        Ok(DatasetInfo {
            name: name.to_string(),
            mount_point: format!("/{name}"),
            used_space: 0,
            available_space: 0,
            file_count: None,
            compression_ratio: None,
            tier,
            properties: HashMap::new(),
        })
    }

    /// Create a dataset with fallback for testing/development environments
    async fn create_with_fallback(
        &self,
        name: &str,
        pool: &str,
        tier: CoreStorageTier,
    ) -> Result<DatasetInfo> {
        info!("Creating dataset: {}/{} on tier: {:?}", pool, name, tier);

        // First try real ZFS dataset creation
        let dataset_path = format!("{pool}/{name}");
        let output = tokio::process::Command::new("zfs")
            .args(["create", &dataset_path])
            .output()
            .await;

        match output {
            Ok(result) if result.status.success() => {
                info!("Created ZFS dataset: {}", dataset_path);
                self.get_dataset_info_with_fallback(&dataset_path).await
            }
            Ok(result) => {
                let error_msg = String::from_utf8_lossy(&result.stderr);
                warn!("ZFS dataset creation failed: {}, using fallback", error_msg);

                // Return fallback dataset info for development
                Ok(DatasetInfo {
                    name: name.to_string(),
                    used_space: 0,
                    available_space: 1024 * 1024 * 1024,
                    file_count: None,
                    compression_ratio: Some(1.0),
                    mount_point: format!("/{name}"),
                    tier,
                    properties: HashMap::new(),
                })
            }
            Err(e) => {
                warn!("Failed to execute ZFS command: {}, using fallback", e);

                // Return fallback dataset info when ZFS is not available
                Ok(DatasetInfo {
                    name: name.to_string(),
                    used_space: 0,
                    available_space: 1024 * 1024 * 1024,
                    file_count: None,
                    compression_ratio: Some(1.0),
                    mount_point: format!("/{name}"),
                    tier,
                    properties: HashMap::new(),
                })
            }
        }
    }

    /// Create a new dataset with full configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_dataset_with_config(&self, name: &str, parent: &str) -> Result<()> {
        tracing::info!("Creating dataset: {}/{}", parent, name);

        let full_name = format!("{parent}/{name}");

        // Build the zfs create command with properties from unified config
        let mut args = vec!["create"];
        let mut options = Vec::new();

        // Add compression property from config
        let compression_opt = "compression=lz4".to_string(); // Canonical default
        options.push(compression_opt);

        // Use canonical defaults instead of complex config extensions
        // Record size optimization
        let recordsize_opt = "recordsize=128K".to_string();
        options.push(recordsize_opt);

        // Quota and reservation handled by canonical storage config if needed

        // Add all options to args
        for option in &options {
            args.extend(&["-o", option.as_str()]);
        }

        // Add the dataset name
        args.push(&full_name);

        let output = Command::new("zfs")
            .args(&args)
            .output()
            .await
            .map_err(|e| {
                create_zfs_error(
                    format!("Failed to execute zfs create: {e}"),
                    ZfsOperation::DatasetCreate,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zfs create failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::DatasetCreate,
            ));
        }

        Ok(())
    }

    /// Delete (destroy) a ZFS dataset.
    ///
    /// Runs `zfs destroy <name>`. The caller is responsible for confirming
    /// the operation — this will permanently remove the dataset and its data.
    ///
    /// # Errors
    ///
    /// Returns an error if the `zfs destroy` command fails or is unavailable.
    pub async fn delete_dataset(&self, name: &str) -> Result<()> {
        info!("Deleting dataset: {}", name);

        let output = tokio::process::Command::new("zfs")
            .args(["destroy", name])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::command_error("zfs destroy", &e.to_string())
            })?;

        if !output.status.success() {
            return Err(crate::error::ZfsErrorBuilder::command_error(
                "zfs destroy",
                &String::from_utf8_lossy(&output.stderr),
            ));
        }

        info!("Destroyed dataset: {}", name);
        Ok(())
    }

    /// Destroy a dataset (alias for [`Self::delete_dataset`]).
    ///
    /// # Errors
    ///
    /// Same as [`Self::delete_dataset`].
    pub async fn destroy_dataset(&self, name: &str) -> Result<()> {
        self.delete_dataset(name).await
    }
}

#[cfg(test)]
mod create_dataset_tests {
    use nestgate_core::canonical_types::StorageTier as CoreStorageTier;

    use super::zfs_create_tier_property_args;

    #[test]
    fn tier_property_args_cover_all_tiers() {
        let hot = zfs_create_tier_property_args(&CoreStorageTier::Hot);
        assert!(hot.contains(&"-o") && hot.contains(&"compression=off"));

        let warm = zfs_create_tier_property_args(&CoreStorageTier::Warm);
        assert!(warm.contains(&"compression=lz4"));

        let cold = zfs_create_tier_property_args(&CoreStorageTier::Cold);
        assert!(cold.contains(&"compression=zstd"));

        let cache = zfs_create_tier_property_args(&CoreStorageTier::Cache);
        assert!(cache.contains(&"recordsize=64K"));

        let arch = zfs_create_tier_property_args(&CoreStorageTier::Archive);
        assert!(arch.contains(&"compression=gzip-9"));
    }

    #[tokio::test]
    async fn create_dataset_returns_error_when_zfs_unavailable_or_fails() {
        let mgr = crate::dataset::ZfsDatasetManager::new_for_testing();
        let err = mgr
            .create_dataset(
                "definitely_missing_dataset_xyz",
                "invalid_pool_zzzz",
                CoreStorageTier::Warm,
            )
            .await
            .expect_err("zfs create should fail in test env without a real pool");
        assert!(
            err.to_string().contains("zfs") || err.to_string().contains("command"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn delete_dataset_errors_on_missing_name() {
        let mgr = crate::dataset::ZfsDatasetManager::new_for_testing();
        let err = mgr
            .delete_dataset("nonexistent/pool/dataset")
            .await
            .expect_err("destroy should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("zfs destroy") || msg.contains("command"),
            "{msg}"
        );
    }

    #[tokio::test]
    async fn create_with_fallback_returns_synthetic_dataset_when_zfs_missing() {
        let mgr = crate::dataset::ZfsDatasetManager::new_for_testing();
        let info = mgr
            .create_with_fallback("nf_fallback_ds", "no_such_pool_zzzz", CoreStorageTier::Cold)
            .await
            .expect("fallback dataset");
        assert_eq!(info.tier, CoreStorageTier::Cold);
        assert_eq!(info.compression_ratio, Some(1.0));
        assert!(info.available_space >= 1024 * 1024);
    }

    #[tokio::test]
    async fn create_dataset_with_config_errors_when_zfs_fails() {
        let mgr = crate::dataset::ZfsDatasetManager::new_for_testing();
        let err = mgr
            .create_dataset_with_config("nf_cfg_child", "invalid_pool_root_xyz")
            .await
            .expect_err("zfs create should fail without pool");
        assert!(
            err.to_string().to_lowercase().contains("zfs")
                || err.to_string().to_lowercase().contains("create"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn destroy_dataset_delegates_to_delete() {
        let mgr = crate::dataset::ZfsDatasetManager::new_for_testing();
        let err = mgr
            .destroy_dataset("nonexistent/destroy/path")
            .await
            .expect_err("destroy should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("zfs destroy") || msg.contains("command"),
            "{msg}"
        );
    }
}
