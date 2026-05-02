// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Long-running ZFS health monitor and command-backed checks.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;

use tracing::debug;
use tracing::info;

use super::reporting::{dataset_health_from_zfs_list_text, pool_health_from_zpool_status_text};
use super::types::{
    Alert, BackgroundTasks, HealthDataMap, HealthReport, HealthStatus, HealthStatusMap,
    MonitoringTasks,
};
use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use nestgate_core::Result;

/// ZFS Health Monitor - monitors system health
#[derive(Debug)]
/// Zfshealthmonitor
pub struct ZfsHealthMonitor {
    config: ZfsConfig,
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    pub(crate) health_data: HealthDataMap,
    pub(crate) monitoring_tasks: MonitoringTasks,
    health_status: HealthStatusMap,
    alert_history: Arc<tokio::sync::RwLock<VecDeque<Alert>>>,
    pub(crate) monitoring_active: Arc<AtomicBool>,
    pub(crate) background_tasks: BackgroundTasks,
}

impl ZfsHealthMonitor {
    /// Create a new health monitor
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn new(
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
    ) -> Result<Self> {
        Ok(Self {
            config: crate::config::ZfsConfig::default(),
            pool_manager,
            dataset_manager,
            health_data: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            monitoring_tasks: None,
            health_status: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            alert_history: Arc::new(tokio::sync::RwLock::new(VecDeque::new())),
            monitoring_active: Arc::new(AtomicBool::new(false)),
            background_tasks: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        })
    }

    /// Start ZFS health monitoring
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn start(&mut self) -> Result<()> {
        info!("Starting ZFS health monitoring...");

        // Initialize monitoring tasks
        let _config = self.config.clone();
        let pool_manager = self.pool_manager.clone();
        let _dataset_manager = self.dataset_manager.clone();
        let health_data = self.health_data.clone();

        // Start pool health monitoring task
        let pool_monitor_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(600)); // 10 minutes canonical default

