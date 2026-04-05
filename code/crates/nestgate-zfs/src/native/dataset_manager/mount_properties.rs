// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Mount, unmount, and property updates for ZFS datasets.

use super::NativeZfsDatasetManager;
use nestgate_core::Result;
use tracing::info;

impl NativeZfsDatasetManager {
    /// Set dataset property
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn set_property(
        &self,
        dataset_name: &str,
        property: &str,
        value: &str,
    ) -> Result<()> {
        let propertyvalue = format!("{property}={value}");
        self.command_executor
            .execute_command_expect_success(&["set", &propertyvalue, dataset_name])
            .await?;

        info!(
            "✅ Set property {}={} on dataset {}",
            property, value, dataset_name
        );
        Ok(())
    }

    /// Mount a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn mount_dataset(&self, dataset_name: &str) -> Result<()> {
        self.command_executor
            .execute_command_expect_success(&["mount", dataset_name])
            .await?;

        info!("✅ Mounted ZFS dataset: {}", dataset_name);
        Ok(())
    }

    /// Unmount a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn unmount_dataset(&self, dataset_name: &str, force: bool) -> Result<()> {
        let mut args = vec!["unmount"];
        if force {
            args.push("-f");
        }
        args.push(dataset_name);

        self.command_executor
            .execute_command_expect_success(&args)
            .await?;

        info!("✅ Unmounted ZFS dataset: {}", dataset_name);
        Ok(())
    }
}
