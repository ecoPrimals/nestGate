//
// Tests for automation and lifecycle management

//! Automation Tests module

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
mod automation_unit_tests {
    use super::*;

    #[test]
    fn test_tier_scoring_algorithm_basic() -> Result<(), Box<dyn std::error::Error>> {
        let large_frequently_accessed = nestgate_zfs::automation::DatasetMetrics {
            file_size: 10 * 1024 * 1024 * 1024, // 10GB
            days_since_access: 1.0,             // Accessed yesterday
            access_frequency: 50.0,             // High frequency
        };

        let tier_scoring = nestgate_zfs::automation::TierScoring::new();
        let recommendation = tier_scoring.evaluate_optimal_tier(&large_frequently_accessed);

        assert_eq!(recommendation.recommended_tier, StorageTier::Hot);
        assert!(recommendation.confidence > 0.5);
    Ok(())
    }

    #[test]
    fn test_dataset_lifecycle_creation() -> Result<(), Box<dyn std::error::Error>> {
        let dataset = DatasetLifecycle {
            dataset_name: "test-dataset".to_string(),
            current_tier: StorageTier::Hot.into(),
            created: SystemTime::now(),
            last_accessed: Some(SystemTime::now()),
            access_count: 10,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::New,
            automation_history: vec![],
        };

        assert_eq!(dataset.dataset_name, "test-dataset");
        assert_eq!(dataset.access_count, 10);
        assert!(matches!(dataset.lifecycle_stage, LifecycleStage::New));
    Ok(())
    }

    #[test]
    fn test_lifecycle_rule_validation() -> Result<(), Box<dyn std::error::Error>> {
        let rule = LifecycleRule {
            name: "test-rule".to_string(),
            condition: "age_days>30".to_string(),
            action: "migrate_to_cold".to_string(),
            enabled: true,
        };

        assert_eq!(rule.name, "test-rule");
        assert!(rule.enabled);
        assert!(!rule.condition.is_empty());
        assert!(!rule.action.is_empty());
    Ok(())
    }
    Ok(())
} 