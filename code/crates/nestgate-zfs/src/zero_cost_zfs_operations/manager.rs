// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ZERO-COST ZFS MANAGER**
//! Main implementation of zero-cost ZFS operations manager

use super::traits::ZeroCostZfsOperations;
use super::types::{DatasetInfoMap, PoolInfoMap, SnapshotInfoMap};
use super::types::{ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo};
use crate::error::{ZfsOperation, create_zfs_error};
use nestgate_core::Result;
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::Duration;
use tokio::sync::RwLock;

/// Zero-cost ZFS operations manager with compile-time capacity limits
pub struct ZeroCostZfsManager<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> {
    pools: PoolInfoMap,
    datasets: DatasetInfoMap,
    snapshots: SnapshotInfoMap,
    #[allow(dead_code)]
    request_id_counter: AtomicU64,
}
impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> Default for ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    /// Create new ZFS manager with compile-time configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            pools: Arc::new(RwLock::new(HashMap::with_capacity(MAX_POOLS))),
            datasets: Arc::new(RwLock::new(HashMap::with_capacity(MAX_DATASETS))),
            snapshots: Arc::new(RwLock::new(HashMap::with_capacity(MAX_SNAPSHOTS))),
            request_id_counter: AtomicU64::new(0),
        }
    }

    /// Get command timeout at compile-time
    #[must_use]
    pub const fn command_timeout() -> Duration {
        Duration::from_millis(COMMAND_TIMEOUT_MS)
    }

    /// Set dataset properties - API compatibility method
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &std::collections::HashMap<String, String>,
    ) -> Result<()> {
        // Implementation using ZFS set command
        for (key, value) in properties {
            self.execute_zfs_command(&["set", &format!("{key}={value}"), dataset_name])
                .await?;
        }
        Ok(())
    }

    /// Destroy snapshot - API compatibility method
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn destroy_snapshot(&self, snapshot_name: &str) -> Result<()> {
        // Implementation using ZFS destroy command
        self.execute_zfs_command(&["destroy", snapshot_name])
            .await?;
        Ok(())
    }

    /// Execute ZFS command with compile-time timeout
    async fn execute_zfs_command(&self, args: &[&str]) -> Result<String> {
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.args(args);

        let output = tokio::time::timeout(Self::command_timeout(), cmd.output())
            .await
            .map_err(|_| {
                create_zfs_error(
                    "ZFS command timed out after self.base_url".to_string(),
                    ZfsOperation::Command,
                )
            })?
            .map_err(|_e| {
                create_zfs_error(
                    "Failed to execute ZFS command: self.base_url".to_string(),
                    ZfsOperation::Command,
                )
            })?;

        if !output.status.success() {
            return Err(create_zfs_error(
                format!(
                    "ZFS command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                ZfsOperation::Command,
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Parse pool properties from ZFS output
    fn parse_pool_properties(&self, output: &str) -> HashMap<String, String> {
        let mut properties = HashMap::new();

        for line in output.lines() {
            if let Some((key, value)) = line.split_once('\t') {
                properties.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        properties
    }

    /// Check if we can create more pools
    async fn can_create_more_pools(&self) -> bool {
        let pools = self.pools.read().await;
        pools.len() < MAX_POOLS
    }

    /// Check if we can create more datasets
    async fn can_create_more_datasets(&self) -> bool {
        let datasets = self.datasets.read().await;
        datasets.len() < MAX_DATASETS
    }

    /// Check if we can create more snapshots
    async fn can_create_more_snapshots(&self) -> bool {
        let snapshots = self.snapshots.read().await;
        snapshots.len() < MAX_SNAPSHOTS
    }
}

impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsOperations<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS>
    for ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    /// Type alias for Error
    type Error = nestgate_core::NestGateError;
    /// Type alias for Pool
    type Pool = ZeroCostPoolInfo;
    /// Type alias for Dataset
    type Dataset = ZeroCostDatasetInfo;
    /// Type alias for Snapshot
    type Snapshot = ZeroCostSnapshotInfo;
    /// Type alias for Properties
    type Properties = HashMap<String, String>;
    /// Creates  Pool
    async fn create_pool(&self, name: &str, devices: &[&str]) -> Result<Self::Pool> {
        // Check capacity at runtime
        if !self.can_create_more_pools().await {
            return Err(create_zfs_error(
                "Cannot create pool: maximum pools reached".to_string(),
                ZfsOperation::PoolCreate,
            ));
        }

        // Build ZFS create command
        let mut args = vec!["create", name];
        args.extend(devices);

        // Execute command
        self.execute_zfs_command(&args).await?;

        // Get pool properties
        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", name])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        // Parse basic pool information
        let size: u64 = properties
            .get("size")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let used: u64 = properties
            .get("allocated")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let available = size.saturating_sub(used);

        let pool_info = ZeroCostPoolInfo {
            name: name.to_string(),
            size,
            used,
            available,
            health: properties
                .get("health")
                .map_or("UNKNOWN".to_string(), std::string::ToString::to_string),
            properties: properties.clone(),
            created_at: std::time::SystemTime::now(),
        };

        // Store in memory cache
        {
            let mut pools_map = self.pools.write().await;
            pools_map.insert(pool_info.name.clone(), pool_info.clone());
        }
        Ok(pool_info)
    }

    /// Creates  Dataset
    async fn create_dataset(
        &self,
        pool: &Self::Pool,
        name: &str,
        tier: StorageTier,
    ) -> Result<Self::Dataset> {
        // Check capacity at runtime
        if !self.can_create_more_datasets().await {
            return Err(create_zfs_error(
                "Cannot create dataset: maximum datasets reached".to_string(),
                ZfsOperation::DatasetCreate,
            ));
        }

        let dataset_path = "dataset.name().to_string()".to_string();

        // Build create command with tier-specific properties
        let mut args = vec!["create"];

        // Apply tier-specific properties
        match tier {
            StorageTier::Hot => {
                args.extend(&["-o", "compression=lz4"]);
                args.extend(&["-o", "sync=always"]);
            }
            StorageTier::Warm => {
                args.extend(&["-o", "compression=gzip"]);
                args.extend(&["-o", "sync=standard"]);
            }
            StorageTier::Cold => {
                args.extend(&["-o", "compression=gzip-9"]);
                args.extend(&["-o", "sync=disabled"]);
            }
            StorageTier::Cache => {
                args.extend(&["-o", "compression=lz4"]);
                args.extend(&["-o", "sync=always"]);
                args.extend(&["-o", "primarycache=all"]);
            }
            StorageTier::Archive => {
                args.extend(&["-o", "compression=gzip-9"]);
                args.extend(&["-o", "sync=disabled"]);
                args.extend(&["-o", "atime=off"]);
            }
        }

        args.push(&dataset_path);

        // Execute command
        self.execute_zfs_command(&args).await?;

        // Get dataset properties
        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", &dataset_path])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        // Parse dataset information
        let used = properties
            .get("used")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let available = properties
            .get("available")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        let size = used + available;

        let mount_point = properties
            .get("mountpoint")
            .filter(|mp| *mp != "none" && *mp != "-")
            .map(PathBuf::from);

        let dataset_info = ZeroCostDatasetInfo {
            name: name.to_string(),
            pool: pool.name.clone(),
            tier,
            size,
            used,
            properties: properties.clone(),
            mount_point,
            created_at: std::time::SystemTime::now(),
        };

        // Store in memory cache
        {
            let mut datasets_map = self.datasets.write().await;
            datasets_map.insert(dataset_info.name.clone(), dataset_info.clone());
        }
        Ok(dataset_info)
    }

    /// Creates  Snapshot
    async fn create_snapshot(
        &self,
        _dataset: &Self::Dataset,
        name: &str,
    ) -> Result<Self::Snapshot> {
        // Check capacity at runtime
        if !self.can_create_more_snapshots().await {
            return Err(create_zfs_error(
                "Cannot create snapshot: maximum snapshots reached".to_string(),
                ZfsOperation::SystemCheck,
            ));
        }

        let dataset_path = "dataset.name().to_string()".to_string();
        let snapshot_path = format!("{dataset_path}@snapshot_name");

        // Execute snapshot command
        self.execute_zfs_command(&["snapshot", &snapshot_path])
            .await?;

        // Get snapshot properties
        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", &snapshot_path])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        let size = properties
            .get("used")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let snapshot_info = ZeroCostSnapshotInfo {
            name: name.to_string(),
            dataset: dataset_path,
            size,
            created_at: std::time::SystemTime::now(),
            properties: properties.clone(),
        };

        // Store in memory cache
        {
            let mut snapshots_map = self.snapshots.write().await;
            snapshots_map.insert(snapshot_info.name.clone(), snapshot_info.clone());
        }
        Ok(snapshot_info)
    }

    /// Gets Pool Properties
    async fn get_pool_properties(&self, pool: &Self::Pool) -> Result<Self::Properties> {
        // Try cache first
        {
            let pools_map = self.pools.read().await;
            if let Some(cached_pool) = pools_map.get(&pool.name) {
                return Ok(cached_pool.properties.clone());
            }
        }

        // Fetch from ZFS
        let properties_output = self
            .execute_zfs_command(&["get", "all", "-H", "-p", &pool.name])
            .await?;

        let properties = self.parse_pool_properties(&properties_output);

        // Update cache
        {
            let mut pools_map = self.pools.write().await;
            let pool_info = ZeroCostPoolInfo {
                name: pool.name.clone(),
                size: properties
                    .get("size")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0),
                used: properties
                    .get("allocated")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0),
                available: properties
                    .get("size")
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0)
                    .saturating_sub(
                        properties
                            .get("allocated")
                            .and_then(|s| s.parse::<u64>().ok())
                            .unwrap_or(0),
                    ),
                health: properties
                    .get("health")
                    .map_or("UNKNOWN".to_string(), std::string::ToString::to_string),
                properties: properties.clone(),
                created_at: std::time::SystemTime::now(),
            };
            pools_map.insert(pool.name.clone(), pool_info);
        }

        Ok(properties)
    }

    /// List Pools
    async fn list_pools(&self) -> Result<Vec<Self::Pool>> {
        // Get pools from ZFS
        let output = self
            .execute_zfs_command(&["list", "-H", "-o", "name,size,used,avail,health"])
            .await?;

        let mut pools = Vec::with_capacity(MAX_POOLS);

        for line in output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                let name = parts[0].to_string();
                let size = parts[1].parse().unwrap_or(0);
                let used = parts[2].parse().unwrap_or(0);
                let available = parts[3].parse().unwrap_or(0);
                let health = parts[4].to_string();

                pools.push(ZeroCostPoolInfo {
                    name: name.clone(),
                    size,
                    used,
                    available,
                    health,
                    properties: HashMap::new(), // Would be populated on demand
                    created_at: std::time::SystemTime::now(), // Approximation
                });

                if pools.len() >= MAX_POOLS {
                    break;
                }
            }
        }

        Ok(pools)
    }

    /// List Datasets
    async fn list_datasets(&self, pool: &Self::Pool) -> Result<Vec<Self::Dataset>> {
        // Get datasets from ZFS
        let output = self
            .execute_zfs_command(&[
                "list",
                "-r",
                "-H",
                "-o",
                "name,used,avail,mountpoint",
                &pool.name,
            ])
            .await?;

        let mut datasets = Vec::with_capacity(MAX_DATASETS);

        for line in output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 4 && parts[0] != pool.name {
                let full_name = parts[0].to_string();
                let name = full_name
                    .strip_prefix(&format!("{}/", pool.name))
                    .unwrap_or(&full_name)
                    .to_string();
                let used = parts[1].parse().unwrap_or(0);
                let available = parts[2].parse().unwrap_or(0);
                let size = used + available;
                let mount_point = if parts[3] != "-" && parts[3] != "none" {
                    Some(PathBuf::from(parts[3]))
                } else {
                    None
                };

                datasets.push(ZeroCostDatasetInfo {
                    name,
                    pool: pool.name.clone(),
                    tier: StorageTier::Warm, // Default, would be determined from properties
                    size,
                    used,
                    properties: HashMap::new(), // Would be populated on demand
                    mount_point,
                    created_at: std::time::SystemTime::now(), // Approximation
                });

                if datasets.len() >= MAX_DATASETS {
                    break;
                }
            }
        }

        Ok(datasets)
    }

    /// List Snapshots
    async fn list_snapshots(&self, _dataset: &Self::Dataset) -> Result<Vec<Self::Snapshot>> {
        let dataset_path = "dataset.name().to_string()".to_string();

        // Get snapshots from ZFS
        let output = self
            .execute_zfs_command(&[
                "list",
                "-r",
                "-t",
                "snapshot",
                "-H",
                "-o",
                "name,used",
                &dataset_path,
            ])
            .await?;

        let mut snapshots = Vec::with_capacity(MAX_SNAPSHOTS);

        for line in output.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let full_name = parts[0].to_string();
                if let Some((ds_path, snap_name)) = full_name.split_once('@') {
                    let size = parts[1].parse().unwrap_or(0);

                    snapshots.push(ZeroCostSnapshotInfo {
                        name: snap_name.to_string(),
                        dataset: ds_path.to_string(),
                        size,
                        created_at: std::time::SystemTime::now(), // Approximation
                        properties: HashMap::new(),               // Would be populated on demand
                    });

                    if snapshots.len() >= MAX_SNAPSHOTS {
                        break;
                    }
                }
            }
        }

        Ok(snapshots)
    }
}

