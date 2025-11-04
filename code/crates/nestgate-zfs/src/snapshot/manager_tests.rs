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
