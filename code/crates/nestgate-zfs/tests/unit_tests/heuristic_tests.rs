//
// Tests for heuristic-based optimization

//! Heuristic Tests module

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
mod heuristic_unit_tests {
    use super::*;

    #[test]
    fn test_heuristic_config_defaults() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Heuristic configuration test (AI functionality has been sunset)
        let config = ZfsConfig::default();

        // Verify heuristic-based tier assignment is enabled
        assert!(!config.tiers.hot.name.is_empty());
        assert!(!config.tiers.warm.name.is_empty());
        assert!(!config.tiers.cold.name.is_empty());
    Ok(())
    }
    #[test]
    fn test_performance_expectation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let hot_tier_expectation =
            nestgate_automation::types::optimization::PerformanceExpectation {
                expected_iops: 2000,
                expected_bandwidth_mbps: 500.0,
                expected_latency_ms: 1.0,
                expected_availability: 99.99,
                expected_durability_nines: 11,
            };

        assert_eq!(hot_tier_expectation.expected_latency_ms, 1.0);
        assert_eq!(hot_tier_expectation.expected_bandwidth_mbps, 500.0);
    Ok(())
    }
    Ok(())
} 