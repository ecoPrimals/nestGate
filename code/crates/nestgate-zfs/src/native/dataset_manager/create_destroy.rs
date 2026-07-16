// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dataset create and destroy operations.

use super::{DatasetCreateOptions, NativeZfsDatasetManager};
use nestgate_core::Result;
use std::collections::HashMap;
use tracing::info;

impl NativeZfsDatasetManager {
    /// Create a new dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_dataset(
        &self,
        dataset_name: &str,
        options: &DatasetCreateOptions,
    ) -> Result<()> {
        let mut properties = HashMap::new();

        // Set compression
        if let Some(compression) = &options.compression {
            properties.insert("compression".into(), compression.clone());
        }

        // Set deduplication
        if let Some(dedup) = options.deduplication {
            properties.insert(
                "dedup".into(),
                if dedup {
                    "on".into()
                } else {
                    "off".into()
                },
            );
        }

        // Set encryption
        if let Some(encryption) = &options.encryption {
            properties.insert("encryption".into(), encryption.clone());
        }

        // Set mount point
        if let Some(mount_point) = &options.mount_point {
            properties.insert("mountpoint".into(), mount_point.clone());
        }

        // Set quota
        if let Some(quota) = options.quota {
            properties.insert("quota".into(), quota.to_string());
        }

        // Set reservation
        if let Some(reservation) = options.reservation {
            properties.insert("reservation".into(), reservation.to_string());
        }

        // Set record size
        if let Some(record_size) = &options.record_size {
            properties.insert("recordsize".into(), record_size.clone());
        }

        // Create the dataset
        self.command_executor
            .create_dataset(dataset_name, &properties)
            .await?;

        info!("Created ZFS dataset: {}", dataset_name);
        Ok(())
    }

    /// Destroy a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn destroy_dataset(&self, dataset_name: &str, force: bool) -> Result<()> {
        let mut args = vec!["destroy"];
        if force {
            args.push("-f");
        }
        args.push(dataset_name);

        self.command_executor
            .execute_command_expect_success(&args)
            .await?;

        info!("Destroyed ZFS dataset: {}", dataset_name);
        Ok(())
    }
}
