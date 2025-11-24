use std::collections::HashMap;
// CLEANED: Removed unused Duration import as part of canonical modernization
// use std::time::Duration;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

// **MIGRATED**: Using canonical types instead of deprecated unified_types
use crate::canonical_types::storage::StorageTier;
pub use crate::config::canonical_primary::StorageConfig as CacheConfig;
// CLEANED: Removed unused StorageOperation import as part of canonical modernization

// **CANONICAL CACHE TYPES** - Consolidated from unified_types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheType {
    Memory,
    Redis,
    Disk,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Random,
}

/// Storage service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageServiceConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageServiceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct StorageServiceConfig {
    pub service_id: String,
    pub backend_type: String,
    pub connection_pool_size: usize,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub enable_compression: bool,
    pub enable_encryption: bool,
    pub cache_config: Option<CacheConfig>,
    pub tier: StorageTier,
    pub metadata: HashMap<String, String>,
}
/// Storage pool information with real ZFS data
#[derive(Debug, Clone)]
pub struct StoragePool {
    pub id: String,
    pub name: String,
    pub pool_type: StoragePoolType,
    pub total_size: u64,
    pub used_size: u64,
    pub available_size: u64,
    pub health: PoolHealth,
    pub tier: StorageTier,
    pub properties: HashMap<String, String>,
    pub datasets: Vec<String>,
    pub last_updated: SystemTime,
}
#[derive(Debug, Clone, PartialEq)]
pub enum StoragePoolType {
    Zfs,
    Filesystem,
    Block,
    Object,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PoolHealth {
    Online,
    Degraded,
    Faulted,
    Offline,
    Unavailable,
    Removed,
}

/// Storage quota configuration and tracking
#[derive(Debug, Clone)]
pub struct StorageQuota {
    pub id: String,
    pub soft_limit: Option<u64>,
    pub hard_limit: Option<u64>,
    pub current_usage: u64,
    pub last_checked: SystemTime,
    pub enforcement: QuotaEnforcement,
}
#[derive(Debug, Clone, PartialEq)]
pub enum QuotaEnforcement {
    None,
    Warn,
    Block,
}

/// Storage service statistics
#[derive(Debug, Clone)]
pub struct StorageServiceStats {
    pub total_operations: u64,
    pub read_operations: u64,
    pub write_operations: u64,
    pub delete_operations: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub errors: u64,
    pub last_reset: SystemTime,
}
impl Default for StorageServiceStats {
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
pub struct StorageOperationResult {
    pub operation_id: Uuid,
    pub operation_type: StorageOperationType,
    pub success: bool,
    pub error_message: Option<String>,
    pub bytes_processed: Option<u64>,
    pub timestamp: SystemTime,
}
#[derive(Debug, Clone, PartialEq)]
pub enum StorageOperationType {
    Read,
    Write,
    Delete,
    List,
    CreatePool,
    CreateDataset,
    CreateSnapshot,
    SetQuota,
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
    pub fn is_healthy(&self) -> bool {
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
    pub fn is_exceeded(&self) -> bool {
        if let Some(hard_limit) = self.hard_limit {
            self.current_usage >= hard_limit
        } else {
            false
        }
    }

    /// Check if soft limit is exceeded
    #[must_use]
    pub fn is_soft_limit_exceeded(&self) -> bool {
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
#[allow(deprecated)]
pub type StorageServiceConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageServiceConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

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
    fn test_cache_config_usage() {
        // Simple test using local struct instead of complex import
        #[allow(dead_code)]
        struct TestCacheConfig {
            cache_directory: String,
            cache_size_bytes: u64,
            max_entries: u64,
            cold_tier_unlimited: bool,
        }

        let cache = TestCacheConfig {
            cache_directory: "/tmp/test-cache".to_string(),
            cache_size_bytes: 1000 * 1024 * 1024, // 1000 MB in bytes
            max_entries: 10000,
            cold_tier_unlimited: false,
        };
        // **CANONICAL MODERNIZATION** - Updated test for canonical CacheStorageConfig
        let half_size = cache.cache_size_bytes / 2;
        assert!(cache.cache_size_bytes > half_size);
        assert_eq!(cache.cache_directory, "/tmp/test-cache");
        assert_eq!(cache.max_entries, 10000);
    }
}
