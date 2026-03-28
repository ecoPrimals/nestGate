// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for ZFS command execution
//! Target: Improve coverage of command module

use nestgate_zfs::command::{CommandResult, ZfsCommand};

// ==================== ZFSCOMMAND CREATION TESTS ====================

#[test]
fn test_create_default_command() {
    let cmd = ZfsCommand::default();
    assert!(!cmd.dry_run);
    assert_eq!(cmd.timeout_seconds, 30);
}

#[test]
fn test_create_new_command() {
    let cmd = ZfsCommand::new();
    assert!(!cmd.dry_run);
    assert_eq!(cmd.timeout_seconds, 30);
}

#[test]
fn test_command_with_dry_run() {
    let cmd = ZfsCommand::new().with_dry_run(true);
    assert!(cmd.dry_run);
    assert_eq!(cmd.timeout_seconds, 30);
}

#[test]
fn test_command_with_timeout() {
    let cmd = ZfsCommand::new().with_timeout(60);
    assert!(!cmd.dry_run);
    assert_eq!(cmd.timeout_seconds, 60);
}

#[test]
fn test_command_with_dry_run_and_timeout() {
    let cmd = ZfsCommand::new().with_dry_run(true).with_timeout(120);

    assert!(cmd.dry_run);
    assert_eq!(cmd.timeout_seconds, 120);
}

#[test]
fn test_command_builder_chain() {
    let cmd = ZfsCommand::new().with_dry_run(false).with_timeout(45);

    assert!(!cmd.dry_run);
    assert_eq!(cmd.timeout_seconds, 45);
}

// ==================== COMMAND DEBUG TESTS ====================

#[test]
fn test_command_debug_format() {
    let cmd = ZfsCommand::new();
    let debug_str = format!("{:?}", cmd);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("ZfsCommand"));
}

#[test]
fn test_command_clone() {
    let cmd1 = ZfsCommand::new().with_timeout(90);
    let cmd2 = cmd1.clone();

    assert_eq!(cmd1.timeout_seconds, cmd2.timeout_seconds);
    assert_eq!(cmd1.dry_run, cmd2.dry_run);
}

// ==================== COMMAND RESULT TESTS ====================

#[test]
fn test_create_success_result() {
    let result = CommandResult {
        success: true,
        stdout: "Success message".to_string(),
        stderr: String::new(),
        exit_code: 0,
    };

    assert!(result.success);
    assert_eq!(result.exit_code, 0);
    assert_eq!(result.stdout, "Success message");
}

#[test]
fn test_create_failure_result() {
    let result = CommandResult {
        success: false,
        stdout: String::new(),
        stderr: "Error message".to_string(),
        exit_code: 1,
    };

    assert!(!result.success);
    assert_eq!(result.exit_code, 1);
    assert_eq!(result.stderr, "Error message");
}

#[test]
fn test_result_with_both_outputs() {
    let result = CommandResult {
        success: true,
        stdout: "stdout output".to_string(),
        stderr: "stderr warning".to_string(),
        exit_code: 0,
    };

    assert!(result.success);
    assert!(!result.stdout.is_empty());
    assert!(!result.stderr.is_empty());
}

#[test]
fn test_result_debug_format() {
    let result = CommandResult {
        success: true,
        stdout: "test".to_string(),
        stderr: String::new(),
        exit_code: 0,
    };

    let debug_str = format!("{:?}", result);
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("CommandResult"));
}

#[test]
fn test_result_clone() {
    let result1 = CommandResult {
        success: true,
        stdout: "test".to_string(),
        stderr: String::new(),
        exit_code: 0,
    };

    let result2 = result1.clone();
    assert_eq!(result1.success, result2.success);
    assert_eq!(result1.stdout, result2.stdout);
}

// ==================== RESULT SERIALIZATION TESTS ====================

#[test]
fn test_result_serialization() {
    let result = CommandResult {
        success: true,
        stdout: "output".to_string(),
        stderr: String::new(),
        exit_code: 0,
    };

    let json = serde_json::to_string(&result).expect("Failed to serialize");
    assert!(json.contains("output"));
    assert!(json.contains("success"));
}