/// **TYPE ALIASES FOR COMMON CONFIGURATIONS**
/// Pre-configured ZFS managers for different use cases
///
/// Development ZFS manager: Small limits, fast timeout
pub type DevelopmentZfsManager = ZeroCostZfsManager<10, 100, 1000, 10_000>; // 10 pools, 100 datasets, 1k snapshots, 10s timeout
/// Production ZFS manager: Large limits, standard timeout
pub type ProductionZfsManager = ZeroCostZfsManager<100, 10_000, 100_000, 30000>; // 100 pools, 10k datasets, 100k snapshots, 30s timeout
/// High-performance ZFS manager: Optimized limits, balanced timeout
pub type HighPerformanceZfsManager = ZeroCostZfsManager<200, 20000, 200000, 45000>; // 200 pools, 20k datasets, 200k snapshots, 45s timeout
/// Testing ZFS manager: Tiny limits, very fast timeout
pub type TestingZfsManager = ZeroCostZfsManager<2, 10, 100, 5000>; // 2 pools, 10 datasets, 100 snapshots, 5s timeout
/// Enterprise ZFS manager: Very large limits, long timeout
pub type EnterpriseZfsManager = ZeroCostZfsManager<1000, 100_000, 1_000_000, 60000>; // 1k pools, 100k datasets, 1M snapshots, 60s timeout

