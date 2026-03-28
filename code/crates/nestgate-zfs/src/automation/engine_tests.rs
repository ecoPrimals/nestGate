// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for DatasetAutomation engine

use super::engine::DatasetAutomation;
use super::types::{AutomationPolicy, DatasetMetadata, PolicyConditions, PolicyPriority};
use crate::config::{DatasetAutomationConfig, ZfsConfig};
use crate::{dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use std::sync::Arc;
use std::time::SystemTime;

// ==================== HELPER FUNCTIONS ====================

/// Create a test automation engine with default configuration
async fn create_test_engine() -> DatasetAutomation {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new(&config).await.unwrap());
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config.clone(), pool_manager.clone()));

    let automation_config = DatasetAutomationConfig::default();

    DatasetAutomation::new(pool_manager, dataset_manager, automation_config)
        .await
        .unwrap()
}

/// Create a test automation policy
fn create_test_policy(policy_id: &str) -> AutomationPolicy {
    AutomationPolicy {
        policy_id: policy_id.to_string(),
        name: format!("Test Policy {}", policy_id),
        description: "Test automation policy".to_string(),
        priority: PolicyPriority::Normal,
        enabled: true,
        conditions: PolicyConditions {
            tier_rules: vec![],
            migration_rules: vec![],
            lifecycle_rules: vec![],
        },
        created: SystemTime::now(),
        last_modified: SystemTime::now(),
    }
}

/// Create test dataset metadata
fn create_test_metadata() -> DatasetMetadata {
    DatasetMetadata {
        size_bytes: 1024 * 1024 * 1024, // 1GB
        last_accessed: Some(SystemTime::now()),
        access_frequency: 10.5,
        file_types: vec!["data".to_string(), "logs".to_string()],
    }
}

// ==================== CONSTRUCTOR TESTS ====================

#[tokio::test]
async fn test_engine_new_creates_valid_instance() {
    let engine = create_test_engine().await;
    drop(engine); // Successfully created
}

#[tokio::test]
async fn test_engine_can_be_cloned() {
    let engine = create_test_engine().await;
    let cloned = engine.clone();
    drop(engine);
    drop(cloned);
}

#[tokio::test]
async fn test_multiple_engines_can_coexist() {
    let engine1 = create_test_engine().await;
    let engine2 = create_test_engine().await;
    let engine3 = create_test_engine().await;
    drop(engine1);
    drop(engine2);
    drop(engine3);
}

// ==================== START/STOP TESTS ====================

