//! ZFS Migration Discovery - Automatic migration candidate discovery
//!
//! Contains the logic for discovering files that should be migrated based on
//! access patterns and tier optimization recommendations.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::{automation::DatasetAnalyzer, types::StorageTier};
use nestgate_core::{types::StorageTier as CoreStorageTier, NestGateError, Result as CoreResult};

use super::types::*;

const MAX_DEPTH: usize = 10;
use tracing::debug;
use tracing::info;

/// Discover files that should be migrated based on access patterns
pub async fn discover_migration_candidates(
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
        if let Ok(entries) = scan_directory_for_files(tier_path).await {
            for file_path in entries {
                match analyze_migration_candidate(&file_path, current_tier, analyzer).await {
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
    for (file_path, current_tier, recommended_tier) in candidates {
        // Check if file size is reasonable for migration
        if let Ok(metadata) = tokio::fs::metadata(&file_path).await {
            let file_size = metadata.len();

            // Skip very small files (< 1MB) or very large files (> 10GB) for automatic migration
            if !(1024 * 1024..=10 * 1024 * 1024 * 1024).contains(&file_size) {
                continue;
            }

            // Create migration job
            let job = MigrationJob::new(
                file_path,
                current_tier,
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
        info!(
            "Discovered and queued {} migration candidates",
            queued_count
        );

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

    scan_directory_recursive(path, &mut files, 0).await?;
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

        let mut dir_reader =
            tokio::fs::read_dir(&dir_path)
                .await
                .map_err(|e| NestGateError::System {
                    message: format!("Failed to read directory {:?}: {}", dir_path, e),
                    resource: nestgate_core::error::SystemResource::Disk,
                    utilization: None,
                    recovery: nestgate_core::error::RecoveryStrategy::Retry,
                })?;

        while let Some(entry) =
            dir_reader
                .next_entry()
                .await
                .map_err(|e| NestGateError::System {
                    message: format!("Failed to read directory entry: {}", e),
                    resource: nestgate_core::error::SystemResource::Disk,
                    utilization: None,
                    recovery: nestgate_core::error::RecoveryStrategy::Retry,
                })?
        {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            } else if path.is_dir() && depth < MAX_DEPTH {
                // Recursively scan subdirectory
                scan_directory_recursive(path, files, depth + 1).await.ok();
            }
        }
        Ok(())
    })
}

/// Analyze a file to determine if it should be migrated
async fn analyze_migration_candidate(
    file_path: &Path,
    _current_tier: StorageTier,
    analyzer: &Arc<DatasetAnalyzer>,
) -> CoreResult<Option<StorageTier>> {
    // Analyze file characteristics
    let _characteristics = analyzer
        .analyze_file(&file_path.to_string_lossy())
        .await
        .map_err(|e| NestGateError::System {
            message: format!("File analysis failed: {}", e),
            resource: nestgate_core::error::SystemResource::Disk,
            utilization: None,
            recovery: nestgate_core::error::RecoveryStrategy::Retry,
        })?;

    // Get file metadata for heuristic analysis
    let metadata = tokio::fs::metadata(file_path)
        .await
        .map_err(|e| NestGateError::System {
            message: format!("Failed to get file metadata: {}", e),
            resource: nestgate_core::error::SystemResource::Disk,
            utilization: None,
            recovery: nestgate_core::error::RecoveryStrategy::Retry,
        })?;

    let file_size = metadata.len();
    let access_time = metadata
        .accessed()
        .unwrap_or_else(|_| std::time::SystemTime::now());

    // Get tier recommendation - using simple heuristic since predict_optimal_tier doesn't exist
    let recommendation = if file_size > 1024 * 1024 * 1024 {
        // > 1GB
        CoreStorageTier::Cold // Large files go to cold storage
    } else if access_time.elapsed().unwrap_or_default().as_secs() < 24 * 3600 {
        // < 24 hours
        CoreStorageTier::Hot // Recently accessed files stay hot
    } else {
        CoreStorageTier::Warm // Default to warm storage
    };

    // Convert from nestgate_core::StorageTier to types::StorageTier
    let recommended_tier = match recommendation {
        CoreStorageTier::Hot => StorageTier::Hot,
        CoreStorageTier::Warm => StorageTier::Warm,
        CoreStorageTier::Cold => StorageTier::Cold,
        CoreStorageTier::Cache => StorageTier::Hot, // Map Cache to Hot
        CoreStorageTier::Archive => StorageTier::Cold, // Map Archive to Cold
    };

    Ok(Some(recommended_tier))
}
