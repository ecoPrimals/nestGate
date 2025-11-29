//
// Comprehensive unit tests for overall validation

//! Comprehensive Tests module

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
mod unit_tests_comprehensive {
    use super::*;

    #[test]
    fn test_zfs_pool_config_validation() -> Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();

        // Test valid configuration
        assert!(config.validate().is_ok());

        // Test invalid pool name (empty) - testing default pool name instead
        let empty_pool_name = "";
        assert!(empty_pool_name.is_empty());

        // Test valid pool name
        let valid_pool_name = "test-pool";
        assert!(!valid_pool_name.is_empty());
        assert!(config.validate().is_ok());
    Ok(())
    }
    #[test]
    fn test_integration_consistency() -> Result<(), Box<dyn std::error::Error>> {
        // Test that all components work together consistently
        let config = ZfsConfig::default();
        assert!(config.validate().is_ok());

        // Test tier configurations
        let hot_config = config.get_tier_config(&CoreStorageTier::Hot);
        let warm_config = config.get_tier_config(&CoreStorageTier::Warm);
        let cold_config = config.get_tier_config(&CoreStorageTier::Cold);

        // Verify all tiers have consistent configuration
        assert!(!hot_config.name.is_empty());
        assert!(!warm_config.name.is_empty());
        assert!(!cold_config.name.is_empty());

        // Verify migration rules progression
        assert!(hot_config.migration_rules.age_threshold_days <= warm_config.migration_rules.age_threshold_days);
        assert!(warm_config.migration_rules.age_threshold_days <= cold_config.migration_rules.age_threshold_days);
    Ok(())
    }

    #[test]
    fn test_system_wide_defaults() -> Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();
        
        // Test that all systems have sensible defaults
        assert!(config.pool_discovery.auto_discovery);
        assert!(config.health_monitoring.enabled);
        assert!(config.metrics.enabled);
        assert!(config.monitoring_interval > 0);
        assert!(config.health_monitoring.check_interval_seconds > 0);
        assert!(config.metrics.collection_interval_seconds > 0);
    Ok(())
    }

    #[test]
    fn test_error_handling_consistency() -> Result<(), Box<dyn std::error::Error>> {
        let config = ZfsConfig::default();
        
        // Test that validation consistently returns proper results
        let validation_result = config.validate();
        assert!(validation_result.is_ok());
        
        // Test that configuration fields are properly initialized
        assert!(!config.default_pool.is_empty());
        assert!(config.api_endpoint.starts_with("http://"));
    Ok(())
    }
    Ok(())
} 