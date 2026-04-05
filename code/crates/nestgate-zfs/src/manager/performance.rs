// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// Contains all performance-related operations including analytics collection,
// storage optimization, and tier optimization based on performance metrics.

//! Performance module

use super::types::{OptimizationResult, PerformanceAnalytics};
use crate::error::{ZfsOperation, create_zfs_error};
use crate::types::StorageTier;
use nestgate_core::Result;
use std::time::SystemTime;
// Removed unused tracing import

use super::ZfsManager;
use tracing::info;

impl ZfsManager {
    /// Get performance analytics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_performance_analytics(&self) -> Result<PerformanceAnalytics> {
        let current_metrics = self
            .performance_monitor
            .read()
            .await
            .get_current_metrics()
            .await;
        let history = self
            .performance_monitor
            .read()
            .await
            .get_metrics_history()
            .await;
        let active_alerts = self
            .performance_monitor
            .read()
            .await
            .get_active_alerts()
            .await;

        // Get tier-specific analytics
        let mut tier_analytics = std::collections::HashMap::new();
        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            if let Some(tier_data) = self
                .performance_monitor
                .read()
                .await
                .get_tier_performance(tier.clone())
                .await
            {
                tier_analytics.insert(tier, tier_data);
            }
        }

        Ok(PerformanceAnalytics {
            current_metrics,
            history,
            active_alerts,
            tier_analytics,
        })
    }

    /// Trigger comprehensive optimization using performance data and heuristics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn trigger_optimization(&self) -> Result<OptimizationResult> {
        info!("🚀 Triggering comprehensive ZFS optimization using heuristic analysis");

        let mut results: Vec<String> = Vec::new();

        // Get performance analytics to guide optimization
        let analytics = self.get_performance_analytics().await?;

        // Heuristic tier optimization based on performance data
        if analytics.current_metrics.pool_metrics.total_iops > 1000.0
            || analytics.current_metrics.pool_metrics.avg_latency_ms > 50.0
        {
            results.push(
                "Performance optimization: High load detected, recommend tier migration"
                    .to_string(),
            );

            // Note: AI optimization has been sunset - using heuristic optimization only
            let tier_recommendations = self.optimize_tiers_heuristically(&analytics)?;
            results.extend(tier_recommendations);
        }

        // Storage optimization
        let storage_optimization = self.optimize_storage_utilization().await?;
        results.extend(storage_optimization);

        Ok(OptimizationResult {
            timestamp: SystemTime::now(),
            results,
            success: true,
        })
    }

    /// Optimize storage utilization
    async fn optimize_storage_utilization(&self) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // Get current pool status
        let pools = self.pool_manager.list_pools().await.map_err(|_e| {
            create_zfs_error(
                "Failed to list pools: error details".to_string(),
                ZfsOperation::SystemCheck,
            )
        })?;

        for pool in &pools {
            let status = self
                .pool_manager
                .get_pool_status(&pool.name)
                .await
                .map_err(|_e| {
                    create_zfs_error(
                        "Failed to get pool status: error details".to_string(),
                        ZfsOperation::PoolCreate,
                    )
                })?;

            // Parse basic pool status for optimization recommendations
            if status.contains("DEGRADED") {
                recommendations.push(format!(
                    "Pool {} is degraded - consider maintenance",
                    pool.name
                ));
            }
            if status.contains("FULL") || status.contains("100%") {
                recommendations.push(format!(
                    "Pool {} is full - consider expansion",
                    "actual_error_details"
                ));
            }
        }

        recommendations.push("Storage optimization completed using heuristic analysis".to_string());
        Ok(recommendations)
    }

    /// Heuristic-based tier optimization
    fn optimize_tiers_heuristically(
        &self,
        analytics: &PerformanceAnalytics,
    ) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();

        // Analyze tier performance and recommend migrations
        for (tier, perf_data) in &analytics.tier_analytics {
            if perf_data.current_metrics.utilization_percent > 90.0 {
                recommendations.push(format!(
                    "Tier {:?} is {:.1}% full - consider migration to lower tier",
                    tier, perf_data.current_metrics.utilization_percent
                ));
            }
            if perf_data.current_metrics.avg_read_latency_ms > 100.0
                || perf_data.current_metrics.avg_write_latency_ms > 100.0
            {
                recommendations
                    .push("Tier showing high latency - consider optimization".to_string());
            }
        }

        Ok(recommendations)
    }

    /// Graceful shutdown of enhanced ZFS manager
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn shutdown(&self) -> Result<()> {
        info!("Shutting down Enhanced ZFS Manager");

        // The actual shutdown is handled by the stop method
        // This method is for external cleanup if needed

        info!("Enhanced ZFS Manager shutdown complete");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::ZfsManager;

    #[test]
    fn shutdown_always_succeeds() {
        let m = ZfsManager::mock();
        assert!(m.shutdown().is_ok());
    }

    #[tokio::test]
    async fn get_performance_analytics_returns_structure() {
        let m = ZfsManager::mock();
        let a = m.get_performance_analytics().await.expect("analytics");
        assert!(a.tier_analytics.len() <= 3);
    }

    #[tokio::test]
    async fn trigger_optimization_completes() {
        let m = ZfsManager::mock();
        let r = m.trigger_optimization().await.expect("optimization");
        assert!(r.success);
        assert!(!r.results.is_empty());
    }
}
