// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive Tests for ZFS Command Execution
//!
//! Tests for ZfsCommand struct and command execution patterns

#[cfg(test)]
mod tests {
    use crate::command::ZfsCommand;

    /// Test 1: ZfsCommand default creation
    #[test]
    fn test_zfs_command_default() {
        let cmd = ZfsCommand::default();
        assert!(!cmd.dry_run);
        assert_eq!(cmd.timeout_seconds, 30);
    }

    /// Test 2: ZfsCommand new() creation
    #[test]
    fn test_zfs_command_new() {
        let cmd = ZfsCommand::new();
        assert!(!cmd.dry_run);
        assert_eq!(cmd.timeout_seconds, 30);
    }

    /// Test 3: ZfsCommand with dry_run enabled
    #[test]
    fn test_zfs_command_with_dry_run() {
        let cmd = ZfsCommand::new().with_dry_run(true);
        assert!(cmd.dry_run);
        assert_eq!(cmd.timeout_seconds, 30);
    }

    /// Test 4: ZfsCommand with dry_run disabled
    #[test]
    fn test_zfs_command_without_dry_run() {
        let cmd = ZfsCommand::new().with_dry_run(false);
        assert!(!cmd.dry_run);
    }

    /// Test 5: ZfsCommand with custom timeout
    #[test]
    fn test_zfs_command_with_custom_timeout() {
        let cmd = ZfsCommand::new().with_timeout(60);
        assert_eq!(cmd.timeout_seconds, 60);
        assert!(!cmd.dry_run);
    }

    /// Test 6: ZfsCommand with zero timeout
    #[test]
    fn test_zfs_command_with_zero_timeout() {
        let cmd = ZfsCommand::new().with_timeout(0);
        assert_eq!(cmd.timeout_seconds, 0);
    }

    /// Test 7: ZfsCommand with maximum timeout
    #[test]
    fn test_zfs_command_with_max_timeout() {
        let cmd = ZfsCommand::new().with_timeout(u64::MAX);
        assert_eq!(cmd.timeout_seconds, u64::MAX);
    }

    /// Test 8: ZfsCommand chaining - dry_run then timeout
    #[test]
    fn test_zfs_command_chaining_dry_run_first() {
        let cmd = ZfsCommand::new().with_dry_run(true).with_timeout(120);

        assert!(cmd.dry_run);
        assert_eq!(cmd.timeout_seconds, 120);
    }

    /// Test 9: ZfsCommand chaining - timeout then dry_run
    #[test]
    fn test_zfs_command_chaining_timeout_first() {
        let cmd = ZfsCommand::new().with_timeout(90).with_dry_run(true);

        assert!(cmd.dry_run);
        assert_eq!(cmd.timeout_seconds, 90);
    }

    /// Test 10: ZfsCommand multiple dry_run toggles
    #[test]
    fn test_zfs_command_multiple_dry_run_toggles() {
        let cmd = ZfsCommand::new()
            .with_dry_run(true)
            .with_dry_run(false)
            .with_dry_run(true);

        assert!(cmd.dry_run, "Final dry_run should be true");
    }

    /// Test 11: ZfsCommand multiple timeout changes
    #[test]
    fn test_zfs_command_multiple_timeout_changes() {
        let cmd = ZfsCommand::new()
            .with_timeout(10)
            .with_timeout(20)
            .with_timeout(30);

        assert_eq!(cmd.timeout_seconds, 30, "Final timeout should be 30");
    }

    /// Test 12: ZfsCommand clone
    #[test]
    fn test_zfs_command_clone() {
        let cmd = ZfsCommand::new().with_dry_run(true).with_timeout(45);

        let cloned = cmd.clone();
        assert_eq!(cloned.dry_run, cmd.dry_run);
        assert_eq!(cloned.timeout_seconds, cmd.timeout_seconds);
    }

    /// Test 13: ZfsCommand debug representation
    #[test]
    fn test_zfs_command_debug() {
        let cmd = ZfsCommand::new().with_dry_run(true);
        let debug_str = format!("{:?}", cmd);

        assert!(debug_str.contains("ZfsCommand"));
        assert!(debug_str.contains("dry_run"));
    }

    /// Test 14: Multiple ZfsCommand instances independence
    #[test]
    fn test_multiple_zfs_commands_independence() {
        let cmd1 = ZfsCommand::new().with_dry_run(true);
        let cmd2 = ZfsCommand::new().with_dry_run(false);

        assert!(cmd1.dry_run);
        assert!(!cmd2.dry_run);
    }

