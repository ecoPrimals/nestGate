//! # Filesystem Backend - Unified Self-Contained Storage
//!
//! This module integrates compression, checksums, and snapshots into a unified
//! storage backend that works on any standard filesystem without requiring ZFS.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────┐
//! │              FilesystemBackend API                        │
//! │  (read_file, write_file, snapshot, verify, list)         │
//! └────────┬─────────────┬────────────────┬──────────────────┘
//!          │             │                │
//!     ┌────▼────┐   ┌────▼─────┐   ┌─────▼─────┐
//!     │Compress │   │Checksum  │   │Snapshot   │
//!     │(Phase1B)│   │(Phase1B) │   │(Phase1B)  │
//!     └─────────┘   └──────────┘   └───────────┘
//!                         │
//!                    ┌────▼──────┐
//!                    │Filesystem │
//!                    │(ext4/NTFS)│
//!                    └───────────┘
//! ```
//!
//! ## Features
//!
//! - **Automatic Compression**: Transparent compression on write, decompression on read
//! - **Data Integrity**: Automatic checksums on write, verification on read
//! - **Snapshots**: Copy-on-write snapshots using Phase 1B snapshot system
//! - **Metadata Tracking**: Efficient file index with compression/checksum info
//! - **Zero System Dependencies**: Pure Rust, works on any filesystem
//!
//! ## Example
//!
//! ```rust,no_run
//! use nestgate_core::universal_storage::filesystem_backend::FilesystemBackend;
//! use nestgate_core::universal_storage::{CompressionAlgorithm, ChecksumAlgorithm};
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create backend with compression and checksums
//! let backend = FilesystemBackend::new(Path::new("/data/storage"))
//!     .with_compression(CompressionAlgorithm::Lz4)
//!     .with_checksum(ChecksumAlgorithm::Blake3)
//!     .build()
//!     .await?;
//!
//! // Write file (automatically compressed and checksummed)
//! backend.write_file("document.txt", b"Hello, World!").await?;
//!
//! // Read file (automatically verified and decompressed)
//! let data = backend.read_file("document.txt").await?;
//!
//! // Create snapshot
//! let snapshot_id = backend.create_snapshot("backup-001").await?;
//!
//! // Verify all files
//! let report = backend.verify_all().await?;
//! # Ok(())
//! # }
//! ```

use crate::universal_storage::{
    ChecksumAlgorithm, CompressionAlgorithm, RustChecksummer, RustCompressor, SnapshotManager,
    SnapshotMetadata, SnapshotStrategy,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Filesystem backend errors
#[derive(Debug, Error)]
pub enum FilesystemError {
    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Compression error
    #[error("Compression error: {0}")]
    CompressionError(String),

    /// Checksum error
    #[error("Checksum error: {0}")]
    ChecksumError(String),

    /// Snapshot error
    #[error("Snapshot error: {0}")]
    SnapshotError(String),

    /// File not found
    #[error("File not found: {0}")]
    FileNotFound(String),

    /// Checksum mismatch (corruption detected)
    #[error("Checksum mismatch for {0}: expected {1}, got {2}")]
    ChecksumMismatch(String, String, String),

    /// Invalid metadata
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Result type for filesystem operations
pub type Result<T> = std::result::Result<T, FilesystemError>;

/// File metadata tracked by the backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    /// Logical file path (relative to backend root)
    pub path: String,

    /// Physical file path on disk
    pub physical_path: PathBuf,

    /// Original (uncompressed) size in bytes
    pub original_size: u64,

    /// Compressed size in bytes (if compressed)
    pub compressed_size: u64,

    /// Checksum of original data
    pub checksum: Vec<u8>,

    /// Compression algorithm used
    pub compression: CompressionAlgorithm,

    /// Checksum algorithm used
    pub checksum_algorithm: ChecksumAlgorithm,

    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Last modified timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,

    /// Custom tags
    pub tags: HashMap<String, String>,
}

impl FileMetadata {
    /// Create new file metadata
    pub fn new(
        path: String,
        physical_path: PathBuf,
        original_size: u64,
        compressed_size: u64,
        checksum: Vec<u8>,
        compression: CompressionAlgorithm,
        checksum_algorithm: ChecksumAlgorithm,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            path,
            physical_path,
            original_size,
            compressed_size,
            checksum,
            compression,
            checksum_algorithm,
            created_at: now,
            modified_at: now,
            tags: HashMap::new(),
        }
    }

    /// Calculate compression ratio
    pub fn compression_ratio(&self) -> f64 {
        if self.original_size == 0 {
            return 1.0;
        }
        self.compressed_size as f64 / self.original_size as f64
    }

    /// Get space saved by compression
    pub fn space_saved(&self) -> u64 {
        self.original_size.saturating_sub(self.compressed_size)
    }
}

