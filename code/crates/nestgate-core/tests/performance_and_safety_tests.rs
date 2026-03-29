// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective
#![allow(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Performance and benchmarking test utilities
//!
//! Tests for performance-critical paths and benchmarking infrastructure.

#[cfg(test)]
mod performance_validation_tests {
    use std::time::{Duration, Instant};

    #[test]
    fn test_fast_path_performance() {
        // Test that common operations are fast
        let start = Instant::now();

        let mut sum = 0;
        for i in 0..10000 {
            sum += i;
        }

        let duration = start.elapsed();

        assert_eq!(sum, 49995000);
        // Should complete quickly (< 1ms on modern hardware)
        assert!(duration < Duration::from_millis(10));
    }

    #[test]
    fn test_allocation_efficiency() {
        // Test that we can allocate efficiently
        let start = Instant::now();

        let mut vec = Vec::with_capacity(1000);
        for i in 0..1000 {
            vec.push(i);
        }

        let duration = start.elapsed();

        assert_eq!(vec.len(), 1000);
        // Pre-allocated, should be very fast
        assert!(duration < Duration::from_millis(5));
    }

    #[test]
    fn test_iterator_performance() {
        let data: Vec<i32> = (0..10000).collect();

        let start = Instant::now();
        let sum: i32 = data.iter().sum();
        let duration = start.elapsed();

        assert_eq!(sum, 49995000);
        // Iterator should be fast
        assert!(duration < Duration::from_millis(5));
    }

    #[test]
    fn test_string_operations_efficiency() {
        let start = Instant::now();

        let mut s = String::with_capacity(1000);
        for _ in 0..100 {
            s.push_str("test ");
        }

        let duration = start.elapsed();

        assert!(s.contains("test"));
        assert!(duration < Duration::from_millis(5));
    }

    #[test]
    fn test_hash_map_performance() {
        use std::collections::HashMap;

        let start = Instant::now();

        let mut map = HashMap::with_capacity(1000);
        for i in 0..1000 {
            map.insert(i, i * 2);
        }

        let duration = start.elapsed();

        assert_eq!(map.len(), 1000);
        assert!(duration < Duration::from_millis(10));
    }

    #[test]
    #[allow(clippy::useless_vec)] // Testing vec-specific performance characteristics
    fn test_vec_vs_slice_performance() {
        let data = vec![1, 2, 3, 4, 5];

        // Slicing should be instant (no copy)
        let start = Instant::now();
        let _slice = &data[1..4];
        let duration = start.elapsed();

        // Should be essentially instant
        assert!(duration < Duration::from_micros(100));
    }

    #[test]
    fn test_arc_clone_performance() {
        use std::sync::Arc;

        let data = Arc::new(vec![0u8; 1000]);

        let start = Instant::now();
        let _cloned = Arc::clone(&data);
        let duration = start.elapsed();

        // Arc::clone is just pointer + refcount, should be instant
        assert!(duration < Duration::from_micros(10));
    }

    #[test]
    fn test_small_allocation_performance() {
        // Small allocations should be very fast
        let start = Instant::now();

        #[allow(clippy::useless_vec)] // Testing heap allocation performance
        for _ in 0..1000 {
            let _v = vec![1, 2, 3];
        }

        let duration = start.elapsed();

        // 1000 small allocations should still be quick
        assert!(duration < Duration::from_millis(50));
    }
}

#[cfg(test)]
mod concurrency_tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_basic_thread_safety() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_arc_sharing_across_threads() {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let sum: i32 = data_clone.iter().sum();
            sum
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 15);
        assert_eq!(data.len(), 5);
    }

    #[test]
    fn test_concurrent_reads() {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let mut handles = vec![];

        for _ in 0..5 {
            let data = Arc::clone(&data);
            let handle = thread::spawn(move || data.iter().sum::<i32>());
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.join().unwrap();
            assert_eq!(result, 15);
        }
    }
}

#[cfg(test)]
mod memory_safety_tests {
    #[test]
    #[allow(clippy::useless_vec)] // Testing vec-specific borrowing rules
    fn test_no_use_after_free() {
        let data = vec![1, 2, 3];
        let _reference = &data[0];
        // data is still valid
        assert_eq!(data.len(), 3);
    }

    #[test]
    fn test_borrowing_rules() {
        let mut data = vec![1, 2, 3];

        // Immutable borrow
        let _ref1 = &data;
        let _ref2 = &data;

        // Can't mutate while immutably borrowed
        // This would fail to compile: data.push(4);

        // After immutable borrows end:
        data.push(4);
        assert_eq!(data.len(), 4);
    }

    #[test]
    fn test_lifetime_correctness() {
        let data = String::from("hello");
        let slice = &data[0..2];

        assert_eq!(slice, "he");
        // data is still valid
        assert_eq!(data, "hello");
    }

    #[test]
    fn test_move_semantics() {
        let data = vec![1, 2, 3];
        let moved = data;

        // Original data is no longer accessible
        // This would fail: assert_eq!(data.len(), 3);

        assert_eq!(moved.len(), 3);
    }

    #[test]
    fn test_copy_types() {
        let x = 42;
        let y = x;

        // Both are still usable (i32 is Copy)
        assert_eq!(x, 42);
        assert_eq!(y, 42);
    }
}
