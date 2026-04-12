// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `ZeroCostZfsOperations` trait wiring for [`super::ZeroCostZfsManager`].

use super::ZeroCostZfsManager;
use crate::zero_cost_zfs_operations::traits::ZeroCostZfsOperations;
use crate::zero_cost_zfs_operations::types::{
    ZeroCostDatasetInfo, ZeroCostPoolInfo, ZeroCostSnapshotInfo,
};
use nestgate_core::Result;
use nestgate_core::canonical_types::StorageTier;
use std::collections::HashMap;

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

    async fn create_pool(&self, name: &str, devices: &[&str]) -> Result<Self::Pool> {
        self.pool_create(name, devices).await
    }

    async fn create_dataset(
        &self,
        pool: &Self::Pool,
        name: &str,
        tier: StorageTier,
    ) -> Result<Self::Dataset> {
        self.dataset_create(pool, name, tier).await
    }

    async fn create_snapshot(&self, dataset: &Self::Dataset, name: &str) -> Result<Self::Snapshot> {
        self.snapshot_create(dataset, name).await
    }

    async fn get_pool_properties(&self, pool: &Self::Pool) -> Result<Self::Properties> {
        self.pool_get_properties(pool).await
    }

    async fn list_pools(&self) -> Result<Vec<Self::Pool>> {
        self.pool_list().await
    }

    async fn list_datasets(&self, pool: &Self::Pool) -> Result<Vec<Self::Dataset>> {
        self.dataset_list(pool).await
    }

    async fn list_snapshots(&self, dataset: &Self::Dataset) -> Result<Vec<Self::Snapshot>> {
        self.snapshot_list(dataset).await
    }
}

#[cfg(test)]
mod tests {
    use super::super::TestingZfsManager;
    use super::super::test_zfs_stub::ZfsCommandStubGuard;
    use crate::zero_cost_zfs_operations::traits::ZeroCostZfsOperations;
    use crate::zero_cost_zfs_operations::types::{ZeroCostDatasetInfo, ZeroCostPoolInfo};
    use nestgate_core::Result;
    use nestgate_core::canonical_types::StorageTier;
    use std::collections::HashMap;
    use std::time::SystemTime;

    fn stub_for_trait_forwards() -> impl Fn(&[&str]) -> Result<String> + Send + Sync + 'static {
        |args: &[&str]| -> Result<String> {
            match args.first().copied() {
                Some("get") => {
                    Ok("used\t64\navailable\t1024\nmountpoint\t/tmp\nsize\t1088\n".to_string())
                }
                Some("list") => {
                    if args.contains(&"snapshot") {
                        Ok("tank/ds@s1\t128\n".to_string())
                    } else if args.contains(&"-r") {
                        Ok("tank/ds\t10\t90\t/mnt\n".to_string())
                    } else {
                        Ok("tank\t1000\t100\t900\tONLINE\n".to_string())
                    }
                }
                _ => Ok(String::new()),
            }
        }
    }

    #[tokio::test]
    async fn trait_list_pools_forwards_to_pool_list() {
        let _g = ZfsCommandStubGuard::set(Box::new(stub_for_trait_forwards()));
        let m = TestingZfsManager::new();
        let pools = ZeroCostZfsOperations::list_pools(&m).await.expect("pools");
        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].name, "tank");
    }

    #[tokio::test]
    async fn trait_list_datasets_forwards_to_dataset_list() {
        let _g = ZfsCommandStubGuard::set(Box::new(stub_for_trait_forwards()));
        let m = TestingZfsManager::new();
        let pool = ZeroCostPoolInfo {
            name: "tank".into(),
            size: 0,
            used: 0,
            available: 0,
            health: "ONLINE".into(),
            properties: HashMap::new(),
            created_at: SystemTime::UNIX_EPOCH,
        };
        let ds = ZeroCostZfsOperations::list_datasets(&m, &pool)
            .await
            .expect("datasets");
        assert_eq!(ds.len(), 1);
        assert_eq!(ds[0].name, "ds");
    }

    #[tokio::test]
    async fn trait_list_snapshots_forwards_to_snapshot_list() {
        let _g = ZfsCommandStubGuard::set(Box::new(stub_for_trait_forwards()));
        let m = TestingZfsManager::new();
        let dataset = ZeroCostDatasetInfo {
            name: "ds".into(),
            pool: "tank".into(),
            tier: StorageTier::Warm,
            size: 100,
            used: 10,
            properties: HashMap::new(),
            mount_point: None,
            created_at: SystemTime::UNIX_EPOCH,
        };
        let snaps = ZeroCostZfsOperations::list_snapshots(&m, &dataset)
            .await
            .expect("snaps");
        assert_eq!(snaps.len(), 1);
        assert_eq!(snaps[0].name, "s1");
    }

    #[tokio::test]
    async fn trait_create_dataset_forwards_to_dataset_create() {
        let _g = ZfsCommandStubGuard::set(Box::new(stub_for_trait_forwards()));
        let m = TestingZfsManager::new();
        let pool = ZeroCostPoolInfo {
            name: "tank".into(),
            size: 0,
            used: 0,
            available: 0,
            health: "ONLINE".into(),
            properties: HashMap::new(),
            created_at: SystemTime::UNIX_EPOCH,
        };
        let ds = ZeroCostZfsOperations::create_dataset(&m, &pool, "ds2", StorageTier::Hot)
            .await
            .expect("dataset");
        assert_eq!(ds.name, "ds2");
        assert_eq!(ds.tier, StorageTier::Hot);
    }

    #[tokio::test]
    async fn trait_create_snapshot_forwards_to_snapshot_create() {
        let _g = ZfsCommandStubGuard::set(Box::new(stub_for_trait_forwards()));
        let m = TestingZfsManager::new();
        let dataset = ZeroCostDatasetInfo {
            name: "ds".into(),
            pool: "tank".into(),
            tier: StorageTier::Warm,
            size: 100,
            used: 10,
            properties: HashMap::new(),
            mount_point: None,
            created_at: SystemTime::UNIX_EPOCH,
        };
        let snap = ZeroCostZfsOperations::create_snapshot(&m, &dataset, "snap-a")
            .await
            .expect("snap");
        assert_eq!(snap.name, "snap-a");
    }

    #[test]
    fn trait_compile_time_limits_via_default_methods() {
        assert_eq!(
            <TestingZfsManager as ZeroCostZfsOperations<2, 10, 100>>::max_pools(),
            2
        );
        assert_eq!(
            <TestingZfsManager as ZeroCostZfsOperations<2, 10, 100>>::max_datasets(),
            10
        );
        assert_eq!(
            <TestingZfsManager as ZeroCostZfsOperations<2, 10, 100>>::max_snapshots(),
            100
        );
        let m = TestingZfsManager::new();
        assert!(ZeroCostZfsOperations::can_create_pool(&m));
        assert!(ZeroCostZfsOperations::can_create_dataset(&m));
        assert!(ZeroCostZfsOperations::can_create_snapshot(&m));
    }
}