/// Metadata index for tracking all files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataIndex {
    /// Map of logical path to metadata
    pub files: HashMap<String, FileMetadata>,

    /// Total original size of all files
    pub total_original_size: u64,

    /// Total compressed size of all files
    pub total_compressed_size: u64,

    /// Index version
    pub version: u32,
}

impl MetadataIndex {
    /// Create new empty index
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            total_original_size: 0,
            total_compressed_size: 0,
            version: 1,
        }
    }

    /// Add or update file metadata
    pub fn insert(&mut self, metadata: FileMetadata) {
        // Remove old stats if updating existing file
        if let Some(old) = self.files.get(&metadata.path) {
            self.total_original_size -= old.original_size;
            self.total_compressed_size -= old.compressed_size;
        }

        // Add new stats
        self.total_original_size += metadata.original_size;
        self.total_compressed_size += metadata.compressed_size;

        self.files.insert(metadata.path.clone(), metadata);
    }

    /// Remove file metadata
    pub fn remove(&mut self, path: &str) -> Option<FileMetadata> {
        if let Some(metadata) = self.files.remove(path) {
            self.total_original_size -= metadata.original_size;
            self.total_compressed_size -= metadata.compressed_size;
            Some(metadata)
        } else {
            None
        }
    }

    /// Get overall compression ratio
    pub fn compression_ratio(&self) -> f64 {
        if self.total_original_size == 0 {
            return 1.0;
        }
        self.total_compressed_size as f64 / self.total_original_size as f64
    }

    /// Get total space saved
    pub fn space_saved(&self) -> u64 {
        self.total_original_size
            .saturating_sub(self.total_compressed_size)
    }
}

impl Default for MetadataIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for filesystem backend
#[derive(Debug, Clone)]
pub struct FilesystemConfig {
    /// Root directory for storage
    pub root_path: PathBuf,

    /// Compression algorithm to use
    pub compression: CompressionAlgorithm,

    /// Checksum algorithm to use
    pub checksum: ChecksumAlgorithm,

    /// Snapshot strategy
    pub snapshot_strategy: SnapshotStrategy,

    /// Verify checksums on read
    pub verify_on_read: bool,

    /// Enable automatic snapshots
    pub auto_snapshot: bool,
}

impl Default for FilesystemConfig {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("./storage"),
            compression: CompressionAlgorithm::Lz4,
            checksum: ChecksumAlgorithm::Blake3,
            snapshot_strategy: SnapshotStrategy::Auto,
            verify_on_read: true,
            auto_snapshot: false,
        }
    }
}

/// Unified filesystem backend
pub struct FilesystemBackend {
    /// Configuration
    config: FilesystemConfig,

    /// Root directory
    root_path: PathBuf,

    /// Data directory (stores actual files)
    data_path: PathBuf,

    /// Metadata directory
    meta_path: PathBuf,

    /// Compressor
    compressor: RustCompressor,

    /// Checksummer
    checksummer: RustChecksummer,

    /// Snapshot manager
    snapshot_manager: SnapshotManager,

    /// Metadata index
    index: MetadataIndex,
}

