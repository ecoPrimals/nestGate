// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Object Operations
//!
//! This module provides object storage operations (CRUD) for the Storage Manager Service.
//! Extracted from the monolithic `service.rs` for improved maintainability.
//!
//! **Phase 3: Smart Refactoring** - Extracted for logical cohesion (Jan 30, 2026)

use crate::Result;
use crate::error::NestGateError;
use bytes::Bytes;
use std::path::PathBuf;
use std::time::SystemTime;
use tracing::info;

use super::super::config::StorageServiceConfig;

/// Store an object in a dataset
///
/// Accepts `impl AsRef<[u8]>` to avoid forcing `.to_vec()` at call sites - allows `Bytes`,
/// `Vec<u8>`, and `&[u8]` without extra allocation in hot paths.
///
/// # Errors
///
/// Returns error if storage fails
pub async fn store_object(
    config: &StorageServiceConfig,
    dataset: &str,
    key: &str,
    data: impl AsRef<[u8]>,
) -> Result<crate::rpc::tarpc_types::ObjectInfo> {
    let data_ref = data.as_ref();
    info!(
        "Storing object: {}/{} ({} bytes)",
        dataset,
        key,
        data_ref.len()
    );

    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(dataset);
    let object_path = dataset_path.join(key);

    // Ensure all parent directories exist. Keys may contain `/` separators
    // (e.g. "test/myapp/hello") which create nested subdirectories.
    if let Some(parent) = object_path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to create key path directories: {e}"))
        })?;
    }

    // Write object
    tokio::fs::write(&object_path, data_ref)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to write object {dataset}/{key}: {e}"))
        })?;

    let now = current_timestamp();
    let object_info = crate::rpc::tarpc_types::ObjectInfo {
        key: key.to_string(),
        dataset: dataset.to_string(),
        size_bytes: u64::try_from(data_ref.len()).unwrap_or(u64::MAX),
        created_at: now,
        modified_at: now,
        content_type: Some("application/octet-stream".to_string()),
        // ✅ EVOLVED: Calculate SHA-256 checksum for data integrity
        checksum: Some(calculate_checksum(data_ref)),
        encrypted: false,
        compressed: false,
        metadata: std::collections::HashMap::new(),
    };

    info!("Object stored: {}/{}", dataset, key);
    Ok(object_info)
}

/// Retrieve an object from a dataset
///
/// Returns `Bytes` (zero-copy) instead of `Vec<u8>` to avoid extra allocation in hot paths.
///
/// # Errors
///
/// Returns error if object not found or retrieval fails
pub async fn retrieve_object(
    config: &StorageServiceConfig,
    dataset: &str,
    key: &str,
) -> Result<(Bytes, crate::rpc::tarpc_types::ObjectInfo)> {
    info!("Retrieving object: {}/{}", dataset, key);

    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(dataset);
    let object_path = dataset_path.join(key);

    // Read object
    let data = tokio::fs::read(&object_path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NestGateError::not_found(format!("object {dataset}/{key}"))
        } else {
            NestGateError::io_error(format!("Failed to read object {dataset}/{key}: {e}"))
        }
    })?;

    // Get metadata
    let metadata = tokio::fs::metadata(&object_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to get metadata: {e}")))?;

    let modified = metadata
        .modified()
        .map_err(|e| NestGateError::io_error(format!("Failed to get modification time: {e}")))?;
    let modified_at = unix_secs(modified);

    let object_info = crate::rpc::tarpc_types::ObjectInfo {
        key: key.to_string(),
        dataset: dataset.to_string(),
        size_bytes: u64::try_from(data.len()).unwrap_or(u64::MAX),
        created_at: modified_at,
        modified_at,
        content_type: Some("application/octet-stream".to_string()),
        checksum: Some(String::new()),
        encrypted: false,
        compressed: false,
        metadata: std::collections::HashMap::new(),
    };

    info!(
        "Object retrieved: {}/{} ({} bytes)",
        dataset,
        key,
        data.len()
    );
    // Zero-copy: Bytes::from takes ownership of Vec from fs::read
    Ok((Bytes::from(data), object_info))
}

