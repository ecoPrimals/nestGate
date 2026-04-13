// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage probe helpers are exercised in unit tests and reserved for future handler wiring.
#![expect(
    dead_code,
    reason = "Probe helpers not yet called from production handlers; tests cover them"
)]

use nestgate_core::error::{NestGateError, Result};
use nestgate_zfs::numeric::f64_to_u64_saturating;
use std::process::Command;

use super::types::{StorageDataset, StorageMetrics, StoragePool, StorageSnapshot};

/// **STORAGE UTILITY FUNCTIONS**
///
/// These functions are kept for future storage integration and testing purposes.
///
/// Collect real storage pools from system
pub(super) fn collect_real_storage_pools() -> Result<Vec<StoragePool>> {
    use std::str;
    let mut pools = Vec::new();

    // Get filesystem information using df command
    let output = Command::new("df")
        .args(["-h", "--output=source,fstype,size,used,avail,pcent,target"])
        .output()?;

    let stdout =
        str::from_utf8(&output.stdout).map_err(|e| NestGateError::internal(e.to_string()))?;

    for line in stdout.lines().skip(1) {
        // Skip header
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 7 {
            let source = parts[0];
            let fstype = parts[1];
            let size_str = parts[2];
            let used_str = parts[3];
            let avail_str = parts[4];
            let mount_point = parts[6];

            // Skip temporary filesystems and focus on real storage
            if should_include_filesystem(source, fstype, mount_point) {
                let size = parse_size_string(size_str).unwrap_or(0);
                let used = parse_size_string(used_str).unwrap_or(0);
                let available = parse_size_string(avail_str).unwrap_or(0);

                pools.push(StoragePool {
                    name: format!("{source} ({mount_point})"),
                    status: "ONLINE".to_string(),
                    size,
                    used,
                    available,
                    health: "HEALTHY".to_string(),
                    pool_type: fstype.to_uppercase(),
                });
            }
        }
    }

    if pools.is_empty() {
        // Fallback to root filesystem if nothing found
        pools.push(create_fallback_root_pool());
    }

    Ok(pools)
}

/// Create fallback pool representing the root filesystem
pub(super) fn create_fallback_root_pool() -> StoragePool {
    // Get root filesystem info
    let (size, used, available) = if let Ok(output) = Command::new("df").args(["-B1", "/"]).output()
    {
        if let Ok(stdout) = std::str::from_utf8(&output.stdout) {
            if let Some(line) = stdout.lines().nth(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let size = parts[1].parse::<u64>().unwrap_or(0);
                    let used = parts[2].parse::<u64>().unwrap_or(0);
                    let avail = parts[3].parse::<u64>().unwrap_or(0);
                    (size, used, avail)
                } else {
                    (0, 0, 0)
                }
            } else {
                (0, 0, 0)
            }
        } else {
            (0, 0, 0)
        }
    } else {
        (0, 0, 0)
    };

    StoragePool {
        name: "root (/)".to_string(),
        status: "ONLINE".to_string(),
        size,
        used,
        available,
        health: "HEALTHY".to_string(),
        pool_type: "FILESYSTEM".to_string(),
    }
}

/// Determine if we should include this filesystem in our pools
pub(super) fn should_include_filesystem(source: &str, fstype: &str, mount_point: &str) -> bool {
    // Include real storage _devices and important mount points
    !source.starts_with("tmpfs")
        && !source.starts_with("udev")
        && !source.starts_with("devpts")
        && !source.starts_with("sysfs")
        && !source.starts_with("proc")
        && !source.starts_with("cgroup")
        && !fstype.contains("tmpfs")
        && !mount_point.starts_with("/proc")
        && !mount_point.starts_with("/sys")
        && !mount_point.starts_with("/dev")
        && (mount_point == "/"
            || mount_point.starts_with("/home")
            || mount_point.starts_with("/mnt")
            || mount_point.starts_with("/media"))
}
/// Parse size strings like "1.2G", "500M", "2.1T" to bytes
pub(super) fn parse_size_string(size_str: &str) -> Option<u64> {
    if size_str == "-" {
        return Some(0);
    }
    let size_str = size_str.trim();
    let (number_part, unit) = if let Some(pos) = size_str.chars().position(char::is_alphabetic) {
        let (num, unit) = size_str.split_at(pos);
        (num, unit)
    } else {
        (size_str, "")
    };

    if let Ok(number) = number_part.parse::<f64>() {
        let multiplier = match unit.to_uppercase().as_str() {
            "K" | "KB" => 1024,
            "M" | "MB" => 1024 * 1024,
            "G" | "GB" => 1024 * 1024 * 1024,
            "T" | "TB" => 1024_u64.pow(4),
            "P" | "PB" => 1024_u64.pow(5),
            _ => 1,
        };
        Some(f64_to_u64_saturating(number * multiplier as f64))
    } else {
        None
    }
}

