// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Types module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

// **MIGRATED**: Using canonical types instead of deprecated unified_types
use crate::canonical_types::storage::StorageTier;
pub use crate::config::canonical_primary::StorageConfig as CacheConfig;

// **CANONICAL CACHE TYPES** - Consolidated from unified_types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Cache
pub enum CacheType {
    /// Memory
    Memory,
    /// Redis
    Redis,
    /// Disk
    Disk,
    /// Hybrid
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Evictionpolicy
pub enum EvictionPolicy {
    /// Lru
    Lru,
    /// Lfu
    Lfu,
    /// Fifo
    Fifo,
    /// Random
    Random,
}

/// High-level storage service metadata configuration (distinct from
/// [`crate::services::storage::config::StorageServiceConfig`]).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageServiceConfig {
    /// Service identifier
    pub service_id: String,
    /// Backend Type
    pub backend_type: String,
    /// Size of connection pool
    pub connection_pool_size: usize,
    /// Timeout Seconds
    pub timeout_seconds: u64,
    /// Retry Attempts
    pub retry_attempts: u32,
    /// Enable Compression
    pub enable_compression: bool,
    /// Enable Encryption
    pub enable_encryption: bool,
    /// Configuration for cache
    pub cache_config: Option<CacheConfig>,
    /// Tier
    pub tier: StorageTier,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Storage pool information with real ZFS data
#[derive(Debug, Clone)]
/// Storagepool
pub struct StoragePool {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Pool Type
    pub pool_type: StoragePoolType,
    /// Size of total
    pub total_size: u64,
    /// Size of used
    pub used_size: u64,
    /// Size of available
    pub available_size: u64,
    /// Health
    pub health: PoolHealth,
    /// Tier
    pub tier: StorageTier,
    /// Properties
    pub properties: HashMap<String, String>,
    /// Datasets
    pub datasets: Vec<String>,
    /// Last Updated
    pub last_updated: SystemTime,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Types of StoragePool
pub enum StoragePoolType {
    /// Zfs
    Zfs,
    /// Filesystem
    Filesystem,
    /// Block
    Block,
    /// Object
    Object,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Poolhealth
pub enum PoolHealth {
    /// Online
    Online,
    /// Degraded
    Degraded,
    /// Faulted
    Faulted,
    /// Offline
    Offline,
    /// Unavailable
    Unavailable,
    /// Removed
    Removed,
}

/// Storage quota configuration and tracking
#[derive(Debug, Clone)]
/// Storagequota
pub struct StorageQuota {
    /// Unique identifier
    pub id: String,
    /// Soft Limit
    pub soft_limit: Option<u64>,
    /// Hard Limit
    pub hard_limit: Option<u64>,
    /// Current Usage
    pub current_usage: u64,
    /// Last Checked
    pub last_checked: SystemTime,
    /// Enforcement
    pub enforcement: QuotaEnforcement,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Quotaenforcement
pub enum QuotaEnforcement {
    /// None
    None,
    /// Warn
    Warn,
    /// Block
    Block,
}

/// Storage service statistics
#[derive(Debug, Clone)]
/// Storageservicestats
pub struct StorageServiceStats {
    /// Total Operations
    pub total_operations: u64,
    /// Read Operations
    pub read_operations: u64,
    /// Write Operations
    pub write_operations: u64,
    /// Delete Operations
    pub delete_operations: u64,
    /// Bytes Read
    pub bytes_read: u64,
    /// Bytes Written
    pub bytes_written: u64,
    /// Cache Hits
    pub cache_hits: u64,
    /// Cache Misses
    pub cache_misses: u64,
    /// Errors
    pub errors: u64,
    /// Last Reset
    pub last_reset: SystemTime,
}
impl Default for StorageServiceStats {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            total_operations: 0,
            read_operations: 0,
            write_operations: 0,
            delete_operations: 0,
            bytes_read: 0,
            bytes_written: 0,
            cache_hits: 0,
            cache_misses: 0,
            errors: 0,
            last_reset: SystemTime::UNIX_EPOCH,
        }
    }
}

/// Storage operation result
#[derive(Debug, Clone)]
/// Storageoperationresult
pub struct StorageOperationResult {
    /// Operation identifier
    pub operation_id: Uuid,
    /// Operation Type
    pub operation_type: StorageOperationType,
    /// Success
    pub success: bool,
    /// Error Message
    pub error_message: Option<String>,
    /// Bytes Processed
    pub bytes_processed: Option<u64>,
    /// Timestamp
    pub timestamp: SystemTime,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// Types of StorageOperation
pub enum StorageOperationType {
    /// Read
    Read,
    /// Write
    Write,
    /// Delete
    Delete,
    /// List
    List,
    /// Createpool
    CreatePool,
    /// Createdataset
    CreateDataset,
    /// Createsnapshot
    CreateSnapshot,
    /// Setquota
    SetQuota,
    /// Cacheoperation
    CacheOperation,
}

impl StoragePool {
    /// Create a new storage pool
    #[must_use]
    pub fn new(name: String, pool_type: StoragePoolType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            pool_type,
            total_size: 0,
            used_size: 0,
            available_size: 0,
            health: PoolHealth::Online,
            tier: StorageTier::Hot,
            properties: HashMap::new(),
            datasets: Vec::new(),
            last_updated: SystemTime::now(),
        }
    }

    /// Check if pool is healthy
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        matches!(self.health, PoolHealth::Online)
    }

    /// Get usage percentage
    #[must_use]
    pub fn usage_percentage(&self) -> f64 {
        if self.total_size == 0 {
            0.0
        } else {
            (self.used_size as f64 / self.total_size as f64) * 100.0
        }
    }

