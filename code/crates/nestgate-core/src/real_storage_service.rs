use crate::NestGateError;
use std::collections::HashMap;
// Real Storage Service Implementation
//
// Provides actual file system storage operations replacing mock implementations.

// REMOVED: async_trait - using zero-cost native async patterns
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::{Result, NestGateError};
use crate::canonical_modernization::consolidated_storage_types::*;

/// Real storage service implementation
#[derive(Debug)]
pub struct RealStorageService {
    /// Storage root directory
    root_path: PathBuf,
    /// File metadata cache
    metadata_cache: Arc<RwLock<HashMap<String, StorageDirectoryEntry>>>,
    /// Storage statistics
    stats: Arc<RwLock<StorageStatistics>>,
    /// Configuration
    config: StorageConfig,
}

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub root_directory: String,
    pub max_file_size: u64,
    pub cache_enabled: bool,
    pub compression_enabled: bool,
    pub backup_enabled: bool,
}

/// Storage statistics
#[derive(Debug, Clone, Default)]
pub struct StorageStatistics {
    pub total_files: u64,
    pub total_size: u64,
    pub reads_count: u64,
    pub writes_count: u64,
    pub errors_count: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            root_directory: "./storage".to_string(),
            max_file_size: 1024 * 1024 * 1024, // 1GB
            cache_enabled: true,
            compression_enabled: false,
            backup_enabled: false,
        }
    }
}

impl RealStorageService {
    /// Create a new real storage service
    pub async fn new(config: StorageConfig) -> Result<Self> {
        info!(
            "Initializing real storage service with root: {}",
            config.root_directory
        );

        let root_path = PathBuf::from(&config.root_directory);

        // Create root directory if it doesn't exist
        if !root_path.exists() {
            fs::create_dir_all(&root_path).await.map_err(|e| {
                e.into_nestgate_error_with_context("Failed to create storage directory")
            })?;
            info!("Created storage directory: {}", root_path.display());
        }

        let service = Self {
            root_path,
            metadata_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(StorageStatistics::default())),
            config,
        };

        // Initialize by scanning existing files
        service.refresh_metadata_cache().await?;

