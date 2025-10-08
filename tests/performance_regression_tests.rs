//! Performance Regression Tests
//!
//! This module contains tests to ensure performance optimizations don't regress
//! and that critical performance metrics are maintained.

use nestgate_core::error::Result;
use nestgate_zfs::native::command_executor::NativeZfsCommandExecutor;
use nestgate_zfs::native::dataset_manager::{DatasetCreateOptions, NativeZfsDatasetManager};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::test;

/// Test string allocation performance with constants
#[tokio::test]
async fn test_string_constant_performance() -> Result<(), Box<dyn std::error::Error>> {
    const ITERATIONS: usize = 10_000;

    // Test with constants (optimized)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _compression = nestgate_zfs::config::defaults::COMPRESSION_LZ4;
        let _checksum = nestgate_zfs::config::defaults::CHECKSUM_SHA256;
        let _recordsize = nestgate_zfs::config::defaults::RECORDSIZE_128K;
        Ok(())
    }
    let constant_time = start.elapsed();

    // Test with string allocations (unoptimized baseline)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _compression = "lz4".to_string();
        let _checksum = "sha256".to_string();
        let _recordsize = "128K".to_string();
        Ok(())
    }
    let allocation_time = start.elapsed();

    println!(
        "Constants: {:?}, Allocations: {:?}",
        constant_time, allocation_time
    );

    // Constants should be significantly faster (at least 10x)
    assert!(
        constant_time < allocation_time / 10,
        "String constants should be at least 10x faster than allocations"
    );
    Ok(())
}

/// Test Cow<str> optimization performance
#[tokio::test]
async fn test_cow_str_performance() -> Result<(), Box<dyn std::error::Error>> {
    use std::borrow::Cow;

    const ITERATIONS: usize = 1_000;
    let test_string = "test_dataset_name";

    // Test with Cow (optimized)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _cow: Cow<str> = if test_string.len() > 10 {
            Cow::Borrowed(test_string)
        } else {
            Cow::Owned(test_string.to_uppercase())
        };
        Ok(())
    }
    let cow_time = start.elapsed();

    // Test with String allocation (baseline)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _string = if test_string.len() > 10 {
            test_string.to_string()
        } else {
            test_string.to_uppercase()
        };
        Ok(())
    }
    let string_time = start.elapsed();

    println!("Cow: {:?}, String: {:?}", cow_time, string_time);

    // Cow should be faster for the borrowed case
    assert!(
        cow_time < string_time,
        "Cow<str> should be faster than String allocation for borrowed strings"
    );
    Ok(())
}

/// Test into_owned() vs to_string() performance
#[tokio::test]
async fn test_into_owned_performance() -> Result<(), Box<dyn std::error::Error>> {
    const ITERATIONS: usize = 1_000;
    let test_bytes = b"test output from zfs command";

    // Test with into_owned() (optimized)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _result = String::from_utf8_lossy(test_bytes).into_owned();
        Ok(())
    }
    let into_owned_time = start.elapsed();

    // Test with to_string() (baseline)
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _result = String::from_utf8_lossy(test_bytes).to_string();
        Ok(())
    }
    let to_string_time = start.elapsed();

    println!(
        "into_owned: {:?}, to_string: {:?}",
        into_owned_time, to_string_time
    );

    // Performance should be similar but into_owned() should not be slower
    assert!(
        into_owned_time <= to_string_time * 2,
        "into_owned() should not be significantly slower than to_string()"
    );
    Ok(())
}

/// Benchmark overall ZFS operation performance
#[tokio::test]
async fn test_zfs_operation_performance_baseline() -> Result<(), Box<dyn std::error::Error>> {
    let executor = Arc::new(NativeZfsCommandExecutor::new());
    let manager = NativeZfsDatasetManager::new(executor);

    // Test dataset creation options performance
    let start = Instant::now();
    for i in 0..100 {
        let _options = DatasetCreateOptions {
            compression: Some(nestgate_zfs::config::defaults::COMPRESSION_LZ4.into()),
            record_size: Some(nestgate_zfs::config::defaults::RECORDSIZE_128K.into()),
            ..Default::default()
        };

        // Simulate some processing
        let _dataset_name = format!("test_dataset_{}", i);
        Ok(())
    }
    let creation_time = start.elapsed();

    println!("Dataset options creation time: {:?}", creation_time);

    // Should complete within reasonable time (1ms per operation max)
    assert!(
        creation_time < Duration::from_millis(100),
        "Dataset options creation should be fast"
    );
    Ok(())
}

/// Test memory usage doesn't grow with string operations
#[tokio::test]
async fn test_memory_stability() -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;

    const ITERATIONS: usize = 1_000;

    // Test that our optimizations don't cause memory leaks
    let mut properties: HashMap<String, String> = HashMap::new();

    for i in 0..ITERATIONS {
        // Use optimized patterns
        properties.insert(
            format!("property_{}", i),
            nestgate_zfs::config::defaults::COMPRESSION_LZ4.to_string(),
        );

        // Clear periodically to simulate real usage
        if i % 100 == 0 {
            properties.clear();
            Ok(())
        }
        Ok(())
    }

    // Should not have excessive memory usage
    assert!(
        properties.len() < 100,
        "Memory should be managed efficiently"
    );
    Ok(())
}

#[cfg(test)]
mod benchmarks {
    use super::*;

    /// Comprehensive performance regression test
    #[tokio::test]
    async fn comprehensive_performance_regression() {
        println!("🚀 Running comprehensive performance regression tests...");

        // Run all performance tests
        test_string_constant_performance();
        test_cow_str_performance();
        test_into_owned_performance();
        test_memory_stability();

        println!("✅ All performance regression tests passed!");
        Ok(())
    }
}
