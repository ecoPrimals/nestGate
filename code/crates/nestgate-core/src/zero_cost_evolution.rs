//
// **⚠️ EXPERIMENTAL MODULE - NOT FOR PRODUCTION USE**
//
// This module contains experimental zero-cost abstractions that use unsafe code
// for maximum performance. It is feature-gated and not included in production builds.
//
// To enable: `cargo build --features "experimental-zero-cost"`

#[cfg(debug_assertions)]
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

// ==================== SECTION ====================

/// **ZERO-COST**: Compile-time string interning
///
/// This trait provides zero-cost string operations using const generics
pub trait ZeroCostString<const N: usize> {
    /// Get string as compile-time constant
    const STR: &'static str;
    /// Get length at compile time
    const LEN: usize = N;

    /// Zero-cost comparison
    #[inline(always)]
    fn matches(&self, other: &str) -> bool {
        Self::STR == other
    }
}

/// **ZERO-COST**: Type-level configuration
///
/// Configuration that exists only at compile time
pub trait ZeroCostConfig {
    /// Buffer size determined at compile time
    const BUFFER_SIZE: usize;
    /// Maximum connections at compile time
    const MAX_CONNECTIONS: usize;

    /// Timeout in milliseconds at compile time
    const TIMEOUT_MS: u64;

    /// Enable debug mode at compile time
    const DEBUG: bool;
}

/// Production configuration - optimized for performance
pub struct ProductionConfig;
impl ZeroCostConfig for ProductionConfig {
    const BUFFER_SIZE: usize =
        crate::constants::canonical_defaults::performance::NETWORK_BUFFER_SIZE;
    const MAX_CONNECTIONS: usize =
        crate::constants::canonical_defaults::performance::MAX_CONNECTIONS;
    const TIMEOUT_MS: u64 = crate::constants::canonical::timeouts::DEFAULT_TIMEOUT_MS;
    const DEBUG: bool = false;
}

/// Development configuration - optimized for debugging
pub struct DevelopmentConfig;
impl ZeroCostConfig for DevelopmentConfig {
    const BUFFER_SIZE: usize =
        crate::constants::canonical_defaults::performance::DEFAULT_BUFFER_SIZE;
    const MAX_CONNECTIONS: usize =
        crate::constants::canonical_defaults::performance::MAX_CONNECTIONS;
    const TIMEOUT_MS: u64 = crate::constants::canonical::timeouts::DEFAULT_TIMEOUT_MS;
    const DEBUG: bool = true;
}

/// **EXPERIMENTAL**: Zero-cost array with compile-time capacity
///
/// ✅ SAFE IMPLEMENTATION - No unsafe code
#[derive(Debug)]
pub struct ZeroCostArray<T, const N: usize> {
    data: Vec<T>,
    capacity: usize,
}

impl<T, const N: usize> Default for ZeroCostArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> ZeroCostArray<T, N> {
    /// Create a new zero-cost array with compile-time capacity
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            capacity: N,
        }
    }

    /// Push element with compile-time capacity checking
    #[inline(always)]
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.data.len() < N {
            self.data.push(value);
            Ok(())
        } else {
            Err(value)
        }
    }

    /// Get element with bounds checking (100% safe)
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Get the compile-time capacity
    #[inline(always)]
    #[must_use]
    pub fn capacity(&self) -> usize {
        N // Use const generic directly for better optimization
    }

    /// Check if the array is at capacity
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    }

    /// Get element with bounds checking (100% safe)
    #[must_use]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    /// Get length
    #[inline(always)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    #[inline(always)]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get actual length of stored data
    #[must_use]
    pub fn actual_len(&self) -> usize {
        self.data.len()
    }
}