/// List objects in a dataset with optional prefix filter and limit.
///
/// # Errors
///
/// Returns error if dataset directory cannot be read.
pub async fn list_objects(
    config: &StorageServiceConfig,
    dataset: &str,
    prefix: Option<&str>,
    limit: Option<usize>,
) -> Result<Vec<crate::rpc::tarpc_types::ObjectInfo>> {
    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(dataset);

    if !dataset_path.exists() {
        return Err(NestGateError::not_found(format!("dataset {dataset}")));
    }

    let mut results = Vec::new();
    collect_objects(
        &dataset_path,
        &dataset_path,
        dataset,
        prefix,
        limit,
        &mut results,
    )
    .await?;
    Ok(results)
}

/// Iteratively collect objects from a directory tree using an explicit stack.
async fn collect_objects(
    base: &std::path::Path,
    start: &std::path::Path,
    dataset: &str,
    prefix: Option<&str>,
    limit: Option<usize>,
    results: &mut Vec<crate::rpc::tarpc_types::ObjectInfo>,
) -> Result<()> {
    let mut stack = vec![start.to_path_buf()];

    while let Some(dir) = stack.pop() {
        if let Some(lim) = limit {
            if results.len() >= lim {
                return Ok(());
            }
        }

        let mut entries = tokio::fs::read_dir(&dir)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to read directory: {e}")))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to read entry: {e}")))?
        {
            if let Some(lim) = limit {
                if results.len() >= lim {
                    return Ok(());
                }
            }
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.is_file() {
                let key = path
                    .strip_prefix(base)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();

                if let Some(pfx) = prefix {
                    if !key.starts_with(pfx) {
                        continue;
                    }
                }

                let metadata = tokio::fs::metadata(&path)
                    .await
                    .map_err(|e| NestGateError::io_error(format!("metadata: {e}")))?;
                let modified = metadata
                    .modified()
                    .map_err(|e| NestGateError::io_error(format!("mod time: {e}")))?;
                let ts = unix_secs(modified);

                results.push(crate::rpc::tarpc_types::ObjectInfo {
                    key,
                    dataset: dataset.to_string(),
                    size_bytes: metadata.len(),
                    created_at: ts,
                    modified_at: ts,
                    content_type: Some("application/octet-stream".to_string()),
                    checksum: None,
                    encrypted: false,
                    compressed: false,
                    metadata: std::collections::HashMap::new(),
                });
            }
        }
    }
    Ok(())
}

/// Get object metadata without reading the body.
///
/// # Errors
///
/// Returns error if the object does not exist.
pub async fn get_object_metadata(
    config: &StorageServiceConfig,
    dataset: &str,
    key: &str,
) -> Result<crate::rpc::tarpc_types::ObjectInfo> {
    let base_path = PathBuf::from(&config.base_path);
    let object_path = base_path.join("datasets").join(dataset).join(key);

    let metadata = tokio::fs::metadata(&object_path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NestGateError::not_found(format!("object {dataset}/{key}"))
        } else {
            NestGateError::io_error(format!("Failed to read metadata for {dataset}/{key}: {e}"))
        }
    })?;
    let modified = metadata
        .modified()
        .map_err(|e| NestGateError::io_error(format!("mod time: {e}")))?;
    let ts = unix_secs(modified);

    Ok(crate::rpc::tarpc_types::ObjectInfo {
        key: key.to_string(),
        dataset: dataset.to_string(),
        size_bytes: metadata.len(),
        created_at: ts,
        modified_at: ts,
        content_type: Some("application/octet-stream".to_string()),
        checksum: None,
        encrypted: false,
        compressed: false,
        metadata: std::collections::HashMap::new(),
    })
}

/// Delete an object from a dataset
///
/// # Errors
///
/// Returns error if object not found or deletion fails
pub async fn delete_object(config: &StorageServiceConfig, dataset: &str, key: &str) -> Result<()> {
    info!("Deleting object: {}/{}", dataset, key);

    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(dataset);
    let object_path = dataset_path.join(key);

    tokio::fs::remove_file(&object_path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NestGateError::not_found(format!("object {dataset}/{key}"))
        } else {
            NestGateError::io_error(format!("Failed to delete object {dataset}/{key}: {e}"))
        }
    })?;

    info!("Object deleted: {}/{}", dataset, key);
    Ok(())
}

/// Calculate SHA-256 checksum for data integrity
fn calculate_checksum(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Get current timestamp in Unix epoch seconds.
fn current_timestamp() -> i64 {
    unix_secs(SystemTime::now())
}

/// Convert a `SystemTime` to Unix epoch seconds (saturating to `i64::MAX`).
fn unix_secs(t: SystemTime) -> i64 {
    let secs = t
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    i64::try_from(secs).unwrap_or(i64::MAX)
}
