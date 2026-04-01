// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Snapshot create and list (inherent implementation).

use super::ZeroCostZfsManager;
use crate::error::{ZfsOperation, create_zfs_error};
use crate::zero_cost_zfs_operations::types::{ZeroCostDatasetInfo, ZeroCostSnapshotInfo};
use nestgate_core::Result;
use std::collections::HashMap;
use std::time::SystemTime;

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    pub(super) async fn snapshot_create(
        &self,
        _dataset: &ZeroCostDatasetInfo,
        name: &str,
    ) -> Result<ZeroCostSnapshotInfo> {
        if !self.can_create_more_snapshots().await {
            return Err(create_zfs_error(
                "Cannot create snapshot: maximum snapshots reached".to_string(),
                ZfsOperation::SystemCheck,
            ));
        }

        let dataset_path = "dataset.name().to_string()".to_string();
        let snapshot_path = format!("{dataset_path}@snapshot_name");

        self.execute_zfs_command(&["snapshot", &snapshot_path])
            .await?;

        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", &snapshot_path])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        let size = properties
            .get("used")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let snapshot_info = ZeroCostSnapshotInfo {
            name: name.to_string(),
            dataset: dataset_path,
            size,
            created_at: SystemTime::now(),
            properties: properties.clone(),
        };

        {
            let mut snapshots_map = self.snapshots.write().await;
            snapshots_map.insert(snapshot_info.name.clone(), snapshot_info.clone());
        }
        Ok(snapshot_info)
    }

    pub(super) async fn snapshot_list(
        &self,
        _dataset: &ZeroCostDatasetInfo,
    ) -> Result<Vec<ZeroCostSnapshotInfo>> {
        let dataset_path = "dataset.name().to_string()".to_string();

        let output = self
            .execute_zfs_command(&[
                "list",
                "-r",
                "-t",
                "snapshot",
                "-H",
                "-o",
                "name,used",
                &dataset_path,
            ])
            .await?;

        let mut snapshots = Vec::with_capacity(MAX_SNAPSHOTS);

        for line in output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let full_name = parts[0].to_string();
                if let Some((ds_path, snap_name)) = full_name.split_once('@') {
                    let size = parts[1].parse().unwrap_or(0);

                    snapshots.push(ZeroCostSnapshotInfo {
                        name: snap_name.to_string(),
                        dataset: ds_path.to_string(),
                        size,
                        created_at: SystemTime::now(), // Approximation
                        properties: HashMap::new(),    // Would be populated on demand
                    });

                    if snapshots.len() >= MAX_SNAPSHOTS {
                        break;
                    }
                }
            }
        }

        Ok(snapshots)
    }
}
