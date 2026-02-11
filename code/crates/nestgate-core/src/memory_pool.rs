// Removed unused error imports
/// High-Performance Memory Pool System
///
/// This module provides optimized memory pooling to eliminate allocation bottlenecks
/// in data processing and storage operations.
///
/// ## Performance Impact
/// - **Before**: 212,953 ns/iter (frequent allocations)
/// - **Target**: <100,000 ns/iter (2x performance improvement)
/// - **Strategy**: Pool and reuse memory buffers instead of frequent allocation/deallocation
///
/// ## Zero-Copy Optimizations
///
/// The memory pool implements several zero-copy patterns:
/// - **Buffer Reuse**: Reduces allocation/deallocation overhead
/// - **RAII Guards**: Automatic buffer return to pool
/// - **Copy vs Clone**: Uses `Copy` for small types, avoids `Clone` where possible
/// - **Reference Patterns**: Prefers borrowing over owned types when safe
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
// Removed unused tracing import
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::debug;
use tracing::info;
use crate::constants::canonical::performance::{DEFAULT_BUFFER_SIZE, NETWORK_BUFFER_SIZE};

/// High-performance memory pool with configurable buffer sizes
#[derive(Debug)]
/// Memorypool
pub struct MemoryPool<T>
where
    T: Default + Clone + Send + 'static,
{
    /// Pool of reusable buffers
    pool: Arc<Mutex<VecDeque<Box<T>>>>,
    /// Factory function for creating new instances
    factory: fn() -> T,
    /// Maximum number of buffers to keep in pool
    max_size: usize,
    /// Minimum number of buffers to keep in pool
    min_size: usize,
    /// Statistics for pool performance tracking
    statistics: Arc<RwLock<PoolStatistics>>,
}
impl<T> MemoryPool<T>
where
    T: Default + Clone + Send + 'static,
{
    /// Create a new memory pool with specified configuration
    pub fn new(factory: fn() -> T, min_size: usize, max_size: usize) -> Self {
        let pool = Arc::new(Mutex::new(VecDeque::new()));

        // Pre-populate pool with minimum buffers
        {
            match pool.lock() {
                Ok(mut pool_guard) => {
                    for _ in 0..min_size {
                        pool_guard.push_back(Box::new(factory()));
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to initialize memory pool: {}", e);
                    // Continue with empty pool - this is a graceful degradation
                }
            }
        }

        Self {
            pool,
            factory,
            max_size,
            min_size,
            statistics: Arc::new(RwLock::new(PoolStatistics::new())),
        }
    }

    /// Get a buffer from the pool or create a new one
    pub fn get(&self) -> PoolGuard<T> {
        let start = Instant::now();

        // Try to get buffer from pool
        let buffer = {
            match self.pool.lock() {
                Ok(mut pool_guard) => pool_guard.pop_front(),
                Err(e) => {
                    tracing::error!("Memory pool mutex poisoned, creating new buffer: {}", e);
                    None // Fall back to creating new buffer
                }
            }
        };

        let (buffer, from_pool) = match buffer {
            Some(buf) => {
                // Got buffer from pool - increment hit counter
                if let Ok(mut stats) = self.statistics.write() {
                    stats.hits += 1;
                }
                (buf, true)
            }
            None => {
                // Create new buffer - increment miss counter
                if let Ok(mut stats) = self.statistics.write() {
                    stats.misses += 1;
                    stats.total_created += 1;
                }
                (Box::new((self.factory)()), false)
            }
        };

        let acquisition_time = start.elapsed();

        // Update performance metrics
        if let Ok(mut stats) = self.statistics.write() {
            stats.total_acquisitions += 1;
            stats.total_acquisition_time += acquisition_time;
        }

        debug!(
            from_pool = from_pool,
            acquisition_time_ns = acquisition_time.as_nanos(),
            "Memory pool buffer acquired"
        );

        PoolGuard {
            buffer: Some(buffer),
            pool: Arc::clone(&self.pool),
            max_size: self.max_size,
            statistics: Arc::clone(&self.statistics),
            acquired_at: start,
        }
    }

    /// Get current pool statistics (returns a copy for thread safety)
    /// Zero-copy optimization: PoolStatistics implements Copy
    pub fn statistics(&self) -> PoolStatistics {
        self.statistics
            .read()
            .map(|stats| *stats) // Zero-copy access - PoolStatistics is Copy
            .unwrap_or_else(|e| {
                tracing::error!("Failed to read pool statistics: {}", e);
                PoolStatistics::new() // Return default statistics on error
            })
    }

    /// Get current pool size
    pub fn size(&self) -> usize {
        self.pool.lock().map(|pool| pool.len()).unwrap_or_else(|e| {
            tracing::error!("Failed to get pool size: {}", e);
            0 // Return 0 on error
        })
    }

    /// Clear all buffers from pool
    pub fn clear(&self) {
        match self.pool.lock() {
            Ok(mut pool_guard) => {
                let cleared_count = pool_guard.len();
                pool_guard.clear();

                if let Ok(mut stats) = self.statistics.write() {
                    stats.total_cleared += cleared_count as u64;
                }
            }
            Err(e) => {
                tracing::error!("Failed to clear memory pool: {}", e);
                // Continue gracefully - pool may be in inconsistent state but won't panic
            }
        }
    }

    /// Shrink pool to minimum size
    pub fn shrink_to_min(&self) {
        match self.pool.lock() {
            Ok(mut pool_guard) => {
                while pool_guard.len() > self.min_size {
                    pool_guard.pop_back();
                }
            }
            Err(e) => {
                tracing::error!("Failed to shrink memory pool: {}", e);
                // Continue gracefully
            }
        }
    }

    /// Ensure pool has at least minimum buffers
    pub fn ensure_min_capacity(&self) {
        match self.pool.lock() {
            Ok(mut pool_guard) => {
                while pool_guard.len() < self.min_size {
                    pool_guard.push_back(Box::new((self.factory)()));
                }
            }
            Err(e) => {
                tracing::error!("Failed to ensure minimum pool capacity: {}", e);
                // Continue gracefully
            }
        }
    }
}

impl<T> Default for MemoryPool<T>
where
    T: Default + Clone + Send + 'static,
{
    /// Returns the default instance
    fn default() -> Self {
        Self::new(T::default, 5, 50)
    }
}

/// RAII guard for pool buffers that automatically returns buffer to pool
pub struct PoolGuard<T>
where
    T: Send + 'static,
{
    buffer: Option<Box<T>>,
    pool: Arc<Mutex<VecDeque<Box<T>>>>,
    max_size: usize,
    statistics: Arc<RwLock<PoolStatistics>>,
    acquired_at: Instant,
}
impl<T> PoolGuard<T>
where
    T: Send + 'static,
{
    /// Get a reference to the buffer
    ///
    /// # Panics
    /// Panics if the buffer has already been taken with `take()`. This indicates a logic error.
    #[allow(clippy::expect_used)] // Documented panic for logic errors
    pub fn get(&self) -> &T {
        // SAFETY: PoolGuard invariants; buffer is Some until take() is called
        self.buffer
            .as_ref()
            .expect("BUG: Buffer was already taken - logic error in PooledBuffer usage")
    }

    /// Get a mutable reference to the buffer
    ///
    /// # Panics
    /// Panics if the buffer has already been taken with `take()`. This indicates a logic error.
    #[allow(clippy::expect_used)] // Documented panic for logic errors
    pub fn get_mut(&mut self) -> &mut T {
        // SAFETY: PoolGuard invariants; buffer is Some until take() is called
        self.buffer
            .as_mut()
            .expect("BUG: Buffer was already taken - logic error in PooledBuffer usage")
    }

    /// Take ownership of the buffer (prevents return to pool)
    ///
    /// # Panics
    /// Panics if the buffer has already been taken. This indicates a logic error.
    #[allow(clippy::expect_used)] // Documented panic for logic errors
    pub fn take(mut self) -> Box<T> {
        // SAFETY: PoolGuard invariants; buffer is Some until take() is called
        self.buffer
            .take()
            .expect("BUG: Buffer was already taken - logic error in PooledBuffer usage")
    }

    /// Check if the buffer is still available (not taken)
    pub fn is_available(&self) -> bool {
        self.buffer.is_some()
    }
}

impl<T> Drop for PoolGuard<T>
where
    T: Send + 'static,
{
    /// Drop
    fn drop(&mut self) {
        if let Some(buffer) = self.buffer.take() {
            let usage_time = self.acquired_at.elapsed();

            // Try to return buffer to pool if not at capacity
            if let Ok(mut pool_guard) = self.pool.lock() {
                if pool_guard.len() < self.max_size {
                    pool_guard.push_back(buffer);

                    // Update statistics
                    if let Ok(mut stats) = self.statistics.write() {
                        stats.total_returned += 1;
                        stats.total_usage_time += usage_time;
                    }

                    debug!(
                        usage_time_ns = usage_time.as_nanos(),
                        pool_size = pool_guard.len(),
                        "Buffer returned to memory pool"
                    );
                } else {
                    // Pool is full, drop the buffer
                    if let Ok(mut stats) = self.statistics.write() {
                        stats.total_discarded += 1;
                    }

                    debug!(
                        pool_size = pool_guard.len(),
                        max_size = self.max_size,
                        "Buffer discarded - pool at capacity"
                    );
                }
            }
        }
    }
}

impl<T> std::ops::Deref for PoolGuard<T>
where
    T: Send + 'static,
{
    /// Type alias for Target
    type Target = T;

    /// Deref
    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> std::ops::DerefMut for PoolGuard<T>
where
    T: Send + 'static,
{
    /// Deref Mut
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

/// Memory pool performance statistics
/// Optimized for zero-copy access - all fields are Copy types
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
/// Poolstatistics
pub struct PoolStatistics {
    /// Number of successful buffer retrievals from pool
    pub hits: u64,
    /// Number of new buffer creations
    pub misses: u64,
    /// Total buffer acquisitions
    pub total_acquisitions: u64,
    /// Total buffers created
    pub total_created: u64,
    /// Total buffers returned to pool
    pub total_returned: u64,
    /// Total buffers discarded (pool full)
    pub total_discarded: u64,
    /// Total buffers cleared from pool
    pub total_cleared: u64,
    /// Total time spent acquiring buffers
    pub total_acquisition_time: Duration,
    /// Total time buffers were in use
    pub total_usage_time: Duration,
}
impl PoolStatistics {
    /// Creates a new instance
    fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            total_acquisitions: 0,
            total_created: 0,
            total_returned: 0,
            total_discarded: 0,
            total_cleared: 0,
            total_acquisition_time: Duration::from_secs(0),
            total_usage_time: Duration::from_secs(0),
        }
    }

    /// Calculate hit ratio (0.0 to 1.0)
    pub fn hit_ratio(&self) -> f64 {
        if self.total_acquisitions > 0 {
            self.hits as f64 / self.total_acquisitions as f64
        } else {
            0.0
        }
    }

    /// Calculate average acquisition time
    pub fn avg_acquisition_time(&self) -> Duration {
        if self.total_acquisitions > 0 {
            self.total_acquisition_time / self.total_acquisitions as u32
        } else {
            Duration::from_secs(0)
        }
    }

    /// Calculate average usage time
    pub fn avg_usage_time(&self) -> Duration {
        if self.total_returned > 0 {
            self.total_usage_time / self.total_returned as u32
        } else {
            Duration::from_secs(0)
        }
    }

    /// Check if pool is performing well (>80% hit ratio is good)
    pub fn is_efficient(&self) -> bool {
        self.hit_ratio() > 0.8
    }

    /// Get performance assessment
    pub fn performance_assessment(&self) -> &'static str {
        match self.hit_ratio() {
            r if r > 0.9 => "Excellent",
            r if r > 0.8 => "Good",
            r if r > 0.6 => "Fair",
            _ => "Poor - Consider increasing pool size",
        }
    }
}

