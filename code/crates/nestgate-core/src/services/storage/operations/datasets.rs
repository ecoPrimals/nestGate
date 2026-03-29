// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Dataset Operations
//!
//! This module provides dataset management operations for the Storage Manager Service.
//! Extracted from the monolithic `service.rs` for improved maintainability.
//!
//! **Phase 3: Smart Refactoring** - Extracted for logical cohesion (Jan 30, 2026)

use crate::Result;
use crate::error::NestGateError;
use std::path::PathBuf;
use std::time::SystemTime;
use tracing::{debug, info};

use super::super::config::StorageServiceConfig;

/// Create a new dataset
///
/// # Errors
///
/// Returns error if dataset already exists or creation fails
pub async fn create_dataset(
    config: &StorageServiceConfig,
    name: &str,
    params: crate::rpc::tarpc_types::DatasetParams,
) -> Result<crate::rpc::tarpc_types::DatasetInfo> {
    info!("📦 Creating dataset: {}", name);

    // Create dataset directory
    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(name);

    tokio::fs::create_dir_all(&dataset_path)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to create dataset directory: {}", e))
        })?;

    // Create dataset info
    let now = current_timestamp();
    let dataset = crate::rpc::tarpc_types::DatasetInfo {
        name: name.to_string(),
        description: params.description.clone(),
        created_at: now,
        modified_at: now,
        size_bytes: 0,
        object_count: 0,
        compression_ratio: 1.0,
        params,
        status: "active".to_string(),
    };

    info!("✅ Dataset created: {}", name);
    Ok(dataset)
}

/// List all datasets
///
/// # Errors
///
/// Returns error if listing fails
pub async fn list_datasets(
    config: &StorageServiceConfig,
) -> Result<Vec<crate::rpc::tarpc_types::DatasetInfo>> {
    debug!("📋 Listing datasets");

    let base_path = PathBuf::from(&config.base_path);
    let datasets_path = base_path.join("datasets");

    // Create datasets dir if it doesn't exist
    tokio::fs::create_dir_all(&datasets_path)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to create datasets directory: {}", e))
        })?;

    let mut datasets = Vec::new();
    let mut entries = tokio::fs::read_dir(&datasets_path).await.map_err(|e| {
        NestGateError::io_error(format!("Failed to read datasets directory: {}", e))
    })?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read directory entry: {}", e)))?
    {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Get directory metadata
                let metadata = tokio::fs::metadata(&path).await.map_err(|e| {
                    NestGateError::io_error(format!("Failed to read metadata: {}", e))
                })?;

                let modified = metadata.modified().map_err(|e| {
                    NestGateError::io_error(format!("Failed to get modification time: {}", e))
                })?;
                let modified_at = modified
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64;

                datasets.push(crate::rpc::tarpc_types::DatasetInfo {
                    name: name.to_string(),
                    description: None,
                    created_at: modified_at,
                    modified_at,
                    size_bytes: 0,
                    object_count: 0,
                    compression_ratio: 1.0,
                    params: crate::rpc::tarpc_types::DatasetParams::default(),
                    status: "active".to_string(),
                });
            }
        }
    }

    debug!("✅ Listed {} datasets", datasets.len());
    Ok(datasets)
}

/// Delete a dataset and all its objects
///
/// # Errors
///
/// Returns error if dataset not found or deletion fails
pub async fn delete_dataset(config: &StorageServiceConfig, name: &str) -> Result<()> {
    info!("🗑️  Deleting dataset: {}", name);

    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(name);

    // Remove dataset directory and all contents
    tokio::fs::remove_dir_all(&dataset_path)
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                NestGateError::not_found(format!("dataset {}", name))
            } else {
                NestGateError::io_error(format!("Failed to delete dataset {}: {}", name, e))
            }
        })?;

    info!("✅ Dataset deleted: {}", name);
    Ok(())
}

/// Get current timestamp in Unix epoch seconds
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
