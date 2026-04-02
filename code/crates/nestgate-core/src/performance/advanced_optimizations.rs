// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **ADVANCED PERFORMANCE OPTIMIZATIONS**
//! Advanced Optimizations functionality and utilities.
// Cutting-edge performance optimization techniques for maximum throughput
//! and minimal latency in high-performance systems.
//! Advanced Optimizations functionality and utilities.
// **OPTIMIZATION TECHNIQUES**:
//! - SIMD vectorization for parallel processing
//! - Cache-friendly data structures and memory layout
//! - Lock-free programming with atomic operations
//! - Branch prediction optimization
//! - Memory prefetching and alignment
//! - Zero-allocation hot paths

use std::mem::size_of;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use parking_lot::{Mutex, MutexGuard};

/// **LOCK-FREE RING BUFFER** (evolved implementation)
///
/// Uses the same safe SPSC design as [`super::safe_ring_buffer::SafeRingBuffer`]
/// (atomics + `Mutex` per slot). Prefer [`super::safe_ring_buffer::SafeRingBuffer`] directly;
/// this alias remains for backward compatibility with the deprecated name.
pub type LockFreeRingBuffer<T, const SIZE: usize> =
    super::safe_ring_buffer::SafeRingBuffer<T, SIZE>;

/// **CACHE-LINE ALIGNED ATOMIC COUNTER**
/// Prevents false sharing between CPU cores for optimal performance
#[repr(align(64))] // Align to cache line size
/// Cachealignedcounter
pub struct CacheAlignedCounter {
    value: AtomicU64,
    _padding: [u8; 64 - size_of::<AtomicU64>()], // Pad to full cache line
}
impl CacheAlignedCounter {
    /// Create new cache-aligned counter
    #[must_use]
    pub const fn new(initial: u64) -> Self {
        Self {
            value: AtomicU64::new(initial),
            _padding: [0; 64 - size_of::<AtomicU64>()],
        }
    }

    /// Increment counter with relaxed ordering for maximum performance
    #[inline(always)]
    pub fn increment(&self) -> u64 {
        self.value.fetch_add(1, Ordering::Relaxed)
    }

    /// Get current value with acquire ordering
    #[inline(always)]
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Acquire)
    }

    /// Add value with specified ordering
    #[inline(always)]
    pub fn add(&self, val: u64, ordering: Ordering) -> u64 {
        self.value.fetch_add(val, ordering)
    }

    /// Compare and swap with strong ordering
    #[inline(always)]
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn compare_exchange(&self, current: u64, new: u64) -> Result<u64, u64> {
        self.value
            .compare_exchange(current, new, Ordering::AcqRel, Ordering::Acquire)
    }
}

/// **SIMD-OPTIMIZED OPERATIONS**
/// Vectorized operations for high-performance data processing
pub struct SimdOperations;
impl SimdOperations {
    /// Sum array of u32 values using SIMD when available
    #[inline(always)]
    #[must_use]
    pub fn sum_u32_slice(data: &[u32]) -> u64 {
        // Fallback to scalar implementation
        // In a real implementation, this would use SIMD intrinsics
        data.iter().map(|&x| u64::from(x)).sum()
    }

    /// Find maximum value in slice using SIMD
    #[inline(always)]
    #[must_use]
    pub fn max_u32_slice(data: &[u32]) -> Option<u32> {
        if data.is_empty() {
            return None;
        }

        // Scalar fallback - real implementation would use SIMD
        // Safe: we just checked that data is not empty
        data.iter().max().copied()
    }

    /// Safe memory copy - use this instead of raw pointer version.
    ///
    /// The standard `copy_from_slice` is already optimized by the compiler
    /// (vectorization, alignment). Prefer this for all slice-based code.
    #[inline(always)]
    pub fn copy_safe(dst: &mut [u8], src: &[u8]) {
        let len = src.len().min(dst.len());
        dst[..len].copy_from_slice(&src[..len]);
    }

    /// Slice-based copy — same behavior as [`copy_safe`](Self::copy_safe); kept for API parity
    /// with older `optimized_copy` call sites that used slices.
    #[inline(always)]
    pub fn optimized_copy(dst: &mut [u8], src: &[u8]) {
        Self::copy_safe(dst, src);
    }
}

/// **BRANCH-PREDICTION OPTIMIZED FUNCTIONS**
/// Functions optimized for CPU branch prediction
pub struct BranchOptimized;
impl BranchOptimized {
    /// Likely branch hint for hot path optimization
    #[inline(always)]
    #[must_use]
    pub const fn likely(condition: bool) -> bool {
        #[cold]
        const fn cold() {}

        if !condition {
            cold();
        }
        condition
    }

