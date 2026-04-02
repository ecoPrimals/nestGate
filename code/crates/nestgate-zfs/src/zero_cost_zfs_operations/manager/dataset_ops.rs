// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dataset create and list (inherent implementation).

use super::ZeroCostZfsManager;
use crate::error::{ZfsOperation, create_zfs_error};
use crate::zero_cost_zfs_operations::types::{ZeroCostDatasetInfo, ZeroCostPoolInfo};
use nestgate_core::Result;
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Build `zfs create` arguments for a tier and dataset path (no process execution).
pub fn build_dataset_create_zfs_args<'a>(
    tier: &StorageTier,
    dataset_path: &'a str,
) -> Vec<&'a str> {
    let mut args: Vec<&str> = vec!["create"];
    match tier {
        StorageTier::Hot => {
            args.extend_from_slice(&["-o", "compression=lz4", "-o", "sync=always"]);
        }
        StorageTier::Warm => {
            args.extend_from_slice(&["-o", "compression=gzip", "-o", "sync=standard"]);
        }
        StorageTier::Cold => {
            args.extend_from_slice(&["-o", "compression=gzip-9", "-o", "sync=disabled"]);
        }
        StorageTier::Cache => {
            args.extend_from_slice(&[
                "-o",
                "compression=lz4",
                "-o",
                "sync=always",
                "-o",
                "primarycache=all",
            ]);
        }
        StorageTier::Archive => {
            args.extend_from_slice(&[
                "-o",
                "compression=gzip-9",
                "-o",
                "sync=disabled",
                "-o",
                "atime=off",
            ]);
        }
    }
    args.push(dataset_path);
    args
}

/// Parse one `zfs list -H` line (`name\tused\tavail\tmountpoint`) for datasets under `pool_name`.
pub fn parse_dataset_list_line(
    line: &str,
    pool_name: &str,
    created_at: SystemTime,
) -> Option<ZeroCostDatasetInfo> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 4 || parts[0] == pool_name {
        return None;
    }
    let full_name = parts[0].to_string();
    let name = full_name
        .strip_prefix(&format!("{pool_name}/"))
        .unwrap_or(&full_name)
        .to_string();
    let used = parts[1].parse().unwrap_or(0);
    let available = parts[2].parse().unwrap_or(0);
    let size = used + available;
    let mount_point = if parts[3] != "-" && parts[3] != "none" {
        Some(PathBuf::from(parts[3]))
    } else {
        None
    };

    Some(ZeroCostDatasetInfo {
        name,
        pool: pool_name.to_string(),
        tier: StorageTier::Warm,
        size,
        used,
        properties: HashMap::new(),
        mount_point,
        created_at,
    })
}

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    pub(super) async fn dataset_create(
        &self,
        pool: &ZeroCostPoolInfo,
        name: &str,
        tier: StorageTier,
    ) -> Result<ZeroCostDatasetInfo> {
        if !self.can_create_more_datasets().await {
            return Err(create_zfs_error(
                "Cannot create dataset: maximum datasets reached".to_string(),
                ZfsOperation::DatasetCreate,
            ));
        }

        let dataset_path = format!("{}/{}", pool.name, name);
        let args = build_dataset_create_zfs_args(&tier, &dataset_path);

        self.execute_zfs_command(&args).await?;

        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", &dataset_path])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        let used = properties
            .get("used")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let available = properties
            .get("available")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let size = used + available;

        let mount_point = properties
            .get("mountpoint")
            .filter(|mp| *mp != "none" && *mp != "-")
            .map(PathBuf::from);

        let dataset_info = ZeroCostDatasetInfo {
            name: name.to_string(),
            pool: pool.name.clone(),
            tier,
            size,
            used,
            properties: properties.clone(),
            mount_point,
            created_at: SystemTime::now(),
        };

        {
            let mut datasets_map = self.datasets.write().await;
            datasets_map.insert(dataset_info.name.clone(), dataset_info.clone());
        }
        Ok(dataset_info)
    }

    pub(super) async fn dataset_list(
        &self,
        pool: &ZeroCostPoolInfo,
    ) -> Result<Vec<ZeroCostDatasetInfo>> {
        let output = self
            .execute_zfs_command(&[
                "list",
                "-r",
                "-H",
                "-o",
                "name,used,avail,mountpoint",
                &pool.name,
            ])
            .await?;

        let mut datasets = Vec::with_capacity(MAX_DATASETS);

        for line in output.lines() {
            if let Some(info) = parse_dataset_list_line(line, &pool.name, SystemTime::now()) {
                datasets.push(info);
                if datasets.len() >= MAX_DATASETS {
                    break;
                }
            }
        }

        Ok(datasets)
    }
}
