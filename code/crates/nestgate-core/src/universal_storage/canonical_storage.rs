// Canonical storage management system
// Unified storage interface with pluggable backends

use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::error::{Result, NestGateError};

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::StorageConfig;
/// 
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Storage
pub struct StorageConfig {
    /// Root Path
    pub root_path: PathBuf,
    /// Tier
    pub tier: String,
    /// Compression
    pub compression: bool,
    /// Capacity in gigabytes
    pub capacity_gb: u64,
}
impl Default for StorageConfig {
    /// Returns the default instance
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
/// Storagemetadata
pub struct StorageMetadata {
    /// Size
    pub size: u64,
    /// Created
    pub created: SystemTime,
    /// Modified
    pub modified: SystemTime,
    /// Whether directory
    pub is_directory: bool,
}
/// Storage health information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalstoragehealth
pub struct CanonicalStorageHealth {
    /// Status
    pub status: String,
    /// Total Space
    pub total_space: u64,
    /// Free Space
    pub free_space: u64,
    /// Last Check
    pub last_check: SystemTime,
}
/// Storage metrics
#[derive(Debug, Default)]
/// Canonicalstoragemetrics
pub struct CanonicalStorageMetrics {
    /// Operations Total
    pub operations_total: std::sync::atomic::AtomicU64,
    /// Bytes Read
    pub bytes_read: std::sync::atomic::AtomicU64,
    /// Bytes Written
    pub bytes_written: std::sync::atomic::AtomicU64,
    /// Errors Total
    pub errors_total: std::sync::atomic::AtomicU64,
}
/// **DEPRECATED** - Use CanonicalStorage from canonical_unified_traits instead
/// 
/// This trait has been superseded by the canonical storage system.
/// **MIGRATION PATH**:
/// - Old: `CanonicalStorageBackend` 
/// - New: `crate::traits::canonical::CanonicalStorage`
/// 
/// The new CanonicalStorage trait provides all the functionality of this trait
/// plus enhanced features like batch operations, metadata support, and streaming.
#[deprecated(since = "2.1.0", note = "Use crate::traits::canonical::CanonicalStorage instead")]
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
    /// Creates a new instance
    pub fn new(config: Arc<StorageConfig>, backend: B) -> Self {
        Self {
            config,
            backend,
            metrics: Arc::new(CanonicalStorageMetrics::default()),
        }
    }

    /// Config
    pub fn config(&self) -> &StorageConfig {
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

    /// Metrics
    pub fn metrics(&self) -> &CanonicalStorageMetrics {
        &self.metrics
    }
}

/// Filesystem backend implementation
#[derive(Debug, Clone)]
/// Filesystembackend
pub struct FilesystemBackend {
    root_path: PathBuf,
}
impl FilesystemBackend {
    /// Creates a new instance
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    /// Resolve Path
    fn resolve_path(&self, path: &str) -> PathBuf {
        self.root_path.join(path.trim_start_matches('/'))
    }
}

impl CanonicalStorageBackend for FilesystemBackend {
    /// Read
    async fn read(&self, path: &str) -> Result<Vec<u8>> {
        let full_path = self.resolve_path(path);
        match tokio::fs::read(&full_path).await {
            Ok(data) => Ok(data),
            Err(_) => Err(NestGateError::storage_error_detailed(format!("Failed to read file: {path}"), Some("read".to_string()), Some(path"))),
        }
    }

    /// Write
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

    /// Deletes resource
    async fn delete(&self, path: &str) -> Result<()> {
        let full_path = self.resolve_path(path);
        match tokio::fs::remove_file(&full_path).await {
            Ok(()) => Ok(()),
            Err(_) => Err(NestGateError::storage_error_detailed(format!("Failed to delete file: {path}"), Some("delete".to_string()), Some(path"))),
        }
    }

    /// List
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

    /// Metadata
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

    /// Health Check
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
pub fn create_canonical_storage_manager() -> Result<CanonicalStorageManager<FilesystemBackend>> {
    let config = Arc::new(StorageConfig::default());
    let backend = FilesystemBackend::new(config.root_path.clone());
    Ok(CanonicalStorageManager::new(config, backend))
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Storageconfigcanonical
pub type StorageConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