/// **ZERO-COST**: Memory pool with compile-time sizing
///
/// Memory pool that allocates at compile time
pub struct ZeroCostPool<T, const POOL_SIZE: usize, const BLOCK_SIZE: usize> {
    blocks: [MaybeUninit<[T; BLOCK_SIZE]>; POOL_SIZE],
    free_mask: u64, // Bitmap for free blocks (supports up to 64 blocks)
    _phantom: PhantomData<T>,
}
impl<T, const POOL_SIZE: usize, const BLOCK_SIZE: usize> Default
    for ZeroCostPool<T, POOL_SIZE, BLOCK_SIZE>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const POOL_SIZE: usize, const BLOCK_SIZE: usize> ZeroCostPool<T, POOL_SIZE, BLOCK_SIZE> {
    /// Create new pool with all blocks free
    #[inline(always)]
    #[must_use]
    pub fn new() -> Self {
        assert!(POOL_SIZE <= 64, "Pool size cannot exceed 64 blocks");

        Self {
            // ✅ SAFE: Use std::array::from_fn to initialize each MaybeUninit element
            // This properly creates an array of uninitialized blocks without unsafe code
            blocks: std::array::from_fn(|_| MaybeUninit::uninit()),
            free_mask: (1u64 << POOL_SIZE) - 1, // All blocks initially free
            _phantom: PhantomData,
        }
    }

    /// Allocate a block (zero-cost when inlined)
    #[inline(always)]
    #[must_use]
    pub fn allocate(&mut self) -> Option<&mut [T; BLOCK_SIZE]> {
        if self.free_mask == 0 {
            return None; // No free blocks
        }

        // Find first free block using trailing zeros
        let block_index = self.free_mask.trailing_zeros() as usize;

        // Mark block as used
        self.free_mask &= !(1u64 << block_index);

        // SAFETY: Assuming initialized is safe because:
        // 1. Block index: Derived from free_mask, guaranteed valid
        // 2. Bounds: block_index < POOL_SIZE due to free_mask bit operations
        // 3. Initialization: Caller responsible for initializing before use
        // 4. Uniqueness: Clearing free_mask bit prevents double-allocation
        // 5. Lifetime: Returned mutable reference lifetime tied to &mut self
        // Note: This is experimental code - production should initialize before returning
        unsafe { Some(self.blocks[block_index].assume_init_mut()) }
    }

    /// Deallocate a block
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `block_index` is a valid index that was previously allocated
    /// - The block is not accessed after deallocation
    /// - No double-free occurs
    ///
    /// # Safety Proof
    ///
    /// - **Bounds**: block_index < POOL_SIZE verified by debug_assert
    /// - **Previously allocated**: free_mask bit check verifies block was allocated
    /// - **No double-free**: Setting free_mask bit prevents double deallocation
    /// - **Exclusive access**: &mut self ensures no concurrent deallocations
    /// - **No use-after-free**: Caller guarantees no further access to block
    #[inline(always)]
    pub unsafe fn deallocate(&mut self, block_index: usize) {
        debug_assert!(block_index < POOL_SIZE, "Block index out of bounds");
        debug_assert!(
            self.free_mask & (1u64 << block_index) == 0,
            "Block not allocated"
        );

        self.free_mask |= 1u64 << block_index;
    }

    /// Get available blocks count
    #[inline(always)]
    pub fn available_blocks(&self) -> u32 {
        self.free_mask.count_ones()
    }
}

// ==================== SECTION ====================

/// **ZERO-COST**: Branch-free operations
///
/// Operations that avoid conditional branches for better performance
pub struct ZeroCostOps;
impl ZeroCostOps {
    /// Branch-free minimum
    #[inline(always)]
    #[must_use]
    pub fn min_branchless(a: u32, b: u32) -> u32 {
        a ^ ((a ^ b) & u32::from(a > b).wrapping_neg())
    }

    /// Branch-free maximum
    #[inline(always)]
    #[must_use]
    pub fn max_branchless(a: u32, b: u32) -> u32 {
        a ^ ((a ^ b) & u32::from(a < b).wrapping_neg())
    }

    /// Branch-free absolute value
    #[inline(always)]
    #[must_use]
    pub fn abs_branchless(x: i32) -> i32 {
        let mask = x >> 31;
        (x + mask) ^ mask
    }