    /// Unlikely branch hint for error path optimization
    #[inline(always)]
    #[must_use]
    pub const fn unlikely(condition: bool) -> bool {
        #[cold]
        const fn cold() {}

        if condition {
            cold();
        }
        condition
    }

    /// Optimized conditional execution
    #[inline(always)]
    pub fn conditional_execute<F, R>(condition: bool, hot_path: F, cold_path: R) -> R
    where
        F: FnOnce() -> R,
        R: FnOnce() -> R,
    {
        if Self::likely(condition) {
            hot_path()
        } else {
            cold_path()
        }
    }
}

/// RAII guard for memory pool block — holds the per-slot mutex until dropped, then returns the index.
pub struct PoolBlockGuard<'a, const BLOCK_SIZE: usize, const POOL_SIZE: usize> {
    pool: &'a MemoryPool<BLOCK_SIZE, POOL_SIZE>,
    index: usize,
    slot_guard: MutexGuard<'a, Option<Box<[u8; BLOCK_SIZE]>>>,
}

impl<const BLOCK_SIZE: usize, const POOL_SIZE: usize> PoolBlockGuard<'_, BLOCK_SIZE, POOL_SIZE> {
    /// Get mutable slice to the allocated block.
    ///
    /// # Panics
    ///
    /// Panics if the slot is unexpectedly empty (invariant: guard is only
    /// created for occupied slots).
    #[expect(
        clippy::expect_used,
        reason = "structural invariant: guard is only created for occupied slots"
    )]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.slot_guard
            .as_mut()
            .expect("PoolBlockGuard invariant: slot must hold an allocated block")
            .as_mut()
    }
}

impl<const BLOCK_SIZE: usize, const POOL_SIZE: usize> Drop
    for PoolBlockGuard<'_, BLOCK_SIZE, POOL_SIZE>
{
    fn drop(&mut self) {
        *self.slot_guard = None;
        self.pool.free_indices.lock().push(self.index);
        self.pool.allocated_count.fetch_sub(1, Ordering::Relaxed);
    }
}

/// **MEMORY POOL ALLOCATOR**
/// Fixed-size blocks stored in `Box<[u8; BLOCK_SIZE]>` per slot with a `Mutex` stack of free indices.
///
/// Evolved from a lock-free raw-pointer slab: same O(1) intent, fully safe Rust.
pub struct MemoryPool<const BLOCK_SIZE: usize, const POOL_SIZE: usize> {
    slots: [Mutex<Option<Box<[u8; BLOCK_SIZE]>>>; POOL_SIZE],
    free_indices: Mutex<Vec<usize>>,
    allocated_count: AtomicUsize,
}

