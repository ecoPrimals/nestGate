//
// Configuration structures for ZFS tier migration operations.

use serde::{Deserialize, Serialize};

/// Capacity limits for migration operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityLimits {
    /// Maximum number of concurrent migrations
    pub max_concurrent_migrations: u32,
    /// Maximum bandwidth for migration operations (bytes/sec)
    pub max_bandwidth_bytes_per_sec: u64,
}

impl Default for CapacityLimits {
    fn default() -> Self {
        Self {
            max_concurrent_migrations: 4,
            max_bandwidth_bytes_per_sec: 100 * 1024 * 1024, // 100MB/s
        }
    }
}

impl CapacityLimits {
    /// Hot tier capacity limits (high performance)
    pub fn hot_tier_defaults() -> Self {
        Self {
            max_concurrent_migrations: 8,
            max_bandwidth_bytes_per_sec: 500 * 1024 * 1024, // 500MB/s
        }
    }
    
    /// Warm tier capacity limits (balanced)
    pub fn warm_tier_defaults() -> Self {
        Self {
            max_concurrent_migrations: 4,
            max_bandwidth_bytes_per_sec: 200 * 1024 * 1024, // 200MB/s
        }
    }
    
    /// Cold tier capacity limits (storage optimized)
    pub fn cold_tier_defaults() -> Self {
        Self {
            max_concurrent_migrations: 2,
            max_bandwidth_bytes_per_sec: 50 * 1024 * 1024, // 50MB/s
        }
    }
}

/// Migration rules and policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRules {
    /// Enable automatic migration based on access patterns
    pub auto_migration_enabled: bool,
    /// Minimum age before considering migration (hours)
    pub min_age_hours: u32,
    /// Access threshold for tier changes
    pub access_threshold: u32,
}

impl Default for MigrationRules {
    fn default() -> Self {
        Self {
            auto_migration_enabled: true,
            min_age_hours: 24,
            access_threshold: 10,
        }
    }
}

impl MigrationRules {
    /// Hot tier migration rules (frequent access)
    pub fn hot_tier_defaults() -> Self {
        Self {
            auto_migration_enabled: true,
            min_age_hours: 1,
            access_threshold: 100,
        }
    }
    
    /// Warm tier migration rules (moderate access)
    pub fn warm_tier_defaults() -> Self {
        Self {
            auto_migration_enabled: true,
            min_age_hours: 24,
            access_threshold: 10,
        }
    }
    
    /// Cold tier migration rules (infrequent access)
    pub fn cold_tier_defaults() -> Self {
        Self {
            auto_migration_enabled: true,
            min_age_hours: 168, // 1 week
            access_threshold: 1,
        }
    }
}

/// Complete migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Migration rules and policies
    pub rules: MigrationRules,
    /// Capacity limits for operations
    pub limits: CapacityLimits,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            rules: MigrationRules::default(),
            limits: CapacityLimits::default(),
        }
    }
} 