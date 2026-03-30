// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ZFS Output Parsers**
//!
//! Production-ready parsers for ZFS command output.
//! Handles real-world ZFS output formats with robust error handling.

use crate::numeric::f64_to_u64_saturating;
use crate::types::{
    DatasetInfo, PoolCapacity, PoolHealth, PoolInfo, PoolState, ZfsError, ZfsResult,
};
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Parse ZFS pool information from `zpool list` or `zpool get` output
///
/// Expected format:
/// ```text
/// size    1000000000
/// allocated       500000000
/// health  ONLINE
/// ```
pub fn pool_info_from_zfs_output(pool_name: &str, output: &str) -> ZfsResult<PoolInfo> {
    let mut properties = HashMap::new();

    // Parse key-value pairs from output
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            properties.insert(key.to_string(), value.to_string());
        }
    }

    // Extract required fields with defaults
    let size = properties
        .get("size")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let allocated = properties
        .get("allocated")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let available = size.saturating_sub(allocated);

    let health_str = properties
        .get("health")
        .cloned()
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let health = match health_str.to_uppercase().as_str() {
        "ONLINE" | "HEALTHY" => PoolHealth::Healthy,
        "DEGRADED" | "WARNING" => PoolHealth::Warning,
        "FAULTED" | "CRITICAL" => PoolHealth::Critical,
        _ => PoolHealth::Unknown,
    };

    let state = match health_str.to_uppercase().as_str() {
        "ONLINE" => PoolState::Online,
        "DEGRADED" => PoolState::Degraded,
        "FAULTED" => PoolState::Faulted,
        "REMOVED" => PoolState::Removed,
        "UNAVAIL" => PoolState::Unavailable,
        "OFFLINE" | _ => PoolState::Offline,
    };

    let utilization_percent = if size > 0 {
        #[expect(
            clippy::cast_precision_loss,
            reason = "utilization ratio; operands are byte counts, approximate percentage is acceptable"
        )]
        {
            (allocated as f64 / size as f64) * 100.0
        }
    } else {
        0.0
    };

    Ok(PoolInfo {
        name: pool_name.to_string(),
        size,
        used: allocated,
        available,
        health,
        state,
        capacity: PoolCapacity {
            total: size,
            total_bytes: size,
            used: allocated,
            used_bytes: allocated,
            available,
            available_bytes: available,
            utilization_percent,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
        },
        properties,
        created_at: SystemTime::now(),
    })
}

/// Parse ZFS dataset information from `zfs get` output
///
/// Expected format:
/// ```text
/// name    pool/dataset
/// used    1024
/// available       2048
/// compression     lz4
/// mountpoint      /mnt/data
/// ```
pub fn dataset_info_from_zfs_output(output: &str) -> ZfsResult<DatasetInfo> {
    let mut properties = HashMap::new();

    // Parse key-value pairs from output
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let key = parts[0].trim();
            let value = parts[1..].join(" ");
            properties.insert(key.to_string(), value);
        }
    }

    // Extract required fields
    let full_name = properties
        .get("name")
        .ok_or_else(|| ZfsError::DatasetError {
            message: "Missing dataset name in output".to_string(),
        })?
        .clone();

    // Parse pool and dataset name
    let (pool, name) = if let Some(pos) = full_name.find('/') {
        (
            full_name[..pos].to_string(),
            full_name[pos + 1..].to_string(),
        )
    } else {
        (full_name.clone(), full_name.clone())
    };

    let used = properties
        .get("used")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let available = properties
        .get("available")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let compression = properties
        .get("compression")
        .cloned()
        .unwrap_or_else(|| "lz4".to_string());

    let mountpoint = properties
        .get("mountpoint")
        .filter(|s| *s != "none" && *s != "-")
        .map(PathBuf::from);

    Ok(DatasetInfo {
        name,
        full_name,
        pool,
        size: used + available,
        used,
        available,
        mountpoint: mountpoint.clone(),
        mount_point: mountpoint,
        dataset_type: "filesystem".to_string(),
        compression,
        checksum: "sha256".to_string(),
        referenced: used,
        compression_ratio: 1.0,
        tier: StorageTier::Warm,
        properties,
        created_at: SystemTime::now(),
    })
}

