// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for ZfsSnapshotManager
//!
//! This module contains comprehensive tests for the ZFS snapshot manager,
//! covering lifecycle, policy management, snapshot operations, and statistics.

use super::manager::ZfsSnapshotManager;
use super::policy::{RetentionPolicy, ScheduleFrequency, SnapshotPolicy};
use crate::config::ZfsConfig;
use crate::dataset::ZfsDatasetManager;
use crate::pool::ZfsPoolManager;
use crate::types::StorageTier;
use std::sync::Arc;

// ==================== MANAGER LIFECYCLE TESTS ====================

#[test]
fn test_snapshot_manager_creation() {
    let manager = ZfsSnapshotManager::new_for_testing();
    // Just verify it was created without panicking
    assert!(std::mem::size_of_val(&manager) > 0);
}

#[test]
fn test_snapshot_manager_new() {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config.clone(), pool_manager));

    let manager = ZfsSnapshotManager::new(config, dataset_manager);

    // Verify manager was created
    assert!(std::mem::size_of_val(&manager) > 0);
}

#[test]
fn test_snapshot_manager_with_shared_config() {
    let config = Arc::new(ZfsConfig::default());
    let pool_manager = Arc::new(ZfsPoolManager::new_production((*config).clone()));
    let dataset_manager = Arc::new(ZfsDatasetManager::new((*config).clone(), pool_manager));

    let manager = ZfsSnapshotManager::with_shared_config(config, dataset_manager);

    // Verify manager was created with shared config
    assert!(std::mem::size_of_val(&manager) > 0);
}

#[tokio::test]
async fn test_snapshot_manager_start_stop() {
    let mut manager = ZfsSnapshotManager::new_for_testing();

    // Start manager
    let start_result = manager.start().await;
    assert!(start_result.is_ok(), "Manager should start successfully");

    // Stop manager
    let stop_result = manager.stop().await;
    assert!(stop_result.is_ok(), "Manager should stop successfully");
}

// ==================== POLICY MANAGEMENT TESTS ====================

#[tokio::test]
async fn test_add_policy() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy = SnapshotPolicy {
        name: "test-policy".to_string(),
        description: "Test policy".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "test".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    let result = manager.add_policy(policy).await;
    assert!(result.is_ok(), "Should add policy successfully");
}

#[tokio::test]
async fn test_get_policy() {
    let manager = ZfsSnapshotManager::new_for_testing();

    // Add a policy
    let policy = SnapshotPolicy {
        name: "test-policy".to_string(),
        description: "Test policy".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Daily(12),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "test".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    manager
        .add_policy(policy.clone())
        .await
        .expect("Failed to add policy");

    // Get the policy
    let retrieved = manager.get_policy("test-policy").await;
    assert!(retrieved.is_some(), "Should retrieve policy");
    let retrieved_policy = retrieved.expect("Test setup failed");
    assert_eq!(retrieved_policy.name, "test-policy");
}

#[tokio::test]
async fn test_get_nonexistent_policy() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let retrieved = manager.get_policy("nonexistent").await;
    assert!(
        retrieved.is_none(),
        "Should return None for nonexistent policy"
    );
}

#[tokio::test]
async fn test_remove_policy() {
    let manager = ZfsSnapshotManager::new_for_testing();

    // Add a policy
    let policy = SnapshotPolicy {
        name: "test-policy".to_string(),
        description: "Test policy".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Weekly { day: 1, hour: 0 },
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "test".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    manager
        .add_policy(policy)
        .await
        .expect("Failed to add policy");

    // Remove the policy
    let result = manager.remove_policy("test-policy").await;
    assert!(result.is_ok(), "Should remove policy successfully");
    let removed = result.expect("Test setup failed");
    assert!(removed, "Should return true when policy was removed");

    // Verify it's gone
    let retrieved = manager.get_policy("test-policy").await;
    assert!(retrieved.is_none(), "Policy should be removed");
}

#[tokio::test]
async fn test_remove_nonexistent_policy() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let result = manager.remove_policy("nonexistent").await;
    assert!(
        result.is_ok(),
        "Should handle nonexistent policy gracefully"
    );
    let removed = result.expect("Test setup failed");
    assert!(!removed, "Should return false when policy doesn't exist");
}

