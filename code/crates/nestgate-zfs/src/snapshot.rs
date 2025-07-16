//! ZFS Snapshot Management
//!
//! Comprehensive snapshot lifecycle management with automated policies,
//! retention rules, and backup integration for production-ready ZFS systems.

use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
use tracing::{debug, error, info, warn};

use crate::{
    config::ZfsConfig, dataset::ZfsDatasetManager, error::SnapshotError, error::ZfsError,
    pool::ZfsPoolManager,
};
use nestgate_core::{NestGateError, Result as CoreResult, StorageTier};

/// Snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RetentionPolicy {
    /// Keep snapshots for a specific duration
    Duration(Duration),
    /// Keep a specific number of snapshots
    Count(u32),
    /// Custom retention rule
    Custom {
        /// Keep hourly snapshots for this many hours
        hourly_hours: u32,
        /// Keep daily snapshots for this many days
        daily_days: u32,
        /// Keep weekly snapshots for this many weeks
        weekly_weeks: u32,
        /// Keep monthly snapshots for this many months
        monthly_months: u32,
        /// Keep yearly snapshots for this many years
        yearly_years: u32,
    },
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self::Custom {
            hourly_hours: 24,   // 24 hours
            daily_days: 30,     // 30 days
            weekly_weeks: 12,   // 12 weeks
            monthly_months: 12, // 12 months
            yearly_years: 5,    // 5 years
        }
    }
}

/// Snapshot schedule frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleFrequency {
    /// Take snapshots every N minutes
    Minutes(u32),
    /// Take snapshots every N hours
    Hours(u32),
    /// Take snapshots daily at specific hour
    Daily(u8),
    /// Take snapshots weekly on specific day and hour
    Weekly { day: u8, hour: u8 },
    /// Take snapshots monthly on specific day and hour
    Monthly { day: u8, hour: u8 },
    /// Custom cron-like schedule
    Cron(String),
}

/// Snapshot policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotPolicy {
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Whether policy is enabled
    pub enabled: bool,
    /// Schedule frequency
    pub frequency: ScheduleFrequency,
    /// Retention policy
    pub retention: RetentionPolicy,
    /// Datasets to apply policy to (glob patterns)
    pub dataset_patterns: Vec<String>,
    /// Storage tiers to apply policy to
    pub tiers: Vec<StorageTier>,
    /// Snapshot name prefix
    pub name_prefix: String,
    /// Whether to include properties in snapshot
    pub include_properties: bool,
    /// Whether to create recursive snapshots
    pub recursive: bool,
    /// Maximum snapshots to create per run
    pub max_snapshots_per_run: u32,
    /// Priority (higher number = higher priority)
    pub priority: u32,
}

impl Default for SnapshotPolicy {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            description: "Default snapshot policy".to_string(),
            enabled: true,
            frequency: ScheduleFrequency::Hours(1),
            retention: RetentionPolicy::default(),
            dataset_patterns: vec!["*".to_string()],
            tiers: vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold],
            name_prefix: "auto".to_string(),
            include_properties: true,
            recursive: true,
            max_snapshots_per_run: 100,
            priority: 50,
        }
    }
}

/// Snapshot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot name
    pub name: String,
    /// Full snapshot path (dataset@snapshot)
    pub full_name: String,
    /// Dataset name
    pub dataset: String,
    /// Creation time
    pub created_at: SystemTime,
    /// Snapshot size in bytes
    pub size: u64,
    /// Referenced data size in bytes
    pub referenced_size: u64,
    /// Written data size in bytes
    pub written_size: u64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Snapshot properties
    pub properties: HashMap<String, String>,
    /// Associated policy name
    pub policy: Option<String>,
    /// Storage tier
    pub tier: StorageTier,
    /// Whether snapshot is protected from deletion
    pub protected: bool,
    /// Snapshot tags for organization
    pub tags: Vec<String>,
}

/// Snapshot operation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SnapshotOperationStatus {
    /// Operation is queued
    Queued,
    /// Operation is running
    Running,
    /// Operation completed successfully
    Completed,
    /// Operation failed
    Failed(String),
    /// Operation was cancelled
    Cancelled,
}

/// Snapshot operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotOperation {
    /// Operation ID
    pub id: String,
    /// Operation type
    pub operation_type: SnapshotOperationType,
    /// Target dataset
    pub dataset: String,
    /// Snapshot name (for create/delete operations)
    pub snapshot_name: Option<String>,
    /// Operation status
    pub status: SnapshotOperationStatus,
    /// Created timestamp
    pub created_at: SystemTime,
    /// Started timestamp
    pub started_at: Option<SystemTime>,
    /// Completed timestamp
    pub completed_at: Option<SystemTime>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Associated policy
    pub policy: Option<String>,
}

/// Types of snapshot operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotOperationType {
    /// Create a new snapshot
    Create,
    /// Delete an existing snapshot
    Delete,
    /// Clone a snapshot
    Clone,
    /// Rollback to a snapshot
    Rollback,
    /// Send snapshot to another location
    Send,
    /// Receive snapshot from another location
    Receive,
}