/// Parse pool health status from ZFS output
#[must_use]
pub fn parse_health_status(status: &str) -> String {
    match status.trim().to_uppercase().as_str() {
        "ONLINE" => "ONLINE".to_string(),
        "DEGRADED" => "DEGRADED".to_string(),
        "FAULTED" => "FAULTED".to_string(),
        "OFFLINE" => "OFFLINE".to_string(),
        "UNAVAIL" => "UNAVAIL".to_string(),
        "REMOVED" => "REMOVED".to_string(),
        _ => status.trim().to_string(),
    }
}

/// Parse size values that may include units (K, M, G, T, P)
pub fn parse_size_with_units(size_str: &str) -> ZfsResult<u64> {
    let size_str = size_str.trim();

    if let Ok(size) = size_str.parse::<u64>() {
        return Ok(size);
    }

    // Handle units (K, M, G, T, P)
    let (num_part, unit) = if size_str.ends_with(char::is_alphabetic) {
        let split_pos = size_str
            .chars()
            .position(char::is_alphabetic)
            .unwrap_or(size_str.len());
        (&size_str[..split_pos], &size_str[split_pos..])
    } else {
        (size_str, "")
    };

    let base_value = num_part
        .parse::<f64>()
        .map_err(|e| ZfsError::DatasetError {
            message: format!("Invalid size value '{size_str}': {e}"),
        })?;

    let multiplier: u64 = match unit.to_uppercase().as_str() {
        "K" | "KB" => 1024,
        "M" | "MB" => 1024 * 1024,
        "G" | "GB" => 1024 * 1024 * 1024,
        "T" | "TB" => 1024 * 1024 * 1024 * 1024,
        "P" | "PB" => 1024 * 1024 * 1024 * 1024 * 1024,
        "" => 1,
        _ => {
            return Err(ZfsError::DatasetError {
                message: format!("Unknown size unit: {unit}"),
            });
        }
    };

    Ok(f64_to_u64_saturating(base_value * multiplier as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_info_parsing_online() {
        let output = "size\t1000000000\nallocated\t500000000\nhealth\tONLINE\n";
        let result = pool_info_from_zfs_output("testpool", output);

        assert!(result.is_ok());
        let pool_info = result.unwrap();
        assert_eq!(pool_info.name, "testpool");
        assert_eq!(pool_info.size, 1000000000);
        assert_eq!(pool_info.used, 500000000);
        assert!(matches!(pool_info.health, PoolHealth::Healthy));
        assert!(matches!(pool_info.state, PoolState::Online));
    }

    #[test]
    fn test_pool_info_parsing_degraded() {
        let output = "size\t2000000000\nallocated\t1000000000\nhealth\tDEGRADED\n";
        let result = pool_info_from_zfs_output("degraded_pool", output);

        assert!(result.is_ok());
        let pool_info = result.unwrap();
        assert_eq!(pool_info.name, "degraded_pool");
        assert!(matches!(pool_info.health, PoolHealth::Warning));
        assert!(matches!(pool_info.state, PoolState::Degraded));
    }

    #[test]
    fn test_dataset_info_parsing() {
        let output = "name\tpool/dataset\nused\t1024\navailable\t2048\ncompression\tlz4\n";
        let result = dataset_info_from_zfs_output(output);

        assert!(result.is_ok());
        let dataset_info = result.unwrap();
        assert_eq!(dataset_info.full_name, "pool/dataset");
        assert_eq!(dataset_info.pool, "pool");
        assert_eq!(dataset_info.name, "dataset");
        assert_eq!(dataset_info.used, 1024);
        assert_eq!(dataset_info.available, 2048);
        assert_eq!(dataset_info.compression, "lz4");
    }

    #[test]
    fn test_dataset_info_with_mountpoint() {
        let output = "name\tpool/data\nused\t1024\navailable\t2048\nmountpoint\t/mnt/data\n";
        let result = dataset_info_from_zfs_output(output);

        assert!(result.is_ok());
        let dataset_info = result.unwrap();
        assert_eq!(dataset_info.mountpoint, Some(PathBuf::from("/mnt/data")));
    }

    #[test]
    fn test_dataset_info_no_mountpoint() {
        let output = "name\tpool/data\nused\t1024\navailable\t2048\nmountpoint\tnone\n";
        let result = dataset_info_from_zfs_output(output);

        assert!(result.is_ok());
        let dataset_info = result.unwrap();
        assert_eq!(dataset_info.mountpoint, None);
    }

    #[test]
    fn test_parse_health_status() {
        assert_eq!(parse_health_status("online"), "ONLINE");
        assert_eq!(parse_health_status("DEGRADED"), "DEGRADED");
        assert_eq!(parse_health_status("  FAULTED  "), "FAULTED");
    }

    #[test]
    fn test_parse_size_with_units() {
        assert_eq!(parse_size_with_units("1024").unwrap(), 1024);
        assert_eq!(parse_size_with_units("1K").unwrap(), 1024);
        assert_eq!(parse_size_with_units("1M").unwrap(), 1024 * 1024);
        assert_eq!(parse_size_with_units("2G").unwrap(), 2 * 1024 * 1024 * 1024);
    }

    #[test]
    fn pool_info_parsing_additional_health_and_state() {
        let out = "size\t1000\nallocated\t100\nhealth\tFAULTED\n";
        let p = pool_info_from_zfs_output("fp", out).unwrap();
        assert!(matches!(p.health, PoolHealth::Critical));
        assert!(matches!(p.state, PoolState::Faulted));

        let out2 = "size\t1000\nallocated\t100\nhealth\tUNAVAIL\n";
        let p2 = pool_info_from_zfs_output("u", out2).unwrap();
        assert!(matches!(p2.state, PoolState::Unavailable));

        let out3 = "size\t1000\nallocated\t100\nhealth\tREMOVED\n";
        let p3 = pool_info_from_zfs_output("r", out3).unwrap();
        assert!(matches!(p3.state, PoolState::Removed));

        let out4 = "size\t1000\nallocated\t100\nhealth\tUNKNOWN\n";
        let p4 = pool_info_from_zfs_output("x", out4).unwrap();
        assert!(matches!(p4.health, PoolHealth::Unknown));
        assert!(matches!(p4.state, PoolState::Offline));
    }

    #[test]
    fn parse_health_status_all_branches() {
        assert_eq!(parse_health_status("OFFLINE"), "OFFLINE");
        assert_eq!(parse_health_status("UNAVAIL"), "UNAVAIL");
        assert_eq!(parse_health_status("REMOVED"), "REMOVED");
        assert_eq!(parse_health_status("custom"), "custom");
    }

    #[test]
    fn parse_size_with_units_tb_pb_and_errors() {
        assert!(parse_size_with_units("1T").unwrap() > 0);
        assert!(parse_size_with_units("1TB").unwrap() > 0);
        assert!(parse_size_with_units("1P").unwrap() > 0);
        assert!(parse_size_with_units("1PB").unwrap() > 0);
        assert!(parse_size_with_units("2.5M").unwrap() > 1024 * 1024);
        assert!(parse_size_with_units("bad_unit").is_err());
        assert!(parse_size_with_units("10Z").is_err());
    }

    #[test]
    fn dataset_info_single_segment_name() {
        let output = "name\tonlyname\nused\t10\navailable\t90\n";
        let d = dataset_info_from_zfs_output(output).unwrap();
        assert_eq!(d.pool, "onlyname");
        assert_eq!(d.name, "onlyname");
    }
}
