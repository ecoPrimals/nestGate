// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use nestgate_core::unified_enums::storage_types::StorageTier;
use std::time::{Duration, SystemTime};

#[test]
fn test_lifecycle_stage_variants() {
    assert_eq!(LifecycleStage::Created, LifecycleStage::Created);
    assert_ne!(LifecycleStage::Active, LifecycleStage::Aging);
}

#[test]
fn test_lifecycle_config_default() {
    let config = LifecycleConfig::default();
    assert_eq!(config.max_concurrent_actions, 5);
    assert!(config.require_approval_for_destructive);
    assert_eq!(config.default_policies[0], "standard");
}

#[test]
fn test_lifecycle_stats_default() {
    let stats = LifecycleStats::default();
    assert_eq!(stats.total_datasets, 0);
    assert!(stats.last_evaluation_time.is_none());
}

#[test]
fn test_comparison_operators() {
    assert!(matches!(
        ComparisonOperator::GreaterThan,
        ComparisonOperator::GreaterThan
    ));
    assert!(matches!(
        ComparisonOperator::LessThan,
        ComparisonOperator::LessThan
    ));
    assert!(matches!(
        ComparisonOperator::Equal,
        ComparisonOperator::Equal
    ));
    assert!(matches!(
        ComparisonOperator::GreaterThanOrEqual,
        ComparisonOperator::GreaterThanOrEqual
    ));
    assert!(matches!(
        ComparisonOperator::LessThanOrEqual,
        ComparisonOperator::LessThanOrEqual
    ));
}

#[test]
fn test_lifecycle_action_variants() {
    assert!(matches!(
        LifecycleAction::ChangeTier(StorageTier::Cold),
        LifecycleAction::ChangeTier(_)
    ));
    assert!(matches!(
        LifecycleAction::EnableCompression,
        LifecycleAction::EnableCompression
    ));
    let _dedup = LifecycleAction::EnableDeduplication;
    let _snap = LifecycleAction::CreateSnapshot;
    let _notify = LifecycleAction::SendNotification("msg".to_string());
    let _script = LifecycleAction::ExecuteScript("echo hi".to_string());
    let _del = LifecycleAction::ScheduleDeletion(Duration::from_secs(86400));
    let _props = LifecycleAction::UpdateProperties(HashMap::from([(
        "key".to_string(),
        "value".to_string(),
    )]));
}

#[tokio::test]
async fn test_lifecycle_manager_new() {
    let _manager = DatasetLifecycleManager::new();
}

#[tokio::test]
async fn test_lifecycle_manager_default() {
    let _manager = DatasetLifecycleManager::default();
}

#[test]
fn test_scheduled_task_variants() {
    assert!(matches!(
        ScheduledTask::PolicyUpdate,
        ScheduledTask::PolicyUpdate
    ));
    assert!(matches!(
        ScheduledTask::StatsCollection,
        ScheduledTask::StatsCollection
    ));
    assert!(matches!(
        ScheduledTask::EvaluateDataset("ds".to_string()),
        ScheduledTask::EvaluateDataset(_)
    ));
    assert!(matches!(
        ScheduledTask::ExecuteAction("ds".to_string(), LifecycleAction::EnableCompression),
        ScheduledTask::ExecuteAction(_, _)
    ));
}

#[tokio::test]
async fn test_add_and_remove_dataset() {
    let manager = DatasetLifecycleManager::new();
    manager.add_dataset("test-pool/data").await.unwrap();
    let state = manager.get_dataset_state("test-pool/data").await;
    assert!(state.is_some());
    assert_eq!(state.unwrap().current_stage, LifecycleStage::Created);
    manager.remove_dataset("test-pool/data").await.unwrap();
    assert!(manager.get_dataset_state("test-pool/data").await.is_none());
}

