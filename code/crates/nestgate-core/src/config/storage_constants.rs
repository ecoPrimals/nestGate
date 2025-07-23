//! Storage Constants Configuration Module
//!
//! Centralized management of storage sizes, limits, and thresholds to eliminate
//! hardcoded values and enable easy tuning for different deployment scenarios.

use serde::{Deserialize, Serialize};

/// Storage size constants with environment variable support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConstants {
    /// File size thresholds
    pub file_sizes: FileSizeThresholds,

    /// Memory usage limits
    pub memory_limits: MemoryLimits,

    /// Cache configurations  
    pub cache_sizes: CacheSizes,

    /// Performance thresholds
    pub performance_thresholds: PerformanceThresholds,

    /// ZFS-specific constants
    pub zfs_constants: ZfsConstants,
}

/// File size thresholds for different operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSizeThresholds {
    /// Small file threshold (1MB by default)
    pub small_file: u64,

    /// Large file threshold (100MB by default)  
    pub large_file: u64,

    /// Maximum file size (100MB by default)
    pub max_file_size: u64,

    /// Archive threshold for tiering (1GB by default)
    pub archive_threshold: u64,

    /// Hot tier size limit for auto-migration
    pub hot_tier_limit: u64,
}

/// Memory usage limits for different components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLimits {
    /// Minimum available memory for operations (100MB)
    pub minimum_available_memory: u64,

    /// Memory pool buffer sizes
    pub buffer_4kb: u32,
    pub buffer_1mb: u32,

    /// Cache size limits
    pub max_cache_size: u64,
    pub default_cache_size: u64,
}

/// Cache size configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSizes {
    /// UUID cache size
    pub uuid_cache_size: u32,

    /// Storage tier cache size (1GB default)
    pub tier_cache_size: u64,

    /// Metadata cache size
    pub metadata_cache_size: u64,

    /// Connection pool size
    pub connection_pool_size: u32,
}

/// Performance threshold constants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// UUID cache operation max time (nanoseconds)
    pub uuid_cache_max_nanos: u64,

    /// Memory pool operation max time (nanoseconds)
    pub memory_pool_max_nanos: u64,

    /// I/O operation timeout (milliseconds)
    pub io_operation_timeout_ms: u64,

    /// Large file processing threshold for optimization
    pub large_file_optimization_threshold: u64,
}

/// ZFS-specific storage constants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConstants {
    /// Default pool capacity (for testing/development)
    pub default_pool_capacity_tb: u64,

    /// Default dataset size limits
    pub default_dataset_limit_gb: u64,

    /// Snapshot retention limits
    pub max_snapshots_per_dataset: u32,

    /// Scrub frequency thresholds
    pub scrub_frequency_days: u32,
}

impl Default for StorageConstants {
    fn default() -> Self {
        Self {
            file_sizes: FileSizeThresholds::default(),
            memory_limits: MemoryLimits::default(),
            cache_sizes: CacheSizes::default(),
            performance_thresholds: PerformanceThresholds::default(),
            zfs_constants: ZfsConstants::default(),
        }
    }
}

impl Default for FileSizeThresholds {
    fn default() -> Self {
        Self {
            small_file: Self::get_small_file_threshold(),
            large_file: Self::get_large_file_threshold(),
            max_file_size: Self::get_max_file_size(),
            archive_threshold: Self::get_archive_threshold(),
            hot_tier_limit: Self::get_hot_tier_limit(),
        }
    }
}