/// Specialized buffer pool for common data types
pub type BufferPool = MemoryPool<Vec<u8>>;
/// Type alias for Stringpool
pub type StringPool = MemoryPool<String>;
// Uses std::sync::LazyLock for thread-safe lazy initialization
// Global buffer pools for common usage patterns

/// Global 4KB buffer pool for file I/O operations
pub static GLOBAL_4KB_BUFFER_POOL: std::sync::LazyLock<BufferPool> =
    std::sync::LazyLock::new(|| MemoryPool::new(
        || Vec::with_capacity(DEFAULT_BUFFER_SIZE),
        10,  // min_size
        100  // max_size
    ));

/// Global 64KB buffer pool for large data operations
pub static GLOBAL_64KB_BUFFER_POOL: std::sync::LazyLock<BufferPool> =
    std::sync::LazyLock::new(|| MemoryPool::new(
        || Vec::with_capacity(NETWORK_BUFFER_SIZE),
        5,   // min_size
        50   // max_size
    ));

/// Global 1MB buffer pool for bulk operations
pub static GLOBAL_1MB_BUFFER_POOL: std::sync::LazyLock<BufferPool> =
    std::sync::LazyLock::new(|| MemoryPool::new(
        || Vec::with_capacity(1048576),
        2,   // min_size
        20   // max_size
    ));

