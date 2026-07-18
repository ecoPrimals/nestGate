// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// Contains all health-related operations including comprehensive service status,
// ZFS health checking, and system monitoring.

//! Health module

use super::types::{
    AiIntegrationStatus, CurrentMetrics, EnhancedServiceStatus, HealthState, MigrationStatus,
    PoolOverallStatus, SnapshotStatus, TierOverallStatus,
};
use crate::command::ZfsOperations;
use crate::error::{ZfsOperation, create_zfs_error};
use nestgate_core::Result;
use std::time::SystemTime;
// Removed unused tracing import

use super::ZfsManager;
use tracing::debug;
use tracing::info;

impl ZfsManager {
    /// Get comprehensive service status including AI and performance metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_service_status(&self) -> Result<EnhancedServiceStatus> {
        debug!("Getting comprehensive service status");

        // Get health status from health monitor - using a simple default for now
        // Get real health state from ZFS
        let health_state = self.get_real_health_state().await?;

        // Get pool status from pool manager
        let pools = self.pool_manager.list_pools().await?;
        let pool_status = PoolOverallStatus {
            pools_online: pools
                .iter()
                .filter(|p| matches!(p.health, crate::pool::PoolHealth::Healthy))
                .count(),
            pools_degraded: pools
                .iter()
                .filter(|p| {
                    matches!(
                        p.health,
                        crate::pool::PoolHealth::Warning | crate::pool::PoolHealth::Critical
                    )
                })
                .count(),
            total_capacity: pools.iter().map(|p| p.capacity.total_bytes).sum(),
            available_capacity: pools.iter().map(|p| p.capacity.available_bytes).sum(),
        };

        let tier_status = TierOverallStatus {
            hot_utilization: self.get_real_tier_utilization("hot").await.unwrap_or(0.0),
            warm_utilization: self.get_real_tier_utilization("warm").await.unwrap_or(0.0),
            cold_utilization: self.get_real_tier_utilization("cold").await.unwrap_or(0.0),
            migration_queue_size: 0,
        };

        // Get performance metrics
        let perf_metrics = self
            .performance_monitor
            .read()
            .await
            .get_current_metrics()
            .await;

        // Get metrics from metrics collector
        let metrics_snapshot = self.metrics.get_current_metrics();
        let metrics = CurrentMetrics {
            operations_per_second: metrics_snapshot.operations_per_second,
            throughput_bytes_per_second: metrics_snapshot.throughput_bytes_per_second,
            average_latency_ms: metrics_snapshot.average_latency_ms,
            error_rate: metrics_snapshot.error_rate,
        };

        // Get AI integration status
        let ai_status = Some(AiIntegrationStatus {
            enabled: false, // AI integration has been sunset
            models_deployed: 0,
            optimization_active: false,
            last_optimization: SystemTime::now(),
            prediction_accuracy: 0.0,
        });

        let migration_status = MigrationStatus {
            active_jobs: self.get_active_migration_jobs().unwrap_or(0),
            queued_jobs: 0,
            completed_jobs: 0,
            failed_jobs: 0,
            total_bytes_migrated: 0,
        };

        let snapshot_status = SnapshotStatus {
            total_snapshots: u64::from(self.get_total_snapshots().await.unwrap_or(0)),
            active_policies: 0,
            pending_operations: 0,
            recent_failures: 0,
        };

        Ok(EnhancedServiceStatus {
            overall_health: health_state,
            pool_status,
            tier_status,
            performance_metrics: perf_metrics,
            ai_status,
            migration_status,
            snapshot_status,
            metrics,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Get ZFS health status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_zfs_health(&self) -> Result<EnhancedServiceStatus> {
        self.get_service_status().await
    }

    /// Get real health state from ZFS pools
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_real_health_state(&self) -> Result<HealthState> {
        use crate::command::ZfsOperations;

        let ops = ZfsOperations::new();
        let pools = ops
            .list_pools()
            .await
            .map_err(|e| create_zfs_error(e.to_string(), ZfsOperation::Configuration))?;

        // Check if any pools are unhealthy
        for pool in pools {
            match pool.health.as_str() {
                "ONLINE" | "HEALTHY" => {}
                "DEGRADED" => return Ok(HealthState::Warning),
                _ => return Ok(HealthState::Critical),
            }
        }

        Ok(HealthState::Healthy)
    }

    /// Get real tier utilization from ZFS
    async fn get_real_tier_utilization(&self, tier: &str) -> Result<f64> {
        let ops = ZfsOperations::new();
        let datasets = ops
            .list_datasets(None)
            .await
            .map_err(|e| create_zfs_error(e.to_string(), ZfsOperation::Configuration))?;

        let tier_datasets: Vec<_> = datasets.iter().filter(|d| d.name.contains(tier)).collect();

        if tier_datasets.is_empty() {
            return Ok(0.0);
        }

        let mut total_used: u64 = 0;
        let mut total_space: u64 = 0;
        for ds in &tier_datasets {
            let used = crate::pool_helpers::parse_size_with_units(&ds.used).unwrap_or(0);
            let avail = crate::pool_helpers::parse_size_with_units(&ds.available).unwrap_or(0);
            total_used += used;
            total_space += used + avail;
        }

        if total_space == 0 {
            return Ok(0.0);
        }

        #[expect(
            clippy::cast_precision_loss,
            reason = "byte counts fit f64 for ratio computation"
        )]
        let utilization = total_used as f64 / total_space as f64;
        Ok(utilization)
    }

    /// Get active migration jobs count.
    ///
    /// Returns zero until migration engine integration is wired.
    const fn get_active_migration_jobs(&self) -> Result<u32> {
        Ok(0)
    }

    /// Get total snapshots count
    async fn get_total_snapshots(&self) -> Result<u32> {
        let ops = ZfsOperations::new();
        let snapshots = ops
            .list_snapshots(None)
            .await
            .map_err(|e| create_zfs_error(e.to_string(), ZfsOperation::Configuration))?;

        Ok(u32::try_from(snapshots.len()).unwrap_or(u32::MAX))
    }

    /// Initialize ZFS system
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn initialize_system(&self) -> Result<()> {
        // Removed unused tracing import

        info!("Initializing ZFS system");

        // Verify ZFS is available
        if !crate::native::is_zfs_available().await {
            return Err(create_zfs_error(
                String::from("ZFS is not available on this system"),
                ZfsOperation::SystemCheck,
            ));
        }

        // Start metrics collection
        // No longer needed - metrics are always collecting

        info!("ZFS system initialized successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::manager::ZfsManager;

    #[tokio::test]
    async fn get_service_status_matches_zfs_availability() {
        let m = ZfsManager::mock();
        let out = m.get_service_status().await;
        if crate::native::is_zfs_available().await {
            assert!(out.is_ok());
        } else {
            assert!(out.is_err());
        }
    }

    #[tokio::test]
    async fn get_zfs_health_matches_service_status() {
        let m = ZfsManager::mock();
        let zfs_present = crate::native::is_zfs_available().await;
        let health = m.get_zfs_health().await;
        if zfs_present {
            assert!(health.is_ok(), "expected Ok when ZFS is available");
        } else {
            assert!(health.is_err(), "expected Err when ZFS is unavailable");
        }
    }

    #[tokio::test]
    async fn initialize_system_errors_when_zfs_not_available() {
        let m = ZfsManager::mock();
        let r = m.initialize_system().await;
        if crate::native::is_zfs_available().await {
            assert!(r.is_ok());
        } else {
            assert!(r.is_err());
        }
    }
}
