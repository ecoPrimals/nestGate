//
// Main manager for coordinating snapshot operations, policies, and automation.

//! Manager module

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
// Removed unused tracing import

use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, types::StorageTier};
use nestgate_core::Result as CoreResult;

use super::operations::SnapshotOperationType;
use super::policy::{RetentionPolicy, ScheduleFrequency, SnapshotPolicy};
use super::scheduler::PolicyScheduler;
use super::types::{SnapshotInfo, SnapshotOperation, SnapshotOperationStatus, SnapshotStatistics};
use tracing::debug;
use tracing::error;
use tracing::info;

// Type aliases for complex types
type SnapshotPolicyMap = Arc<RwLock<HashMap<String, SnapshotPolicy>>>;
/// Type alias for SnapshotInfoCache
type SnapshotInfoCache = Arc<RwLock<HashMap<String, SnapshotInfo>>>;

/// ZFS Snapshot Manager
#[derive(Debug)]
/// Manager for ZfsSnapshot operations
pub struct ZfsSnapshotManager {
    #[allow(dead_code)]
    config: ZfsConfig,
    dataset_manager: Arc<ZfsDatasetManager>,
    /// Snapshot policies
    policies: SnapshotPolicyMap,
    /// Snapshot cache
    snapshot_cache: SnapshotInfoCache,
    /// Operation queue
    operation_queue: Arc<RwLock<Vec<SnapshotOperation>>>,
    /// Statistics
    statistics: Arc<RwLock<SnapshotStatistics>>,

    /// Policy scheduler
    policy_scheduler: PolicyScheduler,