#[cfg(test)]
impl<
    const MAX_POOLS: usize,
    const MAX_DATASETS: usize,
    const MAX_SNAPSHOTS: usize,
    const COMMAND_TIMEOUT_MS: u64,
> ZeroCostZfsManager<MAX_POOLS, MAX_DATASETS, MAX_SNAPSHOTS, COMMAND_TIMEOUT_MS>
{
    pub(crate) fn test_parse_pool_properties(&self, output: &str) -> HashMap<String, String> {
        self.parse_pool_properties(output)
    }

    pub(crate) async fn test_insert_pool_entry(&self, name: String) {
        let mut p = self.pools.write().await;
        p.insert(
            name.clone(),
            ZeroCostPoolInfo {
                name: name.clone(),
                size: 0,
                used: 0,
                available: 0,
                health: "ONLINE".into(),
                properties: HashMap::new(),
                created_at: std::time::SystemTime::UNIX_EPOCH,
            },
        );
    }

    pub(crate) async fn test_pool_map_len(&self) -> usize {
        self.pools.read().await.len()
    }

    pub(crate) async fn test_can_create_more_pools(&self) -> bool {
        self.can_create_more_pools().await
    }

    pub(crate) async fn test_can_create_more_datasets(&self) -> bool {
        self.can_create_more_datasets().await
    }

    pub(crate) async fn test_insert_dataset_entry(&self, name: String, pool: String) {
        let mut d = self.datasets.write().await;
        d.insert(
            name.clone(),
            ZeroCostDatasetInfo {
                name: name.clone(),
                pool,
                tier: StorageTier::Warm,
                size: 0,
                used: 0,
                properties: HashMap::new(),
                mount_point: None,
                created_at: std::time::SystemTime::UNIX_EPOCH,
            },
        );
    }

    pub(crate) async fn test_can_create_more_snapshots(&self) -> bool {
        self.can_create_more_snapshots().await
    }

    pub(crate) async fn test_insert_snapshot_entry(&self, name: String) {
        let mut s = self.snapshots.write().await;
        s.insert(
            name.clone(),
            ZeroCostSnapshotInfo {
                name: name.clone(),
                dataset: "pool/ds".into(),
                size: 0,
                created_at: std::time::SystemTime::UNIX_EPOCH,
                properties: HashMap::new(),
            },
        );
    }

    pub(crate) async fn test_snapshot_map_len(&self) -> usize {
        self.snapshots.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pool_properties_tab_and_trim() {
        let m = TestingZfsManager::new();
        let out = "size\t12345\nallocated\t100\nhealth\tONLINE\n";
        let map = m.test_parse_pool_properties(out);
        assert_eq!(map.get("size"), Some(&"12345".to_string()));
        assert_eq!(map.get("allocated"), Some(&"100".to_string()));
        assert_eq!(map.get("health"), Some(&"ONLINE".to_string()));
    }

    #[test]
    fn parse_pool_properties_skips_lines_without_tab() {
        let m = TestingZfsManager::new();
        let map = m.test_parse_pool_properties("no tab here\nkey\tvalue\n");
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_zero_cost_zfs_manager_new() {
        let manager = ZeroCostZfsManager::<10, 100, 1000, 10_000>::new();
        drop(manager);
    }

    #[test]
    fn test_zero_cost_zfs_manager_default() {
        let manager = TestingZfsManager::default();
        drop(manager);
    }

    #[test]
    fn test_command_timeout_development() {
        let timeout = DevelopmentZfsManager::command_timeout();
        assert_eq!(timeout.as_millis(), 10_000);
    }

    #[test]
    fn test_command_timeout_production() {
        let timeout = ProductionZfsManager::command_timeout();
        assert_eq!(timeout.as_millis(), 30_000);
    }

    #[test]
    fn test_command_timeout_high_performance() {
        let timeout = HighPerformanceZfsManager::command_timeout();
        assert_eq!(timeout.as_millis(), 45_000);
    }

    #[test]
    fn test_command_timeout_testing() {
        let timeout = TestingZfsManager::command_timeout();
        assert_eq!(timeout.as_millis(), 5000);
    }

    #[test]
    fn test_command_timeout_enterprise() {
        let timeout = EnterpriseZfsManager::command_timeout();
        assert_eq!(timeout.as_millis(), 60_000);
    }

    #[test]
    fn test_type_alias_development_zfs_manager() {
        let _manager: DevelopmentZfsManager = DevelopmentZfsManager::new();
    }

    #[test]
    fn test_type_alias_production_zfs_manager() {
        let _manager: ProductionZfsManager = ProductionZfsManager::new();
    }

    #[test]
    fn test_type_alias_high_performance_zfs_manager() {
        let _manager: HighPerformanceZfsManager = HighPerformanceZfsManager::new();
    }

    #[test]
    fn test_type_alias_testing_zfs_manager() {
        let _manager: TestingZfsManager = TestingZfsManager::new();
    }

    #[test]
    fn test_type_alias_enterprise_zfs_manager() {
        let _manager: EnterpriseZfsManager = EnterpriseZfsManager::new();
    }

    #[tokio::test]
    async fn max_pools_capacity_reached_for_testing_manager() {
        let m = TestingZfsManager::new();
        assert!(m.test_can_create_more_pools().await);
        m.test_insert_pool_entry("p0".into()).await;
        m.test_insert_pool_entry("p1".into()).await;
        assert_eq!(m.test_pool_map_len().await, 2);
        assert!(!m.test_can_create_more_pools().await);
    }

    #[tokio::test]
    async fn dataset_capacity_still_available_when_under_limit() {
        let m = TestingZfsManager::new();
        assert!(m.test_can_create_more_datasets().await);
        m.test_insert_dataset_entry("ds0".into(), "p0".into()).await;
        assert_eq!(m.test_can_create_more_datasets().await, true);
    }

    #[tokio::test]
    async fn max_snapshots_capacity_enforced_for_testing_manager() {
        let m = TestingZfsManager::new();
        assert!(m.test_can_create_more_snapshots().await);
        for i in 0..100 {
            m.test_insert_snapshot_entry(format!("snap{i}")).await;
        }
        assert_eq!(m.test_snapshot_map_len().await, 100);
        assert!(!m.test_can_create_more_snapshots().await);
    }

    #[test]
    fn parse_pool_properties_first_tab_wins_on_duplicate_key() {
        let m = TestingZfsManager::new();
        let out = "k\tv1\nk\tv2\n";
        let map = m.test_parse_pool_properties(out);
        assert_eq!(map.get("k"), Some(&"v2".to_string()));
    }

    #[test]
    fn zero_cost_pool_info_serde_roundtrip() {
        let p = ZeroCostPoolInfo {
            name: "tank".into(),
            size: 100,
            used: 10,
            available: 90,
            health: "ONLINE".into(),
            properties: HashMap::from([("ashift".into(), "12".into())]),
            created_at: std::time::SystemTime::UNIX_EPOCH,
        };
        let json = serde_json::to_string(&p).expect("serialize");
        let back: ZeroCostPoolInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.name, p.name);
        assert_eq!(back.health, p.health);
    }

    #[test]
    fn zero_cost_dataset_info_serde_roundtrip() {
        use nestgate_core::canonical_types::StorageTier;
        let d = ZeroCostDatasetInfo {
            name: "d".into(),
            pool: "tank".into(),
            tier: StorageTier::Cold,
            size: 1,
            used: 1,
            properties: HashMap::new(),
            mount_point: None,
            created_at: std::time::SystemTime::UNIX_EPOCH,
        };
        let json = serde_json::to_string(&d).expect("serialize");
        let back: ZeroCostDatasetInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.tier, StorageTier::Cold);
    }
}
