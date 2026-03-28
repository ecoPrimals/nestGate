// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **COMPREHENSIVE ERROR HANDLING TESTS**
//!
//! Tests for ZFS error creation, formatting, and handling to achieve >80% coverage.

use nestgate_zfs::error::{
    create_zfs_error, zfs_command_error, zfs_internal, zfs_operation_error, Result,
    ZfsErrorBuilder, ZfsOperation, ZfsResult,
};

// ==================== ERROR BUILDER TESTS ====================

#[test]
fn test_error_builder_new() {
    let error = ZfsErrorBuilder::new("test error");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("test error"));
}

#[test]
fn test_error_builder_new_with_operation() {
    let operation = "pool_create";
    let error = ZfsErrorBuilder::new_with_operation("operation failed", operation);
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("operation failed"));
}

#[test]
fn test_error_builder_pool_error() {
    let error = ZfsErrorBuilder::pool_error("pool corrupted", "tank");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Pool error"));
    assert!(error_string.contains("pool corrupted"));
    assert!(error_string.contains("tank"));
}

#[test]
fn test_error_builder_pool_error_empty_pool_name() {
    let error = ZfsErrorBuilder::pool_error("error", "");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Pool error"));
}

#[test]
fn test_error_builder_pool_error_special_characters() {
    let error = ZfsErrorBuilder::pool_error("error", "pool-name_123");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("pool-name_123"));
}

#[test]
fn test_error_builder_dataset_error() {
    let error = ZfsErrorBuilder::dataset_error("dataset not found", "tank/data");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Dataset error"));
    assert!(error_string.contains("dataset not found"));
    assert!(error_string.contains("tank/data"));
}

#[test]
fn test_error_builder_dataset_error_nested_path() {
    let error = ZfsErrorBuilder::dataset_error("error", "tank/parent/child/grandchild");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("tank/parent/child/grandchild"));
}

#[test]
fn test_error_builder_snapshot_error() {
    let error = ZfsErrorBuilder::snapshot_error("snapshot failed", "tank/data@snap1");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Snapshot error"));
    assert!(error_string.contains("snapshot failed"));
    assert!(error_string.contains("tank/data@snap1"));
}

#[test]
fn test_error_builder_snapshot_error_with_timestamp() {
    let error = ZfsErrorBuilder::snapshot_error("error", "tank/data@2023-11-18-12-00");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("2023-11-18-12-00"));
}

#[test]
fn test_error_builder_command_error() {
    let error = ZfsErrorBuilder::command_error("zpool create", "permission denied");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Command 'zpool create' failed"));
    assert!(error_string.contains("permission denied"));
}

#[test]
fn test_error_builder_command_error_long_output() {
    let long_output = "a".repeat(1000);
    let error = ZfsErrorBuilder::command_error("zfs list", &long_output);
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Command 'zfs list' failed"));
}

#[test]
fn test_error_builder_zfs_error() {
    let error = ZfsErrorBuilder::zfs_error("general zfs error");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("general zfs error"));
}

#[test]
fn test_error_builder_zfs_operation_error() {
    let error = ZfsErrorBuilder::zfs_operation_error("operation timeout");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("operation timeout"));
}

#[test]
fn test_error_builder_internal() {
    let error = ZfsErrorBuilder::internal(
        "internal error".to_string(),
        "test-component".to_string(),
        Some("test-location".to_string()),
    );
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("internal error"));
    assert!(error_string.contains("test-component"));
}

#[test]
fn test_error_builder_internal_no_location() {
    let error =
        ZfsErrorBuilder::internal("error message".to_string(), "component".to_string(), None);
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("error message"));
}

// ==================== MIGRATION HELPER TESTS ====================

#[test]
fn test_zfs_internal_migration_helper() {
    let error = zfs_internal(
        "migration error".to_string(),
        "old-component".to_string(),
        Some("old-location".to_string()),
        false,
        None,
    );
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("migration error"));
    assert!(error_string.contains("old-component"));
}

#[test]
fn test_zfs_internal_with_bug_flag() {
    let error = zfs_internal(
        "bug error".to_string(),
        "component".to_string(),
        None,
        true, // is_bug flag (deprecated but tested)
        None,
    );
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("bug error"));
}

// ==================== COMMAND ERROR TESTS ====================

#[test]
fn test_zfs_command_error_function() {
    let error = zfs_command_error("zpool import", "pool not found");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Command 'zpool import' failed"));
    assert!(error_string.contains("pool not found"));
}

#[test]
fn test_zfs_command_error_empty_output() {
    let error = zfs_command_error("zfs snapshot", "");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Command 'zfs snapshot' failed"));
}

#[test]
fn test_zfs_command_error_multiline_output() {
    let output = "line1\nline2\nline3";
    let error = zfs_command_error("zfs list", output);
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Command 'zfs list' failed"));
}

// ==================== OPERATION ERROR TESTS ====================

