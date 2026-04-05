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

use crate::performance::types::TierStatsMap;
use crate::{
    config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager, types::StorageTier,
};
use nestgate_core::Result;
use tracing::info;
use tracing::warn;

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
                    active_operations: 0, // Will be updated by migration engine
                },
            );
        }

        *self.tier_stats.write().await = stats;
        Ok(())
    }
}

// ========== TEST-ONLY CONSTRUCTORS ==========
// Isolated from production code to maintain clear boundaries

#[cfg(test)]
impl TierManager {
    /// Create tier manager for testing
    ///
    /// **TEST-ONLY**: This constructor is only available in test builds.
    /// Production code must use `TierManager::new()` with proper configuration.
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::StorageTier;

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
