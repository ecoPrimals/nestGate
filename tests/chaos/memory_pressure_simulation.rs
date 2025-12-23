//! Memory Pressure Simulation Chaos Tests
//!
//! Tests system behavior under memory pressure:
//! - High memory usage scenarios
//! - Memory allocation failures
//! - Cache eviction under pressure
//! - Memory leak detection
//!
//! **MODERN CONCURRENCY**: Uses tokio::time::sleep for realistic allocation timing
//! and yield_now() for cleanup coordination.

#[cfg(test)]
mod memory_pressure_tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Simulated memory manager with configurable limits
    struct MemoryManager {
        allocated: Arc<AtomicUsize>,
        limit: usize,
        allocation_count: Arc<AtomicUsize>,
        deallocation_count: Arc<AtomicUsize>,
    }

    impl MemoryManager {
        fn new(limit: usize) -> Self {
            Self {
                allocated: Arc::new(AtomicUsize::new(0)),
                limit,
                allocation_count: Arc::new(AtomicUsize::new(0)),
                deallocation_count: Arc::new(AtomicUsize::new(0)),
            }
        }

        fn allocate(&self, size: usize) -> Result<MemoryBlock, String> {
            let current = self.allocated.load(Ordering::Relaxed);
            if current + size > self.limit {
                return Err(format!(
                    "Out of memory: requested {} bytes, available {} bytes",
                    size,
                    self.limit - current
                ));
            }

            self.allocated.fetch_add(size, Ordering::Relaxed);
            self.allocation_count.fetch_add(1, Ordering::Relaxed);

            Ok(MemoryBlock {
                size,
                manager: self.allocated.clone(),
                dealloc_counter: self.deallocation_count.clone(),
            })
        }

        fn get_stats(&self) -> (usize, usize, usize) {
            (
                self.allocated.load(Ordering::Relaxed),
                self.allocation_count.load(Ordering::Relaxed),
                self.deallocation_count.load(Ordering::Relaxed),
            )
        }

        fn available(&self) -> usize {
            self.limit - self.allocated.load(Ordering::Relaxed)
        }
    }

    /// Memory block that auto-deallocates on drop
    struct MemoryBlock {
        size: usize,
        manager: Arc<AtomicUsize>,
        dealloc_counter: Arc<AtomicUsize>,
    }

    impl Drop for MemoryBlock {
        fn drop(&mut self) {
            self.manager.fetch_sub(self.size, Ordering::Relaxed);
            self.dealloc_counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Test allocation until memory exhaustion
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_memory_exhaustion() {
        let memory = MemoryManager::new(1024 * 100); // 100 KB limit
        let block_size = 1024; // 1 KB blocks

        let mut blocks = Vec::new();
        let mut successful_allocations = 0;

        // Allocate until we hit the limit
        for _ in 0..150 {
            match memory.allocate(block_size) {
                Ok(block) => {
                    blocks.push(block);
                    successful_allocations += 1;
                }
                Err(_) => break,
            }
        }

        assert_eq!(
            successful_allocations, 100,
            "Should successfully allocate 100 KB"
        );

        // Next allocation should fail
        let result = memory.allocate(block_size);
        assert!(result.is_err(), "Allocation should fail when memory is full");

        // Free half the memory
        blocks.truncate(50);

        // Should be able to allocate again
        let result = memory.allocate(block_size);
        assert!(
            result.is_ok(),
            "Allocation should succeed after freeing memory"
        );

        let (allocated, allocs, deallocs) = memory.get_stats();
        assert_eq!(allocated, 51 * block_size, "Should have 51 KB allocated");
        assert_eq!(allocs, 102, "Should have 102 total allocations");
        assert_eq!(deallocs, 50, "Should have 50 deallocations");
    }

    /// Test cache eviction under memory pressure
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_cache_eviction_under_pressure() {
        let memory = Arc::new(MemoryManager::new(1024 * 50)); // 50 KB limit
        let mut cache: Vec<Option<MemoryBlock>> = Vec::new();

        // Fill cache
        for _ in 0..50 {
            match memory.allocate(1024) {
                Ok(block) => cache.push(Some(block)),
                Err(_) => break,
            }
        }

        assert_eq!(cache.len(), 50, "Cache should be full");

        // Try to allocate more - need to evict
        let eviction_count = 10;
        for i in 0..eviction_count {
            // Evict oldest entries (LRU-style)
            cache[i] = None;
        }

        // Now we should be able to allocate again
        for _ in 0..eviction_count {
            let result = memory.allocate(1024);
            assert!(
                result.is_ok(),
                "Should be able to allocate after eviction"
            );
            cache.push(result.ok());
        }

        let (allocated, _, deallocs) = memory.get_stats();
        assert!(
            allocated <= 50 * 1024,
            "Memory usage should stay within limit"
        );
        assert_eq!(deallocs, eviction_count, "Should evict exact number needed");
    }

    /// Test concurrent allocations under memory pressure
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_concurrent_allocations_under_pressure() {
        let memory = Arc::new(MemoryManager::new(1024 * 100)); // 100 KB limit
        let mut handles = Vec::new();

        // Spawn 20 concurrent tasks trying to allocate
        for i in 0..20 {
            let mem = memory.clone();
            let handle = tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_micros((i * 100) as u64)).await;
                let size = 10 * 1024; // 10 KB each
                mem.allocate(size)
            });
            handles.push(handle);
        }

        let results: Vec<_> = futures::future::join_all(handles).await;

        let successes = results
            .iter()
            .filter(|r| r.is_ok() && r.as_ref().unwrap().is_ok())
            .count();

        let failures = results
            .iter()
            .filter(|r| r.is_err() || r.as_ref().unwrap().is_err())
            .count();

        // Should succeed for 10 allocations (10 * 10KB = 100KB), fail for rest
        assert_eq!(successes, 10, "Should have 10 successful allocations");
        assert_eq!(failures, 10, "Should have 10 failed allocations");
    }

    /// Test memory leak detection
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_memory_leak_detection() {
        let memory = Arc::new(MemoryManager::new(1024 * 100));

        // Simulate operations that should clean up
        for _ in 0..10 {
            let _block = memory.allocate(1024).expect("Allocation should succeed");
            // Block drops here, memory should be freed
        }

        let (allocated, allocs, deallocs) = memory.get_stats();

        assert_eq!(
            allocated, 0,
            "All memory should be freed after blocks drop"
        );
        assert_eq!(allocs, 10, "Should have 10 allocations");
        assert_eq!(deallocs, 10, "Should have 10 deallocations");

        // Verify no leaks
        assert_eq!(
            allocs, deallocs,
            "Allocations should equal deallocations - no leaks"
        );
    }

    /// Test gradual memory pressure increase
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_gradual_memory_pressure() {
        let memory = Arc::new(MemoryManager::new(1024 * 100));
        let mut allocations = Vec::new();

        // Gradually increase memory usage
        for i in 1..=20 {
            let size = i * 1024; // Increasing block sizes
            match memory.allocate(size) {
                Ok(block) => allocations.push(block),
                Err(e) => {
                    tracing::info!("Hit memory limit at iteration {}: {}", i, e);
                    break;
                }
            }

            let available = memory.available();
            tracing::debug!(
                "Iteration {}: Allocated {} KB, Available {} KB",
                i,
                size / 1024,
                available / 1024
            );

            if available < 5 * 1024 {
                tracing::warn!("Memory pressure high: only {} KB available", available / 1024);
            }
        }

        let (allocated, _, _) = memory.get_stats();
        assert!(
            allocated > 50 * 1024,
            "Should have allocated significant memory"
        );
        assert!(
            allocated <= 100 * 1024,
            "Should stay within memory limit"
        );
    }

    /// Test memory fragmentation scenario
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_memory_fragmentation() {
        let memory = Arc::new(MemoryManager::new(1024 * 100));
        let mut allocations = Vec::new();

        // Create fragmented memory pattern
        // Allocate 50 blocks of 1 KB each
        for _ in 0..50 {
            allocations.push(memory.allocate(1024).ok());
        }

        // Free every other block (create holes)
        for i in (0..50).step_by(2) {
            allocations[i] = None;
        }

        let (allocated, _, deallocs) = memory.get_stats();
        assert_eq!(allocated, 25 * 1024, "Should have 25 KB allocated");
        assert_eq!(deallocs, 25, "Should have freed 25 blocks");

        // Try to allocate large contiguous block
        // In real system this might fail due to fragmentation,
        // but our simple allocator doesn't fragment
        let large_block = memory.allocate(25 * 1024);
        assert!(
            large_block.is_ok(),
            "Should be able to allocate after freeing memory"
        );
    }

    /// Test memory pressure with periodic cleanup
    #[tokio::test]
    #[ignore] // Chaos test - run explicitly
    async fn test_memory_pressure_with_cleanup() {
        let memory = Arc::new(MemoryManager::new(1024 * 50));
        let mut allocations = Vec::new();

        // Allocation/cleanup cycle
        for cycle in 0..5 {
            tracing::info!("Cycle {}: Allocating...", cycle);

            // Allocate until 80% full
            while memory.available() > 10 * 1024 {
                match memory.allocate(1024) {
                    Ok(block) => allocations.push(Some(block)),
                    Err(_) => break,
                }
            }

            let (allocated, _, _) = memory.get_stats();
            tracing::info!(
                "Cycle {}: Allocated {} KB, performing cleanup",
                cycle,
                allocated / 1024
            );

            // Cleanup - remove half
            let to_remove = allocations.len() / 2;
            for _ in 0..to_remove {
                allocations.pop();
            }

            tokio::task::yield_now().await;
        }

        // Final stats
        let (allocated, allocs, deallocs) = memory.get_stats();
        tracing::info!(
            "Final: {} KB allocated, {} allocs, {} deallocs",
            allocated / 1024,
            allocs,
            deallocs
        );

        assert!(deallocs > 0, "Should have performed cleanup");
        assert!(
            allocated < 50 * 1024,
            "Memory should be manageable after cleanup cycles"
        );
    }
}

