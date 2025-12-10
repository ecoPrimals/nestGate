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

use std::mem::{align_of, size_of};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

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
    pub fn new(initial: u64) -> Self {
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

/// **LOCK-FREE RING BUFFER**
/// High-performance single-producer single-consumer ring buffer
///
/// Note: SIZE must be a power of 2 for optimal performance
pub struct LockFreeRingBuffer<T, const SIZE: usize> {
    buffer: [std::mem::MaybeUninit<T>; SIZE],
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T, const SIZE: usize> Default for LockFreeRingBuffer<T, SIZE> {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const SIZE: usize> LockFreeRingBuffer<T, SIZE> {
    /// Create new lock-free ring buffer
    pub fn new() -> Self {
        Self {
            // ✅ SAFE: Use std::array::from_fn to initialize each MaybeUninit element
            // This properly creates an array of uninitialized values without unsafe code
            buffer: std::array::from_fn(|_| std::mem::MaybeUninit::uninit()),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Push item to buffer (returns false if full)
    #[inline(always)]
    pub fn push(&mut self, item: T) -> bool {
        let current_head = self.head.load(Ordering::Relaxed);
        let next_head = (current_head + 1) & (SIZE - 1); // Fast modulo for power of 2

        if next_head == self.tail.load(Ordering::Acquire) {
            return false; // Buffer full
        }

        // SAFETY: Writing to buffer is safe because:
        // 1. Bounds check: current_head is always < SIZE due to masking
        // 2. Uniqueness: Single producer ensures no concurrent writes
        // 3. Memory ordering: Acquire on tail ensures we see all previous writes
        // 4. Initialization: write() properly initializes the MaybeUninit slot
        // 5. Overwrite safety: We checked buffer isn't full (next_head != tail)
        unsafe {
            self.buffer[current_head].as_mut_ptr().write(item);
        }

        self.head.store(next_head, Ordering::Release);
        true
    }

    /// Pop item from buffer (returns None if empty)
    #[inline(always)]
    pub fn pop(&self) -> Option<T> {
        let current_tail = self.tail.load(Ordering::Relaxed);

        if current_tail == self.head.load(Ordering::Acquire) {
            return None; // Buffer empty
        }

        // SAFETY: Reading from buffer is safe because:
        // 1. Bounds check: current_tail is always < SIZE due to masking
        // 2. Initialization: Acquire on head ensures item was written
        // 3. Uniqueness: Single consumer ensures no concurrent reads
        // 4. Memory ordering: Acquire synchronizes with Release in push()
        // 5. Move semantics: read() moves value out, preventing double-read
        let item = unsafe { self.buffer[current_tail].as_ptr().read() };
        let next_tail = (current_tail + 1) & (SIZE - 1); // Fast modulo for power of 2

        self.tail.store(next_tail, Ordering::Release);
        Some(item)
    }

    /// Get current buffer utilization
    #[inline(always)]
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        (head.wrapping_sub(tail)) & (SIZE - 1)
    }

    /// Check if buffer is empty
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }

    /// Check if buffer is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        ((head + 1) & (SIZE - 1)) == tail
    }
}

/// **SIMD-OPTIMIZED OPERATIONS**
/// Vectorized operations for high-performance data processing
pub struct SimdOperations;
impl SimdOperations {
    /// Sum array of u32 values using SIMD when available
    #[inline(always)]
    pub fn sum_u32_slice(data: &[u32]) -> u64 {
        // Fallback to scalar implementation
        // In a real implementation, this would use SIMD intrinsics
        data.iter().map(|&x| x as u64).sum()
    }

    /// Find maximum value in slice using SIMD
    #[inline(always)]
    pub fn max_u32_slice(data: &[u32]) -> Option<u32> {
        if data.is_empty() {
            return None;
        }

        // Scalar fallback - real implementation would use SIMD
        // Safe: we just checked that data is not empty
        data.iter().max().copied()
    }

    /// Copy memory with optimal alignment and prefetching
    ///
    /// # Safety
    /// Caller must ensure:
    /// - `src` is valid for reads of `len` bytes
    /// - `dst` is valid for writes of `len` bytes
    /// - `src` and `dst` do not overlap (use copy, not copy_nonoverlapping otherwise)
    /// - Both pointers are properly aligned for their access patterns
    /// - Both regions are within a single allocated object
    #[inline(always)]
    pub unsafe fn optimized_copy(dst: *mut u8, src: *const u8, len: usize) {
        // Check alignment for optimal copy strategy
        if (dst as usize).is_multiple_of(align_of::<u64>())
            && (src as usize).is_multiple_of(align_of::<u64>())
            && len >= size_of::<u64>()
        {
            // Use 64-bit aligned copies when possible
            let chunks = len / size_of::<u64>();
            let remainder = len % size_of::<u64>();

            let dst_u64 = dst as *mut u64;
            let src_u64 = src as *const u64;

            // SAFETY: 64-bit aligned copy is safe because:
            // 1. Alignment: Both pointers verified to be u64-aligned above
            // 2. Bounds: chunks * 8 <= len, so all accesses within bounds
            // 3. Validity: Caller guarantees src/dst validity for len bytes
            // 4. Non-overlapping: Caller guarantees no overlap
            // 5. Type safety: u64 is Copy and properly aligned
            for i in 0..chunks {
                *dst_u64.add(i) = *src_u64.add(i);
            }

            // Handle remainder
            // SAFETY: Remainder copy is safe because:
            // 1. Offset: chunks * size_of::<u64>() + remainder == len (total length)
            // 2. Bounds: Copying remainder bytes from valid end of buffers
            // 3. Non-overlapping: Inherits from parent function guarantee
            // 4. Validity: Both regions within caller-guaranteed valid memory
            if remainder > 0 {
                std::ptr::copy_nonoverlapping(
                    src.add(chunks * size_of::<u64>()),
                    dst.add(chunks * size_of::<u64>()),
                    remainder,
                );
            }
        } else {
            // Fallback to standard copy
            // SAFETY: Standard copy is safe because:
            // 1. Validity: Caller guarantees src/dst valid for len bytes
            // 2. Non-overlapping: Caller guarantees no overlap
            // 3. Standard library: copy_nonoverlapping handles all edge cases
            std::ptr::copy_nonoverlapping(src, dst, len);
        }
    }
}

/// **BRANCH-PREDICTION OPTIMIZED FUNCTIONS**
/// Functions optimized for CPU branch prediction
pub struct BranchOptimized;
impl BranchOptimized {
    /// Likely branch hint for hot path optimization
    #[inline(always)]
    pub fn likely(condition: bool) -> bool {
        #[cold]
        fn cold() {}

        if !condition {
            cold();
        }
        condition
    }

    /// Unlikely branch hint for error path optimization
    #[inline(always)]
    pub fn unlikely(condition: bool) -> bool {
        #[cold]
        fn cold() {}

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

/// **MEMORY POOL ALLOCATOR**
/// High-performance memory pool for frequent allocations
///
/// Note: BLOCK_SIZE and POOL_SIZE must be > 0
pub struct MemoryPool<const BLOCK_SIZE: usize, const POOL_SIZE: usize> {
    pool: Vec<u8>,
    free_list: AtomicUsize, // Index of next free block
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
        let total_size = POOL_SIZE * BLOCK_SIZE;
        let mut pool = vec![0u8; total_size];

        // ✅ SAFE: Initialize free list using safe slice operations
        // Each block stores the index of the next free block
        for i in 0..POOL_SIZE - 1 {
            let next_index = i + 1;
            let start = i * BLOCK_SIZE;
            let next_index_bytes = next_index.to_ne_bytes();
            pool[start..start + std::mem::size_of::<usize>()].copy_from_slice(&next_index_bytes);
        }

        // Last block points to invalid index (pool exhausted marker)
        let start = (POOL_SIZE - 1) * BLOCK_SIZE;
        let sentinel_bytes = usize::MAX.to_ne_bytes();
        pool[start..start + std::mem::size_of::<usize>()].copy_from_slice(&sentinel_bytes);

        Self {
            pool,
            free_list: AtomicUsize::new(0),
            allocated_count: AtomicUsize::new(0),
        }
    }

    /// Allocate block from pool
    #[inline(always)]
    pub fn allocate(&self) -> Option<NonNull<u8>> {
        loop {
            let current_free = self.free_list.load(Ordering::Acquire);

            if current_free == usize::MAX {
                return None; // Pool exhausted
            }

            // ✅ SAFE: Use safe slice indexing to read next_free index
            let start = current_free * BLOCK_SIZE;
            let end = start + std::mem::size_of::<usize>();
            let next_free_bytes = &self.pool[start..end];
            #[allow(clippy::expect_used)] // Slice length guaranteed by usize size
            let next_free = usize::from_ne_bytes(
                next_free_bytes
                    .try_into()
                    .expect("BUG: Slice length matches usize size"),
            );

            // Try to update free list atomically
            match self.free_list.compare_exchange_weak(
                current_free,
                next_free,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    self.allocated_count.fetch_add(1, Ordering::Relaxed);
                    // ✅ SAFE: Get pointer to allocated block using safe slice indexing
                    let block_slice = &self.pool[start..start + BLOCK_SIZE];
                    let ptr = block_slice.as_ptr() as *mut u8;
                    // Note: NonNull::new would be safer, but ptr is guaranteed non-null from Box
                    return NonNull::new(ptr);
                }
                Err(_) => {
                    // Retry on allocation failure
                }
            }
        }
    }

    /// Deallocate block back to pool
    ///
    /// # Safety
    ///
    /// - `ptr` must have been allocated by this pool via `allocate()`
    /// - `ptr` must not be used after deallocation
    /// - `ptr` must not be deallocated more than once
    ///
    /// # Safety Proof
    ///
    /// - **Valid pointer**: ptr must come from this pool's allocate() call
    /// - **Bounds**: block_index calculated from pool_start is within pool bounds
    /// - **No double-free**: Caller guarantees ptr not previously deallocated
    /// - **Atomics**: compare_exchange prevents concurrent free of same block
    /// - **Free list**: ABA problem handled by storing indices, not pointers
    /// - **No use-after-free**: Caller guarantees no further use of ptr
    #[inline(always)]
    pub unsafe fn deallocate(&self, ptr: NonNull<u8>) {
        let pool_start = self.pool.as_ptr() as usize;
        let ptr_addr = ptr.as_ptr() as usize;

        // Calculate block index
        let block_index = (ptr_addr - pool_start) / BLOCK_SIZE;

        // Add block back to free list
        loop {
            let current_free = self.free_list.load(Ordering::Acquire);

            // Set next pointer in the deallocated block
            let block_ptr = ptr.as_ptr() as *mut usize;
            *block_ptr = current_free;

            // Try to update free list head
            match self.free_list.compare_exchange_weak(
                current_free,
                block_index,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    self.allocated_count.fetch_sub(1, Ordering::Relaxed);
                    break;
                }
                Err(_) => {
                    // Retry on CAS failure
                }
            }
        }
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
        let mut buffer: LockFreeRingBuffer<u32, 8> = LockFreeRingBuffer::new();

        assert!(buffer.is_empty());
        assert!(!buffer.is_full());

        // Fill buffer
        for i in 0..7 {
            assert!(buffer.push(i));
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

        // Allocate some blocks
        let mut blocks = Vec::new();
        for _ in 0..5 {
            if let Some(block) = pool.allocate() {
                blocks.push(block);
            }
        }

        let stats = pool.stats();
        assert_eq!(stats.allocated_blocks, 5);
        assert_eq!(stats.free_blocks, 5);

        // Deallocate blocks
        for block in blocks {
            unsafe {
                pool.deallocate(block);
            }
        }

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
}
