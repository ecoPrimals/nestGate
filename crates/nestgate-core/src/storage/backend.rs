// NestGate Storage Backend - Content-Addressed File Storage
//
// Implements efficient content-addressed storage with:
// - Blake3 content hashing
// - Sharded directory structure (for filesystem performance)
// - Metadata tracking
// - Async I/O with tokio
// - Optional ZFS integration

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use super::ContentHash;

/// Storage backend for NestGate
#[derive(Clone)]
pub struct StorageBackend {
    base_path: PathBuf,
    shard_depth: usize,  // How deep to shard (e.g., 2 = aa/bb/aabbcc...)
}

impl StorageBackend {
    /// Create new storage backend
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
            shard_depth: 2,  // Default: 2-level sharding
        }
    }
    
    /// Initialize backend (create directories)
    pub async fn initialize(&self) -> Result<()> {
        fs::create_dir_all(&self.base_path)
            .await
            .context("Failed to create storage directory")?;
        
        // Create metadata directory
        let metadata_dir = self.base_path.join("metadata");
        fs::create_dir_all(&metadata_dir)
            .await
            .context("Failed to create metadata directory")?;
        
        Ok(())
    }
    
    /// Write data to storage
    pub async fn write(&self, hash: &ContentHash, data: &[u8]) -> Result<()> {
        let data_path = self.hash_to_data_path(hash);
        
        // Create parent directory if needed
        if let Some(parent) = data_path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create shard directory")?;
        }
        
        // Write data atomically (write to temp, then rename)
        let temp_path = data_path.with_extension("tmp");
        fs::write(&temp_path, data)
            .await
            .context("Failed to write data")?;
        
        fs::rename(&temp_path, &data_path)
            .await
            .context("Failed to commit data")?;
        
        Ok(())
    }
    
    /// Read data from storage
    pub async fn read(&self, hash: &ContentHash) -> Result<Vec<u8>> {
        let data_path = self.hash_to_data_path(hash);
        
        fs::read(&data_path)
            .await
            .context("Failed to read data")
    }
    
    /// Check if data exists
    pub async fn exists(&self, hash: &ContentHash) -> Result<bool> {
        let data_path = self.hash_to_data_path(hash);
        Ok(data_path.exists())
    }
    
    /// Delete data
    pub async fn delete(&self, hash: &ContentHash) -> Result<()> {
        let data_path = self.hash_to_data_path(hash);
        let metadata_path = self.hash_to_metadata_path(hash);
        
        // Delete data file
        if data_path.exists() {
            fs::remove_file(&data_path)
                .await
                .context("Failed to delete data")?;
        }
        
        // Delete metadata file
        if metadata_path.exists() {
            fs::remove_file(&metadata_path)
                .await
                .context("Failed to delete metadata")?;
        }
        
        Ok(())
    }
    
    /// Write metadata
    pub async fn write_metadata(&self, hash: &ContentHash, metadata: &StorageMetadata) -> Result<()> {
        let metadata_path = self.hash_to_metadata_path(hash);
        
        // Create parent directory if needed
        if let Some(parent) = metadata_path.parent() {
            fs::create_dir_all(parent)
                .await
                .context("Failed to create metadata directory")?;
        }
        
        let json = serde_json::to_string_pretty(metadata)
            .context("Failed to serialize metadata")?;
        
        fs::write(&metadata_path, json)
            .await
            .context("Failed to write metadata")?;
        
        Ok(())
    }
    
    /// Read metadata
    pub async fn get_metadata(&self, hash: &ContentHash) -> Result<StorageMetadata> {
        let metadata_path = self.hash_to_metadata_path(hash);
        
        let json = fs::read_to_string(&metadata_path)
            .await
            .context("Failed to read metadata")?;
        
        let metadata = serde_json::from_str(&json)
            .context("Failed to parse metadata")?;
        
        Ok(metadata)
    }
    
    /// Convert hash to data file path
    fn hash_to_data_path(&self, hash: &ContentHash) -> PathBuf {
        let hex = hex::encode(hash);
        
        let mut path = self.base_path.join("data");
        
        // Add sharding directories
        let mut offset = 0;
        for _ in 0..self.shard_depth {
            if offset + 2 <= hex.len() {
                path = path.join(&hex[offset..offset + 2]);
                offset += 2;
            }
        }
        
        // Add full hash as filename
        path.join(&hex)
    }
    
    /// Convert hash to metadata file path
    fn hash_to_metadata_path(&self, hash: &ContentHash) -> PathBuf {
        let hex = hex::encode(hash);
        
        let mut path = self.base_path.join("metadata");
        
        // Add sharding directories (same as data)
        let mut offset = 0;
        for _ in 0..self.shard_depth {
            if offset + 2 <= hex.len() {
                path = path.join(&hex[offset..offset + 2]);
                offset += 2;
            }
        }
        
        // Add full hash with .json extension
        path.join(format!("{}.json", hex))
    }
}

