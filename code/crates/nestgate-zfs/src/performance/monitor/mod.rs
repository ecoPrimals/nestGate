/// Split from monolithic monitor.rs for maintainability and 2000-line compliance
/// Core monitoring implementation with specialized metrics, analysis, and reporting modules
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{error, info};

use crate::types::StorageTier;
use crate::{ZfsDatasetManager, ZfsPoolManager};
use nestgate_core::Result as CoreResult;

use super::types::*;

// Re-export specialized modules
pub mod analysis;
pub mod metrics;
pub mod real_metrics;
pub mod reporting;

impl ZfsPerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(pool_manager: Arc<ZfsPoolManager>, dataset_manager: Arc<ZfsDatasetManager>) -> Self {
        Self {
            // config field removed - using shared ZfsConfig instead
            pool_manager,
            dataset_manager,
            current_metrics: Arc::new(RwLock::new(CurrentPerformanceMetrics::default())),
            metrics_history: Arc::new(RwLock::new(VecDeque::new())),
            tier_metrics: Arc::new(RwLock::new(HashMap::new())),
            alert_conditions: Arc::new(RwLock::new(Vec::new())),
            active_alerts: Arc::new(RwLock::new(Vec::new())),
            collection_task: None,
            analysis_task: None,
            alert_task: None,
            alert_sender: None,
        }
    }

    /// Start performance monitoring
    pub async fn start(&mut self) -> CoreResult<()> {
        info!("Starting ZFS performance monitoring");

        // Load default alert conditions
        self.load_default_alert_conditions().await?;

        // Initialize tier targets
        self.initialize_tier_targets().await?;

        // Start background tasks
        self.start_collection_task().await?;
        self.start_analysis_task().await?;

        // Always enable alerting by default since config was removed
        self.start_alert_task().await?;
        Ok(())
    }

    /// Stop performance monitoring
    pub async fn stop(&mut self) -> CoreResult<()> {
        info!("Stopping ZFS performance monitoring");

        // Stop all background tasks
        if let Some(task) = self.collection_task.take() {
            task.abort();
        }
        if let Some(task) = self.analysis_task.take() {
            task.abort();
        }
        if let Some(task) = self.alert_task.take() {
            task.abort();
        }
        Ok(())
    }

    /// Load default alert conditions
    async fn load_default_alert_conditions(&self) -> CoreResult<()> {
        let mut conditions = self.alert_conditions.write().await;

        // High latency alert
        conditions.push(AlertCondition {
            id: "high-latency".to_string(),
            name: "High Latency".to_string(),
            description: "Average latency exceeds threshold".to_string(),
            metric: AlertMetric::Latency,
            operator: AlertOperator::GreaterThan,
            threshold: 100.0, // 100ms
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_LATENCY_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes default
            ),
            severity: AlertSeverity::Warning,
            enabled: true,
        });

        // High IOPS alert
        conditions.push(AlertCondition {
            id: "high-iops".to_string(),
            name: "High IOPS".to_string(),
            description: "IOPS exceeds safe threshold".to_string(),
            metric: AlertMetric::Iops,
            operator: AlertOperator::GreaterThan,
            threshold: 10000.0, // 10K IOPS
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_IOPS_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(180), // 3 minutes default
            ),
            severity: AlertSeverity::Critical,
            enabled: true,
        });

        // High utilization alert
        conditions.push(AlertCondition {
            id: "high-utilization".to_string(),
            name: "High Disk Utilization".to_string(),
            description: "Disk utilization exceeds safe threshold".to_string(),
            metric: AlertMetric::Utilization,
            operator: AlertOperator::GreaterThan,
            threshold: 85.0, // 85% utilization
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_UTILIZATION_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(600), // 10 minutes default
            ),
            severity: AlertSeverity::Warning,
            enabled: true,
        });

        // Low cache hit ratio alert
        conditions.push(AlertCondition {
            id: "low-cache-hit".to_string(),
            name: "Low Cache Hit Ratio".to_string(),
            description: "ZFS cache hit ratio is below optimal threshold".to_string(),
            metric: AlertMetric::CacheHitRatio,
            operator: AlertOperator::LessThan,
            threshold: 80.0, // 80% cache hit ratio
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_CACHE_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(900), // 15 minutes default
            ),
            severity: AlertSeverity::Info,
            enabled: true,
        });
        Ok(())
    }

    /// Initialize tier performance targets
    async fn initialize_tier_targets(&self) -> CoreResult<()> {
        let mut tier_metrics = self.tier_metrics.write().await;

        // Initialize tier data with performance targets
        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            let _targets = match tier {
                StorageTier::Hot => TierPerformanceTargets {
                    target_iops: 10000.0,
                    target_throughput_mbs: 500.0,
                    target_latency_ms: 10.0,
                    target_utilization_percent: 80.0,
                    target_availability_percent: 99.9,
                },
                StorageTier::Warm => TierPerformanceTargets {
                    target_iops: 5000.0,
                    target_throughput_mbs: 250.0,
                    target_latency_ms: 50.0,
                    target_utilization_percent: 85.0,
                    target_availability_percent: 99.5,
                },
                StorageTier::Cold => TierPerformanceTargets {
                    target_iops: 1000.0,
                    target_throughput_mbs: 100.0,
                    target_latency_ms: 200.0,
                    target_utilization_percent: 90.0,
                    target_availability_percent: 99.0,
                },
                StorageTier::Cache => TierPerformanceTargets {
                    target_iops: 50000.0,
                    target_throughput_mbs: 1000.0,
                    target_latency_ms: 1.0,
                    target_utilization_percent: 70.0,
                    target_availability_percent: 99.9,
                },
                StorageTier::Archive => TierPerformanceTargets {
                    target_iops: 100.0,
                    target_throughput_mbs: 50.0,
                    target_latency_ms: 100.0,
                    target_utilization_percent: 95.0,
                    target_availability_percent: 98.0,
                },
            };

            tier_metrics.insert(
                tier,
                TierPerformanceData {
                    tier,
                    current_metrics: TierMetrics::default_for_tier(tier),
                    history: VecDeque::new(),
                    trends: PerformanceTrends::default(),
                },
            );
        }
        Ok(())
    }

    /// Start metrics collection task
    async fn start_collection_task(&mut self) -> CoreResult<()> {
        let pool_manager = Arc::clone(&self.pool_manager);
        let dataset_manager = Arc::clone(&self.dataset_manager);
        let current_metrics = Arc::clone(&self.current_metrics);
        let tier_metrics = Arc::clone(&self.tier_metrics);
        // Use default collection interval since config was removed
        let collection_interval = 30; // 30 seconds default

        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(collection_interval));

            loop {
                interval.tick().await;

                if let Err(e) = Self::collect_metrics(
                    &pool_manager,
                    &dataset_manager,
                    &current_metrics,
                    &tier_metrics,
                )
                .await
                {
                    error!("Metrics collection failed: {}", e);
                }
            }
        });

        self.collection_task = Some(task);
        Ok(())
    }
}
