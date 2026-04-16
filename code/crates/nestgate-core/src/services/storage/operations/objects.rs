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
    async fn store_retrieve_delete_round_trip() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        let ds = "ds1";
        let key = "obj.bin";
        let payload: &[u8] = b"payload-bytes";
        let info = store_object(&cfg, ds, key, payload)
            .await
            .expect("store should succeed");
        assert_eq!(info.key, key);
        assert_eq!(info.dataset, ds);
        assert_eq!(info.size_bytes, payload.len() as u64);
        let (bytes, meta) = retrieve_object(&cfg, ds, key).await.expect("retrieve");
        assert_eq!(bytes.as_ref(), payload);
        assert_eq!(meta.key, key);
        delete_object(&cfg, ds, key).await.expect("delete");
        let err = retrieve_object(&cfg, ds, key)
            .await
            .expect_err("missing object");
        assert!(
            err.to_string().to_lowercase().contains("not found")
                || err.to_string().contains("object"),
            "unexpected error: {err}"
        );
    }

    #[tokio::test]
    async fn nested_key_path_creates_directories() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        let key = "nested/path/file.txt";
        store_object(&cfg, "ds", key, b"x")
            .await
            .expect("nested store");
        let (data, _) = retrieve_object(&cfg, "ds", key)
            .await
            .expect("nested retrieve");
        assert_eq!(data.as_ref(), b"x");
    }

    #[tokio::test]
    async fn list_objects_prefix_and_limit() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        store_object(&cfg, "ds", "a/1", b"1").await.expect("a/1");
        store_object(&cfg, "ds", "a/2", b"2").await.expect("a/2");
        store_object(&cfg, "ds", "b/1", b"3").await.expect("b/1");
        let listed = list_objects(&cfg, "ds", Some("a/"), None)
            .await
            .expect("list with prefix");
        assert_eq!(listed.len(), 2);
        let limited = list_objects(&cfg, "ds", None, Some(1))
            .await
            .expect("list with limit");
        assert_eq!(limited.len(), 1);
    }

    #[tokio::test]
    async fn list_objects_dataset_missing() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        let err = list_objects(&cfg, "nope", None, None)
            .await
            .expect_err("missing dataset");
        assert!(
            err.to_string().contains("dataset"),
            "unexpected error: {err}"
        );
    }

    #[tokio::test]
    async fn get_object_metadata_matches_store() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        let stored = store_object(&cfg, "ds", "k", b"abc").await.expect("store");
        let meta = get_object_metadata(&cfg, "ds", "k")
            .await
            .expect("metadata");
        assert_eq!(meta.size_bytes, 3);
        assert_eq!(meta.key, stored.key);
    }

    #[tokio::test]
    async fn metadata_not_found() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        tokio::fs::create_dir_all(dir.path().join("datasets").join("ds"))
            .await
            .expect("mkdir");
        let err = get_object_metadata(&cfg, "ds", "missing")
            .await
            .expect_err("no object");
        assert!(err.to_string().contains("object"), "{err}");
    }

    #[tokio::test]
    async fn delete_missing_object_errors() {
        let dir = tempfile::tempdir().expect("tempdir");
        let cfg = test_config(dir.path());
        tokio::fs::create_dir_all(dir.path().join("datasets").join("ds"))
            .await
            .expect("mkdir");
        let err = delete_object(&cfg, "ds", "nope")
            .await
            .expect_err("delete missing");
        assert!(err.to_string().contains("object"), "{err}");
    }
}
