// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **SAFE OPERATIONS INTEGRATION TESTS**
//!
//! Tests for panic-free alternatives to unwrap() and other unsafe operations

use crate::common::*;
use nestgate_core::safe_operations::*;
use std::collections::HashMap;

/// Test safe string operations
#[test]
async fn test_safe_string_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Test safe string parsing
    let valid_number = "42";
    let invalid_number = "not_a_number";
    
    assert_eq!(safe_parse_int(valid_number), Some(42));
    assert_eq!(safe_parse_int(invalid_number), None);
    
    // Test safe string operations
    let test_string = "Hello, NestGate!";
    assert!(safe_contains(test_string, "NestGate"));
    assert!(!safe_contains(test_string, "missing"));
    
    // Test safe substring extraction
    assert_eq!(safe_substring(test_string, 0, 5), Some("Hello".to_string()));
    assert_eq!(safe_substring(test_string, 100, 105), None); // Out of bounds
    Ok(())
}

/// Test safe collection operations
#[test]
async fn test_safe_collection_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut test_vec = vec![1, 2, 3, 4, 5];
    
    // Test safe vector access
    assert_eq!(safe_get(&test_vec, 0), Some(&1));
    assert_eq!(safe_get(&test_vec, 2), Some(&3));
    assert_eq!(safe_get(&test_vec, 10), None); // Out of bounds
    
    // Test safe vector modification
    assert!(safe_set(&mut test_vec, 0, 10));
    assert_eq!(test_vec[0], 10);
    assert!(!safe_set(&mut test_vec, 10, 99)); // Out of bounds
    
    // Test safe push operations
    let original_len = test_vec.len();
    safe_push(&mut test_vec, 6);
    assert_eq!(test_vec.len(), original_len + 1);
    assert_eq!(test_vec[test_vec.len() - 1], 6);
    Ok(())
}

/// Test safe HashMap operations
#[test]
async fn test_safe_hashmap_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    
    // Test safe get operations
    assert_eq!(safe_get_key(&map, "key1"), Some("value1"));
    assert_eq!(safe_get_key(&map, "missing"), None);
    
    // Test safe insert operations
    safe_insert(&mut map, "key3".to_string(), "value3".to_string());
    assert_eq!(map.get("key3"), Some(&"value3".to_string()));
    
    // Test safe remove operations
    assert_eq!(safe_remove(&mut map, "key1"), Some("value1".to_string()));
    assert_eq!(safe_remove(&mut map, "missing"), None);
    assert!(!map.contains_key("key1"));
    Ok(())
}

/// Test safe file operations
#[test]
async fn test_safe_file_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Test safe path operations
    let valid_path = "/tmp/test_file.txt";
    let invalid_path = "/nonexistent/directory/file.txt";
    
    // These should not panic even with invalid paths
    let path_exists = safe_path_exists(valid_path);
    let invalid_exists = safe_path_exists(invalid_path);
    
    // Results depend on filesystem but should not panic
    assert!(path_exists.is_some());
    assert!(invalid_exists.is_some());
    
    // Test safe directory operations
    let temp_dir = "/tmp";
    let result = safe_is_directory(temp_dir);
    assert!(result.is_some());
    Ok(())
}

/// Test safe numeric operations
#[test]
async fn test_safe_numeric_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Test safe division
    assert_eq!(safe_divide(10, 2), Some(5));
    assert_eq!(safe_divide(10, 0), None); // Division by zero
    
    // Test safe modulo
    assert_eq!(safe_modulo(10, 3), Some(1));
    assert_eq!(safe_modulo(10, 0), None); // Modulo by zero
    
    // Test safe square root
    assert_eq!(safe_sqrt(25.0), Some(5.0));
    assert_eq!(safe_sqrt(-1.0), None); // Negative square root
    
    // Test safe overflow operations
    let max_val = i32::MAX;
    assert_eq!(safe_add(max_val, 1), None); // Overflow
    assert_eq!(safe_add(5, 10), Some(15)); // Normal addition
    Ok(())
}