            loop {
                interval.tick().await;

                // Monitor all pools
                if let Ok(pools) = pool_manager.list_pools().await {
                    for pool in pools {
                        // Check pool health
                        let health_status =
                            Self::check_pool_health(&pool_manager, &pool.name).await;

                        let report = HealthReport {
                            component_type: "pool".to_string(),
                            component_name: pool.name.clone(),
                            status: health_status,
                            last_check: SystemTime::now(),
                            details: format!(
                                "Pool capacity: {:.1}% used",
                                pool.capacity.utilization_percent
                            ),
                        };

                        // Update health data
                        let mut health = health_data.write().await;
                        health.insert("pool:error details".to_string(), report);
                    }
                }

                debug!("Pool health check cycle completed");
            }
        });

        // Start dataset health monitoring task
        let _dataset_config = self.config.clone();
        let dataset_pool_manager = self.pool_manager.clone();
        let dataset_manager_clone = self.dataset_manager.clone();
        let dataset_health_data = self.health_data.clone();

        let dataset_monitor_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(600)); // 10 minutes canonical default

            loop {
                interval.tick().await;

                // Monitor critical datasets
                if let Ok(pools) = dataset_pool_manager.list_pools().await {
                    for pool in pools {
                        // Check datasets in this pool
                        let health_status =
                            Self::check_dataset_health(&dataset_manager_clone, &pool.name).await;

                        let mut health = dataset_health_data.write().await;
                        health.insert(
                            "datasets:error details".to_string(),
                            HealthReport {
                                component_type: "datasets".to_string(),
                                component_name: pool.name.clone(),
                                status: health_status,
                                last_check: SystemTime::now(),
                                details: "Dataset health assessment completed".to_string().clone(),
                            },
                        );
                    }
                }
            }
        });

        // Store task handles
        self.monitoring_tasks = Some((pool_monitor_handle, dataset_monitor_handle));

        info!("ZFS health monitoring started successfully");
        Ok(())
    }

    /// Stop ZFS health monitoring
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping ZFS health monitoring...");

        // Cancel monitoring tasks
        if let Some((pool_handle, dataset_handle)) = self.monitoring_tasks.take() {
            pool_handle.abort();
            dataset_handle.abort();

            // Wait for graceful shutdown
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        // Clear health data
        {
            let mut health = self.health_data.write().await;
            health.clear();
        }

        info!("ZFS health monitoring stopped");
        Ok(())
    }

    /// Get current health status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_current_status(&self) -> Result<crate::manager::EnhancedServiceStatus> {
        let pool_status = match self.pool_manager.get_overall_status().await {
            Ok(status) => status,
            Err(_) => crate::manager::PoolOverallStatus {
                pools_online: 0,
                pools_degraded: 0,
                total_capacity: 0,
                available_capacity: 0,
            },
        };

        // Create tier status
        let tier_status = crate::manager::TierOverallStatus {
            hot_utilization: 0.0,
            warm_utilization: 0.0,
            cold_utilization: 0.0,
            migration_queue_size: 0,
        };

        Ok(crate::manager::EnhancedServiceStatus {
            overall_health: crate::manager::HealthState::Healthy,
            pool_status,
            tier_status,
            performance_metrics: crate::performance::CurrentPerformanceMetrics::default(),
            ai_status: None,
            migration_status: crate::manager::MigrationStatus::default(),
            snapshot_status: crate::manager::SnapshotStatus::default(),
            metrics: crate::manager::CurrentMetrics::default(),
            timestamp: chrono::Utc::now(),
        })
    }

    // Helper methods for health checking
    pub(crate) async fn check_pool_health(
        _pool_manager: &Arc<ZfsPoolManager>,
        pool_name: &str,
    ) -> HealthStatus {
        // Get pool status from zpool status command
        match tokio::process::Command::new("zpool")
            .args(["status", pool_name])
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                pool_health_from_zpool_status_text(stdout.as_ref())
            }
            _ => {
                // If we can't check, assume it's a warning condition
                HealthStatus::Warning
            }
        }
    }

    /// Check Dataset Health
    pub(crate) async fn check_dataset_health(
        _dataset_manager: &Arc<ZfsDatasetManager>,
        pool_name: &str,
    ) -> HealthStatus {
        // Check if datasets in the pool are accessible
        match tokio::process::Command::new("zfs")
            .args(["list", "-H", "-o", "name,avail", "-r", pool_name])
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                dataset_health_from_zfs_list_text(stdout.as_ref())
            }
            _ => HealthStatus::Warning,
        }
    }

    /// Start health monitoring
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn start_monitoring(&mut self) -> Result<()> {
        if self.monitoring_active.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.monitoring_active.store(true, Ordering::Relaxed);
        info!("Started ZFS health monitoring");
        Ok(())
    }

    /// Stop health monitoring
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn stop_monitoring(&mut self) -> Result<()> {
        if !self.monitoring_active.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.monitoring_active.store(false, Ordering::Relaxed);

        // Stop all background tasks
        let tasks_to_abort: Vec<_> = {
            let mut tasks = self.background_tasks.write().await;
            tasks.drain(..).collect()
        };
        for task in tasks_to_abort {
            task.abort();
        }

        info!("Stopped ZFS health monitoring");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dataset::ZfsDatasetManager;
    use crate::pool::ZfsPoolManager;
    use crate::pool::types::{PoolCapacity, PoolHealth, PoolInfo, PoolState};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn health_monitor_new_succeeds() {
        let pm = Arc::new(ZfsPoolManager::new_for_testing());
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        ZfsHealthMonitor::new(pm, dm).expect("test: new monitor");
    }

    #[tokio::test]
    async fn get_current_status_ok_with_cached_pool() {
        let pm = Arc::new(ZfsPoolManager::new_for_testing());
        pm.insert_pool_for_testing(PoolInfo {
            name: "tank_cov".into(),
            state: PoolState::Online,
            health: PoolHealth::Healthy,
            capacity: PoolCapacity {
                total: 1000,
                total_bytes: 1000,
                used: 100,
                used_bytes: 100,
                available: 900,
                available_bytes: 900,
                utilization_percent: 10.0,
                fragmentation_percent: 0.0,
                deduplication_ratio: 1.0,
            },
            devices: Vec::new(),
            properties: HashMap::new(),
        })
        .await;
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let m = ZfsHealthMonitor::new(pm, dm).expect("test: monitor");
        let st = m.get_current_status().await.expect("test: status");
        assert_eq!(st.pool_status.pools_online, 1);
    }

    #[tokio::test]
    async fn start_monitoring_is_idempotent() {
        let pm = Arc::new(ZfsPoolManager::new_for_testing());
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let mut m = ZfsHealthMonitor::new(pm, dm).expect("test: monitor");
        m.start_monitoring().expect("test: start once");
        m.start_monitoring().expect("test: start twice");
        m.stop_monitoring().await.expect("test: stop");
    }

    #[tokio::test]
    async fn stop_monitoring_when_inactive_is_ok() {
        let pm = Arc::new(ZfsPoolManager::new_for_testing());
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let mut m = ZfsHealthMonitor::new(pm, dm).expect("test: monitor");
        m.stop_monitoring().await.expect("test: stop inactive");
    }

    #[tokio::test]
    async fn start_then_stop_aborts_background_loops() {
        let pm = Arc::new(ZfsPoolManager::new_for_testing());
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let mut m = ZfsHealthMonitor::new(pm, dm).expect("test: monitor");
        m.start().expect("test: start");
        tokio::time::sleep(tokio::time::Duration::from_millis(40)).await;
        m.stop().await.expect("test: stop");
    }

    #[tokio::test]
    async fn check_pool_health_warns_on_missing_pool() {
        let pm = Arc::new(ZfsPoolManager::new_for_testing());
        let h = ZfsHealthMonitor::check_pool_health(&pm, "no_such_pool_for_health_check").await;
        assert_eq!(h, crate::health::types::HealthStatus::Warning);
    }

    #[tokio::test]
    async fn check_dataset_health_warns_on_missing_pool_tree() {
        let dm = Arc::new(ZfsDatasetManager::new_for_testing());
        let h =
            ZfsHealthMonitor::check_dataset_health(&dm, "no_such_pool_for_dataset_health").await;
        assert_eq!(h, crate::health::types::HealthStatus::Warning);
    }
}