    /// Shutdown signal
    shutdown_tx: Option<mpsc::Sender<()>>,
    #[allow(dead_code)]
    background_tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl ZfsSnapshotManager {
    /// Create a new snapshot manager
    #[must_use]
    pub fn new(config: ZfsConfig, dataset_manager: Arc<ZfsDatasetManager>) -> Self {
        let policies = Arc::new(RwLock::new(HashMap::new()));
        let operation_queue = Arc::new(RwLock::new(Vec::new()));

        let dataset_manager_clone = Arc::clone(&dataset_manager);

        Self {
            config,
            dataset_manager,
            policies: Arc::clone(&policies),
            snapshot_cache: Arc::new(RwLock::new(HashMap::new())),
            operation_queue: Arc::clone(&operation_queue),
            statistics: Arc::new(RwLock::new(SnapshotStatistics::default())),
            policy_scheduler: PolicyScheduler::new(
                dataset_manager_clone,
                Arc::clone(&policies),
                Arc::clone(&operation_queue),
            ),
            shutdown_tx: None,
            background_tasks: Vec::new(),
        }
    }

    /// Create a new snapshot manager with shared config (zero-copy optimization)
    #[must_use]
    pub fn with_shared_config(
        config: Arc<ZfsConfig>,
        dataset_manager: Arc<ZfsDatasetManager>,
    ) -> Self {
        let policies = Arc::new(RwLock::new(HashMap::new()));
        let operation_queue = Arc::new(RwLock::new(Vec::new()));

        let dataset_manager_clone = Arc::clone(&dataset_manager);

        Self {
            config: (*config).clone(),
            dataset_manager,
            policies: Arc::clone(&policies),
            snapshot_cache: Arc::new(RwLock::new(HashMap::new())),
            operation_queue: Arc::clone(&operation_queue),
            statistics: Arc::new(RwLock::new(SnapshotStatistics::default())),
            policy_scheduler: PolicyScheduler::new(
                dataset_manager_clone,
                Arc::clone(&policies),
                Arc::clone(&operation_queue),
            ),
            shutdown_tx: None,
            background_tasks: Vec::new(),
        }
    }

    /// Start the snapshot manager
    pub async fn start(&mut self) -> CoreResult<()> {
        info!("Starting ZFS snapshot manager");

        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        // Load default policies
        self.load_default_policies().await?;

        // Start policy scheduler
        let policy_scheduler_clone = PolicyScheduler::new(
            Arc::clone(&self.dataset_manager),
            Arc::clone(&self.policies),
            Arc::clone(&self.operation_queue),
        );

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(
                std::env::var("NESTGATE_ZFS_SNAPSHOT_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60), // 60 seconds default
            ));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = policy_scheduler_clone.process_policies().await {
                            error!("Error processing snapshot policies: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Snapshot policy scheduler shutting down");
                        break;
                    }
                }
            }
        });

        // Start cache updater
        self.start_cache_updater().await?;

        info!("ZFS snapshot manager started successfully");
        Ok(())
    }

    /// Stop the snapshot manager
    pub async fn stop(&mut self) -> CoreResult<()> {
        info!("Stopping ZFS snapshot manager");

        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
        }

        info!("ZFS snapshot manager stopped");
        Ok(())
    }

    /// Add a snapshot policy
    pub async fn add_policy(&self, policy: SnapshotPolicy) -> CoreResult<()> {
        info!("Adding snapshot policy: {}", policy.name);

        let mut policies = self.policies.write().await;
        policies.insert(policy.name.clone(), policy);
        Ok(())
    }

    /// Remove a snapshot policy
    pub async fn remove_policy(&self, name: &str) -> CoreResult<bool> {
        info!("Removing snapshot policy: {}", name);

        let mut policies = self.policies.write().await;
        Ok(policies.remove(name).is_some())
    }

    /// Get a snapshot policy
    pub async fn get_policy(&self, name: &str) -> Option<SnapshotPolicy> {
        let policies = self.policies.read().await;
        policies.get(name).cloned()
    }

    /// List all policies
    pub async fn list_policies(&self) -> Vec<SnapshotPolicy> {
        let policies = self.policies.read().await;
        policies.values().cloned().collect()
    }

    /// Create a snapshot manually
    pub async fn create_snapshot(
        &self,
        dataset: &str,
        name: &str,
        _recursive: bool,
    ) -> CoreResult<String> {
        info!("Creating snapshot: {}@{}", dataset, name);

        let operation = SnapshotOperation {
            id: self.generate_operation_id(),
            operation_type: SnapshotOperationType::Create,
            dataset: dataset.to_string(),
            snapshot_name: Some(name.to_string()),
            status: SnapshotOperationStatus::Queued,
            created_at: SystemTime::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            policy: None,
        };

        let operation_id = operation.id.clone();

        let mut queue = self.operation_queue.write().await;
        queue.push(operation);

        Ok(operation_id)
    }

    /// Delete a snapshot
    pub async fn delete_snapshot(&self, dataset: &str, name: &str) -> CoreResult<String> {
        info!("Deleting snapshot: {}@{}", dataset, name);

        let operation = SnapshotOperation {
            id: self.generate_operation_id(),
            operation_type: SnapshotOperationType::Delete,
            dataset: dataset.to_string(),
            snapshot_name: Some(name.to_string()),
            status: SnapshotOperationStatus::Queued,
            created_at: SystemTime::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            policy: None,
        };

        let operation_id = operation.id.clone();

        let mut queue = self.operation_queue.write().await;
        queue.push(operation);

        Ok(operation_id)
    }

    /// List snapshots for a dataset
    pub async fn list_snapshots(&self, dataset: &str) -> CoreResult<Vec<SnapshotInfo>> {
        debug!("Listing snapshots for dataset: {}", dataset);

        let cache = self.snapshot_cache.read().await;
        let snapshots: Vec<_> = cache
            .values()
            .filter(|snapshot| snapshot.dataset == dataset)
            .cloned()
            .collect();

        Ok(snapshots)
    }

    /// Get snapshot statistics
    pub async fn get_statistics(&self) -> SnapshotStatistics {
        self.statistics.read().await.clone()
    }

    /// Get operation status
    pub async fn get_operation_status(&self, operation_id: &str) -> Option<SnapshotOperation> {
        let queue = self.operation_queue.read().await;
        queue.iter().find(|op| op.id == operation_id).cloned()
    }

    /// Load default policies
    async fn load_default_policies(&self) -> CoreResult<()> {
        info!("Loading default snapshot policies");

        let hot_policy = SnapshotPolicy {
            name: "Hot Tier Snapshots".to_string(),
            description: "Frequent snapshots for hot tier data".to_string(),
            enabled: true,
            frequency: ScheduleFrequency::Hours(1),
            retention: RetentionPolicy::Custom {
                hourly_hours: 24,
                daily_days: 7,
                weekly_weeks: 4,
                monthly_months: 3,
                yearly_years: 1,
            },
            dataset_patterns: vec!["*/hot/*".to_string()],
            tiers: vec![StorageTier::Hot],
            name_prefix: "hot".to_string(),
            include_properties: true,
            recursive: true,
            max_snapshots_per_run: 10,
            priority: 3,
        };

        let warm_policy = SnapshotPolicy {
            name: "Warm Tier Snapshots".to_string(),
            description: "Regular snapshots for warm tier data".to_string(),
            enabled: true,
            frequency: ScheduleFrequency::Daily(2),
            retention: RetentionPolicy::Custom {
                hourly_hours: 0,
                daily_days: 14,
                weekly_weeks: 8,
                monthly_months: 6,
                yearly_years: 2,
            },
            dataset_patterns: vec!["*/warm/*".to_string()],
            tiers: vec![StorageTier::Warm],
            name_prefix: "warm".to_string(),
            include_properties: true,
            recursive: true,
            max_snapshots_per_run: 5,
            priority: 2,
        };

        let cold_policy = SnapshotPolicy {
            name: "Cold Tier Snapshots".to_string(),
            description: "Infrequent snapshots for cold tier data".to_string(),
            enabled: true,
            frequency: ScheduleFrequency::Weekly { day: 0, hour: 3 },
            retention: RetentionPolicy::Custom {
                hourly_hours: 0,
                daily_days: 0,
                weekly_weeks: 12,
                monthly_months: 12,
                yearly_years: 5,
            },
            dataset_patterns: vec!["*/cold/*".to_string()],
            tiers: vec![StorageTier::Cold],
            name_prefix: "cold".to_string(),
            include_properties: true,
            recursive: true,
            max_snapshots_per_run: 2,
            priority: 1,
        };

        // Add policies
        self.add_policy(hot_policy).await?;
        self.add_policy(warm_policy).await?;
        self.add_policy(cold_policy).await?;

        info!("Default snapshot policies loaded");
        Ok(())
    }

    /// Start cache updater
    pub async fn start_cache_updater(&self) -> CoreResult<()> {
        let snapshot_cache = Arc::clone(&self.snapshot_cache);
        let statistics = Arc::clone(&self.statistics);
        let dataset_manager = Arc::clone(&self.dataset_manager);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(
                std::env::var("NESTGATE_ZFS_SNAPSHOT_CACHE_UPDATE_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes default
            ));

            loop {
                interval.tick().await;

                if let Err(e) =
                    Self::update_cache(&snapshot_cache, &statistics, &dataset_manager).await
                {
                    error!("Error updating snapshot cache: {}", e);
                }
            }
        });
        Ok(())
    }

    /// Update snapshot cache
    async fn update_cache(
        snapshot_cache: &SnapshotInfoCache,
        statistics: &Arc<RwLock<SnapshotStatistics>>,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        debug!("Updating snapshot cache");

        let datasets = dataset_manager.list_datasets().await?;
        let mut cache = snapshot_cache.write().await;
        cache.clear();

        let mut total_snapshots = 0;
        let mut total_size = 0;

        for dataset in datasets {
            if let Ok(snapshots) = dataset_manager.list_snapshots(&dataset.name).await {
                for snapshot in snapshots {
                    total_snapshots += 1;
                    total_size += snapshot.size;
                    cache.insert(snapshot.full_name.clone(), snapshot);
                }
            }
        }

        // Update statistics
        let mut stats = statistics.write().await;
        stats.total_snapshots = total_snapshots;
        stats.total_size = total_size;

        debug!(
            "Cache updated: {} snapshots, {} bytes total",
            total_snapshots, total_size
        );
        Ok(())
    }

    /// Generate unique operation ID
    fn generate_operation_id(&self) -> String {
        format!(
            "snap_op_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_millis()
        )
    }

    /// Parse schedule frequency to duration for next execution
    pub fn parse_schedule(&self, schedule: &ScheduleFrequency) -> CoreResult<Duration> {
        self.policy_scheduler.parse_schedule(schedule)
    }
}

// ========== TEST-ONLY CONSTRUCTORS ==========
// Isolated from production code to maintain clear boundaries

#[cfg(test)]
impl ZfsSnapshotManager {
    /// Create snapshot manager for testing
    ///
    /// **TEST-ONLY**: This constructor is only available in test builds.
    /// Production code must use `ZfsSnapshotManager::new()` with proper configuration.
    #[must_use]
    pub fn new_for_testing() -> Self {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
        let dataset_manager = Arc::new(ZfsDatasetManager::new(config, pool_manager));
        let policies = Arc::new(RwLock::new(HashMap::new()));
        let operation_queue = Arc::new(RwLock::new(Vec::new()));

        Self {
            config: ZfsConfig::default(),
            dataset_manager: dataset_manager.clone(),
            policies: policies.clone(),
            snapshot_cache: Arc::new(RwLock::new(HashMap::new())),
            operation_queue: operation_queue.clone(),
            statistics: Arc::new(RwLock::new(SnapshotStatistics::default())),
            policy_scheduler: PolicyScheduler::new(dataset_manager, policies, operation_queue),
            shutdown_tx: None,
            background_tasks: Vec::new(),
        }
    }
}