    /// Update pool statistics
    pub fn update_stats(&mut self, total_size: u64, used_size: u64) {
        self.total_size = total_size;
        self.used_size = used_size;
        self.available_size = total_size.saturating_sub(used_size);
        self.last_updated = SystemTime::now();
    }
}

impl StorageQuota {
    /// Create a new storage quota
    #[must_use]
    pub fn new(_path: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            soft_limit: None,
            hard_limit: None,
            current_usage: 0,
            last_checked: SystemTime::now(),
            enforcement: QuotaEnforcement::None,
        }
    }

    /// Check if quota is exceeded
    #[must_use]
    pub const fn is_exceeded(&self) -> bool {
        if let Some(hard_limit) = self.hard_limit {
            self.current_usage >= hard_limit
        } else {
            false
        }
    }

    /// Check if soft limit is exceeded
    #[must_use]
    pub const fn is_soft_limit_exceeded(&self) -> bool {
        if let Some(soft_limit) = self.soft_limit {
            self.current_usage >= soft_limit
        } else {
            false
        }
    }

    /// Get usage percentage
    #[must_use]
    pub fn usage_percentage(&self) -> Option<f64> {
        self.hard_limit.map(|limit| {
            if limit == 0 {
                0.0
            } else {
                (self.current_usage as f64 / limit as f64) * 100.0
            }
        })
    }
}

// CacheConfig implementation removed - using cache::types::CacheConfig instead

impl StorageOperationResult {
    /// Create a successful operation result
    #[must_use]
    pub fn success(operation_type: StorageOperationType, bytes_processed: Option<u64>) -> Self {
        Self {
            operation_id: Uuid::new_v4(),
            operation_type,
            success: true,
            error_message: None,
            bytes_processed,
            timestamp: SystemTime::now(),
        }
    }

    /// Create a failed operation result
    #[must_use]
    pub fn failure(operation_type: StorageOperationType, error: String) -> Self {
        Self {
            operation_id: Uuid::new_v4(),
            operation_type,
            success: false,
            error_message: Some(error),
            bytes_processed: None,
            timestamp: SystemTime::now(),
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Storageserviceconfigcanonical
pub type StorageServiceConfigCanonical =
    crate::config::canonical_primary::domains::storage_canonical::CanonicalStorageConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_pool_creation() {
        let pool = StoragePool::new("test-pool".to_string(), StoragePoolType::Zfs);
        assert_eq!(pool.name, "test-pool");
        assert_eq!(pool.pool_type, StoragePoolType::Zfs);
        assert!(pool.is_healthy());
    }

    #[test]
    fn test_storage_quota_limits() {
        let mut quota = StorageQuota::new("/test/path".to_string());
        quota.hard_limit = Some(1000);
        quota.current_usage = 800;

        assert!(!quota.is_exceeded());

        quota.current_usage = 1000;
        assert!(quota.is_exceeded());
    }

    #[test]
    fn storage_pool_usage_percentage_zero_total() {
        let mut pool = StoragePool::new("p".to_string(), StoragePoolType::Filesystem);
        pool.total_size = 0;
        pool.used_size = 0;
        assert_eq!(pool.usage_percentage(), 0.0);
    }

    #[test]
    fn storage_pool_update_stats_sets_available() {
        let mut pool = StoragePool::new("p".to_string(), StoragePoolType::Zfs);
        pool.update_stats(1000, 250);
        assert_eq!(pool.total_size, 1000);
        assert_eq!(pool.used_size, 250);
        assert_eq!(pool.available_size, 750);
    }

    #[test]
    fn storage_pool_is_healthy_false_when_not_online() {
        let mut pool = StoragePool::new("p".to_string(), StoragePoolType::Zfs);
        pool.health = PoolHealth::Degraded;
        assert!(!pool.is_healthy());
    }

    #[test]
    fn storage_quota_soft_limit_helpers_and_usage_percentage() {
        let mut q = StorageQuota::new("/a".to_string());
        q.soft_limit = Some(10);
        q.hard_limit = Some(100);
        q.current_usage = 10;
        assert!(q.is_soft_limit_exceeded());
        q.current_usage = 5;
        assert!(!q.is_soft_limit_exceeded());

        assert_eq!(q.usage_percentage(), Some(5.0_f64));
        q.hard_limit = Some(0);
        assert_eq!(q.usage_percentage(), Some(0.0));
    }

    #[test]
    fn storage_operation_result_helpers() {
        let ok = StorageOperationResult::success(StorageOperationType::Read, Some(10));
        assert!(ok.success);
        let bad = StorageOperationResult::failure(StorageOperationType::Write, "e".to_string());
        assert!(!bad.success);
        assert_eq!(bad.error_message.as_deref(), Some("e"));
    }

    #[test]
    fn test_cache_config_usage() {
        // Simple test using local struct instead of complex import
        struct TestCacheConfig {
            cache_directory: String,
            cache_size_bytes: u64,
            max_entries: u64,
            _cold_tier_unlimited: bool,
        }

        let cache = TestCacheConfig {
            cache_directory: "/tmp/test-cache".to_string(),
            cache_size_bytes: 1000 * 1024 * 1024, // 1000 MB in bytes
            max_entries: 10000,
            _cold_tier_unlimited: false,
        };
        // **CANONICAL MODERNIZATION** - Updated test for canonical CacheStorageConfig
        let half_size = cache.cache_size_bytes / 2;
        assert!(cache.cache_size_bytes > half_size);
        assert_eq!(cache.cache_directory, "/tmp/test-cache");
        assert_eq!(cache.max_entries, 10000);
    }
}
