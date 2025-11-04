//! Comprehensive error handling tests for ZFS operations
//!
//! These tests ensure robust error handling across all ZFS operations.

use nestgate_zfs::error::{ZfsError, ZfsResult};

#[test]
fn test_zfs_error_display() {
    let error = ZfsError::PoolError {
        message: "Pool not found: test-pool".to_string(),
    };
    let display = format!("{}", error);
    assert!(display.contains("test-pool"));
}

#[test]
fn test_zfs_error_debug() {
    let error = ZfsError::PoolError {
        message: "Pool not found: test-pool".to_string(),
    };
    let debug = format!("{:?}", error);
    assert!(debug.contains("PoolError"));
    assert!(debug.contains("test-pool"));
}

#[test]
fn test_zfs_error_pool_not_found() {
    let error = ZfsError::PoolError {
        message: "Pool not found: mypool".to_string(),
    };
    match error {
        ZfsError::PoolError { message } => assert!(message.contains("mypool")),
        _ => panic!("Wrong error variant"),
    }
}

#[test]
fn test_zfs_error_dataset_not_found() {
    let error = ZfsError::DatasetError {
        message: "Dataset not found: mydataset".to_string(),
    };
    match error {
        ZfsError::DatasetError { message } => assert!(message.contains("mydataset")),
        _ => panic!("Wrong error variant"),
    }
}

#[test]
fn test_zfs_error_invalid_pool_name() {
    let error = ZfsError::ConfigError {
        message: "Invalid pool name".to_string(),
    };
    match error {
        ZfsError::ConfigError { message } => assert!(message.contains("Invalid")),
        _ => panic!("Wrong error variant"),
    }
}

#[test]
fn test_zfs_error_command_failed() {
    let error = ZfsError::CommandError {
        message: "Command 'zpool create' failed with exit code 1: error message".to_string(),
    };

    match error {
        ZfsError::CommandError { message } => {
            assert!(message.contains("zpool create"));
            assert!(message.contains("error message"));
        }
        _ => panic!("Wrong error variant"),
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
    let result: ZfsResult<i32> = Err(ZfsError::PoolError {
        message: "Pool not found: test".to_string(),
    });
    assert!(result.is_err());
}

#[test]
fn test_zfs_error_from_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let zfs_error = ZfsError::from(io_error);

    match zfs_error {
        ZfsError::Io(_) => {} // Expected
        _ => panic!("Wrong error variant"),
    }
}

#[test]
fn test_zfs_error_chain() {
    let result: ZfsResult<()> = Err(ZfsError::PoolError {
        message: "Pool not found: test".to_string(),
    });
    let chained = result.map_err(|e| ZfsError::CommandError {
        message: format!("Command 'test' failed with exit code 1: {}", e),
    });

    assert!(chained.is_err());
}

// Edge case tests
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_pool_name_error() {
        let error = ZfsError::InvalidPoolName("".to_string());
        let display = format!("{}", error);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_very_long_pool_name_error() {
        let long_name = "a".repeat(10000);
        let error = ZfsError::PoolNotFound(long_name.clone());
        match error {
            ZfsError::PoolNotFound(name) => assert_eq!(name.len(), 10000),
            _ => panic!("Wrong error variant"),
        }
    }

    #[test]
    fn test_special_characters_in_error() {
        let error = ZfsError::PoolNotFound("test-pool_123/456".to_string());
        let display = format!("{}", error);
        assert!(display.contains("test-pool_123/456"));
    }

    #[test]
    fn test_unicode_in_error() {
        let error = ZfsError::PoolNotFound("プール".to_string());
        let display = format!("{}", error);
        assert!(display.contains("プール"));
    }

    #[test]
    fn test_command_failed_with_empty_stderr() {
        let error = ZfsError::CommandFailed {
            command: "zpool".to_string(),
            exit_code: 127,
            stderr: "".to_string(),
        };

        let display = format!("{}", error);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_command_failed_with_multiline_stderr() {
        let error = ZfsError::CommandFailed {
            command: "zpool".to_string(),
            exit_code: 1,
            stderr: "line1\nline2\nline3".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("line1") || display.contains("zpool"));
    }

    #[test]
    fn test_zero_exit_code_in_command_failed() {
        let error = ZfsError::CommandFailed {
            command: "zpool".to_string(),
            exit_code: 0,
            stderr: "weird error".to_string(),
        };

        match error {
            ZfsError::CommandFailed { exit_code, .. } => assert_eq!(exit_code, 0),
            _ => panic!("Wrong error variant"),
        }
    }
}

// Error propagation tests
mod propagation {
    use super::*;

    #[test]
    fn test_error_propagation_with_question_mark() {
        fn inner_function() -> ZfsResult<i32> {
            Err(ZfsError::PoolNotFound("test".to_string()))
        }

        fn outer_function() -> ZfsResult<i32> {
            let _value = inner_function()?;
            Ok(42)
        }

        let result = outer_function();
        assert!(result.is_err());
        match result.unwrap_err() {
            ZfsError::PoolNotFound(name) => assert_eq!(name, "test"),
            _ => panic!("Wrong error variant"),
        }
    }

    #[test]
    fn test_error_conversion_chain() {
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let zfs_error: ZfsError = io_error.into();

        match zfs_error {
            ZfsError::Io(e) => assert_eq!(e.kind(), std::io::ErrorKind::PermissionDenied),
            _ => panic!("Wrong error variant"),
        }
    }
}
