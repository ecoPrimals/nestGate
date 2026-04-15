// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Main manager for coordinating snapshot operations, policies, and automation.

//! Manager module

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{RwLock, mpsc};
use tokio::time::interval;
// Removed unused tracing import

use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, types::StorageTier};
use nestgate_core::Result as CoreResult;
use nestgate_types::{EnvSource, ProcessEnv, env_parsed};

use super::operations::SnapshotOperationType;
use super::policy::{RetentionPolicy, ScheduleFrequency, SnapshotPolicy};
use super::scheduler::PolicyScheduler;
use super::types::{SnapshotInfo, SnapshotOperation, SnapshotOperationStatus, SnapshotStatistics};
use tracing::debug;
use tracing::error;
use tracing::info;

// Type aliases for complex types
type SnapshotPolicyMap = Arc<RwLock<HashMap<String, SnapshotPolicy>>>;
/// Type alias for `SnapshotInfoCache`
type SnapshotInfoCache = Arc<RwLock<HashMap<String, SnapshotInfo>>>;

/// ZFS Snapshot Manager
#[derive(Debug)]
/// Manager for `ZfsSnapshot` operations
pub struct ZfsSnapshotManager {
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
        self.start_from_env_source(&ProcessEnv).await
    }

    /// Like [`Self::start`], but reads interval env vars from an injectable [`EnvSource`].
    pub async fn start_from_env_source(
        &mut self,
        env: &(impl EnvSource + ?Sized),
    ) -> CoreResult<()> {
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

        let policy_interval_secs =
            env_parsed(env, "NESTGATE_ZFS_SNAPSHOT_CHECK_INTERVAL_SECS", 60_u64);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(policy_interval_secs));

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
        self.start_cache_updater_from_env_source(env)?;

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

        self.policies
            .write()
            .await
            .insert(policy.name.clone(), policy);
        Ok(())
    }

    /// Remove a snapshot policy
    pub async fn remove_policy(&self, name: &str) -> CoreResult<bool> {
        info!("Removing snapshot policy: {}", name);

        Ok(self.policies.write().await.remove(name).is_some())
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

        self.operation_queue.write().await.push(operation);

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

        self.operation_queue.write().await.push(operation);

        Ok(operation_id)
    }

    /// List snapshots for a dataset
    pub async fn list_snapshots(&self, dataset: &str) -> CoreResult<Vec<SnapshotInfo>> {
        debug!("Listing snapshots for dataset: {}", dataset);

        let snapshots: Vec<_> = {
            let cache = self.snapshot_cache.read().await;
            cache
                .values()
                .filter(|snapshot| snapshot.dataset == dataset)
                .cloned()
                .collect()
        };

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
    pub fn start_cache_updater(&self) -> CoreResult<()> {
        self.start_cache_updater_from_env_source(&ProcessEnv)
    }

    /// Like [`Self::start_cache_updater`], but reads the interval from an injectable [`EnvSource`].
    pub fn start_cache_updater_from_env_source(
        &self,
        env: &(impl EnvSource + ?Sized),
    ) -> CoreResult<()> {
        let snapshot_cache = Arc::clone(&self.snapshot_cache);
        let statistics = Arc::clone(&self.statistics);
        let dataset_manager = Arc::clone(&self.dataset_manager);
        let cache_interval_secs = env_parsed(
            env,
            "NESTGATE_ZFS_SNAPSHOT_CACHE_UPDATE_INTERVAL_SECS",
            300_u64,
        );

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(cache_interval_secs));

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

        let mut merged: HashMap<String, SnapshotInfo> = HashMap::new();
        let mut total_snapshots = 0;
        let mut total_size = 0;

        for dataset in datasets {
            if let Ok(snapshots) = dataset_manager.list_snapshots(&dataset.name).await {
                for snapshot in snapshots {
                    total_snapshots += 1;
                    total_size += snapshot.size;
                    merged.insert(snapshot.full_name.clone(), snapshot);
                }
            }
        }

        {
            let mut cache = snapshot_cache.write().await;
            cache.clear();
            cache.extend(merged);
        }

        // Update statistics
        {
            let mut stats = statistics.write().await;
            stats.total_snapshots = total_snapshots;
            stats.total_size = total_size;
        }

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

#[cfg(any(test, feature = "dev-stubs"))]
impl ZfsSnapshotManager {
    /// Create snapshot manager for testing
    ///
    /// **TEST-ONLY**: This constructor is only available in test builds.
    /// Production code must use `ZfsSnapshotManager::new()` with proper configuration.
    #[must_use]
    pub fn new_for_testing() -> Self {
        use crate::ZfsPoolManager;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::snapshot::policy::{ScheduleFrequency, SnapshotPolicy};

    #[tokio::test]
    async fn add_get_list_remove_policy_roundtrip() {
        let mgr = ZfsSnapshotManager::new_for_testing();
        let p = SnapshotPolicy {
            name: "p1".into(),
            ..SnapshotPolicy::default()
        };
        mgr.add_policy(p.clone()).await.expect("test: add_policy");
        let got = mgr
            .get_policy("p1")
            .await
            .expect("test: policy p1 should exist");
        assert_eq!(got.name, "p1");

        let list = mgr.list_policies().await;
        assert!(list.iter().any(|x| x.name == "p1"));

        let removed = mgr.remove_policy("p1").await.expect("test: remove_policy");
        assert!(removed);
        assert!(mgr.get_policy("p1").await.is_none());
    }

    #[tokio::test]
    async fn create_and_delete_snapshot_return_operation_ids() {
        let mgr = ZfsSnapshotManager::new_for_testing();
        let id_create = mgr
            .create_snapshot("pool/ds", "snap1", false)
            .await
            .expect("test: create_snapshot");
        let id_delete = mgr
            .delete_snapshot("pool/ds", "snap1")
            .await
            .expect("test: delete_snapshot");
        assert!(id_create.starts_with("snap_op_"));
        assert!(id_delete.starts_with("snap_op_"));

        let op_c = mgr
            .get_operation_status(&id_create)
            .await
            .expect("test: create op in queue");
        assert_eq!(op_c.snapshot_name.as_deref(), Some("snap1"));

        let op_d = mgr
            .get_operation_status(&id_delete)
            .await
            .expect("test: delete op in queue");
        assert_eq!(op_d.snapshot_name.as_deref(), Some("snap1"));
    }

    #[tokio::test]
    async fn list_snapshots_empty_cache() {
        let mgr = ZfsSnapshotManager::new_for_testing();
        let snaps = mgr
            .list_snapshots("any")
            .await
            .expect("test: list_snapshots");
        assert!(snaps.is_empty());
    }

    #[tokio::test]
    async fn get_statistics_default() {
        let mgr = ZfsSnapshotManager::new_for_testing();
        let stats = mgr.get_statistics().await;
        assert_eq!(stats.total_snapshots, 0);
    }

    #[tokio::test]
    async fn parse_schedule_delegates_to_scheduler() {
        let mgr = ZfsSnapshotManager::new_for_testing();
        let d = mgr
            .parse_schedule(&ScheduleFrequency::Hours(2))
            .expect("test: parse_schedule hours");
        assert_eq!(d, std::time::Duration::from_secs(7200));
    }

    #[tokio::test]
    async fn parse_schedule_custom_errors() {
        let mgr = ZfsSnapshotManager::new_for_testing();
        let err = mgr
            .parse_schedule(&ScheduleFrequency::Custom("0 * * * *".into()))
            .expect_err("test: custom schedule should error");
        let _ = err;
    }

    #[tokio::test]
    #[ignore = "Requires real ZFS"]
    async fn start_loads_policies_and_spawns_background_tasks() {
        let mut mgr = ZfsSnapshotManager::new_for_testing();
        mgr.start().await.expect("test: snapshot manager start");
        mgr.stop().await.expect("test: snapshot manager stop");
    }
}
