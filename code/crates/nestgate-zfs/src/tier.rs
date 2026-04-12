// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// This module will be fully implemented in Week 2

//! Tier module

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::dataset::DatasetInfo;
use crate::performance::types::TierStatsMap;
use crate::{
    config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager, types::StorageTier,
};
use nestgate_core::Result;
use tracing::info;
use tracing::warn;

fn tier_stats_map_from_datasets(datasets: &[DatasetInfo]) -> HashMap<StorageTier, TierStats> {
    let mut stats = HashMap::new();

    for tier in [
        StorageTier::Hot,
        StorageTier::Warm,
        StorageTier::Cold,
        StorageTier::Cache,
    ] {
        let tier_datasets: Vec<_> = datasets.iter().filter(|d| d.tier == tier).collect();

        let total_capacity = tier_datasets
            .iter()
            .map(|d| d.used_space + d.available_space)
            .sum();

        let used_capacity = tier_datasets.iter().map(|d| d.used_space).sum();

        let file_count = tier_datasets
            .iter()
            .map(|d| d.file_count.unwrap_or(0))
            .sum();

        stats.insert(
            tier,
            TierStats {
                total_capacity,
                used_capacity,
                file_count,
                active_operations: 0,
            },
        );
    }

    stats
}

/// Manages tiered storage operations
#[derive(Debug)]
/// Manager for Tier operations
pub struct TierManager {
    _config: ZfsConfig,
    _pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    tier_stats: TierStatsMap,
}
#[derive(Debug, Clone, Default)]
/// Tierstats
pub struct TierStats {
    /// Total Capacity
    pub total_capacity: u64,
    /// Used Capacity
    pub used_capacity: u64,
    /// Count of file
    pub file_count: u64,
    /// Active Operations
    pub active_operations: u32,
}
#[derive(Debug, Clone)]
/// Tierstatus
pub struct TierStatus {
    /// Tier
    pub tier: StorageTier,
    /// Health
    pub health: String,
    /// Utilization
    pub utilization: f64,
    /// Stats
    pub stats: TierStats,
}
impl TierManager {
    /// Create a new tier manager
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn new(
        config: &ZfsConfig,
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
    ) -> Result<Self> {
        info!("Initializing tier manager");

        let mut tier_stats = HashMap::new();
        tier_stats.insert(StorageTier::Hot, TierStats::default());
        tier_stats.insert(StorageTier::Warm, TierStats::default());
        tier_stats.insert(StorageTier::Cold, TierStats::default());
        tier_stats.insert(StorageTier::Cache, TierStats::default());

        let manager = Self {
            _config: config.clone(),
            _pool_manager: pool_manager,
            dataset_manager,
            tier_stats: Arc::new(RwLock::new(tier_stats)),
        };

        // Initialize tier statistics
        manager.refresh_tier_stats().await?;

        Ok(manager)
    }

    /// Initialize tier configurations
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn initialize_tiers(&self) -> Result<()> {
        // Initialize all storage tiers
        for tier in [
            StorageTier::Hot,
            StorageTier::Warm,
            StorageTier::Cold,
            StorageTier::Cache,
        ] {
            self.configure_tier_properties(&tier)?;
        }
        Ok(())
    }

    /// Configure Tier Properties
    fn configure_tier_properties(&self, tier: &StorageTier) -> Result<()> {
        info!("Configuring properties for {:?} tier", tier);
        // Real tier configuration would set ZFS properties based on tier type
        // e.g., compression, recordsize, primarycache, etc.
        Ok(())
    }

    /// Get tier status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_tier_status(&self, tier: StorageTier) -> Result<TierStatus> {
        let stats = self.tier_stats.read().await;
        let tier_stats = stats.get(&tier).cloned().unwrap_or_default();

        let utilization = if tier_stats.total_capacity > 0 {
            (tier_stats.used_capacity as f64 / tier_stats.total_capacity as f64) * 100.0
        } else {
            0.0
        };

        let health = if utilization > 90.0 {
            "Critical".to_string()
        } else if utilization > 75.0 {
            "Warning".to_string()
        } else {
            "Healthy".to_string()
        };

        Ok(TierStatus {
            tier,
            health,
            utilization,
            stats: tier_stats,
        })
    }

    /// Graceful shutdown
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down tier manager gracefully");

        // Cancel any active operations
        let mut stats = self.tier_stats.write().await;
        for (tier, tier_stats) in stats.iter_mut() {
            if tier_stats.active_operations > 0 {
                warn!(
                    "Canceling {} active operations for tier {:?}",
                    tier_stats.active_operations, tier
                );
                tier_stats.active_operations = 0;
            }
        }

        info!("Tier manager shutdown complete");
        Ok(())
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn get_all_tier_status(&self) -> Result<Vec<TierStatus>> {
        let mut statuses = Vec::new();

        for tier in [
            StorageTier::Hot,
            StorageTier::Warm,
            StorageTier::Cold,
            StorageTier::Cache,
        ] {
            statuses.push(self.get_tier_status(tier).await?);
        }

        Ok(statuses)
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn refresh_tier_stats(&self) -> Result<()> {
        info!("Refreshing tier statistics");

        let datasets = self.dataset_manager.list_datasets().await?;
        let stats = tier_stats_map_from_datasets(&datasets);

        *self.tier_stats.write().await = stats;
        Ok(())
    }
}