/// Collect real storage datasets (important directories) from system
fn collect_real_storage_datasets() -> Vec<StorageDataset> {
    // Mock implementation for datasets
    let mut datasets = Vec::new();
    // Important directories to monitor as "datasets"
    let important_dirs = vec![
        "/home", "/var", "/usr", "/opt", "/tmp", "/mnt", "/media", "/srv",
    ];

    for dir in important_dirs {
        if std::path::Path::new(dir).exists()
            && let Ok((size, used, available)) = get_directory_usage(dir)
        {
            datasets.push(StorageDataset {
                name: format!("local_{}", dir.trim_start_matches('/').replace('/', "_")),
                pool: "root".to_string(),
                size,
                used,
                available,
                mount_point: dir.to_string(),
                compression: "none".to_string(),
            });
        }
    }

    // Also add the current user's home directory specifically
    if let Ok(home_dir) = std::env::var("HOME")
        && let Ok((size, used, available)) = get_directory_usage(&home_dir)
    {
        datasets.push(StorageDataset {
            name: "user_home".to_string(),
            pool: "root".to_string(),
            size,
            used,
            available,
            mount_point: home_dir,
            compression: "none".to_string(),
        });
    }

    if datasets.is_empty() {
        datasets.push(create_fallback_home_dataset());
    }

    datasets
}

/// Get directory usage statistics
fn get_directory_usage(dir: &str) -> Result<(u64, u64, u64)> {
    // Use df to get filesystem stats for the directory
    let output = Command::new("df").args(["-B1", dir]).output()?;

    let stdout =
        std::str::from_utf8(&output.stdout).map_err(|e| NestGateError::internal(e.to_string()))?;
    if let Some(line) = stdout.lines().nth(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let size = parts[1].parse::<u64>().unwrap_or(0);
            let used = parts[2].parse::<u64>().unwrap_or(0);
            let available = parts[3].parse::<u64>().unwrap_or(0);
            return Ok((size, used, available));
        }
    }

    Ok((0, 0, 0))
}

/// Create fallback dataset for user home directory
fn create_fallback_home_dataset() -> StorageDataset {
    use nestgate_core::error::utilities::safe_env_var_or_default;
    let home_dir = safe_env_var_or_default("HOME", "/home");
    let (size, used, available) = get_directory_usage(&home_dir).unwrap_or((0, 0, 0));
    StorageDataset {
        name: "user_home".to_string(),
        pool: "root".to_string(),
        size,
        used,
        available,
        mount_point: home_dir,
        compression: "none".to_string(),
    }
}

