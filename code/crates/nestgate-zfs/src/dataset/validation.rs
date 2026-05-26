// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(clippy::redundant_pub_crate)]

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_hint_hot() {
        assert!(matches!(
            tier_hint_from_dataset_name("pool/hot-data"),
            CoreStorageTier::Hot
        ));
    }

    #[test]
    fn tier_hint_cold() {
        assert!(matches!(
            tier_hint_from_dataset_name("archive/cold-storage"),
            CoreStorageTier::Cold
        ));
    }

    #[test]
    fn tier_hint_default_warm() {
        assert!(matches!(
            tier_hint_from_dataset_name("tank/data"),
            CoreStorageTier::Warm
        ));
    }

    #[test]
    fn parse_dataset_list_line_valid() {
        let line = "pool/data\t1024\t2048\t/pool/data";
        let info = parse_zfs_dataset_list_line("pool/data", line).expect("valid line");
        assert_eq!(info.name, "pool/data");
        assert_eq!(info.used_space, 1024);
        assert_eq!(info.available_space, 2048);
        assert_eq!(info.mount_point, "/pool/data");
    }

    #[test]
    fn parse_dataset_list_line_too_few_fields() {
        assert!(parse_zfs_dataset_list_line("pool", "pool\t100").is_none());
    }

    #[test]
    fn parse_dataset_list_line_non_numeric_uses_zero() {
        let line = "pool/x\tNaN\tbad\t/mnt";
        let info = parse_zfs_dataset_list_line("pool/x", line).expect("fallback to 0");
        assert_eq!(info.used_space, 0);
        assert_eq!(info.available_space, 0);
    }

    #[test]
    fn parse_snapshot_list_line_valid() {
        let line = "pool/data@snap1\t512\t4096\tMon May 26 12:00 2025";
        let snap = parse_zfs_snapshot_list_line(line, "pool/data").expect("valid snapshot");
        assert_eq!(snap.name, "snap1");
        assert_eq!(snap.full_name, "pool/data@snap1");
        assert_eq!(snap.dataset, "pool/data");
        assert_eq!(snap.size, 512);
        assert_eq!(snap.referenced_size, 4096);
    }

    #[test]
    fn parse_snapshot_list_line_too_few_fields() {
        assert!(parse_zfs_snapshot_list_line("pool@snap\t100", "pool").is_none());
    }

    #[test]
    fn parse_list_datasets_row_valid() {
        let line = "tank/hot-ssd\t8192\t16384\t/tank/hot-ssd";
        let info = parse_zfs_list_datasets_row(line).expect("valid row");
        assert_eq!(info.name, "tank/hot-ssd");
        assert_eq!(info.used_space, 8192);
        assert!(matches!(info.tier, CoreStorageTier::Hot));
    }

    #[test]
    fn parse_list_datasets_row_short_line() {
        assert!(parse_zfs_list_datasets_row("tank\t100").is_none());
    }
}