impl FilesystemBackend {
    /// Create a new filesystem backend
    ///
    /// # Arguments
    /// * `root_path` - Root directory for storage
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::filesystem_backend::FilesystemBackend;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let backend = FilesystemBackend::new(Path::new("/data/storage")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(root_path: &Path) -> Result<Self> {
        let config = FilesystemConfig {
            root_path: root_path.to_path_buf(),
            ..Default::default()
        };
        Self::with_config(config).await
    }

    /// Create a new filesystem backend with custom configuration
    pub async fn with_config(config: FilesystemConfig) -> Result<Self> {
        let root_path = config.root_path.clone();
        let data_path = root_path.join("data");
        let meta_path = root_path.join("meta");

        // Create directories
        fs::create_dir_all(&data_path).await?;
        fs::create_dir_all(&meta_path).await?;

        // Initialize components
        let compressor = RustCompressor::new(config.compression);
        let checksummer = RustChecksummer::new(config.checksum);
        let snapshot_manager = SnapshotManager::with_strategy(&data_path, config.snapshot_strategy)
            .await
            .map_err(|e| FilesystemError::SnapshotError(e.to_string()))?;

        // Load or create index
        let index_path = meta_path.join("index.json");
        let index = if index_path.exists() {
            let data = fs::read_to_string(&index_path).await?;
            serde_json::from_str(&data)
                .map_err(|e| FilesystemError::SerializationError(e.to_string()))?
        } else {
            MetadataIndex::new()
        };

        Ok(Self {
            config,
            root_path,
            data_path,
            meta_path,
            compressor,
            checksummer,
            snapshot_manager,
            index,
        })
    }

    /// Get backend configuration
    pub fn config(&self) -> &FilesystemConfig {
        &self.config
    }

    /// Get root path
    pub fn root_path(&self) -> &Path {
        &self.root_path
    }

    /// Get metadata index
    pub fn index(&self) -> &MetadataIndex {
        &self.index
    }

    /// Save metadata index to disk
    async fn save_index(&self) -> Result<()> {
        let index_path = self.meta_path.join("index.json");
        let json = serde_json::to_string_pretty(&self.index)
            .map_err(|e| FilesystemError::SerializationError(e.to_string()))?;

        // Write atomically with temp file + rename
        let temp_path = index_path.with_extension("json.tmp");
        let mut file = fs::File::create(&temp_path).await?;
        file.write_all(json.as_bytes()).await?;
        file.sync_all().await?;
        drop(file);

        fs::rename(&temp_path, &index_path).await?;

        Ok(())
    }

    /// Write a file with automatic compression and checksumming
    ///
    /// # Arguments
    /// * `path` - Logical file path (relative)
    /// * `data` - File contents
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::filesystem_backend::FilesystemBackend;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let backend = FilesystemBackend::new(Path::new("/tmp/test")).await?;
    /// backend.write_file("docs/readme.txt", b"Hello, World!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn write_file(&mut self, path: &str, data: &[u8]) -> Result<()> {
        // Calculate checksum of original data
        let checksum = self.checksummer.checksum(data);
        let original_size = data.len() as u64;

        // Compress data
        let compressed = self
            .compressor
            .compress(data)
            .map_err(|e| FilesystemError::CompressionError(e.to_string()))?;
        let compressed_size = compressed.len() as u64;

        // Determine physical path
        let physical_path = self.data_path.join(path);

        // Create parent directories
        if let Some(parent) = physical_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write compressed data
        let mut file = fs::File::create(&physical_path).await?;
        file.write_all(&compressed).await?;
        file.sync_all().await?;

        // Update metadata
        let metadata = FileMetadata::new(
            path.to_string(),
            physical_path,
            original_size,
            compressed_size,
            checksum,
            self.config.compression,
            self.config.checksum,
        );

        self.index.insert(metadata);
        self.save_index().await?;

        Ok(())
    }

    /// Read a file with automatic verification and decompression
    ///
    /// # Arguments
    /// * `path` - Logical file path
    ///
    /// # Returns
    /// File contents (decompressed and verified)
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::filesystem_backend::FilesystemBackend;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let backend = FilesystemBackend::new(Path::new("/tmp/test")).await?;
    /// let data = backend.read_file("docs/readme.txt").await?;
    /// println!("Read {} bytes", data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_file(&self, path: &str) -> Result<Vec<u8>> {
        // Get metadata
        let metadata = self
            .index
            .files
            .get(path)
            .ok_or_else(|| FilesystemError::FileNotFound(path.to_string()))?;

        // Read compressed data
        let compressed = fs::read(&metadata.physical_path).await?;

        // Decompress
        let compressor = RustCompressor::new(metadata.compression);
        let decompressed = compressor
            .decompress(&compressed, Some(metadata.original_size as usize))
            .map_err(|e| FilesystemError::CompressionError(e.to_string()))?;

        // Verify checksum if enabled
        if self.config.verify_on_read {
            let checksummer = RustChecksummer::new(metadata.checksum_algorithm);
            let actual_checksum = checksummer.checksum(&decompressed);

            if actual_checksum != metadata.checksum {
                return Err(FilesystemError::ChecksumMismatch(
                    path.to_string(),
                    hex::encode(&metadata.checksum),
                    hex::encode(&actual_checksum),
                ));
            }
        }

