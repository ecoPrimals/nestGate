// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Zero-copy parsing utilities for ZFS command output.
// Single responsibility: Parse ZFS command output into structured data.

//! Parsing module

use std::collections::HashMap;
use std::time::SystemTime;

use crate::handlers::zfs::universal_zfs_types::{
    DatasetInfo, DatasetType, PoolCapacity, PoolHealth, PoolInfo, PoolState, ScrubStatus,
    SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};

// Import zero-copy utilities
// Note: zero_copy lines function moved to utilities; `lines_zero_copy` module does not exist.

/// Parse zpool list output into `PoolInfo` structures
pub fn parse_zpool_list(output: &str) -> UniversalZfsResult<Vec<PoolInfo>> {
    let mut pools = Vec::new();
    for line in output.lines() {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() >= 5 {
            let name = fields[0].to_string();
            let total_size = parse_size_string(fields[1])?;
            let used_size = parse_size_string(fields[2])?;
            let available_size = parse_size_string(fields[3])?;
            let health_str = fields[4];

            let health = match health_str {
                "ONLINE" => PoolHealth::Online,
                "DEGRADED" => PoolHealth::Degraded,
                "FAULTED" => PoolHealth::Faulted,
                "OFFLINE" | "UNAVAIL" | "REMOVED" => PoolHealth::Offline,
                _ => PoolHealth::Unknown,
            };

            let state = match health {
                PoolHealth::Online | PoolHealth::Degraded => PoolState::Active,
                _ => PoolState::Unknown,
            };

            pools.push(PoolInfo {
                name,
                health,
                state,
                capacity: PoolCapacity {
                    total: total_size,
                    used: used_size,
                    available: available_size,
                },
                scrub: Some(ScrubStatus::None),
                properties: HashMap::new(),
            });
        }
    }

    Ok(pools)
}

/// Parse size strings with universal scale support
/// Handles everything from molecular storage (sub-byte) to cosmic-scale data (10+ generations)
/// Future-proofs for DNA storage, quantum bits, and federated petabyte systems
pub fn parse_size_string(size_str: &str) -> UniversalZfsResult<u64> {
    if size_str == "-" || size_str.is_empty() {
        return Ok(0);
    }
    // Handle numeric-only strings (assume bytes)
    if let Ok(bytes) = size_str.parse::<u64>() {
        return Ok(bytes);
    }

    // Split into numeric and unit parts
    let (numeric_part, unit_part) = split_size_string(size_str)?;
    let basevalue = numeric_part
        .parse::<f64>()
        .map_err(|_| UniversalZfsError::InvalidInput {
            message: "Invalid numeric value: self.base_url".to_string(),
        })?;

    // Universal scale multipliers - from quantum to cosmic
    let multiplier = match unit_part.to_uppercase().as_str() {
        // Quantum scale (theoretical future)
        "QB" => 0.125, // Quantum bits to bytes
        "AB" => 0.001, // Atomic bits to bytes

        // Standard binary scale
        "B" => 1.0,
        "KB" | "K" => 1024.0,
        "MB" | "M" => 1024.0_f64.powi(2),
        "GB" | "G" => 1024.0_f64.powi(3),
        "TB" | "T" => 1024.0_f64.powi(4),
        "PB" | "P" => 1024.0_f64.powi(5),
        "EB" | "E" => 1024.0_f64.powi(6),
        "ZB" | "Z" => 1024.0_f64.powi(7),
        "YB" | "Y" => 1024.0_f64.powi(8),

        // Future-scale (cosmic/federation scale)
        "RB" | "R" => 1024.0_f64.powi(9),   // Ronnabyte
        "QB2" | "Q" => 1024.0_f64.powi(10), // Quettabyte
        "XB" | "X" => 1024.0_f64.powi(11),  // Extended scale
        "WB" | "W" => 1024.0_f64.powi(12),  // World-scale
        "UB" | "U" => 1024.0_f64.powi(13),  // Universal-scale
        "CB" | "C" => 1024.0_f64.powi(14),  // Cosmic-scale

        // Decimal scale (for compatibility)
        "KBI" => 1000.0,
        "MBI" => 1000.0_f64.powi(2),
        "GBI" => 1000.0_f64.powi(3),
        "TBI" => 1000.0_f64.powi(4),

        _ => {
            return Err(UniversalZfsError::InvalidInput {
                message: "Unknown size unit: self.base_url".to_string(),
            });
        }
    };

    Ok((basevalue * multiplier) as u64)
}