impl FileSizeThresholds {
    /// Get small file threshold from environment or default (1MB)
    pub fn get_small_file_threshold() -> u64 {
        std::env::var("NESTGATE_SMALL_FILE_THRESHOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024 * 1024) // 1MB
    }

    /// Get large file threshold from environment or default (100MB)
    pub fn get_large_file_threshold() -> u64 {
        std::env::var("NESTGATE_LARGE_FILE_THRESHOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100 * 1024 * 1024) // 100MB
    }

    /// Get maximum file size from environment or default (100MB)
    pub fn get_max_file_size() -> u64 {
        std::env::var("NESTGATE_MAX_FILE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100 * 1024 * 1024) // 100MB
    }

    /// Get archive threshold from environment or default (1GB)
    pub fn get_archive_threshold() -> u64 {
        std::env::var("NESTGATE_ARCHIVE_THRESHOLD")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1_000_000_000) // 1GB
    }

    /// Get hot tier limit from environment or default (10GB)
    pub fn get_hot_tier_limit() -> u64 {
        std::env::var("NESTGATE_HOT_TIER_LIMIT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10_000_000_000) // 10GB
    }
}

impl Default for MemoryLimits {
    fn default() -> Self {
        Self {
            minimum_available_memory: Self::get_minimum_available_memory(),
            buffer_4kb: Self::get_buffer_4kb(),
            buffer_1mb: Self::get_buffer_1mb(),
            max_cache_size: Self::get_max_cache_size(),
            default_cache_size: Self::get_default_cache_size(),
        }
    }
}

impl MemoryLimits {
    /// Get minimum available memory from environment or default (100MB)
    pub fn get_minimum_available_memory() -> u64 {
        std::env::var("NESTGATE_MIN_AVAILABLE_MEMORY")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100_000_000) // 100MB
    }

    /// Get 4KB buffer size from environment or default
    pub fn get_buffer_4kb() -> u32 {
        std::env::var("NESTGATE_BUFFER_4KB")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(4 * 1024) // 4KB
    }

    /// Get 1MB buffer size from environment or default
    pub fn get_buffer_1mb() -> u32 {
        std::env::var("NESTGATE_BUFFER_1MB")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024 * 1024) // 1MB
    }

    /// Get maximum cache size from environment or default (5GB)
    pub fn get_max_cache_size() -> u64 {
        std::env::var("NESTGATE_MAX_CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5 * 1024 * 1024 * 1024) // 5GB
    }

    /// Get default cache size from environment or default (1GB)
    pub fn get_default_cache_size() -> u64 {
        std::env::var("NESTGATE_DEFAULT_CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024 * 1024 * 1024) // 1GB
    }
}

impl Default for CacheSizes {
    fn default() -> Self {
        Self {
            uuid_cache_size: Self::get_uuid_cache_size(),
            tier_cache_size: Self::get_tier_cache_size(),
            metadata_cache_size: Self::get_metadata_cache_size(),
            connection_pool_size: Self::get_connection_pool_size(),
        }
    }
}

impl CacheSizes {
    /// Get UUID cache size from environment or default (10000 entries)
    pub fn get_uuid_cache_size() -> u32 {
        std::env::var("NESTGATE_UUID_CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10000)
    }

    /// Get tier cache size from environment or default (1GB)
    pub fn get_tier_cache_size() -> u64 {
        std::env::var("NESTGATE_TIER_CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1024 * 1024 * 1024) // 1GB
    }

    /// Get metadata cache size from environment or default (100MB)
    pub fn get_metadata_cache_size() -> u64 {
        std::env::var("NESTGATE_METADATA_CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100 * 1024 * 1024) // 100MB
    }

    /// Get connection pool size from environment or default (10 connections)
    pub fn get_connection_pool_size() -> u32 {
        std::env::var("NESTGATE_CONNECTION_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10)
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            uuid_cache_max_nanos: 50_000_000,                     // 50ms
            memory_pool_max_nanos: 10_000_000,                    // 10ms
            io_operation_timeout_ms: 30_000,                      // 30s
            large_file_optimization_threshold: 100 * 1024 * 1024, // 100MB
        }
    }
}

impl Default for ZfsConstants {
    fn default() -> Self {
        Self {
            default_pool_capacity_tb: 10,   // 10TB
            default_dataset_limit_gb: 1000, // 1TB
            max_snapshots_per_dataset: 100, // 100 snapshots
            scrub_frequency_days: 30,       // Monthly scrubs
        }
    }
}

impl StorageConstants {
    /// Get storage constants from environment variables
    pub fn from_environment() -> Self {
        Self::default()
    }

    /// Get a file size constant by name
    pub fn get_file_size(&self, size_type: &str) -> Option<u64> {
        match size_type {
            "small_file" => Some(self.file_sizes.small_file),
            "large_file" => Some(self.file_sizes.large_file),
            "max_file" => Some(self.file_sizes.max_file_size),
            "archive_threshold" => Some(self.file_sizes.archive_threshold),
            _ => None,
        }
    }

    /// Get a memory limit by name
    pub fn get_memory_limit(&self, limit_type: &str) -> Option<u64> {
        match limit_type {
            "minimum_memory" => Some(self.memory_limits.minimum_available_memory),
            "max_cache" => Some(self.memory_limits.max_cache_size),
            "default_cache" => Some(self.memory_limits.default_cache_size),
            _ => None,
        }
    }

    /// Validate storage constants
    pub fn validate(&self) -> Result<(), String> {
        // Validate file size relationships
        if self.file_sizes.small_file >= self.file_sizes.large_file {
            return Err("Small file threshold must be less than large file threshold".to_string());
        }

        if self.file_sizes.large_file > self.file_sizes.max_file_size {
            return Err("Large file threshold cannot exceed maximum file size".to_string());
        }

        // Validate memory limits
        if self.memory_limits.default_cache_size > self.memory_limits.max_cache_size {
            return Err("Default cache size cannot exceed maximum cache size".to_string());
        }

        // Validate ZFS constants
        if self.zfs_constants.max_snapshots_per_dataset == 0 {
            return Err("Maximum snapshots per dataset must be greater than 0".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_storage_constants() {
        let constants = StorageConstants::default();

        // Test file size thresholds
        assert_eq!(constants.file_sizes.small_file, 1024 * 1024); // 1MB
        assert_eq!(constants.file_sizes.large_file, 100 * 1024 * 1024); // 100MB
        assert_eq!(constants.file_sizes.archive_threshold, 1_000_000_000); // 1GB

        // Test memory limits
        assert_eq!(
            constants.memory_limits.minimum_available_memory,
            100_000_000
        ); // 100MB
        assert_eq!(
            constants.memory_limits.default_cache_size,
            1024 * 1024 * 1024
        ); // 1GB

        // Validate configuration
        assert!(constants.validate().is_ok());
    }

    #[test]
    fn test_file_size_getters() {
        let constants = StorageConstants::default();

        assert_eq!(constants.get_file_size("small_file"), Some(1024 * 1024));
        assert_eq!(
            constants.get_file_size("large_file"),
            Some(100 * 1024 * 1024)
        );
        assert_eq!(constants.get_file_size("invalid"), None);
    }

    #[test]
    fn test_memory_limit_getters() {
        let constants = StorageConstants::default();

        assert_eq!(
            constants.get_memory_limit("minimum_memory"),
            Some(100_000_000)
        );
        assert_eq!(
            constants.get_memory_limit("default_cache"),
            Some(1024 * 1024 * 1024)
        );
        assert_eq!(constants.get_memory_limit("invalid"), None);
    }

    #[test]
    fn test_validation_errors() {
        let mut constants = StorageConstants::default();

        // Test invalid file size relationship
        constants.file_sizes.small_file = 200 * 1024 * 1024; // Larger than large_file
        assert!(constants.validate().is_err());

        // Reset and test cache size relationship
        constants.file_sizes.small_file = 1024 * 1024;
        constants.memory_limits.default_cache_size = 10 * 1024 * 1024 * 1024; // Larger than max
        assert!(constants.validate().is_err());
    }
}