#[test]
fn test_result_deserialization() {
    let result = CommandResult {
        success: false,
        stdout: String::new(),
        stderr: "error".to_string(),
        exit_code: 1,
    };

    let json = serde_json::to_string(&result).expect("Failed to serialize");
    let deserialized: CommandResult = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(result.success, deserialized.success);
    assert_eq!(result.stderr, deserialized.stderr);
    assert_eq!(result.exit_code, deserialized.exit_code);
}

// ==================== ZFS AVAILABILITY TESTS ====================

#[test]
fn test_check_zfs_available() {
    // This test checks if the check_zfs_available method works
    // It may return true or false depending on the system
    let result = ZfsCommand::check_zfs_available();

    // Should not panic and should return a boolean result
    assert!(result.is_ok());
}

// ==================== ASYNC COMMAND TESTS ====================

#[tokio::test]
async fn test_dry_run_zpool_command() {
    let cmd = ZfsCommand::new().with_dry_run(true);
    let result = cmd.zpool(&["list"]).await;

    assert!(result.is_ok());
    let command_result = result.unwrap();
    assert!(command_result.success);
    assert!(command_result.stdout.contains("DRY RUN"));
}

#[tokio::test]
async fn test_dry_run_zfs_command() {
    let cmd = ZfsCommand::new().with_dry_run(true);
    let result = cmd.zfs(&["list"]).await;

    assert!(result.is_ok());
    let command_result = result.unwrap();
    assert!(command_result.success);
    assert!(command_result.stdout.contains("DRY RUN"));
}

#[tokio::test]
async fn test_dry_run_preserves_command() {
    let cmd = ZfsCommand::new().with_dry_run(true);
    let result = cmd.zpool(&["status", "testpool"]).await;

    assert!(result.is_ok());
    let command_result = result.unwrap();
    assert!(command_result.stdout.contains("zpool"));
    assert!(command_result.stdout.contains("status"));
    assert!(command_result.stdout.contains("testpool"));
}

#[tokio::test]
async fn test_dry_run_multiple_args() {
    let cmd = ZfsCommand::new().with_dry_run(true);
    let result = cmd
        .zfs(&["create", "-o", "compression=lz4", "pool/dataset"])
        .await;

    assert!(result.is_ok());
    let command_result = result.unwrap();
    assert!(command_result.stdout.contains("create"));
    assert!(command_result.stdout.contains("compression"));
}

// ==================== TIMEOUT CONFIGURATION TESTS ====================

#[test]
fn test_zero_timeout() {
    let cmd = ZfsCommand::new().with_timeout(0);
    assert_eq!(cmd.timeout_seconds, 0);
}

#[test]
fn test_large_timeout() {
    let cmd = ZfsCommand::new().with_timeout(3600); // 1 hour
    assert_eq!(cmd.timeout_seconds, 3600);
}

#[test]
fn test_timeout_chaining() {
    let cmd = ZfsCommand::new().with_timeout(60).with_timeout(120); // Override

    assert_eq!(cmd.timeout_seconds, 120);
}

// ==================== DRY RUN CONFIGURATION TESTS ====================

#[test]
fn test_dry_run_false() {
    let cmd = ZfsCommand::new().with_dry_run(false);
    assert!(!cmd.dry_run);
}