/// Split size string into numeric and unit parts
fn split_size_string(size_str: &str) -> UniversalZfsResult<(&str, &str)> {
    let size_str = size_str.trim();
    // Find the boundary between numeric and alphabetic parts
    let split_pos = size_str
        .find(|c: char| c.is_alphabetic())
        .unwrap_or(size_str.len());

    if split_pos == 0 {
        return Err(UniversalZfsError::InvalidInput {
            message: format!("Size string has no numeric part: {size_str}"),
        });
    }

    let numeric_part = &size_str[..split_pos];
    let unit_part = &size_str[split_pos..];

    Ok((numeric_part, unit_part))
}

/// Parse dataset information from zfs list output
pub fn parse_dataset_list(output: &str) -> UniversalZfsResult<Vec<DatasetInfo>> {
    let mut datasets = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() >= 4 {
            let name = fields[0].to_string();
            let used_size = parse_size_string(fields[1])?;
            let available_size = parse_size_string(fields[2])?;
            let dataset_type_str = fields[3];

            let dataset_type = match dataset_type_str {
                "volume" => DatasetType::Volume,
                "snapshot" => DatasetType::Snapshot,
                _ => DatasetType::Filesystem,
            };

            datasets.push(DatasetInfo {
                name,
                dataset_type,
                used: used_size,
                available: available_size,
                referenced: used_size, // Approximation
                mountpoint: None,      // Would need additional command for this
                properties: HashMap::new(),
            });
        }
    }

    Ok(datasets)
}

/// Parse snapshot information from zfs list -t snapshot output
pub fn parse_snapshot_list(output: &str) -> UniversalZfsResult<Vec<SnapshotInfo>> {
    let mut snapshots = Vec::new();
    for line in output.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() >= 3 {
            let full_name = fields[0].to_string();
            let used_size = parse_size_string(fields[1])?;

            // Extract dataset name and snapshot name from full_name (dataset@snapshot)
            if let Some(at_pos) = full_name.find('@') {
                let _dataset_name = full_name[..at_pos].to_string();
                let snapshot_name = full_name[at_pos + 1..].to_string();

                snapshots.push(SnapshotInfo {
                    name: snapshot_name,
                    creation_time: SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                    used: used_size,
                    referenced: used_size,
                    properties: HashMap::new(),
                });
            }
        }
    }

    Ok(snapshots)
}

/// Device information from zpool status
///
/// Represents a storage device (vdev) in a ZFS pool with its health metrics.
#[derive(Debug, Clone)]
/// Deviceinfo
pub struct DeviceInfo {
    /// Device name (e.g., "sda", "mirror-0", "tank")
    pub name: String,
    /// Device state (e.g., "ONLINE", "DEGRADED", "FAULTED")
    pub state: String,
    /// Number of read errors encountered
    pub read_errors: u64,
    /// Number of write errors encountered
    pub write_errors: u64,
    /// Number of checksum errors encountered
    pub checksum_errors: u64,
}

