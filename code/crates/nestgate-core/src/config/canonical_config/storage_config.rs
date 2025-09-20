//
// Storage-related configuration including tiers, compression, encryption,
// backup, replication, and performance tuning.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// Import unified constants
use crate::canonical_modernization::canonical_constants::{
    storage::{GB},
};

/// Storage configuration (consolidates 25+ storage configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage backend type
    pub backend_type: String,
    /// Storage tiers configuration
    pub tiers: StorageTiersConfig,
    /// Compression configuration
    pub compression: CompressionConfig,
    /// Encryption configuration
    pub encryption: StorageEncryptionConfig,
    /// Backup configuration
    pub backup: BackupConfig,
    /// Replication configuration
    pub replication: ReplicationConfig,
    /// Performance tuning
    pub performance: StoragePerformanceConfig,
    /// Cache configuration
    pub cache: CacheStorageConfig,
}
/// Storage tiers configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct StorageTiersConfig {
    /// Hot tier configuration
    pub hot: TierConfig,
    /// Warm tier configuration
    pub warm: TierConfig,
    /// Cold tier configuration
    pub cold: TierConfig,
    /// Archive tier configuration
    pub archive: TierConfig,
}
/// Tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    /// Storage path
    /// Maximum size (bytes)
    pub max_size_bytes: u64,
    /// Compression level (0-9)
    pub compression_level: u8,
    /// Retention policy
    pub retention_days: u32,
    /// Access frequency threshold
    pub access_frequency_threshold: u32,
}
/// Cache storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStorageConfig {
    /// Cache directory path
    pub cache_directory: String,
    /// Cache size in bytes
    pub cache_size_bytes: u64,
    /// Maximum cache entries
    pub max_entries: u64,
    /// Whether cold tier is unlimited
    pub cold_tier_unlimited: bool,
}
/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Compression algorithm
    pub algorithm: String,
    /// Compression level
    pub level: u8,
    /// Enable compression
    pub enabled: bool,
}
/// Storage encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEncryptionConfig {
    /// Enable encryption
    pub enabled: bool,
    /// Encryption algorithm
    pub algorithm: String,
    /// Key derivation function
    pub kdf: String,
}
/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable backups
    pub enabled: bool,
    /// Backup interval
    pub interval: Duration,
    /// Backup retention
    pub retention_days: u32,
}
/// Replication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationConfig {
    /// Enable replication
    pub enabled: bool,
    /// Replication factor
    pub factor: u32,
    /// Replication strategy
    pub strategy: String,
}
/// Storage performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceConfig {
    /// I/O buffer size
    pub io_buffer_size: usize,
    /// Read ahead size
    pub read_ahead_size: usize,
    /// Write behind enabled
    pub write_behind_enabled: bool,
}
impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend_type: "filesystem".to_string(),
            tiers: StorageTiersConfig::default(),
            compression: CompressionConfig::default(),
            encryption: StorageEncryptionConfig::default(),
            backup: BackupConfig::default(),
            replication: ReplicationConfig::default(),
            performance: StoragePerformanceConfig::default(),
            cache: CacheStorageConfig::default(),
        }
    }
}


impl Default for TierConfig {
    fn default() -> Self {
        Self {
            max_size_bytes: 1024 * 1024 * 1024, // 1GB
            compression_level: 6,
            retention_days: 30,
            access_frequency_threshold: 10,
        }
    }
}

impl Default for CacheStorageConfig {
    fn default() -> Self {
        Self {
            cache_directory: "/tmp/nestgate-cache".to_string(),
            cache_size_bytes: GB,
            max_entries: 10000,
            cold_tier_unlimited: false,
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: "lz4".to_string(),
            level: 6,
            enabled: true,
        }
    }
}

impl Default for StorageEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "aes-256-gcm".to_string(),
            kdf: "pbkdf2".to_string(),
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: Duration::from_secs(86400), // Daily
            retention_days: 7,
        }
    }
}

impl Default for ReplicationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            factor: 3,
            strategy: "async".to_string(),
        }
    }
}

impl Default for StoragePerformanceConfig {
    fn default() -> Self {
        Self {
            io_buffer_size: 64 * 1024, // 64KB
            read_ahead_size: 128 * 1024, // 128KB
            write_behind_enabled: true,
        }
    }
} 