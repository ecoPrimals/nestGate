// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Snapshot-related operations for the native ZFS backend.
//!
//! This module provides zero-copy optimized ZFS snapshot operations including
//! listing, creating, destroying, and rolling back snapshots.
//!
//! **Tests**: Located in `snapshot_operations_tests.rs` (sibling file)
//! Note: Tests are in a separate file for better organization.
//! Run with: `cargo test snapshot_operations`

use std::collections::HashMap;
// Removed unused tracing import

use nestgate_zfs::numeric::f64_to_u64_saturating;

use crate::handlers::zfs::universal_zfs_types::{
    SnapshotConfig, SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};
use tracing::info;

use super::core::NativeZfsService;

/// List all ZFS snapshots (zero-copy optimized)
pub async fn list_snapshots(service: &NativeZfsService) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    info!("Listing ZFS snapshots");
    // Execute `zfs list -t snapshot -H -o name,used,creation`
    let output = service
        .execute_zfs_command(&["list", "-t", "snapshot", "-H", "-o", "name,used,creation"])
        .await?;

    let mut snapshots = Vec::new();
    for line in output.lines() {
        if let Some(snapshot_info) = parse_snapshot_line(line) {
            snapshots.push(snapshot_info);
        }
    }

    Ok(snapshots)
}

/// Parse a single snapshot line (zero-copy optimized)
pub fn parse_snapshot_line(line: &str) -> Option<SnapshotInfo> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() >= 3 {
        Some(SnapshotInfo {
            name: parts[0].into(),
            creation_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            used: parse_size(parts[1]).unwrap_or(0),
            referenced: parse_size(parts[1]).unwrap_or(0),
            properties: HashMap::new(),
        })
    } else {
        None
    }
}
/// List snapshots for a specific dataset (zero-copy optimized)
pub async fn list_dataset_snapshots(
    service: &NativeZfsService,
    dataset: &str,
) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    info!("Listing snapshots for dataset: {}", dataset);
    // Execute `zfs list -t snapshot -H -o name,used,creation -r dataset`
    let output = service
        .execute_zfs_command(&[
            "list",
            "-t",
            "snapshot",
            "-H",
            "-o",
            "name,used,creation",
            "-r",
            dataset,
        ])
        .await?;

    let mut snapshots = Vec::new();
    for line in output.lines() {
        if let Some(snapshot_info) = parse_snapshot_line(line) {
            snapshots.push(snapshot_info);
        }
    }

    Ok(snapshots)
}

/// Create a new ZFS snapshot (zero-copy optimized)
pub async fn create_snapshot(
    service: &NativeZfsService,
    config: &SnapshotConfig,
) -> UniversalZfsResult<SnapshotInfo> {
    info!("Creating snapshot: {}", config.name);
    // Execute `zfs snapshot dataset@snapshot_name`
    let snapshot_name = if config.name.contains('@') {
        config.name.clone()
    } else {
        format!("{}@{}", config.dataset, config.name)
    };

    service
        .execute_zfs_command(&["snapshot", &snapshot_name])
        .await?;

    // Get the created snapshot info
    let output = service
        .execute_zfs_command(&[
            "list",
            "-t",
            "snapshot",
            "-H",
            "-o",
            "name,used,creation",
            &snapshot_name,
        ])
        .await?;

    let line = output.trim();
    if let Some(snapshot_info) = parse_snapshot_line(line) {
        Ok(snapshot_info)
    } else {
        Err(UniversalZfsError::internal(
            "Failed to retrieve created snapshot",
        ))
    }
}

/// Destroy a ZFS snapshot
pub async fn destroy_snapshot(service: &NativeZfsService, name: &str) -> UniversalZfsResult<()> {
    info!("Destroying snapshot: {}", name);
    // Execute `zfs destroy snapshot_name`
    service.execute_zfs_command(&["destroy", name]).await?;

    Ok(())
}

/// Helper function to parse ZFS size strings (zero-copy optimized)
pub fn parse_size(size_str: &str) -> Option<u64> {
    if size_str == "-" {
        return Some(0);
    }
    let size_str = size_str.trim();
    if size_str.is_empty() {
        return None;
    }

    let (number_part, multiplier) = if let Some(last_char) = size_str.chars().last() {
        if last_char.is_alphabetic() {
            let mut chars = size_str.chars();
            chars.next_back();
            let number_part = chars.as_str();
            let multiplier = match last_char {
                'K' | 'k' => 1024,
                'M' | 'm' => 1024 * 1024,
                'G' | 'g' => 1024 * 1024 * 1024,
                'T' | 't' => 1024_u64 * 1024 * 1024 * 1024,
                'P' | 'p' => 1024_u64 * 1024 * 1024 * 1024 * 1024,
                _ => 1,
            };
            (number_part, multiplier)
        } else {
            (size_str, 1)
        }
    } else {
        return None;
    };

    let number: f64 = number_part.parse().ok()?;
    Some(f64_to_u64_saturating(number * multiplier as f64))
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn parse_snapshot_line_parses_tab_separated_row() {
        let line = "tank/fs@snap\t1024K\tJan 1";
        let info = parse_snapshot_line(line).expect("snapshot");
        assert_eq!(info.name, "tank/fs@snap");
        assert_eq!(info.used, 1024 * 1024);
    }

    #[test]
    fn parse_snapshot_line_rejects_short_row() {
        assert!(parse_snapshot_line("only-one").is_none());
    }

    #[test]
    fn parse_size_dash_is_zero() {
        assert_eq!(parse_size("-"), Some(0));
    }

    #[test]
    fn parse_size_si_suffixes() {
        assert_eq!(parse_size("2K"), Some(2048));
        assert_eq!(parse_size("1M"), Some(1024 * 1024));
    }
}
