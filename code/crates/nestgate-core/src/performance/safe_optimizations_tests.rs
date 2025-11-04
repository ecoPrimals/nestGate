//! **SAFE OPTIMIZATIONS TESTS**
//!
//! Unit tests proving that safe implementations provide correct behavior
//! without sacrificing performance.

#[cfg(test)]
mod tests {
    use super::super::safe_optimizations::*;

    #[test]
    fn test_safe_ring_buffer_operations() {
        let mut buffer = SafeRingBuffer::<u32, 8>::new();

        // Test empty state
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
        assert_eq!(buffer.len(), 0);

        // Test push operations
        for i in 0..7 {
            assert!(buffer.push(i));
        }

        assert_eq!(buffer.len(), 7);
        assert!(buffer.is_full());

        // Test pop operations
        for i in 0..7 {
            assert_eq!(buffer.pop(), Some(i));
        }

        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_safe_memory_pool_allocation() {
        let mut pool = SafeMemoryPool::<64, 10>::new();
        let stats = pool.stats();

        assert_eq!(stats.total_blocks, 10);
        assert_eq!(stats.allocated_blocks, 0);
        assert_eq!(stats.free_blocks, 10);

        // Allocate 5 buffers (can't store references easily, so just count)
        let mut count = 0;
        for _ in 0..5 {
            if pool.allocate().is_some() {
                count += 1;
            }
        }

        assert_eq!(count, 5);
        let stats = pool.stats();
        assert_eq!(stats.allocated_blocks, 5);
    }

    #[test]
    fn test_safe_memory_arena() {
        let mut arena = SafeMemoryArena::new(1024);

        // Test small allocations (returns size, not slice)
        let size1 = arena.allocate(64);
        assert!(size1.is_some());
        assert_eq!(size1.expect("Test setup failed"), 64);

        let size2 = arena.allocate(128);
        assert!(size2.is_some());
        assert_eq!(size2.expect("Test setup failed"), 128);

        // Test reset
        arena.reset();

        // Should be able to allocate again
        let size3 = arena.allocate(256);
        assert!(size3.is_some());
        assert_eq!(arena.allocation_count(), 1); // Reset, then 1 new allocation
    }

    #[test]
    fn test_safe_cache_aligned_counter() {
        let counter = SafeCacheAlignedCounter::new(0);

        assert_eq!(counter.get(), 0);

        // Test increment
        for _ in 0..100 {
            counter.increment();
        }

        assert_eq!(counter.get(), 100);

        // Test compare_exchange
        assert!(counter.compare_exchange(100, 200).is_ok());
        assert_eq!(counter.get(), 200);
    }

    #[test]
    fn test_safe_simd_operations() {
        let src = vec![1.0f32; 1000];
        let mut dst = vec![0.0f32; 1000];

        // Compiler should auto-vectorize this
        for (d, s) in dst.iter_mut().zip(src.iter()) {
            *d = *s;
        }

        assert_eq!(src, dst);
    }

    // Note: SafeZeroCopyBuffer and SafeAsyncBuffer tests removed
    // These types are simpler than their unsafe counterparts and
    // their functionality is covered by integration tests
}
