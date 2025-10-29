//
// Contains all snapshot-related operations for the native ZFS backend.

use std::collections::HashMap;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::types::{
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
fn parse_snapshot_line(line: &str) -> Option<SnapshotInfo> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() >= 3 {
        Some(SnapshotInfo {
            name: parts[0].into(),
            dataset: parts[0].split('@').next().unwrap_or("").into(),
            created_at: std::time::SystemTime::now(),
            size_bytes: parse_size(parts[1]).unwrap_or(0),
            properties: HashMap::new(),
            description: None,
        })
    } else {
        None
    }
}
/// List snapshots for a specific dataset (zero-copy optimized)
pub fn list_dataset_snapshots(
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
pub fn create_snapshot(
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
pub fn destroy_snapshot(service: &NativeZfsService, name: &str) -> UniversalZfsResult<()> {
    info!("Destroying snapshot: {}", name);
    // Execute `zfs destroy snapshot_name`
    service.execute_zfs_command(&["destroy", name]).await?;

    Ok(())
}

/// Helper function to parse ZFS size strings (zero-copy optimized)
fn parse_size(size_str: &str) -> Option<u64> {
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
    Some((number * multiplier as f64) as u64)
}