        Ok(decompressed)
    }

    /// Check if a file exists
    pub fn exists(&self, path: &str) -> bool {
        self.index.files.contains_key(path)
    }

    /// List all files
    pub fn list_files(&self) -> Vec<&FileMetadata> {
        self.index.files.values().collect()
    }

    /// Get file metadata
    pub fn get_metadata(&self, path: &str) -> Option<&FileMetadata> {
        self.index.files.get(path)
    }

    /// Delete a file
    pub async fn delete_file(&mut self, path: &str) -> Result<()> {
        let metadata = self
            .index
            .remove(path)
            .ok_or_else(|| FilesystemError::FileNotFound(path.to_string()))?;

        // Delete physical file
        if metadata.physical_path.exists() {
            fs::remove_file(&metadata.physical_path).await?;
        }

        self.save_index().await?;

        Ok(())
    }

    /// Create a snapshot
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::filesystem_backend::FilesystemBackend;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let backend = FilesystemBackend::new(Path::new("/tmp/test")).await?;
    /// let snapshot_id = backend.create_snapshot("backup-daily").await?;
    /// println!("Created snapshot: {}", snapshot_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_snapshot(&self, name: &str) -> Result<String> {
        self.snapshot_manager
            .create_snapshot(name, "")
            .await
            .map_err(|e| FilesystemError::SnapshotError(e.to_string()))
    }

    /// List all snapshots
    pub async fn list_snapshots(&self) -> Result<Vec<SnapshotMetadata>> {
        self.snapshot_manager
            .list_snapshots()
            .await
            .map_err(|e| FilesystemError::SnapshotError(e.to_string()))
    }

    /// Rollback to a snapshot
    pub async fn rollback(&self, snapshot_id: &str) -> Result<()> {
        self.snapshot_manager
            .rollback(snapshot_id)
            .await
            .map_err(|e| FilesystemError::SnapshotError(e.to_string()))
    }

    /// Verify all files (check checksums)
    ///
    /// Returns a map of file paths to verification status
    pub async fn verify_all(&self) -> Result<HashMap<String, bool>> {
        let mut results = HashMap::new();

        for path in self.index.files.keys() {
            // Read and verify
            let is_valid = match self.read_file(path).await {
                Ok(_) => true,
                Err(FilesystemError::ChecksumMismatch(_, _, _)) => false,
                Err(_) => false,
            };

            results.insert(path.clone(), is_valid);
        }

        Ok(results)
    }

    /// Get storage statistics
    pub fn stats(&self) -> StorageStats {
        StorageStats {
            file_count: self.index.files.len(),
            total_original_size: self.index.total_original_size,
            total_compressed_size: self.index.total_compressed_size,
            compression_ratio: self.index.compression_ratio(),
            space_saved: self.index.space_saved(),
        }
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    /// Number of files
    pub file_count: usize,

    /// Total original (uncompressed) size
    pub total_original_size: u64,

    /// Total compressed size
    pub total_compressed_size: u64,

    /// Overall compression ratio
    pub compression_ratio: f64,

    /// Total space saved by compression
    pub space_saved: u64,
}

/// Builder for FilesystemBackend
pub struct FilesystemBackendBuilder {
    config: FilesystemConfig,
}

