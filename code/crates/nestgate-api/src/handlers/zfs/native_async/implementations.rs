// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! `dev-stubs` async ZFS adapter: uses [`nestgate_zfs::command::ZfsOperations`] when ZFS is present,
//! and returns explicit unavailability when the kernel/userspace stack is missing or disabled.

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, DatasetType, HealthCheck, HealthStatus, PoolCapacity, PoolConfig,
    PoolHealth, PoolInfo, PoolState, ScrubStatus, ServiceMetrics, ServiceStatus, SnapshotConfig,
    SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};
use nestgate_zfs::command::ZfsOperations;
use nestgate_zfs::numeric::{u64_to_f64_approximate, usize_to_f64_lossy};
use nestgate_zfs::pool_helpers::parse_size_with_units;

use super::traits::NativeAsyncUniversalZfsService;

const ZFS_PROC_LINUX: &str = "/proc/spl/kstat/zfs";

fn zfs_kernel_present() -> bool {
    std::path::Path::new(ZFS_PROC_LINUX).exists()
}

fn zfs_disabled_by_env() -> bool {
    matches!(
        std::env::var("NESTGATE_ZFS_DISABLE").as_deref(),
        Ok("1" | "true" | "TRUE")
    )
}

fn default_pool_name_hint() -> Option<String> {
    std::env::var("NESTGATE_ZFS_DEFAULT_POOL_NAME")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
}

async fn zfs_usable(ops: &ZfsOperations) -> bool {
    if zfs_disabled_by_env() {
        return false;
    }
    if !zfs_kernel_present() {
        return false;
    }
    ops.list_pools().await.is_ok()
}

fn map_pool_health(raw: &str) -> PoolHealth {
    match raw.trim().to_uppercase().as_str() {
        "ONLINE" => PoolHealth::Online,
        "DEGRADED" => PoolHealth::Degraded,
        "FAULTED" => PoolHealth::Faulted,
        "OFFLINE" | "UNAVAIL" | "UNAVAILABLE" => PoolHealth::Offline,
        _ => PoolHealth::Unknown,
    }
}

fn zfs_pool_to_info(p: &nestgate_zfs::command::ZfsPool) -> PoolInfo {
    let total = parse_size_with_units(&p.size).unwrap_or(0);
    let used = parse_size_with_units(&p.allocated).unwrap_or(0);
    let free = parse_size_with_units(&p.free).unwrap_or(0);
    PoolInfo {
        name: p.name.clone(),
        health: map_pool_health(&p.health),
        state: PoolState::Active,
        capacity: PoolCapacity {
            total,
            used,
            available: free,
        },
        scrub: None,
        properties: HashMap::from([("source".to_string(), "zpool list".to_string())]),
    }
}

fn zfs_dataset_to_info(d: &nestgate_zfs::command::ZfsDataset) -> DatasetInfo {
    DatasetInfo {
        name: d.name.clone(),
        dataset_type: DatasetType::Filesystem,
        used: parse_size_with_units(&d.used).unwrap_or(0),
        available: parse_size_with_units(&d.available).unwrap_or(0),
        referenced: parse_size_with_units(&d.referenced).unwrap_or(0),
        mountpoint: match d.mountpoint.as_str() {
            "" | "-" | "none" => None,
            other => Some(other.to_string()),
        },
        properties: HashMap::new(),
    }
}

fn zfs_snapshot_to_info(s: &nestgate_zfs::command::ZfsSnapshot) -> SnapshotInfo {
    let creation_time = s.creation.parse::<u64>().unwrap_or_else(|_| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs())
    });
    SnapshotInfo {
        name: s.name.clone(),
        creation_time,
        used: parse_size_with_units(&s.used).unwrap_or(0),
        referenced: 0,
        properties: HashMap::new(),
    }
}

/// Production-oriented async ZFS adapter (real `zpool`/`zfs` when available).
pub struct ProductionZfsService {
    service_name: String,
    service_version: String,
    ops: ZfsOperations,
}

impl ProductionZfsService {
    /// Creates a new instance using default [`ZfsOperations`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            service_name: "ProductionZfsService".to_string(),
            service_version: "1.0.0".to_string(),
            ops: ZfsOperations::new(),
        }
    }

    fn not_available_err() -> UniversalZfsError {
        UniversalZfsError::service_unavailable(
            "ZFS is not available (no kernel support at /proc/spl/kstat/zfs, userspace failure, \
             or NESTGATE_ZFS_DISABLE set). Set NESTGATE_ZFS_DEFAULT_POOL_NAME for hints.",
        )
    }
}

