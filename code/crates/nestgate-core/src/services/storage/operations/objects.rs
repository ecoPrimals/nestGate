// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
        "💾 Storing object: {}/{} ({} bytes)",
        dataset,
        key,
        data_ref.len()
    );

    let base_path = PathBuf::from(&config.base_path);
    let dataset_path = base_path.join("datasets").join(dataset);
    let object_path = dataset_path.join(key);

    // Ensure all parent directories exist. Keys may contain `/` separators
    // (e.g. "test/primalspring/hello") which create nested subdirectories.
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
        size_bytes: data_ref.len() as u64,
        created_at: now,
        modified_at: now,
        content_type: Some("application/octet-stream".to_string()),
        // ✅ EVOLVED: Calculate SHA-256 checksum for data integrity
        checksum: Some(calculate_checksum(data_ref)),
        encrypted: false,
        compressed: false,
        metadata: std::collections::HashMap::new(),
    };

    info!("✅ Object stored: {}/{}", dataset, key);
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
    info!("📖 Retrieving object: {}/{}", dataset, key);

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
    let modified_at = modified
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let object_info = crate::rpc::tarpc_types::ObjectInfo {
        key: key.to_string(),
        dataset: dataset.to_string(),
        size_bytes: data.len() as u64,
        created_at: modified_at,
        modified_at,
        content_type: Some("application/octet-stream".to_string()),
        checksum: Some(String::new()),
        encrypted: false,
        compressed: false,
        metadata: std::collections::HashMap::new(),
    };

    info!(
        "✅ Object retrieved: {}/{} ({} bytes)",
        dataset,
        key,
        data.len()
    );
    // Zero-copy: Bytes::from takes ownership of Vec from fs::read
    Ok((Bytes::from(data), object_info))
}

/// Delete an object from a dataset
///
/// # Errors
///
/// Returns error if object not found or deletion fails
pub async fn delete_object(config: &StorageServiceConfig, dataset: &str, key: &str) -> Result<()> {
    info!("🗑️  Deleting object: {}/{}", dataset, key);

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

    info!("✅ Object deleted: {}/{}", dataset, key);
    Ok(())
}

/// Calculate SHA-256 checksum for data integrity
fn calculate_checksum(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Get current timestamp in Unix epoch seconds
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