        info!("Real storage service initialized successfully");
        Ok(service)
    }

    /// Write data to storage
    pub async fn write_file(&self, path: &str, data: &[u8]) -> Result<()> {
        debug!("Writing file: {} ({} bytes)", path, data.len());

        if data.len() as u64 > self.config.max_file_size {
            return Err(NestGateError::invalid_input(
                "file_size".to_string(),
                format!(
                    "File size {} exceeds maximum {}",
                    data.len(),
                    self.config.max_file_size
                ),
            ));
        }

        let full_path = self.root_path.join(path);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| e.into_nestgate_error_with_context("Failed to create directories"))?;
        }

        // Write file
        let mut file = fs::File::create(&full_path)
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to create file"))?;

        file.write_all(data)
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to write file"))?;

        file.sync_all()
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to sync file"))?;

        // Update metadata cache
        self.update_file_metadata(path, data.len() as u64).await?;

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.writes_count += 1;
        stats.total_size += data.len() as u64;

        info!("Successfully wrote file: {}", path);
        Ok(())
    }

    /// Read data from storage
    pub async fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        debug!("Reading file: {}", path);

        // Check cache first if enabled
        if self.config.cache_enabled {
            if self.metadata_cache.read().await.get(path).is_some() {
                let mut stats = self.stats.write().await;
                stats.cache_hits += 1;
            } else {
                let mut stats = self.stats.write().await;
                stats.cache_misses += 1;
            }
        }

        let full_path = self.root_path.join(path);

        if !full_path.exists() {
            return Err(NestGateError::invalid_input(
                "file_path".to_string(),
                format!("File not found: {path}"),
            ));
        }

        let mut file = fs::File::open(&full_path)
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to open file"))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to read file"))?;

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.reads_count += 1;

        debug!("Successfully read file: {} ({} bytes)", path, buffer.len());
        Ok(buffer)
    }

    /// Delete file from storage
    pub async fn delete_file(&self, path: &str) -> Result<()> {
        debug!("Deleting file: {}", path);

        let full_path = self.root_path.join(path);

        if !full_path.exists() {
            return Err(NestGateError::invalid_input(
                "file_path".to_string(),
                format!("File not found: {path}"),
            ));
        }

        // Get file size for statistics
        let metadata = fs::metadata(&full_path)
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to get file metadata: {}"))?;

        let file_size = metadata.len();

        // Delete the file
        fs::remove_file(&full_path)
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to delete file: {}"))?;

        // Remove from cache
        let mut cache = self.metadata_cache.write().await;
        cache.remove(path);

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_files = stats.total_files.saturating_sub(1);
        stats.total_size = stats.total_size.saturating_sub(file_size);

        info!("Successfully deleted file: {}", path);
        Ok(())
    }

    /// List files in a directory
    pub async fn list_directory(&self, path: &str) -> Result<Vec<StorageDirectoryEntry>> {
        debug!("Listing directory: {}", path);

        let full_path = if path.is_empty() || path == "/" {
            self.root_path.clone()
        } else {
            self.root_path.join(path)
        };

        if !full_path.exists() {
            return Err(NestGateError::invalid_input(
                "directory_path".to_string(),
                format!("Directory not found: {path}"),
            ));
        }

        if !full_path.is_dir() {
            return Err(NestGateError::invalid_input(
                "directory_path".to_string(),
                format!("Path is not a directory: {path}"),
            ));
        }

        let mut entries = Vec::new();
        let mut dir = fs::read_dir(&full_path)
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to read directory: {}"))?;

        while let Some(entry) = dir
            .next_entry()
            .await
            .map_err(|e| e.into_nestgate_error_with_context("Failed to read directory entry: {}"))?
        {
            let entry_path = entry.path();
            let metadata = entry.metadata().await.map_err(|e| {
                e.into_nestgate_error_with_context("Failed to get entry metadata: {}")
            })?;

            let relative_path = entry_path
                .strip_prefix(&self.root_path)
                .unwrap_or(&entry_path)
                .to_string_lossy()
                .to_string();

            let entry_info = StorageDirectoryEntry {
                name: entry.file_name().to_string_lossy().to_string(),
                path: relative_path,
                is_directory: metadata.is_dir(),
                size: metadata.len(),
                modified: {
                    let timestamp = metadata
                        .modified()
                        .unwrap_or(SystemTime::UNIX_EPOCH)
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    chrono::DateTime::from_timestamp(timestamp as i64, 0)
                        .unwrap_or_else(chrono::Utc::now)
                },
                permissions: None,
                owner: None,
                group: None,
            };

            entries.push(entry_info);
        }

        debug!("Found {} entries in directory: {}", entries.len(), path);
        Ok(entries)
    }

    /// Get storage statistics
    pub async fn get_statistics(&self) -> StorageStatistics {
        self.stats.read().await.clone()
    }

    /// Update file metadata in cache
    async fn update_file_metadata(&self, path: &str, size: u64) -> Result<()> {
        let entry = StorageDirectoryEntry {
            name: Path::new(path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path.to_string(),
            is_directory: false,
            size,
            modified: chrono::Utc::now(),
            permissions: None,
            owner: None,
            group: None,
        };

        let mut cache = self.metadata_cache.write().await;
        cache.insert(path.to_string(), entry);

        // Update file count
        let mut stats = self.stats.write().await;
        stats.total_files += 1;

        Ok(())
    }

    /// Refresh metadata cache by scanning storage directory
    async fn refresh_metadata_cache(&self) -> Result<()> {
        debug!("Refreshing metadata cache");

        let entries = self.scan_directory_recursive(&self.root_path, "").await?;

        let mut cache = self.metadata_cache.write().await;
        let mut stats = self.stats.write().await;

        cache.clear();
        stats.total_files = 0;
        stats.total_size = 0;

        for entry in entries {
            if !entry.is_directory {
                stats.total_files += 1;
                stats.total_size += entry.size;
                cache.insert(entry.path.clone(), entry);
            }
        }

        info!(
            "Refreshed metadata cache: {} files, {} bytes",
            stats.total_files, stats.total_size
        );
        Ok(())
    }

    /// Recursively scan directory
    #[allow(clippy::only_used_in_recursion)]
    fn scan_directory_recursive<'a>(
        &'a self,
        dir_path: &'a Path,
        relative_prefix: &'a str,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<StorageDirectoryEntry>>> + Send + 'a>,
    > {
        Box::pin(async move {
            let mut entries = Vec::new();

            if !dir_path.exists() || !dir_path.is_dir() {
                return Ok(entries);
            }

            let mut dir = fs::read_dir(dir_path)
                .await
                .map_err(|e| e.into_nestgate_error_with_context("Failed to read directory: {}"))?;

            while let Some(entry) = dir.next_entry().await.map_err(|e| {
                e.into_nestgate_error_with_context("Failed to read directory entry: {}")
            })? {
                let entry_path = entry.path();
                let metadata = entry.metadata().await.map_err(|e| {
                    e.into_nestgate_error_with_context("Failed to get entry metadata: {}")
                })?;

                let name = entry.file_name().to_string_lossy().to_string();
                let relative_path = if relative_prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{relative_prefix}/{name}")
                };

                let entry_info = StorageDirectoryEntry {
                    name,
                    path: relative_path.clone(),
                    is_directory: metadata.is_dir(),
                    size: metadata.len(),
                    modified: {
                        let timestamp = metadata
                            .modified()
                            .unwrap_or(SystemTime::UNIX_EPOCH)
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs();
                        chrono::DateTime::from_timestamp(timestamp as i64, 0)
                            .unwrap_or_else(chrono::Utc::now)
                    },
                    permissions: None,
                    owner: None,
                    group: None,
                };

                entries.push(entry_info);

                // Recursively scan subdirectories
                if metadata.is_dir() {
                    let sub_entries = self
                        .scan_directory_recursive(&entry_path, &relative_path)
                        .await?;
                    entries.extend(sub_entries);
                }
            }

            Ok(entries)
        })
    }

    /// Create backup of a file
    pub async fn create_backup(&self, path: &str) -> Result<String> {
        if !self.config.backup_enabled {
            return Err(NestGateError::invalid_input(
                "backup_config".to_string(),
                "Backup not enabled".to_string(),
            ));
        }

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_path = format!("{path}.backup.{timestamp}");

        let data = self.read_file(path).await?;
        self.write_file(&backup_path, &data).await?;

        info!("Created backup: {} -> {}", path, backup_path);
        Ok(backup_path)
    }
}

