//
// Tests for performance metrics and tier functionality

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
mod performance_unit_tests {
    use super::*;

    #[test]
    fn test_tier_metrics_hierarchy() -> Result<(), Box<dyn std::error::Error>> {
        // Test that tier metrics have expected hierarchy
        let hot_metrics = TierMetrics::default_for_tier(CoreStorageTier::Hot);
        let warm_metrics = TierMetrics::default_for_tier(CoreStorageTier::Warm);
        let cold_metrics = TierMetrics::default_for_tier(CoreStorageTier::Cold);

        // Hot tier should have lowest latency
        assert!(hot_metrics.avg_read_latency_ms <= warm_metrics.avg_read_latency_ms);
        assert!(warm_metrics.avg_read_latency_ms <= cold_metrics.avg_read_latency_ms);

        // Hot tier should have highest IOPS
        assert!(hot_metrics.read_iops >= warm_metrics.read_iops);
        assert!(warm_metrics.read_iops >= cold_metrics.read_iops);
    Ok(())
    }
    #[test]
    fn test_performance_profile_characteristics() -> Result<(), Box<dyn std::error::Error>> {
        // Test performance profile characteristics
        let config = ZfsConfig::default();
        let hot_config = config.get_tier_config(&CoreStorageTier::Hot);
        let warm_config = config.get_tier_config(&CoreStorageTier::Warm);
        let cold_config = config.get_tier_config(&CoreStorageTier::Cold);

        // Verify tier names
        assert_eq!(hot_config.name, "hot");
        assert_eq!(warm_config.name, "warm");
        assert_eq!(cold_config.name, "cold");

        // Verify compression settings
        assert_eq!(hot_config.properties.get("compression").unwrap_or_else(|_e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {}", "actual_error_details")
).into())
}), "lz4");
        assert_eq!(warm_config.properties.get("compression").unwrap_or_else(|_e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {}", "actual_error_details")
).into())
}), "zstd");
        assert_eq!(cold_config.properties.get("compression").unwrap_or_else(|_e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {}", "actual_error_details")
).into())
}), "gzip-9");
    Ok(())
    }

    #[test]
    fn test_alert_conditions() -> Result<(), Box<dyn std::error::Error>> {
        // Test alert condition creation
        let alert = AlertCondition {
            metric: AlertMetric::ReadLatency,
            operator: AlertOperator::GreaterThan,
            threshold: 10.0,
            severity: AlertSeverity::Warning,
            duration_seconds: 60,
        };

        assert!(matches!(alert.metric, AlertMetric::ReadLatency));
        assert!(matches!(alert.operator, AlertOperator::GreaterThan));
        assert_eq!(alert.threshold, 10.0);
        assert!(matches!(alert.severity, AlertSeverity::Warning));
        assert_eq!(alert.duration_seconds, 60);
    Ok(())
    }
    Ok(())
} 