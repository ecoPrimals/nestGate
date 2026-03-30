// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS CLI execution and tabular property parsing.

use super::ZeroCostZfsManager;
use crate::error::{ZfsOperation, create_zfs_error};
use nestgate_core::Result;
use std::collections::HashMap;

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    /// Execute ZFS command with compile-time timeout
    pub(super) async fn execute_zfs_command(&self, args: &[&str]) -> Result<String> {
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args(args);

        let output = tokio::time::timeout(Self::command_timeout(), cmd.output())
            .await
            .map_err(|_| {
                create_zfs_error(
                    "ZFS command timed out after self.base_url".to_string(),
                    ZfsOperation::Command,
                )
            })?
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to execute ZFS command: self.base_url".to_string(),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "ZFS command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Parse pool properties from ZFS output
    pub(super) fn parse_pool_properties(&self, output: &str) -> HashMap<String, String> {
        let mut properties = HashMap::new();

        for line in output.lines() {
            if let Some((key, value)) = line.split_once('\t') {
                properties.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        properties
    }
}