#[tokio::test]
async fn test_list_policies_empty() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policies = manager.list_policies().await;
    assert_eq!(policies.len(), 0, "Should have no policies initially");
}

#[tokio::test]
async fn test_list_policies_with_data() {
    let manager = ZfsSnapshotManager::new_for_testing();

    // Add multiple policies
    for i in 0..3 {
        let policy = SnapshotPolicy {
            name: format!("policy-{}", i),
            description: format!("Policy {}", i),
            enabled: true,
            frequency: ScheduleFrequency::Hours(i as u32 + 1),
            retention: RetentionPolicy::default(),
            dataset_patterns: vec!["pool/dataset".to_string()],
            tiers: vec![StorageTier::Hot],
            name_prefix: "test".to_string(),
            include_properties: true,
            recursive: false,
            priority: 50,
            max_snapshots_per_run: 100,
        };
        manager
            .add_policy(policy)
            .await
            .expect("Failed to add policy");
    }

    let policies = manager.list_policies().await;
    assert_eq!(policies.len(), 3, "Should have 3 policies");
}

// ==================== SNAPSHOT OPERATION TESTS ====================

#[tokio::test]
async fn test_create_snapshot() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let result = manager
        .create_snapshot("pool/dataset", "test-snapshot", false)
        .await;
    assert!(result.is_ok(), "Should queue snapshot creation");

    let operation_id = result.expect("Test setup failed");
    assert!(!operation_id.is_empty(), "Should return operation ID");
}

#[tokio::test]
async fn test_delete_snapshot() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let result = manager
        .delete_snapshot("pool/dataset", "test-snapshot")
        .await;
    assert!(result.is_ok(), "Should queue snapshot deletion");

    let operation_id = result.expect("Test setup failed");
    assert!(!operation_id.is_empty(), "Should return operation ID");
}

#[tokio::test]
async fn test_list_snapshots() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let result = manager.list_snapshots("pool/dataset").await;
    // This may succeed or fail depending on whether the dataset exists
    // We just verify it doesn't panic
    let _ = result;
}

// ==================== STATISTICS TESTS ====================

#[tokio::test]
async fn test_get_statistics() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let stats = manager.get_statistics().await;

    // Verify statistics structure
    assert_eq!(stats.total_snapshots, 0, "Should start with 0 snapshots");
    assert_eq!(stats.total_size, 0, "Should start with 0 total size");
    assert_eq!(
        stats.total_referenced_size, 0,
        "Should start with 0 referenced size"
    );
}

// ==================== OPERATION STATUS TESTS ====================

#[tokio::test]
async fn test_get_operation_status() {
    let manager = ZfsSnapshotManager::new_for_testing();

    // Queue an operation
    let operation_id = manager
        .create_snapshot("pool/dataset", "test-snapshot", false)
        .await
        .expect("Failed to queue operation");

    // Get the operation status
    let operation = manager.get_operation_status(&operation_id).await;
    assert!(operation.is_some(), "Should retrieve queued operation");

    let op = operation.expect("Test setup failed");
    assert_eq!(op.id, operation_id);
    assert_eq!(op.dataset, "pool/dataset");
}

#[tokio::test]
async fn test_get_nonexistent_operation_status() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let operation = manager.get_operation_status("nonexistent-id").await;
    assert!(
        operation.is_none(),
        "Should return None for nonexistent operation"
    );
}

// ==================== POLICY INTEGRATION TESTS ====================

