// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(clippy::redundant_pub_crate)]

//! Parsing and tier inference for ZFS dataset listings.

use super::types::DatasetInfo;
use nestgate_core::canonical_types::StorageTier as CoreStorageTier;
use std::collections::HashMap;
use std::time::SystemTime;

/// Infer storage tier from dataset name hints (used when listing without property queries).
pub(crate) fn tier_hint_from_dataset_name(name: &str) -> CoreStorageTier {
    if name.contains("hot") {
        CoreStorageTier::Hot
    } else if name.contains("cold") {
        CoreStorageTier::Cold
    } else {
        CoreStorageTier::Warm
    }
}

/// Parse one tab-separated line from `zfs list -H -o name,used,avail,mountpoint`.
pub(crate) fn parse_zfs_dataset_list_line(dataset_name: &str, line: &str) -> Option<DatasetInfo> {
    let fields: Vec<&str> = line.split('\t').collect();
    if fields.len() < 4 {
        return None;
    }
    let used_space = fields[1].parse::<u64>().unwrap_or(0);
    let available_space = fields[2].parse::<u64>().unwrap_or(0);
    let mount_point = fields[3].to_string();
    let tier = tier_hint_from_dataset_name(dataset_name);
    Some(DatasetInfo {
        name: dataset_name.to_string(),
        used_space,
        available_space,
        file_count: None,
        compression_ratio: None,
        mount_point,
        tier,
        properties: HashMap::new(),
    })
}

/// Parse one line from `zfs list -t snapshot -H -o name,used,referenced,creation`.
pub(crate) fn parse_zfs_snapshot_list_line(
    line: &str,
    dataset_name: &str,
) -> Option<crate::snapshot::SnapshotInfo> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 4 {
        return None;
    }
    let full_name = parts[0].to_string();
    let name = full_name
        .split('@')
        .next_back()
        .unwrap_or(&full_name)
        .to_string();
    let used_space: u64 = parts[1].parse().unwrap_or(0);
    let referenced_size: u64 = parts[2].parse().unwrap_or(0);

    Some(crate::snapshot::SnapshotInfo {
        name,
        full_name,
        dataset: dataset_name.to_string(),
        created_at: SystemTime::now(),
        size: used_space,
        referenced_size,
        written_size: used_space,
        compression_ratio: 1.0,
        properties: HashMap::new(),
        policy: None,
        tier: CoreStorageTier::Warm,
        protected: false,
        tags: Vec::new(),
    })
}

/// Parse one line from `zfs list -H -p -o name,used,avail,mountpoint` (pool-wide listing).
pub(crate) fn parse_zfs_list_datasets_row(line: &str) -> Option<DatasetInfo> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 4 {
        return None;
    }
    let name = parts[0].to_string();
    let used_space: u64 = parts[1].parse().unwrap_or(0);
    let available_space: u64 = parts[2].parse().unwrap_or(0);
    let mount_point = parts[3].to_string();
    Some(DatasetInfo {
        name: name.clone(),
        used_space,
        available_space,
        file_count: None,
        compression_ratio: None,
        mount_point,
        tier: tier_hint_from_dataset_name(&name),
        properties: HashMap::new(),
    })
}
