// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Listing and read-only queries for ZFS datasets.

use super::NativeZfsDatasetManager;
use crate::types::DatasetInfo;
use nestgate_core::Result;
use std::collections::HashMap;

impl NativeZfsDatasetManager {
    /// List all datasets
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_datasets(&self) -> Result<Vec<String>> {
        let output = self
            .command_executor
            .execute_command_expect_success(&["list", "-H", "-o", "name", "-t", "filesystem"])
            .await?;

        Ok(output
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect())
    }

    /// Get dataset information
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_info(&self, dataset_name: &str) -> Result<DatasetInfo> {
        let properties = self.command_executor.get_dataset_info(dataset_name).await?;

        let used_bytes = properties
            .get("used")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let available_bytes = properties
            .get("available")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let mount_point = properties
            .get("mountpoint")
            .cloned()
            .unwrap_or_else(|| "none".into());

        let compression_ratio = properties
            .get("compressratio")
            .and_then(|s| s.trim_end_matches('x').parse::<f64>().ok());

        Ok(DatasetInfo {
            name: dataset_name.to_string(),
            full_name: dataset_name.to_string(),
            pool: dataset_name.split('/').next().unwrap_or("").to_string(),
            size: used_bytes + available_bytes,
            used: used_bytes,
            available: available_bytes,
            mountpoint: if mount_point.is_empty() || mount_point == "none" {
                None
            } else {
                Some(std::path::PathBuf::from(&mount_point))
            },
            mount_point: if mount_point.is_empty() || mount_point == "none" {
                None
            } else {
                Some(std::path::PathBuf::from(mount_point))
            },
            dataset_type: "filesystem".to_string(),
            compression: compression_ratio.map_or_else(|| "lz4".into(), |r| r.to_string()),
            checksum: "sha256".into(), // Default checksum
            referenced: used_bytes,    // Approximation
            compression_ratio: compression_ratio.unwrap_or(1.0),
            tier: nestgate_core::canonical_types::StorageTier::Warm, // Default tier
            properties,
            created_at: std::time::SystemTime::now(),
        })
    }

    /// Get dataset property
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_property(&self, dataset_name: &str, property: &str) -> Result<String> {
        let output = self
            .command_executor
            .execute_command_expect_success(&["get", "-H", "-o", "value", property, dataset_name])
            .await?;

        Ok(output.trim().to_string())
    }

    /// Get dataset usage statistics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_usage(&self, dataset_name: &str) -> Result<HashMap<String, u64>> {
        let properties = self.command_executor.get_dataset_info(dataset_name).await?;

        let mut usage = HashMap::new();

        // Parse numeric properties
        for (key, value) in properties {
            if let Ok(bytes) = value.parse::<u64>() {
                match key.as_str() {
                    "used" | "available" | "referenced" | "compressratio" | "written" => {
                        usage.insert(key, bytes);
                    }
                    _ => {}
                }
            }
        }

        Ok(usage)
    }
}