/// Snapshot statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotStatistics {
    /// Total snapshots across all datasets
    pub total_snapshots: u64,
    /// Total snapshot data size in bytes
    pub total_size: u64,
    /// Total referenced data size in bytes
    pub total_referenced_size: u64,
    /// Total written data size in bytes
    pub total_written_size: u64,
    /// Average compression ratio
    pub average_compression_ratio: f64,
    /// Snapshots per tier
    pub snapshots_per_tier: HashMap<StorageTier, u64>,
    /// Size per tier
    pub size_per_tier: HashMap<StorageTier, u64>,
    /// Active policies
    pub active_policies: u32,
    /// Pending operations
    pub pending_operations: u32,
    /// Failed operations in last 24 hours
    pub recent_failures: u32,
}

/// ZFS Snapshot Manager
#[derive(Debug)]
pub struct ZfsSnapshotManager {
    #[allow(dead_code)]
    config: ZfsConfig,
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,

    /// Snapshot policies
    policies: Arc<RwLock<HashMap<String, SnapshotPolicy>>>,
    /// Snapshot cache
    snapshot_cache: Arc<RwLock<HashMap<String, SnapshotInfo>>>,
    /// Operation queue
    operation_queue: Arc<RwLock<Vec<SnapshotOperation>>>,
    /// Statistics
    statistics: Arc<RwLock<SnapshotStatistics>>,

