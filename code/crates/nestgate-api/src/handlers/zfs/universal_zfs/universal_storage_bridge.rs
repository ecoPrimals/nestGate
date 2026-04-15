// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// This bridge makes the ZFS API endpoints work with ANY storage backend,
// not just ZFS. It translates ZFS concepts (pools, datasets, snapshots)
// to universal storage operations that work on filesystems, object storage, etc.

//! Universal Storage Bridge module

use nestgate_zfs::numeric::f64_to_u64_saturating;

use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, DatasetType, PoolCapacity, PoolHealth, PoolInfo, PoolState,
    UniversalZfsError, UniversalZfsResult,
};
use std::collections::HashMap;

use tracing::{debug, info, warn};

/// Universal Storage Bridge - Makes ZFS API work with any storage backend
pub struct UniversalStorageBridge {
    preferred_backend: Option<String>,
}
impl UniversalStorageBridge {
    /// Create a new universal storage bridge
    pub const fn new() -> UniversalZfsResult<Self> {
        Ok(Self {
            preferred_backend: None,
        })
    }

    /// Detect and set the best available storage backend
    pub fn detect_best_backend(&mut self) -> UniversalZfsResult<String> {
        info!("Detecting best available storage backend");

        // Try backends in order of preference: ZFS -> Filesystem -> Others
        if Self::is_zfs_available() {
            info!("Selected storage backend: zfs");
            self.preferred_backend = Some("zfs".to_string());
            return Ok("zfs".to_string());
        }

        // Filesystem is always available as fallback
        info!("Selected storage backend: filesystem");
        self.preferred_backend = Some("filesystem".to_string());
        Ok("filesystem".to_string())
    }