/// Test safe conversion operations
#[test]
async fn test_safe_conversion_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Test safe usize to u32 conversion
    assert_eq!(safe_usize_to_u32(100), Some(100u32));
    assert_eq!(safe_usize_to_u32(usize::MAX), None); // Overflow on 64-bit systems
    
    // Test safe string to various types
    assert_eq!(safe_parse_f64("3.14"), Some(3.14));
    assert_eq!(safe_parse_f64("not_a_float"), None);
    
    assert_eq!(safe_parse_bool("true"), Some(true));
    assert_eq!(safe_parse_bool("false"), Some(false));
    assert_eq!(safe_parse_bool("maybe"), None);
    Ok(())
}

/// Test safe memory operations
#[test]
async fn test_safe_memory_operations() -> Result<(), Box<dyn std::error::Error>> {
    let test_data = vec![1u8, 2, 3, 4, 5];
    
    // Test safe memory copy
    let mut dest = vec![0u8; 5];
    assert!(safe_copy_memory(&test_data, &mut dest));
    assert_eq!(dest, test_data);
    
    // Test safe memory copy with size mismatch
    let mut small_dest = vec![0u8; 3];
    assert!(!safe_copy_memory(&test_data, &mut small_dest)); // Should fail safely
    
    // Test safe memory comparison
    let same_data = vec![1u8, 2, 3, 4, 5];
    let different_data = vec![1u8, 2, 3, 4, 6];
    
    assert!(safe_memory_equal(&test_data, &same_data));
    assert!(!safe_memory_equal(&test_data, &different_data));
    Ok(())
}

/// Test safe async operations
#[tokio::test]
async fn test_safe_async_operations() -> Result<(), Box<dyn std::error::Error>> {
    init_test_logging();
    
    // Test safe async timeout
    let quick_operation = async {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        "completed"
    };
    
    let result = safe_timeout(quick_operation, std::time::Duration::from_millis(100)).await;
    assert_eq!(result, Some("completed"));
    
    // Test safe async timeout with actual timeout
    let slow_operation = async {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        "should_timeout"
    };
    
    let timeout_result = safe_timeout(slow_operation, std::time::Duration::from_millis(50)).await;
    assert_eq!(timeout_result, None);
    Ok(())
}

/// Test safe error handling
#[test]
fn test_safe_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Test safe result unwrapping
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");
    
    assert_eq!(safe_unwrap_or(ok_result, 0), 42);
    assert_eq!(safe_unwrap_or(err_result, 0), 0);
    
    // Test safe option unwrapping
    let some_value = Some(42);
    let none_value: Option<i32> = None;
    
    assert_eq!(safe_unwrap_option_or(some_value, 0), 42);
    assert_eq!(safe_unwrap_option_or(none_value, 0), 0);
    Ok(())
}

/// Test safe resource management
#[test]
fn test_safe_resource_management() -> Result<(), Box<dyn std::error::Error>> {
    // Test safe resource allocation
    let resource = safe_allocate_buffer(1024);
    assert!(resource.is_some());
    
    if let Some(buffer) = resource {
        assert_eq!(buffer.len(), 1024);
        assert_eq!(buffer[0], 0); // Should be zero-initialized
    Ok(())
    }
    
    // Test safe resource allocation with large size
    let large_resource = safe_allocate_buffer(usize::MAX);
    assert!(large_resource.is_none()); // Should fail safely
    Ok(())
}

/// Comprehensive integration test
#[test]
fn test_safe_operations_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test chaining multiple safe operations
    let data = vec!["1", "2", "3", "invalid", "5"];
    let mut results = Vec::new();
    
    for item in data {
        if let Some(num) = safe_parse_int(item) {
            if let Some(doubled) = safe_multiply(num, 2) {
                safe_push(&mut results, doubled);
    Ok(())
            }
    Ok(())
        }
    Ok(())
    }
    
    // Should have successfully processed "1", "2", "3", "5" but skipped "invalid"
    assert_eq!(results, vec![2, 4, 6, 10]);
    
    // Test safe operations don't panic under stress
    for i in 0..1000 {
        let _ = safe_divide(i, i % 10); // Some divisions by zero
        let _ = safe_get(&results, i); // Many out-of-bounds accesses
        let _ = safe_parse_int(&format!("invalid_{}", i)); // Many parse failures
    Ok(())
    }
    
    // If we reach here, no panics occurred
    assert!(true);
    Ok(())
} 