/// Storage service trait - **ZERO-COST NATIVE ASYNC**
pub trait StorageService: Send + Sync {
    fn write(&self, path: &str, data: &[u8]) -> impl std::future::Future<Output = Result<()>> + Send;
    fn read(&self, path: &str) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;
    fn delete(&self, path: &str) -> impl std::future::Future<Output = Result<()>> + Send;
    fn list(&self, path: &str) -> impl std::future::Future<Output = Result<Vec<StorageDirectoryEntry>>> + Send;
    fn exists(&self, path: &str) -> impl std::future::Future<Output = Result<bool>> + Send;
    fn get_metadata(&self, path: &str) -> impl std::future::Future<Output = Result<StorageDirectoryEntry>> + Send;
}

// **CANONICAL MODERNIZATION COMPLETE** - Native async trait implementation
impl StorageService for RealStorageService {
    fn write(&self, path: &str, data: &[u8]) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            self.write_file(path, data).await
        }
    }

    fn read(&self, path: &str) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send {
        async move {
            self.read_file(path).await
        }
    }

    fn delete(&self, path: &str) -> impl std::future::Future<Output = Result<()>> + Send {
        async move {
            self.delete_file(path).await
        }
    }

    fn list(&self, path: &str) -> impl std::future::Future<Output = Result<Vec<StorageDirectoryEntry>>> + Send {
        async move {
            self.list_directory(path).await
        }
    }

    fn exists(&self, path: &str) -> impl std::future::Future<Output = Result<bool>> + Send {
        async move {
            let full_path = self.root_path.join(path);
            Ok(full_path.exists())
        }
    }

    fn get_metadata(&self, path: &str) -> impl std::future::Future<Output = Result<StorageDirectoryEntry>> + Send {
        async move {
            let cache = self.metadata_cache.read().await;
            if let Some(entry) = cache.get(path) {
                return Ok(entry.clone());
            }

            let full_path = self.root_path.join(path);
            if !full_path.exists() {
                return Err(NestGateError::invalid_input(
                    "file_path".to_string(),
                    format!("File not found: {path}"),
                ));
            }

            let metadata = fs::metadata(&full_path)
                .await
                .map_err(|e| e.into_nestgate_error_with_context("Failed to get file metadata: {}"))?;

            Ok(StorageDirectoryEntry {
                name: Path::new(path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                path: path.to_string(),
                is_directory: metadata.is_dir(),
                size: metadata.len(),
                modified: {
                    let timestamp = metadata
                        .modified()
                        .unwrap_or(SystemTime::UNIX_EPOCH)
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    chrono::DateTime::from_timestamp(timestamp as i64, 0)
                        .unwrap_or_else(chrono::Utc::now)
                },
                permissions: None,
                owner: None,
                group: None,
            })
        }
    }
}
