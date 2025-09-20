// Canonical storage management system
// Unified storage interface with pluggable backends

use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::error::{Result, NestGateError};

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub root_path: PathBuf,
    pub tier: String,
    pub compression: bool,
    pub capacity_gb: u64,
}
impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("/var/nestgate/storage"),
            tier: "standard".to_string(),
            compression: false,
            capacity_gb: 100,
        }
    }
}

/// Storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    pub size: u64,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub is_directory: bool,
}
/// Storage health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalStorageHealth {
    pub status: String,
    pub total_space: u64,
    pub free_space: u64,
    pub last_check: SystemTime,
}
/// Storage metrics
#[derive(Debug, Default)]
pub struct CanonicalStorageMetrics {
    pub operations_total: std::sync::atomic::AtomicU64,
    pub bytes_read: std::sync::atomic::AtomicU64,
    pub bytes_written: std::sync::atomic::AtomicU64,
    pub errors_total: std::sync::atomic::AtomicU64,
}
/// **DEPRECATED** - Use CanonicalStorage from canonical_unified_traits instead
/// 
/// This trait has been superseded by the canonical storage system.
/// **MIGRATION PATH**:
/// - Old: `CanonicalStorageBackend` 
/// - New: `crate::traits::canonical_unified_traits::CanonicalStorage`
/// 
/// The new CanonicalStorage trait provides all the functionality of this trait
/// plus enhanced features like batch operations, metadata support, and streaming.
#[deprecated(since = "2.1.0", note = "Use crate::traits::canonical_unified_traits::CanonicalStorage instead")]
pub trait CanonicalStorageBackend: Send + Sync {
    /// Read data from storage - native async, no Future boxing
    fn read(&self, path: &str) -> impl std::future::Future<Output = Result<Vec<u8>>> + Send;
    
    /// Write data to storage - native async
    fn write(&self, path: &str, data: &[u8]) -> impl std::future::Future<Output = Result<()>> + Send;
    
    /// Delete data from storage - native async
    fn delete(&self, path: &str) -> impl std::future::Future<Output = Result<()>> + Send;
    
    /// List directory contents - native async
    fn list(&self, path: &str) -> impl std::future::Future<Output = Result<Vec<String>>> + Send;
    
    /// Get storage metadata - native async
    fn metadata(&self, path: &str) -> impl std::future::Future<Output = Result<StorageMetadata>> + Send;
    
    /// Perform health check - native async
    fn health_check(&self) -> impl std::future::Future<Output = Result<CanonicalStorageHealth>> + Send;
}
/// Canonical storage manager
pub struct CanonicalStorageManager<B: CanonicalStorageBackend> {
    config: Arc<StorageConfig>,
    backend: B,
    metrics: Arc<CanonicalStorageMetrics>,
}
impl<B: CanonicalStorageBackend> CanonicalStorageManager<B> {
    pub const fn new(config: Arc<StorageConfig>, backend: B) -> Self {
        Self {
            config,
            backend,
            metrics: Arc::new(CanonicalStorageMetrics::default()),
        }
    }

