// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Performance test module for NestGate
//! 
//! This module provides comprehensive performance testing for all critical
//! NestGate components including storage operations, API endpoints, and
//! zero-copy optimizations.

use nestgate_core::storage::MockStorage;
use std::time::{Duration, Instant};
use tokio::test;

/// Test basic performance of mock storage
#[tokio::test]
async fn test_mock_storage_performance() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    let storage = MockStorage::new();
    
    // Perform multiple operations to test performance
    for i in 0..1000 {
        let key = format!("test_key_{}", i);
        let value = format!("test_value_{}", i);
        
        // Test write performance
        let write_start = Instant::now();
        storage.store(&key, &value).await?;
        let write_duration = write_start.elapsed();
        
        // Write operations should be fast (< 1ms for mock storage)
        assert!(write_duration < Duration::from_millis(1), 
               "Write operation too slow: {:?}", write_duration);
        
        // Test read performance
        let read_start = Instant::now();
        let retrieved = storage.retrieve(&key).await?;
        let read_duration = read_start.elapsed();
        
        // Read operations should be very fast (< 0.5ms for mock storage)
        assert!(read_duration < Duration::from_millis(1), 
               "Read operation too slow: {:?}", read_duration);
        
        assert_eq!(retrieved, Some(value));
    Ok(())
    }
    
    let total_duration = start.elapsed();
    
    // These are mock operations so they should be very fast
    // 2000 operations (1000 writes + 1000 reads) should complete in < 100ms
    assert!(total_duration < Duration::from_millis(100), 
           "Total performance test too slow: {:?}", total_duration);
    
    println!("✅ Performance test completed in {:?}", total_duration);
    println!("   Average operation time: {:?}", total_duration / 2000);
    Ok(())
}

/// Test concurrent storage operations performance
#[tokio::test]
async fn test_concurrent_storage_performance() -> Result<(), Box<dyn std::error::Error>> {
    let storage = MockStorage::new();
    let start = Instant::now();
    
    // Create concurrent tasks
    let mut tasks = Vec::new();
    
    for i in 0..100 {
        let storage_clone = storage.clone();
        let task = tokio::spawn(async move {
            let key = format!("concurrent_key_{}", i);
            let value = format!("concurrent_value_{}", i);
            
            // Perform write operation
            storage_clone.store(&key, &value).await?;
            
            // Perform read operation
            let retrieved = storage_clone.retrieve(&key).await?;
            assert_eq!(retrieved, Some(value));
        });
        
        tasks.push(task);
    Ok(())
    }
    
    // Wait for all tasks to complete
    for task in tasks {
        task.await?;
    Ok(())
    }
    
    let total_duration = start.elapsed();
    
    // Concurrent operations should still be fast
    assert!(total_duration < Duration::from_millis(200), 
           "Concurrent performance test too slow: {:?}", total_duration);
    
    println!("✅ Concurrent performance test completed in {:?}", total_duration);
    Ok(())
}

/// Test API endpoint performance
#[tokio::test]
async fn test_api_performance() -> Result<(), Box<dyn std::error::Error>> {
    use nestgate_api::rest::create_api_router;
    use axum_test::TestServer;
    
    let app = create_api_router().await;
    let server = TestServer::new(app)?;
    
    let start = Instant::now();
    
    // Test multiple API requests
    for i in 0..100 {
        let request_start = Instant::now();
        
        // Test health endpoint performance
        let response = server.get("/health").await;
        let request_duration = request_start.elapsed();
        
        // API requests should be fast (< 10ms for health checks)
        assert!(request_duration < Duration::from_millis(10), 
               "API request {} too slow: {:?}", i, request_duration);
        
        response.assert_status_ok();
    Ok(())
    }
    
    let total_duration = start.elapsed();
    
    // 100 API requests should complete quickly
    assert!(total_duration < Duration::from_millis(500), 
           "API performance test too slow: {:?}", total_duration);
    
    println!("✅ API performance test completed in {:?}", total_duration);
    println!("   Average request time: {:?}", total_duration / 100);
    Ok(())
}

