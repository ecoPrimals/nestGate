//! ZFS Snapshot Management
//!
//! Comprehensive snapshot lifecycle management with automated policies,
//! retention rules, and backup integration for production-ready ZFS systems.

use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{RwLock, mpsc};
use tokio::time::{interval, sleep};
use tracing::{debug, error, info, warn};
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Timelike, Utc};

use nestgate_core::Result as CoreResult;
use crate::{
    config::ZfsConfig,
    pool::ZfsPoolManager,
    dataset::ZfsDatasetManager,
    types::StorageTier,
    error::SnapshotError,
};

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
            hourly_hours: 24,    // 24 hours
            daily_days: 30,      // 30 days
            weekly_weeks: 12,    // 12 weeks
            monthly_months: 12,  // 12 months
            yearly_years: 5,     // 5 years
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
            let mut interval = interval(Duration::from_secs(60)); // Check every minute
            
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
        recursive: bool,
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
                if let Err(e) = Self::execute_policy(
                    policy,
                    operation_queue,
                    pool_manager,
                    dataset_manager,
                ).await {
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
            ScheduleFrequency::Minutes(minutes) => {
                // Check if enough minutes have passed since last execution
                // TODO: Track last execution time
                true
            }
            ScheduleFrequency::Hours(hours) => {
                // Check if enough hours have passed
                // TODO: Track last execution time
                true
            }
            ScheduleFrequency::Daily(hour) => {
                now.hour() == *hour as u32 && now.minute() < 5 // 5-minute window
            }
            ScheduleFrequency::Weekly { day, hour } => {
                now.weekday().number_from_monday() == *day as u32 &&
                now.hour() == *hour as u32 && now.minute() < 5
            }
            ScheduleFrequency::Monthly { day, hour } => {
                now.day() == *day as u32 &&
                now.hour() == *hour as u32 && now.minute() < 5
            }
            ScheduleFrequency::Cron(_) => {
                // TODO: Implement cron parsing
                false
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
        
        // TODO: Implement policy execution
        // This would:
        // 1. Find datasets matching the policy patterns
        // 2. Create snapshot operations for each dataset
        // 3. Add operations to the queue
        // 4. Clean up old snapshots based on retention policy
        
        Ok(())
    }

    /// Start the operation processor
    async fn start_operation_processor(&self) -> CoreResult<()> {
        let operation_queue = Arc::clone(&self.operation_queue);
        let statistics = Arc::clone(&self.statistics);
        let pool_manager = Arc::clone(&self.pool_manager);
        let dataset_manager = Arc::clone(&self.dataset_manager);
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(5));
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::process_operations(
                    &operation_queue,
                    &statistics,
                    &pool_manager,
                    &dataset_manager,
                ).await {
                    error!("Error processing snapshot operations: {}", e);
                }
            }
        });
        
        Ok(())
    }

    /// Process snapshot operations
    async fn process_operations(
        operation_queue: &Arc<RwLock<Vec<SnapshotOperation>>>,
        statistics: &Arc<RwLock<SnapshotStatistics>>,
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        let mut queue = operation_queue.write().await;
        let mut completed_indices = Vec::new();
        
        for (index, operation) in queue.iter_mut().enumerate() {
            if matches!(operation.status, SnapshotOperationStatus::Queued) {
                operation.status = SnapshotOperationStatus::Running;
                operation.started_at = Some(SystemTime::now());
                
                let result = Self::execute_operation(operation, pool_manager, dataset_manager).await;
                
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
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        match operation.operation_type {
            SnapshotOperationType::Create => {
                if let Some(snapshot_name) = &operation.snapshot_name {
                    info!("Creating snapshot: {}@{}", operation.dataset, snapshot_name);
                    // TODO: Implement actual snapshot creation
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
            SnapshotOperationType::Delete => {
                if let Some(snapshot_name) = &operation.snapshot_name {
                    info!("Deleting snapshot: {}@{}", operation.dataset, snapshot_name);
                    // TODO: Implement actual snapshot deletion
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
            }
            _ => {
                // TODO: Implement other operation types
            }
        }
        
        Ok(())
    }

    /// Start the cache updater
    async fn start_cache_updater(&self) -> CoreResult<()> {
        let snapshot_cache = Arc::clone(&self.snapshot_cache);
        let statistics = Arc::clone(&self.statistics);
        let pool_manager = Arc::clone(&self.pool_manager);
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(300)); // Update every 5 minutes
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::update_cache(
                    &snapshot_cache,
                    &statistics,
                    &pool_manager,
                ).await {
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
        pool_manager: &Arc<ZfsPoolManager>,
    ) -> CoreResult<()> {
        debug!("Updating snapshot cache");
        
        // TODO: Implement cache update
        // This would:
        // 1. Query ZFS for current snapshots
        // 2. Update the cache with new snapshot information
        // 3. Update statistics
        
        Ok(())
    }

    /// Generate a unique operation ID
    fn generate_operation_id(&self) -> String {
        format!("snap_{}_{}", 
            SystemTime::now().duration_since(UNIX_EPOCH)
                .unwrap_or_default().as_millis(),
            rand::random::<u32>()
        )
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
            yearly_years 
        } = retention {
            assert_eq!(hourly_hours, 24);
            assert_eq!(daily_days, 30);
            assert_eq!(weekly_weeks, 12);
            assert_eq!(monthly_months, 12);
            assert_eq!(yearly_years, 5);
        } else {
            panic!("Expected Custom retention policy");
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