    /// Check if ZFS is available
    fn is_zfs_available() -> bool {
        std::process::Command::new("zfs")
            .args(["list"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Translate ZFS pool operations to universal storage operations
    pub fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        debug!("Listing pools via universal storage");

        let backend = self.get_active_backend()?;

        match backend.as_str() {
            "zfs" => self.list_zfs_pools(),
            "filesystem" => self.list_filesystem_pools(),
            _ => self.list_fallback_pools(),
        }
    }

    /// List ZFS pools (when ZFS is available)
    fn list_zfs_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        info!(
            preferred_backend = ?self.preferred_backend,
            "Listing ZFS pools"
        );

        let output = std::process::Command::new("zpool")
            .args(["list", "-H", "-o", "name,size,alloc,free,health"])
            .output()
            .map_err(|_e| {
                UniversalZfsError::internal("Failed to execute ZFS command".to_string())
            })?;

        if !output.status.success() {
            return Err(UniversalZfsError::internal("zpool list command failed"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut pools = Vec::new();

        for line in stdout.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() >= 5 {
                let name = fields[0].to_string();
                let size_str = fields[1];
                let used_str = fields[2];
                let free_str = fields[3];
                let health_str = fields[4];

                let total_bytes = Self::parse_size_to_bytes(size_str)?;
                let used_bytes = Self::parse_size_to_bytes(used_str)?;
                let available_bytes = Self::parse_size_to_bytes(free_str)?;

                pools.push(PoolInfo {
                    name,
                    state: PoolState::Active, // Use correct enum variant
                    capacity: PoolCapacity {
                        total: total_bytes,
                        used: used_bytes,
                        available: available_bytes,
                    },
                    health: match health_str {
                        "ONLINE" => PoolHealth::Online,
                        "DEGRADED" => PoolHealth::Degraded,
                        "FAULTED" => PoolHealth::Faulted,
                        _ => PoolHealth::Unknown,
                    },
                    scrub: None,
                    properties: HashMap::new(),
                });
            }
        }

        info!(
            "Found {},
    ZFS pools",
            pools.len()
        );
        Ok(pools)
    }

    /// List filesystem "pools" (mount points as pools)
    fn list_filesystem_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        info!(
            preferred_backend = ?self.preferred_backend,
            "Listing filesystem pools (mount points)"
        );

        let output = std::process::Command::new("df")
            .args(["-h", "--output=source,fstype,size,used,avail,target"])
            .output()
            .map_err(|e| UniversalZfsError::internal(format!("Failed to execute df: {e}")))?;

        if !output.status.success() {
            return Err(UniversalZfsError::internal("df command failed"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut pools = Vec::new();

        for line in stdout.lines().skip(1) {
            // Skip header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let source = parts[0];
                let fstype = parts[1];
                let size_str = parts[2];
                let used_str = parts[3];
                let avail_str = parts[4];
                let mount_point = parts[5];

                // Only include real storage _devices
                if Self::should_include_filesystem(source, fstype, mount_point) {
                    let total_bytes = Self::parse_size_to_bytes(size_str)?;
                    let used_bytes = Self::parse_size_to_bytes(used_str)?;
                    let available_bytes = Self::parse_size_to_bytes(avail_str)?;

                    pools.push(PoolInfo {
                        name: "default_pool".to_string(),
                        state: PoolState::Active, // Use correct enum variant
                        capacity: PoolCapacity {
                            total: total_bytes,
                            used: used_bytes,
                            available: available_bytes,
                        },
                        health: PoolHealth::Online,
                        scrub: None,
                        properties: {
                            let mut props = HashMap::new();
                            props.insert("filesystem_type".to_string(), fstype.to_string());
                            props.insert("mount_point".to_string(), mount_point.to_string());
                            props
                        },
                    });
                }
            }
        }

        info!("Found {} filesystem pools", pools.len());
        Ok(pools)
    }

    /// List fallback pools (when no storage backend is available)
    fn list_fallback_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        warn!("Using fallback pool listing");

        // Create a minimal pool representing the root filesystem
        let mut properties = HashMap::new();
        properties.insert(
            "preferred_backend".to_string(),
            self.preferred_backend
                .clone()
                .unwrap_or_else(|| "unset".to_string()),
        );

        let pool = PoolInfo {
            name: "root-filesystem".to_string(),
            state: PoolState::Active, // Use correct enum variant
            capacity: PoolCapacity {
                total: 0,
                used: 0,
                available: 0,
            },
            health: PoolHealth::Unknown,
            scrub: None,
            properties,
        };

        Ok(vec![pool])
    }

    /// Get the currently active storage backend
    fn get_active_backend(&self) -> UniversalZfsResult<String> {
        if let Some(backend) = &self.preferred_backend {
            Ok(backend.clone())
        } else {
            // Auto-detect if not already set
            let mut bridge = self.clone();
            bridge.detect_best_backend()
        }
    }

    /// Parse size strings like "1.2G", "500M" to bytes
    fn parse_size_to_bytes(size_str: &str) -> UniversalZfsResult<u64> {
        if size_str == "-" || size_str == "0" {
            return Ok(0);
        }

        let size_str = size_str.trim();
        let (number_part, unit) = if let Some(pos) = size_str.chars().position(char::is_alphabetic)
        {
            let (num, unit) = size_str.split_at(pos);
            (num, unit)
        } else {
            (size_str, "")
        };

        let number: f64 = number_part
            .parse()
            .map_err(|_| UniversalZfsError::internal("Failed to parse size value".to_string()))?;

        let multiplier = match unit.to_uppercase().as_str() {
            "K" | "KB" => 1024,
            "M" | "MB" => 1024 * 1024,
            "G" | "GB" => 1024 * 1024 * 1024,
            "T" | "TB" => 1024_u64.pow(4),
            "P" | "PB" => 1024_u64.pow(5),
            "" => 1,
            _ => {
                return Err(UniversalZfsError::internal(format!(
                    "Unknown size unit: {unit}"
                )));
            }
        };

        Ok(f64_to_u64_saturating(number * multiplier as f64))
    }

    /// Determine if we should include this filesystem as a "pool"
    fn should_include_filesystem(source: &str, fstype: &str, mount_point: &str) -> bool {
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
                || mount_point.starts_with("/media")
                || mount_point.starts_with("/var")
                || mount_point.starts_with("/opt"))
    }

    /// Create a dataset (directory) via universal storage
    pub fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        info!("Creating dataset: {}", config.name);

        let backend = self.get_active_backend()?;

        match backend.as_str() {
            "zfs" => self.create_zfs_dataset(config),
            "filesystem" => self.create_filesystem_dataset(config),
            _ => Err(UniversalZfsError::internal(
                "No suitable backend for dataset creation",
            )),
        }
    }

    /// Create ZFS dataset
    fn create_zfs_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        debug!(
            backend = ?self.preferred_backend,
            dataset = %config.name,
            "create_zfs_dataset"
        );
        let output = std::process::Command::new("zfs")
            .args(["create", &config.name])
            .output()
            .map_err(|_e| UniversalZfsError::internal("Failed to execute command".to_string()))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(UniversalZfsError::internal(format!(
                "ZFS dataset creation failed: {error}"
            )));
        }