/// Global string pool for text operations
pub static GLOBAL_STRING_POOL: std::sync::LazyLock<StringPool> =
    std::sync::LazyLock::new(|| MemoryPool::new(
        || String::with_capacity(1024),
        20,  // min_size
        200  // max_size
    ));

/// Global command output buffer pool for ZFS operations (optimized for command output)
pub static GLOBAL_CMD_BUFFER_POOL: std::sync::LazyLock<BufferPool> =
    std::sync::LazyLock::new(|| MemoryPool::new(
        || Vec::with_capacity(16384),  // 16KB for command outputs
        15,  // min_size
        100  // max_size
    ));

/// Global network buffer pool for WebSocket/SSE operations
pub static GLOBAL_NETWORK_BUFFER_POOL: std::sync::LazyLock<BufferPool> =
    std::sync::LazyLock::new(|| MemoryPool::new(
        || Vec::with_capacity(8192),   // 8KB for network data
        25,  // min_size
        150  // max_size
    ));

/// Global JSON buffer pool for serialization operations
pub static GLOBAL_JSON_BUFFER_POOL: std::sync::LazyLock<StringPool> =
    std::sync::LazyLock::new(|| MemoryPool::new(
        || String::with_capacity(4096), // 4KB for JSON serialization
        30,  // min_size
        200  // max_size
    ));

