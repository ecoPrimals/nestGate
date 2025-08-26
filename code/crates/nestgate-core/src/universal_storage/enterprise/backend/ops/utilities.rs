use crate::NestGateError;
use std::collections::HashMap;
//
// This module provides shared utility functions for enterprise storage operations,
// extracted from the monolithic enterprise_ops.rs file.
//
// **PROVIDES**:
// - Directory tree operations (copy, clear, selective operations)
// - File system utilities and size calculations
// - Hash computation for deduplication analysis
// - Path and metadata manipulation utilities
//
// **EXTRACTED FROM**: enterprise_ops.rs lines 425-935 (510+ lines)

use crate::{Result, NestGateError};
use std::path::Path;
use super::super::core::EnterpriseStorageBackend;
use super::FileHashMap;

// ==================== DIRECTORY OPERATIONS ====================

impl EnterpriseStorageBackend {
    /// Recursively copy directory tree
    pub(crate) async fn copy_directory_tree(&self, src: &Path, dst: &Path) -> Result<()> {
        if !src.exists() {
            return Ok(());
        }

        if src.is_file() {
            if let Some(parent) = dst.parent() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| {
                                    NestGateError::storage_error(
                    &format!("Failed to create parent directory: {e}"),
                    "create_directory",
                    None,
                )
                })?;
            }
            tokio::fs::copy(src, dst).await.map_err(|e| {
                NestGateError::storage_error(&format!("Failed to copy file: {e}"), "copy_file", None)
            })?;
            return Ok(());
        }

        // Create destination directory
        tokio::fs::create_dir_all(dst).await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to create directory: {e}"), "create_directory", None)
        })?;

        // Read source directory
        let mut entries = tokio::fs::read_dir(src).await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read directory: {e}"), "read_directory", None)
        })?;

        // Process each entry
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read directory entry: {e}"), "read_directory_entry", None)
        })? {
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                Box::pin(self.copy_directory_tree(&src_path, &dst_path)).await?;
            } else {
                tokio::fs::copy(&src_path, &dst_path).await.map_err(|e| {
                    NestGateError::storage_error(&format!("Failed to copy file: {e}"), "copy_file", None)
                })?;
            }
        }

        Ok(())
    }

    /// Copy directory tree while excluding certain directories
    pub(crate) async fn copy_directory_tree_selective(
        &self,
        src: &Path,
        dst: &Path,
        exclude: &[&str],
    ) -> Result<()> {
        if !src.exists() {
            return Ok(());
        }

        if src.is_file() {
            if let Some(parent) = dst.parent() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| {
                    NestGateError::storage_error(
                        &format!("Failed to create parent directory: {e}"),
                        "create_directory",
                        None,
                    )
                })?;
            }
            tokio::fs::copy(src, dst).await.map_err(|e| {
                NestGateError::storage_error(&format!("Failed to copy file: {e}"), "copy_file", None)
            })?;
            return Ok(());
        }

        // Create destination directory
        tokio::fs::create_dir_all(dst).await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to create directory: {e}"), "create_directory", None)
        })?;

        // Read source directory
        let mut entries = tokio::fs::read_dir(src).await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read directory: {e}"), "read_directory", None)
        })?;

        // Process each entry
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read directory entry: {e}"), "read_directory_entry", None)
        })? {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Skip excluded directories
            if exclude.contains(&file_name_str.as_ref()) {
                continue;
            }

            let src_path = entry.path();
            let dst_path = dst.join(file_name);

            if src_path.is_dir() {
                Box::pin(self.copy_directory_tree_selective(&src_path, &dst_path, exclude)).await?;
            } else {
                tokio::fs::copy(&src_path, &dst_path).await.map_err(|e| {
                    NestGateError::storage_error(&format!("Failed to copy file: {e}"), "copy_file", None)
                })?;
            }
        }

        Ok(())
    }

    /// Clear directory contents while preserving certain subdirectories
    pub(crate) async fn clear_directory_selective(
        &self,
        dir: &Path,
        exclude: &[&str],
    ) -> Result<()> {
        let mut entries = tokio::fs::read_dir(dir).await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read directory: {e}"), "read_directory", None)
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read directory entry: {e}"), "read_directory_entry", None)
        })? {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            // Skip excluded directories
            if exclude.contains(&file_name_str.as_ref()) {
                continue;
            }

            let path = entry.path();
            if path.is_dir() {
                tokio::fs::remove_dir_all(&path).await.map_err(|e| {
                    NestGateError::storage_error(&format!("Failed to remove directory: {e}"), "remove_directory", None)
                })?;
            } else {
                tokio::fs::remove_file(&path).await.map_err(|e| {
                    NestGateError::storage_error(&format!("Failed to remove file: {e}"), "remove_file", None)
                })?;
            }
        }

        Ok(())
    }

    /// Calculate total size of directory recursively
    pub(super) async fn calculate_directory_size(&self, dir: &Path) -> Result<u64> {
        let mut total_size = 0u64;
        let mut entries = tokio::fs::read_dir(dir).await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read directory: {e}"), "read_directory", None)
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read directory entry: {e}"), "read_directory_entry", None)
        })? {
            let path = entry.path();
            let metadata = entry.metadata().await.map_err(|e| {
                NestGateError::storage_error(&format!("Failed to read metadata: {e}"), "read_metadata", None)
            })?;

            if metadata.is_file() {
                total_size += metadata.len();
            } else if metadata.is_dir() {
                total_size += Box::pin(self.calculate_directory_size(&path)).await?;
            }
        }

        Ok(total_size)
    }

    // ==================== HASH COMPUTATION UTILITIES ====================

    /// Iteratively compute file hashes (avoiding recursion)
    pub(crate) async fn compute_hashes_recursive(
        &self,
        root_dir: &Path,
        file_hashes: &mut FileHashMap,
    ) -> Result<()> {
        use sha2::{Digest, Sha256};
        use std::collections::VecDeque;
        use tokio::fs;
        use tokio::io::AsyncReadExt;

        let mut queue = VecDeque::new();
        queue.push_back(root_dir.to_path_buf());

        while let Some(dir) = queue.pop_front() {
            let mut entries = fs::read_dir(&dir).await.map_err(|e| {
                NestGateError::storage_error(&format!("Failed to read directory: {e}"), "read_directory", None)
            })?;

            while let Some(entry) = entries.next_entry().await.map_err(|e| {
                NestGateError::storage_error(&format!("Failed to read directory entry: {e}"), "read_directory_entry", None)
            })? {
                let path = entry.path();
                let metadata = entry.metadata().await.map_err(|e| {
                    NestGateError::storage_error(&format!("Failed to read metadata: {e}"), "read_metadata", None)
                })?;

                if metadata.is_file() {
                    // Skip very large files to avoid memory issues
                    if metadata.len() > 100 * 1024 * 1024 {
                        // 100MB limit
                        continue;
                    }

                    // Compute file hash
                    let mut file = fs::File::open(&path).await.map_err(|e| {
                        NestGateError::storage_error(&format!("Failed to open file: {e}"), "open_file", None)
                    })?;

                    let mut hasher = Sha256::new();
                    let mut buffer = [0u8; 8192];

                    loop {
                        let bytes_read = file.read(&mut buffer).await.map_err(|e| {
                            NestGateError::storage_error(&format!("Failed to read file: {e}"), "read_file", None)
                        })?;

                        if bytes_read == 0 {
                            break;
                        }

                        hasher.update(&buffer[..bytes_read]);
                    }

                    let hash = format!("{:x}", hasher.finalize());
                    file_hashes
                        .entry(hash)
                        .or_default()
                        .push((path, metadata.len()));
                } else if metadata.is_dir() {
                    // Add subdirectory to queue for processing
                    queue.push_back(path);
                }
            }
        }

        Ok(())
    }
} 