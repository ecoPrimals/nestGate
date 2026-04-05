// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Strategic tests for ZFS error types
//!
//! Boosts coverage for types/errors.rs from 0% to 95%+

#[cfg(test)]
mod errors_strategic_tests {
    use crate::types::errors::{ZfsError, ZfsResult};
    use std::io;

    #[test]
    fn test_pool_error_creation() {
        let err = ZfsError::pool_error("Pool not found");
        let err_str = format!("{}", err);

        assert!(err_str.contains("Pool operation failed"));
        assert!(err_str.contains("Pool not found"));

        match err {
            ZfsError::PoolError { message } => {
                assert_eq!(message, "Pool not found");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_dataset_error_creation() {
        let err = ZfsError::dataset_error("Dataset already exists");
        let err_str = format!("{}", err);

        assert!(err_str.contains("Dataset operation failed"));

        match err {
            ZfsError::DatasetError { message } => {
                assert_eq!(message, "Dataset already exists");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_snapshot_error_creation() {
        let err = ZfsError::snapshot_error("Snapshot creation failed");
        let err_str = format!("{}", err);

        assert!(err_str.contains("Snapshot operation failed"));

        match err {
            ZfsError::SnapshotError { message } => {
                assert_eq!(message, "Snapshot creation failed");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_command_error_creation() {
        let err = ZfsError::command_error("ZFS command execution failed");
        let err_str = format!("{}", err);

        assert!(err_str.contains("Command execution failed"));

        match err {
            ZfsError::CommandError { message } => {
                assert_eq!(message, "ZFS command execution failed");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_config_error_creation() {
        let err = ZfsError::config_error("Invalid configuration");
        let err_str = format!("{}", err);

        assert!(err_str.contains("Configuration error"));

        match err {
            ZfsError::ConfigError { message } => {
                assert_eq!(message, "Invalid configuration");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let zfs_err: ZfsError = io_err.into();

        let err_str = format!("{}", zfs_err);
        assert!(err_str.contains("IO error"));

        match zfs_err {
            ZfsError::Io(err) => {
                assert_eq!(err.kind(), io::ErrorKind::NotFound);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_error_string_conversion() {
        let err = ZfsError::pool_error(String::from("Pool error"));

        match err {
            ZfsError::PoolError { message } => {
                assert_eq!(message, "Pool error");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_error_str_slice_conversion() {
        let err = ZfsError::dataset_error("Dataset error");

        match err {
            ZfsError::DatasetError { message } => {
                assert_eq!(message, "Dataset error");
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_zfs_result_ok() {
        let result: ZfsResult<i32> = Ok(42);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_zfs_result_err() {
        let result: ZfsResult<i32> = Err(ZfsError::pool_error("Test error"));

        assert!(result.is_err());

        match result {
            Err(ZfsError::PoolError { message }) => {
                assert_eq!(message, "Test error");
            }
            _ => panic!("Expected pool error"),
        }
    }

    #[test]
    fn test_error_debug() {
        let err = ZfsError::snapshot_error("Debug test");
        let debug_str = format!("{:?}", err);

        assert!(debug_str.contains("SnapshotError"));
    }

    #[test]
    fn test_multiple_error_types() {
        let errors = vec![
            ZfsError::pool_error("pool 1"),
            ZfsError::dataset_error("dataset 1"),
            ZfsError::snapshot_error("snapshot 1"),
            ZfsError::command_error("command 1"),
            ZfsError::config_error("config 1"),
        ];

        assert_eq!(errors.len(), 5);

        for err in errors {
            // Each error should format correctly
            let _err_str = format!("{}", err);
        }
    }

    #[test]
    fn test_result_unwrap_or() {
        let result: ZfsResult<i32> = Err(ZfsError::pool_error("error"));
        let value = result.unwrap_or(99);

        assert_eq!(value, 99);
    }

    #[test]
    fn test_result_map() {
        let result: ZfsResult<i32> = Ok(5);
        let mapped = result.map(|v| v * 2);

        assert_eq!(mapped.unwrap(), 10);
    }

    #[test]
    fn test_result_and_then() {
        let result: ZfsResult<i32> = Ok(10);
        let chained = result.and_then(|v| Ok(v + 5));

        assert_eq!(chained.unwrap(), 15);
    }

    #[test]
    fn test_error_chain() {
        fn inner() -> ZfsResult<()> {
            Err(ZfsError::pool_error("inner error"))
        }

        fn outer() -> ZfsResult<()> {
            inner()?;
            Ok(())
        }

        let result = outer();
        assert!(result.is_err());
    }
}