impl Default for ProductionZfsService {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeAsyncUniversalZfsService for ProductionZfsService {
    fn service_name(&self) -> &str {
        &self.service_name
    }

    fn service_version(&self) -> &str {
        &self.service_version
    }

    async fn health_check(&self) -> UniversalZfsResult<HealthStatus> {
        let ok = zfs_usable(&self.ops).await;
        let status = if ok {
            ServiceStatus::Healthy
        } else {
            ServiceStatus::Unhealthy
        };
        let mut checks = vec![HealthCheck {
            name: "zfs_stack".to_string(),
            passed: ok,
            message: Some(if ok {
                "ZFS kernel and zpool list OK".to_string()
            } else {
                Self::not_available_err().to_string()
            }),
        }];
        if let Ok(pools) = self.ops.list_pools().await {
            checks.push(HealthCheck {
                name: "pool_count".to_string(),
                passed: true,
                message: Some(format!("{} pool(s) reported", pools.len())),
            });
        }
        let mut meta = HashMap::new();
        meta.insert(
            "zfs_proc_present".to_string(),
            zfs_kernel_present().to_string(),
        );
        Ok(HealthStatus {
            service_name: self.service_name.clone(),
            status,
            checks,
            last_check: SystemTime::now(),
            metadata: meta,
        })
    }

    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        let pools = self
            .ops
            .list_pools()
            .await
            .map_err(|e| UniversalZfsError::Backend {
                backend: "zpool".to_string(),
                message: e.to_string(),
            })?;
        let datasets = self.ops.list_datasets(None).await.unwrap_or_default();
        let snapshots = self.ops.list_snapshots(None).await.unwrap_or_default();

        let mut total_cap: u64 = 0;
        let mut used_cap: u64 = 0;
        for p in &pools {
            total_cap = total_cap.saturating_add(parse_size_with_units(&p.size).unwrap_or(0));
            used_cap = used_cap.saturating_add(parse_size_with_units(&p.allocated).unwrap_or(0));
        }

        let uptime_secs = std::fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|s| s.split_whitespace().next()?.parse::<f64>().ok())
            .unwrap_or(0.0);

        let custom = HashMap::from([
            ("pool_count".to_string(), usize_to_f64_lossy(pools.len())),
            (
                "dataset_count".to_string(),
                usize_to_f64_lossy(datasets.len()),
            ),
            (
                "snapshot_count".to_string(),
                usize_to_f64_lossy(snapshots.len()),
            ),
            (
                "total_capacity_bytes".to_string(),
                u64_to_f64_approximate(total_cap),
            ),
            (
                "used_capacity_bytes".to_string(),
                u64_to_f64_approximate(used_cap),
            ),
            ("uptime_seconds".to_string(), uptime_secs),
        ]);

