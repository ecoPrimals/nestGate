// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
    info!("Creating dataset: {}", name);

    // Create dataset directory
    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(name);

    tokio::fs::create_dir_all(&dataset_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to create dataset directory: {e}")))?;

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

    info!("Dataset created: {}", name);
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
    debug!("Listing datasets");

    let base_path = PathBuf::from(&config.base_path);
    let datasets_path = base_path.join("datasets");

    // Create datasets dir if it doesn't exist
    tokio::fs::create_dir_all(&datasets_path)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to create datasets directory: {e}"))
        })?;

    let mut datasets = Vec::new();
    let mut entries = tokio::fs::read_dir(&datasets_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read datasets directory: {e}")))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read directory entry: {e}")))?
    {
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Get directory metadata
                let metadata = tokio::fs::metadata(&path).await.map_err(|e| {
                    NestGateError::io_error(format!("Failed to read metadata: {e}"))
                })?;

                let modified = metadata.modified().map_err(|e| {
                    NestGateError::io_error(format!("Failed to get modification time: {e}"))
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

    debug!("Listed {} datasets", datasets.len());
    Ok(datasets)
}

/// Get a single dataset by name.
///
/// # Errors
///
/// Returns error if dataset does not exist or metadata cannot be read.
pub async fn get_dataset(
    config: &StorageServiceConfig,
    name: &str,
) -> Result<crate::rpc::tarpc_types::DatasetInfo> {
    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(name);

    let metadata = tokio::fs::metadata(&dataset_path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NestGateError::not_found(format!("dataset {name}"))
        } else {
            NestGateError::io_error(format!("Failed to read dataset {name}: {e}"))
        }
    })?;

    let modified = metadata
        .modified()
        .map_err(|e| NestGateError::io_error(format!("Failed to get modification time: {e}")))?;
    let modified_at = modified
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    Ok(crate::rpc::tarpc_types::DatasetInfo {
        name: name.to_string(),
        description: None,
        created_at: modified_at,
        modified_at,
        size_bytes: 0,
        object_count: 0,
        compression_ratio: 1.0,
        params: crate::rpc::tarpc_types::DatasetParams::default(),
        status: "active".to_string(),
    })
}

/// Delete a dataset and all its objects
///
/// # Errors
///
/// Returns error if dataset not found or deletion fails
pub async fn delete_dataset(config: &StorageServiceConfig, name: &str) -> Result<()> {
    info!("Deleting dataset: {}", name);

    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(name);

    // Remove dataset directory and all contents
    tokio::fs::remove_dir_all(&dataset_path)
        .await
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                NestGateError::not_found(format!("dataset {name}"))
            } else {
                NestGateError::io_error(format!("Failed to delete dataset {name}: {e}"))
            }
        })?;

    info!("Dataset deleted: {}", name);
    Ok(())
}

/// Get current timestamp in Unix epoch seconds
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::storage::config::StorageServiceConfig;

    fn test_config(dir: &std::path::Path) -> StorageServiceConfig {
        StorageServiceConfig {
            base_path: dir.to_string_lossy().to_string(),
            ..StorageServiceConfig::default()
        }
    }

    #[tokio::test]
    async fn create_list_get_delete_dataset() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        let params = crate::rpc::tarpc_types::DatasetParams {
            description: Some("d".to_string()),
            ..Default::default()
        };
        let info = create_dataset(&cfg, "alpha", params.clone())
            .await
            .expect("create");
        assert_eq!(info.name, "alpha");
        assert_eq!(info.description, params.description);
        let listed = list_datasets(&cfg).await.expect("list");
        assert!(listed.iter().any(|d| d.name == "alpha"));
        let got = get_dataset(&cfg, "alpha").await.expect("get");
        assert_eq!(got.name, "alpha");
        delete_dataset(&cfg, "alpha").await.expect("delete");
        let err = get_dataset(&cfg, "alpha").await.expect_err("gone");
        assert!(err.to_string().contains("dataset"), "{err}");
    }

    #[tokio::test]
    async fn delete_missing_dataset_errors() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        let err = delete_dataset(&cfg, "missing").await.expect_err("delete");
        assert!(err.to_string().contains("dataset"), "{err}");
    }

    #[tokio::test]
    async fn list_datasets_creates_root_directory() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        let empty = list_datasets(&cfg).await.expect("list empty");
        assert!(empty.is_empty());
    }
}