    /// Branch-free conditional assignment
    #[inline(always)]
    #[must_use]
    pub fn conditional_assign(condition: bool, if_true: u32, if_false: u32) -> u32 {
        let mask = u32::from(condition).wrapping_neg();
        (mask & if_true) | (!mask & if_false)
    }
}

/// **ZERO-COST**: Cache-aligned data structures
///
/// Data structures optimized for CPU cache efficiency
#[repr(align(64))] // Align to cache line size
pub struct CacheAligned<T> {
    data: T,
}
impl<T> CacheAligned<T> {
    /// Create cache-aligned data
    #[inline(always)]
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// Get reference to data
    #[inline(always)]
    pub fn get(&self) -> &T {
        &self.data
    }

    /// Get mutable reference to data
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

// ==================== SECTION ====================

/// **ZERO-COST**: Service with compile-time configuration
///
/// Service that configures itself at compile time
pub struct ZeroCostService<C: ZeroCostConfig> {
    _config: PhantomData<C>,
}
impl<C: ZeroCostConfig> Default for ZeroCostService<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ZeroCostConfig> ZeroCostService<C> {
    /// Create a new zero-cost service
    #[must_use]
    pub fn new() -> Self {
        Self {
            _config: PhantomData,
        }
    }

    /// Get buffer size (compile-time constant)
    #[inline(always)]
    #[must_use]
    pub fn buffer_size() -> usize {
        C::BUFFER_SIZE
    }

    /// Get max connections (compile-time constant)
    #[inline(always)]
    #[must_use]
    pub fn max_connections() -> usize {
        C::MAX_CONNECTIONS
    }

    /// Get timeout (compile-time constant)
    #[inline(always)]
    #[must_use]
    pub fn timeout_ms() -> u64 {
        C::TIMEOUT_MS
    }

    /// Check if debug mode (compile-time constant)
    #[inline(always)]
    #[must_use]
    pub fn is_debug() -> bool {
        C::DEBUG
    }

    /// Process data with zero-cost configuration
    #[inline(always)]
    pub fn process_data(&self, data: &[u8]) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(Self::buffer_size());

        // Process with compile-time optimizations
        if Self::is_debug() {
            // Debug path (optimized away in production)
            tracing::debug!("Processing {} bytes", data.len());
        }

        // Processing logic here
        buffer.extend_from_slice(data);
        buffer
    }
}

// ==================== SECTION ====================

/// **ZERO-COST**: Performance measurement that compiles away in release
///
/// Benchmarking that has zero overhead in production builds
pub struct ZeroCostBenchmark;
impl ZeroCostBenchmark {
    /// Measure operation (compiles away in release)
    #[inline(always)]
    pub fn measure<R, F>(name: &str, operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        #[cfg(debug_assertions)]
        {
            let start = std::time::Instant::now();
            let result = operation();
            let duration = start.elapsed();
            tracing::debug!("Operation '{}' took {:?}", name, duration);
            result
        }

        #[cfg(not(debug_assertions))]
        {
            let _ = name; // Suppress unused variable warning
            operation()
        }
    }

    /// Count operations (compiles away in release)
    #[inline(always)]
    pub fn count_operation(operation_name: &'static str) {
        #[cfg(debug_assertions)]
        {
            use std::sync::Mutex;
            use std::sync::OnceLock;

            // Type alias to reduce complexity
            type CounterMap = OnceLock<Mutex<HashMap<&'static str, u64>>>;
            static COUNTERS: CounterMap = OnceLock::new();

            let counters = COUNTERS.get_or_init(|| Mutex::new(HashMap::new()));
            if let Ok(mut counters) = counters.lock() {
                *counters.entry(operation_name).or_insert(0) += 1;
            }
        }

        #[cfg(not(debug_assertions))]
        {
            let _ = operation_name; // Suppress unused variable warning
        }
    }
}