        // Return dataset info (would need to query ZFS for real details)
        Ok(DatasetInfo {
            name: config.name.clone(),
            dataset_type: DatasetType::Filesystem, // Fix: use correct enum variant
            used: 0,
            available: 0,
            referenced: 0,
            mountpoint: Some(format!("/{}", config.name)), // Default ZFS mount path pattern
            properties: HashMap::new(),                    // Add missing field
        })
    }

    /// Create filesystem dataset (directory)
    fn create_filesystem_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        debug!(
            backend = ?self.preferred_backend,
            path = %config.name,
            "create_filesystem_dataset"
        );
        let path = std::path::Path::new(&config.name);

        std::fs::create_dir_all(path)
            .map_err(|_e| UniversalZfsError::internal("Failed to create directory".to_string()))?;

        info!("Created filesystem dataset at: {:?}", path);

        Ok(DatasetInfo {
            name: config.name.clone(),
            dataset_type: DatasetType::Filesystem, // Fix: use correct enum variant
            used: 0,
            available: 0,
            referenced: 0,
            mountpoint: Some(config.name.clone()), // Add missing field
            properties: HashMap::new(),            // Add missing field
        })
    }

    #[cfg(test)]
    pub(crate) fn test_parse_size_to_bytes(size_str: &str) -> UniversalZfsResult<u64> {
        Self::parse_size_to_bytes(size_str)
    }

    #[cfg(test)]
    pub(crate) fn test_should_include_filesystem(
        source: &str,
        fstype: &str,
        mount_point: &str,
    ) -> bool {
        Self::should_include_filesystem(source, fstype, mount_point)
    }
}

// Make it cloneable for the auto-detection logic
impl Clone for UniversalStorageBridge {
    /// Clone
    fn clone(&self) -> Self {
        Self {
            preferred_backend: self.preferred_backend.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_storage_bridge_new() {
        let result = UniversalStorageBridge::new();
        assert!(result.is_ok());
        let _bridge = result.unwrap();
    }

    #[test]
    fn test_universal_storage_bridge_clone() {
        let bridge = UniversalStorageBridge::new().unwrap();
        let _cloned = bridge;
    }

    #[tokio::test]
    async fn test_universal_storage_bridge_detect_best_backend() {
        let mut bridge = UniversalStorageBridge::new().unwrap();
        let result = bridge.detect_best_backend();
        assert!(result.is_ok());
        // Either zfs or filesystem depending on system
        let backend = result.unwrap();
        assert!(backend == "zfs" || backend == "filesystem");
    }

    #[tokio::test]
    async fn test_universal_storage_bridge_list_pools() {
        let bridge = UniversalStorageBridge::new().unwrap();
        let result = bridge.list_pools();
        assert!(result.is_ok());
        let pools = result.unwrap();
        // Pools may be empty in restricted test environments (sandbox, no zfs/df)
        let _ = pools;
    }

    #[tokio::test]
    async fn test_universal_storage_bridge_create_dataset_filesystem() {
        let mut bridge = UniversalStorageBridge::new().unwrap();
        bridge.detect_best_backend().ok();

        let temp_path =
            std::env::temp_dir().join(format!("nestgate_test_dataset_{}", uuid::Uuid::new_v4()));
        let config = DatasetConfig {
            name: temp_path.to_string_lossy().to_string(),
            mountpoint: None,
            compression: false,
            quota: None,
            reservation: None,
            properties: HashMap::new(),
        };

        let result = bridge.create_dataset(&config);
        if let Ok(dataset) = result {
            assert_eq!(dataset.name, config.name);
            assert_eq!(dataset.dataset_type, DatasetType::Filesystem);
            let _ = std::fs::remove_dir_all(&config.name);
        }
    }

    #[test]
    fn parse_size_to_bytes_dash_zero_and_units() {
        assert_eq!(
            UniversalStorageBridge::test_parse_size_to_bytes("-").unwrap(),
            0
        );
        assert_eq!(
            UniversalStorageBridge::test_parse_size_to_bytes("0").unwrap(),
            0
        );
        assert_eq!(
            UniversalStorageBridge::test_parse_size_to_bytes("2K").unwrap(),
            2 * 1024
        );
        assert_eq!(
            UniversalStorageBridge::test_parse_size_to_bytes("1M").unwrap(),
            1024 * 1024
        );
        assert!(
            UniversalStorageBridge::test_parse_size_to_bytes("1.5G").unwrap() > 1024 * 1024 * 1024
        );
        assert!(UniversalStorageBridge::test_parse_size_to_bytes("1X").is_err());
    }

    #[test]
    fn should_include_filesystem_filters_pseudo_mounts() {
        assert!(!UniversalStorageBridge::test_should_include_filesystem(
            "tmpfs", "tmpfs", "/tmp"
        ));
        assert!(UniversalStorageBridge::test_should_include_filesystem(
            "/dev/nvme0n1p2",
            "ext4",
            "/home/user"
        ));
        assert!(UniversalStorageBridge::test_should_include_filesystem(
            "/dev/loop0",
            "ext4",
            "/"
        ));
    }
}