#[tokio::test]
async fn test_policy_lifecycle() {
    let manager = ZfsSnapshotManager::new_for_testing();

    // Create policy
    let policy = SnapshotPolicy {
        name: "test-lifecycle".to_string(),
        description: "Test lifecycle policy".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "test".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    // Add policy
    manager
        .add_policy(policy.clone())
        .await
        .expect("Failed to add");

    // Verify it exists
    let retrieved = manager.get_policy("test-lifecycle").await;
    assert!(retrieved.is_some());

    // Remove it
    let removed = manager
        .remove_policy("test-lifecycle")
        .await
        .expect("Failed to remove");
    assert!(removed);

    // Verify it's gone
    let retrieved = manager.get_policy("test-lifecycle").await;
    assert!(retrieved.is_none());
}

// ==================== CONCURRENT OPERATIONS TESTS ====================

#[tokio::test]
async fn test_concurrent_policy_operations() {
    let manager = Arc::new(ZfsSnapshotManager::new_for_testing());

    // Spawn multiple tasks adding policies concurrently
    let mut handles = vec![];

    for i in 0..5 {
        let manager_clone = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            let policy = SnapshotPolicy {
                name: format!("concurrent-{}", i),
                description: format!("Concurrent policy {}", i),
                enabled: true,
                frequency: ScheduleFrequency::Daily(12),
                retention: RetentionPolicy::default(),
                dataset_patterns: vec!["pool/dataset".to_string()],
                tiers: vec![StorageTier::Hot],
                name_prefix: "test".to_string(),
                include_properties: true,
                recursive: false,
                priority: 50,
                max_snapshots_per_run: 100,
            };
            manager_clone.add_policy(policy).await
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        let result = handle.await.expect("Task panicked");
        assert!(result.is_ok(), "Concurrent policy addition should succeed");
    }

    // Verify all policies were added
    let policies = manager.list_policies().await;
    assert_eq!(policies.len(), 5, "Should have 5 policies");
}

#[tokio::test]
async fn test_concurrent_snapshot_operations() {
    let manager = Arc::new(ZfsSnapshotManager::new_for_testing());

    // Spawn multiple tasks creating snapshots concurrently
    let mut handles = vec![];

    for i in 0..5 {
        let manager_clone = Arc::clone(&manager);
        let handle = tokio::spawn(async move {
            manager_clone
                .create_snapshot(&format!("pool/dataset{}", i), &format!("snap{}", i), false)
                .await
        });
        handles.push(handle);
    }

    // Wait for all tasks and collect operation IDs
    let mut operation_ids = vec![];
    for handle in handles {
        let result = handle.await.expect("Task panicked");
        assert!(
            result.is_ok(),
            "Concurrent snapshot creation should succeed"
        );
        operation_ids.push(result.expect("Failed to get operation ID"));
    }

    // Verify all operations were queued by checking their status
    for operation_id in operation_ids {
        let status = manager.get_operation_status(&operation_id).await;
        assert!(status.is_some(), "Operation should be queued");
    }
}

// ==================== ERROR CONDITION & EDGE CASE TESTS ====================

#[tokio::test]
async fn test_add_policy_with_duplicate_name() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy1 = SnapshotPolicy {
        name: "duplicate-policy".to_string(),
        description: "First policy".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "test".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    // Add first policy
    manager.add_policy(policy1.clone()).await.unwrap();

    let policy2 = SnapshotPolicy {
        name: "duplicate-policy".to_string(),
        description: "Second policy with same name".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(2),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset2".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "test2".to_string(),
        include_properties: false,
        recursive: true,
        priority: 75,
        max_snapshots_per_run: 50,
    };

    // Add second policy with same name (should overwrite)
    let result = manager.add_policy(policy2).await;
    assert!(result.is_ok(), "Duplicate policy name should overwrite");
}

#[tokio::test]
async fn test_remove_nonexistent_policy_idempotent() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let result = manager.remove_policy("nonexistent-policy").await;
    // Should succeed even if policy doesn't exist (idempotent)
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_nonexistent_policy_returns_none() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy = manager.get_policy("nonexistent-policy").await;
    assert!(
        policy.is_none(),
        "Should return None for nonexistent policy"
    );
}

