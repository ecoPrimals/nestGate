//! ZFS Migration Queue - Queue processing and migration execution
//!
//! Contains the queue processing logic for managing migration jobs and
//! executing migrations between storage tiers.

use chrono::Timelike;
use std::sync::Arc;
use std::time::Instant;

// Removed unused tracing import

use crate::{dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use nestgate_core::{NestGateError, Result as CoreResult};

use super::types::*;
use tracing::error;
use tracing::info;

/// Process the migration queue
pub async fn process_migration_queue(context: MigrationContext<'_>) -> CoreResult<()> {
    // Check if we can start new migrations
    if context.migration_semaphore.available_permits() == 0 {
        return Ok(()); // No available slots
    }

    // Get next job from queue
    let job = {
        let mut queue = context.job_queue.write().await;
        queue.pop_front()
    };

    if let Some(mut job) = job {
        // Check if migration is allowed at this time
        let current_hour = chrono::Utc::now().hour() as u8;
        if !context.config.allowed_hours.contains(&current_hour) {
            // Put job back in queue
            let mut queue = context.job_queue.write().await;
            queue.push_front(job);
            return Ok(());
        }

        // Acquire migration permit
        let _permit =
            context
                .migration_semaphore
                .acquire()
                .await
                .map_err(|e| NestGateError::Internal {
                    message: format!("Failed to acquire migration permit: {}", e),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: None,
                    is_bug: false,
                })?;

        // Start migration
        job.status = MigrationStatus::Running;
        job.started_at = Some(std::time::SystemTime::now());

        let job_id = job.id.clone();

        // Add to active migrations
        {
            let mut active = context.active_migrations.write().await;
            active.insert(job_id.clone(), job.clone());
        }

        // Update statistics
        {
            let mut stats = context.statistics.write().await;
            stats.active_migrations += 1;
            stats.queued_migrations = stats.queued_migrations.saturating_sub(1);
        }

        // Spawn migration task
        let job_clone = job.clone();
        let active_migrations_clone = Arc::clone(context.active_migrations);
        let migration_history_clone = Arc::clone(context.migration_history);
        let statistics_clone = Arc::clone(context.statistics);
        let pool_manager_clone = Arc::clone(context.pool_manager);
        let dataset_manager_clone = Arc::clone(context.dataset_manager);

        tokio::spawn(async move {
            let result =
                execute_migration(job_clone, &pool_manager_clone, &dataset_manager_clone).await;

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

            final_job.completed_at = Some(std::time::SystemTime::now());

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
    info!(
        "Executing migration: {} -> {:?}",
        job.source_path.display(),
        job.target_tier
    );

    let start_time = Instant::now();

    // 1. Validate source file exists
    if !job.source_path.exists() {
        return Err(NestGateError::System {
            message: format!("Source file does not exist: {:?}", job.source_path),
            resource: nestgate_core::error::SystemResource::Disk,
            utilization: None,
            recovery: nestgate_core::error::RecoveryStrategy::Retry,
        });
    }

    // 2. Get target dataset path based on tier
    let target_dataset = super::utilities::get_target_dataset_for_tier(&job.target_tier)?;
    let target_path = super::utilities::construct_target_path(&job.source_path, &target_dataset)?;

    // 3. Ensure target dataset exists
    super::utilities::ensure_target_dataset_exists(
        &target_dataset,
        &job.target_tier,
        dataset_manager,
    )
    .await?;

    // 4. Ensure target directory exists
    if let Some(parent) = target_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| NestGateError::System {
                message: format!("Failed to create target directory: {}", e),
                resource: nestgate_core::error::SystemResource::Disk,
                utilization: None,
                recovery: nestgate_core::error::RecoveryStrategy::Retry,
            })?;
    }

    // 5. Copy file to target tier with progress tracking
    let source_path_clone = job.source_path.clone();
    super::file_operations::copy_file_with_progress(&source_path_clone, &target_path, &mut job)
        .await?;

    // 6. Verify copy integrity
    super::file_operations::verify_file_integrity(&job.source_path, &target_path).await?;

    // 7. Update file metadata and access patterns
    super::file_operations::update_file_metadata(&target_path, &job).await?;

    // 8. Remove file from source tier (only if different from target)
    if super::utilities::get_tier_from_path(&job.source_path)? != job.target_tier {
        tokio::fs::remove_file(&job.source_path)
            .await
            .map_err(|e| NestGateError::System {
                message: format!("Failed to remove source file: {}", e),
                resource: nestgate_core::error::SystemResource::Disk,
                utilization: None,
                recovery: nestgate_core::error::RecoveryStrategy::Retry,
            })?;
    }

    let duration = start_time.elapsed();
    let transfer_rate = job.file_size as f64 / duration.as_secs_f64();

    info!(
        "Migration completed: {} ({:.2} MB/s)",
        job.id,
        transfer_rate / (1024.0 * 1024.0)
    );
    Ok(())
}