#[tokio::test]
async fn test_initialize_and_shutdown() {
    let manager = DatasetLifecycleManager::new();
    manager.initialize().await.unwrap();
    let stats = manager.get_stats().await;
    assert!(!format!("{stats:?}").is_empty());
    manager.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_execute_action() {
    let manager = DatasetLifecycleManager::new();
    manager.add_dataset("test-ds").await.unwrap();
    manager
        .execute_action("test-ds", LifecycleAction::EnableCompression)
        .await
        .unwrap();
    assert_eq!(manager.get_stats().await.total_actions_executed, 1);
}

#[tokio::test]
async fn test_add_policy() {
    let manager = DatasetLifecycleManager::new();
    let policy = LifecyclePolicy {
        name: "custom".to_string(),
        description: "Custom policy".to_string(),
        transitions: vec![],
        stage_actions: HashMap::new(),
        priority: 50,
        enabled: true,
    };
    manager.add_policy(policy).await.unwrap();
}

#[test]
fn test_lifecycle_transition_creation() {
    let transition = LifecycleTransition {
        from_stage: LifecycleStage::Active,
        to_stage: LifecycleStage::Aging,
        conditions: vec![TransitionCondition::AgeExceeds(Duration::from_secs(86400))],
        min_stage_duration: Duration::from_secs(3600),
        requires_approval: false,
    };
    assert_eq!(transition.from_stage, LifecycleStage::Active);
    assert!(!transition.requires_approval);
}

#[test]
fn test_transition_condition_variants() {
    assert!(matches!(
        TransitionCondition::AgeExceeds(Duration::from_secs(1)),
        TransitionCondition::AgeExceeds(_)
    ));
    assert!(matches!(
        TransitionCondition::AccessBelowThreshold(10),
        TransitionCondition::AccessBelowThreshold(10)
    ));
    assert!(matches!(
        TransitionCondition::SizeExceeds(1_000_000),
        TransitionCondition::SizeExceeds(1_000_000)
    ));
    assert!(matches!(
        TransitionCondition::TierMatches(StorageTier::Hot),
        TransitionCondition::TierMatches(_)
    ));
    let custom =
        TransitionCondition::CustomMetric("cpu".to_string(), 0.8, ComparisonOperator::GreaterThan);
    assert!(matches!(custom, TransitionCondition::CustomMetric(_, _, _)));
}

#[test]
fn test_dataset_lifecycle_state_creation() {
    let state = DatasetLifecycleState {
        dataset_name: "pool/ds".to_string(),
        current_stage: LifecycleStage::Created,
        stage_entered_at: SystemTime::now(),
        last_evaluated_at: SystemTime::now(),
        applied_policies: vec!["standard".to_string()],
        pending_actions: vec![],
        metrics: HashMap::new(),
    };
    assert_eq!(state.dataset_name, "pool/ds");
    assert_eq!(state.applied_policies.len(), 1);
}

#[test]
fn test_lifecycle_evaluation_creation() {
    let eval = LifecycleEvaluation {
        dataset_name: "pool/ds".to_string(),
        current_stage: LifecycleStage::Active,
        recommended_stage: Some(LifecycleStage::Aging),
        recommended_actions: vec![LifecycleAction::EnableCompression],
        applied_policies: vec!["standard".to_string()],
        evaluation_timestamp: SystemTime::now(),
        next_evaluation: SystemTime::now() + Duration::from_secs(3600),
    };
    assert_eq!(eval.recommended_stage, Some(LifecycleStage::Aging));
    assert_eq!(eval.recommended_actions.len(), 1);
}

#[tokio::test]
async fn test_with_config() {
    let config = LifecycleConfig {
        evaluation_interval: Duration::from_secs(7200),
        max_concurrent_actions: 10,
        require_approval_for_destructive: false,
        default_policies: vec!["custom".to_string()],
    };
    let _manager = DatasetLifecycleManager::with_config(config);
}

#[tokio::test]
async fn test_evaluate_dataset_not_found() {
    let manager = DatasetLifecycleManager::new();
    assert!(manager.evaluate_dataset("nonexistent").await.is_err());
}

#[tokio::test]
async fn test_execute_various_actions() {
    let manager = DatasetLifecycleManager::new();
    manager.add_dataset("ds").await.unwrap();

    for action in [
        LifecycleAction::ChangeTier(StorageTier::Cold),
        LifecycleAction::CreateSnapshot,
        LifecycleAction::SendNotification("alert".to_string()),
        LifecycleAction::ScheduleDeletion(Duration::from_secs(3600)),
        LifecycleAction::UpdateProperties(HashMap::from([(
            "compression".to_string(),
            "lz4".to_string(),
        )])),
    ] {
        manager.execute_action("ds", action).await.unwrap();
    }
    assert_eq!(manager.get_stats().await.total_actions_executed, 5);
}