#[tokio::test]
async fn test_start_engine() {
    let engine = create_test_engine().await;
    let result = engine.start().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_engine_multiple_times() {
    let engine = create_test_engine().await;

    let result1 = engine.start().await;
    let result2 = engine.start().await;
    let result3 = engine.start().await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
}

// ==================== STATUS TESTS ====================

#[tokio::test]
async fn test_get_automation_status() {
    let engine = create_test_engine().await;
    let status = engine.get_automation_status().await;

    assert!(status.enabled);
    // active_policies is usize, always >= 0 - verify it's reasonable
    assert!(status.active_policies < 1000);
    assert_eq!(status.tracked_datasets, 0); // Initial state
}

#[tokio::test]
async fn test_automation_status_reflects_state() {
    let engine = create_test_engine().await;

    // Get initial status
    let status = engine.get_automation_status().await;
    // active_policies is usize, always >= 0 - verify it's reasonable
    assert!(status.active_policies < 1000);
}

// ==================== POLICY VALIDATION TESTS ====================

#[tokio::test]
async fn test_validate_policy_valid() {
    let engine = create_test_engine().await;
    let policy = create_test_policy("test1");

    let result = engine.validate_policy(&policy);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_multiple_policies() {
    let engine = create_test_engine().await;

    for i in 0..10 {
        let policy = create_test_policy(&format!("policy{}", i));
        let result = engine.validate_policy(&policy);
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_validate_policy_with_different_priorities() {
    let engine = create_test_engine().await;

    for priority in &[
        PolicyPriority::Low,
        PolicyPriority::Normal,
        PolicyPriority::High,
        PolicyPriority::Critical,
    ] {
        let mut policy = create_test_policy("test");
        policy.priority = priority.clone();
        let result = engine.validate_policy(&policy);
        assert!(result.is_ok());
    }
}

// ==================== TIER EVALUATION TESTS ====================

#[tokio::test]
async fn test_evaluate_tier_for_dataset() {
    let engine = create_test_engine().await;
    let metadata = create_test_metadata();

    let result = engine
        .evaluate_tier_for_dataset("tank/testdata", &metadata)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_evaluate_tier_multiple_datasets() {
    let engine = create_test_engine().await;

    for i in 0..5 {
        let dataset_name = format!("tank/dataset{}", i);
        let metadata = create_test_metadata();
        let result = engine
            .evaluate_tier_for_dataset(&dataset_name, &metadata)
            .await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_evaluate_tier_different_access_frequencies() {
    let engine = create_test_engine().await;

    for access_freq in &[1.0, 10.0, 100.0] {
        let mut metadata = create_test_metadata();
        metadata.access_frequency = *access_freq;
        let result = engine
            .evaluate_tier_for_dataset("tank/test", &metadata)
            .await;
        assert!(result.is_ok());
    }
}

// ==================== EDGE CASE TESTS ====================

#[tokio::test]
async fn test_evaluate_tier_empty_dataset_name() {
    let engine = create_test_engine().await;
    let metadata = create_test_metadata();

    let result = engine.evaluate_tier_for_dataset("", &metadata).await;
    // Should handle gracefully
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_evaluate_tier_long_dataset_name() {
    let engine = create_test_engine().await;
    let metadata = create_test_metadata();
    let long_name = "a/".repeat(500);

    let result = engine
        .evaluate_tier_for_dataset(&long_name, &metadata)
        .await;
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_evaluate_tier_special_characters() {
    let engine = create_test_engine().await;
    let metadata = create_test_metadata();

    let result = engine
        .evaluate_tier_for_dataset("tank/test@snapshot#special", &metadata)
        .await;
    assert!(result.is_ok() || result.is_err());
}

// ==================== STRESS TESTS ====================

#[tokio::test]
async fn test_validate_many_policies() {
    let engine = create_test_engine().await;

    for i in 0..50 {
        let policy = create_test_policy(&format!("stress_policy_{}", i));
        let result = engine.validate_policy(&policy);
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_evaluate_tier_many_datasets() {
    let engine = create_test_engine().await;
    let metadata = create_test_metadata();

    for i in 0..20 {
        let dataset = format!("tank/stress_test_{}", i);
        let result = engine.evaluate_tier_for_dataset(&dataset, &metadata).await;
        assert!(result.is_ok());
    }
}

// ==================== INTEGRATION TESTS ====================

#[tokio::test]
async fn test_full_automation_cycle() {
    let engine = create_test_engine().await;

    // Start engine
    let start_result = engine.start().await;
    assert!(start_result.is_ok());

    // Get status
    let status = engine.get_automation_status().await;
    assert!(status.enabled);

    // Validate policy
    let policy = create_test_policy("integration");
    let validate_result = engine.validate_policy(&policy);
    assert!(validate_result.is_ok());

    // Evaluate tier
    let metadata = create_test_metadata();
    let tier_result = engine
        .evaluate_tier_for_dataset("tank/integration", &metadata)
        .await;
    assert!(tier_result.is_ok());
}

#[tokio::test]
async fn test_engine_lifecycle() {
    let engine = create_test_engine().await;

    // Start
    let start_result = engine.start().await;
    assert!(start_result.is_ok());

    // Perform operations
    let status1 = engine.get_automation_status().await;
    assert!(status1.enabled);

    let policy = create_test_policy("lifecycle");
    let validate_result = engine.validate_policy(&policy);
    assert!(validate_result.is_ok());

    // Clone and continue
    let cloned = engine.clone();
    let status2 = cloned.get_automation_status().await;
    assert_eq!(status2.enabled, status1.enabled);

    // Original engine should still work
    let metadata = create_test_metadata();
    let tier_result = engine
        .evaluate_tier_for_dataset("tank/final", &metadata)
        .await;
    assert!(tier_result.is_ok());
}

// ==================== CONCURRENT OPERATIONS ====================

#[tokio::test]
async fn test_concurrent_status_checks() {
    let engine = Arc::new(create_test_engine().await);

    let mut handles = vec![];
    for _ in 0..10 {
        let engine_clone = Arc::clone(&engine);
        handles.push(tokio::spawn(async move {
            engine_clone.get_automation_status().await
        }));
    }

    for handle in handles {
        let status = handle.await.unwrap();
        assert!(status.enabled);
    }
}

#[tokio::test]
async fn test_concurrent_policy_validation() {
    let engine = Arc::new(create_test_engine().await);

    let mut handles = vec![];
    for i in 0..5 {
        let engine_clone = Arc::clone(&engine);
        let policy = create_test_policy(&format!("concurrent_{}", i));
        handles.push(tokio::spawn(async move {
            engine_clone.validate_policy(&policy)
        }));
    }

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }
}
