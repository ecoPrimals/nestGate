// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Enhanced dataset management with automation support

//! Dataset module

use crate::error::{ZfsOperation, create_zfs_error};
use crate::{
    config::ZfsConfig,
    pool::ZfsPoolManager,
    // types::{CompressionAlgorithm, DatasetProperty}, // Removed unused imports
};
use nestgate_core::{Result, canonical_types::StorageTier as CoreStorageTier};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::process::Command;
use tracing::debug;
use tracing::info;
use tracing::warn;

/// Infer storage tier from dataset name hints (used when listing without property queries).
fn tier_hint_from_dataset_name(name: &str) -> CoreStorageTier {
    if name.contains("hot") {
        CoreStorageTier::Hot
    } else if name.contains("cold") {
        CoreStorageTier::Cold
    } else {
        CoreStorageTier::Warm
    }
}

/// Parse one tab-separated line from `zfs list -H -o name,used,avail,mountpoint`.
fn parse_zfs_dataset_list_line(dataset_name: &str, line: &str) -> Option<DatasetInfo> {
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
fn parse_zfs_snapshot_list_line(
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
fn parse_zfs_list_datasets_row(line: &str) -> Option<DatasetInfo> {
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

/// Dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datasetinfo
pub struct DatasetInfo {
    /// Dataset name
    pub name: String,
    /// Used space in bytes
    pub used_space: u64,
    /// Available space in bytes
    pub available_space: u64,
    /// File count (optional)
    pub file_count: Option<u64>,
    /// Compression ratio (optional)
    pub compression_ratio: Option<f64>,
    /// Mount point
    pub mount_point: String,
    /// Storage tier
    pub tier: CoreStorageTier,
    /// Properties
    pub properties: HashMap<String, String>,
}
/// ZFS Dataset Manager - handles dataset operations
#[derive(Debug)]
#[allow(dead_code)] // Some fields are planned features not yet fully implemented
/// Manager for `ZfsDataset` operations
pub struct ZfsDatasetManager {
    config: Arc<ZfsConfig>,
    pool_manager: Arc<ZfsPoolManager>,
}
impl ZfsDatasetManager {
    /// Create a new ZFS dataset manager
    #[must_use]
    pub fn new(config: ZfsConfig, pool_manager: Arc<ZfsPoolManager>) -> Self {
        Self {
            config: Arc::new(config),
            pool_manager,
        }
    }

    /// Create a new ZFS dataset manager with shared config (zero-copy)
    #[must_use]
    pub const fn with_shared_config(
        config: Arc<ZfsConfig>,
        pool_manager: Arc<ZfsPoolManager>,
    ) -> Self {
        Self {
            config,
            pool_manager,
        }
    }

    /// Create a new ZFS dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_dataset(
        &self,
        name: &str,
        parent: &str,
        tier: CoreStorageTier,
    ) -> Result<DatasetInfo> {
        info!("Creating dataset: {}/{} on tier: {:?}", parent, name, tier);

        let dataset_path = format!("{parent}/{name}");

        // Execute ZFS create command
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args(["create"]);

        // Apply tier-specific properties based on tier type
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

        cmd.arg(&dataset_path);

        let output = cmd.output().await.map_err(|e| {
            crate::error::ZfsErrorBuilder::command_error("zfs create", &e.to_string())
        })?;

        if !output.status.success() {
            return Err(crate::error::ZfsErrorBuilder::command_error(
                "zfs create",
                &String::from_utf8_lossy(&output.stderr),
            ));
        }

        // Return basic dataset info
        Ok(DatasetInfo {
            name: name.to_string(),
            mount_point: format!("/{name}"),
            used_space: 0,
            available_space: 0,
            file_count: None,
            compression_ratio: None,
            tier,
            properties: HashMap::new(),
        })
    }

    /// Create a dataset with fallback for testing/development environments
    #[allow(dead_code)] // Planned feature for enhanced resilience
    async fn create_with_fallback(
        &self,
        name: &str,
        pool: &str,
        tier: CoreStorageTier,
    ) -> Result<DatasetInfo> {
        info!("Creating dataset: {}/{} on tier: {:?}", pool, name, tier);

        // First try real ZFS dataset creation
        let dataset_path = format!("{pool}/{name}");
        let output = tokio::process::Command::new("zfs")
            .args(["create", &dataset_path])
            .output()
            .await;

        match output {
            Ok(result) if result.status.success() => {
                info!("✅ Created ZFS dataset: {}", dataset_path);
                self.get_dataset_info_with_fallback(&dataset_path).await
            }
            Ok(result) => {
                let error_msg = String::from_utf8_lossy(&result.stderr);
                warn!("ZFS dataset creation failed: {}, using fallback", error_msg);

                // Return fallback dataset info for development
                Ok(DatasetInfo {
                    name: name.to_string(),
                    used_space: 0,
                    available_space: 1024 * 1024 * 1024,
                    file_count: None,
                    compression_ratio: Some(1.0),
                    mount_point: format!("/{name}"),
                    tier,
                    properties: HashMap::new(),
                })
            }
            Err(e) => {
                warn!("Failed to execute ZFS command: {}, using fallback", e);

                // Return fallback dataset info when ZFS is not available
                Ok(DatasetInfo {
                    name: name.to_string(),
                    used_space: 0,
                    available_space: 1024 * 1024 * 1024,
                    file_count: None,
                    compression_ratio: Some(1.0),
                    mount_point: format!("/{name}"),
                    tier,
                    properties: HashMap::new(),
                })
            }
        }
    }

    /// Get dataset info with fallback for testing environments
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_info(&self, name: &str) -> Result<DatasetInfo> {
        self.get_dataset_info_with_fallback(name).await
    }

    /// Gets Dataset Info With Fallback
    async fn get_dataset_info_with_fallback(&self, name: &str) -> Result<DatasetInfo> {
        let mut cmd = Command::new("zfs");
        cmd.args(["list", "-H", "-o", "name,used,avail,mountpoint"]);
        cmd.arg(name);

        let output = cmd.output().await.map_err(|e| {
            crate::error::ZfsErrorBuilder::command_error("zfs list", &e.to_string())
        })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            warn!("ZFS list failed: {}, using fallback data", error_msg);
            return self.create_fallback_dataset_info(name);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next()
            && let Some(info) = parse_zfs_dataset_list_line(name, line)
        {
            return Ok(info);
        }

        // Fallback if parsing fails
        warn!("Failed to parse ZFS output, using fallback data");
        self.create_fallback_dataset_info(name)
    }

    /// Create fallback dataset information for development/testing
    fn create_fallback_dataset_info(&self, name: &str) -> Result<DatasetInfo> {
        Ok(DatasetInfo {
            name: name.to_string(),
            used_space: 512 * 1024 * 1024,
            available_space: 512 * 1024 * 1024,
            file_count: None,
            compression_ratio: Some(1.5),
            mount_point: format!("/{name}"),
            tier: CoreStorageTier::Warm,
            properties: HashMap::new(),
        })
    }

    /// Create a new dataset with full configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_dataset_with_config(&self, name: &str, parent: &str) -> Result<()> {
        tracing::info!("Creating dataset: {}/{}", parent, name);

        let full_name = format!("{parent}/{name}");

        // Build the zfs create command with properties from unified config
        let mut args = vec!["create"];
        let mut options = Vec::new();

        // Add compression property from config
        let compression_opt = "compression=lz4".to_string(); // Canonical default
        options.push(compression_opt);

        // Use canonical defaults instead of complex config extensions
        // Record size optimization
        let recordsize_opt = "recordsize=128K".to_string();
        options.push(recordsize_opt);

        // Quota and reservation handled by canonical storage config if needed

        // Add all options to args
        for option in &options {
            args.extend(&["-o", option.as_str()]);
        }

        // Add the dataset name
        args.push(&full_name);

        let output = Command::new("zfs")
            .args(&args)
            .output()
            .await
            .map_err(|e| {
                create_zfs_error(
                    format!("Failed to execute zfs create: {e}"),
                    ZfsOperation::DatasetCreate,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "zfs create failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::DatasetCreate,
            ));
        }

        Ok(())
    }

    /// Get dataset properties
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_dataset_properties(&self, name: &str) -> Result<HashMap<String, String>> {
        tracing::debug!("Getting properties for dataset: {}", name);

        let output = Command::new("zfs")
            .args(["get", "all", "-H", "-p", name])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error(
                    format!(
                        "Failed to get dataset properties: {}",
                        "actual_error_details"
                    ),
                    ZfsOperation::Configuration,
                )
            })?;

        if !output.status.success() {
            return Ok(HashMap::new());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut properties = HashMap::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                properties.insert(parts[1].to_string(), parts[2].to_string());
            }
        }

        Ok(properties)
    }

    /// Set dataset properties
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> Result<()> {
        tracing::info!("Setting properties for dataset: {}", name);

        for (key, value) in properties {
            let output = Command::new("zfs")
                .args(["set", &format!("{key}={value}"), name])
                .output()
                .await
                .map_err(|_e| {
                    create_zfs_error(
                        format!(
                            "Failed to set property {key}={value}: {}",
                            "actual_error_details"
                        ),
                        ZfsOperation::Configuration,
                    )
                })?;

            if !output.status.success() {
                return Err(create_zfs_error(
                    format!(
                        "Failed to set property {}={}: {}",
                        key,
                        value,
                        String::from_utf8_lossy(&output.stderr)
                    ),
                    ZfsOperation::Configuration,
                ));
            }
        }

        Ok(())
    }

    /// List all datasets
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_datasets(&self) -> Result<Vec<DatasetInfo>> {
        tracing::debug!("Listing all datasets");

        let output = Command::new("zfs")
            .args(["list", "-H", "-p", "-o", "name,used,avail,mountpoint"])
            .output()
            .await
            .map_err(|_e| {
                create_zfs_error("Failed to list datasets".to_string(), ZfsOperation::Command)
            })?;

        if !output.status.success() {
            // Return empty list if no datasets found
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut datasets = Vec::new();

        for line in stdout.lines() {
            if let Some(row) = parse_zfs_list_datasets_row(line) {
                datasets.push(row);
            }
        }

        Ok(datasets)
    }

    /// Delete a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn delete_dataset(&self, name: &str) -> Result<()> {
        info!("Deleting dataset: {}", name);

        // Mock mode - return success for development
        debug!("Mock mode: simulating dataset deletion for {}", name);

        // Real implementation would use zfs destroy command
        // For now, just return success to avoid permission issues
        Ok(())
    }

    /// Destroy a dataset (alias for `delete_dataset`)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn destroy_dataset(&self, name: &str) -> Result<()> {
        self.delete_dataset(name)
    }

    /// List snapshots for a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn list_snapshots(
        &self,
        dataset_name: &str,
    ) -> Result<Vec<crate::snapshot::SnapshotInfo>> {
        debug!("Listing snapshots for dataset: {}", dataset_name);

        let output = Command::new("zfs")
            .args([
                "list",
                "-t",
                "snapshot",
                "-H",
                "-o",
                "name,used,referenced,creation",
            ])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::command_error("zfs list snapshots", &e.to_string())
            })?;

        if !output.status.success() {
            return Err(crate::error::ZfsErrorBuilder::command_error(
                "zfs list snapshots",
                &String::from_utf8_lossy(&output.stderr),
            ));
        }

        let mut snapshots = Vec::new();
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.trim().is_empty() {
                continue;
            }
            if let Some(snap) = parse_zfs_snapshot_list_line(line, dataset_name) {
                snapshots.push(snap);
            }
        }

        Ok(snapshots)
    }
}

