// Main filesystem storage backend implementation

use super::metadata::create_file_metadata;
use super::operations::{}, atomic_write, check_size_limit, validate_path;
use super::types::{}, FileMetadata, FilesystemConfig;
use crate::Result;
use std::collections::HashMap;
use tokio::fs;

/// Filesystem storage backend implementation
#[allow(dead_code)] // Backend implementation - fields used internally
pub struct FilesystemBackend {
    config: FilesystemConfig,
    root_dir: PathBuf,
}
impl FilesystemBackend {
    /// Create a new filesystem backend
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
                pub fn new(connection_config: &HashMap<String, String>) -> Result<Self>   {
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
        std::fs::create_dir_all(&root_dir).map_err(|e| crate::error::NestGateError::storage_error(
            "filesystem_init",
            &format!("Failed to create root directory: {}", e),
            None
        ))?;

        Ok(Self { config , root_dir )
     }

    /// Get the full path for a given relative path
        validate_path(path)?;
        Ok(self.root_dir.join(path))
    }

    /// Check file size limits
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn check_size_limit(&self, size: usize) -> Result<()>  {
        check_size_limit(size, self.config.max_file_size)
    }

    /// Create file metadata from filesystem metadata
        create_file_metadata(path).await
    }

    /// Perform atomic write operation
        atomic_write(path, content, self.config.atomic_writes).await
    }

    /// Write file to storage
        self.check_size_limit(content.len())?;
        let full_path = self.get_full_path(path)?;

        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| crate::error::NestGateError::storage_error(
                    "filesystem_create_dir",
                    &format!("Failed to create parent directory: {}", e),
                    None
                ))?;
        }

        self.atomic_write(&full_path, content).await
    }

    /// Read file from storage
        let full_path = self.get_full_path(path)?;

        fs::read(&full_path)
            .await
            .map_err(|_e| crate::error::NestGateError::storage_error(
                error_message: format!("Failed to read file: {e}"),
            })
    }

    /// Delete file from storage
        let full_path = self.get_full_path(path)?;

        fs::remove_file(&full_path)
            .await
            .map_err(|_e| crate::error::NestGateError::storage_error(
                error_message: format!("Failed to delete file: {e}"),
            })
    }

    /// Check if file exists
        let full_path = self.get_full_path(path)?;
        Ok(fs::try_exists(&full_path).await.unwrap_or(false))
    }

    /// Get file metadata
        let full_path = self.get_full_path(path)?;
        self.create_file_metadata(&full_path).await
    }

    /// List files in directory
        let full_path = self.get_full_path(path)?;

        let mut entries =
            fs::read_dir(&full_path)
                .await
                .map_err(|e| crate::error::NestGateError::storage_error(
                    "filesystem_read_dir",
                    &format!("Failed to read directory: {}", e),
                    None
                ))?;

        let mut files = Vec::new();
        while let Some(entry) =
            entries
                .next_entry()
                .await
                .map_err(|e| crate::error::NestGateError::storage_error(
                    "filesystem_read_entry",
                    &format!("Failed to read directory entry: {}", e),
                    None
                ))?
        {
            if let Some(name) = entry.file_name().to_str() {
                files.push(name.to_string());
            }
        }

        Ok(files)
    }

    /// Create directory
        let full_path = self.get_full_path(path)?;

        fs::create_dir_all(&full_path)
            .await
            .map_err(|_e| crate::error::NestGateError::storage_error(
                error_message: format!("Failed to create directory: {e}"),
            })
    }

    /// Delete directory
        let full_path = self.get_full_path(path)?;

        fs::remove_dir_all(&full_path)
            .await
            .map_err(|_e| crate::error::NestGateError::storage_error(
                error_message: format!("Failed to delete directory: {e}"),
            })
    }
}
