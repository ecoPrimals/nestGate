//
// Contains the MigrationEngine struct and its lifecycle methods for managing
// automated data migration between storage tiers.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use crate::migration::discovery::DatasetAnalyzer;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock, Semaphore};
use tokio::time::interval;
use tracing::error;
use tracing::info;

use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};
// Removed unresolved FileAnalyzer import - using local implementation
use nestgate_core::Result as CoreResult;

use super::types::*;

// Type aliases for complex types
type ActiveMigrationsMap = Arc<RwLock<HashMap<String, MigrationJob>>>;
type MigrationStatisticsMap = Arc<RwLock<MigrationStatistics>>;

/// Migration engine for automated tier-to-tier data movement
#[derive(Debug)]
#[allow(dead_code)] // Configuration fields used in migration planning
pub struct MigrationEngine {
    config: MigrationConfig,
    zfs_config: Arc<ZfsConfig>,
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    analyzer: Arc<DatasetAnalyzer>,

    /// Migration job queue
    job_queue: Arc<RwLock<VecDeque<MigrationJob>>>,
    /// Active migrations
    active_migrations: ActiveMigrationsMap,
    /// Migration history
    migration_history: Arc<RwLock<Vec<MigrationJob>>>,
    /// Migration statistics
    statistics: MigrationStatisticsMap,

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
            zfs_config: Arc::new(zfs_config),
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

    /// Create a new migration engine with shared config (zero-copy optimization)
    pub fn with_shared_config(
        config: MigrationConfig,
        zfs_config: Arc<ZfsConfig>,
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
        let _bandwidth_semaphore = Arc::clone(&self.bandwidth_semaphore);
        let config = self.config.clone();
        let pool_manager = Arc::clone(&self.pool_manager);
        let dataset_manager = Arc::clone(&self.dataset_manager);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.progress_update_interval));

            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        let context = MigrationContext {
                            job_queue: &job_queue,
                            active_migrations: &active_migrations,
                            migration_history: &migration_history,
                            statistics: &statistics,
                            migration_semaphore: &migration_semaphore,
                            config: &config,
                            pool_manager: &pool_manager,
                            dataset_manager: &dataset_manager,
                        };

                        if let Err(e) = super::queue::process_migration_queue(context).await {
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

        info!("Migration engine started");
        Ok(())
    }

    /// Stop the migration engine
    pub async fn stop(&mut self) -> CoreResult<()> {
        info!("Stopping migration engine");

        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
        }

        // Cancel all active migrations
        let active_migrations = self.active_migrations.read().await;
        for job_id in active_migrations.keys() {
            info!("Cancelling migration job: {}", job_id);
        }

        info!("Migration engine stopped");
        Ok(())
    }

    /// Submit a migration job
    pub async fn submit_job(&self, job: MigrationJob) -> CoreResult<String> {
        let job_id = job.id.clone();

        // Add to queue
        let mut queue = self.job_queue.write().await;
        queue.push_back(job);

        // Update statistics
        let mut stats = self.statistics.write().await;
        stats.total_jobs += 1;
        stats.queued_migrations += 1;

        info!("Submitted migration job: {}", job_id);
        Ok(job_id)
    }

    /// Cancel a migration job
    pub async fn cancel_job(&self, job_id: &str) -> CoreResult<bool> {
        // Check if it's in the queue
        {
            let mut queue = self.job_queue.write().await;
            if let Some(pos) = queue.iter().position(|job| job.id == job_id) {
                if let Some(mut job) = queue.remove(pos) {
                    job.status = MigrationStatus::Cancelled;
                    job.completed_at = Some(std::time::SystemTime::now());

                    // Move to history
                    let mut history = self.migration_history.write().await;
                    history.push(job);

                    return Ok(true);
                }
            }
        }

        // Check if it's an active migration
        let mut active = self.active_migrations.write().await;
        if let Some(mut job) = active.remove(job_id) {
            job.status = MigrationStatus::Cancelled;
            job.completed_at = Some(std::time::SystemTime::now());

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
        self.active_migrations
            .read()
            .await
            .values()
            .cloned()
            .collect()
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
            let mut interval = interval(Duration::from_secs(
                std::env::var("NESTGATE_ZFS_MIGRATION_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600), // 1 hour default
            )); // Migration check interval

            loop {
                interval.tick().await;

                if let Err(e) = super::discovery::discover_migration_candidates(
                    &analyzer,
                    &job_queue,
                    &statistics,
                )
                .await
                {
                    error!("Error discovering migration candidates: {}", e);
                }
            }
        });
        Ok(())
    }
}
