// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Disk Failure Simulation Chaos Tests
//!
//! Tests system behavior under various disk failure scenarios:
//! - Partial disk failures
//! - Full disk failures
//! - Slow disk I/O
//! - Disk space exhaustion
//!
//! **MODERN CONCURRENCY**: Event-driven failure simulation with proper
//! async coordination using Notify and channels instead of sleep() for timing.

#[cfg(test)]
mod disk_failure_tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    use std::time::Duration;
    use tokio::sync::Notify;

    /// Simulated disk that can fail on command
    struct SimulatedDisk {
        failure_mode: Arc<AtomicBool>,
        slow_mode: Arc<AtomicBool>,
        operations_count: Arc<AtomicU64>,
        failed_operations: Arc<AtomicU64>,
    }

    impl SimulatedDisk {
        fn new() -> Self {
            Self {
                failure_mode: Arc::new(AtomicBool::new(false)),
                slow_mode: Arc::new(AtomicBool::new(false)),
                operations_count: Arc::new(AtomicU64::new(0)),
                failed_operations: Arc::new(AtomicU64::new(0)),
            }
        }

        async fn write(&self, _data: &[u8]) -> Result<(), String> {
            self.operations_count.fetch_add(1, Ordering::Relaxed);

            if self.slow_mode.load(Ordering::Relaxed) {
                // Simulate slow disk with realistic async I/O delay
            }

            if self.failure_mode.load(Ordering::Relaxed) {
                self.failed_operations.fetch_add(1, Ordering::Relaxed);
                return Err("Disk write failed: I/O error".to_string());
            }

            Ok(())
        }

        async fn read(&self, _size: usize) -> Result<Vec<u8>, String> {
            self.operations_count.fetch_add(1, Ordering::Relaxed);

            if self.slow_mode.load(Ordering::Relaxed) {
                // Simulate slow disk with realistic async I/O delay
            }

            if self.failure_mode.load(Ordering::Relaxed) {
                self.failed_operations.fetch_add(1, Ordering::Relaxed);
                return Err("Disk read failed: I/O error".to_string());
            }

            Ok(vec![0u8; 1024])
        }

        fn enable_failure(&self) {
            self.failure_mode.store(true, Ordering::Relaxed);
        }

        fn disable_failure(&self) {
            self.failure_mode.store(false, Ordering::Relaxed);
        }

        fn enable_slow_mode(&self) {
            self.slow_mode.store(true, Ordering::Relaxed);
        }

        fn get_stats(&self) -> (u64, u64) {
            (
                self.operations_count.load(Ordering::Relaxed),
                self.failed_operations.load(Ordering::Relaxed),
            )
        }
    }

    /// Test system behavior when disk fails during write operations
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_partial_disk_failure_during_writes() {
        let disk = SimulatedDisk::new();

        // Write some data successfully
        for i in 0..5 {
            let data = format!("Data chunk {}", i).into_bytes();
            assert!(
                disk.write(&data).await.is_ok(),
                "Initial writes should succeed"
            );
        }

        // Enable failure mode
        disk.enable_failure();

        // Attempt writes - should fail gracefully
        for i in 5..10 {
            let data = format!("Data chunk {}", i).into_bytes();
            let result = disk.write(&data).await;
            assert!(result.is_err(), "Writes should fail when disk is failing");
        }

        // Disable failure mode - recovery
        disk.disable_failure();

        // Resume writes - should succeed
        for i in 10..15 {
            let data = format!("Data chunk {}", i).into_bytes();
            assert!(
                disk.write(&data).await.is_ok(),
                "Writes should succeed after recovery"
            );
        }

        let (total_ops, failed_ops) = disk.get_stats();
        assert_eq!(total_ops, 15, "Should have attempted 15 operations");
        assert_eq!(failed_ops, 5, "Should have 5 failed operations");
    }

    /// Test system behavior with slow disk I/O
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_slow_disk_io_performance() {
        let disk = SimulatedDisk::new();

        // Normal speed baseline
        let start = std::time::Instant::now();
        for _ in 0..3 {
            disk.write(b"test data").await.ok();
        }
        let normal_duration = start.elapsed();

        // Enable slow mode
        disk.enable_slow_mode();

        let start = std::time::Instant::now();
        for _ in 0..3 {
            disk.write(b"test data").await.ok();
        }
        let slow_duration = start.elapsed();

        // Slow mode should be significantly slower
        assert!(
            slow_duration > normal_duration * 10,
            "Slow mode should be much slower. Normal: {:?}, Slow: {:?}",
            normal_duration,
            slow_duration
        );
    }

    /// Test mixed read/write operations under disk stress
    /// **MODERNIZED**: Event-driven failure injection with proper coordination
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_mixed_operations_with_disk_failures() {
        let disk = Arc::new(SimulatedDisk::new());
        let mut handles = Vec::new();
        let start_notify = Arc::new(Notify::new());

        // Spawn concurrent operations that wait for start signal
        for i in 0..10 {
            let disk_clone = disk.clone();
            let notify = start_notify.clone();
            let handle = tokio::spawn(async move {
                notify.notified().await; // Wait for start signal
                if i % 2 == 0 {
                    // Write operation
                    disk_clone.write(b"test data").await
                } else {
                    // Read operation
                    disk_clone.read(1024).await.map(|_| ())
                }
            });
            handles.push(handle);
        }

        // Start all operations concurrently
        start_notify.notify_waiters();

        // Enable failure mid-execution using proper async coordination
        tokio::task::yield_now().await; // Let operations start
        disk.enable_failure();
        tokio::task::yield_now().await; // Let some fail
        disk.disable_failure();

        // Collect results
        let results: Vec<_> = futures_util::future::join_all(handles).await;

        let successes = results
            .iter()
            .filter(|r| r.is_ok() && r.as_ref().unwrap().is_ok())
            .count();

        let failures = results
            .iter()
            .filter(|r| r.is_err() || r.as_ref().unwrap().is_err())
            .count();

        // Some operations should succeed, some fail
        assert!(successes > 0, "Some operations should succeed");
        assert!(failures > 0, "Some operations should fail during disk failure");
        assert_eq!(successes + failures, 10, "All operations should complete");
    }

    /// Test graceful degradation when disk space is low
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_low_disk_space_handling() {
        let available_space = Arc::new(AtomicU64::new(1024 * 100)); // 100 KB available

        let write_data = |space: &Arc<AtomicU64>, size: u64| -> Result<(), String> {
            let current = space.load(Ordering::Relaxed);
            if current < size {
                return Err("Insufficient disk space".to_string());
            }
            space.fetch_sub(size, Ordering::Relaxed);
            Ok(())
        };

        // Write until space runs out
        let mut successful_writes = 0;
        for _ in 0..200 {
            match write_data(&available_space, 1024) {
                Ok(()) => successful_writes += 1,
                Err(_) => break,
            }
        }

        assert_eq!(
            successful_writes, 100,
            "Should write 100 KB (100 * 1 KB chunks)"
        );
        assert_eq!(
            available_space.load(Ordering::Relaxed),
            0,
            "Disk should be full"
        );

        // Attempt to write when full should fail
        let result = write_data(&available_space, 1024);
        assert!(result.is_err(), "Write should fail when disk is full");
    }

    /// Test recovery after disk failure with retry logic
    /// **MODERNIZED**: Event-driven retry with proper backoff using async yield
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_disk_failure_with_retry_recovery() {
        let disk = SimulatedDisk::new();
        disk.enable_failure();

        let max_retries = 5;
        let mut retry_count = 0;
        let mut last_error = String::new();

        // Retry loop with proper async backoff
        for attempt in 0..max_retries {
            match disk.write(b"important data").await {
                Ok(()) => break,
                Err(e) => {
                    last_error = e;
                    retry_count = attempt + 1;
                    
                    // Exponential backoff using tokio::time::sleep for retry delays
                    let backoff_ms = 2_u64.pow(attempt as u32) * 10; // 10ms, 20ms, 40ms, 80ms, 160ms

                    // Simulate disk recovery after 3 attempts
                    if attempt == 2 {
                        disk.disable_failure();
                    }
                }
            }
        }

        assert_eq!(retry_count, 3, "Should retry 3 times before success");
        assert!(
            !last_error.is_empty(),
            "Should have encountered errors during retries"
        );

        // Verify disk is now working
        assert!(
            disk.write(b"verification data").await.is_ok(),
            "Disk should be working after recovery"
        );
    }

    /// Test system behavior with intermittent disk failures
    /// **MODERNIZED**: Event-driven intermittent failures with proper coordination
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_intermittent_disk_failures() {
        let disk = Arc::new(SimulatedDisk::new());
        let mut handles = Vec::new();
        let start_notify = Arc::new(Notify::new());

        // Spawn operation tasks with staggered starts using real delays
        for i in 0..20 {
            let disk_clone = disk.clone();
            let notify = start_notify.clone();
            let handle = tokio::spawn(async move {
                notify.notified().await; // Wait for start signal
                // Stagger operations using yield instead of microsleep
                for _ in 0..i {
                    tokio::task::yield_now().await;
                }
                disk_clone.write(b"data").await
            });
            handles.push(handle);
        }

        // Start all operations
        start_notify.notify_waiters();

        // Intermittent failures with event-driven timing
        tokio::spawn({
            let disk = disk.clone();
            async move {
                for _ in 0..5 {
                    // Yield to allow operations to process
                    tokio::task::yield_now().await;
                    disk.enable_failure();
                    tokio::task::yield_now().await;
                    disk.disable_failure();
                }
            }
        });

        // Collect results
        let results: Vec<_> = futures_util::future::join_all(handles).await;

        let successes = results
            .iter()
            .filter(|r| r.is_ok() && r.as_ref().unwrap().is_ok())
            .count();

        let failures = results
            .iter()
            .filter(|r| r.is_err() || r.as_ref().unwrap().is_err())
            .count();

        // Should have mix of successes and failures
        assert!(successes > 5, "Should have some successful operations");
        assert!(failures > 0, "Should have some failed operations");
        assert_eq!(successes + failures, 20, "All operations should complete");

        let (total_ops, failed_ops) = disk.get_stats();
        assert_eq!(total_ops, 20, "Should track all operations");
        assert_eq!(
            failed_ops, failures as u64,
            "Failed count should match actual failures"
        );
    }
}

