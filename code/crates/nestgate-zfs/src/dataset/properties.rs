// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Dataset property get/set.

use crate::error::{ZfsOperation, create_zfs_error};
use nestgate_core::Result;
use std::collections::HashMap;
use tokio::process::Command;

use super::types::ZfsDatasetManager;

impl ZfsDatasetManager {
    /// Get dataset properties
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_properties(&self, name: &str) -> Result<HashMap<String, String>> {
        tracing::debug!("Getting properties for dataset: {}", name);

        let output = Command::new("zfs")
            .args(["get", "all", "-H", "-p", name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!(
                        "Failed to get dataset properties: {}",
                        "actual_error_details"
                    ),
                    ZfsOperation::Configuration,
                )
            })?;

        if !output.status.success() {
            return Ok(HashMap::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut properties = HashMap::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                properties.insert(parts[1].to_string(), parts[2].to_string());
            }
        }

        Ok(properties)
    }

    /// Set dataset properties
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> Result<()> {
        tracing::info!("Setting properties for dataset: {}", name);

        for (key, value) in properties {
            let output = Command::new("zfs")
                .args(["set", &format!("{key}={value}"), name])
                .output()
                .await
                .map_err(|_e| {
                    create_zfs_error(
                        format!(
                            "Failed to set property {key}={value}: {}",
                            "actual_error_details"
                        ),
                        ZfsOperation::Configuration,
                    )
                })?;

            if !output.status.success() {
                return Err(create_zfs_error(
                    format!(
                        "Failed to set property {}={}: {}",
                        key,
                        value,
                        String::from_utf8_lossy(&output.stderr)
                    ),
                    ZfsOperation::Configuration,
                ));
            }
        }

        Ok(())
    }
}