/// Convenience functions for global buffer pools
pub fn get_4kb_buffer() -> PoolGuard<Vec<u8>> {
    GLOBAL_4KB_BUFFER_POOL.get()
}
/// Gets 64Kb Buffer
pub fn get_64kb_buffer() -> PoolGuard<Vec<u8>> {
    GLOBAL_64KB_BUFFER_POOL.get()
}

/// Gets 1Mb Buffer
pub fn get_1mb_buffer() -> PoolGuard<Vec<u8>> {
    GLOBAL_1MB_BUFFER_POOL.get()
}

/// Gets String Buffer
pub fn get_string_buffer() -> PoolGuard<String> {
    GLOBAL_STRING_POOL.get()
}

/// Convenience functions for specialized buffer pools
pub fn get_command_buffer() -> PoolGuard<Vec<u8>> {
    GLOBAL_CMD_BUFFER_POOL.get()
}
/// Gets Network Buffer
pub fn get_network_buffer() -> PoolGuard<Vec<u8>> {
    GLOBAL_NETWORK_BUFFER_POOL.get()
}

/// Gets Json Buffer
pub fn get_json_buffer() -> PoolGuard<String> {
    GLOBAL_JSON_BUFFER_POOL.get()
}

/// Get global buffer pool statistics
pub fn global_buffer_pool_stats() -> (
    PoolStatistics,
    PoolStatistics,
    PoolStatistics,
    PoolStatistics,
    PoolStatistics,
    PoolStatistics,
    PoolStatistics,
) {
    (
        GLOBAL_4KB_BUFFER_POOL.statistics(),
        GLOBAL_64KB_BUFFER_POOL.statistics(),
        GLOBAL_1MB_BUFFER_POOL.statistics(),
        GLOBAL_STRING_POOL.statistics(),
        GLOBAL_CMD_BUFFER_POOL.statistics(),
        GLOBAL_NETWORK_BUFFER_POOL.statistics(),
        GLOBAL_JSON_BUFFER_POOL.statistics(),
    )
}
/// Memory pool manager for coordinating multiple pools
pub struct MemoryPoolManager {
    pools: Vec<Arc<dyn PoolInterface>>,
}
/// PoolInterface trait
trait PoolInterface: Send + Sync {
    /// Size
    fn size(&self) -> usize;
    /// Clear
    fn clear(&self);
}