impl FilesystemBackendBuilder {
    /// Create a new builder
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            config: FilesystemConfig {
                root_path,
                ..Default::default()
            },
        }
    }

    /// Set compression algorithm
    pub fn with_compression(mut self, compression: CompressionAlgorithm) -> Self {
        self.config.compression = compression;
        self
    }

    /// Set checksum algorithm
    pub fn with_checksum(mut self, checksum: ChecksumAlgorithm) -> Self {
        self.config.checksum = checksum;
        self
    }

    /// Set snapshot strategy
    pub fn with_snapshot_strategy(mut self, strategy: SnapshotStrategy) -> Self {
        self.config.snapshot_strategy = strategy;
        self
    }

    /// Enable/disable checksum verification on read
    pub fn verify_on_read(mut self, verify: bool) -> Self {
        self.config.verify_on_read = verify;
        self
    }

    /// Build the backend
    pub async fn build(self) -> Result<FilesystemBackend> {
        FilesystemBackend::with_config(self.config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_backend_creation() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let backend = FilesystemBackend::new(temp_dir.path()).await.expect("Storage operation failed");

        assert_eq!(backend.index().files.len(), 0);
        assert!(backend.root_path().exists());
    }

    #[tokio::test]
    async fn test_write_and_read_file() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let mut backend = FilesystemBackend::new(temp_dir.path()).await.expect("Storage operation failed");

        let test_data = b"Hello, World! This is a test file.";

        // Write file
        backend.write_file("test.txt", test_data).await.expect("Storage operation failed");

        // Verify metadata
        assert!(backend.exists("test.txt"));
        let metadata = backend.get_metadata("test.txt").expect("Storage operation failed");
        assert_eq!(metadata.original_size, test_data.len() as u64);

        // Read file
        let read_data = backend.read_file("test.txt").await.expect("Storage operation failed");
        assert_eq!(read_data, test_data);
    }

    #[tokio::test]
    async fn test_compression() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let mut backend = FilesystemBackend::new(temp_dir.path()).await.expect("Storage operation failed");

        // Write highly compressible data
        let test_data = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        backend
            .write_file("compressible.txt", test_data)
            .await
            .expect("Storage operation failed");

        let metadata = backend.get_metadata("compressible.txt").expect("Storage operation failed");

        // Should be compressed
        assert!(metadata.compressed_size < metadata.original_size);
        println!(
            "Compression ratio: {:.2}%",
            metadata.compression_ratio() * 100.0
        );

        // Read back should still work
        let read_data = backend.read_file("compressible.txt").await.expect("Storage operation failed");
        assert_eq!(read_data, test_data);
    }

    #[tokio::test]
    async fn test_checksum_verification() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let mut backend = FilesystemBackend::new(temp_dir.path()).await.expect("Storage operation failed");

        // Write file with data that compresses to a known size
        let original_data = b"Original data that will be compressed";
        backend.write_file("test.txt", original_data).await.expect("Storage operation failed");

        // Get the physical path
        let physical_path = {
            let metadata = backend.get_metadata("test.txt").expect("Storage operation failed");
            metadata.physical_path.clone()
        };

        // Read and corrupt the compressed data
        let mut compressed_data = fs::read(&physical_path).await.expect("Storage operation failed");
        if !compressed_data.is_empty() {
            let idx = compressed_data.len() / 2;
            compressed_data[idx] ^= 0xFF;
            fs::write(&physical_path, &compressed_data).await.expect("Storage operation failed");
        }

        // Read should fail (either with checksum mismatch or compression error)
        let result = backend.read_file("test.txt").await;
        assert!(result.is_err(), "Reading corrupted file should fail");
        // Can be either ChecksumMismatch or CompressionError depending on corruption
    }

    #[tokio::test]
    async fn test_file_deletion() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let mut backend = FilesystemBackend::new(temp_dir.path()).await.expect("Storage operation failed");

        backend.write_file("delete_me.txt", b"Test").await.expect("Storage operation failed");
        assert!(backend.exists("delete_me.txt"));

        backend.delete_file("delete_me.txt").await.expect("Storage operation failed");
        assert!(!backend.exists("delete_me.txt"));
    }

    #[tokio::test]
    async fn test_list_files() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let mut backend = FilesystemBackend::new(temp_dir.path()).await.expect("Storage operation failed");

        backend.write_file("file1.txt", b"Data 1").await.expect("Storage operation failed");
        backend.write_file("file2.txt", b"Data 2").await.expect("Storage operation failed");
        backend
            .write_file("dir/file3.txt", b"Data 3")
            .await
            .expect("Storage operation failed");

        let files = backend.list_files();
        assert_eq!(files.len(), 3);
    }

    #[tokio::test]
    async fn test_storage_stats() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let mut backend = FilesystemBackend::new(temp_dir.path()).await.expect("Storage operation failed");

        backend.write_file("file1.txt", b"Data").await.expect("Storage operation failed");
        backend.write_file("file2.txt", b"More data").await.expect("Storage operation failed");

        let stats = backend.stats();
        assert_eq!(stats.file_count, 2);
        assert!(stats.total_original_size > 0);
        assert!(stats.compression_ratio > 0.0);
    }

    #[tokio::test]
    async fn test_snapshot_integration() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let mut backend = FilesystemBackend::new(temp_dir.path()).await.expect("Storage operation failed");

        // Write initial file
        backend.write_file("test.txt", b"Original").await.expect("Storage operation failed");

        // Create snapshot
        let snapshot_id = backend.create_snapshot("backup").await.expect("Storage operation failed");
        assert!(!snapshot_id.is_empty());

        // List snapshots
        let snapshots = backend.list_snapshots().await.expect("Storage operation failed");
        assert_eq!(snapshots.len(), 1);
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let temp_dir = TempDir::new().expect("Storage operation failed");

        let backend = FilesystemBackendBuilder::new(temp_dir.path().to_path_buf())
            .with_compression(CompressionAlgorithm::Zstd(3))
            .with_checksum(ChecksumAlgorithm::Sha256)
            .verify_on_read(false)
            .build()
            .await
            .expect("Storage operation failed");

        assert!(matches!(
            backend.config().compression,
            CompressionAlgorithm::Zstd(3)
        ));
        assert_eq!(backend.config().checksum, ChecksumAlgorithm::Sha256);
        assert!(!backend.config().verify_on_read);
    }
}