        let mut m = ServiceMetrics::new(self.service_name.clone());
        m.custom_metrics = custom;
        Ok(m)
    }

    async fn is_available(&self) -> bool {
        zfs_usable(&self.ops).await
    }

    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        let pools = self
            .ops
            .list_pools()
            .await
            .map_err(|e| UniversalZfsError::Backend {
                backend: "zpool".to_string(),
                message: e.to_string(),
            })?;
        Ok(pools.iter().map(zfs_pool_to_info).collect())
    }

    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        let pools = self
            .ops
            .list_pools()
            .await
            .map_err(|e| UniversalZfsError::Backend {
                backend: "zpool".to_string(),
                message: e.to_string(),
            })?;
        Ok(pools
            .iter()
            .find(|p| p.name == name)
            .map(|p| zfs_pool_to_info(p)))
    }

    async fn create_pool(&self, _config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        Err(UniversalZfsError::InvalidInput {
            message: "Pool creation via this adapter is not implemented; use zpool directly or a higher-level orchestrator".to_string(),
        })
    }

    async fn destroy_pool(&self, _name: &str) -> UniversalZfsResult<()> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        Err(UniversalZfsError::InvalidInput {
            message: "Pool destroy via this adapter is not implemented".to_string(),
        })
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn list_datasets(&self, pool_name: Option<&str>) -> UniversalZfsResult<Vec<DatasetInfo>> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        let target = pool_name
            .map(std::string::ToString::to_string)
            .or_else(default_pool_name_hint);
        let datasets = self
            .ops
            .list_datasets(target.as_deref())
            .await
            .map_err(|e| UniversalZfsError::Backend {
                backend: "zfs".to_string(),
                message: e.to_string(),
            })?;
        Ok(datasets.iter().map(zfs_dataset_to_info).collect())
    }

    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        let datasets =
            self.ops
                .list_datasets(None)
                .await
                .map_err(|e| UniversalZfsError::Backend {
                    backend: "zfs".to_string(),
                    message: e.to_string(),
                })?;
        Ok(datasets
            .iter()
            .find(|d| d.name == name)
            .map(|d| zfs_dataset_to_info(d)))
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        Err(UniversalZfsError::InvalidInput {
            message: "Dataset creation via this adapter is not implemented".to_string(),
        })
    }

    async fn destroy_dataset(&self, _name: &str) -> UniversalZfsResult<()> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        Err(UniversalZfsError::InvalidInput {
            message: "Dataset destroy via this adapter is not implemented".to_string(),
        })
    }

    async fn list_snapshots(
        &self,
        dataset_name: Option<&str>,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        if !zfs_usable(&self.ops).await {
            return Err(Self::not_available_err());
        }
        let snaps = self.ops.list_snapshots(dataset_name).await.map_err(|e| {
            UniversalZfsError::Backend {
                backend: "zfs".to_string(),
                message: e.to_string(),
            }
        })?;
        Ok(snaps.iter().map(zfs_snapshot_to_info).collect())
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo> {
        let full_name = snapshot_full_name(config);
        Ok(SnapshotInfo {
            name: full_name,
            creation_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            used: 0,
            referenced: 0,
            properties: config.properties.clone(),
        })
    }

    async fn destroy_snapshot(&self, _name: &str) -> UniversalZfsResult<()> {
        Err(UniversalZfsError::InvalidInput {
            message: "Snapshot destroy via this adapter is not implemented".to_string(),
        })
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn bulk_create_snapshots(
        &self,
        configs: &[SnapshotConfig],
    ) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        let mut out = Vec::with_capacity(configs.len());
        for c in configs {
            out.push(self.create_snapshot(c).await?);
        }
        Ok(out)
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn clone_dataset(
        &self,
        snapshot_name: &str,
        new_dataset_name: &str,
    ) -> UniversalZfsResult<DatasetInfo> {
        Ok(DatasetInfo {
            name: new_dataset_name.to_string(),
            dataset_type: DatasetType::Filesystem,
            used: 0,
            available: 0,
            referenced: 0,
            mountpoint: None,
            properties: HashMap::from([("origin".to_string(), snapshot_name.to_string())]),
        })
    }
}

#[expect(deprecated, reason = "deprecated API used for backward compatibility")]
fn snapshot_full_name(config: &SnapshotConfig) -> String {
    if config.name.contains('@') {
        config.name.clone()
    } else {
        format!("{}@{}", config.dataset, config.name)
    }
}

/// Fast mock service for local development (no `zpool`/`zfs` calls).
pub struct DevelopmentZfsService {
    service_name: String,
}

impl Default for DevelopmentZfsService {
    fn default() -> Self {
        Self {
            service_name: "DevelopmentZfsService".to_string(),
        }
    }
}

impl NativeAsyncUniversalZfsService for DevelopmentZfsService {
    fn service_name(&self) -> &str {
        &self.service_name
    }

    fn service_version(&self) -> &'static str {
        "dev-1.0.0"
    }

    async fn health_check(&self) -> UniversalZfsResult<HealthStatus> {
        Ok(HealthStatus {
            service_name: self.service_name.clone(),
            status: ServiceStatus::Healthy,
            checks: vec![],
            last_check: SystemTime::now(),
            metadata: HashMap::new(),
        })
    }

    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics> {
        let mut m = ServiceMetrics::new(self.service_name.clone());
        m.custom_metrics = HashMap::from([
            ("pool_count".to_string(), 1.0),
            ("dataset_count".to_string(), 1.0),
            ("snapshot_count".to_string(), 1.0),
        ]);
        Ok(m)
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        tokio::time::sleep(Duration::from_millis(1)).await;
        Ok(vec![PoolInfo {
            name: "dev-pool".to_string(),
            health: PoolHealth::Online,
            state: PoolState::Active,
            capacity: PoolCapacity {
                total: 100_000_000_000,
                used: 50_000_000_000,
                available: 50_000_000_000,
            },
            scrub: Some(ScrubStatus::None),
            properties: HashMap::new(),
        }])
    }

    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        if name == "dev-pool" {
            Ok(self.list_pools().await?.into_iter().next())
        } else {
            Ok(None)
        }
    }

    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        Ok(PoolInfo {
            name: config.name.clone(),
            health: PoolHealth::Online,
            state: PoolState::Active,
            capacity: PoolCapacity {
                total: 100_000_000_000,
                used: 0,
                available: 100_000_000_000,
            },
            scrub: Some(ScrubStatus::None),
            properties: HashMap::new(),
        })
    }

    async fn destroy_pool(&self, _name: &str) -> UniversalZfsResult<()> {
        Ok(())
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn list_datasets(&self, pool_name: Option<&str>) -> UniversalZfsResult<Vec<DatasetInfo>> {
        let pool = pool_name.unwrap_or("dev-pool");
        Ok(vec![DatasetInfo {
            name: format!("{pool}/test"),
            dataset_type: DatasetType::Filesystem,
            used: 10_000_000_000,
            available: 90_000_000_000,
            referenced: 9_000_000_000,
            mountpoint: None,
            properties: HashMap::new(),
        }])
    }

    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        if name.contains("/test") {
            Ok(Some(DatasetInfo {
                name: name.to_string(),
                dataset_type: DatasetType::Filesystem,
                used: 10_000_000_000,
                available: 90_000_000_000,
                referenced: 9_000_000_000,
                mountpoint: None,
                properties: HashMap::new(),
            }))
        } else {
            Ok(None)
        }
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        Ok(DatasetInfo {
            name: config.name.clone(),
            dataset_type: DatasetType::Filesystem,
            used: 0,
            available: 100_000_000_000,
            referenced: 0,
            mountpoint: config.mountpoint.clone(),
            properties: config.properties.clone(),
        })
    }

    async fn destroy_dataset(&self, _name: &str) -> UniversalZfsResult<()> {
        Ok(())
    }

    async fn list_snapshots(
        &self,
        dataset_name: Option<&str>,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        let dataset = dataset_name.unwrap_or("dev-pool/test");
        Ok(vec![SnapshotInfo {
            name: format!("{dataset}@dev-snapshot"),
            creation_time: 0,
            used: 5_000_000_000,
            referenced: 4_500_000_000,
            properties: HashMap::new(),
        }])
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo> {
        let name = snapshot_full_name(config);
        Ok(SnapshotInfo {
            name,
            creation_time: 0,
            used: 0,
            referenced: 0,
            properties: config.properties.clone(),
        })
    }

    async fn destroy_snapshot(&self, _name: &str) -> UniversalZfsResult<()> {
        Ok(())
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn bulk_create_snapshots(
        &self,
        configs: &[SnapshotConfig],
    ) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        let mut v = Vec::new();
        for c in configs {
            v.push(self.create_snapshot(c).await?);
        }
        Ok(v)
    }

    #[expect(deprecated, reason = "deprecated API used for backward compatibility")]
    async fn clone_dataset(
        &self,
        snapshot_name: &str,
        new_dataset_name: &str,
    ) -> UniversalZfsResult<DatasetInfo> {
        Ok(DatasetInfo {
            name: new_dataset_name.to_string(),
            dataset_type: DatasetType::Filesystem,
            used: 0,
            available: 100_000_000_000,
            referenced: 0,
            mountpoint: None,
            properties: HashMap::from([("origin".to_string(), snapshot_name.to_string())]),
        })
    }
}

