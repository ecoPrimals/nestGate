//! Universal Storage Bridge for ZFS API
//!
//! This bridge makes the ZFS API endpoints work with ANY storage backend,
//! not just ZFS. It translates ZFS concepts (pools, datasets, snapshots)
//! to universal storage operations that work on filesystems, object storage, etc.

use crate::handlers::zfs::universal_zfs::types::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::{debug, info, warn};

/// Universal Storage Bridge - Makes ZFS API work with any storage backend
pub struct UniversalStorageBridge {
    preferred_backend: Option<String>,
}

impl UniversalStorageBridge {
    /// Create a new universal storage bridge
    pub async fn new() -> UniversalZfsResult<Self> {
        Ok(Self {
            preferred_backend: None,
        })
    }

    /// Detect and set the best available storage backend
    pub async fn detect_best_backend(&mut self) -> UniversalZfsResult<String> {
        info!("🔍 Detecting best available storage backend");

        // Try backends in order of preference: ZFS -> Filesystem -> Others
        if self.is_zfs_available().await {
            info!("✅ Selected storage backend: zfs");
            self.preferred_backend = Some("zfs".to_string());
            return Ok("zfs".to_string());
        }

        // Filesystem is always available as fallback
        info!("✅ Selected storage backend: filesystem");
        self.preferred_backend = Some("filesystem".to_string());
        Ok("filesystem".to_string())
    }

    /// Check if ZFS is available
    async fn is_zfs_available(&self) -> bool {
        std::process::Command::new("zfs")
            .args(["list"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Translate ZFS pool operations to universal storage operations
    pub async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        debug!("🔍 Listing pools via universal storage");

        let backend = self.get_active_backend().await?;

        match backend.as_str() {
            "zfs" => self.list_zfs_pools().await,
            "filesystem" => self.list_filesystem_pools().await,
            _ => self.list_fallback_pools().await,
        }
    }

    /// List ZFS pools (when ZFS is available)
    async fn list_zfs_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        info!("📋 Listing ZFS pools");

        let output = std::process::Command::new("zpool")
            .args(["list", "-H", "-o", "name,size,alloc,free,health"])
            .output()
            .map_err(|e| {
                UniversalZfsError::internal(format!("Failed to execute zpool list: {}", e))
            })?;

        if !output.status.success() {
            return Err(UniversalZfsError::internal("zpool list command failed").into());
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

                let total_bytes = self.parse_size_to_bytes(size_str)?;
                let used_bytes = self.parse_size_to_bytes(used_str)?;
                let available_bytes = self.parse_size_to_bytes(free_str)?;

                pools.push(PoolInfo {
                    name,
                    state: PoolState::Active, // Use correct enum variant
                    capacity: PoolCapacity {
                        total_bytes,
                        used_bytes,
                        available_bytes,
                        utilization_percent: if total_bytes > 0 {
                            (used_bytes as f64 / total_bytes as f64) * 100.0
                        } else {
                            0.0
                        },
                    },
                    health: match health_str {
                        "ONLINE" => PoolHealth::Online,
                        "DEGRADED" => PoolHealth::Degraded,
                        "FAULTED" => PoolHealth::Faulted,
                        _ => PoolHealth::Unknown,
                    },
                    devices: vec![], // Would need additional zpool status call
                    errors: vec![],  // Fix: use Vec instead of HashMap
                    created_at: SystemTime::now(), // Would need zpool history
                    last_scrub: None,
                    scrub_status: ScrubStatus::None, // Add missing field
                    properties: HashMap::new(),
                });
            }
        }

        info!("✅ Found {} ZFS pools", pools.len());
        Ok(pools)
    }

    /// List filesystem "pools" (mount points as pools)
    async fn list_filesystem_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        info!("📋 Listing filesystem pools (mount points)");

        let output = std::process::Command::new("df")
            .args(["-h", "--output=source,fstype,size,used,avail,target"])
            .output()
            .map_err(|e| UniversalZfsError::internal(format!("Failed to execute df: {}", e)))?;