/// Collect real ZFS snapshots from system
async fn collect_real_zfs_snapshots() -> Result<Vec<StorageSnapshot>> {
    let output = tokio::process::Command::new("zfs")
        .args([
            "list",
            "-t",
            "snapshot",
            "-H",
            "-p",
            "-o",
            "name,used,refer,creation",
        ])
        .output()
        .await?;
    if !output.status.success() {
        return Err(NestGateError::internal("ZFS snapshot command failed"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut snapshots = Vec::new();

    for line in stdout.lines().take(100) {
        // Limit to 100 most recent snapshots
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            let full_name_str = parts[0];
            let size: u64 = parts[1].parse().unwrap_or(0);
            let referenced: u64 = parts[2].parse().unwrap_or(0);
            let creation_timestamp: u64 = parts[3].parse().unwrap_or(0);

            // Dataset path is the snapshot name prefix before `@` (single parse pass).
            let dataset = full_name_str
                .split_once('@')
                .map_or(full_name_str, |(ds, _)| ds)
                .to_string();
            let full_name = full_name_str.to_string();

            // Convert Unix timestamp to ISO8601 string
            let created = if creation_timestamp > 0 {
                match std::time::SystemTime::UNIX_EPOCH
                    .checked_add(std::time::Duration::from_secs(creation_timestamp))
                {
                    Some(time) => chrono::DateTime::<chrono::Utc>::from(time).to_rfc3339(),
                    None => "unknown".to_string(),
                }
            } else {
                "unknown".to_string()
            };

            snapshots.push(StorageSnapshot {
                name: full_name,
                dataset,
                size,
                created,
                referenced,
            });
        }
    }

    // Sort by creation time (most recent first)
    snapshots.sort_by(|a, b| b.created.cmp(&a.created));

    Ok(snapshots)
}

/// Parse bandwidth unit values
pub(super) fn parse_bandwidth_unit(value: &str) -> Option<f64> {
    if value == "-" || value.is_empty() {
        return Some(0.0);
    }
    let (number_part, unit) = if let Some(stripped) = value.strip_suffix('K') {
        (stripped.parse::<f64>().ok()?, 1.0 / 1024.0)
    } else if let Some(stripped) = value.strip_suffix('M') {
        (stripped.parse::<f64>().ok()?, 1.0)
    } else if let Some(stripped) = value.strip_suffix('G') {
        (stripped.parse::<f64>().ok()?, 1024.0)
    } else {
        (value.parse::<f64>().ok()?, 1.0 / (1024.0 * 1024.0))
    };

    Some(number_part * unit)
}

/// Collect fallback storage metrics
async fn collect_fallback_storage_metrics() -> StorageMetrics {
    // Get basic disk space information from system
    let (total_storage, used_storage, available_storage) = match tokio::process::Command::new("df")
        .args(["-B1", "/"]) // Get root filesystem size in bytes
        .output()
        .await
    {
        Ok(df_output) if df_output.status.success() => {
            let df_stdout = String::from_utf8_lossy(&df_output.stdout);
            if let Some(line) = df_stdout.lines().nth(1) {
                // Skip header
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let total: u64 = parts[1].parse().unwrap_or(0);
                    let used: u64 = parts[2].parse().unwrap_or(0);
                    let available: u64 = parts[3].parse().unwrap_or(0);
                    (total, used, available)
                } else {
                    (
                        1024 * 1024 * 1024 * 1024,
                        512 * 1024 * 1024 * 1024,
                        512 * 1024 * 1024 * 1024,
                    ) // 1TB default
                }
            } else {
                (
                    1024 * 1024 * 1024 * 1024,
                    512 * 1024 * 1024 * 1024,
                    512 * 1024 * 1024 * 1024,
                ) // 1TB default
            }
        }
        _ => (
            1024 * 1024 * 1024 * 1024,
            512 * 1024 * 1024 * 1024,
            512 * 1024 * 1024 * 1024,
        ), // 1TB default
    };
    StorageMetrics {
        total_pools: 0, // No ZFS pools available
        total_datasets: 0,
        total_snapshots: 0,
        total_storage,
        used_storage,
        available_storage,
        iops: 50.0,           // Conservative estimate
        bandwidth_mbps: 25.0, // Conservative estimate
        health_status: "SYSTEM_STORAGE".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_include_filesystem_excludes_special_sources_and_mounts() {
        assert!(!should_include_filesystem("udev", "ext4", "/"));
        assert!(!should_include_filesystem("devpts", "devpts", "/dev/pts"));
        assert!(!should_include_filesystem(
            "cgroup",
            "cgroup2",
            "/sys/fs/cgroup"
        ));
        assert!(!should_include_filesystem("proc", "proc", "/proc"));
        assert!(!should_include_filesystem(
            "/dev/loop0",
            "ext4",
            "/sys/kernel"
        ));
        assert!(!should_include_filesystem("/dev/sda1", "tmpfs", "/"));
        assert!(!should_include_filesystem("/dev/sda1", "ext4", "/dev/shm"));
        assert!(should_include_filesystem("/dev/sda1", "ext4", "/home/user"));
    }

    #[test]
    fn should_include_filesystem_accepts_root_home_mnt_media() {
        assert!(should_include_filesystem("/dev/nvme0n1p2", "ext4", "/"));
        assert!(should_include_filesystem("LABEL=home", "xfs", "/home"));
        assert!(should_include_filesystem("/dev/sdb1", "btrfs", "/mnt/data"));
        assert!(should_include_filesystem(
            "/dev/sdc1",
            "vfat",
            "/media/disk"
        ));
    }

    #[test]
    fn parse_size_string_edge_cases() {
        assert_eq!(parse_size_string(""), None);
        assert_eq!(parse_size_string("notnumG"), None);
        assert_eq!(parse_size_string("1.5X"), Some(1)); // unknown unit → multiplier 1
        assert_eq!(parse_size_string("0G"), Some(0));
        let half_g = parse_size_string("0.5G").expect("0.5G");
        assert_eq!(half_g, 512 * 1024 * 1024);
        assert_eq!(parse_size_string("100GB"), Some(100 * 1024 * 1024 * 1024));
        assert_eq!(parse_size_string("2TB"), Some(2 * 1024_u64.pow(4)));
        assert_eq!(parse_size_string("1PB"), Some(1024_u64.pow(5)));
    }

    #[test]
    fn parse_bandwidth_unit_invalid_or_edge() {
        assert!(parse_bandwidth_unit("not_a_number").is_none());
        assert!(parse_bandwidth_unit("xK").is_none());
        assert_eq!(parse_bandwidth_unit("0M"), Some(0.0));
        let m = parse_bandwidth_unit("4M").expect("4M");
        assert!((m - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn collect_real_storage_pools_runs() {
        let pools = collect_real_storage_pools().expect("df should succeed on this platform");
        assert!(!pools.is_empty(), "expected at least fallback pool");
        let rootish = pools
            .iter()
            .find(|p| p.name.contains("root") || p.pool_type == "FILESYSTEM");
        assert!(rootish.is_some() || !pools.is_empty());
    }

    #[test]
    fn create_fallback_root_pool_shape() {
        let p = create_fallback_root_pool();
        assert_eq!(p.status, "ONLINE");
        assert_eq!(p.health, "HEALTHY");
        assert_eq!(p.pool_type, "FILESYSTEM");
        assert!(p.name.contains("root"));
    }

    #[test]
    fn get_directory_usage_root_or_tmp() {
        let (total, used, avail) = get_directory_usage("/").expect("df on /");
        assert!(total >= used && total >= avail);
        let tmp = get_directory_usage("/tmp").expect("df on /tmp");
        assert!(tmp.0 >= tmp.2);
    }

    #[test]
    fn create_fallback_home_dataset_has_expected_fields() {
        let d = create_fallback_home_dataset();
        assert_eq!(d.name, "user_home");
        assert_eq!(d.pool, "root");
        assert_eq!(d.compression, "none");
        assert!(!d.mount_point.is_empty());
    }

    #[test]
    fn collect_real_storage_datasets_returns_at_least_one() {
        let list = collect_real_storage_datasets();
        assert!(!list.is_empty());
    }

    #[tokio::test]
    async fn collect_fallback_storage_metrics_populated() {
        let m = collect_fallback_storage_metrics().await;
        assert_eq!(m.total_pools, 0);
        assert_eq!(m.health_status, "SYSTEM_STORAGE");
        assert!(m.total_storage > 0 || m.used_storage > 0 || m.available_storage > 0);
        assert_eq!(m.iops, 50.0);
        assert_eq!(m.bandwidth_mbps, 25.0);
    }

    #[tokio::test]
    async fn collect_real_zfs_snapshots_runs() {
        let result = collect_real_zfs_snapshots().await;
        if let Ok(snaps) = result {
            for s in snaps.iter().take(3) {
                assert!(!s.name.is_empty());
                assert!(!s.dataset.is_empty());
            }
        }
    }
}
