//! Helper functions for ZFS dataset management
//!
//! This module contains utility functions that support dataset operations
//! but don't require access to the ZfsDatasetManager struct.

use nestgate_core::canonical_types::StorageTier as CoreStorageTier;
use std::collections::HashMap;
use tokio::process::Command as TokioCommand;

/// Create a fallback DatasetInfo for development/testing environments
///
/// # Arguments
///
/// * `name` - The dataset name
/// * `tier` - The storage tier
///
/// # Returns
///
/// A DatasetInfo with default fallback values
pub fn create_fallback_dataset_info(
    name: &str,
    tier: CoreStorageTier,
) -> crate::dataset::DatasetInfo {
    crate::dataset::DatasetInfo {
        name: name.to_string(),
        used_space: 0,
        available_space: 1024 * 1024 * 1024,
        file_count: None,
        compression_ratio: Some(1.0),
        mount_point: format!("/{name}"),
        tier,
        properties: HashMap::new(),
    }
}

/// Apply tier-specific ZFS properties to a command
///
/// This function configures compression, recordsize, and other properties
/// based on the storage tier type.
///
/// # Arguments
///
/// * `cmd` - The tokio Command to apply properties to
/// * `tier` - The storage tier type
pub fn apply_tier_properties(cmd: &mut TokioCommand, tier: &CoreStorageTier) {
    match tier {
        CoreStorageTier::Hot => {
            // Hot tier: optimized for performance
            cmd.args(["-o", "compression=off"]);
            cmd.args(["-o", "recordsize=128K"]);
        }
        CoreStorageTier::Warm => {
            // Warm tier: balanced performance and compression
            cmd.args(["-o", "compression=lz4"]);
            cmd.args(["-o", "recordsize=128K"]);
        }
        CoreStorageTier::Cold => {
            // Cold tier: optimized for space efficiency
            cmd.args(["-o", "compression=zstd"]);
            cmd.args(["-o", "recordsize=1M"]);
        }
        CoreStorageTier::Cache => {
            // Cache tier: ultra-fast, no compression
            cmd.args(["-o", "compression=off"]);
            cmd.args(["-o", "recordsize=64K"]);
        }
        CoreStorageTier::Archive => {
            // Archive tier: maximum compression
            cmd.args(["-o", "compression=gzip-9"]);
            cmd.args(["-o", "recordsize=1M"]);
        }
    }
}

/// Parse ZFS property output into key-value pairs
///
/// # Arguments
///
/// * `stdout` - The output from a `zfs get` command
///
/// # Returns
///
/// A HashMap of property names to values
pub fn parse_properties(stdout: &str) -> std::collections::HashMap<String, String> {
    let mut properties = std::collections::HashMap::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 3 {
            properties.insert(parts[1].to_string(), parts[2].to_string());
        }
    }

    properties
}

/// Parse ZFS snapshot list output
///
/// # Arguments
///
/// * `stdout` - The output from a `zfs list -t snapshot` command
///
/// # Returns
///
/// A vector of snapshot information
#[allow(dead_code)] // Reserved for future use
pub fn parse_snapshots(stdout: &str) -> Vec<crate::snapshot::SnapshotInfo> {
    let mut snapshots = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            let full_name = parts[0].to_string();
            let name = full_name
                .split('@')
                .next_back()
                .unwrap_or(&full_name)
                .to_string();
            let dataset = full_name
                .split('@')
                .next()
                .unwrap_or(&full_name)
                .to_string();

            snapshots.push(crate::snapshot::SnapshotInfo {
                name,
                full_name,
                dataset,
                created_at: std::time::SystemTime::now(), // Would need proper parsing
                size: parts[1].parse().unwrap_or(0),
                referenced_size: parts[2].parse().unwrap_or(0),
                written_size: parts[1].parse().unwrap_or(0),
                compression_ratio: 1.0,
                properties: std::collections::HashMap::new(),
                policy: None,
                tier: CoreStorageTier::Warm,
                protected: false,
                tags: Vec::new(),
            });
        }
    }

    snapshots
}

/// Parse dataset list output into DatasetInfo structures
///
/// # Arguments
///
/// * `stdout` - The output from a `zfs list` command
///
/// # Returns
///
/// A vector of DatasetInfo structures
pub fn parse_dataset_list(stdout: &str) -> Vec<crate::dataset::DatasetInfo> {
    let mut datasets = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            let name = parts[0].to_string();
            let used_space = parts[1].parse().unwrap_or(0);
            let available_space = parts[2].parse().unwrap_or(0);
            let mount_point = parts[3].to_string();

            datasets.push(crate::dataset::DatasetInfo {
                name,
                used_space,
                available_space,
                file_count: None,
                compression_ratio: None,
                mount_point,
                tier: CoreStorageTier::Warm, // Default tier, would need tier detection logic
                properties: std::collections::HashMap::new(),
            });
        }
    }

    datasets
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::process::Command;

    #[test]
    fn test_apply_tier_properties_hot() {
        let mut cmd = Command::new("zfs");
        cmd.arg("create");
        apply_tier_properties(&mut cmd, &CoreStorageTier::Hot);

        // Command construction succeeded without panicking - test passes
    }

    #[test]
    fn test_apply_tier_properties_archive() {
        let mut cmd = Command::new("zfs");
        cmd.arg("create");
        apply_tier_properties(&mut cmd, &CoreStorageTier::Archive);

        // Command construction succeeded without panicking - test passes
    }

    #[test]
    fn test_parse_properties() {
        let stdout = "pool/dataset\tcompression\tlz4\n\
                      pool/dataset\trecordsize\t128K\n\
                      pool/dataset\tmounted\tyes\n";

        let props = parse_properties(stdout);

        assert_eq!(props.get("compression"), Some(&"lz4".to_string()));
        assert_eq!(props.get("recordsize"), Some(&"128K".to_string()));
        assert_eq!(props.get("mounted"), Some(&"yes".to_string()));
    }

    #[test]
    fn test_parse_properties_empty() {
        let stdout = "";
        let props = parse_properties(stdout);
        assert!(props.is_empty());
    }

    #[test]
    fn test_parse_snapshots() {
        let stdout = "pool/dataset@snap1\t1024\t2048\t1234567890\n\
                      pool/dataset@snap2\t2048\t4096\t1234567900\n";

        let snaps = parse_snapshots(stdout);

        assert_eq!(snaps.len(), 2);
        assert_eq!(snaps[0].name, "snap1");
        assert_eq!(snaps[0].full_name, "pool/dataset@snap1");
        assert_eq!(snaps[0].size, 1024);
        assert_eq!(snaps[1].name, "snap2");
    }

    #[test]
    fn test_parse_dataset_list() {
        let stdout = "pool/dataset1\t1024\t2048\t/mnt/dataset1\n\
                      pool/dataset2\t2048\t4096\t/mnt/dataset2\n";

        let datasets = parse_dataset_list(stdout);

        assert_eq!(datasets.len(), 2);
        assert_eq!(datasets[0].name, "pool/dataset1");
        assert_eq!(datasets[0].used_space, 1024);
        assert_eq!(datasets[1].mount_point, "/mnt/dataset2");
    }
}
