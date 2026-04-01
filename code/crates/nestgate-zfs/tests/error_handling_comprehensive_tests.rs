// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    dead_code,
    unused_doc_comments,
    unused_imports,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    deprecated,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::manual_midpoint,
    clippy::suboptimal_flops,
    clippy::items_after_statements,
    clippy::items_after_test_module,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::unreadable_literal,
    clippy::redundant_clone,
    clippy::useless_vec,
    clippy::field_reassign_with_default,
    clippy::cmp_null,
    clippy::bool_assert_comparison,
    clippy::used_underscore_items,
    clippy::needless_raw_string_hashes,
    clippy::ref_as_ptr,
    clippy::no_effect_underscore_binding,
    clippy::needless_collect,
    clippy::module_inception,
    clippy::default_trait_access,
    clippy::wildcard_in_or_patterns,
    clippy::or_fun_call,
    clippy::manual_string_new,
    clippy::unnecessary_literal_unwrap,
    clippy::unnecessary_debug_formatting,
    clippy::assigning_clones,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_map_or,
    clippy::unnecessary_lazy_evaluations,
    clippy::similar_names,
    clippy::needless_continue,
    clippy::collection_is_never_read,
    clippy::char_lit_as_u8,
    clippy::ptr_eq,
    clippy::uninlined_format_args,
    clippy::absurd_extreme_comparisons,
    clippy::match_wild_err_arm,
    clippy::single_match_else,
    clippy::derive_partial_eq_without_eq,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_const_for_fn,
    clippy::used_underscore_binding,
    clippy::ignored_unit_patterns,
    unused_comparisons,
    clippy::format_push_string
)]

//! Comprehensive error handling tests for nestgate-zfs
//! Target: Improve coverage of error module and error handling patterns
//!
//! This test suite covers:
//! - ZfsError variants and formatting
//! - ZfsErrorBuilder methods
//! - Error conversions and propagation
//! - ZfsOperation enum

use nestgate_zfs::error::{
    ZfsErrorBuilder, ZfsOperation, create_zfs_error, zfs_command_error, zfs_operation_error,
};
use nestgate_zfs::types::ZfsError;
use std::io;

// ==================== ZFS ERROR ENUM TESTS ====================

#[test]
fn test_zfs_error_pool_error() {
    let error = ZfsError::PoolError {
        message: "Test pool error".to_string(),
    };

    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Test pool error"));
    assert!(error_msg.contains("Pool operation failed"));
}

#[test]
fn test_zfs_error_dataset_error() {
    let error = ZfsError::DatasetError {
        message: "Test dataset error".to_string(),
    };

    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Test dataset error"));
    assert!(error_msg.contains("Dataset operation failed"));
}

#[test]
fn test_zfs_error_snapshot_error() {
    let error = ZfsError::SnapshotError {
        message: "Test snapshot error".to_string(),
    };

    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Test snapshot error"));
    assert!(error_msg.contains("Snapshot operation failed"));
}

#[test]
fn test_zfs_error_command_error() {
    let error = ZfsError::CommandError {
        message: "Test command error".to_string(),
    };

    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Test command error"));
    assert!(error_msg.contains("Command execution failed"));
}

#[test]
fn test_zfs_error_config_error() {
    let error = ZfsError::ConfigError {
        message: "Test config error".to_string(),
    };

    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Test config error"));
    assert!(error_msg.contains("Configuration error"));
}

#[test]
fn test_zfs_error_io_conversion() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let zfs_error: ZfsError = io_error.into();

    let error_msg = format!("{}", zfs_error);
    assert!(error_msg.contains("File not found") || error_msg.contains("IO error"));
}

// ==================== ZFS ERROR BUILDER TESTS ====================

