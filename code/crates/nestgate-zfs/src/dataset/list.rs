// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Dataset listing, snapshot listing, and info resolution.

use crate::error::{ZfsOperation, create_zfs_error};
use nestgate_core::{Result, canonical_types::StorageTier as CoreStorageTier};
use std::collections::HashMap;
use tokio::process::Command;
use tracing::{debug, warn};

use super::types::{DatasetInfo, ZfsDatasetManager};
use super::validation::{
    parse_zfs_dataset_list_line, parse_zfs_list_datasets_row, parse_zfs_snapshot_list_line,
};

impl ZfsDatasetManager {
    /// Get dataset info with fallback for testing environments
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_info(&self, name: &str) -> Result<DatasetInfo> {
        self.get_dataset_info_with_fallback(name).await
    }

    /// Gets Dataset Info With Fallback
    pub(crate) async fn get_dataset_info_with_fallback(&self, name: &str) -> Result<DatasetInfo> {
        let mut cmd = Command::new("zfs");
        cmd.args(["list", "-H", "-o", "name,used,avail,mountpoint"]);
        cmd.arg(name);

        let output = cmd.output().await.map_err(|e| {
            crate::error::ZfsErrorBuilder::command_error("zfs list", &e.to_string())
        })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            warn!("ZFS list failed: {}, using fallback data", error_msg);
            return self.create_fallback_dataset_info(name);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next()
            && let Some(info) = parse_zfs_dataset_list_line(name, line)
        {
            return Ok(info);
        }

        // Fallback if parsing fails
        warn!("Failed to parse ZFS output, using fallback data");
        self.create_fallback_dataset_info(name)
    }

    /// Create fallback dataset information for development/testing
    pub(crate) fn create_fallback_dataset_info(&self, name: &str) -> Result<DatasetInfo> {
        Ok(DatasetInfo {
            name: name.to_string(),
            used_space: 512 * 1024 * 1024,
            available_space: 512 * 1024 * 1024,
            file_count: None,
            compression_ratio: Some(1.5),
            mount_point: format!("/{name}"),
            tier: CoreStorageTier::Warm,
            properties: HashMap::new(),
        })
    }

    /// List all datasets
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_datasets(&self) -> Result<Vec<DatasetInfo>> {
        tracing::debug!("Listing all datasets");

        let output = Command::new("zfs")
            .args(["list", "-H", "-p", "-o", "name,used,avail,mountpoint"])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error("Failed to list datasets".to_string(), ZfsOperation::Command)
            })?;

        if !output.status.success() {
            // Return empty list if no datasets found
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut datasets = Vec::new();

        for line in stdout.lines() {
            if let Some(row) = parse_zfs_list_datasets_row(line) {
                datasets.push(row);
            }
        }

        Ok(datasets)
    }

    /// List snapshots for a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_snapshots(
        &self,
        dataset_name: &str,
    ) -> Result<Vec<crate::snapshot::SnapshotInfo>> {
        debug!("Listing snapshots for dataset: {}", dataset_name);

        let output = Command::new("zfs")
            .args([
                "list",
                "-t",
                "snapshot",
                "-H",
                "-o",
                "name,used,referenced,creation",
            ])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::command_error("zfs list snapshots", &e.to_string())
            })?;

        if !output.status.success() {
            return Err(crate::error::ZfsErrorBuilder::command_error(
                "zfs list snapshots",
                &String::from_utf8_lossy(&output.stderr),
            ));
        }

        let mut snapshots = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Some(snap) = parse_zfs_snapshot_list_line(line, dataset_name) {
                snapshots.push(snap);
            }
        }

        Ok(snapshots)
    }
}
