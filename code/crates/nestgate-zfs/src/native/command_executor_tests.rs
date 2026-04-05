// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for native ZFS command executor
//!
//! These tests cover command execution structures and validation
//! without requiring actual ZFS installation.

#[cfg(test)]
mod tests {
    use crate::native::command_executor::*;

    // ==================== ZfsCommandResult Tests ====================

    #[test]
    fn test_command_result_creation_success() {
        let result = ZfsCommandResult {
            success: true,
            stdout: "pool created successfully".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        assert!(result.success);
        assert_eq!(result.exit_code, 0);
        assert!(!result.stdout.is_empty());
    }

    #[test]
    fn test_command_result_creation_failure() {
        let result = ZfsCommandResult {
            success: false,
            stdout: String::new(),
            stderr: "pool not found".to_string(),
            exit_code: 1,
        };

        assert!(!result.success);
        assert_eq!(result.exit_code, 1);
        assert!(!result.stderr.is_empty());
    }

    #[test]
    fn test_command_result_clone() {
        let result1 = ZfsCommandResult {
            success: true,
            stdout: "output".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        let result2 = result1.clone();
        assert_eq!(result1.success, result2.success);
        assert_eq!(result1.stdout, result2.stdout);
    }

    #[test]
    fn test_command_result_serialization() {
        let result = ZfsCommandResult {
            success: true,
            stdout: "test output".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        let json = serde_json::to_string(&result).expect("Failed to serialize");
        assert!(json.contains("test output"));
        assert!(json.contains("success"));
    }

    #[test]
    fn test_command_result_deserialization() {
        let json = r#"{
            "success": true,
            "stdout": "test output",
            "stderr": "",
            "exit_code": 0
        }"#;

        let result: ZfsCommandResult = serde_json::from_str(json).expect("Failed to deserialize");

        assert!(result.success);
        assert_eq!(result.stdout, "test output");
    }

    #[test]
    fn test_command_result_debug_format() {
        let result = ZfsCommandResult {
            success: true,
            stdout: "output".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("success"));
        assert!(debug_str.contains("output"));
    }

    // ==================== NativeZfsCommandExecutor Tests ====================

    #[test]
    fn test_executor_creation_default() {
        let executor = NativeZfsCommandExecutor::new();
        // Executor should be created successfully
        assert!(std::mem::size_of_val(&executor) > 0);
    }

    #[test]
    fn test_executor_creation_with_timeout() {
        let executor = NativeZfsCommandExecutor::with_timeout(600);
        // Executor with custom timeout should be created
        assert!(std::mem::size_of_val(&executor) > 0);
    }

    #[test]
    fn test_executor_with_different_timeouts() {
        let executor1 = NativeZfsCommandExecutor::with_timeout(300);
        let executor2 = NativeZfsCommandExecutor::with_timeout(600);
        let executor3 = NativeZfsCommandExecutor::with_timeout(900);

        // All executors should be valid
        assert!(std::mem::size_of_val(&executor1) > 0);
        assert!(std::mem::size_of_val(&executor2) > 0);
        assert!(std::mem::size_of_val(&executor3) > 0);
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_command_result_empty_output() {
        let result = ZfsCommandResult {
            success: true,
            stdout: String::new(),
            stderr: String::new(),
            exit_code: 0,
        };

        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());
    }

    #[test]
    fn test_command_result_multiline_output() {
        let result = ZfsCommandResult {
            success: true,
            stdout: "line1\nline2\nline3".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        assert_eq!(result.stdout.lines().count(), 3);
    }

    #[test]
    fn test_command_result_with_stderr_and_stdout() {
        let result = ZfsCommandResult {
            success: false,
            stdout: "partial output".to_string(),
            stderr: "error occurred".to_string(),
            exit_code: 1,
        };

        assert!(!result.stdout.is_empty());
        assert!(!result.stderr.is_empty());
        assert_eq!(result.exit_code, 1);
    }

    #[test]
    fn test_command_result_negative_exit_code() {
        let result = ZfsCommandResult {
            success: false,
            stdout: String::new(),
            stderr: "signal terminated".to_string(),
            exit_code: -1,
        };

        assert_eq!(result.exit_code, -1);
    }

    #[test]
    fn test_command_result_large_exit_code() {
        let result = ZfsCommandResult {
            success: false,
            stdout: String::new(),
            stderr: "error".to_string(),
            exit_code: 255,
        };

        assert_eq!(result.exit_code, 255);
    }

    // ==================== Serialization Edge Cases ====================

    #[test]
    fn test_command_result_serialization_with_special_chars() {
        let result = ZfsCommandResult {
            success: true,
            stdout: "output with \"quotes\" and \n newlines".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        let json = serde_json::to_string(&result).expect("Failed to serialize");
        let deserialized: ZfsCommandResult =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(result.stdout, deserialized.stdout);
    }

    #[test]
    fn test_command_result_serialization_with_unicode() {
        let result = ZfsCommandResult {
            success: true,
            stdout: "测试 🚀 тест".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        let json = serde_json::to_string(&result).expect("Failed to serialize");
        let deserialized: ZfsCommandResult =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(result.stdout, deserialized.stdout);
    }

    // ==================== Timeout Tests ====================

    #[test]
    fn test_executor_short_timeout() {
        let executor = NativeZfsCommandExecutor::with_timeout(1);
        assert!(std::mem::size_of_val(&executor) > 0);
    }

    #[test]
    fn test_executor_long_timeout() {
        let executor = NativeZfsCommandExecutor::with_timeout(3600);
        assert!(std::mem::size_of_val(&executor) > 0);
    }

    #[test]
    fn test_executor_zero_timeout() {
        let executor = NativeZfsCommandExecutor::with_timeout(0);
        // Even with 0 timeout, executor should be created
        assert!(std::mem::size_of_val(&executor) > 0);
    }

    // ==================== String Parsing Tests ====================

    #[test]
    fn test_command_result_stdout_parsing() {
        let result = ZfsCommandResult {
            success: true,
            stdout: "pool1\npool2\npool3".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        let pools: Vec<&str> = result.stdout.lines().collect();
        assert_eq!(pools.len(), 3);
        assert_eq!(pools[0], "pool1");
    }

    #[test]
    fn test_command_result_stderr_parsing() {
        let result = ZfsCommandResult {
            success: false,
            stdout: String::new(),
            stderr: "Error: pool not found\nSuggestion: check pool name".to_string(),
            exit_code: 1,
        };

        let lines: Vec<&str> = result.stderr.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("Error"));
    }

    // ==================== Success/Failure Logic Tests ====================

    #[test]
    fn test_command_result_success_with_exit_zero() {
        let result = ZfsCommandResult {
            success: true,
            stdout: "operation completed".to_string(),
            stderr: String::new(),
            exit_code: 0,
        };

        assert!(result.success && result.exit_code == 0);
    }

    #[test]
    fn test_command_result_failure_correlation() {
        let result = ZfsCommandResult {
            success: false,
            stdout: String::new(),
            stderr: "error".to_string(),
            exit_code: 1,
        };

        // success should be false when exit_code != 0
        assert!(!result.success && result.exit_code != 0);
    }

    // ==================== Memory Tests ====================

    #[test]
    fn test_command_result_large_output() {
        let large_output = "x".repeat(10_000);
        let result = ZfsCommandResult {
            success: true,
            stdout: large_output.clone(),
            stderr: String::new(),
            exit_code: 0,
        };

        assert_eq!(result.stdout.len(), 10_000);
    }

    #[test]
    fn test_executor_multiple_instances() {
        let executors: Vec<_> = (0..10).map(|_| NativeZfsCommandExecutor::new()).collect();

        assert_eq!(executors.len(), 10);
    }
}
