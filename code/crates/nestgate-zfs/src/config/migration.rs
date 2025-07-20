//! Migration Configuration Module
//!
//! Configuration for data migration between tiers, capacity limits, and migration rules.

use serde::{Deserialize, Serialize};

/// Migration rules between tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRules {
    /// Age threshold for migration (in days)
    pub age_threshold_days: u32,
    /// Access frequency threshold
    pub access_frequency_threshold: f64,
    /// Size threshold for migration
    pub size_threshold_bytes: u64,
    /// Enable automatic migration
    pub auto_migration_enabled: bool,
    /// Migration schedule (cron-like expression)
    pub migration_schedule: Option<String>,
}

/// Capacity limits for tiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityLimits {
    /// Maximum capacity percentage (0.0 - 1.0)
    pub max_utilization: f64,
    /// Warning threshold percentage
    pub warning_threshold: f64,
    /// Reserved space in bytes
    pub reserved_bytes: u64,
}

/// Migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Enable background migration
    pub background_migration: bool,
    /// Maximum concurrent migrations
    pub max_concurrent_migrations: u32,
    /// Migration bandwidth limit (bytes per second)
    pub bandwidth_limit_bps: Option<u64>,
    /// Migration queue size
    pub queue_size: u32,
    /// Retry attempts for failed migrations
    pub retry_attempts: u32,
}

impl MigrationRules {
    /// Default migration rules for hot tier
    pub fn hot_tier_defaults() -> Self {
        Self {
            age_threshold_days: 7,                    // Move to warm after 7 days
            access_frequency_threshold: 10.0,         // 10+ accesses per day to stay hot
            size_threshold_bytes: 1024 * 1024 * 1024, // 1GB threshold
            auto_migration_enabled: true,
            migration_schedule: Some("0 2 * * *".to_string()), // Daily at 2 AM
        }
    }

    /// Default migration rules for warm tier
    pub fn warm_tier_defaults() -> Self {
        Self {
            age_threshold_days: 30,                        // Move to cold after 30 days
            access_frequency_threshold: 2.0,               // 2+ accesses per day to stay warm
            size_threshold_bytes: 10 * 1024 * 1024 * 1024, // 10GB threshold
            auto_migration_enabled: true,
            migration_schedule: Some("0 3 * * 0".to_string()), // Weekly at 3 AM on Sunday
        }
    }

    /// Default migration rules for cold tier
    pub fn cold_tier_defaults() -> Self {
        Self {
            age_threshold_days: 365,                        // Archive after 1 year
            access_frequency_threshold: 0.1,                // Very low access threshold
            size_threshold_bytes: 100 * 1024 * 1024 * 1024, // 100GB threshold
            auto_migration_enabled: false,                  // Manual migration for cold tier
            migration_schedule: None,
        }
    }
}

impl CapacityLimits {
    /// Default capacity limits for hot tier
    pub fn hot_tier_defaults() -> Self {
        Self {
            max_utilization: 0.8,                    // 80% max utilization
            warning_threshold: 0.7,                  // Warning at 70%
            reserved_bytes: 10 * 1024 * 1024 * 1024, // Reserve 10GB
        }
    }

    /// Default capacity limits for warm tier
    pub fn warm_tier_defaults() -> Self {
        Self {
            max_utilization: 0.85,                   // 85% max utilization
            warning_threshold: 0.75,                 // Warning at 75%
            reserved_bytes: 50 * 1024 * 1024 * 1024, // Reserve 50GB
        }
    }

    /// Default capacity limits for cold tier
    pub fn cold_tier_defaults() -> Self {
        Self {
            max_utilization: 0.9,                     // 90% max utilization
            warning_threshold: 0.8,                   // Warning at 80%
            reserved_bytes: 100 * 1024 * 1024 * 1024, // Reserve 100GB
        }
    }
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            background_migration: true,
            max_concurrent_migrations: 2,
            bandwidth_limit_bps: None,
            queue_size: 100,
            retry_attempts: 3,
        }
    }
}
