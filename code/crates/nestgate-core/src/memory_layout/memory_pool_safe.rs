//! SAFE High-performance memory pool - Zero unsafe code!
//!
//! This module provides cache-optimized memory pools with SAFE Rust
//! Demonstrates that performance doesn't require unsafe code.

use super::cache_alignment::CacheAligned;
use std::sync::atomic::{AtomicUsize, Ordering};

/// **SAFE CACHE-OPTIMIZED MEMORY POOL**
///
/// Zero-fragmentation memory pool optimized for cache performance
/// PERFORMANCE: Zero allocation overhead for hot paths
/// SAFETY: 100% safe Rust - no unsafe blocks!
pub struct SafeCacheOptimizedMemoryPool<T, const POOL_SIZE: usize = 1024> {
    /// Pre-allocated memory blocks (using safe Vec instead of raw array)
    blocks: Vec<Option<T>>,
    /// Track which slots are free
    free_list: Vec<usize>,
    /// Pool statistics
    stats: CacheAligned<PoolStats>,
}

impl<T, const POOL_SIZE: usize> Default for SafeCacheOptimizedMemoryPool<T, POOL_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const POOL_SIZE: usize> SafeCacheOptimizedMemoryPool<T, POOL_SIZE> {
    /// Create new memory pool - completely safe!
    #[must_use]
    pub fn new() -> Self {
        Self {
            blocks: (0..POOL_SIZE).map(|_| None).collect(),
            free_list: (0..POOL_SIZE).collect(),
            stats: CacheAligned::new(PoolStats {
                allocated: AtomicUsize::new(0),
                deallocated: AtomicUsize::new(0),
                peak_usage: AtomicUsize::new(0),
                fragmentation_ratio: 0.0,
            }),
        }
    }

    /// Allocate object from pool - completely safe!
    pub fn allocate(&mut self, value: T) -> Option<PoolHandle<T>> {
        // Get next free slot from free list
        let index = self.free_list.pop()?;

        // SAFE: Store value using safe Vec indexing
        self.blocks[index] = Some(value);

        // Update statistics
        self.stats.get().allocated.fetch_add(1, Ordering::Relaxed);
        let current_usage = self.stats.get().allocated.load(Ordering::Relaxed)
            - self.stats.get().deallocated.load(Ordering::Relaxed);

        let peak = self.stats.get().peak_usage.load(Ordering::Relaxed);
        if current_usage > peak {
            self.stats
                .get()
                .peak_usage
                .store(current_usage, Ordering::Relaxed);
        }

        Some(PoolHandle {
            index,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Deallocate object back to pool - completely safe!
    ///
    /// Returns the value if handle was valid, None otherwise
    pub fn deallocate(&mut self, handle: PoolHandle<T>) -> Option<T> {
        if handle.index >= POOL_SIZE {
            return None;
        }

        // SAFE: Take value using safe Vec indexing and Option::take()
        let value = self.blocks.get_mut(handle.index)?.take();

        // Return slot to free list
        self.free_list.push(handle.index);

        // Update statistics
        self.stats.get().deallocated.fetch_add(1, Ordering::Relaxed);

        value
    }

    /// Get pool statistics
    pub fn stats(&self) -> &PoolStats {
        self.stats.get()
    }

    /// Get pool utilization ratio (0.0 to 1.0)
    pub fn utilization(&self) -> f64 {
        let allocated = self.stats.get().allocated.load(Ordering::Relaxed);
        let deallocated = self.stats.get().deallocated.load(Ordering::Relaxed);
        let active = allocated.saturating_sub(deallocated);
        active as f64 / POOL_SIZE as f64
    }
}

/// Handle to allocated object in memory pool
pub struct PoolHandle<T> {
    index: usize,
    _phantom: std::marker::PhantomData<T>,
}

/// Memory pool statistics
pub struct PoolStats {
    /// Total allocations performed
    pub allocated: AtomicUsize,
    /// Total deallocations performed
    pub deallocated: AtomicUsize,
    /// Peak concurrent usage
    pub peak_usage: AtomicUsize,
    /// Memory fragmentation ratio
    pub fragmentation_ratio: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_memory_pool_creation() {
        let pool: SafeCacheOptimizedMemoryPool<u64, 16> = SafeCacheOptimizedMemoryPool::new();
        assert_eq!(pool.utilization(), 0.0);
    }

    #[test]
    fn test_safe_memory_pool_allocation() {
        let mut pool: SafeCacheOptimizedMemoryPool<u64, 16> = SafeCacheOptimizedMemoryPool::new();

        let handle = pool.allocate(42u64);
        assert!(handle.is_some());
        assert!(pool.utilization() > 0.0);
    }

    #[test]
    fn test_safe_memory_pool_deallocation() {
        let mut pool: SafeCacheOptimizedMemoryPool<u64, 16> = SafeCacheOptimizedMemoryPool::new();

        let handle = pool.allocate(42u64).unwrap();
        // SAFE: No unsafe code needed! Just call deallocate
        let value = pool.deallocate(handle);
        assert_eq!(value, Some(42u64));
    }

    #[test]
    fn test_safe_memory_pool_exhaustion() {
        let mut pool: SafeCacheOptimizedMemoryPool<u64, 2> = SafeCacheOptimizedMemoryPool::new();

        let handle1 = pool.allocate(1u64);
        let handle2 = pool.allocate(2u64);
        let handle3 = pool.allocate(3u64);

        assert!(handle1.is_some());
        assert!(handle2.is_some());
        assert!(handle3.is_none()); // Pool exhausted
    }

    #[test]
    fn test_safe_memory_pool_reuse() {
        let mut pool: SafeCacheOptimizedMemoryPool<u64, 2> = SafeCacheOptimizedMemoryPool::new();

        let handle1 = pool.allocate(1u64).unwrap();
        let _handle2 = pool.allocate(2u64).unwrap();

        // Pool exhausted
        assert!(pool.allocate(3u64).is_none());

        // Free a slot
        pool.deallocate(handle1);

        // Can allocate again
        assert!(pool.allocate(3u64).is_some());
    }

    #[test]
    fn test_safe_memory_pool_statistics() {
        let mut pool: SafeCacheOptimizedMemoryPool<u64, 16> = SafeCacheOptimizedMemoryPool::new();

        let _handle = pool.allocate(42u64);
        let stats = pool.stats();

        assert_eq!(stats.allocated.load(Ordering::Relaxed), 1);
        assert_eq!(stats.deallocated.load(Ordering::Relaxed), 0);
    }
}
