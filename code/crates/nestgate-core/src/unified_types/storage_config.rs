use std::collections::HashMap;
//
// This module provides consolidated storage configuration types,
// eliminating fragmentation across storage backends.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Unified storage configuration for all backends
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorageConfig {
    pub primary_backend: String,
    pub backup_backends: Vec<String>,
    pub cache_config: StorageCacheConfig,
    pub replication_config: StorageReplicationConfig,
    pub compression_config: StorageCompressionConfig,
    pub encryption_config: StorageEncryptionConfig,
    pub performance_config: StoragePerformanceConfig,
    pub metadata: HashMap<String, String>,
}

impl Default for UnifiedStorageConfig {
    fn default() -> Self {
        Self {
            primary_backend: "filesystem".to_string(),
            backup_backends: vec!["s3".to_string()],
            cache_config: StorageCacheConfig::default(),
            replication_config: StorageReplicationConfig::default(),
            compression_config: StorageCompressionConfig::default(),
            encryption_config: StorageEncryptionConfig::default(),
            performance_config: StoragePerformanceConfig::default(),
            metadata: HashMap::new(),
        }
    }
}

/// Storage cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCacheConfig {
    pub enabled: bool,
    pub max_size_mb: usize,
    pub ttl_secs: u64,
    pub write_through: bool,
}

impl Default for StorageCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 1024,
            ttl_secs: 3600,
            write_through: false,
        }
    }
}

/// Storage replication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageReplicationConfig {
    pub enabled: bool,
    pub replication_factor: usize,
    pub sync_interval: Duration,
    pub conflict_resolution: ConflictResolutionStrategy,
}

impl Default for StorageReplicationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            replication_factor: 3,
            sync_interval: Duration::from_secs(300),
            conflict_resolution: ConflictResolutionStrategy::LastWriteWins,
        }
    }
}

/// Storage compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCompressionConfig {
    pub enabled: bool,
    pub algorithm: CompressionAlgorithm,
    pub level: CompressionLevel,
    pub min_file_size_bytes: usize,
}

impl Default for StorageCompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: CompressionAlgorithm::Lz4,
            level: CompressionLevel::Medium,
            min_file_size_bytes: 1024,
        }
    }
}

/// Storage encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEncryptionConfig {
    pub enabled: bool,
    pub algorithm: EncryptionAlgorithm,
    pub key_rotation_days: u32,
    pub encrypt_metadata: bool,
}

impl Default for StorageEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_rotation_days: 90,
            encrypt_metadata: false,
        }
    }
}

/// Storage performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceConfig {
    pub max_concurrent_operations: usize,
    pub operation_timeout: Duration,
    pub retry_attempts: usize,
    pub batch_size: usize,
}

impl Default for StoragePerformanceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(30),
            retry_attempts: 3,
            batch_size: 1000,
        }
    }
}

/// Conflict resolution strategies for replication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    LastWriteWins,
    FirstWriteWins,
    Manual,
    Merge,
}

/// Compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    None,
    Gzip,
    Lz4,
    Zstd,
    Brotli,
}

/// Compression levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionLevel {
    Low,
    Medium,
    High,
    Maximum,
}

/// Encryption algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    None,
    Aes128Gcm,
    Aes256Gcm,
    ChaCha20Poly1305,
} 