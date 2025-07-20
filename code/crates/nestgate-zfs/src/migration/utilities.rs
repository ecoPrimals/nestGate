//! ZFS Migration Utilities - Helper functions for paths, tiers, and file operations
//!
//! Contains utility functions for working with storage tiers, file paths,
//! dataset operations, and other migration-related helpers.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::info;

use crate::{dataset::ZfsDatasetManager, types::StorageTier};
use nestgate_core::{NestGateError, Result as CoreResult};

/// Get target dataset name for a tier
pub fn get_target_dataset_for_tier(tier: &StorageTier) -> CoreResult<String> {
    match tier {
        StorageTier::Hot => Ok("storage/hot".to_string()),
        StorageTier::Warm => Ok("storage/warm".to_string()),
        StorageTier::Cold => Ok("storage/cold".to_string()),
        StorageTier::Cache => Ok("storage/cache".to_string()),
    }
}

/// Construct target path based on source path and target dataset
pub fn construct_target_path(source_path: &Path, target_dataset: &str) -> CoreResult<PathBuf> {
    // Extract relative path from source
    let file_name = source_path
        .file_name()
        .ok_or_else(|| NestGateError::Storage("Invalid source file path".to_string()))?;

    // Construct target path: /mnt/{dataset}/{filename}
    let target_path = PathBuf::from("/mnt").join(target_dataset).join(file_name);
    Ok(target_path)
}

/// Ensure target dataset exists
pub async fn ensure_target_dataset_exists(
    dataset_name: &str,
    tier: &StorageTier,
    dataset_manager: &Arc<ZfsDatasetManager>,
) -> CoreResult<()> {
    // Check if dataset exists
    let datasets = dataset_manager
        .list_datasets()
        .await
        .map_err(|e| NestGateError::Storage(format!("Failed to list datasets: {e}")))?;

    let dataset_exists = datasets.iter().any(|d| d.name == dataset_name);

    if !dataset_exists {
        info!("Creating target dataset: {}", dataset_name);

        // Create dataset with appropriate properties for the tier
        let mut _properties = std::collections::HashMap::new();

        match tier {
            StorageTier::Hot => {
                _properties.insert("compression".to_string(), "lz4".to_string());
                _properties.insert("recordsize".to_string(), "128K".to_string());
            }
            StorageTier::Warm => {
                _properties.insert("compression".to_string(), "gzip".to_string());
                _properties.insert("recordsize".to_string(), "1M".to_string());
            }
            StorageTier::Cold => {
                _properties.insert("compression".to_string(), "gzip-9".to_string());
                _properties.insert("recordsize".to_string(), "1M".to_string());
            }
            StorageTier::Cache => {
                _properties.insert("compression".to_string(), "off".to_string());
                _properties.insert("recordsize".to_string(), "64K".to_string());
            }
        }

        // Convert tier to nestgate_core::StorageTier
        let core_tier = match tier {
            StorageTier::Hot => nestgate_core::StorageTier::Hot,
            StorageTier::Warm => nestgate_core::StorageTier::Warm,
            StorageTier::Cold => nestgate_core::StorageTier::Cold,
            StorageTier::Cache => nestgate_core::StorageTier::Cache,
        };

        dataset_manager
            .create_dataset(dataset_name, "storage", core_tier)
            .await
            .map_err(|e| NestGateError::Storage(format!("Failed to create dataset: {e}")))?;
    }

    Ok(())
}

/// Get tier from file path
pub fn get_tier_from_path(path: &Path) -> CoreResult<StorageTier> {
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