    /// Test 15: ZfsCommand timeout edge values
    #[test]
    fn test_zfs_command_timeout_edge_values() {
        let cmd_min = ZfsCommand::new().with_timeout(1);
        let cmd_max = ZfsCommand::new().with_timeout(u64::MAX);

        assert_eq!(cmd_min.timeout_seconds, 1);
        assert_eq!(cmd_max.timeout_seconds, u64::MAX);
    }

    /// Test 16: ZfsCommand builder pattern fluency
    #[test]
    fn test_zfs_command_builder_fluency() {
        let cmd = ZfsCommand::new()
            .with_timeout(60)
            .with_dry_run(true)
            .with_timeout(120)
            .with_dry_run(false)
            .with_timeout(180);

        assert!(!cmd.dry_run);
        assert_eq!(cmd.timeout_seconds, 180);
    }

    /// Test 17: ZfsCommand default vs explicit
    #[test]
    fn test_zfs_command_default_vs_explicit() {
        let default = ZfsCommand::default();
        let explicit = ZfsCommand::new().with_dry_run(false).with_timeout(30);

        assert_eq!(default.dry_run, explicit.dry_run);
        assert_eq!(default.timeout_seconds, explicit.timeout_seconds);
    }

    /// Test 18: ZfsCommand struct size
    #[test]
    fn test_zfs_command_struct_size() {
        use std::mem::size_of;

        let size = size_of::<ZfsCommand>();
        // Should be small (bool + u64 = ~16 bytes with alignment)
        assert!(size < 32, "ZfsCommand should be small");
    }

    /// Test 19: ZfsCommand check_zfs_available static method
    #[test]
    fn test_zfs_check_available_static() {
        // This will succeed or fail based on system ZFS installation
        // Test that the method doesn't panic
        let _result = ZfsCommand::check_zfs_available();
        // Just verify the method is callable without panicking
    }

    /// Test 20: ZfsCommand configuration combinations
    #[test]
    fn test_zfs_command_config_combinations() {
        let configs = vec![
            (false, 30),
            (true, 30),
            (false, 60),
            (true, 60),
            (false, 0),
            (true, u64::MAX),
        ];

        for (dry_run, timeout) in configs {
            let cmd = ZfsCommand::new()
                .with_dry_run(dry_run)
                .with_timeout(timeout);

            assert_eq!(cmd.dry_run, dry_run);
            assert_eq!(cmd.timeout_seconds, timeout);
        }
    }
}

#[cfg(test)]
mod command_async_tests {
    use crate::command::ZfsCommand;

    /// Async Test 1: Dry run zpool command
    #[tokio::test]
    async fn test_zpool_dry_run() {
        let cmd = ZfsCommand::new().with_dry_run(true);
        let result = cmd.zpool(&["list"]).await;

        assert!(result.is_ok(), "Dry run should always succeed");
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.stdout.contains("DRY RUN"));
    }

    /// Async Test 2: Dry run zfs command
    #[tokio::test]
    async fn test_zfs_dry_run() {
        let cmd = ZfsCommand::new().with_dry_run(true);
        let result = cmd.zfs(&["list"]).await;

        assert!(result.is_ok(), "Dry run should always succeed");
        let output = result.unwrap();
        assert!(output.success);
        assert!(output.stdout.contains("DRY RUN"));
    }

    /// Async Test 3: Multiple dry run commands
    #[tokio::test]
    async fn test_multiple_dry_run_commands() {
        let cmd = ZfsCommand::new().with_dry_run(true);

        let result1 = cmd.zpool(&["status"]).await;
        let result2 = cmd.zfs(&["list"]).await;
        let result3 = cmd.zpool(&["list"]).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
    }

    /// Async Test 4: Dry run with various arguments
    #[tokio::test]
    async fn test_dry_run_various_args() {
        let cmd = ZfsCommand::new().with_dry_run(true);

        let args_sets = vec![
            vec!["list"],
            vec!["list", "-H"],
            vec!["get", "all"],
            vec!["status", "-v"],
        ];

        for args in args_sets {
            let result = cmd.zpool(&args).await;
            assert!(result.is_ok());
        }
    }

    /// Async Test 5: Concurrent dry run commands
    #[tokio::test]
    async fn test_concurrent_dry_run_commands() {
        let cmd = ZfsCommand::new().with_dry_run(true);

        let mut handles = vec![];
        for _ in 0..10 {
            let cmd_clone = cmd.clone();
            let handle = tokio::spawn(async move { cmd_clone.zpool(&["list"]).await });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.expect("Task should complete");
            assert!(result.is_ok());
        }
    }
}