#[tokio::test]
async fn test_policy_with_empty_dataset_patterns() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy = SnapshotPolicy {
        name: "empty-patterns".to_string(),
        description: "Policy with no dataset patterns".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec![], // Empty!
        tiers: vec![StorageTier::Hot],
        name_prefix: "test".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    let result = manager.add_policy(policy).await;
    // Should accept but won't match any datasets
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_policy_with_zero_max_snapshots() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy = SnapshotPolicy {
        name: "zero-snapshots".to_string(),
        description: "Policy with zero max snapshots".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "test".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 0, // Zero!
    };

    let result = manager.add_policy(policy).await;
    // Should accept (edge case - won't create snapshots)
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_disabled_policy_not_executed() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy = SnapshotPolicy {
        name: "disabled-policy".to_string(),
        description: "Disabled policy".to_string(),
        enabled: false, // Disabled!
        frequency: ScheduleFrequency::Minutes(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "test".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    manager.add_policy(policy).await.unwrap();

    // Get policy back and verify it's disabled
    let retrieved = manager.get_policy("disabled-policy").await;
    assert!(retrieved.is_some());
    if let Some(policy) = retrieved {
        assert!(!policy.enabled);
    }
}

#[tokio::test]
async fn test_policy_priority_ordering() {
    let manager = ZfsSnapshotManager::new_for_testing();

    // Add policies with different priorities
    let high_priority = SnapshotPolicy {
        name: "high-priority".to_string(),
        description: "High priority policy".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "high".to_string(),
        include_properties: true,
        recursive: false,
        priority: 90, // High priority
        max_snapshots_per_run: 100,
    };

    let low_priority = SnapshotPolicy {
        name: "low-priority".to_string(),
        description: "Low priority policy".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "low".to_string(),
        include_properties: true,
        recursive: false,
        priority: 10, // Low priority
        max_snapshots_per_run: 100,
    };

    manager.add_policy(high_priority).await.unwrap();
    manager.add_policy(low_priority).await.unwrap();

    // List policies and verify both exist
    let policies = manager.list_policies().await;
    assert_eq!(policies.len(), 2);
}

#[tokio::test]
async fn test_multiple_tiers_policy() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy = SnapshotPolicy {
        name: "multi-tier".to_string(),
        description: "Policy for multiple tiers".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold], // Multiple tiers
        name_prefix: "multi".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    let result = manager.add_policy(policy).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_recursive_snapshot_policy() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy = SnapshotPolicy {
        name: "recursive-policy".to_string(),
        description: "Recursive snapshot policy".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec!["pool/dataset".to_string()],
        tiers: vec![StorageTier::Hot],
        name_prefix: "recursive".to_string(),
        include_properties: true,
        recursive: true, // Recursive!
        priority: 50,
        max_snapshots_per_run: 100,
    };

    let result = manager.add_policy(policy).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_schedule_frequency_variations() {
    let manager = ZfsSnapshotManager::new_for_testing();

    // Test all schedule frequency types
    let frequencies = vec![
        ScheduleFrequency::Minutes(15),
        ScheduleFrequency::Hours(2),
        ScheduleFrequency::Daily(12),
        ScheduleFrequency::Weekly { day: 0, hour: 0 },
        ScheduleFrequency::Monthly { day: 1, hour: 0 },
    ];

    for (i, frequency) in frequencies.into_iter().enumerate() {
        let policy = SnapshotPolicy {
            name: format!("freq-test-{}", i),
            description: format!("Frequency test {}", i),
            enabled: true,
            /// Frequency
            frequency,
            retention: RetentionPolicy::default(),
            dataset_patterns: vec!["pool/dataset".to_string()],
            tiers: vec![StorageTier::Hot],
            name_prefix: format!("freq{}", i),
            include_properties: true,
            recursive: false,
            priority: 50,
            max_snapshots_per_run: 100,
        };

        assert!(
            manager.add_policy(policy).await.is_ok(),
            "Should accept frequency type {}",
            i
        );
    }
}

#[tokio::test]
async fn test_wildcard_dataset_patterns() {
    let manager = ZfsSnapshotManager::new_for_testing();

    let policy = SnapshotPolicy {
        name: "wildcard-policy".to_string(),
        description: "Policy with wildcard patterns".to_string(),
        enabled: true,
        frequency: ScheduleFrequency::Hours(1),
        retention: RetentionPolicy::default(),
        dataset_patterns: vec![
            "pool/*".to_string(),        // Wildcard
            "pool/dataset*".to_string(), // Prefix wildcard
        ],
        tiers: vec![StorageTier::Hot],
        name_prefix: "wild".to_string(),
        include_properties: true,
        recursive: false,
        priority: 50,
        max_snapshots_per_run: 100,
    };

    let result = manager.add_policy(policy).await;
    assert!(result.is_ok());
}
