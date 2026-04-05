// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! E2E Scenario 26: Concurrency Safety Validation
//!
//! **Purpose**: Validate thread-safe operations and Arc/Mutex patterns
//! **Coverage**: Concurrent access, deadlock prevention, race conditions

#[cfg(test)]
mod concurrency_safety {
    use std::sync::Arc;
    use tokio::sync::{Mutex, RwLock};

    #[tokio::test]
    async fn test_concurrent_reads_rwlock() {
        let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
        let mut handles = vec![];

        // 10 concurrent readers
        for _ in 0..10 {
            let data_clone = Arc::clone(&data);
            let handle = tokio::spawn(async move {
                let guard = data_clone.read().await;
                let sum: i32 = guard.iter().sum();
                assert_eq!(sum, 15);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.expect("Task panicked");
        }
    }

    #[tokio::test]
    async fn test_concurrent_writes_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        // 100 concurrent increments
        for _ in 0..100 {
            let counter_clone = Arc::clone(&counter);
            let handle = tokio::spawn(async move {
                let mut guard = counter_clone.lock().await;
                *guard += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.expect("Task panicked");
        }

        let final_count = *counter.lock().await;
        assert_eq!(final_count, 100);
    }

    #[tokio::test]
    async fn test_rwlock_write_exclusivity() {
        let data = Arc::new(RwLock::new(0));

        // Writer task
        let data_writer = Arc::clone(&data);
        let writer = tokio::spawn(async move {
            let mut guard = data_writer.write().await;
            *guard = 42;
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            *guard = 100;
        });

        // Give writer time to acquire lock
        tokio::time::sleep(tokio::time::Duration::from_millis(25)).await;

        // Reader should wait for writer
        let final_value = *data.read().await;

        writer.await.expect("Writer panicked");
        assert!(final_value == 0 || final_value == 100); // Either before or after write
    }

    #[tokio::test]
    async fn test_arc_reference_counting() {
        let data = Arc::new(vec![1, 2, 3]);
        assert_eq!(Arc::strong_count(&data), 1);

        let clone1 = Arc::clone(&data);
        assert_eq!(Arc::strong_count(&data), 2);

        let clone2 = Arc::clone(&data);
        assert_eq!(Arc::strong_count(&data), 3);

        drop(clone1);
        assert_eq!(Arc::strong_count(&data), 2);

        drop(clone2);
        assert_eq!(Arc::strong_count(&data), 1);
    }
}