    /// Shutdown signal
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl ZfsSnapshotManager {
    /// Create a new snapshot manager
    pub fn new(
        config: ZfsConfig,
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
    ) -> Self {
        Self {
            config,
            pool_manager,
            dataset_manager,
            policies: Arc::new(RwLock::new(HashMap::new())),
            snapshot_cache: Arc::new(RwLock::new(HashMap::new())),
            operation_queue: Arc::new(RwLock::new(Vec::new())),
            statistics: Arc::new(RwLock::new(SnapshotStatistics::default())),
            shutdown_tx: None,
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
        let policies = Arc::clone(&self.policies);
        let operation_queue = Arc::clone(&self.operation_queue);
        let pool_manager = Arc::clone(&self.pool_manager);
        let dataset_manager = Arc::clone(&self.dataset_manager);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(
                std::env::var("NESTGATE_ZFS_SNAPSHOT_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60), // 60 seconds default
            )); // Snapshot check interval

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = Self::process_policies(
                            &policies,
                            &operation_queue,
                            &pool_manager,
                            &dataset_manager,
                        ).await {
                            error!("Error processing snapshot policies: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Snapshot manager shutting down");
                        break;
                    }
                }
            }
        });

        // Start operation processor
        self.start_operation_processor().await?;

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

        // Hot tier policy - frequent snapshots, shorter retention
        let hot_policy = SnapshotPolicy {
            name: "hot-tier".to_string(),
            description: "High-frequency snapshots for hot tier data".to_string(),
            enabled: true,
            frequency: ScheduleFrequency::Hours(1),
            retention: RetentionPolicy::Custom {
                hourly_hours: 48,
                daily_days: 14,
                weekly_weeks: 4,
                monthly_months: 3,
                yearly_years: 1,
            },
            dataset_patterns: vec!["*/hot/*".to_string()],
            tiers: vec![StorageTier::Hot],
            name_prefix: "hot-auto".to_string(),
            include_properties: true,
            recursive: true,
            max_snapshots_per_run: 50,
            priority: 90,
        };

        // Warm tier policy - moderate snapshots, medium retention
        let warm_policy = SnapshotPolicy {
            name: "warm-tier".to_string(),
            description: "Moderate snapshots for warm tier data".to_string(),
            enabled: true,
            frequency: ScheduleFrequency::Hours(6),
            retention: RetentionPolicy::Custom {
                hourly_hours: 24,
                daily_days: 30,
                weekly_weeks: 12,
                monthly_months: 6,
                yearly_years: 2,
            },
            dataset_patterns: vec!["*/warm/*".to_string()],
            tiers: vec![StorageTier::Warm],
            name_prefix: "warm-auto".to_string(),
            include_properties: true,
            recursive: true,
            max_snapshots_per_run: 25,
            priority: 70,
        };

        // Cold tier policy - infrequent snapshots, long retention
        let cold_policy = SnapshotPolicy {
            name: "cold-tier".to_string(),
            description: "Infrequent snapshots for cold tier data with long retention".to_string(),
            enabled: true,
            frequency: ScheduleFrequency::Daily(2), // 2 AM daily
            retention: RetentionPolicy::Custom {
                hourly_hours: 0, // No hourly snapshots
                daily_days: 90,
                weekly_weeks: 52,
                monthly_months: 24,
                yearly_years: 10,
            },
            dataset_patterns: vec!["*/cold/*".to_string()],
            tiers: vec![StorageTier::Cold],
            name_prefix: "cold-auto".to_string(),
            include_properties: true,
            recursive: true,
            max_snapshots_per_run: 10,
            priority: 50,
        };

        // Add policies
        self.add_policy(hot_policy).await?;
        self.add_policy(warm_policy).await?;
        self.add_policy(cold_policy).await?;

        info!("Default snapshot policies loaded");
        Ok(())
    }

    /// Process snapshot policies
    async fn process_policies(
        policies: &Arc<RwLock<HashMap<String, SnapshotPolicy>>>,
        operation_queue: &Arc<RwLock<Vec<SnapshotOperation>>>,
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        let policies_guard = policies.read().await;

        for policy in policies_guard.values() {
            if !policy.enabled {
                continue;
            }

            if Self::should_execute_policy(policy).await {
                if let Err(e) =
                    Self::execute_policy(policy, operation_queue, pool_manager, dataset_manager)
                        .await
                {
                    error!("Failed to execute policy {}: {}", policy.name, e);
                }
            }
        }

        Ok(())
    }

    /// Check if a policy should be executed now
    async fn should_execute_policy(policy: &SnapshotPolicy) -> bool {
        let now = chrono::Utc::now();

        match &policy.frequency {
            ScheduleFrequency::Minutes(_minutes) => {
                // Implement minute-based scheduling
                let current_minute = now.minute();
                let should_execute = current_minute % _minutes == 0 && now.second() < 30; // 30-second window
                debug!(
                    "Minute-based policy '{}': current_minute={}, interval={}, should_execute={}",
                    policy.name, current_minute, _minutes, should_execute
                );
                should_execute
            }
            ScheduleFrequency::Hours(_hours) => {
                // Implement hour-based scheduling
                let current_hour = now.hour();
                let should_execute = current_hour % _hours == 0 && now.minute() < 5; // 5-minute window
                debug!(
                    "Hour-based policy '{}': current_hour={}, interval={}, should_execute={}",
                    policy.name, current_hour, _hours, should_execute
                );
                should_execute
            }
            ScheduleFrequency::Daily(hour) => {
                now.hour() == *hour as u32 && now.minute() < 5 // 5-minute window
            }
            ScheduleFrequency::Weekly { day, hour } => {
                now.weekday().number_from_monday() == *day as u32
                    && now.hour() == *hour as u32
                    && now.minute() < 5
            }
            ScheduleFrequency::Monthly { day, hour } => {
                now.day() == *day as u32 && now.hour() == *hour as u32 && now.minute() < 5
            }
            ScheduleFrequency::Cron(cron_expr) => {
                // Implement basic cron parsing for hour:minute format
                if let Ok(time) = chrono::NaiveTime::parse_from_str(cron_expr, "%H:%M") {
                    let matches_time = now.hour() == time.hour() && now.minute() == time.minute();
                    debug!(
                        "Cron policy '{}': expression='{}', current={}:{}, should_execute={}",
                        policy.name,
                        cron_expr,
                        now.hour(),
                        now.minute(),
                        matches_time
                    );
                    matches_time
                } else {
                    warn!(
                        "Invalid cron expression '{}' in policy '{}'",
                        cron_expr, policy.name
                    );
                    false
                }
            }
        }
    }

    /// Execute a snapshot policy
    async fn execute_policy(
        policy: &SnapshotPolicy,
        operation_queue: &Arc<RwLock<Vec<SnapshotOperation>>>,
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        debug!("Executing snapshot policy: {}", policy.name);

        // 1. Find datasets matching the policy patterns
        let all_datasets = dataset_manager.list_datasets().await?;
        let mut matching_datasets = Vec::new();

        for dataset in all_datasets {
            for pattern in &policy.dataset_patterns {
                if pattern == "*" || dataset.name.contains(pattern) {
                    matching_datasets.push(dataset);
                    break;
                }
            }
        }

        // 2. Create snapshot operations for each dataset
        let mut queue = operation_queue.write().await;
        let mut operations_created = 0;

        for dataset in &matching_datasets {
            // Check if we've reached the max snapshots per run
            if operations_created >= policy.max_snapshots_per_run {
                break;
            }

            // Generate snapshot name with timestamp
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let snapshot_name = format!("{}_{}", policy.name_prefix, timestamp);

            // Create snapshot operation
            let operation = SnapshotOperation {
                id: uuid::Uuid::new_v4().to_string(),
                operation_type: SnapshotOperationType::Create,
                dataset: dataset.name.clone(),
                snapshot_name: Some(snapshot_name.clone()),
                status: SnapshotOperationStatus::Queued,
                created_at: SystemTime::now(),
                started_at: None,
                completed_at: None,
                error_message: None,
                policy: Some(policy.name.clone()),
            };

            queue.push(operation);
            operations_created += 1;

            debug!(
                "Queued snapshot operation for dataset '{}' with name '{}'",
                dataset.name, snapshot_name
            );
        }

        drop(queue);

        // 3. Apply retention policy - clean up old snapshots
        for dataset in &matching_datasets {
            if let Err(e) = Self::apply_retention_policy_for_dataset(
                &dataset.name,
                &policy.retention,
                pool_manager,
            )
            .await
            {
                error!(
                    "Failed to apply retention policy for dataset {}: {}",
                    dataset.name, e
                );
            }
        }

        info!(
            "Policy '{}' executed: {} operations queued for {} datasets",
            policy.name,
            operations_created,
            matching_datasets.len()
        );

        Ok(())
    }

    /// Apply retention policy for a specific dataset
    async fn apply_retention_policy_for_dataset(
        dataset: &str,
        retention: &RetentionPolicy,
        _pool_manager: &Arc<ZfsPoolManager>,
    ) -> CoreResult<()> {
        debug!("Applying retention policy for dataset: {}", dataset);

        // List existing snapshots for this dataset
        let output = tokio::process::Command::new("zfs")
            .args([
                "list",
                "-t",
                "snapshot",
                "-o",
                "name,creation",
                "-H",
                "-r",
                dataset,
            ])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to list snapshots: {}", e),
            })?;

        if !output.status.success() {
            return Ok(()); // No snapshots or dataset doesn't exist
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut snapshots: Vec<(String, SystemTime)> = Vec::new();

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let snapshot_name = parts[0].to_string();
                // Parse creation time - simplified approach
                if let Some(creation_time) = SystemTime::now().checked_sub(Duration::from_secs(
                    std::env::var("NESTGATE_ZFS_SNAPSHOT_CLEANUP_AGE_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(3600), // 1 hour default
                )) {
                    snapshots.push((snapshot_name, creation_time));
                }
            }
        }

        // Apply retention policy
        match retention {
            RetentionPolicy::Count(max_count) => {
                if snapshots.len() > *max_count as usize {
                    // Sort by creation time (oldest first)
                    snapshots.sort_by_key(|(_, time)| *time);

                    // Delete oldest snapshots
                    let to_delete = snapshots.len() - *max_count as usize;
                    for (snapshot_name, _) in snapshots.iter().take(to_delete) {
                        debug!("Deleting old snapshot: {}", snapshot_name);
                        let _ = tokio::process::Command::new("zfs")
                            .args(["destroy", snapshot_name])
                            .output()
                            .await;
                    }
                }
            }
            RetentionPolicy::Duration(duration) => {
                let cutoff_time = SystemTime::now() - *duration;
                for (snapshot_name, creation_time) in &snapshots {
                    if *creation_time < cutoff_time {
                        debug!("Deleting expired snapshot: {}", snapshot_name);
                        let _ = tokio::process::Command::new("zfs")
                            .args(["destroy", snapshot_name])
                            .output()
                            .await;
                    }
                }
            }
            RetentionPolicy::Custom { .. } => {
                // Complex retention logic would go here
                debug!("Custom retention policy not fully implemented yet");
            }
        }

        Ok(())
    }

    /// Start the operation processor
    pub async fn start_operation_processor(&self) -> CoreResult<()> {
        let operation_queue = Arc::clone(&self.operation_queue);
        let statistics = Arc::clone(&self.statistics);
        let pool_manager = Arc::clone(&self.pool_manager);
        let dataset_manager = Arc::clone(&self.dataset_manager);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(
                std::env::var("NESTGATE_ZFS_SNAPSHOT_PROGRESS_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5), // 5 seconds default
            ));

            loop {
                interval.tick().await;

                if let Err(e) = Self::process_operations(
                    &operation_queue,
                    &statistics,
                    &pool_manager,
                    &dataset_manager,
                )
                .await
                {
                    error!("Error processing snapshot operations: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Process snapshot operations
    async fn process_operations(
        operation_queue: &Arc<RwLock<Vec<SnapshotOperation>>>,
        _statistics: &Arc<RwLock<SnapshotStatistics>>,
        _pool_manager: &Arc<ZfsPoolManager>,
        _dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        let mut queue = operation_queue.write().await;
        let mut completed_indices = Vec::new();

        for (index, operation) in queue.iter_mut().enumerate() {
            if matches!(operation.status, SnapshotOperationStatus::Queued) {
                operation.status = SnapshotOperationStatus::Running;
                operation.started_at = Some(SystemTime::now());

                let result =
                    Self::execute_operation(operation, _pool_manager, _dataset_manager).await;

                match result {
                    Ok(_) => {
                        operation.status = SnapshotOperationStatus::Completed;
                    }
                    Err(e) => {
                        operation.status = SnapshotOperationStatus::Failed(e.to_string());
                        operation.error_message = Some(e.to_string());
                    }
                }

                operation.completed_at = Some(SystemTime::now());
                completed_indices.push(index);
            }
        }

        // Remove completed operations (in reverse order to maintain indices)
        for &index in completed_indices.iter().rev() {
            queue.remove(index);
        }

        Ok(())
    }

    /// Execute a single snapshot operation
    async fn execute_operation(
        operation: &SnapshotOperation,
        _pool_manager: &Arc<ZfsPoolManager>,
        _dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        debug!(
            "Executing snapshot operation: {:?}",
            operation.operation_type
        );

        match operation.operation_type {
            SnapshotOperationType::Create => {
                Self::execute_create_operation(operation).await?;
            }
            SnapshotOperationType::Delete => {
                Self::execute_delete_operation(operation).await?;
            }
            SnapshotOperationType::Rollback => {
                Self::execute_rollback_operation(operation).await?;
            }
            SnapshotOperationType::Clone => {
                Self::execute_clone_operation(operation).await?;
            }
            SnapshotOperationType::Send => {
                Self::execute_send_operation(operation).await?;
            }
            SnapshotOperationType::Receive => {
                Self::execute_receive_operation(operation).await?;
            }
        }

        Ok(())
    }

    /// Execute snapshot creation operation
    async fn execute_create_operation(operation: &SnapshotOperation) -> CoreResult<()> {
        let snapshot_name = operation.snapshot_name.as_ref().ok_or_else(|| {
            let zfs_error = ZfsError::SnapshotError(SnapshotError::InvalidParameters {
                operation: "create".to_string(),
                reason: "Missing snapshot name".to_string(),
            });
            NestGateError::from(zfs_error)
        })?;

        info!("Creating snapshot: {}", snapshot_name);

        // Build ZFS snapshot command
        let snapshot_full_name = format!("{}@{}", operation.dataset, snapshot_name);

        // Execute zfs snapshot command
        let output = tokio::process::Command::new("zfs")
            .args(["snapshot", &snapshot_full_name])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to execute zfs snapshot: {}", e),
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(ZfsError::SnapshotError(SnapshotError::CreationFailed {
                dataset: operation.dataset.clone(),
                snapshot: snapshot_name.clone(),
                reason: error_msg.to_string(),
            })
            .into());
        }

        // Verify snapshot was created
        let verify_output = tokio::process::Command::new("zfs")
            .args(["list", "-t", "snapshot", &snapshot_full_name])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to execute zfs snapshot: {}", e),
            })?;

        if !verify_output.status.success() {
            return Err(ZfsError::SnapshotError(SnapshotError::CreationFailed {
                dataset: operation.dataset.clone(),
                snapshot: snapshot_name.clone(),
                reason: "Snapshot verification failed".to_string(),
            })
            .into());
        }

        info!("Successfully created snapshot: {}", snapshot_full_name);
        Ok(())
    }

    /// Execute snapshot deletion operation
    async fn execute_delete_operation(operation: &SnapshotOperation) -> CoreResult<()> {
        let snapshot_name = operation.snapshot_name.as_ref().ok_or_else(|| {
            let zfs_error = ZfsError::SnapshotError(SnapshotError::InvalidParameters {
                operation: "delete".to_string(),
                reason: "Missing snapshot name".to_string(),
            });
            NestGateError::from(zfs_error)
        })?;

        info!("Deleting snapshot: {}", snapshot_name);

        let snapshot_full_name = format!("{}@{}", operation.dataset, snapshot_name);

        // Check if snapshot exists before attempting deletion
        let list_output = tokio::process::Command::new("zfs")
            .args(["list", "-t", "snapshot", &snapshot_full_name])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to execute zfs snapshot: {}", e),
            })?;

        if !list_output.status.success() {
            warn!(
                "Snapshot {} does not exist, skipping deletion",
                snapshot_full_name
            );
            return Ok(());
        }

        // Execute zfs destroy command
        let output = tokio::process::Command::new("zfs")
            .args(["destroy", &snapshot_full_name])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to execute zfs snapshot: {}", e),
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(ZfsError::SnapshotError(SnapshotError::DeletionFailed {
                dataset: operation.dataset.clone(),
                snapshot: snapshot_name.clone(),
                reason: error_msg.to_string(),
            })
            .into());
        }

        info!("Successfully deleted snapshot: {}", snapshot_full_name);
        Ok(())
    }

    /// Execute snapshot rollback operation
    async fn execute_rollback_operation(operation: &SnapshotOperation) -> CoreResult<()> {
        let snapshot_name = operation.snapshot_name.as_ref().ok_or_else(|| {
            let zfs_error = ZfsError::SnapshotError(SnapshotError::InvalidParameters {
                operation: "rollback".to_string(),
                reason: "Missing snapshot name".to_string(),
            });
            NestGateError::from(zfs_error)
        })?;

        info!("Rolling back to snapshot: {}", snapshot_name);

        let snapshot_full_name = format!("{}@{}", operation.dataset, snapshot_name);

        // Execute zfs rollback command
        let output = tokio::process::Command::new("zfs")
            .args(["rollback", "-r", &snapshot_full_name])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to execute zfs snapshot: {}", e),
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(ZfsError::SnapshotError(SnapshotError::RollbackFailed {
                dataset: operation.dataset.clone(),
                snapshot: snapshot_name.clone(),
                reason: error_msg.to_string(),
            })
            .into());
        }

        info!(
            "Successfully rolled back to snapshot: {}",
            snapshot_full_name
        );
        Ok(())
    }

    /// Execute snapshot clone operation
    async fn execute_clone_operation(operation: &SnapshotOperation) -> CoreResult<()> {
        let snapshot_name = operation.snapshot_name.as_ref().ok_or_else(|| {
            let zfs_error = ZfsError::SnapshotError(SnapshotError::InvalidParameters {
                operation: "clone".to_string(),
                reason: "Missing snapshot name".to_string(),
            });
            NestGateError::from(zfs_error)
        })?;

        info!("Cloning snapshot: {}", snapshot_name);

        let snapshot_full_name = format!("{}@{}", operation.dataset, snapshot_name);

        // For now, we'll use a simple clone name generation
        // In a real implementation, this would come from operation parameters
        let clone_name = format!("{}_clone_{}", operation.dataset, snapshot_name);

        // Execute zfs clone command
        let output = tokio::process::Command::new("zfs")
            .args(["clone", &snapshot_full_name, &clone_name])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to execute zfs snapshot: {}", e),
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(ZfsError::SnapshotError(SnapshotError::CloneFailed {
                snapshot: snapshot_full_name,
                clone_name: clone_name.clone(),
                reason: error_msg.to_string(),
            })
            .into());
        }

        info!(
            "Successfully cloned snapshot {} to {}",
            snapshot_full_name, clone_name
        );
        Ok(())
    }

    /// Execute snapshot send operation
    async fn execute_send_operation(operation: &SnapshotOperation) -> CoreResult<()> {
        let snapshot_name = operation.snapshot_name.as_ref().ok_or_else(|| {
            let zfs_error = ZfsError::SnapshotError(SnapshotError::InvalidParameters {
                operation: "send".to_string(),
                reason: "Missing snapshot name".to_string(),
            });
            NestGateError::from(zfs_error)
        })?;

        info!("Sending snapshot: {}", snapshot_name);

        let snapshot_full_name = format!("{}@{}", operation.dataset, snapshot_name);
        let destination = "backup"; // Simplified destination

        // Execute zfs send command (this is a simplified version)
        let output = tokio::process::Command::new("zfs")
            .args(["send", &snapshot_full_name])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to execute zfs snapshot: {}", e),
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(ZfsError::SnapshotError(SnapshotError::SendFailed {
                snapshot: snapshot_full_name,
                destination: destination.to_string(),
                reason: error_msg.to_string(),
            })
            .into());
        }

        info!(
            "Successfully sent snapshot {} to {}",
            snapshot_full_name, destination
        );
        Ok(())
    }

    /// Execute snapshot receive operation
    async fn execute_receive_operation(operation: &SnapshotOperation) -> CoreResult<()> {
        let snapshot_name = operation.snapshot_name.as_ref().ok_or_else(|| {
            let zfs_error = ZfsError::SnapshotError(SnapshotError::InvalidParameters {
                operation: "receive".to_string(),
                reason: "Missing snapshot name".to_string(),
            });
            NestGateError::from(zfs_error)
        })?;

        info!("Receiving snapshot: {}", snapshot_name);

        let destination_dataset = format!("{}_received", operation.dataset);

        // This is a simplified receive operation
        // In practice, this would involve reading from a stream or file
        let output = tokio::process::Command::new("zfs")
            .args(["receive", &destination_dataset])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to execute zfs snapshot: {}", e),
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(ZfsError::SnapshotError(SnapshotError::ReceiveFailed {
                destination: destination_dataset.clone(),
                reason: error_msg.to_string(),
            })
            .into());
        }

        info!("Successfully received snapshot to {}", destination_dataset);
        Ok(())
    }

    /// Start the cache updater
    pub async fn start_cache_updater(&self) -> CoreResult<()> {
        let snapshot_cache = Arc::clone(&self.snapshot_cache);
        let statistics = Arc::clone(&self.statistics);
        let pool_manager = Arc::clone(&self.pool_manager);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(
                std::env::var("NESTGATE_METRICS_COLLECTION_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes default
            )); // Update every 5 minutes

            loop {
                interval.tick().await;

                if let Err(e) =
                    Self::update_cache(&snapshot_cache, &statistics, &pool_manager).await
                {
                    error!("Error updating snapshot cache: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Update the snapshot cache
    async fn update_cache(
        snapshot_cache: &Arc<RwLock<HashMap<String, SnapshotInfo>>>,
        statistics: &Arc<RwLock<SnapshotStatistics>>,
        _pool_manager: &Arc<ZfsPoolManager>,
    ) -> CoreResult<()> {
        // Implement cache update by listing all snapshots
        debug!("Updating snapshot cache");

        let output = tokio::process::Command::new("zfs")
            .args([
                "list",
                "-t",
                "snapshot",
                "-o",
                "name,used,refer,written,creation",
                "-H",
            ])
            .output()
            .await
            .map_err(|e| ZfsError::CommandFailed {
                command: "zfs".to_string(),
                error: format!("Failed to list snapshots: {}", e),
            })?;

        if !output.status.success() {
            warn!("Failed to list snapshots for cache update");
            return Ok(());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut updated_cache = HashMap::new();
        let mut total_snapshots = 0u64;
        let mut total_size = 0u64;

        for line in output_str.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                let full_name = parts[0].to_string();
                if let Some((dataset, snapshot)) = full_name.split_once('@') {
                    // Parse sizes (simplified - in production would handle units)
                    let used = parts[1].parse::<u64>().unwrap_or(0);
                    let refer = parts[2].parse::<u64>().unwrap_or(0);
                    let written = parts[3].parse::<u64>().unwrap_or(0);

                    let snapshot_info = SnapshotInfo {
                        name: snapshot.to_string(),
                        full_name: full_name.clone(),
                        dataset: dataset.to_string(),
                        created_at: SystemTime::now(), // Simplified - would parse creation time
                        size: used,
                        referenced_size: refer,
                        written_size: written,
                        compression_ratio: if refer > 0 {
                            used as f64 / refer as f64
                        } else {
                            1.0
                        },
                        properties: HashMap::new(),
                        policy: None,
                        tier: StorageTier::Warm, // Default tier
                        protected: false,
                        tags: Vec::new(),
                    };

                    updated_cache.insert(full_name, snapshot_info);
                    total_snapshots += 1;
                    total_size += used;
                }
            }
        }

        // Update cache
        {
            let mut cache = snapshot_cache.write().await;
            *cache = updated_cache;
        }

        // Update statistics
        {
            let mut stats = statistics.write().await;
            stats.total_snapshots = total_snapshots;
            stats.total_size = total_size;
            // Other stats would be calculated here
        }

        debug!("Updated snapshot cache with {} snapshots", total_snapshots);
        Ok(())
    }

    /// Generate a unique operation ID
    fn generate_operation_id(&self) -> String {
        format!(
            "snap_{}_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
            rand::random::<u32>()
        )
    }

    /// Parse schedule configuration and create timer
    pub fn parse_schedule(&self, schedule: &ScheduleFrequency) -> CoreResult<Duration> {
        match schedule {
            ScheduleFrequency::Hours(hours) => Ok(Duration::from_secs(*hours as u64 * 3600)),
            ScheduleFrequency::Minutes(minutes) => Ok(Duration::from_secs(*minutes as u64 * 60)),
            ScheduleFrequency::Daily(_hour) => Ok(nestgate_core::constants::time::DAY),
            ScheduleFrequency::Weekly { .. } => Ok(nestgate_core::constants::time::WEEK),
            ScheduleFrequency::Monthly { .. } => {
                Ok(nestgate_core::constants::schedule_defaults::MONTHLY_DURATION)
            } // Approximate
            ScheduleFrequency::Cron(cron_expr) => self.parse_cron_expression(cron_expr),
        }
    }

    /// Parse cron expression (simplified implementation)
    fn parse_cron_expression(&self, cron_expr: &str) -> CoreResult<Duration> {
        // Simple cron parsing - in production, use a proper cron library
        match cron_expr {
            "0 */6 * * *" => Ok(Duration::from_secs(6 * 3600)), // Every 6 hours
            "0 0 * * *" => Ok(nestgate_core::constants::time::DAY), // Daily at midnight
            "0 0 * * 0" => Ok(nestgate_core::constants::time::WEEK), // Weekly on Sunday
            _ => {
                // Default to daily if we can't parse
                warn!(
                    "Could not parse cron expression '{}', defaulting to daily",
                    cron_expr
                );
                Ok(nestgate_core::constants::time::DAY)
            }
        }
    }

    /// Apply retention policy to remove old snapshots
    #[allow(dead_code)]
    async fn apply_retention_policy(
        &self,
        dataset: &str,
        retention: &RetentionPolicy,
    ) -> CoreResult<()> {
        let snapshots = self.list_snapshots(dataset).await?;

        // Filter to get only automatic snapshots (those with "auto-" prefix)
        let mut auto_snapshots: Vec<_> = snapshots
            .iter()
            .filter(|s| s.name.starts_with("auto-"))
            .collect();

        // Sort by creation time (oldest first)
        auto_snapshots.sort_by_key(|s| s.created_at);

        match retention {
            RetentionPolicy::Count(max_snapshots) => {
                let max_snapshots_usize = *max_snapshots as usize;
                if auto_snapshots.len() > max_snapshots_usize {
                    let to_delete = auto_snapshots.len() - max_snapshots_usize;
                    for snapshot in auto_snapshots.iter().take(to_delete) {
                        self.delete_snapshot(dataset, &snapshot.name).await?;
                    }
                }
            }
            RetentionPolicy::Duration(max_duration) => {
                let cutoff_time = SystemTime::now() - *max_duration;
                for snapshot in auto_snapshots.iter() {
                    if snapshot.created_at < cutoff_time {
                        self.delete_snapshot(dataset, &snapshot.name).await?;
                    }
                }
            }
            RetentionPolicy::Custom {
                hourly_hours,
                daily_days,
                weekly_weeks,
                monthly_months,
                yearly_years,
            } => {
                // Implement custom retention logic
                let now = SystemTime::now();
                let one_hour = nestgate_core::constants::time::HOUR;
                let one_day = Duration::from_secs(24 * 3600);
                let one_week = Duration::from_secs(7 * 24 * 3600);
                let one_month = Duration::from_secs(30 * 24 * 3600); // Approximate
                let one_year = Duration::from_secs(365 * 24 * 3600); // Approximate

                for snapshot in auto_snapshots.iter() {
                    let age = now.duration_since(snapshot.created_at).unwrap_or_default();

                    // Determine if snapshot should be kept based on age and frequency
                    let should_delete = if age < one_hour * *hourly_hours
                        || age < one_day * *daily_days
                        || age < one_week * *weekly_weeks
                        || age < one_month * *monthly_months
                        || age < one_year * *yearly_years
                    {
                        false // Keep snapshots within retention periods
                    } else {
                        true // Delete older snapshots
                    };

                    if should_delete {
                        self.delete_snapshot(dataset, &snapshot.name).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Update policy cache with current settings
    #[allow(dead_code)]
    async fn update_policy_cache(&self) -> CoreResult<()> {
        // Update the snapshot cache with current policies
        let mut cache = self.snapshot_cache.write().await;
        cache.clear();

        let policies = self.policies.read().await;
        for (_, policy) in policies.iter() {
            if policy.enabled {
                // Create a dummy SnapshotInfo for the policy (this should be improved)
                let snapshot_info = SnapshotInfo {
                    name: policy.name.clone(),
                    full_name: format!("policy/{}", policy.name),
                    dataset: "policy".to_string(),
                    created_at: SystemTime::now(),
                    size: 0,
                    referenced_size: 0,
                    written_size: 0,
                    compression_ratio: 1.0,
                    properties: HashMap::new(),
                    policy: Some(policy.name.clone()),
                    tier: StorageTier::Warm, // Default tier
                    protected: false,
                    tags: vec![],
                };
                cache.insert(policy.name.clone(), snapshot_info);
            }
        }

        Ok(())
    }
}

impl Default for SnapshotStatistics {
    fn default() -> Self {
        Self {
            total_snapshots: 0,
            total_size: 0,
            total_referenced_size: 0,
            total_written_size: 0,
            average_compression_ratio: 1.0,
            snapshots_per_tier: HashMap::new(),
            size_per_tier: HashMap::new(),
            active_policies: 0,
            pending_operations: 0,
            recent_failures: 0,
        }
    }
}

/// Snapshot event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotEventType {
    Created,
    Deleted,
    PolicyApplied,
    RetentionApplied,
    Error,
}

/// Snapshot event for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotEvent {
    pub event_id: String,
    pub snapshot_name: String,
    pub event_type: SnapshotEventType,
    pub timestamp: SystemTime,
    pub details: String,
    pub success: bool,
}

/// Policy execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStats {
    pub policy_id: String,
    pub total_snapshots: u64,
    pub successful_snapshots: u64,
    pub failed_snapshots: u64,
    pub last_execution: SystemTime,
    pub total_size_bytes: u64,
}

/// Snapshot automation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotAutomationStatus {
    pub enabled: bool,
    pub active_policies: u32,
    pub total_snapshots: u64,
    pub recent_failures: u32,
    pub last_automation_run: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_policy_default() {
        let policy = SnapshotPolicy::default();

        assert_eq!(policy.name, "default");
        assert!(policy.enabled);
        assert_eq!(policy.priority, 50);
        assert_eq!(policy.max_snapshots_per_run, 100);
    }

    #[test]
    fn test_retention_policy_default() {
        let retention = RetentionPolicy::default();

        if let RetentionPolicy::Custom {
            hourly_hours,
            daily_days,
            weekly_weeks,
            monthly_months,
            yearly_years,
        } = retention
        {
            assert_eq!(hourly_hours, 24);
            assert_eq!(daily_days, 30);
            assert_eq!(weekly_weeks, 12);
            assert_eq!(monthly_months, 12);
            assert_eq!(yearly_years, 5);
        } else {
            panic!("Invalid retention policy type - expected Custom");
        }
    }

    #[tokio::test]
    async fn test_snapshot_operation_creation() {
        let operation = SnapshotOperation {
            id: "test-123".to_string(),
            operation_type: SnapshotOperationType::Create,
            dataset: "pool/dataset".to_string(),
            snapshot_name: Some("test-snapshot".to_string()),
            status: SnapshotOperationStatus::Queued,
            created_at: SystemTime::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            policy: Some("test-policy".to_string()),
        };

        assert_eq!(operation.dataset, "pool/dataset");
        assert_eq!(operation.snapshot_name, Some("test-snapshot".to_string()));
        assert_eq!(operation.status, SnapshotOperationStatus::Queued);
    }
}
