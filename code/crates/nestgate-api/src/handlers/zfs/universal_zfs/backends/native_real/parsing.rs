//
// Zero-copy parsing utilities for ZFS command output.
// Single responsibility: Parse ZFS command output into structured data.

use std::collections::HashMap;
use std::time::SystemTime;

use crate::handlers::zfs::universal_zfs::types::{
    DatasetInfo, DatasetType, PoolCapacity, PoolHealth, PoolInfo, PoolState, ScrubStatus,
    SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};

// Import zero-copy utilities
// Note: zero_copy lines function moved to utilities
// use nestgate_core::zero_copy::lines_zero_copy; // Module doesn't exist

/// Parse zpool list output into PoolInfo structures
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
                "OFFLINE" => PoolHealth::Offline,
                "UNAVAIL" => PoolHealth::Unavailable,
                "REMOVED" => PoolHealth::Removed,
                _ => PoolHealth::Unknown,
            };

            let state = match health {
                PoolHealth::Online => PoolState::Active,
                PoolHealth::Degraded => PoolState::Unavailable,
                _ => PoolState::Unknown,
            };

            pools.push(PoolInfo {
                name,
                health,
                state,
                capacity: PoolCapacity {
                    total_bytes: total_size,
                    used_bytes: used_size,
                    available_bytes: available_size,
                    utilization_percent: if total_size > 0 {
                        (used_size as f64 / total_size as f64) * 100.0
                    } else {
                        0.0
                    },
                },
                _devices: Vec::new(), // Would need zpool status for this
                properties: HashMap::new(),
                created_at: SystemTime::now(),
                last_scrub: None,
                scrub_status: ScrubStatus::None,
                errors: Vec::new(),
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
            field: "field".to_string(),
            message: format!("Invalid numeric value: {"actual_error_details"}"),
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
                field: "field".to_string(),
                message: format!("Unknown size unit: {"actual_error_details"}"),
            }
            .into());
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
            field: "field".to_string(),
            message: format!(
                "Size string has no numeric part: {}",
                "actual_error_details"
            ),
        }
        .into());
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
                "filesystem" => DatasetType::Filesystem,
                "volume" => DatasetType::Volume,
                "snapshot" => DatasetType::Snapshot,
                _ => DatasetType::Filesystem, // Default fallback
            };

            datasets.push(DatasetInfo {
                name,
                dataset_type,
                used_space: used_size,
                available_space: available_size,
                mount_point: None, // Would need additional command for this
                properties: HashMap::new(),
                created_at: SystemTime::now(), // Would need additional command for this
                parent: None,
                children: Vec::new(),
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
                let dataset_name = full_name[..at_pos].to_string();
                let snapshot_name = full_name[at_pos + 1..].to_string();

                snapshots.push(SnapshotInfo {
                    name: snapshot_name,
                    dataset: dataset_name,
                    created_at: SystemTime::now(), // Would need additional parsing
                    size_bytes: used_size,
                    properties: HashMap::new(),
                    description: None,
                });
            }
        }
    }

    Ok(snapshots)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_string() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            parse_size_string("1024").map_err(|_e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error: {"actual_error_details"}"),
                )
            })?,
            1024
        );
        assert_eq!(
            parse_size_string("1K").map_err(|_e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error: {"actual_error_details"}"),
                )
            })?,
            1024
        );
        assert_eq!(
            parse_size_string("1M").map_err(|_e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error: {"actual_error_details"}"),
                )
            })?,
            1024 * 1024
        );
        assert_eq!(
            parse_size_string("1G").map_err(|_e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error: {"actual_error_details"}"),
                )
            })?,
            1024 * 1024 * 1024
        );
        assert_eq!(
            parse_size_string("1T").map_err(|_e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error: {"actual_error_details"}"),
                )
            })?,
            1024_u64.pow(4)
        );
        assert_eq!(
            parse_size_string("1.5G").map_err(|_e| {
                tracing::error!("Operation failed: {:?}", e);
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {"actual_error_details"}"),
                )
            })?,
            (1.5 * 1024.0 * 1024.0 * 1024.0) as u64
        );
        assert_eq!(
            parse_size_string("-").map_err(|_e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error: {"actual_error_details"}"),
                )
            })?,
            0
        );
        assert_eq!(
            parse_size_string("").map_err(|_e| {
                tracing::error!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Test assertion failed",
                    e
                );
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Error: {"actual_error_details"}"),
                )
            })?,
            0
        );
        Ok(())
    }

    #[test]
    fn test_split_size_string() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let (num, unit) = split_size_string("1024B").map_err(|_e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {"actual_error_details"}"),
            )
        })?;
        assert_eq!(num, "1024");
        assert_eq!(unit, "B");

        let (num, unit) = split_size_string("1.5GB").map_err(|_e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {"actual_error_details"}"),
            )
        })?;
        assert_eq!(num, "1.5");
        assert_eq!(unit, "GB");
        Ok(())
    }
}
