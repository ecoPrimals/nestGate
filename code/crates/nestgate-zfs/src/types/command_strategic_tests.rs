// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Strategic tests for ZFS command types
//!
//! Boosts coverage for types/command.rs from 0% to 90%+

#[cfg(test)]
mod command_strategic_tests {
    use crate::types::command::{CommandResult, ZfsCommand};
    use std::collections::HashMap;

    #[test]
    fn test_command_result_success() {
        let result = CommandResult::success("operation completed".to_string());

        assert!(result.is_success());
        assert!(!result.is_failure());
        assert_eq!(result.stdout, "operation completed");
        assert_eq!(result.stderr, "");
        assert_eq!(result.exit_code, Some(0));
    }

    #[test]
    fn test_command_result_failure() {
        let result = CommandResult::failure("error occurred".to_string(), Some(1));

        assert!(!result.is_success());
        assert!(result.is_failure());
        assert_eq!(result.stdout, "");
        assert_eq!(result.stderr, "error occurred");
        assert_eq!(result.exit_code, Some(1));
    }

    #[test]
    fn test_command_result_failure_no_exit_code() {
        let result = CommandResult::failure("unknown error".to_string(), None);

        assert!(result.is_failure());
        assert_eq!(result.exit_code, None);
    }

    #[test]
    fn test_zfs_command_create_pool() {
        let cmd = ZfsCommand::CreatePool {
            name: "tank".to_string(),
            devices: vec!["/dev/sda".to_string(), "/dev/sdb".to_string()],
        };

        match cmd {
            ZfsCommand::CreatePool { name, devices } => {
                assert_eq!(name, "tank");
                assert_eq!(devices.len(), 2);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_create_dataset() {
        let mut props = HashMap::new();
        props.insert("compression".to_string(), "lz4".to_string());

        let cmd = ZfsCommand::CreateDataset {
            name: "tank/data".to_string(),
            properties: props,
        };

        match cmd {
            ZfsCommand::CreateDataset { name, properties } => {
                assert_eq!(name, "tank/data");
                assert_eq!(properties.get("compression"), Some(&"lz4".to_string()));
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_create_snapshot() {
        let cmd = ZfsCommand::CreateSnapshot {
            dataset: "tank/data".to_string(),
            snapshot: "backup".to_string(),
            recursive: true,
        };

        match cmd {
            ZfsCommand::CreateSnapshot {
                dataset,
                snapshot,
                recursive,
            } => {
                assert_eq!(dataset, "tank/data");
                assert_eq!(snapshot, "backup");
                assert!(recursive);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_list_pools() {
        let cmd = ZfsCommand::ListPools;

        match cmd {
            ZfsCommand::ListPools => {
                // Valid variant
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_list_datasets_all() {
        let cmd = ZfsCommand::ListDatasets { pool: None };

        match cmd {
            ZfsCommand::ListDatasets { pool } => {
                assert!(pool.is_none());
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_list_datasets_specific() {
        let cmd = ZfsCommand::ListDatasets {
            pool: Some("tank".to_string()),
        };

        match cmd {
            ZfsCommand::ListDatasets { pool } => {
                assert_eq!(pool, Some("tank".to_string()));
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_get_pool_status() {
        let cmd = ZfsCommand::GetPoolStatus {
            pool: "tank".to_string(),
        };

        match cmd {
            ZfsCommand::GetPoolStatus { pool } => {
                assert_eq!(pool, "tank");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_set_property() {
        let cmd = ZfsCommand::SetProperty {
            dataset: "tank/data".to_string(),
            property: "compression".to_string(),
            value: "lz4".to_string(),
        };

        match cmd {
            ZfsCommand::SetProperty {
                dataset,
                property,
                value,
            } => {
                assert_eq!(dataset, "tank/data");
                assert_eq!(property, "compression");
                assert_eq!(value, "lz4");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_destroy_normal() {
        let cmd = ZfsCommand::Destroy {
            target: "tank/data@snap1".to_string(),
            force: false,
        };

        match cmd {
            ZfsCommand::Destroy { target, force } => {
                assert_eq!(target, "tank/data@snap1");
                assert!(!force);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_command_destroy_forced() {
        let cmd = ZfsCommand::Destroy {
            target: "tank/data".to_string(),
            force: true,
        };

        match cmd {
            ZfsCommand::Destroy { target, force } => {
                assert_eq!(target, "tank/data");
                assert!(force);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_command_result_clone() {
        let result = CommandResult::success("test".to_string());
        let cloned = result.clone();

        assert_eq!(result.success, cloned.success);
        assert_eq!(result.stdout, cloned.stdout);
    }

    #[test]
    fn test_zfs_command_clone() {
        let cmd = ZfsCommand::ListPools;
        let cloned = cmd.clone();

        match (cmd, cloned) {
            (ZfsCommand::ListPools, ZfsCommand::ListPools) => {
                // Both are ListPools variant
            }
            _ => panic!("Clone failed"),
        }
    }

    #[test]
    fn test_command_result_debug() {
        let result = CommandResult::success("test output".to_string());
        let debug_str = format!("{:?}", result);

        assert!(debug_str.contains("CommandResult"));
        assert!(debug_str.contains("success"));
    }

    #[test]
    fn test_zfs_command_debug() {
        let cmd = ZfsCommand::ListPools;
        let debug_str = format!("{:?}", cmd);

        assert!(debug_str.contains("ListPools"));
    }
}