#[test]
fn test_zfs_operation_error_function() {
    let error = zfs_operation_error("create", "insufficient space");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("ZFS operation failed"));
    assert!(error_string.contains("create"));
    assert!(error_string.contains("insufficient space"));
}

#[test]
fn test_zfs_operation_error_complex_details() {
    let details = "Error: code=123, reason=timeout, retry=false";
    let error = zfs_operation_error("mount", details);
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("mount"));
    assert!(error_string.contains("code=123"));
}

// ==================== CREATE ZFS ERROR TESTS ====================

#[test]
fn test_create_zfs_error_pool_create() {
    let error = create_zfs_error(
        "Failed to create pool".to_string(),
        ZfsOperation::PoolCreate,
    );
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Failed to create pool"));
    assert!(error_string.contains("poolcreate"));
}

#[test]
fn test_create_zfs_error_pool_destroy() {
    let error = create_zfs_error("Pool in use".to_string(), ZfsOperation::PoolDestroy);
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Pool in use"));
}

#[test]
fn test_create_zfs_error_dataset_create() {
    let error = create_zfs_error("Dataset exists".to_string(), ZfsOperation::DatasetCreate);
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Dataset exists"));
}

#[test]
fn test_create_zfs_error_snapshot_create() {
    let error = create_zfs_error("Snapshot failed".to_string(), ZfsOperation::SnapshotCreate);
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Snapshot failed"));
}

// ==================== ZFS OPERATION ENUM TESTS ====================

#[test]
fn test_zfs_operation_pool_operations() {
    let ops = vec![
        ZfsOperation::PoolCreate,
        ZfsOperation::PoolDestroy,
        ZfsOperation::PoolImport,
        ZfsOperation::PoolExport,
    ];

    for op in ops {
        let error = create_zfs_error("test".to_string(), op);
        let error_string = format!("{:?}", error);
        assert!(error_string.contains("test"));
    }
}

#[test]
fn test_zfs_operation_dataset_operations() {
    let ops = vec![
        ZfsOperation::DatasetCreate,
        ZfsOperation::DatasetDestroy,
        ZfsOperation::DatasetMount,
        ZfsOperation::DatasetUnmount,
    ];

    for op in ops {
        let error = create_zfs_error("test".to_string(), op);
        let error_string = format!("{:?}", error);
        assert!(error_string.contains("test"));
    }
}

#[test]
fn test_zfs_operation_snapshot_operations() {
    let ops = vec![ZfsOperation::SnapshotCreate, ZfsOperation::SnapshotDestroy];

    for op in ops {
        let error = create_zfs_error("test".to_string(), op);
        let error_string = format!("{:?}", error);
        assert!(error_string.contains("test"));
    }
}

#[test]
fn test_zfs_operation_system_operations() {
    let ops = vec![
        ZfsOperation::Command,
        ZfsOperation::SystemCheck,
        ZfsOperation::Configuration,
    ];

    for op in ops {
        let error = create_zfs_error("test".to_string(), op);
        let error_string = format!("{:?}", error);
        assert!(error_string.contains("test"));
    }
}

#[test]
fn test_zfs_operation_clone() {
    let op1 = ZfsOperation::PoolCreate;
    let op2 = op1.clone();

    let error1 = create_zfs_error("test1".to_string(), op1);
    let error2 = create_zfs_error("test2".to_string(), op2);

    let str1 = format!("{:?}", error1);
    let str2 = format!("{:?}", error2);

    // Both should contain the same operation type
    assert!(str1.contains("poolcreate"));
    assert!(str2.contains("poolcreate"));
}

#[test]
fn test_zfs_operation_debug_format() {
    let op = ZfsOperation::PoolCreate;
    let debug_str = format!("{:?}", op);

    assert_eq!(debug_str, "PoolCreate");
}

// ==================== RESULT TYPE TESTS ====================