    pub const fn config(&self) -> &StorageConfig {
        &self.config
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn read(&self, path: &str) -> Result<Vec<u8>>   {
        self.metrics.operations_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match self.backend.read(path).await {
            Ok(data) => {
                self.metrics.bytes_read.fetch_add(data.len() as u64, std::sync::atomic::Ordering::Relaxed);
                Ok(data)
            }
            Err(e) => {
                self.metrics.errors_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Err(e)
            }
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn write(&self, path: &str, data: &[u8]) -> Result<()>   {
        self.metrics.operations_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match self.backend.write(path, data).await {
            Ok(()) => {
                self.metrics.bytes_written.fetch_add(data.len() as u64, std::sync::atomic::Ordering::Relaxed);
                Ok(())
            }
            Err(e) => {
                self.metrics.errors_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Err(e)
            }
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn delete(&self, path: &str) -> Result<()>   {
        self.metrics.operations_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.backend.delete(path).await
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn list(&self, path: &str) -> Result<Vec<String>>   {
        self.metrics.operations_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.backend.list(path).await
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn metadata(&self, path: &str) -> Result<StorageMetadata>   {
        self.metrics.operations_total.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.backend.metadata(path).await
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn health_check(&self) -> Result<CanonicalStorageHealth>   {
        self.backend.health_check().await
    }

    pub const fn metrics(&self) -> &CanonicalStorageMetrics {
        &self.metrics
    }
}

/// Filesystem backend implementation
#[derive(Debug, Clone)]
pub struct FilesystemBackend {
    root_path: PathBuf,
}
impl FilesystemBackend {
    pub const fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    fn resolve_path(&self, path: &str) -> PathBuf {
        self.root_path.join(path.trim_start_matches('/'))
    }
}

impl CanonicalStorageBackend for FilesystemBackend {
    async fn read(&self, path: &str) -> Result<Vec<u8>> {
        let full_path = self.resolve_path(path);
        match tokio::fs::read(&full_path).await {
            Ok(data) => Ok(data),
            Err(_) => Err(NestGateError::storage_error_detailed(format!("Failed to read file: {path}"), Some("read".to_string()), Some(path"))),
        }
    }

    async fn write(&self, path: &str, data: &[u8]) -> Result<()> {
        let full_path = self.resolve_path(path);
        if let Some(parent) = full_path.parent() {
            let _ = tokio::fs::create_dir_all(parent).await;
        }
        match tokio::fs::write(&full_path, data).await {
            Ok(()) => Ok(()),
            Err(_) => Err(NestGateError::storage_error_detailed(format!("Failed to write file: {path}"), Some("write".to_string()), Some(path"))),
        }
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let full_path = self.resolve_path(path);
        match tokio::fs::remove_file(&full_path).await {
            Ok(()) => Ok(()),
            Err(_) => Err(NestGateError::storage_error_detailed(format!("Failed to delete file: {path}"), Some("delete".to_string()), Some(path"))),
        }
    }

    async fn list(&self, path: &str) -> Result<Vec<String>> {
        let full_path = self.resolve_path(path);
        match tokio::fs::read_dir(&full_path).await {
            Ok(mut entries) => {
                let mut files = Vec::new();
                while let Ok(Some(entry)) = entries.next_entry().await {
                    if let Some(name) = entry.file_name().to_str() {
                        files.push(name.to_string());
                    }
                }
                Ok(files)
            }
            Err(_) => Err(NestGateError::storage_error_detailed(format!("Failed to list directory: {path}"), Some("list".to_string()), Some(path"))),
        }
    }

    async fn metadata(&self, path: &str) -> Result<StorageMetadata> {
        let full_path = self.resolve_path(path);
        match tokio::fs::metadata(&full_path).await {
            Ok(metadata) => Ok(StorageMetadata {
                size: metadata.len(),
                created: metadata.created().unwrap_or(SystemTime::UNIX_EPOCH),
                modified: metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH),
                is_directory: metadata.is_dir(),
            }),
            Err(_) => Err(NestGateError::storage_error_detailed(format!("Failed to get metadata for: {path}"), Some("metadata".to_string()), Some(path"))),
        }
    }

    async fn health_check(&self) -> Result<CanonicalStorageHealth> {
        Ok(CanonicalStorageHealth {
            status: "healthy".to_string(),
            total_space: 1024 * 1024 * 1024,
            free_space: 512 * 1024 * 1024,
            last_check: SystemTime::now(),
        })
    }
}

/// Convenience constructor for filesystem backend
#[must_use]
pub const fn create_canonical_storage_manager() -> Result<CanonicalStorageManager<FilesystemBackend>> {
    let config = Arc::new(StorageConfig::default());
    let backend = FilesystemBackend::new(config.root_path.clone());
    Ok(CanonicalStorageManager::new(config, backend))
}