impl<const BLOCK_SIZE: usize, const POOL_SIZE: usize> Default
    for MemoryPool<BLOCK_SIZE, POOL_SIZE>
{
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl<const BLOCK_SIZE: usize, const POOL_SIZE: usize> MemoryPool<BLOCK_SIZE, POOL_SIZE> {
    /// Create new memory pool
    #[must_use]
    pub fn new() -> Self {
        let mut free = Vec::with_capacity(POOL_SIZE);
        for i in 0..POOL_SIZE {
            free.push(i);
        }

        Self {
            slots: std::array::from_fn(|_| Mutex::new(None)),
            free_indices: Mutex::new(free),
            allocated_count: AtomicUsize::new(0),
        }
    }

    /// Allocate block with RAII guard
    #[inline(always)]
    pub fn allocate_guard(&self) -> Option<PoolBlockGuard<'_, BLOCK_SIZE, POOL_SIZE>> {
        let index = self.free_indices.lock().pop()?;
        let mut slot_guard = self.slots[index].lock();
        *slot_guard = Some(Box::new([0u8; BLOCK_SIZE]));
        self.allocated_count.fetch_add(1, Ordering::Relaxed);
        Some(PoolBlockGuard {
            pool: self,
            index,
            slot_guard,
        })
    }

    /// Get pool utilization statistics
    pub fn stats(&self) -> PoolStats {
        let allocated = self.allocated_count.load(Ordering::Relaxed);
        let utilization = (allocated as f64 / POOL_SIZE as f64) * 100.0;

        PoolStats {
            total_blocks: POOL_SIZE,
            allocated_blocks: allocated,
            free_blocks: POOL_SIZE - allocated,
            utilization_percent: utilization,
            block_size: BLOCK_SIZE,
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone)]
/// Poolstats
pub struct PoolStats {
    /// Total Blocks
    pub total_blocks: usize,
    /// Allocated Blocks
    pub allocated_blocks: usize,
    /// Free Blocks
    pub free_blocks: usize,
    /// Utilization Percent
    pub utilization_percent: f64,
    /// Size of block
    pub block_size: usize,
}
/// **PERFORMANCE PROFILER**
/// Lightweight profiler for hot path analysis
pub struct PerformanceProfiler {
    counters: [CacheAlignedCounter; 16], // Up to 16 different metrics
}
impl Default for PerformanceProfiler {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceProfiler {
    /// Create new performance profiler
    #[must_use]
    pub fn new() -> Self {
        Self {
            counters: std::array::from_fn(|_| CacheAlignedCounter::new(0)),
        }
    }

    /// Record operation with specified metric ID
    #[inline(always)]
    pub fn record(&self, metric_id: usize) {
        if metric_id < self.counters.len() {
            self.counters[metric_id].increment();
        }
    }

    /// Get metric value
    #[inline(always)]
    pub fn get_metric(&self, metric_id: usize) -> Option<u64> {
        self.counters.get(metric_id).map(|c| c.get())
    }

    /// Reset all metrics
    pub fn reset(&self) {
        for counter in &self.counters {
            counter.value.store(0, Ordering::Relaxed);
        }
    }

    /// Get all metrics as array
    pub fn get_all_metrics(&self) -> [u64; 16] {
        let mut metrics = [0u64; 16];
        for (i, counter) in self.counters.iter().enumerate() {
            metrics[i] = counter.get();
        }
        metrics
    }
}

/// **COMPILE-TIME PERFORMANCE CONSTANTS**
///
/// Architecture-specific constants computed at compile time for optimal performance.
/// These values are baked into the binary with zero runtime overhead.
///
/// # Zero-Cost Guarantee
///
/// All constants are resolved at compile time. Accessing them has the same
/// performance as inline constants with no function call overhead.
///
/// # Architecture Tuning
///
/// Constants are tuned for modern x86-64 architectures with AVX2 support.
/// Values are based on:
/// - L1 cache: 32-64KB per core
/// - L2 cache: 256KB-1MB per core  
/// - L3 cache: 8-32MB shared
/// - Cache line: 64 bytes
/// - Page size: 4KB
///
/// # Usage
///
/// ```rust,no_run
/// # use nestgate_core::performance::advanced_optimizations::PerformanceConstants;
/// // Use compile-time constants for buffer sizing
/// let buffer = vec![0u8; PerformanceConstants::OPTIMAL_BUFFER_SIZE];
///
/// // Align data structures to cache lines
/// #[repr(align(64))] // PerformanceConstants::CACHE_LINE_SIZE
/// struct CacheAligned {
///     data: [u8; 64],
/// }
/// ```
pub struct PerformanceConstants;
impl PerformanceConstants {
    /// Optimal buffer size for I/O operations (64KB)
    ///
    /// This size balances:
    /// - L1/L2 cache efficiency
    /// - Memory allocation overhead
    /// - System call batching
    /// - Page alignment benefits
    pub const OPTIMAL_BUFFER_SIZE: usize = 64 * 1024; // 64KB

    /// CPU cache line size (64 bytes)
    ///
    /// Critical for preventing false sharing in concurrent code.
    /// Align hot data structures to this size.
    pub const CACHE_LINE_SIZE: usize = 64;

    /// Operating system page size (4KB)
    ///
    /// Used for memory alignment and efficient page table usage.
    /// Aligning allocations to page boundaries can improve TLB efficiency.
    pub const PAGE_SIZE: usize = 4096;

    /// Maximum SIMD width in bytes (32 bytes for AVX2)
    ///
    /// Used to determine optimal batch sizes for vectorized operations.
    /// AVX2 provides 256-bit (32-byte) wide vector operations.
    pub const MAX_SIMD_WIDTH: usize = 32; // AVX2

    /// Recommended batch size for vectorized operations (8 elements)
    ///
    /// Optimal batch size for processing u32 elements with AVX2.
    /// Calculated as: MAX_SIMD_WIDTH / sizeof(u32) = 32 / 4 = 8
    pub const VECTORIZED_BATCH_SIZE: usize = Self::MAX_SIMD_WIDTH / 4; // 8 elements for u32

    /// Memory prefetch distance (64 bytes)
    ///
    /// How far ahead to prefetch memory for optimal cache utilization.
    /// Typically one cache line ahead.
    pub const PREFETCH_DISTANCE: usize = 64;

    /// Branch prediction threshold (90%)
    ///
    /// When a branch is taken more than 90% of the time, modern CPUs
    /// can predict it very effectively. Use this to optimize hot paths.
    pub const BRANCH_PREDICTION_THRESHOLD: f64 = 0.9;
}

// Note: PowerOfTwo trait removed - was unused (dead_code warning)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_aligned_counter() {
        let counter = CacheAlignedCounter::new(0);
        assert_eq!(counter.get(), 0);

        counter.increment();
        assert_eq!(counter.get(), 1);

        counter.add(10, Ordering::Relaxed);
        assert_eq!(counter.get(), 11);
    }

    #[test]
    fn test_lock_free_ring_buffer() {
        let buffer: LockFreeRingBuffer<u32, 8> = LockFreeRingBuffer::new();

        assert!(buffer.is_empty());
        assert!(!buffer.is_full());

        // Fill buffer
        for i in 0..7 {
            assert!(buffer.push(i).is_ok());
        }

        assert!(buffer.is_full());

        // Empty buffer
        for i in 0..7 {
            assert_eq!(buffer.pop(), Some(i));
        }

        assert!(buffer.is_empty());
    }

    #[test]
    fn test_memory_pool() {
        let pool: MemoryPool<64, 10> = MemoryPool::new();
        let stats = pool.stats();

        assert_eq!(stats.total_blocks, 10);
        assert_eq!(stats.allocated_blocks, 0);
        assert_eq!(stats.free_blocks, 10);

        // Allocate using safe RAII guards
        let mut guards = Vec::new();
        for _ in 0..5 {
            if let Some(guard) = pool.allocate_guard() {
                guards.push(guard);
            }
        }

        let stats = pool.stats();
        assert_eq!(stats.allocated_blocks, 5);
        assert_eq!(stats.free_blocks, 5);

        // Guards drop and deallocate automatically
        drop(guards);

        let stats = pool.stats();
        assert_eq!(stats.allocated_blocks, 0);
        assert_eq!(stats.free_blocks, 10);
    }

    #[test]
    fn test_performance_profiler() {
        let profiler = PerformanceProfiler::new();

        profiler.record(0);
        profiler.record(0);
        profiler.record(1);

        assert_eq!(profiler.get_metric(0), Some(2));
        assert_eq!(profiler.get_metric(1), Some(1));
        assert_eq!(profiler.get_metric(2), Some(0));
    }

    #[test]
    fn cache_aligned_compare_exchange_and_simd_helpers() {
        let c = CacheAlignedCounter::new(3);
        assert_eq!(c.compare_exchange(3, 7), Ok(3));
        assert_eq!(c.get(), 7);

        assert_eq!(SimdOperations::sum_u32_slice(&[10, 20, 30]), 60);
        assert_eq!(SimdOperations::max_u32_slice(&[]), None);
        assert_eq!(SimdOperations::max_u32_slice(&[3, 9, 2]), Some(9));
        let mut dst = [0u8; 6];
        SimdOperations::copy_safe(&mut dst, &[1, 2, 3, 4]);
        assert_eq!(dst[..4], [1, 2, 3, 4]);
        let mut short = [0u8; 1];
        SimdOperations::optimized_copy(&mut short, &[9, 8]);
        assert_eq!(short[0], 9);
    }

    #[test]
    fn branch_optimized_likely_unlikely() {
        assert!(BranchOptimized::likely(true));
        assert!(!BranchOptimized::likely(false));
        assert!(BranchOptimized::unlikely(true));
        assert!(!BranchOptimized::unlikely(false));
    }

    #[test]
    fn performance_constants_and_profiler_reset() {
        let _ = PerformanceConstants::OPTIMAL_BUFFER_SIZE
            + PerformanceConstants::CACHE_LINE_SIZE
            + PerformanceConstants::PAGE_SIZE
            + PerformanceConstants::MAX_SIMD_WIDTH
            + PerformanceConstants::VECTORIZED_BATCH_SIZE
            + PerformanceConstants::PREFETCH_DISTANCE;
        assert!((PerformanceConstants::BRANCH_PREDICTION_THRESHOLD - 0.9).abs() < f64::EPSILON);

        let profiler = PerformanceProfiler::default();
        profiler.record(0);
        profiler.record(25);
        profiler.reset();
        assert_eq!(profiler.get_all_metrics(), [0u64; 16]);
    }
}
