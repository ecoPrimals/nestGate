// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS Error Handling Tests - December 18, 2025
//!
//! Comprehensive error handling tests for ZFS operations.
//! Part of test coverage expansion (73.58% → 90%).
//!
//! **Focus**: Error paths, edge cases, invalid inputs, boundary conditions

use nestgate_zfs::ZfsError;
use std::path::PathBuf;

// ==================== CAPACITY ERRORS ====================

#[test]
fn test_capacity_too_small_error() {
    let error = ZfsError::capacity_too_small("Pool size must be at least 64MB");

    match error {
        ZfsError::CapacityTooSmall { .. } => {
            // Expected variant
        }
        _ => panic!("Expected CapacityTooSmall variant"),
    }
}

#[test]
fn test_capacity_exceeded_error() {
    let error = ZfsError::capacity_exceeded("Pool capacity exceeded");

    match error {
        ZfsError::CapacityExceeded { .. } => {
            // Expected variant
        }
        _ => panic!("Expected CapacityExceeded variant"),
    }
}

#[test]
fn test_capacity_errors_are_different() {
    let too_small = ZfsError::capacity_too_small("too small");
    let exceeded = ZfsError::capacity_exceeded("exceeded");

    // These should be different error variants
    assert!(std::mem::discriminant(&too_small) != std::mem::discriminant(&exceeded));
}

// ==================== PATH VALIDATION ====================

#[test]
fn test_invalid_path_error() {
    let error = ZfsError::invalid_path("/invalid/../path");

    match error {
        ZfsError::InvalidPath { .. } => {
            // Expected variant
        }
        _ => panic!("Expected InvalidPath variant"),
    }
}

#[test]
fn test_path_with_null_bytes() {
    let path_with_null = PathBuf::from("path\0with\0nulls");
    let error = ZfsError::invalid_path(path_with_null.display().to_string());

    match error {
        ZfsError::InvalidPath { .. } => {
            // Expected
        }
        _ => panic!("Expected InvalidPath variant"),
    }
}

// ==================== PROPERTY VALIDATION ====================

#[test]
fn test_invalid_property_error() {
    let error = ZfsError::invalid_property("nonexistent_property");

    match error {
        ZfsError::InvalidProperty { .. } => {
            // Expected variant
        }
        _ => panic!("Expected InvalidProperty variant"),
    }
}

#[test]
fn test_property_with_invalid_chars() {
    let error = ZfsError::invalid_property("property@#$%");

    match error {
        ZfsError::InvalidProperty { .. } => {
            // Expected variant
        }
        _ => panic!("Expected InvalidProperty variant"),
    }
}

// ==================== CROSS-POOL OPERATIONS ====================

#[test]
fn test_cross_pool_rename_error() {
    let error = ZfsError::cross_pool_rename("Cannot rename across pools");

    match error {
        ZfsError::CrossPoolRename { .. } => {
            // Expected variant
        }
        _ => panic!("Expected CrossPoolRename variant"),
    }
}

// ==================== ERROR MESSAGE QUALITY ====================

#[test]
fn test_error_messages_are_informative() {
    let errors = vec![
        ZfsError::capacity_too_small("Size too small"),
        ZfsError::capacity_exceeded("Size exceeded"),
        ZfsError::invalid_path("/bad/path"),
        ZfsError::invalid_property("bad_property"),
        ZfsError::cross_pool_rename("Cannot rename"),
    ];

    for error in errors {
        let msg = format!("{}", error);
        // Error messages should not be empty
        assert!(!msg.is_empty(), "Error message should not be empty");
        // Should have some content beyond just the error type
        assert!(msg.len() > 10, "Error message should be descriptive");
    }
}

// ==================== ERROR CONVERSION ====================

#[test]
fn test_error_display() {
    let error = ZfsError::capacity_too_small("Test message");
    let display = format!("{}", error);

    // Display should include the message
    assert!(display.contains("Test message") || display.contains("capacity"));
}

#[test]
fn test_error_debug() {
    let error = ZfsError::invalid_path("/test/path");
    let debug = format!("{:?}", error);

    // Debug should show the variant name and content
    assert!(!debug.is_empty());
    assert!(debug.contains("InvalidPath") || debug.contains("path"));
}

// ==================== CONCURRENT ERROR HANDLING ====================

#[test]
fn test_concurrent_error_creation() {
    use std::sync::Arc;
    use std::thread;

    let errors = Arc::new(std::sync::Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..10)
        .map(|i| {
            let errors = Arc::clone(&errors);
            thread::spawn(move || {
                let err = ZfsError::capacity_too_small(format!("Error {}", i));
                errors.lock().unwrap().push(err);
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread should complete");
    }

    let errors = errors.lock().unwrap();
    assert_eq!(errors.len(), 10);
}

// ==================== BOUNDARY CONDITIONS ====================

#[test]
fn test_very_long_error_message() {
    let long_msg = "x".repeat(10000);
    let error = ZfsError::capacity_too_small(long_msg);

    // Should handle very long messages without panicking
    let _display = format!("{}", error);
}

#[test]
fn test_empty_error_message() {
    let error = ZfsError::capacity_too_small("");

    // Should handle empty messages gracefully
    let display = format!("{}", error);
    assert!(!display.is_empty()); // Should still have error type name
}

#[test]
fn test_unicode_in_error_message() {
    let error = ZfsError::invalid_path("路径/数据集/🚀");

    // Should handle Unicode without panicking
    let display = format!("{}", error);
    assert!(!display.is_empty());
}

// ==================== ERROR EQUALITY ====================

#[test]
fn test_same_error_type_different_messages() {
    let error1 = ZfsError::capacity_too_small("Message 1");
    let error2 = ZfsError::capacity_too_small("Message 2");

    // Same variant type
    assert_eq!(
        std::mem::discriminant(&error1),
        std::mem::discriminant(&error2)
    );
}

#[test]
fn test_different_error_types() {
    let error1 = ZfsError::capacity_too_small("test");
    let error2 = ZfsError::capacity_exceeded("test");

    // Different variant types
    assert_ne!(
        std::mem::discriminant(&error1),
        std::mem::discriminant(&error2)
    );
}

// ==================== ERROR CHAINING ====================

#[test]
fn test_error_conversion_to_string() {
    let errors = vec![
        ZfsError::capacity_too_small("test"),
        ZfsError::capacity_exceeded("test"),
        ZfsError::invalid_path("test"),
        ZfsError::invalid_property("test"),
        ZfsError::cross_pool_rename("test"),
    ];

    for error in errors {
        let s = error.to_string();
        assert!(!s.is_empty());
    }
}

// ==================== REGRESSION TESTS ====================

#[test]
fn test_error_variants_exist() {
    // Ensure all required error variants exist
    let _ = ZfsError::capacity_too_small("test");
    let _ = ZfsError::capacity_exceeded("test");
    let _ = ZfsError::invalid_path("test");
    let _ = ZfsError::invalid_property("test");
    let _ = ZfsError::cross_pool_rename("test");

    // If we get here, all variants exist and compile
}
