//
// Contains all performance-related operations including analytics collection,
// storage optimization, and tier optimization based on performance metrics.

use super::types::{OptimizationResult, PerformanceAnalytics};
use nestgate_core::error::conversions::create_zfs_error;
use nestgate_core::error::domain_errors::ZfsOperation;
use crate::types::StorageTier;
use nestgate_core::Result;
use std::time::SystemTime;
// Removed unused tracing import

use super::ZfsManager;
use tracing::info;

impl ZfsManager {
    /// Get performance analytics
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
                .get_tier_performance(tier)
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
    pub async fn trigger_optimization(&self) -> Result<OptimizationResult> {
        info!("🚀 Triggering comprehensive ZFS optimization using heuristic analysis");

        let mut results = Vec::new();

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
            let tier_recommendations = self.optimize_tiers_heuristically(&analytics).await?;
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
        let pools = self.pool_manager.list_pools().await.map_err(|e| {
            create_zfs_error(
                format!("Failed to list pools: {e}"),
                ZfsOperation::SystemCheck
            )
        })?;

        for pool in &pools {
            let status = self
                .pool_manager
                .get_pool_status(&pool.name)
                .await
                .map_err(|e| {
                    create_zfs_error(
                        format!("Failed to get pool status: {e}"),
                        ZfsOperation::PoolCreate
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
                recommendations.push(format!("Pool {} is full - consider expansion", pool.name));
            }
        }

        recommendations.push("Storage optimization completed using heuristic analysis".to_string());
        Ok(recommendations)
    }

    /// Heuristic-based tier optimization
    async fn optimize_tiers_heuristically(
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
                recommendations.push(format!("Tier {:?} showing high latency (read: {:.1}ms, write: {:.1}ms) - consider optimization",
                                           tier, perf_data.current_metrics.avg_read_latency_ms, perf_data.current_metrics.avg_write_latency_ms));
            }
        }

        Ok(recommendations)
    }

    /// Graceful shutdown of enhanced ZFS manager
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down Enhanced ZFS Manager");

        // The actual shutdown is handled by the stop method
        // This method is for external cleanup if needed

        info!("Enhanced ZFS Manager shutdown complete");
        Ok(())
    }
}