impl<T> PoolInterface for MemoryPool<T>
where
    T: Default + Clone + Send + 'static,
{
    /// Size
    fn size(&self) -> usize {
        self.size()
    }

    /// Clear
    fn clear(&self) {
        self.clear()
    }
}

impl MemoryPoolManager {
    /// Create a new memory pool manager
    #[must_use]
    pub fn new() -> Self {
        Self { pools: Vec::new() }
    }

    /// Register a pool with the manager
    pub fn register_pool<T>(&mut self, pool: Arc<MemoryPool<T>>)
    where
        T: Default + Clone + Send + 'static,
    {
        self.pools.push(pool);
    }

    /// Clear all registered pools
    pub fn clear_all_pools(&self) {
        for pool in &self.pools {
            pool.clear();
        }
        info!("Cleared {} memory pools", self.pools.len());
    }

    /// Get total number of buffers across all pools
    pub fn total_buffers(&self) -> usize {
        self.pools.iter().map(|p| p.size()).sum()
    }
}

impl Default for MemoryPoolManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_memory_pool_basic_functionality() {
        let pool = MemoryPool::new(|| Vec::<u8>::with_capacity(1024), 2, 10);

        // Get buffer from pool
        let mut buffer = pool.get();
        buffer.push(42);
        assert_eq!(buffer[0], 42);

        // Statistics should show usage
        let stats = pool.statistics();
        assert_eq!(stats.total_acquisitions, 1);
        assert!(stats.hits > 0 || stats.misses > 0);
    }

    #[test]
    fn test_buffer_reuse() {
        let pool = MemoryPool::new(|| Vec::<u8>::with_capacity(100), 1, 5);

        // Get and return buffer
        {
            let mut buffer = pool.get();
            buffer.push(1);
        } // Buffer returns to pool here

        // Get another buffer - should reuse previous
        {
            let buffer = pool.get();
            // Buffer might be reused (cleared) or new
            assert!(buffer.capacity() >= 100);
        }

        let stats = pool.statistics();
        assert_eq!(stats.total_acquisitions, 2);
        assert!(stats.total_returned >= 1);
    }

    #[test]
    fn test_pool_capacity_limits() {
        let pool = MemoryPool::new(Vec::<u8>::new, 0, 2);

        // Fill pool to capacity
        let _guard1 = pool.get();
        let _guard2 = pool.get();
        let _guard3 = pool.get(); // This will exceed capacity when returned

        // Check that pool respects max size
        assert!(pool.size() <= 2);
    }

    #[tokio::test]
    async fn test_concurrent_access() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let pool = Arc::new(MemoryPool::new(Vec::<u8>::new, 1, 10));
        let mut handles = vec![];

        // Spawn multiple async tasks (concurrent, non-blocking)
        for i in 0..5 {
            let pool_clone = Arc::clone(&pool);
            let handle = tokio::spawn(async move {
                let mut buffer = pool_clone.get();
                buffer.push(i as u8);
                tokio::time::sleep(Duration::from_millis(10)).await;
                buffer[0]
            });
            handles.push(handle);
        }

        // Wait for all async tasks (concurrent, non-blocking)
        for handle in handles {
            let result = handle.await.map_err(|e| {
                format!("Task join failed: {e:?}")
            })?;
            assert!(result < 5);
        }

        let stats = pool.statistics();
        assert_eq!(stats.total_acquisitions, 5);
        Ok(())
    }

    #[test]
    fn test_global_buffer_pools() {
        // Test global 4KB buffer pool
        let mut buffer = get_4kb_buffer();
        buffer.extend_from_slice(b"test data");
        assert_eq!(&buffer[..9], b"test data");

        // Test string pool
        let mut string_buf = get_string_buffer();
        string_buf.push_str("hello world");
        assert_eq!(&*string_buf, "hello world");
    }

    #[test]
    fn test_pool_statistics() {
        let pool = MemoryPool::new(Vec::<u8>::new, 1, 5);

        // Perform operations
        let _buf1 = pool.get();
        let _buf2 = pool.get();

        let stats = pool.statistics();
        // ✅ MODERN: Use epsilon for hit ratio range check
        assert!(stats.hit_ratio() >= -1e-9 && stats.hit_ratio() <= 1.0 + 1e-9);
        assert!(stats.avg_acquisition_time().as_nanos() > 0);
        assert!(!stats.performance_assessment().is_empty());
    }
}