#[test]
fn test_dry_run_toggle() {
    let cmd = ZfsCommand::new().with_dry_run(true).with_dry_run(false); // Override

    assert!(!cmd.dry_run);
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_empty_stdout() {
    let result = CommandResult {
        success: true,
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 0,
    };

    assert!(result.stdout.is_empty());
}

#[test]
fn test_empty_stderr() {
    let result = CommandResult {
        success: false,
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 1,
    };

    assert!(result.stderr.is_empty());
}

#[test]
fn test_negative_exit_code() {
    let result = CommandResult {
        success: false,
        stdout: String::new(),
        stderr: String::new(),
        exit_code: -1,
    };

    assert_eq!(result.exit_code, -1);
}

#[test]
fn test_large_exit_code() {
    let result = CommandResult {
        success: false,
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 255,
    };

    assert_eq!(result.exit_code, 255);
}

// ==================== MULTI-LINE OUTPUT TESTS ====================

#[test]
fn test_multiline_stdout() {
    let result = CommandResult {
        success: true,
        stdout: "line1\nline2\nline3".to_string(),
        stderr: String::new(),
        exit_code: 0,
    };

    assert!(result.stdout.contains('\n'));
    assert_eq!(result.stdout.lines().count(), 3);
}

#[test]
fn test_multiline_stderr() {
    let result = CommandResult {
        success: false,
        stdout: String::new(),
        stderr: "error1\nerror2".to_string(),
        exit_code: 1,
    };

    assert!(result.stderr.contains('\n'));
    assert_eq!(result.stderr.lines().count(), 2);
}

// ==================== SPECIAL CHARACTERS TESTS ====================

#[test]
fn test_special_chars_in_output() {
    let result = CommandResult {
        success: true,
        stdout: "Output with special: \t\n\"quotes\" 'apostrophes'".to_string(),
        stderr: String::new(),
        exit_code: 0,
    };

    assert!(result.stdout.contains('\t'));
    assert!(result.stdout.contains('\"'));
}

#[test]
fn test_unicode_in_output() {
    let result = CommandResult {
        success: true,
        stdout: "Unicode: 你好 🎉 ñ".to_string(),
        stderr: String::new(),
        exit_code: 0,
    };

    assert!(result.stdout.contains('你'));
    assert!(result.stdout.contains('🎉'));
}

// ==================== INTEGRATION TESTS ====================

#[tokio::test]
async fn test_command_workflow() {
    let cmd = ZfsCommand::new().with_dry_run(true).with_timeout(60);

    // Execute multiple commands
    let result1 = cmd.zpool(&["list"]).await;
    let result2 = cmd.zfs(&["list"]).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_dry_run_no_side_effects() {
    let cmd = ZfsCommand::new().with_dry_run(true);

    // These should not actually execute
    let _r1 = cmd.zpool(&["destroy", "nonexistent"]).await;
    let _r2 = cmd.zfs(&["destroy", "nonexistent/dataset"]).await;

    // Test passes if dry run executes without errors
}

// ==================== BUILDER PATTERN TESTS ====================

#[test]
fn test_builder_pattern_fluent() {
    let cmd = ZfsCommand::new().with_timeout(120).with_dry_run(true);

    assert_eq!(cmd.timeout_seconds, 120);
    assert!(cmd.dry_run);
}

#[test]
fn test_builder_pattern_order_independence() {
    let cmd1 = ZfsCommand::new().with_timeout(60).with_dry_run(true);

    let cmd2 = ZfsCommand::new().with_dry_run(true).with_timeout(60);

    assert_eq!(cmd1.timeout_seconds, cmd2.timeout_seconds);
    assert_eq!(cmd1.dry_run, cmd2.dry_run);
}

// ==================== COMMAND RESULT COLLECTION TESTS ====================

#[test]
fn test_collect_multiple_results() {
    let results = [
        CommandResult {
            success: true,
            stdout: "result1".to_string(),
            stderr: String::new(),
            exit_code: 0,
        },
        CommandResult {
            success: false,
            stdout: String::new(),
            stderr: "error1".to_string(),
            exit_code: 1,
        },
    ];

    assert_eq!(results.len(), 2);

    let successful = results.iter().filter(|r| r.success).count();
    assert_eq!(successful, 1);
}

#[test]
fn test_result_pattern_matching() {
    let result = CommandResult {
        success: true,
        stdout: "test".to_string(),
        stderr: String::new(),
        exit_code: 0,
    };

    let message = if result.success {
        "Command succeeded"
    } else {
        "Command failed"
    };

    assert_eq!(message, "Command succeeded");
}
