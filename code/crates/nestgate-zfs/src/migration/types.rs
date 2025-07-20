//! ZFS Migration Types - Data structures and enums for migration system
//!
//! Contains all the core data structures used by the migration system including
//! job definitions, configurations, statistics, and processing contexts.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{RwLock, Semaphore};
use uuid;

use crate::{dataset::ZfsDatasetManager, pool::ZfsPoolManager, types::StorageTier};

/// Migration job status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MigrationStatus {
    /// Job is queued for execution
    Queued,
    /// Job is currently running
    Running,
    /// Job completed successfully
    Completed,
    /// Job failed with error
    Failed(String),
    /// Job was cancelled
    Cancelled,
    /// Job is paused
    Paused,
}

/// Migration job priority
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MigrationPriority {
    /// Low priority - background migration
    Low = 1,
    /// Normal priority - standard migration
    Normal = 2,
    /// High priority - user-requested migration
    High = 3,
    /// Critical priority - emergency migration
    Critical = 4,
}

/// Migration job definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationJob {
    /// Unique job identifier
    pub id: String,
    /// Source file path
    pub source_path: PathBuf,
    /// Source tier
    pub source_tier: StorageTier,
    /// Target tier
    pub target_tier: StorageTier,
    /// Job priority
    pub priority: MigrationPriority,
    /// Job status
    pub status: MigrationStatus,
    /// Creation time
    pub created_at: SystemTime,
    /// Started time
    pub started_at: Option<SystemTime>,
    /// Completed time
    pub completed_at: Option<SystemTime>,
    /// File size in bytes
    pub file_size: u64,
    /// Progress percentage (0-100)
    pub progress: f64,
    /// Transfer rate in bytes per second
    pub transfer_rate: f64,
    /// Estimated time remaining in seconds
    pub eta_seconds: Option<u64>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Retry count
    pub retry_count: u32,
    /// Maximum retries allowed
    pub max_retries: u32,
    /// Job metadata (e.g., for performance tracking)
    pub metadata: HashMap<String, String>,
}

/// Enhanced migration job with performance optimizations
impl MigrationJob {
    /// Create a new migration job with optimized settings
    pub fn new(
        source_path: PathBuf,
        source_tier: StorageTier,
        target_tier: StorageTier,
        priority: MigrationPriority,
        file_size: u64,
    ) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = SystemTime::now();

        Self {
            id,
            source_path,
            source_tier,
            target_tier,
            priority,
            file_size,
            status: MigrationStatus::Queued,
            created_at,
            started_at: None,
            completed_at: None,
            progress: 0.0,
            transfer_rate: 0.0,
            eta_seconds: None,
            error_message: None,
            retry_count: 0,
            max_retries: 3,
            metadata: HashMap::new(),
        }
    }

    /// Determine optimal migration priority based on tier transition
    pub fn get_optimal_priority(
        source_tier: &StorageTier,
        target_tier: &StorageTier,
    ) -> MigrationPriority {
        match (source_tier, target_tier) {
            // Cache operations = critical priority (check this first)
            (StorageTier::Cache, _) | (_, StorageTier::Cache) => MigrationPriority::Critical,
            // Moving to hot tier = high priority for performance
            (_, StorageTier::Hot) => MigrationPriority::High,
            // Moving from hot tier = normal priority unless to cold
            (StorageTier::Hot, StorageTier::Cold) => MigrationPriority::High, // Free up hot storage
            (StorageTier::Hot, _) => MigrationPriority::Normal,
            // Moving to cold tier = low priority (archival)
            (_, StorageTier::Cold) => MigrationPriority::Low,
            // Default = normal priority
            _ => MigrationPriority::Normal,
        }
    }
}

/// Migration engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Maximum concurrent migrations
    pub max_concurrent_migrations: usize,
    /// Maximum bandwidth per migration in bytes per second
    pub max_bandwidth_per_migration: u64,
    /// Total bandwidth limit in bytes per second
    pub total_bandwidth_limit: u64,
    /// Migration schedule (hours when migration is allowed)
    pub allowed_hours: Vec<u8>,
    /// Performance impact threshold (0.0-1.0)
    pub performance_impact_threshold: f64,
    /// Minimum free space required per tier (percentage)
    pub min_free_space_percent: f64,
    /// Migration batch size
    pub batch_size: usize,
    /// Progress update interval in seconds
    pub progress_update_interval: u64,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            max_concurrent_migrations: 3,
            max_bandwidth_per_migration: 100 * 1024 * 1024, // 100 MB/s
            total_bandwidth_limit: 200 * 1024 * 1024,       // 200 MB/s
            allowed_hours: (0..24).collect(),               // All hours allowed by default
            performance_impact_threshold: 0.05,             // 5% impact threshold
            min_free_space_percent: 10.0,                   // 10% minimum free space
            batch_size: 10,
            progress_update_interval: 5, // 5 seconds
        }
    }
}

/// Migration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatistics {
    /// Total jobs processed
    pub total_jobs: u64,
    /// Successful migrations
    pub successful_migrations: u64,
    /// Failed migrations
    pub failed_migrations: u64,
    /// Total bytes migrated
    pub total_bytes_migrated: u64,
    /// Average migration time in seconds
    pub average_migration_time: f64,
    /// Average transfer rate in bytes per second
    pub average_transfer_rate: f64,
    /// Current active migrations
    pub active_migrations: u32,
    /// Queued migrations
    pub queued_migrations: u32,
    /// Completed migrations
    pub completed_migrations: u64,
    /// Average migration time in seconds
    pub average_migration_time_seconds: f64,
    /// Success rate
    pub success_rate: f64,
}

impl Default for MigrationStatistics {
    fn default() -> Self {
        Self {
            total_jobs: 0,
            successful_migrations: 0,
            failed_migrations: 0,
            total_bytes_migrated: 0,
            average_migration_time: 0.0,
            average_transfer_rate: 0.0,
            active_migrations: 0,
            queued_migrations: 0,
            completed_migrations: 0,
            average_migration_time_seconds: 0.0,
            success_rate: 1.0,
        }
    }
}

/// Migration queue processing context
pub struct MigrationContext<'a> {
    pub job_queue: &'a Arc<RwLock<VecDeque<MigrationJob>>>,
    pub active_migrations: &'a Arc<RwLock<HashMap<String, MigrationJob>>>,
    pub migration_history: &'a Arc<RwLock<Vec<MigrationJob>>>,
    pub statistics: &'a Arc<RwLock<MigrationStatistics>>,
    pub migration_semaphore: &'a Arc<Semaphore>,
    pub config: &'a MigrationConfig,
    pub pool_manager: &'a Arc<ZfsPoolManager>,
    pub dataset_manager: &'a Arc<ZfsDatasetManager>,
}
