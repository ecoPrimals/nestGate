// **STORAGE CONSTANTS**
//! Storage functionality and utilities.
// Consolidated storage and ZFS-related constants for NestGate.

use super::types::*;

/// Default ZFS pool cache size in bytes
pub const DEFAULT_ZFS_CACHE_SIZE: u64 = 1024 * 1024 * 1024; // 1GB
/// Default storage operation timeout in seconds
pub const STORAGE_TIMEOUT_SECS: u64 = 60;
/// Maximum file size for operations in bytes
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024 * 1024; // 100GB
/// Default block size for ZFS operations
pub const DEFAULT_BLOCK_SIZE: usize = 131_072; // 128KB
/// Default compression level
pub const DEFAULT_COMPRESSION_LEVEL: u8 = 6;
/// Maximum snapshot count per dataset
pub const MAX_SNAPSHOTS_PER_DATASET: u32 = 1000;
/// Default replication batch size
pub const DEFAULT_REPLICATION_BATCH_SIZE: usize = 1000;
/// Storage health check interval in seconds
pub const STORAGE_HEALTH_CHECK_INTERVAL_SECS: u64 = 300; // 5 minutes
/// Default tier migration threshold in bytes
pub const TIER_MIGRATION_THRESHOLD: u64 = 10 * 1024 * 1024 * 1024; // 10GB
/// Storage constants registration helper
pub fn register_storage_constants() -> Vec<(String, ConstantValue, String)> {
    vec![
        (
            "DEFAULT_ZFS_CACHE_SIZE".to_string(),
            ConstantValue::Size(DEFAULT_ZFS_CACHE_SIZE),
            "Default ZFS pool cache size".to_string(),
        ),
        (
            "STORAGE_TIMEOUT_SECS".to_string(),
            ConstantValue::Duration(STORAGE_TIMEOUT_SECS),
            "Default storage operation timeout".to_string(),
        ),
        (
            "MAX_FILE_SIZE".to_string(),
            ConstantValue::Size(MAX_FILE_SIZE),
            "Maximum file size for operations".to_string(),
        ),
        (
            "DEFAULT_BLOCK_SIZE".to_string(),
            ConstantValue::UnsignedInteger(DEFAULT_BLOCK_SIZE as u64),
            "Default block size for ZFS operations".to_string(),
        ),
        (
            "DEFAULT_COMPRESSION_LEVEL".to_string(),
            ConstantValue::UnsignedInteger(DEFAULT_COMPRESSION_LEVEL as u64),
            "Default compression level".to_string(),
        ),
        (
            "MAX_SNAPSHOTS_PER_DATASET".to_string(),
            ConstantValue::UnsignedInteger(MAX_SNAPSHOTS_PER_DATASET as u64),
            "Maximum snapshot count per dataset".to_string(),
        ),
    ]
}
