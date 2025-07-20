//! Lifecycle Management Unit Tests
//!
//! Tests for Phase 2 lifecycle management

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use nestgate_core::StorageTier as CoreStorageTier;
use nestgate_zfs::performance::TierMetrics;
use nestgate_zfs::performance::{AlertCondition, AlertMetric, AlertOperator, AlertSeverity};
use nestgate_zfs::{
    automation::{DatasetLifecycle, LifecycleRule, LifecycleStage},
    config::ZfsConfig,
    migration::{MigrationJob, MigrationPriority, MigrationStatus},
    snapshot::*,
    types::StorageTier,
};

// Helper function for stage determination
fn determine_lifecycle_stage(age_days: f64, access_count: f64, days_since_access: f64) -> LifecycleStage {
    if age_days < 7.0 {
        LifecycleStage::New
    } else if access_count > 10.0 && days_since_access < 30.0 {
        LifecycleStage::Active
    } else {
        LifecycleStage::Archived
    }
}

// Helper function for condition evaluation
fn evaluate_lifecycle_condition(
    condition: &str,
    age_days: f64,
    access_count: f64,
    days_since_access: f64,
    size_bytes: u64,
) -> bool {
    let size_gb = size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    
    match condition {
        "age_days>30" => age_days > 30.0,
        "age_days<30" => age_days < 30.0,
        "access_count<5" => access_count < 5.0,
        "days_since_access>60" => days_since_access > 60.0,
        "size_gb>1" => size_gb > 1.0,
        _ => false,
    }
}

#[cfg(test)]
mod phase2_lifecycle_management_tests {
    use super::*;

    #[test]
    fn test_lifecycle_stage_progression() {
        // Test stage progression logic
        let test_cases = vec![
            (0.5, 10.0, 1.0, LifecycleStage::New),        // New dataset
            (30.0, 100.0, 2.0, LifecycleStage::Active),   // Frequently accessed
            (90.0, 5.0, 30.0, LifecycleStage::Archived),  // Older, archived
            (365.0, 1.0, 90.0, LifecycleStage::Archived), // Very old, archived
        ];

        for (age_days, access_count, days_since_access, expected_stage) in test_cases {
            let stage = determine_lifecycle_stage(age_days, access_count, days_since_access);
            assert_eq!(
                stage, expected_stage,
                "Failed for age: {age_days}, access: {access_count}, since: {days_since_access}"
            );
        }
    }

    #[test]
    fn test_condition_parsing_and_evaluation() {
        let test_conditions = vec![
            ("age_days>30", 45.0, 10.0, 100.0, 1024, true),
            ("age_days<30", 45.0, 10.0, 100.0, 1024, false),
            ("access_count<5", 20.0, 3.0, 50.0, 1024, true),
            ("days_since_access>60", 20.0, 10.0, 80.0, 1024, true),
            ("size_gb>1", 20.0, 10.0, 50.0, 2 * 1024 * 1024 * 1024, true),
        ];

        for (condition, age, access_count, days_since_access, size_bytes, expected) in
            test_conditions
        {
            let result = evaluate_lifecycle_condition(
                condition,
                age,
                access_count,
                days_since_access,
                size_bytes,
            );
            assert_eq!(
                result, expected,
                "Condition evaluation failed for: {condition}"
            );
        }
    }

    #[test]
    fn test_automated_lifecycle_transitions() {
        let mut dataset = DatasetLifecycle {
            dataset_name: "test-dataset".to_string(),
            current_tier: StorageTier::Hot.into(),
            created: SystemTime::now() - Duration::from_secs(86400 * 100), // 100 days ago
            last_accessed: Some(SystemTime::now() - Duration::from_secs(86400 * 50)), // 50 days ago
            access_count: 5,
            total_migrations: 0,
            last_optimization: None,
            lifecycle_stage: LifecycleStage::New,
            automation_history: vec![],
        };

        // Simulate aging process
        let new_stage = determine_lifecycle_stage(100.0, 5.0, 50.0);
        assert_eq!(
            new_stage,
            LifecycleStage::Archived,
            "Dataset should be in archived stage after 100 days"
        );

        dataset.lifecycle_stage = new_stage;
        assert_eq!(dataset.lifecycle_stage, LifecycleStage::Archived);
    }
} 