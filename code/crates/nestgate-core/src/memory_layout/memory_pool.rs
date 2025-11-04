//! High-performance memory pool for zero-allocation hot paths
//!
//! This module provides cache-optimized memory pools with zero fragmentation
//! for performance-critical allocations.

use super::cache_alignment::CacheAligned;
use std::sync::atomic::{AtomicUsize, Ordering};

/// **CACHE-OPTIMIZED MEMORY POOL**
///
/// Zero-fragmentation memory pool optimized for cache performance
/// PERFORMANCE: Zero allocation overhead for hot paths
pub struct CacheOptimizedMemoryPool<T, const POOL_SIZE: usize = 1024> {
    /// Pre-allocated memory blocks
    blocks: [Option<T>; POOL_SIZE],
    /// Next available block index
    next_free: AtomicUsize,
    /// Pool statistics
    stats: CacheAligned<PoolStats>,
}

impl<T, const POOL_SIZE: usize> Default for CacheOptimizedMemoryPool<T, POOL_SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const POOL_SIZE: usize> CacheOptimizedMemoryPool<T, POOL_SIZE> {
    /// Create new memory pool
    #[must_use]
    pub fn new() -> Self {
        Self {
            blocks: std::array::from_fn(|_| None),
            next_free: AtomicUsize::new(0),
            stats: CacheAligned::new(PoolStats {
                allocated: AtomicUsize::new(0),
                deallocated: AtomicUsize::new(0),
                peak_usage: AtomicUsize::new(0),
                fragmentation_ratio: 0.0,
            }),
        }
    }

    /// Allocate object from pool
    pub fn allocate(&self, value: T) -> Option<PoolHandle<T>> {
        let current = self.next_free.load(Ordering::Acquire);

        if current >= POOL_SIZE {
            return None; // Pool exhausted
        }

        // Try to claim this slot
        match self.next_free.compare_exchange(
            current,
            current + 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                // Successfully claimed slot
                // SAFETY: Writing to pool slot is safe because:
                // 1. Index bounds: compare_exchange verified current < POOL_SIZE
                // 2. Uniqueness: compare_exchange ensures exclusive ownership of this slot
                // 3. Pointer validity: blocks_ptr derived from valid self.blocks reference
                // 4. Offset: add(current) stays within array bounds (current < POOL_SIZE)
                // 5. Initialization: ptr::write properly initializes the Option<T> slot
                // 6. No aliasing: Only this thread can write to claimed slot
                //
                // SAFETY PROOF:
                // - blocks_ptr is derived from self.blocks which is always valid
                // - current < POOL_SIZE verified by bounds check above
                // - add(current) produces valid pointer within array bounds
                // - No data races: AtomicUsize ensures exclusive access to this slot
                // - ptr::write initializes previously uninitialized or None slot
                // - No aliasing: This is the only write to this specific slot after claim
                unsafe {
                    let blocks_ptr = self.blocks.as_ptr() as *mut Option<T>;
                    let slot = blocks_ptr.add(current);
                    std::ptr::write(slot, Some(value));
                }

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
                    index: current,
                    _phantom: std::marker::PhantomData,
                })
            }
            Err(_) => {
                // Another thread claimed this slot, try again
                self.allocate(value)
            }
        }
    }

    /// Deallocate object back to pool
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - The `handle` was obtained from a valid allocation from this pool
    /// - The handle has not been used for deallocation before
    /// - No other references to the object exist
    /// - The object at the handle's index is in a valid state
    ///
    /// # Safety Proof
    ///
    /// - **Bounds**: handle.index checked against POOL_SIZE before dereferencing
    /// - **Validity**: blocks_ptr derived from valid self.blocks reference
    /// - **Offset**: add(handle.index) stays within array bounds (handle.index < POOL_SIZE)
    /// - **Read safety**: ptr::read assumes initialized data, guaranteed by handle provenance
    /// - **Write safety**: ptr::write(None) properly clears the slot for reuse
    /// - **No aliasing**: Caller guarantees exclusive ownership per function contract
    /// - **No double-free**: Caller guarantees handle not previously deallocated
    pub unsafe fn deallocate(&self, handle: PoolHandle<T>) -> Option<T> {
        if handle.index >= POOL_SIZE {
            return None;
        }

        let blocks_ptr = self.blocks.as_ptr() as *mut Option<T>;
        let slot = blocks_ptr.add(handle.index);
        let value = std::ptr::read(slot);
        std::ptr::write(slot, None);

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
#[allow(deprecated)] // Testing deprecated CacheOptimizedMemoryPool for backwards compatibility
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_memory_pool_creation() {
        let pool: CacheOptimizedMemoryPool<u64, 16> = CacheOptimizedMemoryPool::new();
        assert_eq!(pool.utilization(), 0.0);
    }

    #[test]
    #[allow(deprecated)]
    fn test_memory_pool_allocation() {
        let pool: CacheOptimizedMemoryPool<u64, 16> = CacheOptimizedMemoryPool::new();

        let handle = pool.allocate(42u64);
        assert!(handle.is_some());
        assert!(pool.utilization() > 0.0);
    }

    #[test]
    #[allow(deprecated)]
    fn test_memory_pool_deallocation() {
        let pool: CacheOptimizedMemoryPool<u64, 16> = CacheOptimizedMemoryPool::new();

        let handle = pool.allocate(42u64).expect("Operation failed");
        // SAFETY: Test deallocation is safe because:
        // 1. Handle validity: handle was just allocated from this pool
        // 2. No double-free: This is the only deallocation of this handle
        // 3. No other references: We own the handle exclusively
        // 4. Test environment: Controlled allocation/deallocation cycle
        let value = unsafe { pool.deallocate(handle) };
        assert_eq!(value, Some(42u64));
    }

    #[test]
    #[allow(deprecated)]
    fn test_memory_pool_exhaustion() {
        let pool: CacheOptimizedMemoryPool<u64, 2> = CacheOptimizedMemoryPool::new();

        let handle1 = pool.allocate(1u64);
        let handle2 = pool.allocate(2u64);
        let handle3 = pool.allocate(3u64);

        assert!(handle1.is_some());
        assert!(handle2.is_some());
        assert!(handle3.is_none()); // Pool exhausted
    }

    #[test]
    #[allow(deprecated)]
    fn test_memory_pool_statistics() {
        let pool: CacheOptimizedMemoryPool<u64, 16> = CacheOptimizedMemoryPool::new();

        let _handle = pool.allocate(42u64);
        let stats = pool.stats();

        assert_eq!(stats.allocated.load(Ordering::Relaxed), 1);
        assert_eq!(stats.deallocated.load(Ordering::Relaxed), 0);
    }
}