// ========== TEST-ONLY CONSTRUCTORS ==========
// Isolated from production code to maintain clear boundaries

#[cfg(any(test, feature = "dev-stubs"))]
impl TierManager {
    /// Create tier manager for testing
    ///
    /// **TEST-ONLY**: This constructor is only available in test builds.
    /// Production code must use `TierManager::new()` with proper configuration.
    #[must_use]
    pub fn new_for_testing() -> Self {
        let mut tier_stats_inner = HashMap::new();
        tier_stats_inner.insert(StorageTier::Hot, TierStats::default());
        tier_stats_inner.insert(StorageTier::Warm, TierStats::default());
        tier_stats_inner.insert(StorageTier::Cold, TierStats::default());
        let tier_stats = Arc::new(RwLock::new(tier_stats_inner));

        Self {
            _config: ZfsConfig::default(),
            _pool_manager: Arc::new(ZfsPoolManager::new_production(ZfsConfig::default())),
            dataset_manager: Arc::new({
                let config = ZfsConfig::default();
                let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
                ZfsDatasetManager::new(config, pool_manager)
            }),
            tier_stats,
        }
    }

    /// Override tier stats for unit tests (no ZFS).
    pub async fn set_tier_stats_for_test(&self, tier: crate::types::StorageTier, stats: TierStats) {
        self.tier_stats.write().await.insert(tier, stats);
    }

    /// Rebuild tier statistics from an in-memory dataset list (unit tests; no `zfs` subprocess).
    #[cfg(test)]
    pub async fn refresh_tier_stats_from_datasets(&self, datasets: Vec<DatasetInfo>) -> Result<()> {
        let stats = tier_stats_map_from_datasets(&datasets);
        *self.tier_stats.write().await = stats;
        Ok(())
    }
}

#[cfg(test)]
#[expect(
    clippy::float_cmp,
    reason = "tier status tests compare sentinel 0.0 utilization (zero-capacity branch) and epsilon elsewhere"
)]
mod tests {
    use super::*;
    use crate::dataset::DatasetInfo;
    use crate::types::StorageTier;
    use std::collections::HashMap;

    fn sample_dataset(
        name: &str,
        tier: StorageTier,
        used: u64,
        avail: u64,
        files: Option<u64>,
    ) -> DatasetInfo {
        DatasetInfo {
            name: name.to_string(),
            used_space: used,
            available_space: avail,
            file_count: files,
            compression_ratio: None,
            mount_point: "/mnt".into(),
            tier,
            properties: HashMap::new(),
        }
    }

    #[test]
    fn tier_stats_map_from_datasets_empty() {
        let m = tier_stats_map_from_datasets(&[]);
        for t in [
            StorageTier::Hot,
            StorageTier::Warm,
            StorageTier::Cold,
            StorageTier::Cache,
        ] {
            let s = m.get(&t).expect("tier key");
            assert_eq!(s.total_capacity, 0);
            assert_eq!(s.used_capacity, 0);
            assert_eq!(s.file_count, 0);
        }
    }

    #[test]
    fn tier_stats_map_from_datasets_sums_per_tier_and_none_file_count() {
        let datasets = vec![
            sample_dataset("t/a", StorageTier::Hot, 10, 90, None),
            sample_dataset("t/b", StorageTier::Hot, 5, 45, Some(3)),
            sample_dataset("t/c", StorageTier::Warm, 1, 1, Some(10)),
        ];
        let m = tier_stats_map_from_datasets(&datasets);
        let hot = m.get(&StorageTier::Hot).expect("hot");
        assert_eq!(hot.total_capacity, 150);
        assert_eq!(hot.used_capacity, 15);
        assert_eq!(hot.file_count, 3);
        let warm = m.get(&StorageTier::Warm).expect("warm");
        assert_eq!(warm.total_capacity, 2);
        assert_eq!(warm.used_capacity, 1);
        assert_eq!(warm.file_count, 10);
    }

    #[tokio::test]
    async fn refresh_tier_stats_from_datasets_updates_status_without_zfs() {
        let m = TierManager::new_for_testing();
        let datasets = vec![sample_dataset(
            "pool/ds",
            StorageTier::Cold,
            100,
            900,
            Some(42),
        )];
        m.refresh_tier_stats_from_datasets(datasets)
            .await
            .expect("refresh from fixture");
        let cold = m.get_tier_status(StorageTier::Cold).await.expect("cold");
        assert_eq!(cold.stats.total_capacity, 1000);
        assert_eq!(cold.stats.used_capacity, 100);
        assert_eq!(cold.stats.file_count, 42);
    }

    #[tokio::test]
    async fn get_tier_status_uses_default_stats_for_missing_tier_key() {
        let m = TierManager::new_for_testing();
        let s = m.get_tier_status(StorageTier::Cache).await.expect("cache");
        assert_eq!(s.stats.total_capacity, 0);
        assert_eq!(s.stats.used_capacity, 0);
        assert_eq!(s.stats.file_count, 0);
        assert_eq!(s.stats.active_operations, 0);
        assert_eq!(s.utilization, 0.0);
        assert_eq!(s.health, "Healthy");
    }

