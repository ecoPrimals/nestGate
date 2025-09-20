//
// This module contains all policy-related structures and configurations
// including tier assignment rules, lifecycle management policies,
// migration rules, and performance thresholds.

use nestgate_core::canonical_types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tier assignment rules for automatic dataset placement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierAssignmentRules {
    /// Auto-assign new datasets based on predicted usage
    pub auto_assign_new: bool,
    /// File size thresholds for tier assignment
    pub size_thresholds: TierSizeThresholds,
    /// Access pattern based assignment
    pub access_pattern_rules: AccessPatternRules,
    /// Performance requirements
    pub performance_requirements: HashMap<StorageTier, PerformanceRequirement>,
}
/// Size-based tier thresholds for automatic assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierSizeThresholds {
    /// Files smaller than this go to hot tier (bytes)
    pub hot_max_size: u64,
    /// Files smaller than this go to warm tier (bytes)
    pub warm_max_size: u64,
}
/// Access pattern rules for tier assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPatternRules {
    /// Daily access count threshold for hot tier
    pub hot_access_threshold: u32,
    /// Daily access count threshold for warm tier
    pub warm_access_threshold: u32,
    /// Age in days before moving to cold tier
    pub cold_age_threshold: u32,
}
/// Performance requirements per tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirement {
    pub max_latency_ms: f64,
    pub min_throughput_mbps: f64,
    pub min_iops: u32,
}
/// Lifecycle management rules with comprehensive automation policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleRules {
    /// Automatic cleanup of old files
    pub enable_cleanup: bool,
    /// Age threshold for cleanup (days)
    pub cleanup_age_days: u32,
    /// Automatic compression based on age
    pub enable_auto_compression: bool,
    /// Age threshold for compression (days)
    pub compression_age_days: u32,
    /// Automatic archival to cold storage
    pub enable_auto_archival: bool,
    /// Age threshold for archival (days)
    pub archival_age_days: u32,
}
/// Migration automation rules with scheduling and performance controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRules {
    /// Enable automatic migration based on access patterns
    pub enable_auto_migration: bool,
    /// Migration schedule (background processing)
    pub migration_schedule: MigrationSchedule,
    /// Performance impact limits
    pub performance_limits: MigrationPerformanceLimits,
    /// Bandwidth limits for migrations
    pub bandwidth_limits: super::types::BandwidthLimits,
    /// Age threshold in days (for compatibility with tiers.rs)
    pub age_threshold_days: u32,
    /// Access frequency threshold (for compatibility with tiers.rs)
    pub access_frequency_threshold: f64,
    /// Auto migration enabled flag (alias for enable_auto_migration)
    pub auto_migration_enabled: bool,
}
/// Migration scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationSchedule {
    /// Hours during which migration is allowed
    pub allowed_hours: Vec<u8>,
    /// Maximum concurrent migrations
    pub max_concurrent: u32,
    /// Priority boost during off-peak hours
    pub off_peak_priority_boost: bool,
}
/// Performance impact limits for migrations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationPerformanceLimits {
    /// Maximum CPU usage during migration (%)
    pub max_cpu_usage: f64,
    /// Maximum memory usage during migration (%)
    pub max_memory_usage: f64,
    /// Maximum IO impact (%)
    pub max_io_impact: f64,
}
/// Performance thresholds for optimization triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Latency threshold that triggers optimization (ms)
    pub max_latency_ms: f64,
    /// Minimum throughput before optimization (MB/s)
    pub min_throughput_mbps: f64,
    /// Error rate threshold (%)
    pub max_error_rate: f64,
    /// Utilization threshold for tier rebalancing (%)
    pub max_utilization: f64,
}
impl MigrationRules {
    /// Hot tier defaults
    pub const fn hot_tier_defaults() -> Self {
        Self {
            enable_auto_migration: true,
            migration_schedule: MigrationSchedule::default(),
            performance_limits: MigrationPerformanceLimits::default(),
            bandwidth_limits: super::types::BandwidthLimits::default(),
            age_threshold_days: 7,
            access_frequency_threshold: 100.0,
            auto_migration_enabled: true,
        }
    }

    /// Warm tier defaults
    pub const fn warm_tier_defaults() -> Self {
        Self {
            enable_auto_migration: true,
            migration_schedule: MigrationSchedule::default(),
            performance_limits: MigrationPerformanceLimits::default(),
            bandwidth_limits: super::types::BandwidthLimits::default(),
            age_threshold_days: 30,
            access_frequency_threshold: 20.0,
            auto_migration_enabled: true,
        }
    }

    /// Cold tier defaults
    pub const fn cold_tier_defaults() -> Self {
        Self {
            enable_auto_migration: true,
            migration_schedule: MigrationSchedule::default(),
            performance_limits: MigrationPerformanceLimits::default(),
            bandwidth_limits: super::types::BandwidthLimits::default(),
            age_threshold_days: 90,
            access_frequency_threshold: 1.0,
            auto_migration_enabled: true,
        }
    }
}