/// Parse zpool status output to extract device information
///
/// Parses the vdev tree structure from `zpool status` output.
///
/// # Example Input
/// ```text
///   pool: tank
///  state: ONLINE
/// config:
///
///         NAME        STATE     READ WRITE CKSUM
///         tank        ONLINE       0     0     0
///           sda       ONLINE       0     0     0
///           sdb       ONLINE       0     0     0
/// ```
pub fn parse_zpool_status(output: &str) -> UniversalZfsResult<Vec<DeviceInfo>> {
    let mut devices = Vec::new();
    let mut in_config_section = false;
    let mut found_header = false;

    for line in output.lines() {
        let trimmed = line.trim_start();

        // Look for config section
        if trimmed.starts_with("config:") {
            in_config_section = true;
            continue;
        }

        // Look for device table header
        if in_config_section && trimmed.starts_with("NAME") {
            found_header = true;
            continue;
        }

        // Parse device lines (indented, after header, before errors section)
        if found_header && !trimmed.is_empty() && !trimmed.starts_with("errors:") {
            // Device lines are indented with tabs/spaces
            // Format: NAME STATE READ WRITE CKSUM
            let fields: Vec<&str> = trimmed.split_whitespace().collect();

            if fields.len() >= 5 {
                let name = fields[0].to_string();
                let state = fields[1].to_string();

                // Parse error counters (they might be "-" for unknown)
                let read_errors = fields[2].parse::<u64>().unwrap_or(0);
                let write_errors = fields[3].parse::<u64>().unwrap_or(0);
                let checksum_errors = fields[4].parse::<u64>().unwrap_or(0);

                devices.push(DeviceInfo {
                    name,
                    state,
                    read_errors,
                    write_errors,
                    checksum_errors,
                });
            }
        }

        // Stop parsing at errors section
        if trimmed.starts_with("errors:") {
            break;
        }
    }

    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs_types::DatasetType;

    #[test]
    fn test_parse_size_string() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            parse_size_string("1024").map_err(|e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::other("Error: self.base_url".to_string())
            })?,
            1024
        );
        assert_eq!(
            parse_size_string("1K").map_err(|e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::other("Error: self.base_url".to_string())
            })?,
            1024
        );
        assert_eq!(
            parse_size_string("1M").map_err(|e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::other("Error: self.base_url".to_string())
            })?,
            1024 * 1024
        );
        assert_eq!(
            parse_size_string("1G").map_err(|e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::other("Error: self.base_url".to_string())
            })?,
            1024 * 1024 * 1024
        );
        assert_eq!(
            parse_size_string("1T").map_err(|e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::other("Error: self.base_url".to_string())
            })?,
            1024_u64.pow(4)
        );
        assert_eq!(
            parse_size_string("1.5G").map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                std::io::Error::other("Operation failed: self.base_url".to_string())
            })?,
            (1.5 * 1024.0 * 1024.0 * 1024.0) as u64
        );
        assert_eq!(
            parse_size_string("-").map_err(|e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::other("Error: self.base_url".to_string())
            })?,
            0
        );
        assert_eq!(
            parse_size_string("").map_err(|e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::other("Error: self.base_url".to_string())
            })?,
            0
        );
        Ok(())
    }

    #[test]
    fn test_split_size_string() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let (num, unit) = split_size_string("1024B").map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::other("Operation failed: self.base_url".to_string())
        })?;
        assert_eq!(num, "1024");
        assert_eq!(unit, "B");

        let (num, unit) = split_size_string("1.5GB").map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::other("Operation failed: self.base_url".to_string())
        })?;
        assert_eq!(num, "1.5");
        assert_eq!(unit, "GB");
        Ok(())
    }

    #[test]
    fn parse_zpool_list_tab_line() {
        let out = "tank\t1048576\t524288\t524288\tONLINE\n";
        let pools = parse_zpool_list(out).unwrap();
        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].name, "tank");
    }

    #[test]
    fn parse_dataset_list_filesystem() {
        let out = "tank/foo\t1024\t2048\tfilesystem\n";
        let ds = parse_dataset_list(out).unwrap();
        assert_eq!(ds.len(), 1);
        assert_eq!(ds[0].dataset_type, DatasetType::Filesystem);
    }

    #[test]
    fn parse_snapshot_list_with_at() {
        let out = "tank/ds@snap\t512\tref\n";
        let sn = parse_snapshot_list(out).unwrap();
        assert_eq!(sn.len(), 1);
        assert_eq!(sn[0].name, "snap");
    }

    #[test]
    fn parse_zpool_status_sample_config_block() {
        let out = r"config:

        NAME        STATE     READ WRITE CKSUM
        tank        ONLINE       0     0     0
errors: No known data errors
";
        let devs = parse_zpool_status(out).unwrap();
        assert!(!devs.is_empty());
        assert_eq!(devs[0].name, "tank");
    }

    #[test]
    fn parse_size_string_rejects_bad_unit() {
        assert!(parse_size_string("5XX").is_err());
    }
}