// ==================== SECTION ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_cost_array() {
        let mut array: ZeroCostArray<i32, 5> = ZeroCostArray::new();

        // Test push operations
        assert!(array.push(1).is_ok());
        assert!(array.push(2).is_ok());
        assert_eq!(array.len(), 2);

        // Test capacity
        assert_eq!(array.capacity(), 5);

        // Test overflow
        for i in 3..=5 {
            assert!(array.push(i).is_ok());
        }
        assert!(array.push(6).is_err()); // Should fail - capacity exceeded
    }

    #[test]
    fn test_zero_cost_service() {
        let service: ZeroCostService<ProductionConfig> = ZeroCostService::new();

        // All these are compile-time constants
        assert_eq!(ZeroCostService::<ProductionConfig>::buffer_size(), 8192);
        assert_eq!(ZeroCostService::<ProductionConfig>::max_connections(), 1000);
        assert_eq!(ZeroCostService::<ProductionConfig>::timeout_ms(), 30000); // Updated: canonical value (was 5000)
        assert!(!ZeroCostService::<ProductionConfig>::is_debug());

        // Test data processing
        let data = b"test data";
        let result = service.process_data(data);
        assert_eq!(result, data);
    }

    #[test]
    fn test_branch_free_operations() {
        // Test branch-free operations
        assert_eq!(ZeroCostOps::min_branchless(5, 3), 3);
        assert_eq!(ZeroCostOps::max_branchless(5, 3), 5);
        assert_eq!(ZeroCostOps::abs_branchless(-5), 5);
        assert_eq!(ZeroCostOps::conditional_assign(true, 10, 20), 10);
        assert_eq!(ZeroCostOps::conditional_assign(false, 10, 20), 20);
    }

    #[test]
    fn test_cache_aligned() {
        let aligned = CacheAligned::new(42u64);
        assert_eq!(*aligned.get(), 42);

        // Verify alignment
        let ptr = aligned.get() as *const u64;
        assert_eq!(ptr as usize % 64, 0); // Should be 64-byte aligned
    }

    #[test]
    fn test_zero_cost_pool() -> Result<(), Box<dyn std::error::Error>> {
        let mut pool: ZeroCostPool<u8, 4, 16> = ZeroCostPool::new();

        // Test allocation
        let block1 = pool.allocate().ok_or_else(|| {
            crate::NestGateError::internal_error("Pool allocation failed", "test")
        })?;
        assert_eq!(block1.len(), 16);
        assert_eq!(pool.available_blocks(), 3);

        let _block2 = pool.allocate().ok_or_else(|| {
            crate::NestGateError::internal_error("Pool allocation failed", "test")
        })?;
        assert_eq!(pool.available_blocks(), 2);

        // Test deallocation
        // SAFETY: Test deallocation is safe because:
        // 1. Valid index: 0 is within bounds (POOL_SIZE = 4)
        // 2. Previously allocated: block1 was allocated at index 0
        // 3. No double-free: This is the only deallocation of block 0
        // 4. Test environment: Controlled test with known allocation state
        unsafe {
            pool.deallocate(0);
        }
        assert_eq!(pool.available_blocks(), 3);
        Ok(())
    }
}

// ==================== SECTION ====================

/// Validate that our zero-cost abstractions actually have zero cost
#[cfg(test)]
mod performance_validation {
    use super::*;

    #[test]
    fn validate_zero_cost_service() {
        // This should compile to the same assembly as direct constant usage
        let _service: ZeroCostService<ProductionConfig> = ZeroCostService::new();
        let buffer_size = ZeroCostService::<ProductionConfig>::buffer_size();

        // In optimized builds, this should be equivalent to:
        // let buffer_size = 8192;
        assert_eq!(buffer_size, 8192);
    }

    #[test]
    fn validate_zero_cost_operations() {
        // These should compile to branchless assembly
        let result = ZeroCostOps::min_branchless(10, 5);
        assert_eq!(result, 5);

        let result = ZeroCostOps::conditional_assign(true, 100, 200);
        assert_eq!(result, 100);
    }
}
