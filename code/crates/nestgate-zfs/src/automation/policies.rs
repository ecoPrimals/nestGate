// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module contains all policy-related structures and configurations
// including tier assignment rules, lifecycle management policies,
// migration rules, and performance thresholds.

use nestgate_core::canonical_types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tier assignment rules for automatic dataset placement
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierassignmentrules
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
/// Tiersizethresholds
pub struct TierSizeThresholds {
    /// Files smaller than this go to hot tier (bytes)
    pub hot_max_size: u64,
    /// Files smaller than this go to warm tier (bytes)
    pub warm_max_size: u64,
}
/// Access pattern rules for tier assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Accesspatternrules
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
/// Performancerequirement
pub struct PerformanceRequirement {
    /// Max Latency Ms
    pub max_latency_ms: f64,
    /// Min Throughput Mbps
    pub min_throughput_mbps: f64,
    /// Min Iops
    pub min_iops: u32,
}
/// Lifecycle management rules with comprehensive automation policies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Lifecyclerules
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
/// Migrationrules
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
    /// Auto migration enabled flag (alias for `enable_auto_migration`)
    pub auto_migration_enabled: bool,
}
/// Migration scheduling configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Migrationschedule
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
/// Migrationperformancelimits
pub struct MigrationPerformanceLimits {
    /// Maximum CPU usage during migration (%)
    pub maxcpu_usage: f64,
    /// Maximum memory usage during migration (%)
    pub max_memory_usage: f64,
    /// Maximum IO impact (%)
    pub max_io_impact: f64,
}
/// Performance thresholds for optimization triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancethresholds
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
    #[must_use]
    pub fn hot_tier_defaults() -> Self {
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
    #[must_use]
    pub fn warm_tier_defaults() -> Self {
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
    #[must_use]
    pub fn cold_tier_defaults() -> Self {
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

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_rules_hot_tier_defaults() {
        let rules = MigrationRules::hot_tier_defaults();
        assert!(rules.enable_auto_migration);
        assert_eq!(rules.age_threshold_days, 7);
        assert_eq!(rules.access_frequency_threshold, 100.0);
        assert!(rules.auto_migration_enabled);
    }

    #[test]
    fn test_migration_rules_warm_tier_defaults() {
        let rules = MigrationRules::warm_tier_defaults();
        assert!(rules.enable_auto_migration);
        assert_eq!(rules.age_threshold_days, 30);
        assert_eq!(rules.access_frequency_threshold, 20.0);
    }

    #[test]
    fn test_migration_rules_cold_tier_defaults() {
        let rules = MigrationRules::cold_tier_defaults();
        assert!(rules.enable_auto_migration);
        assert_eq!(rules.age_threshold_days, 90);
        assert_eq!(rules.access_frequency_threshold, 1.0);
    }

    #[test]
    fn test_migration_schedule_default() {
        let schedule = MigrationSchedule::default();
        assert_eq!(schedule.allowed_hours.len(), 0);
        assert_eq!(schedule.max_concurrent, 0);
        assert!(!schedule.off_peak_priority_boost);
    }

    #[test]
    fn test_migration_performance_limits_default() {
        let limits = MigrationPerformanceLimits::default();
        assert_eq!(limits.maxcpu_usage, 0.0);
        assert_eq!(limits.max_memory_usage, 0.0);
        assert_eq!(limits.max_io_impact, 0.0);
    }

    #[test]
    fn test_tier_size_thresholds_creation() {
        let thresholds = TierSizeThresholds {
            hot_max_size: 1024 * 1024 * 100,   // 100MB
            warm_max_size: 1024 * 1024 * 1024, // 1GB
        };
        assert_eq!(thresholds.hot_max_size, 104857600);
        assert_eq!(thresholds.warm_max_size, 1073741824);
    }

    #[test]
    fn test_access_pattern_rules_creation() {
        let rules = AccessPatternRules {
            hot_access_threshold: 100,
            warm_access_threshold: 20,
            cold_age_threshold: 90,
        };
        assert_eq!(rules.hot_access_threshold, 100);
        assert_eq!(rules.warm_access_threshold, 20);
        assert_eq!(rules.cold_age_threshold, 90);
    }

    #[test]
    fn test_performance_requirement_creation() {
        let req = PerformanceRequirement {
            max_latency_ms: 10.0,
            min_throughput_mbps: 100.0,
            min_iops: 1000,
        };
        assert_eq!(req.max_latency_ms, 10.0);
        assert_eq!(req.min_throughput_mbps, 100.0);
        assert_eq!(req.min_iops, 1000);
    }

    #[test]
    fn test_lifecycle_rules_creation() {
        let rules = LifecycleRules {
            enable_cleanup: true,
            cleanup_age_days: 365,
            enable_auto_compression: true,
            compression_age_days: 30,
            enable_auto_archival: true,
            archival_age_days: 90,
        };
        assert!(rules.enable_cleanup);
        assert_eq!(rules.cleanup_age_days, 365);
        assert!(rules.enable_auto_compression);
    }

    #[test]
    fn test_performance_thresholds_creation() {
        let thresholds = PerformanceThresholds {
            max_latency_ms: 100.0,
            min_throughput_mbps: 50.0,
            max_error_rate: 0.01,
            max_utilization: 80.0,
        };
        assert_eq!(thresholds.max_latency_ms, 100.0);
        assert_eq!(thresholds.max_error_rate, 0.01);
    }

    #[test]
    fn test_migration_schedule_clone() {
        let schedule1 = MigrationSchedule {
            allowed_hours: vec![1, 2, 3],
            max_concurrent: 5,
            off_peak_priority_boost: true,
        };
        let schedule2 = schedule1.clone();
        assert_eq!(schedule1.allowed_hours, schedule2.allowed_hours);
    }

    #[test]
    fn test_tier_assignment_rules_serialization() {
        let rules = TierAssignmentRules {
            auto_assign_new: true,
            size_thresholds: TierSizeThresholds {
                hot_max_size: 100,
                warm_max_size: 1000,
            },
            access_pattern_rules: AccessPatternRules {
                hot_access_threshold: 50,
                warm_access_threshold: 10,
                cold_age_threshold: 90,
            },
            performance_requirements: HashMap::new(),
        };
        let serialized = serde_json::to_string(&rules).unwrap();
        assert!(serialized.contains("auto_assign_new"));
    }
}