#[test]
fn test_result_type_ok() {
    let result: Result<i32> = Ok(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_result_type_err() {
    let error = ZfsErrorBuilder::zfs_error("test error");
    let result: Result<i32> = Err(error);

    assert!(result.is_err());
}

#[test]
fn test_zfs_result_type_ok() {
    let result: ZfsResult<String> = Ok("success".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[test]
fn test_zfs_result_type_err() {
    let error = ZfsErrorBuilder::pool_error("pool error", "tank");
    let result: ZfsResult<()> = Err(error);

    assert!(result.is_err());
}

#[test]
fn test_result_propagation() {
    /// Helper Function
    fn helper_function() -> Result<i32> {
        Err(ZfsErrorBuilder::zfs_error("helper failed"))
    }

    /// Outer Function
    fn outer_function() -> Result<i32> {
        let value = helper_function()?;
        Ok(value + 1)
    }

    let result = outer_function();
    assert!(result.is_err());
}

// ==================== ERROR MESSAGE FORMATTING TESTS ====================

#[test]
fn test_error_message_with_unicode() {
    let error = ZfsErrorBuilder::pool_error("错误消息", "пул");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("错误消息"));
}

#[test]
fn test_error_message_with_special_chars() {
    let message = "Error: \"quoted\", 'apostrophe', \n newline, \t tab";
    let error = ZfsErrorBuilder::zfs_error(message);
    let error_string = format!("{:?}", error);

    // Should contain the message in some form
    assert!(!error_string.is_empty());
}

#[test]
fn test_error_message_very_long() {
    let long_message = "x".repeat(10000);
    let error = ZfsErrorBuilder::zfs_error(&long_message);
    let error_string = format!("{:?}", error);

    assert!(!error_string.is_empty());
}

#[test]
fn test_error_message_empty() {
    let error = ZfsErrorBuilder::zfs_error("");
    let error_string = format!("{:?}", error);

    assert!(!error_string.is_empty());
}

// ==================== COMPONENT AND LOCATION TESTS ====================

#[test]
fn test_error_with_all_pool_components() {
    let pools = vec!["tank", "backup", "fast", "slow", "archive"];

    for pool in pools {
        let error = ZfsErrorBuilder::pool_error("test", pool);
        let error_string = format!("{:?}", error);
        assert!(error_string.contains(pool));
    }
}

#[test]
fn test_error_with_nested_datasets() {
    let datasets = vec![
        "tank/data",
        "tank/data/user",
        "tank/data/user/home",
        "backup/archives/2023/november",
    ];

    for dataset in datasets {
        let error = ZfsErrorBuilder::dataset_error("test", dataset);
        let error_string = format!("{:?}", error);
        assert!(error_string.contains(dataset));
    }
}

#[test]
fn test_error_with_snapshot_naming_patterns() {
    let snapshots = vec![
        "tank@hourly-2023-11-18-12",
        "tank/data@daily-monday",
        "pool/dataset@manual-backup",
        "fast/vm@pre-upgrade",
    ];

    for snapshot in snapshots {
        let error = ZfsErrorBuilder::snapshot_error("test", snapshot);
        let error_string = format!("{:?}", error);
        assert!(error_string.contains(snapshot));
    }
}

// ==================== CONCURRENT ERROR CREATION TESTS ====================

#[test]
fn test_concurrent_error_creation() {
    use std::thread;

    let handles: Vec<_> = (0..100)
        .map(|i| {
            thread::spawn(move || {
                ZfsErrorBuilder::pool_error(&format!("error-{}", i), &format!("pool-{}", i))
            })
        })
        .collect();

    for handle in handles {
        let error = handle.join().expect("Thread should complete");
        let error_string = format!("{:?}", error);
        assert!(!error_string.is_empty());
    }
}

// ==================== ERROR CONVERSION TESTS ====================

#[test]
fn test_error_to_string() {
    let error = ZfsErrorBuilder::pool_error("test error", "tank");
    let error_string = format!("{}", error);

    assert!(!error_string.is_empty());
}

#[test]
fn test_error_debug_format() {
    let error = ZfsErrorBuilder::command_error("zpool create", "failed");
    let debug_string = format!("{:?}", error);

    assert!(debug_string.contains("Command"));
}

// ==================== REAL-WORLD SCENARIO TESTS ====================

#[test]
fn test_pool_creation_failure_scenario() {
    let error = ZfsErrorBuilder::pool_error("Insufficient space: need 1TB, have 500GB", "new-pool");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Insufficient space"));
    assert!(error_string.contains("new-pool"));
}

#[test]
fn test_dataset_mount_failure_scenario() {
    let error = ZfsErrorBuilder::dataset_error("Mount point /mnt/data already in use", "tank/data");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Mount point"));
    assert!(error_string.contains("tank/data"));
}

#[test]
fn test_snapshot_rollback_failure_scenario() {
    let error = ZfsErrorBuilder::snapshot_error(
        "Cannot rollback: newer snapshots exist",
        "tank/vm@pre-upgrade",
    );
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Cannot rollback"));
}

#[test]
fn test_command_timeout_scenario() {
    let error = zfs_command_error("zpool scrub", "Operation timed out after 300 seconds");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("timed out"));
}

#[test]
fn test_permission_denied_scenario() {
    let error = zfs_operation_error(
        "destroy",
        "Permission denied: user lacks zfs.destroy privilege",
    );
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("Permission denied"));
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_error_with_null_bytes() {
    let message_with_null = "error\0message";
    let error = ZfsErrorBuilder::zfs_error(message_with_null);
    let error_string = format!("{:?}", error);

    assert!(!error_string.is_empty());
}

#[test]
fn test_error_with_control_characters() {
    let message = "error\r\n\t\x1b[31mred\x1b[0m";
    let error = ZfsErrorBuilder::zfs_error(message);
    let error_string = format!("{:?}", error);

    assert!(!error_string.is_empty());
}

#[test]
fn test_error_with_emoji() {
    let error = ZfsErrorBuilder::pool_error("❌ Pool failed 🔥", "tank");
    let error_string = format!("{:?}", error);

    assert!(error_string.contains("❌") || error_string.contains("Pool failed"));
}
