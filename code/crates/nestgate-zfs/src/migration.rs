//! ZFS Tier Migration Engine
//!
//! Automated data migration system for moving files between storage tiers
//! based on access patterns, performance requirements, and system policies.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
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
    /// Completed migrations
    pub completed_migrations: u64,
    /// Average migration time in seconds
    pub average_migration_time_seconds: f64,
    /// Success rate
    pub success_rate: f64,
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
        
        // Create shutdown channel
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        self.shutdown_tx = Some(shutdown_tx);
        
        // Start automatic migration discovery
        self.start_automatic_migration_discovery().await?;
        
        // Start migration queue processor
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
            let mut interval = interval(Duration::from_secs(config.progress_update_interval));
            
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
        
        info!("Migration engine started successfully");
        Ok(())
    }

    /// Stop the migration engine
    pub async fn stop(&mut self) -> CoreResult<()> {
        info!("Stopping migration engine");
        
        // Send shutdown signal
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            if let Err(e) = shutdown_tx.send(()).await {
                warn!("Failed to send shutdown signal: {}", e);
            }
        }
        
        // Wait for active migrations to complete (with timeout)
        let timeout_duration = Duration::from_secs(30);
        let start_time = Instant::now();
        
        while start_time.elapsed() < timeout_duration {
            let active_count = {
                let active = self.active_migrations.read().await;
                active.len()
            };
            
            if active_count == 0 {
                break;
            }
            
            info!("Waiting for {} active migrations to complete...", active_count);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        
        // Cancel any remaining active migrations
        let active_jobs: Vec<String> = {
            let active = self.active_migrations.read().await;
            active.keys().cloned().collect()
        };
        
        for job_id in active_jobs {
            if let Err(e) = self.cancel_migration(&job_id).await {
                warn!("Failed to cancel migration {}: {}", job_id, e);
            }
        }
        
        // Clear the job queue
        {
            let mut queue = self.job_queue.write().await;
            let remaining_jobs = queue.len();
            if remaining_jobs > 0 {
                info!("Clearing {} remaining jobs from queue", remaining_jobs);
                queue.clear();
            }
        }
        
        info!("Migration engine stopped successfully");
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
        let characteristics = self.analyzer.analyze_file(&source_path.to_string_lossy()).await
            .map_err(|e| NestGateError::Internal(format!("File analysis failed: {}", e)))?;
        let recommendation = self.analyzer.recommend_tier(&characteristics).await
            .map_err(|e| NestGateError::Internal(format!("Tier recommendation failed: {}", e)))?;
        
        // Convert from nestgate_core::StorageTier to types::StorageTier
        let source_tier = match recommendation {
            nestgate_core::StorageTier::Hot => crate::types::StorageTier::Hot,
            nestgate_core::StorageTier::Warm => crate::types::StorageTier::Warm,
            nestgate_core::StorageTier::Cold => crate::types::StorageTier::Cold,
            nestgate_core::StorageTier::Cache => crate::types::StorageTier::Hot, // Map Cache to Hot
        };
        
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
            if let Some(mut job) = queue.remove(pos) {
                job.status = MigrationStatus::Cancelled;
                job.completed_at = Some(SystemTime::now());
                
                // Move to history
                let mut history = self.migration_history.write().await;
                history.push(job);
                
                return Ok(true);
            }
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
        
        // Scan all tier directories for files
        let tier_paths = vec![
            ("/mnt/storage/hot", StorageTier::Hot),
            ("/mnt/storage/warm", StorageTier::Warm),
            ("/mnt/storage/cold", StorageTier::Cold),
        ];
        
        let mut candidates = Vec::new();
        
        for (tier_path, current_tier) in tier_paths {
            if let Ok(entries) = Self::scan_directory_for_files(tier_path).await {
                for file_path in entries {
                    match Self::analyze_migration_candidate(&file_path, current_tier, analyzer).await {
                        Ok(Some(recommended_tier)) if recommended_tier != current_tier => {
                            candidates.push((file_path, current_tier, recommended_tier));
                        }
                        Ok(_) => {
                            // File is in correct tier or no recommendation
                        }
                        Err(e) => {
                            debug!("Failed to analyze file {:?}: {}", file_path, e);
                        }
                    }
                }
            }
        }
        
        // Queue migration jobs for candidates
        let mut queued_count = 0;
        for (file_path, _current_tier, recommended_tier) in candidates {
            // Check if file size is reasonable for migration
            if let Ok(metadata) = tokio::fs::metadata(&file_path).await {
                let file_size = metadata.len();
                
                // Skip very small files (< 1MB) or very large files (> 10GB) for automatic migration
                if file_size < 1024 * 1024 || file_size > 10 * 1024 * 1024 * 1024 {
                    continue;
                }
                
                // Create migration job
                let job = MigrationJob::new(
                    file_path,
                    _current_tier,
                    recommended_tier,
                    MigrationPriority::Low, // Automatic migrations are low priority
                    file_size,
                );
                
                // Add to queue
                let mut queue = job_queue.write().await;
                queue.push_back(job);
                queued_count += 1;
                
                // Limit automatic discovery to prevent queue overflow
                if queued_count >= 50 {
                    break;
                }
            }
        }
        
        if queued_count > 0 {
            info!("Discovered and queued {} migration candidates", queued_count);
            
            // Update statistics
            let mut stats = statistics.write().await;
            stats.queued_migrations += queued_count as u32;
        }
        
        Ok(())
    }
    
    /// Scan directory for files recursively
    async fn scan_directory_for_files(dir_path: &str) -> CoreResult<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        let path = PathBuf::from(dir_path);
        if !path.exists() {
            return Ok(files);
        }
        
        Self::scan_directory_recursive(path, &mut files, 0).await?;
        Ok(files)
    }
    
    /// Recursive directory scanning helper
    fn scan_directory_recursive(
        dir_path: PathBuf,
        files: &mut Vec<PathBuf>,
        depth: usize,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = CoreResult<()>> + Send + '_>> {
        Box::pin(async move {
            // Limit recursion depth to prevent infinite loops
            if depth > 10 {
                return Ok(());
            }
            
            let mut entries = tokio::fs::read_dir(&dir_path).await
                .map_err(|e| NestGateError::Storage(format!("Failed to read directory {:?}: {}", dir_path, e)))?;
            
            while let Some(entry) = entries.next_entry().await
                .map_err(|e| NestGateError::Storage(format!("Failed to read directory entry: {}", e)))? {
                
                let path = entry.path();
                
                if path.is_file() {
                    files.push(path);
                    
                    // Limit total files to prevent memory issues
                    if files.len() >= 1000 {
                        break;
                    }
                } else if path.is_dir() {
                    Self::scan_directory_recursive(path, files, depth + 1).await?;
                }
            }
            
            Ok(())
        })
    }
    
    /// Analyze a file to determine if it should be migrated
    async fn analyze_migration_candidate(
        file_path: &PathBuf,
        _current_tier: StorageTier,
        analyzer: &Arc<DatasetAnalyzer>,
    ) -> CoreResult<Option<StorageTier>> {
        // Analyze file characteristics
        let characteristics = analyzer.analyze_file(&file_path.to_string_lossy()).await
            .map_err(|e| NestGateError::Internal(format!("File analysis failed: {}", e)))?;
        
        // Get tier recommendation
        let recommendation = analyzer.recommend_tier(&characteristics).await
            .map_err(|e| NestGateError::Internal(format!("Tier recommendation failed: {}", e)))?;
        
        // Convert from nestgate_core::StorageTier to types::StorageTier
        let recommended_tier = match recommendation {
            nestgate_core::StorageTier::Hot => StorageTier::Hot,
            nestgate_core::StorageTier::Warm => StorageTier::Warm,
            nestgate_core::StorageTier::Cold => StorageTier::Cold,
            nestgate_core::StorageTier::Cache => StorageTier::Hot, // Map Cache to Hot
        };
        
        Ok(Some(recommended_tier))
    }

    /// Process the migration queue
    async fn process_migration_queue(
        job_queue: &Arc<RwLock<VecDeque<MigrationJob>>>,
        active_migrations: &Arc<RwLock<HashMap<String, MigrationJob>>>,
        migration_history: &Arc<RwLock<Vec<MigrationJob>>>,
        statistics: &Arc<RwLock<MigrationStatistics>>,
        migration_semaphore: &Arc<Semaphore>,
        _bandwidth_semaphore: &Arc<Semaphore>,
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
                    match active.remove(&job_id) {
                        Some(job) => job,
                        None => {
                            error!("Migration job {} not found in active migrations", job_id);
                            return;
                        }
                    }
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
        _pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        info!("Executing migration: {} -> {:?}", job.source_path.display(), job.target_tier);
        
        let start_time = Instant::now();
        
        // 1. Validate source file exists
        if !job.source_path.exists() {
            return Err(NestGateError::Storage(format!("Source file does not exist: {:?}", job.source_path)));
        }
        
        // 2. Get target dataset path based on tier
        let target_dataset = Self::get_target_dataset_for_tier(&job.target_tier)?;
        let target_path = Self::construct_target_path(&job.source_path, &target_dataset)?;
        
        // 3. Ensure target dataset exists
        Self::ensure_target_dataset_exists(&target_dataset, &job.target_tier, dataset_manager).await?;
        
        // 4. Ensure target directory exists
        if let Some(parent) = target_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| NestGateError::Storage(format!("Failed to create target directory: {}", e)))?;
        }
        
        // 5. Copy file to target tier with progress tracking
        let source_path_clone = job.source_path.clone();
        Self::copy_file_with_progress(&source_path_clone, &target_path, &mut job).await?;
        
        // 6. Verify copy integrity
        Self::verify_file_integrity(&job.source_path, &target_path).await?;
        
        // 7. Update file metadata and access patterns
        Self::update_file_metadata(&target_path, &job).await?;
        
        // 8. Remove file from source tier (only if different from target)
        if Self::get_tier_from_path(&job.source_path)? != job.target_tier {
            tokio::fs::remove_file(&job.source_path).await
                .map_err(|e| NestGateError::Storage(format!("Failed to remove source file: {}", e)))?;
        }
        
        let duration = start_time.elapsed();
        let transfer_rate = job.file_size as f64 / duration.as_secs_f64();
        
        info!("Migration completed: {} ({:.2} MB/s)", 
              job.id, transfer_rate / (1024.0 * 1024.0));
        
        Ok(())
    }
    
    /// Get target dataset name for a tier
    fn get_target_dataset_for_tier(tier: &StorageTier) -> CoreResult<String> {
        match tier {
            StorageTier::Hot => Ok("storage/hot".to_string()),
            StorageTier::Warm => Ok("storage/warm".to_string()),
            StorageTier::Cold => Ok("storage/cold".to_string()),
            StorageTier::Cache => Ok("storage/cache".to_string()),
        }
    }
    
    /// Construct target path based on source path and target dataset
    fn construct_target_path(source_path: &PathBuf, target_dataset: &str) -> CoreResult<PathBuf> {
        // Extract relative path from source
        let file_name = source_path.file_name()
            .ok_or_else(|| NestGateError::Storage("Invalid source file path".to_string()))?;
        
        // Construct target path: /mnt/{dataset}/{filename}
        let target_path = PathBuf::from("/mnt").join(target_dataset).join(file_name);
        Ok(target_path)
    }
    
    /// Ensure target dataset exists
    async fn ensure_target_dataset_exists(
        dataset_name: &str,
        tier: &StorageTier,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<()> {
        // Check if dataset exists
        let datasets = dataset_manager.list_datasets().await
            .map_err(|e| NestGateError::Storage(format!("Failed to list datasets: {}", e)))?;
        
        let dataset_exists = datasets.iter().any(|d| d.name == dataset_name);
        
        if !dataset_exists {
            info!("Creating target dataset: {}", dataset_name);
            
            // Create dataset with appropriate properties for the tier
            let mut properties = std::collections::HashMap::new();
            
            match tier {
                StorageTier::Hot => {
                    properties.insert("compression".to_string(), "lz4".to_string());
                    properties.insert("recordsize".to_string(), "128K".to_string());
                }
                StorageTier::Warm => {
                    properties.insert("compression".to_string(), "gzip".to_string());
                    properties.insert("recordsize".to_string(), "1M".to_string());
                }
                StorageTier::Cold => {
                    properties.insert("compression".to_string(), "gzip-9".to_string());
                    properties.insert("recordsize".to_string(), "1M".to_string());
                }
                StorageTier::Cache => {
                    properties.insert("compression".to_string(), "off".to_string());
                    properties.insert("recordsize".to_string(), "64K".to_string());
                }
            }
            
            // Convert tier to nestgate_core::StorageTier
            let core_tier = match tier {
                StorageTier::Hot => nestgate_core::StorageTier::Hot,
                StorageTier::Warm => nestgate_core::StorageTier::Warm,
                StorageTier::Cold => nestgate_core::StorageTier::Cold,
                StorageTier::Cache => nestgate_core::StorageTier::Cache,
            };
            
            dataset_manager.create_dataset(dataset_name, "storage", core_tier).await
                .map_err(|e| NestGateError::Storage(format!("Failed to create dataset: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Copy file with progress tracking
    async fn copy_file_with_progress(
        source_path: &PathBuf,
        target_path: &PathBuf,
        job: &mut MigrationJob,
    ) -> CoreResult<()> {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        
        let mut source_file = tokio::fs::File::open(source_path).await
            .map_err(|e| NestGateError::Storage(format!("Failed to open source file: {}", e)))?;
        
        let mut target_file = tokio::fs::File::create(target_path).await
            .map_err(|e| NestGateError::Storage(format!("Failed to create target file: {}", e)))?;
        
        let mut buffer = vec![0u8; 1024 * 1024]; // 1MB buffer
        let mut total_copied = 0u64;
        let start_time = Instant::now();
        
        loop {
            let bytes_read = source_file.read(&mut buffer).await
                .map_err(|e| NestGateError::Storage(format!("Failed to read source file: {}", e)))?;
            
            if bytes_read == 0 {
                break; // EOF
            }
            
            target_file.write_all(&buffer[..bytes_read]).await
                .map_err(|e| NestGateError::Storage(format!("Failed to write target file: {}", e)))?;
            
            total_copied += bytes_read as u64;
            
            // Update progress
            job.progress = (total_copied as f64 / job.file_size as f64) * 100.0;
            
            // Calculate transfer rate and ETA
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                job.transfer_rate = total_copied as f64 / elapsed;
                
                if job.transfer_rate > 0.0 {
                    let remaining_bytes = job.file_size.saturating_sub(total_copied);
                    job.eta_seconds = Some((remaining_bytes as f64 / job.transfer_rate) as u64);
                }
            }
            
            // Yield to prevent blocking
            if total_copied % (10 * 1024 * 1024) == 0 { // Every 10MB
                tokio::task::yield_now().await;
            }
        }
        
        // Ensure all data is written to disk
        target_file.sync_all().await
            .map_err(|e| NestGateError::Storage(format!("Failed to sync target file: {}", e)))?;
        
        Ok(())
    }
    
    /// Verify file integrity after copy
    async fn verify_file_integrity(source_path: &PathBuf, target_path: &PathBuf) -> CoreResult<()> {
        // Compare file sizes
        let source_metadata = tokio::fs::metadata(source_path).await
            .map_err(|e| NestGateError::Storage(format!("Failed to get source metadata: {}", e)))?;
        
        let target_metadata = tokio::fs::metadata(target_path).await
            .map_err(|e| NestGateError::Storage(format!("Failed to get target metadata: {}", e)))?;
        
        if source_metadata.len() != target_metadata.len() {
            return Err(NestGateError::Storage(format!(
                "File size mismatch: source {} bytes, target {} bytes",
                source_metadata.len(),
                target_metadata.len()
            )));
        }
        
        // For small files, do a full content comparison
        if source_metadata.len() < 10 * 1024 * 1024 { // 10MB threshold
            let source_content = tokio::fs::read(source_path).await
                .map_err(|e| NestGateError::Storage(format!("Failed to read source for verification: {}", e)))?;
            
            let target_content = tokio::fs::read(target_path).await
                .map_err(|e| NestGateError::Storage(format!("Failed to read target for verification: {}", e)))?;
            
            if source_content != target_content {
                return Err(NestGateError::Storage("File content mismatch after copy".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Update file metadata and access patterns
    async fn update_file_metadata(target_path: &PathBuf, job: &MigrationJob) -> CoreResult<()> {
        // Preserve original timestamps if possible
        if let Ok(source_metadata) = tokio::fs::metadata(&job.source_path).await {
            if let Ok(modified_time) = source_metadata.modified() {
                // Note: Setting file times requires platform-specific code
                // For now, we'll just log this operation
                debug!("Would preserve modified time: {:?} for {:?}", modified_time, target_path);
            }
        }
        
        // Record migration in metadata (could be extended to use extended attributes)
        debug!("Recording migration metadata for {:?} -> {:?}", job.source_path, target_path);
        
        Ok(())
    }
    
    /// Get tier from file path
    fn get_tier_from_path(path: &PathBuf) -> CoreResult<StorageTier> {
        let path_str = path.to_string_lossy();
        
        if path_str.contains("/hot/") || path_str.contains("storage/hot") {
            Ok(StorageTier::Hot)
        } else if path_str.contains("/warm/") || path_str.contains("storage/warm") {
            Ok(StorageTier::Warm)
        } else if path_str.contains("/cold/") || path_str.contains("storage/cold") {
            Ok(StorageTier::Cold)
        } else {
            // Default to Hot tier if unclear
            Ok(StorageTier::Hot)
        }
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
            completed_migrations: 0,
            average_migration_time_seconds: 0.0,
            success_rate: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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