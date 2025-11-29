//
// Tests for ZFS configuration settings and defaults

//! Config Tests module

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use nestgate_core::StorageTier as CoreStorageTier;
use nestgate_zfs::performance::TierMetrics;
use nestgate_zfs::performance::{AlertCondition, AlertMetric, AlertOperator, AlertSeverity};
use nestgate_zfs::{
use nestgate_core::canonical_types::StorageTier;
    automation::{DatasetLifecycle, LifecycleRule, LifecycleStage},
    config::ZfsConfig,
    migration::{MigrationJob, MigrationPriority, MigrationStatus},
    snapshot::*,
    types::StorageTier,
};

#[cfg(test)]
mod config_unit_tests {
    use super::*;

    #[test]
    fn test_zfs_config_defaults() -> Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();

        // Test configuration defaults (using actual field names from struct)
        assert!(config.api_endpoint.starts_with("http://localhost:"));
        assert_eq!(config.default_pool, "zfspool");
        assert!(config.use_real_zfs);
        assert_eq!(config.tiers.hot.name, "hot");
        assert_eq!(config.tiers.warm.name, "warm");
        assert_eq!(config.tiers.cold.name, "cold");
        assert!(config.pool_discovery.auto_discovery);
        assert!(config.health_monitoring.enabled);
        assert_eq!(config.health_monitoring.check_interval_seconds, 30);
        assert!(config.metrics.enabled);
        assert_eq!(config.metrics.collection_interval_seconds, 60);
        assert_eq!(config.monitoring_interval, 300);
    Ok(())
    }
    #[test]
    fn test_tier_config_hierarchy() -> Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();

        let hot = config.get_tier_config(&CoreStorageTier::Hot);
        let warm = config.get_tier_config(&CoreStorageTier::Warm);
        let cold = config.get_tier_config(&CoreStorageTier::Cold);

        // Verify compression algorithms
        assert_eq!(hot.properties.get("compression").unwrap_or_else(|_e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {}", "actual_error_details")
).into())
}), "lz4");
        assert_eq!(warm.properties.get("compression").unwrap_or_else(|_e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {}", "actual_error_details")
).into())
}), "zstd");
        assert_eq!(cold.properties.get("compression").unwrap_or_else(|_e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {}", "actual_error_details")
).into())
}), "gzip-9");

        // Verify performance profiles
        assert!(matches!(
            hot.performance_profile,
            nestgate_zfs::config::PerformanceProfile::HighPerformance
        ));
        assert!(matches!(
            warm.performance_profile,
            nestgate_zfs::config::PerformanceProfile::Balanced
        ));
        assert!(matches!(
            cold.performance_profile,
            nestgate_zfs::config::PerformanceProfile::HighCompression
        ));
    Ok(())
    }

    #[test]
    fn test_migration_rules_thresholds() -> Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();

        let hot = config.get_tier_config(&CoreStorageTier::Hot);
        let warm = config.get_tier_config(&CoreStorageTier::Warm);
        let cold = config.get_tier_config(&CoreStorageTier::Cold);

        // Hot tier should migrate faster than warm
        assert!(hot.migration_rules.age_threshold_days < warm.migration_rules.age_threshold_days);
        assert!(warm.migration_rules.age_threshold_days < cold.migration_rules.age_threshold_days);

        // Access frequency thresholds should decrease
        assert!(
            hot.migration_rules.access_frequency_threshold
                > warm.migration_rules.access_frequency_threshold
        );
    Ok(())
    }

    #[test]
    fn test_capacity_limits() -> Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();

        let hot = config.get_tier_config(&CoreStorageTier::Hot);
        let warm = config.get_tier_config(&CoreStorageTier::Warm);
        let cold = config.get_tier_config(&CoreStorageTier::Cold);

        // Cold tier should allow higher utilization
        assert!(cold.capacity_limits.max_utilization > warm.capacity_limits.max_utilization);
        assert!(warm.capacity_limits.max_utilization > hot.capacity_limits.max_utilization);
    Ok(())
    }
    Ok(())
} 