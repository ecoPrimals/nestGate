//! Main filesystem storage backend implementation

use super::metadata::create_file_metadata;
use super::operations::{atomic_write, check_size_limit, validate_path};
use super::types::{FileMetadata, FilesystemConfig};
use crate::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Filesystem storage backend implementation
#[allow(dead_code)] // Backend implementation - fields used internally
pub struct FilesystemBackend {
    config: FilesystemConfig,
    root_dir: PathBuf,
}

impl FilesystemBackend {
    /// Create a new filesystem backend
    pub fn new(connection_config: &HashMap<String, String>) -> Result<Self> {
        let mut config = FilesystemConfig::default();

        // Parse configuration from connection settings
        if let Some(root_dir) = connection_config.get("root_dir") {
            config.root_dir = PathBuf::from(root_dir);
        }

        if let Some(atomic_writes) = connection_config.get("atomic_writes") {
            config.atomic_writes = atomic_writes.parse().unwrap_or(true);
        }

        if let Some(track_metadata) = connection_config.get("track_metadata") {
            config.track_metadata = track_metadata.parse().unwrap_or(true);
        }

        if let Some(max_file_size) = connection_config.get("max_file_size") {
            config.max_file_size = max_file_size.parse().unwrap_or(0);
        }

        let root_dir = config.root_dir.clone();

        // Ensure root directory exists
        std::fs::create_dir_all(&root_dir).map_err(|e| crate::error::NestGateError::Io {
            operation: "create_root_directory".to_string(),
            error_message: format!("Failed to create root directory: {e}"),
            resource: Some(root_dir.to_string_lossy().to_string()),
            retryable: true,
        })?;

        Ok(Self { config, root_dir })
    }

    /// Get the full path for a given relative path
    pub fn get_full_path(&self, path: &str) -> Result<PathBuf> {
        validate_path(path)?;
        Ok(self.root_dir.join(path))
    }

    /// Check file size limits
    pub fn check_size_limit(&self, size: usize) -> Result<()> {
        check_size_limit(size, self.config.max_file_size)
    }

    /// Create file metadata from filesystem metadata
    pub async fn create_file_metadata(&self, path: &Path) -> Result<FileMetadata> {
        create_file_metadata(path).await
    }

    /// Perform atomic write operation
    pub async fn atomic_write(&self, path: &Path, content: &[u8]) -> Result<()> {
        atomic_write(path, content, self.config.atomic_writes).await
    }

    /// Write file to storage
    pub async fn write_file(&self, path: &str, content: &[u8]) -> Result<()> {
        self.check_size_limit(content.len())?;
        let full_path = self.get_full_path(path)?;

        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| crate::error::NestGateError::Io {
                    operation: "create_parent_dir".to_string(),
                    error_message: format!("Failed to create parent directory: {e}"),
                    resource: Some(parent.to_string_lossy().to_string()),
                    retryable: true,
                })?;
        }

        self.atomic_write(&full_path, content).await
    }

    /// Read file from storage
    pub async fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        let full_path = self.get_full_path(path)?;

        fs::read(&full_path)
            .await
            .map_err(|e| crate::error::NestGateError::Io {
                operation: "read_file".to_string(),
                error_message: format!("Failed to read file: {e}"),
                resource: Some(full_path.to_string_lossy().to_string()),
                retryable: true,
            })
    }

    /// Delete file from storage
    pub async fn delete_file(&self, path: &str) -> Result<()> {
        let full_path = self.get_full_path(path)?;

        fs::remove_file(&full_path)
            .await
            .map_err(|e| crate::error::NestGateError::Io {
                operation: "delete_file".to_string(),
                error_message: format!("Failed to delete file: {e}"),
                resource: Some(full_path.to_string_lossy().to_string()),
                retryable: false,
            })
    }

    /// Check if file exists
    pub async fn file_exists(&self, path: &str) -> Result<bool> {
        let full_path = self.get_full_path(path)?;
        Ok(fs::try_exists(&full_path).await.unwrap_or(false))
    }

    /// Get file metadata
    pub async fn get_metadata(&self, path: &str) -> Result<FileMetadata> {
        let full_path = self.get_full_path(path)?;
        self.create_file_metadata(&full_path).await
    }

    /// List files in directory
    pub async fn list_directory(&self, path: &str) -> Result<Vec<String>> {
        let full_path = self.get_full_path(path)?;

        let mut entries =
            fs::read_dir(&full_path)
                .await
                .map_err(|e| crate::error::NestGateError::Io {
                    operation: "read_directory".to_string(),
                    error_message: format!("Failed to read directory: {e}"),
                    resource: Some(full_path.to_string_lossy().to_string()),
                    retryable: true,
                })?;

        let mut files = Vec::new();
        while let Some(entry) =
            entries
                .next_entry()
                .await
                .map_err(|e| crate::error::NestGateError::Io {
                    operation: "read_directory_entry".to_string(),
                    error_message: format!("Failed to read directory entry: {e}"),
                    resource: Some(full_path.to_string_lossy().to_string()),
                    retryable: true,
                })?
        {
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }

        Ok(files)
    }

    /// Create directory
    pub async fn create_directory(&self, path: &str) -> Result<()> {
        let full_path = self.get_full_path(path)?;

        fs::create_dir_all(&full_path)
            .await
            .map_err(|e| crate::error::NestGateError::Io {
                operation: "create_directory".to_string(),
                error_message: format!("Failed to create directory: {e}"),
                resource: Some(full_path.to_string_lossy().to_string()),
                retryable: true,
            })
    }

    /// Delete directory
    pub async fn delete_directory(&self, path: &str) -> Result<()> {
        let full_path = self.get_full_path(path)?;

        fs::remove_dir_all(&full_path)
            .await
            .map_err(|e| crate::error::NestGateError::Io {
                operation: "delete_directory".to_string(),
                error_message: format!("Failed to delete directory: {e}"),
                resource: Some(full_path.to_string_lossy().to_string()),
                retryable: false,
            })
    }
}