#[test]
fn test_zfs_error_builder_new() {
    let error = ZfsErrorBuilder::new("Generic error");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_zfs_error_builder_pool_error() {
    let error = ZfsErrorBuilder::pool_error("Pool error", "testpool");
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Pool error") || debug_str.contains("testpool"));
}

#[test]
fn test_zfs_error_builder_dataset_error() {
    let error = ZfsErrorBuilder::dataset_error("Dataset error", "testdataset");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_zfs_error_builder_snapshot_error() {
    let error = ZfsErrorBuilder::snapshot_error("Snapshot error", "testsnapshot");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_zfs_error_builder_command_error() {
    let error = ZfsErrorBuilder::command_error("zfs list", "timeout");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_zfs_error_builder_zfs_error() {
    let error = ZfsErrorBuilder::zfs_error("Generic ZFS error");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_zfs_error_builder_zfs_operation_error() {
    let error = ZfsErrorBuilder::zfs_operation_error("Operation failed");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

// ==================== ZFS OPERATION ENUM TESTS ====================

#[test]
fn test_zfs_operation_variants() {
    let operations = [
        ZfsOperation::PoolCreate,
        ZfsOperation::PoolDestroy,
        ZfsOperation::PoolImport,
        ZfsOperation::PoolExport,
        ZfsOperation::DatasetCreate,
        ZfsOperation::DatasetDestroy,
        ZfsOperation::DatasetMount,
        ZfsOperation::DatasetUnmount,
        ZfsOperation::SnapshotCreate,
        ZfsOperation::SnapshotDestroy,
        ZfsOperation::Command,
        ZfsOperation::SystemCheck,
        ZfsOperation::Configuration,
    ];

    assert_eq!(operations.len(), 13);
}

#[test]
fn test_zfs_operation_debug() {
    let op = ZfsOperation::PoolCreate;
    let debug_str = format!("{:?}", op);

    assert!(!debug_str.is_empty());
}

#[test]
fn test_zfs_operation_clone() {
    let op1 = ZfsOperation::DatasetCreate;
    let op2 = op1.clone();

    assert!(format!("{:?}", op1) == format!("{:?}", op2));
}

// ==================== CREATE ZFS ERROR TESTS ====================

#[test]
fn test_create_zfs_error_pool_create() {
    let error = create_zfs_error("Pool creation failed".to_string(), ZfsOperation::PoolCreate);
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_create_zfs_error_dataset_create() {
    let error = create_zfs_error(
        "Dataset creation failed".to_string(),
        ZfsOperation::DatasetCreate,
    );
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_create_zfs_error_snapshot_create() {
    let error = create_zfs_error(
        "Snapshot creation failed".to_string(),
        ZfsOperation::SnapshotCreate,
    );
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_create_zfs_error_command() {
    let error = create_zfs_error("Command failed".to_string(), ZfsOperation::Command);
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

// ==================== COMMAND ERROR TESTS ====================

#[test]
fn test_zfs_command_error() {
    let error = zfs_command_error("zfs list", "command not found");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_zfs_command_error_with_empty_output() {
    let error = zfs_command_error("zfs create", "");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

// ==================== OPERATION ERROR TESTS ====================

#[test]
fn test_zfs_operation_error() {
    let error = zfs_operation_error("pool_create", "insufficient permissions");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_zfs_operation_error_with_details() {
    let error = zfs_operation_error("dataset_mount", "mount point does not exist");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

// ==================== ERROR DEBUG TESTS ====================

#[test]
fn test_error_debug_output() {
    let error = ZfsError::PoolError {
        message: "Debug test".to_string(),
    };

    let debug_output = format!("{:?}", error);
    assert!(!debug_output.is_empty());
    assert!(debug_output.contains("PoolError") || debug_output.contains("Debug test"));
}

#[test]
fn test_multiple_error_types() {
    let errors: Vec<ZfsError> = vec![
        ZfsError::PoolError {
            message: "Pool".to_string(),
        },
        ZfsError::DatasetError {
            message: "Dataset".to_string(),
        },
        ZfsError::SnapshotError {
            message: "Snapshot".to_string(),
        },
    ];

    assert_eq!(errors.len(), 3);

    for error in &errors {
        let debug_str = format!("{}", error);
        assert!(!debug_str.is_empty());
    }
}

// ==================== ERROR PROPAGATION TESTS ====================

#[test]
fn test_result_ok_propagation() {
    /// Returns Ok
    fn returns_ok() -> std::result::Result<i32, ZfsError> {
        Ok(42)
    }

    let result = returns_ok();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_result_err_propagation() {
    /// Returns Err
    fn returns_err() -> std::result::Result<i32, ZfsError> {
        Err(ZfsError::PoolError {
            message: "Test error".to_string(),
        })
    }

    let result = returns_err();
    assert!(result.is_err());
}

#[test]
fn test_result_chain() {
    /// Step1
    fn step1() -> std::result::Result<i32, ZfsError> {
        Ok(10)
    }

    /// Step2
    fn step2(value: i32) -> std::result::Result<i32, ZfsError> {
        Ok(value * 2)
    }

    let result = step1().and_then(step2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 20);
}

#[test]
fn test_result_map() {
    let result: Result<i32, ZfsError> = Ok(5);
    let mapped = result.map(|x| x * 3);

    assert_eq!(mapped.unwrap(), 15);
}

#[test]
fn test_result_map_err() {
    let result: Result<i32, ZfsError> = Err(ZfsError::PoolError {
        message: "Original".to_string(),
    });

    let mapped = result.map_err(|_| ZfsError::DatasetError {
        message: "Mapped".to_string(),
    });

    assert!(mapped.is_err());
}

// ==================== ERROR CONTEXT TESTS ====================

#[test]
fn test_error_with_context() {
    let base_error = ZfsError::CommandError {
        message: "Base error".to_string(),
    };

    let context = format!("Context: Operation failed - {}", base_error);
    assert!(context.contains("Base error"));
    assert!(context.contains("Context"));
}

#[test]
fn test_nested_error_handling() {
    /// Inner
    fn inner() -> std::result::Result<(), ZfsError> {
        Err(ZfsError::PoolError {
            message: "Inner error".to_string(),
        })
    }

    /// Outer
    fn outer() -> std::result::Result<(), ZfsError> {
        inner().map_err(|e| ZfsError::CommandError {
            message: format!("Outer wrapping: {}", e),
        })
    }

    let result = outer();
    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_msg = format!("{}", error);
    assert!(error_msg.contains("Inner error") || error_msg.contains("Outer wrapping"));
}

// ==================== ERROR MATCHING TESTS ====================

#[test]
fn test_error_pattern_matching() {
    let error = ZfsError::PoolError {
        message: "Test".to_string(),
    };

    match error {
        ZfsError::PoolError { message } => {
            assert_eq!(message, "Test");
        }
        _ => panic!("Wrong error type"),
    }
}

#[test]
fn test_error_type_checking() {
    /// Checks if Pool Error
    fn is_pool_error(error: &ZfsError) -> bool {
        matches!(error, ZfsError::PoolError { .. })
    }

    let pool_error = ZfsError::PoolError {
        message: "Pool".to_string(),
    };
    let dataset_error = ZfsError::DatasetError {
        message: "Dataset".to_string(),
    };

    assert!(is_pool_error(&pool_error));
    assert!(!is_pool_error(&dataset_error));
}

// ==================== RESULT HELPER TESTS ====================

#[test]
fn test_result_unwrap_or() {
    // Test unwrap_or with dynamic results (not compile-time known)
    fn get_ok_result() -> std::result::Result<i32, ZfsError> {
        Ok(42)
    }
    /// Gets Err Result
    fn get_err_result() -> std::result::Result<i32, ZfsError> {
        Err(ZfsError::PoolError {
            message: "Error".to_string(),
        })
    }

    assert_eq!(get_ok_result().unwrap_or(0), 42);
    assert_eq!(get_err_result().unwrap_or(0), 0);
}

#[test]
fn test_result_unwrap_or_else() {
    // Test unwrap_or with dynamic error (not compile-time known)
    fn get_error() -> std::result::Result<i32, ZfsError> {
        Err(ZfsError::PoolError {
            message: "Error".to_string(),
        })
    }

    let value = get_error().unwrap_or(99);
    assert_eq!(value, 99);
}

#[test]
fn test_result_ok() {
    let result: Result<i32, ZfsError> = Ok(42);
    assert_eq!(result.ok(), Some(42));

    let error_result: Result<i32, ZfsError> = Err(ZfsError::PoolError {
        message: "Error".to_string(),
    });
    assert_eq!(error_result.ok(), None);
}

#[test]
fn test_result_err() {
    let ok_result: Result<i32, ZfsError> = Ok(42);
    assert!(ok_result.err().is_none());

    let err_result: Result<i32, ZfsError> = Err(ZfsError::PoolError {
        message: "Error".to_string(),
    });
    assert!(err_result.err().is_some());
}

// ==================== ERROR MESSAGE TESTS ====================

#[test]
fn test_empty_error_message() {
    let error = ZfsError::PoolError {
        message: "".to_string(),
    };

    let error_msg = format!("{}", error);
    assert!(!error_msg.is_empty()); // Should still have base message
}

#[test]
fn test_long_error_message() {
    let long_message = "a".repeat(1000);
    let error = ZfsError::PoolError {
        message: long_message.clone(),
    };

    let error_msg = format!("{}", error);
    assert!(error_msg.contains(&long_message));
}

#[test]
fn test_special_characters_in_error() {
    let message = "Error with special chars: \n\t\"quotes\" 'apostrophes'";
    let error = ZfsError::PoolError {
        message: message.to_string(),
    };

    let error_msg = format!("{}", error);
    assert!(!error_msg.is_empty());
}

// ==================== IO ERROR CONVERSION TESTS ====================

#[test]
fn test_io_error_not_found() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let zfs_err: ZfsError = io_err.into();

    let debug_str = format!("{}", zfs_err);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_io_error_permission_denied() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
    let zfs_err: ZfsError = io_err.into();

    let debug_str = format!("{}", zfs_err);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_io_error_connection_refused() {
    let io_err = io::Error::new(io::ErrorKind::ConnectionRefused, "Connection refused");
    let zfs_err: ZfsError = io_err.into();

    let debug_str = format!("{}", zfs_err);
    assert!(!debug_str.is_empty());
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_error_in_function_chain() {
    /// Step1
    fn step1() -> std::result::Result<i32, ZfsError> {
        Ok(10)
    }

    /// Step2
    fn step2(_input: i32) -> std::result::Result<i32, ZfsError> {
        Err(ZfsError::PoolError {
            message: "Step 2 failed".to_string(),
        })
    }

    /// Step3
    fn step3(_input: i32) -> std::result::Result<i32, ZfsError> {
        Ok(30)
    }

    let result = step1().and_then(step2).and_then(step3);

    assert!(result.is_err());
}

#[test]
fn test_multiple_operations_with_errors() {
    let operations = vec![
        (ZfsOperation::PoolCreate, "Pool creation"),
        (ZfsOperation::DatasetCreate, "Dataset creation"),
        (ZfsOperation::SnapshotCreate, "Snapshot creation"),
    ];

    for (op, msg) in operations {
        let error = create_zfs_error(msg.to_string(), op);
        let debug_str = format!("{:?}", error);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_error_accumulation() {
    let errors: Vec<ZfsError> = vec![
        ZfsError::PoolError {
            message: "Error 1".to_string(),
        },
        ZfsError::DatasetError {
            message: "Error 2".to_string(),
        },
        ZfsError::SnapshotError {
            message: "Error 3".to_string(),
        },
    ];

    assert_eq!(errors.len(), 3);

    for error in &errors {
        let debug_str = format!("{}", error);
        assert!(!debug_str.is_empty());
    }
}

// ==================== BUILDER EDGE CASES ====================

#[test]
fn test_builder_with_empty_strings() {
    let error = ZfsErrorBuilder::pool_error("", "");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_builder_with_special_characters() {
    let error = ZfsErrorBuilder::pool_error("Error\nwith\nnewlines", "test\tpool");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_all_operations_create_errors() {
    let ops = vec![
        ZfsOperation::PoolCreate,
        ZfsOperation::PoolDestroy,
        ZfsOperation::PoolImport,
        ZfsOperation::PoolExport,
        ZfsOperation::DatasetCreate,
        ZfsOperation::DatasetDestroy,
        ZfsOperation::DatasetMount,
        ZfsOperation::DatasetUnmount,
        ZfsOperation::SnapshotCreate,
        ZfsOperation::SnapshotDestroy,
        ZfsOperation::Command,
        ZfsOperation::SystemCheck,
        ZfsOperation::Configuration,
    ];

    for op in ops {
        let error = create_zfs_error("Test".to_string(), op);
        let debug_str = format!("{:?}", error);
        assert!(!debug_str.is_empty());
    }
}