    #[tokio::test]
    async fn shutdown_warn_path_resets_active_operations() {
        let m = TierManager::new_for_testing();
        m.set_tier_stats_for_test(
            StorageTier::Hot,
            TierStats {
                active_operations: 7,
                ..TierStats::default()
            },
        )
        .await;
        m.shutdown().await.expect("shutdown");
        let s = m.get_tier_status(StorageTier::Hot).await.expect("hot");
        assert_eq!(s.stats.active_operations, 0);
    }

    #[test]
    fn tier_stats_tier_status_debug_format() {
        let st = format!("{:?}", TierStats::default());
        assert!(st.contains("TierStats"));
        let ts = TierStatus {
            tier: StorageTier::Warm,
            health: "OK".into(),
            utilization: 1.0,
            stats: TierStats::default(),
        };
        assert!(format!("{ts:?}").contains("TierStatus"));
    }

    #[tokio::test]
    async fn initialize_tiers_and_shutdown_are_ok() {
        let m = TierManager::new_for_testing();
        m.initialize_tiers().expect("initialize tiers");
        m.shutdown().await.expect("shutdown");
    }

    #[tokio::test]
    async fn tier_status_health_branches() {
        let m = TierManager::new_for_testing();

        m.set_tier_stats_for_test(
            StorageTier::Hot,
            TierStats {
                total_capacity: 100,
                used_capacity: 50,
                ..TierStats::default()
            },
        )
        .await;
        let s = m.get_tier_status(StorageTier::Hot).await.expect("status");
        assert_eq!(s.health, "Healthy");
        assert!((s.utilization - 50.0).abs() < f64::EPSILON);

        m.set_tier_stats_for_test(
            StorageTier::Hot,
            TierStats {
                total_capacity: 100,
                used_capacity: 75,
                ..TierStats::default()
            },
        )
        .await;
        let s = m.get_tier_status(StorageTier::Hot).await.expect("status");
        assert_eq!(s.health, "Healthy", "exactly 75% is not Warning");

        m.set_tier_stats_for_test(
            StorageTier::Hot,
            TierStats {
                total_capacity: 100,
                used_capacity: 76,
                ..TierStats::default()
            },
        )
        .await;
        let s = m.get_tier_status(StorageTier::Hot).await.expect("status");
        assert_eq!(s.health, "Warning");

        m.set_tier_stats_for_test(
            StorageTier::Hot,
            TierStats {
                total_capacity: 100,
                used_capacity: 90,
                ..TierStats::default()
            },
        )
        .await;
        let s = m.get_tier_status(StorageTier::Hot).await.expect("status");
        assert_eq!(s.health, "Warning", "exactly 90% is not Critical");

        m.set_tier_stats_for_test(
            StorageTier::Hot,
            TierStats {
                total_capacity: 100,
                used_capacity: 91,
                ..TierStats::default()
            },
        )
        .await;
        let s = m.get_tier_status(StorageTier::Hot).await.expect("status");
        assert_eq!(s.health, "Critical");

        m.set_tier_stats_for_test(
            StorageTier::Warm,
            TierStats {
                total_capacity: 100,
                used_capacity: 80,
                ..TierStats::default()
            },
        )
        .await;
        let s = m.get_tier_status(StorageTier::Warm).await.expect("status");
        assert_eq!(s.health, "Warning");

        m.set_tier_stats_for_test(
            StorageTier::Cold,
            TierStats {
                total_capacity: 100,
                used_capacity: 95,
                ..TierStats::default()
            },
        )
        .await;
        let s = m.get_tier_status(StorageTier::Cold).await.expect("status");
        assert_eq!(s.health, "Critical");

        m.set_tier_stats_for_test(
            StorageTier::Cache,
            TierStats {
                total_capacity: 0,
                used_capacity: 0,
                ..TierStats::default()
            },
        )
        .await;
        let s = m.get_tier_status(StorageTier::Cache).await.expect("status");
        assert_eq!(s.utilization, 0.0);
        assert_eq!(s.health, "Healthy");
    }

    #[tokio::test]
    async fn get_all_tier_status_returns_four_tiers() {
        let m = TierManager::new_for_testing();
        let v = m.get_all_tier_status().await.expect("all");
        assert_eq!(v.len(), 4);
    }

    #[tokio::test]
    #[ignore = "requires ZFS kernel module — calls `zfs list` via dataset_manager"]
    async fn refresh_tier_stats_updates_internal_map() {
        let m = TierManager::new_for_testing();
        m.refresh_tier_stats().await.expect("refresh");
        let statuses = m.get_all_tier_status().await.expect("all");
        assert_eq!(statuses.len(), 4);
        for s in statuses {
            assert!(matches!(
                s.tier,
                StorageTier::Hot | StorageTier::Warm | StorageTier::Cold | StorageTier::Cache
            ));
        }
    }
}