/// Metadata stored alongside data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    /// Original size (before compression/encryption)
    pub original_size: usize,
    
    /// Stored size (after compression/encryption)
    pub stored_size: usize,
    
    /// Is data encrypted?
    pub encrypted: bool,
    
    /// Encryption key ID (if encrypted)
    pub encryption_key_id: Option<String>,
    
    /// Compression algorithm used
    pub compression: Option<CompressionMetadata>,
    
    /// Data entropy (0.0-8.0)
    pub entropy: f64,
    
    /// Detected format
    pub format: Option<String>,
    
    /// When data was stored
    pub stored_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMetadata {
    pub algorithm: String,  // "zstd", "lz4", "snappy"
    pub level: Option<i32>,  // Compression level (if applicable)
    pub ratio: f64,          // Compression ratio achieved
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_backend_write_read() {
        let temp_dir = TempDir::new().unwrap();
        let backend = StorageBackend::new(temp_dir.path().to_path_buf());
        backend.initialize().await.unwrap();
        
        // Create test data
        let data = b"Hello, NestGate!";
        let hash = blake3::hash(data);
        let hash_array: [u8; 32] = hash.as_bytes().try_into().unwrap();
        
        // Write
        backend.write(&hash_array, data).await.unwrap();
        
        // Read
        let read_data = backend.read(&hash_array).await.unwrap();
        assert_eq!(read_data, data);
    }
    
    #[tokio::test]
    async fn test_backend_exists() {
        let temp_dir = TempDir::new().unwrap();
        let backend = StorageBackend::new(temp_dir.path().to_path_buf());
        backend.initialize().await.unwrap();
        
        let data = b"Test data";
        let hash = blake3::hash(data);
        let hash_array: [u8; 32] = hash.as_bytes().try_into().unwrap();
        
        // Should not exist yet
        assert!(!backend.exists(&hash_array).await.unwrap());
        
        // Write
        backend.write(&hash_array, data).await.unwrap();
        
        // Should exist now
        assert!(backend.exists(&hash_array).await.unwrap());
    }
    
    #[tokio::test]
    async fn test_backend_metadata() {
        let temp_dir = TempDir::new().unwrap();
        let backend = StorageBackend::new(temp_dir.path().to_path_buf());
        backend.initialize().await.unwrap();
        
        let data = b"Test data";
        let hash = blake3::hash(data);
        let hash_array: [u8; 32] = hash.as_bytes().try_into().unwrap();
        
        // Create metadata
        let metadata = StorageMetadata {
            original_size: 1024,
            stored_size: 256,
            encrypted: false,
            encryption_key_id: None,
            compression: Some(CompressionMetadata {
                algorithm: "zstd".to_string(),
                level: Some(6),
                ratio: 4.0,
            }),
            entropy: 2.5,
            format: Some("text/plain".to_string()),
            stored_at: chrono::Utc::now(),
        };
        
        // Write metadata
        backend.write_metadata(&hash_array, &metadata).await.unwrap();
        
        // Read metadata
        let read_metadata = backend.get_metadata(&hash_array).await.unwrap();
        assert_eq!(read_metadata.original_size, 1024);
        assert_eq!(read_metadata.stored_size, 256);
        assert_eq!(read_metadata.compression.as_ref().unwrap().ratio, 4.0);
    }
    
    #[tokio::test]
    async fn test_hash_sharding() {
        let temp_dir = TempDir::new().unwrap();
        let backend = StorageBackend::new(temp_dir.path().to_path_buf());
        
        // Create a known hash: "aabbccddee..."
        let hash: ContentHash = [
            0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11,
            0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
            0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x11,
            0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99,
        ];
        
        let path = backend.hash_to_data_path(&hash);
        let path_str = path.to_string_lossy();
        
        // Should be sharded: .../data/aa/bb/aabbccddee...
        assert!(path_str.contains("/aa/"));
        assert!(path_str.contains("/bb/"));
        assert!(path_str.ends_with("aabbccddee"));
    }
}