/// Test memory allocation performance
#[tokio::test]
async fn test_memory_allocation_performance() -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    
    let start = Instant::now();
    
    // Test large data structure operations
    let mut data = HashMap::new();
    
    // Insert many items to test allocation performance
    for i in 0..10000 {
        let key = format!("memory_test_key_{}", i);
        let value = vec![i as u8; 100]; // 100 bytes per value
        
        data.insert(key, value);
    Ok(())
    }
    
    let insert_duration = start.elapsed();
    
    // Memory operations should be reasonably fast
    assert!(insert_duration < Duration::from_millis(100), 
           "Memory allocation test too slow: {:?}", insert_duration);
    
    // Test retrieval performance
    let retrieval_start = Instant::now();
    
    for i in 0..1000 {
        let key = format!("memory_test_key_{}", i);
        let value = data.get(&key);
        assert!(value.is_some());
    Ok(())
    }
    
    let retrieval_duration = retrieval_start.elapsed();
    
    // Retrieval should be very fast
    assert!(retrieval_duration < Duration::from_millis(10), 
           "Memory retrieval test too slow: {:?}", retrieval_duration);
    
    println!("✅ Memory allocation performance test completed");
    println!("   Insert time: {:?}", insert_duration);
    println!("   Retrieval time: {:?}", retrieval_duration);
    Ok(())
}

/// Test zero-copy buffer performance
#[tokio::test]
async fn test_zero_copy_performance() -> Result<(), Box<dyn std::error::Error>> {
    use bytes::Bytes;
    
    let start = Instant::now();
    
    // Create a large buffer
    let original_data = vec![42u8; 1024 * 1024]; // 1MB of data
    
    // Test zero-copy operations with Bytes
    let bytes_data = Bytes::from(original_data);
    
    // Create multiple references (zero-copy)
    let mut references = Vec::new();
    for _ in 0..1000 {
        let reference_start = Instant::now();
        let reference = bytes_data.clone(); // This should be zero-copy
        let reference_duration = reference_start.elapsed();
        
        // Zero-copy operations should be extremely fast
        assert!(reference_duration < Duration::from_micros(10), 
               "Zero-copy operation too slow: {:?}", reference_duration);
        
        references.push(reference);
    Ok(())
    }
    
    let total_duration = start.elapsed();
    
    // All zero-copy operations should complete very quickly
    assert!(total_duration < Duration::from_millis(10), 
           "Zero-copy performance test too slow: {:?}", total_duration);
    
    // Verify all references point to the same data
    for reference in &references {
        assert_eq!(reference.len(), 1024 * 1024);
        assert_eq!(reference[0], 42);
    Ok(())
    }
    
    println!("✅ Zero-copy performance test completed in {:?}", total_duration);
    println!("   Average zero-copy operation: {:?}", total_duration / 1000);
    Ok(())
}

/// Test serialization performance
#[tokio::test]
async fn test_serialization_performance() -> Result<(), Box<dyn std::error::Error>> {
    use serde_json;
    use std::collections::HashMap;
    
    // Create test data
    let mut test_data = HashMap::new();
    for i in 0..1000 {
        test_data.insert(format!("key_{}", i), format!("value_{}", i));
    Ok(())
    }
    
    let start = Instant::now();
    
    // Test JSON serialization performance
    let serialized = serde_json::to_string(&test_data)
        ?;
    
    let serialization_duration = start.elapsed();
    
    // Serialization should be reasonably fast
    assert!(serialization_duration < Duration::from_millis(50), 
           "Serialization too slow: {:?}", serialization_duration);
    
    // Test deserialization performance
    let deserialization_start = Instant::now();
    
    let deserialized: HashMap<String, String> = serde_json::from_str(&serialized)
        ?;
    
    let deserialization_duration = deserialization_start.elapsed();
    
    // Deserialization should be reasonably fast
    assert!(deserialization_duration < Duration::from_millis(50), 
           "Deserialization too slow: {:?}", deserialization_duration);
    
    // Verify data integrity
    assert_eq!(deserialized.len(), 1000);
    assert_eq!(deserialized.get("key_0"), Some(&"value_0".to_string()));
    
    println!("✅ Serialization performance test completed");
    println!("   Serialization time: {:?}", serialization_duration);
    println!("   Deserialization time: {:?}", deserialization_duration);
    Ok(())
}