        if !output.status.success() {
            return Err(UniversalZfsError::internal("df command failed").into());
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

                // Only include real storage devices
                if self.should_include_filesystem(source, fstype, mount_point) {
                    let total_bytes = self.parse_size_to_bytes(size_str)?;
                    let used_bytes = self.parse_size_to_bytes(used_str)?;
                    let available_bytes = self.parse_size_to_bytes(avail_str)?;

                    pools.push(PoolInfo {
                        name: format!("{} ({})", source, mount_point),
                        state: PoolState::Active, // Use correct enum variant
                        capacity: PoolCapacity {
                            total_bytes,
                            used_bytes,
                            available_bytes,
                            utilization_percent: if total_bytes > 0 {
                                (used_bytes as f64 / total_bytes as f64) * 100.0
                            } else {
                                0.0
                            },
                        },
                        health: PoolHealth::Online,
                        devices: vec![],
                        errors: vec![], // Fix: use Vec instead of HashMap
                        created_at: SystemTime::now(),
                        last_scrub: None,
                        scrub_status: ScrubStatus::None, // Add missing field
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

        info!("✅ Found {} filesystem pools", pools.len());
        Ok(pools)
    }

    /// List fallback pools (when no storage backend is available)
    async fn list_fallback_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        warn!("📋 Using fallback pool listing");

        // Create a minimal pool representing the root filesystem
        let pool = PoolInfo {
            name: "root-filesystem".to_string(),
            state: PoolState::Active, // Use correct enum variant
            capacity: PoolCapacity {
                total_bytes: 0,
                used_bytes: 0,
                available_bytes: 0,
                utilization_percent: 0.0,
            },
            health: PoolHealth::Unknown,
            devices: vec![],
            errors: vec![], // Fix: use Vec instead of HashMap
            created_at: SystemTime::now(),
            last_scrub: None,
            scrub_status: ScrubStatus::None, // Add missing field
            properties: HashMap::new(),
        };

        Ok(vec![pool])
    }

    /// Get the currently active storage backend
    async fn get_active_backend(&self) -> UniversalZfsResult<String> {
        if let Some(backend) = &self.preferred_backend {
            Ok(backend.clone())
        } else {
            // Auto-detect if not already set
            let mut bridge = self.clone();
            bridge.detect_best_backend().await
        }
    }

    /// Parse size strings like "1.2G", "500M" to bytes
    fn parse_size_to_bytes(&self, size_str: &str) -> UniversalZfsResult<u64> {
        if size_str == "-" || size_str == "0" {
            return Ok(0);
        }

        let size_str = size_str.trim();
        let (number_part, unit) =
            if let Some(pos) = size_str.chars().position(|c| c.is_alphabetic()) {
                let (num, unit) = size_str.split_at(pos);
                (num, unit)
            } else {
                (size_str, "")
            };

        let number: f64 = number_part.parse().map_err(|_| {
            UniversalZfsError::internal(format!("Failed to parse size: {}", size_str))
        })?;

        let multiplier = match unit.to_uppercase().as_str() {
            "K" | "KB" => 1024,
            "M" | "MB" => 1024 * 1024,
            "G" | "GB" => 1024 * 1024 * 1024,
            "T" | "TB" => 1024_u64.pow(4),
            "P" | "PB" => 1024_u64.pow(5),
            "" => 1,
            _ => {
                return Err(
                    UniversalZfsError::internal(format!("Unknown size unit: {}", unit)).into(),
                )
            }
        };

        Ok((number * multiplier as f64) as u64)
    }

    /// Determine if we should include this filesystem as a "pool"
    fn should_include_filesystem(&self, source: &str, fstype: &str, mount_point: &str) -> bool {
        // Include real storage devices and important mount points
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
    pub async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        info!("🏗️ Creating dataset: {}", config.name);

        let backend = self.get_active_backend().await?;

        match backend.as_str() {
            "zfs" => self.create_zfs_dataset(config).await,
            "filesystem" => self.create_filesystem_dataset(config).await,
            _ => Err(UniversalZfsError::internal(
                "No suitable backend for dataset creation",
            )),
        }
    }

    /// Create ZFS dataset
    async fn create_zfs_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        let output = std::process::Command::new("zfs")
            .args(["create", &config.name])
            .output()
            .map_err(|e| {
                UniversalZfsError::internal(format!("Failed to create ZFS dataset: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(UniversalZfsError::internal(format!(
                "ZFS dataset creation failed: {}",
                error
            ))
            .into());
        }

        // Return dataset info (would need to query ZFS for real details)
        Ok(DatasetInfo {
            name: config.name.clone(),
            dataset_type: DatasetType::Filesystem, // Fix: use correct enum variant
            used_space: 0,
            available_space: 0,
            parent: None,
            children: vec![],
            created_at: SystemTime::now(), // Add missing field
            mount_point: Some(format!("/{}", config.name)), // Add missing field
            properties: HashMap::new(),    // Add missing field
        })
    }

    /// Create filesystem dataset (directory)
    async fn create_filesystem_dataset(
        &self,
        config: &DatasetConfig,
    ) -> UniversalZfsResult<DatasetInfo> {
        let path = std::path::Path::new(&config.name);

        std::fs::create_dir_all(path).map_err(|e| {
            UniversalZfsError::internal(format!("Failed to create directory: {}", e))
        })?;

        info!("✅ Created filesystem dataset at: {:?}", path);

        Ok(DatasetInfo {
            name: config.name.clone(),
            dataset_type: DatasetType::Filesystem, // Fix: use correct enum variant
            used_space: 0,
            available_space: 0,
            parent: None,
            children: vec![],
            created_at: SystemTime::now(),          // Add missing field
            mount_point: Some(config.name.clone()), // Add missing field
            properties: HashMap::new(),             // Add missing field
        })
    }
}

// Make it cloneable for the auto-detection logic
impl Clone for UniversalStorageBridge {
    fn clone(&self) -> Self {
        Self {
            preferred_backend: self.preferred_backend.clone(),
        }
    }
}
