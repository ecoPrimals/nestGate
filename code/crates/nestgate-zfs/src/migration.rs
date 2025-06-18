//! ZFS Tier Migration Engine
//!
//! Automated data migration system for moving files between storage tiers
//! based on access patterns, performance requirements, and system policies.

use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::{RwLock, Semaphore, mpsc};
use tokio::time::{interval, sleep};
use tracing::{debug, error, info, warn};
use serde::{Serialize, Deserialize};
use chrono::Timelike;

use nestgate_core::{Result as CoreResult, NestGateError};
use crate::{
    config::ZfsConfig,
    pool::ZfsPoolManager,
    dataset::ZfsDatasetManager,
    automation::DatasetAnalyzer,
    types::StorageTier,
    error::MigrationError,
};

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
}

impl MigrationJob {
    /// Create a new migration job
    pub fn new(
        source_path: PathBuf,
        source_tier: StorageTier,
        target_tier: StorageTier,
        priority: MigrationPriority,
        file_size: u64,
    ) -> Self {
        let id = format!("mig_{}_{}", 
            SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default().as_millis(),
            rand::random::<u32>()
        );
        
        Self {
            id,
            source_path,
            source_tier,
            target_tier,
            priority,
            status: MigrationStatus::Queued,
            created_at: SystemTime::now(),
            started_at: None,
            completed_at: None,
            file_size,
            progress: 0.0,
            transfer_rate: 0.0,
            eta_seconds: None,
            error_message: None,
            retry_count: 0,
            max_retries: 3,
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
            allowed_hours: (0..24).collect(), // All hours allowed by default
            performance_impact_threshold: 0.05, // 5% impact threshold
            min_free_space_percent: 10.0,      // 10% minimum free space
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
}

/// Migration engine for automated tier-to-tier data movement
#[derive(Debug)]
pub struct MigrationEngine {
    config: MigrationConfig,
    zfs_config: ZfsConfig,
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    analyzer: Arc<DatasetAnalyzer>,
    
    /// Migration job queue
    job_queue: Arc<RwLock<VecDeque<MigrationJob>>>,
    /// Active migrations
    active_migrations: Arc<RwLock<HashMap<String, MigrationJob>>>,
    /// Migration history
    migration_history: Arc<RwLock<Vec<MigrationJob>>>,
    /// Migration statistics
    statistics: Arc<RwLock<MigrationStatistics>>,
    
    /// Concurrency control
    migration_semaphore: Arc<Semaphore>,
    /// Bandwidth control
    bandwidth_semaphore: Arc<Semaphore>,
    
    /// Shutdown signal
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl MigrationEngine {
    /// Create a new migration engine
    pub fn new(
        config: MigrationConfig,
        zfs_config: ZfsConfig,
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
        analyzer: Arc<DatasetAnalyzer>,
    ) -> Self {
        let migration_semaphore = Arc::new(Semaphore::new(config.max_concurrent_migrations));
        let bandwidth_semaphore = Arc::new(Semaphore::new(config.total_bandwidth_limit as usize));
        
        Self {
            config,
            zfs_config,
            pool_manager,
            dataset_manager,
            analyzer,
            job_queue: Arc::new(RwLock::new(VecDeque::new())),
            active_migrations: Arc::new(RwLock::new(HashMap::new())),
            migration_history: Arc::new(RwLock::new(Vec::new())),
            statistics: Arc::new(RwLock::new(MigrationStatistics::default())),
            migration_semaphore,
            bandwidth_semaphore,
            shutdown_tx: None,
        }
    }

    /// Start the migration engine
    pub async fn start(&mut self) -> CoreResult<()> {
        info!("Starting migration engine");
        
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);
        
        // Start migration worker
        let job_queue = Arc::clone(&self.job_queue);
        let active_migrations = Arc::clone(&self.active_migrations);
        let migration_history = Arc::clone(&self.migration_history);
        let statistics = Arc::clone(&self.statistics);
        let migration_semaphore = Arc::clone(&self.migration_semaphore);
        let bandwidth_semaphore = Arc::clone(&self.bandwidth_semaphore);
        let config = self.config.clone();
        let pool_manager = Arc::clone(&self.pool_manager);
        let dataset_manager = Arc::clone(&self.dataset_manager);
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Err(e) = Self::process_migration_queue(
                            &job_queue,
                            &active_migrations,
                            &migration_history,
                            &statistics,
                            &migration_semaphore,
                            &bandwidth_semaphore,
                            &config,
                            &pool_manager,
                            &dataset_manager,
                        ).await {
                            error!("Error processing migration queue: {}", e);
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        info!("Migration engine shutting down");
                        break;
                    }
                }
            }
        });
        
        // Start automatic migration discovery
        self.start_automatic_migration_discovery().await?;
        
        info!("Migration engine started successfully");
        Ok(())
    }

    /// Stop the migration engine
    pub async fn stop(&mut self) -> CoreResult<()> {
        info!("Stopping migration engine");
        
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
        }
        
        // Wait for active migrations to complete
        let mut retries = 0;
        while retries < 30 { // Wait up to 30 seconds
            let active_count = self.active_migrations.read().await.len();
            if active_count == 0 {
                break;
            }
            
            info!("Waiting for {} active migrations to complete", active_count);
            sleep(Duration::from_secs(1)).await;
            retries += 1;
        }
        
        info!("Migration engine stopped");
        Ok(())
    }

    /// Queue a migration job
    pub async fn queue_migration(
        &self,
        source_path: PathBuf,
        target_tier: StorageTier,
        priority: MigrationPriority,
    ) -> CoreResult<String> {
        debug!("Queuing migration: {:?} -> {:?}", source_path, target_tier);
        
        // Analyze file to determine source tier and characteristics
        let characteristics = self.analyzer.analyze_file(&source_path).await?;
        let recommendation = self.analyzer.recommend_tier(&characteristics).await?;
        let source_tier = recommendation.tier;
        
        // Create migration job
        let job = MigrationJob::new(
            source_path,
            source_tier,
            target_tier,
            priority,
            characteristics.size,
        );
        
        let job_id = job.id.clone();
        
        // Add to queue
        let mut queue = self.job_queue.write().await;
        queue.push_back(job);
        
        // Sort queue by priority
        let mut jobs: Vec<_> = queue.drain(..).collect();
        jobs.sort_by(|a, b| b.priority.cmp(&a.priority));
        queue.extend(jobs);
        
        // Update statistics
        let mut stats = self.statistics.write().await;
        stats.total_jobs += 1;
        stats.queued_migrations += 1;
        
        info!("Migration queued: {} (priority: {:?})", job_id, priority);
        Ok(job_id)
    }

    /// Get migration job status
    pub async fn get_job_status(&self, job_id: &str) -> CoreResult<Option<MigrationJob>> {
        // Check active migrations first
        let active = self.active_migrations.read().await;
        if let Some(job) = active.get(job_id) {
            return Ok(Some(job.clone()));
        }
        
        // Check queue
        let queue = self.job_queue.read().await;
        for job in queue.iter() {
            if job.id == job_id {
                return Ok(Some(job.clone()));
            }
        }
        
        // Check history
        let history = self.migration_history.read().await;
        for job in history.iter() {
            if job.id == job_id {
                return Ok(Some(job.clone()));
            }
        }
        
        Ok(None)
    }

    /// Cancel a migration job
    pub async fn cancel_migration(&self, job_id: &str) -> CoreResult<bool> {
        info!("Cancelling migration: {}", job_id);
        
        // Try to remove from queue first
        let mut queue = self.job_queue.write().await;
        if let Some(pos) = queue.iter().position(|job| job.id == job_id) {
            let mut job = queue.remove(pos).unwrap();
            job.status = MigrationStatus::Cancelled;
            job.completed_at = Some(SystemTime::now());
            
            // Move to history
            let mut history = self.migration_history.write().await;
            history.push(job);
            
            return Ok(true);
        }
        
        // Check if it's an active migration
        let mut active = self.active_migrations.write().await;
        if let Some(mut job) = active.remove(job_id) {
            job.status = MigrationStatus::Cancelled;
            job.completed_at = Some(SystemTime::now());
            
            // Move to history
            let mut history = self.migration_history.write().await;
            history.push(job);
            
            return Ok(true);
        }
        
        Ok(false)
    }

    /// Get migration statistics
    pub async fn get_statistics(&self) -> MigrationStatistics {
        self.statistics.read().await.clone()
    }

    /// Get all active migrations
    pub async fn get_active_migrations(&self) -> Vec<MigrationJob> {
        self.active_migrations.read().await.values().cloned().collect()
    }

    /// Get queued migrations
    pub async fn get_queued_migrations(&self) -> Vec<MigrationJob> {
        self.job_queue.read().await.iter().cloned().collect()
    }

    /// Start automatic migration discovery based on access patterns
    async fn start_automatic_migration_discovery(&self) -> CoreResult<()> {
        let analyzer = Arc::clone(&self.analyzer);
        let job_queue = Arc::clone(&self.job_queue);
        let statistics = Arc::clone(&self.statistics);
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(3600)); // Check every hour
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::discover_migration_candidates(
                    &analyzer,
                    &job_queue,
                    &statistics,
                ).await {
                    error!("Error discovering migration candidates: {}", e);
                }
            }
        });
        
        Ok(())
    }

    /// Discover files that should be migrated based on access patterns
    async fn discover_migration_candidates(
        analyzer: &Arc<DatasetAnalyzer>,
        job_queue: &Arc<RwLock<VecDeque<MigrationJob>>>,
        statistics: &Arc<RwLock<MigrationStatistics>>,
    ) -> CoreResult<()> {
        debug!("Discovering migration candidates");
        
        // TODO: Implement file system scanning and analysis
        // This would scan all tiers and identify files that should be migrated
        // based on their access patterns and current tier assignment
        
        Ok(())
    }

    /// Process the migration queue
    async fn process_migration_queue(
        job_queue: &Arc<RwLock<VecDeque<MigrationJob>>>,
        active_migrations: &Arc<RwLock<HashMap<String, MigrationJob>>>,
        migration_history: &Arc<RwLock<Vec<MigrationJob>>>,
        statistics: &Arc<RwLock<MigrationStatistics>>,
        migration_semaphore: &Arc<Semaphore>,
        bandwidth_semaphore: &Arc<Semaphore>,
        config: &MigrationConfig,
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        // Check if we can start new migrations
        if migration_semaphore.available_permits() == 0 {
            return Ok(()); // No available slots
        }
        
        // Get next job from queue
        let job = {
            let mut queue = job_queue.write().await;
            queue.pop_front()
        };
        
        if let Some(mut job) = job {
            // Check if migration is allowed at this time
            let current_hour = chrono::Local::now().hour() as u8;
            if !config.allowed_hours.contains(&current_hour) {
                // Put job back in queue
                let mut queue = job_queue.write().await;
                queue.push_front(job);
                return Ok(());
            }
            
            // Acquire migration permit
            let _permit = migration_semaphore.acquire().await
                .map_err(|e| NestGateError::Internal(format!("Failed to acquire migration permit: {}", e)))?;
            
            // Start migration
            job.status = MigrationStatus::Running;
            job.started_at = Some(SystemTime::now());
            
            let job_id = job.id.clone();
            
            // Add to active migrations
            {
                let mut active = active_migrations.write().await;
                active.insert(job_id.clone(), job.clone());
            }
            
            // Update statistics
            {
                let mut stats = statistics.write().await;
                stats.active_migrations += 1;
                stats.queued_migrations = stats.queued_migrations.saturating_sub(1);
            }
            
            // Spawn migration task
            let job_clone = job.clone();
            let active_migrations_clone = Arc::clone(active_migrations);
            let migration_history_clone = Arc::clone(migration_history);
            let statistics_clone = Arc::clone(statistics);
            let pool_manager_clone = Arc::clone(pool_manager);
            let dataset_manager_clone = Arc::clone(dataset_manager);
            
            tokio::spawn(async move {
                let result = Self::execute_migration(
                    job_clone,
                    &pool_manager_clone,
                    &dataset_manager_clone,
                ).await;
                
                // Handle migration result
                let mut final_job = {
                    let mut active = active_migrations_clone.write().await;
                    active.remove(&job_id).unwrap()
                };
                
                match result {
                    Ok(_) => {
                        final_job.status = MigrationStatus::Completed;
                        final_job.progress = 100.0;
                        
                        let mut stats = statistics_clone.write().await;
                        stats.successful_migrations += 1;
                        stats.total_bytes_migrated += final_job.file_size;
                    }
                    Err(e) => {
                        final_job.status = MigrationStatus::Failed(e.to_string());
                        final_job.error_message = Some(e.to_string());
                        
                        let mut stats = statistics_clone.write().await;
                        stats.failed_migrations += 1;
                    }
                }
                
                final_job.completed_at = Some(SystemTime::now());
                
                // Update statistics
                {
                    let mut stats = statistics_clone.write().await;
                    stats.active_migrations = stats.active_migrations.saturating_sub(1);
                }
                
                // Move to history
                let mut history = migration_history_clone.write().await;
                history.push(final_job);
                
                // Keep history size manageable
                if history.len() > 1000 {
                    history.drain(0..100); // Remove oldest 100 entries
                }
            });
        }
        
        Ok(())
    }

    /// Execute a single migration
    async fn execute_migration(
        mut job: MigrationJob,
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        info!("Executing migration: {} -> {:?}", job.source_path.display(), job.target_tier);
        
        let start_time = Instant::now();
        
        // TODO: Implement actual file migration logic
        // This would involve:
        // 1. Ensuring target dataset exists
        // 2. Copying file to target tier
        // 3. Updating file metadata
        // 4. Removing file from source tier
        // 5. Updating access pattern records
        
        // Simulate migration for now
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let duration = start_time.elapsed();
        let transfer_rate = job.file_size as f64 / duration.as_secs_f64();
        
        info!("Migration completed: {} ({:.2} MB/s)", 
              job.id, transfer_rate / (1024.0 * 1024.0));
        
        Ok(())
    }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_migration_job_creation() {
        let source_path = PathBuf::from("/test/file.txt");
        let job = MigrationJob::new(
            source_path.clone(),
            StorageTier::Hot,
            StorageTier::Warm,
            MigrationPriority::Normal,
            1024,
        );
        
        assert_eq!(job.source_path, source_path);
        assert_eq!(job.source_tier, StorageTier::Hot);
        assert_eq!(job.target_tier, StorageTier::Warm);
        assert_eq!(job.priority, MigrationPriority::Normal);
        assert_eq!(job.file_size, 1024);
        assert_eq!(job.status, MigrationStatus::Queued);
        assert_eq!(job.progress, 0.0);
    }
    
    #[tokio::test]
    async fn test_migration_config_default() {
        let config = MigrationConfig::default();
        
        assert_eq!(config.max_concurrent_migrations, 3);
        assert_eq!(config.max_bandwidth_per_migration, 100 * 1024 * 1024);
        assert_eq!(config.total_bandwidth_limit, 200 * 1024 * 1024);
        assert_eq!(config.allowed_hours.len(), 24);
        assert_eq!(config.performance_impact_threshold, 0.05);
        assert_eq!(config.min_free_space_percent, 10.0);
    }
} 