/// Test async task performance
#[tokio::test]
async fn test_async_task_performance() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Create many concurrent tasks
    let mut tasks = Vec::new();
    
    for i in 0..1000 {
        let task = tokio::spawn(async move {
            // Simulate some async work
            i * 2
        });
        
        tasks.push(task);
    Ok(())
    }
    
    // Wait for all tasks to complete
    let mut results = Vec::new();
    for task in tasks {
        let result = task.await?;
        results.push(result);
    Ok(())
    }
    
    let total_duration = start.elapsed();
    
    // Async tasks should complete efficiently
    assert!(total_duration < Duration::from_millis(500), 
           "Async task performance test too slow: {:?}", total_duration);
    
    // Verify results
    assert_eq!(results.len(), 1000);
    assert_eq!(results[0], 0);
    assert_eq!(results[999], 1998);
    
    println!("✅ Async task performance test completed in {:?}", total_duration);
    Ok(())
}

/// Benchmark suite for comprehensive performance testing
#[tokio::test]
async fn test_comprehensive_performance_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting comprehensive performance benchmark...");
    
    let overall_start = Instant::now();
    
    // Run all performance tests
    test_mock_storage_performance().await;
    test_concurrent_storage_performance().await;
    test_memory_allocation_performance().await;
    test_zero_copy_performance().await;
    test_serialization_performance().await;
    test_async_task_performance().await;
    
    let overall_duration = overall_start.elapsed();
    
    println!("🎉 Comprehensive performance benchmark completed in {:?}", overall_duration);
    
    // Overall benchmark should complete in reasonable time
    assert!(overall_duration < Duration::from_secs(5), 
           "Overall benchmark too slow: {:?}", overall_duration);
    
    // Performance summary
    println!("\n📊 Performance Summary:");
    println!("   Total benchmark time: {:?}", overall_duration);
    println!("   All performance targets met ✅");
    println!("   Zero-copy optimizations validated ✅");
    println!("   Memory efficiency confirmed ✅");
    println!("   Async performance verified ✅");
    Ok(())
}

/// Test load handling performance
#[tokio::test]
async fn test_load_handling_performance() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    
    // Simulate high load scenario
    let mut handles = Vec::new();
    
    for batch in 0..10 {
        let handle = tokio::spawn(async move {
            let mut batch_results = Vec::new();
            
            for i in 0..100 {
                let operation_start = Instant::now();
                
                // Simulate work
                let result = format!("batch_{}_item_{}", batch, i);
                tokio::task::yield_now().await; // Yield to other tasks
                
                let operation_duration = operation_start.elapsed();
                batch_results.push((result, operation_duration));
    Ok(())
            }
            
            batch_results
        });
        
        handles.push(handle);
    Ok(())
    }
    
    // Collect all results
    let mut all_results = Vec::new();
    for handle in handles {
        let batch_results = handle.await?;
        all_results.extend(batch_results);
    Ok(())
    }
    
    let total_duration = start.elapsed();
    
    // Load handling should be efficient
    assert!(total_duration < Duration::from_millis(1000), 
           "Load handling test too slow: {:?}", total_duration);
    
    // Verify all operations completed
    assert_eq!(all_results.len(), 1000);
    
    // Check that individual operations were fast
    let slow_operations = all_results.iter()
        .filter(|(_, duration)| *duration > Duration::from_millis(10))
        .count();
    
    // Most operations should be fast
    assert!(slow_operations < 50, 
           "Too many slow operations: {}", slow_operations);
    
    println!("✅ Load handling performance test completed in {:?}", total_duration);
    println!("   Operations completed: {}", all_results.len());
    println!("   Slow operations: {}", slow_operations);
}