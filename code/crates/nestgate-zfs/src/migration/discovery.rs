//! ZFS Migration Discovery - Automatic migration candidate discovery
//!
//! Contains the logic for discovering files that should be migrated based on
//! access patterns and tier optimization recommendations.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::{automation::DatasetAnalyzer, types::StorageTier};
use nestgate_core::{NestGateError, Result as CoreResult};

use super::types::*;

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

        let mut entries = tokio::fs::read_dir(&dir_path).await.map_err(|e| {
            NestGateError::Storage(format!("Failed to read directory {dir_path:?}: {e}"))
        })?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| NestGateError::Storage(format!("Failed to read directory entry: {e}")))?
        {
            let path = entry.path();

            if path.is_file() {
                files.push(path);

                // Limit total files to prevent memory issues
                if files.len() >= 1000 {
                    break;
                }
            } else if path.is_dir() {
                scan_directory_recursive(path, files, depth + 1).await?;
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
        .map_err(|e| NestGateError::Internal(format!("File analysis failed: {e}")))?;

    // Get tier recommendation
    let recommendation = analyzer
        .predict_optimal_tier(&file_path.to_string_lossy())
        .await
        .map_err(|e| NestGateError::Internal(format!("Tier recommendation failed: {e}")))?;

    // Convert from nestgate_core::StorageTier to types::StorageTier
    let recommended_tier = match recommendation {
        nestgate_core::StorageTier::Hot => StorageTier::Hot,
        nestgate_core::StorageTier::Warm => StorageTier::Warm,
        nestgate_core::StorageTier::Cold => StorageTier::Cold,
        nestgate_core::StorageTier::Cache => StorageTier::Hot, // Map Cache to Hot
    };

    Ok(Some(recommended_tier))
}
