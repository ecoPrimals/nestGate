// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Snapshot create and list (inherent implementation).

use super::ZeroCostZfsManager;
use crate::error::{ZfsOperation, create_zfs_error};
use crate::zero_cost_zfs_operations::types::{ZeroCostDatasetInfo, ZeroCostSnapshotInfo};
use nestgate_core::Result;
use std::collections::HashMap;
use std::time::SystemTime;

/// Full ZFS snapshot identifier: `pool/dataset@snap`.
pub fn build_snapshot_zfs_path(dataset_path: &str, snapshot_name: &str) -> String {
    format!("{dataset_path}@{snapshot_name}")
}

/// Parse one `zfs list -t snapshot -H` line (`name\tused`).
pub fn parse_snapshot_list_line(
    line: &str,
    created_at: SystemTime,
) -> Option<ZeroCostSnapshotInfo> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 2 {
        return None;
    }
    let full_name = parts[0].to_string();
    let (ds_path, snap_name) = full_name.split_once('@')?;
    let size = parts[1].parse().unwrap_or(0);

    Some(ZeroCostSnapshotInfo {
        name: snap_name.to_string(),
        dataset: ds_path.to_string(),
        size,
        created_at,
        properties: HashMap::new(),
    })
}

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    pub(super) async fn snapshot_create(
        &self,
        dataset: &ZeroCostDatasetInfo,
        name: &str,
    ) -> Result<ZeroCostSnapshotInfo> {
        if !self.can_create_more_snapshots().await {
            return Err(create_zfs_error(
                "Cannot create snapshot: maximum snapshots reached".to_string(),
                ZfsOperation::SystemCheck,
            ));
        }

        let dataset_path = format!("{}/{}", dataset.pool, dataset.name);
        let snapshot_path = build_snapshot_zfs_path(&dataset_path, name);

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
            dataset: dataset_path.clone(),
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
        dataset: &ZeroCostDatasetInfo,
    ) -> Result<Vec<ZeroCostSnapshotInfo>> {
        let dataset_path = format!("{}/{}", dataset.pool, dataset.name);

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
            if let Some(s) = parse_snapshot_list_line(line, SystemTime::now()) {
                snapshots.push(s);
                if snapshots.len() >= MAX_SNAPSHOTS {
                    break;
                }
            }
        }

        Ok(snapshots)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_snapshot_list_line;
    use std::time::SystemTime;

    #[test]
    fn parse_snapshot_list_line_accepts_extra_tab_separated_fields() {
        let t = SystemTime::UNIX_EPOCH;
        let s = parse_snapshot_list_line("pool/ds@snap\t1024\t2048\t2025-01-01", t)
            .expect("normal snapshot list line should parse");
        assert_eq!(s.dataset, "pool/ds");
        assert_eq!(s.name, "snap");
        assert_eq!(s.size, 1024);
        assert_eq!(s.created_at, t);
    }

    #[test]
    fn parse_snapshot_list_line_returns_none_when_too_few_fields() {
        assert!(
            parse_snapshot_list_line("only_one_field", SystemTime::UNIX_EPOCH).is_none(),
            "expected fewer than two tab fields to yield None"
        );
        assert!(
            parse_snapshot_list_line("x", SystemTime::UNIX_EPOCH).is_none(),
            "single token without tab should yield None"
        );
    }

    #[test]
    fn parse_snapshot_list_line_non_numeric_used_becomes_zero() {
        let s = parse_snapshot_list_line("pool/ds@snap\tnot_a_number", SystemTime::UNIX_EPOCH)
            .expect("line with @ should parse; bad size coerces to 0");
        assert_eq!(s.size, 0);
    }

    #[test]
    fn parse_snapshot_list_line_empty_line_is_none() {
        assert!(parse_snapshot_list_line("", SystemTime::UNIX_EPOCH).is_none());
    }
}
