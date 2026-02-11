//
// This module will be fully implemented in Week 2

//! Health module

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

// Removed unused tracing import

use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use nestgate_core::Result;

use tracing::debug;
use tracing::info;

/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Warning
    Warning,
    /// Critical
    Critical,
    /// Unknown
    Unknown,
}
/// Health report for a component
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthreport
pub struct HealthReport {
    /// Component Type
    pub component_type: String,
    /// Component name
    pub component_name: String,
    /// Status
    pub status: HealthStatus,
    /// Last Check
    pub last_check: SystemTime,
    /// Details
    pub details: String,
}
/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertlevel
pub enum AlertLevel {
    /// Info
    Info,
    /// Warning
    Warning,
    /// Critical
    Critical,
}
/// Alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alert
pub struct Alert {
    /// Unique identifier
    pub id: String,
    /// Level
    pub level: AlertLevel,
    /// Message
    pub message: String,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Component
    pub component: String,
}
// ==================== SECTION ====================

/// Type alias for health data storage
pub type HealthDataMap = Arc<tokio::sync::RwLock<HashMap<String, HealthReport>>>;
/// Type alias for monitoring task handles
pub type MonitoringTasks = Option<(tokio::task::JoinHandle<()>, tokio::task::JoinHandle<()>)>;
/// Type alias for health status storage
pub type HealthStatusMap = Arc<tokio::sync::RwLock<HashMap<String, HealthStatus>>>;
/// Type alias for background task storage
pub type BackgroundTasks = Arc<tokio::sync::RwLock<Vec<tokio::task::JoinHandle<()>>>>;
/// ZFS Health Monitor - monitors system health
#[derive(Debug)]
#[allow(dead_code)] // Fields used in comprehensive health monitoring system
/// Zfshealthmonitor
pub struct ZfsHealthMonitor {
    config: ZfsConfig,
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    health_data: HealthDataMap,
    monitoring_tasks: MonitoringTasks,
    health_status: HealthStatusMap,
    alert_history: Arc<tokio::sync::RwLock<VecDeque<Alert>>>,
    monitoring_active: Arc<AtomicBool>,
    background_tasks: BackgroundTasks,
}
impl HealthStatus {
    /// Returns `true` if the health status is critical.
    #[must_use]
    pub fn is_critical(&self) -> bool {
        matches!(self, Self::Critical)
    }

    /// Returns `true` if the health status is healthy.
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }
}

impl std::fmt::Display for HealthStatus {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Warning => write!(f, "Warning"),
            HealthStatus::Critical => write!(f, "Critical"),
            HealthStatus::Unknown => write!(f, "Unknown"),
        }
    }
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
            config: Default::default(),
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
    pub async fn start(&mut self) -> Result<()> {
        info!("🏥 Starting ZFS health monitoring...");

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

                        // Update health data
                        let mut health = health_data.write().await;
                        health.insert(
                            "pool:error details".to_string(),
                            HealthReport {
                                component_type: "pool".to_string(),
                                component_name: pool.name.clone(),
                                status: health_status,
                                last_check: SystemTime::now(),
                                details: format!(
                                    "Pool capacity: {:.1}% used",
                                    pool.capacity.utilization_percent
                                ),
                            },
                        );
                    }
                }

                debug!("🔍 Pool health check cycle completed");
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
                                details: "Dataset health assessment completed"
                                    .to_string()
                                    .to_string(),
                            },
                        );
                    }
                }
            }
        });

        // Store task handles
        self.monitoring_tasks = Some((pool_monitor_handle, dataset_monitor_handle));

        info!("✅ ZFS health monitoring started successfully");
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
        info!("🛑 Stopping ZFS health monitoring...");

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

        info!("✅ ZFS health monitoring stopped");
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
            performance_metrics: Default::default(),
            ai_status: None,
            migration_status: crate::manager::MigrationStatus::default(),
            snapshot_status: crate::manager::SnapshotStatus::default(),
            metrics: crate::manager::CurrentMetrics::default(),
            timestamp: chrono::Utc::now(),
        })
    }

    // Helper methods for health checking
    async fn check_pool_health(
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

                if stdout.contains("ONLINE") && !stdout.contains("errors:") {
                    HealthStatus::Healthy
                } else if stdout.contains("DEGRADED")
                    || stdout.contains("FAULTED")
                    || stdout.contains("UNAVAIL")
                {
                    HealthStatus::Critical
                } else {
                    HealthStatus::Warning
                }
            }
            _ => {
                // If we can't check, assume it's a warning condition
                HealthStatus::Warning
            }
        }
    }

    /// Check Dataset Health
    async fn check_dataset_health(
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

                // Check if all datasets have reasonable available space
                let mut total_datasets = 0;
                let mut low_space_datasets = 0;

                for line in stdout.lines() {
                    let fields: Vec<&str> = line.split('\t').collect();
                    if fields.len() >= 2 {
                        total_datasets += 1;

                        // Parse available space and check if it's critically low
                        if let Ok(avail_bytes) = fields[1].parse::<u64>() {
                            if avail_bytes < 1024 * 1024 * 1024 {
                                // Less than 1GB available
                                low_space_datasets += 1;
                            }
                        }
                    }
                }

                if low_space_datasets == 0 {
                    HealthStatus::Healthy
                } else if low_space_datasets < total_datasets / 2 {
                    HealthStatus::Warning
                } else {
                    HealthStatus::Critical
                }
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
        let mut tasks = self.background_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }

        info!("Stopped ZFS health monitoring");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};
    use std::sync::Arc;

    #[test]
    fn test_health_status_is_critical_variants() {
        assert!(HealthStatus::Critical.is_critical());
        assert!(!HealthStatus::Healthy.is_critical());
    }

    #[test]
    fn test_health_status_is_healthy_variants() {
        assert!(HealthStatus::Healthy.is_healthy());
        assert!(!HealthStatus::Critical.is_healthy());
    }

    #[test]
    fn test_health_status_display_all() {
        assert_eq!(format!("{}", HealthStatus::Healthy), "Healthy");
        assert_eq!(format!("{}", HealthStatus::Warning), "Warning");
        assert_eq!(format!("{}", HealthStatus::Critical), "Critical");
        assert_eq!(format!("{}", HealthStatus::Unknown), "Unknown");
    }

    #[test]
    fn test_health_report_serialization() {
        let report = HealthReport {
            component_type: "pool".to_string(),
            component_name: "tank".to_string(),
            status: HealthStatus::Healthy,
            last_check: SystemTime::now(),
            details: "OK".to_string(),
        };
        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("tank"));
    }

    #[test]
    fn test_alert_creation() {
        let alert = Alert {
            id: "a1".to_string(),
            level: AlertLevel::Info,
            message: "msg".to_string(),
            timestamp: SystemTime::now(),
            component: "pool".to_string(),
        };
        assert_eq!(alert.id, "a1");
        assert!(matches!(alert.level, AlertLevel::Info));
    }

    #[test]
    fn test_health_monitor_new() {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
        let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
        let result = ZfsHealthMonitor::new(pool_manager, dataset_manager);
        assert!(result.is_ok());
    }

    #[test]
    fn test_start_monitoring_idempotent() {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
        let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager.clone()));
        let mut monitor = ZfsHealthMonitor::new(pool_manager, dataset_manager).unwrap();
        let r1 = monitor.start_monitoring();
        assert!(r1.is_ok());
        let r2 = monitor.start_monitoring();
        assert!(r2.is_ok());
    }
}