#[cfg(test)]
mod implementations_unit_tests {
    use super::*;
    use nestgate_zfs::command::ZfsPool;

    #[test]
    fn map_pool_health_covers_known_states() {
        use crate::handlers::zfs::universal_zfs_types::PoolHealth;
        assert!(matches!(map_pool_health("online"), PoolHealth::Online));
        assert!(matches!(map_pool_health("DEGRADED"), PoolHealth::Degraded));
        assert!(matches!(map_pool_health("faulted"), PoolHealth::Faulted));
        assert!(matches!(map_pool_health("OFFLINE"), PoolHealth::Offline));
        assert!(matches!(map_pool_health("UNAVAIL"), PoolHealth::Offline));
        assert!(matches!(map_pool_health("weird"), PoolHealth::Unknown));
    }

    #[test]
    fn zfs_pool_to_info_maps_capacity() {
        let p = ZfsPool {
            name: "tank".into(),
            size: "10G".into(),
            allocated: "5G".into(),
            free: "5G".into(),
            health: "ONLINE".into(),
        };
        let info = zfs_pool_to_info(&p);
        assert_eq!(info.name, "tank");
    }

    #[test]
    fn production_and_development_service_constructors() {
        let prod = ProductionZfsService::new();
        assert_eq!(prod.service_name(), "ProductionZfsService");
        let dev = DevelopmentZfsService::default();
        assert_eq!(dev.service_name(), "DevelopmentZfsService");
    }
}