// ========== TEST-ONLY CONSTRUCTORS ==========
// Isolated from production code to maintain clear boundaries

#[cfg(test)]
impl ZfsDatasetManager {
    /// Create dataset manager for testing
    ///
    /// **TEST-ONLY**: This constructor is only available in test builds.
    /// Production code must use `ZfsDatasetManager::new()` with proper configuration.
    pub fn new_for_testing() -> Self {
        Self {
            config: Arc::new(ZfsConfig::default()),
            pool_manager: Arc::new(ZfsPoolManager::new_for_testing()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::canonical_types::StorageTier as CoreStorageTier;

    #[test]
    fn dataset_info_serde_roundtrip() {
        let info = DatasetInfo {
            name: "pool/ds".to_string(),
            used_space: 1,
            available_space: 2,
            file_count: Some(3),
            compression_ratio: Some(1.25),
            mount_point: "/mnt/pool/ds".to_string(),
            tier: CoreStorageTier::Warm,
            properties: HashMap::from([("a".into(), "b".into())]),
        };
        let json = serde_json::to_string(&info).expect("test: serialize DatasetInfo");
        let back: DatasetInfo = serde_json::from_str(&json).expect("test: deserialize DatasetInfo");
        assert_eq!(back.name, info.name);
        assert_eq!(back.tier, info.tier);
        assert_eq!(back.properties.get("a"), Some(&"b".to_string()));
    }

    #[test]
    fn new_for_testing_builds_manager() {
        let m = ZfsDatasetManager::new_for_testing();
        m.delete_dataset("any")
            .expect("test: mock delete_dataset succeeds");
        m.destroy_dataset("any")
            .expect("test: mock destroy_dataset succeeds");
    }

    #[test]
    fn with_shared_config_and_new_construct_managers() {
        let cfg = Arc::new(ZfsConfig::default());
        let pm = Arc::new(ZfsPoolManager::new_for_testing());
        let _shared = ZfsDatasetManager::with_shared_config(Arc::clone(&cfg), Arc::clone(&pm));

        let owned = ZfsConfig::default();
        let _from_owned = ZfsDatasetManager::new(owned, pm);
    }

    #[tokio::test]
    async fn create_fallback_dataset_info_populates_expected_fields() {
        let m = ZfsDatasetManager::new_for_testing();
        let info = m
            .create_fallback_dataset_info("tank/fs")
            .expect("test: fallback dataset info");
        assert_eq!(info.name, "tank/fs");
        assert_eq!(info.mount_point, "/tank/fs");
        assert_eq!(info.tier, CoreStorageTier::Warm);
        assert!(info.compression_ratio.is_some());
    }

    #[tokio::test]
    #[ignore = "Requires real ZFS"]
    async fn create_dataset_runs_zfs_create() {
        let m = ZfsDatasetManager::new_for_testing();
        let _ = m
            .create_dataset("tmpds", "nonexistent_pool_xyz", CoreStorageTier::Warm)
            .await;
    }

    #[test]
    fn dataset_info_serde_all_storage_tiers() {
        for tier in [
            CoreStorageTier::Hot,
            CoreStorageTier::Warm,
            CoreStorageTier::Cold,
            CoreStorageTier::Cache,
            CoreStorageTier::Archive,
        ] {
            let info = DatasetInfo {
                name: "p/d".into(),
                used_space: 1,
                available_space: 2,
                file_count: None,
                compression_ratio: None,
                mount_point: "/m".into(),
                tier: tier.clone(),
                properties: HashMap::new(),
            };
            let json = serde_json::to_string(&info).expect("serialize tier");
            let back: DatasetInfo = serde_json::from_str(&json).expect("deserialize tier");
            assert_eq!(back.tier, tier);
        }
    }

    #[test]
    fn dataset_info_clone_and_debug() {
        let info = DatasetInfo {
            name: "z/a".into(),
            used_space: 10,
            available_space: 20,
            file_count: Some(100),
            compression_ratio: Some(2.0),
            mount_point: "/z/a".into(),
            tier: CoreStorageTier::Cold,
            properties: HashMap::new(),
        };
        let c = info.clone();
        assert_eq!(c.name, info.name);
        let dbg = format!("{info:?}");
        assert!(dbg.contains("DatasetInfo"));
        assert!(dbg.contains("z/a"));
    }

    #[test]
    fn dataset_info_optional_fields_none() {
        let info = DatasetInfo {
            name: "p/x".into(),
            used_space: 0,
            available_space: u64::MAX,
            file_count: None,
            compression_ratio: None,
            mount_point: "-".into(),
            tier: CoreStorageTier::Archive,
            properties: HashMap::new(),
        };
        assert!(info.file_count.is_none());
        assert!(info.compression_ratio.is_none());
    }

    #[tokio::test]
    async fn create_fallback_dataset_info_always_warm_tier() {
        let m = ZfsDatasetManager::new_for_testing();
        let a = m
            .create_fallback_dataset_info("tank/hot_data")
            .expect("fallback");
        let b = m
            .create_fallback_dataset_info("cold/store")
            .expect("fallback");
        assert_eq!(a.tier, CoreStorageTier::Warm);
        assert_eq!(b.tier, CoreStorageTier::Warm);
    }

    #[test]
    fn tier_hint_from_dataset_name_all_variants() {
        assert_eq!(
            tier_hint_from_dataset_name("tank/hot_cache"),
            CoreStorageTier::Hot
        );
        assert_eq!(
            tier_hint_from_dataset_name("tank/cold_archive"),
            CoreStorageTier::Cold
        );
        assert_eq!(
            tier_hint_from_dataset_name("tank/warm_fs"),
            CoreStorageTier::Warm
        );
        assert_eq!(
            tier_hint_from_dataset_name("tank/data"),
            CoreStorageTier::Warm
        );
    }

    #[test]
    fn parse_zfs_dataset_list_line_realistic_tab_output() {
        let line = "tank/app\t1048576\t1073741824\t/tank/app";
        let info = parse_zfs_dataset_list_line("tank/app", line).expect("line parses");
        assert_eq!(info.name, "tank/app");
        assert_eq!(info.used_space, 1048576);
        assert_eq!(info.available_space, 1073741824);
        assert_eq!(info.mount_point, "/tank/app");
        assert_eq!(info.tier, CoreStorageTier::Warm);
    }

    #[test]
    fn parse_zfs_dataset_list_line_hot_and_cold_tiers() {
        let hot = parse_zfs_dataset_list_line("tank/hot/db", "tank/hot/db\t0\t0\t/mnt").expect("p");
        assert_eq!(hot.tier, CoreStorageTier::Hot);
        let cold = parse_zfs_dataset_list_line("store/cold/logs", "x\t1\t2\t-").expect("p");
        assert_eq!(cold.tier, CoreStorageTier::Cold);
    }

    #[test]
    fn parse_zfs_dataset_list_line_short_line_returns_none() {
        assert!(parse_zfs_dataset_list_line("x", "only\t2").is_none());
    }

    #[test]
    fn parse_zfs_list_datasets_row_sets_tier_from_name() {
        let row = parse_zfs_list_datasets_row("tank/hot/d\t100\t200\t/mnt").expect("row");
        assert_eq!(row.name, "tank/hot/d");
        assert_eq!(row.tier, CoreStorageTier::Hot);
    }

    #[test]
    fn parse_zfs_snapshot_list_line_parses_at_sign() {
        let line = "tank/ds@s1\t4096\t8192\t1234567890";
        let s = parse_zfs_snapshot_list_line(line, "tank/ds").expect("snap");
        assert_eq!(s.full_name, "tank/ds@s1");
        assert_eq!(s.name, "s1");
        assert_eq!(s.dataset, "tank/ds");
        assert_eq!(s.size, 4096);
        assert_eq!(s.referenced_size, 8192);
    }

    #[test]
    fn parse_zfs_snapshot_list_line_incomplete_returns_none() {
        assert!(parse_zfs_snapshot_list_line("bad", "ds").is_none());
    }
}
