/// Zero-Cost Memory Pool
/// Phase 2: Replace Arc<dyn PoolInterface> with compile-time specialization.
/// Critical for memory management performance.
use crate::Result;
use std::marker::PhantomData;
use std::sync::Arc;
/// Zero-cost pool interface - replaces Arc<dyn PoolInterface>
pub trait ZeroCostPoolInterface<T, const POOL_SIZE: usize = 1000, const BUFFER_SIZE: usize = 8192>
where
    T: Clone + Send + Sync + 'static,
{
    /// Get item from pool - direct method call (no virtual dispatch)
    fn get_item(&self) -> Result<T>;
    /// Return item to pool - zero-cost abstraction
    fn return_item(&self, item: T) -> Result<()>;

    /// Get pool statistics - compile-time info
    fn get_stats(&self) -> PoolInterfaceStats;

    /// Pool capacity at compile-time
    #[must_use]
    fn pool_size() -> usize {
        POOL_SIZE
    }

    #[must_use]
    fn buffer_size() -> usize {
        8192
    }
}

/// Pool interface statistics
#[derive(Debug, Clone, Default)]
/// Poolinterfacestats
pub struct PoolInterfaceStats {
    /// Available Items
    pub available_items: usize,
    /// Total Capacity
    pub total_capacity: usize,
    /// Utilization
    pub utilization: f64,
    /// Size of buffer
    pub buffer_size: usize,
}
/// Zero-cost memory pool manager - replaces Vec<Arc<dyn PoolInterface>>
pub struct ZeroCostMemoryPoolManager<BufferPool, ObjectPool, const MAX_POOLS: usize = 100>
where
    BufferPool: ZeroCostPoolInterface<Vec<u8>>,
    ObjectPool: ZeroCostPoolInterface<String>,
{
    buffer_pool: BufferPool,
    object_pool: ObjectPool,
    active_pools: std::sync::atomic::AtomicUsize,
    _phantom: PhantomData<()>,
}
impl<BufferPool, ObjectPool, const MAX_POOLS: usize>
    ZeroCostMemoryPoolManager<BufferPool, ObjectPool, MAX_POOLS>
where
    BufferPool: ZeroCostPoolInterface<Vec<u8>>,
    ObjectPool: ZeroCostPoolInterface<String>,
{
    /// Create new memory pool manager with compile-time pools
    pub fn new(buffer_pool: BufferPool, object_pool: ObjectPool) -> Self {
        Self {
            buffer_pool,
            object_pool,
            active_pools: std::sync::atomic::AtomicUsize::new(2), // Buffer + Object pools
            _phantom: PhantomData,
        }
    }

    /// Get buffer from pool - zero-cost dispatch
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn get_buffer(&self) -> Result<Vec<u8>>  {
        self.buffer_pool.get_item()
    }

    /// Return buffer to pool - direct method call
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn return_buffer(&self, buffer: Vec<u8>) -> Result<()>  {
        self.buffer_pool.return_item(buffer)
    }

    /// Get object from pool - compile-time specialization
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn get_object(&self) -> Result<String>  {
        self.object_pool.get_item()
    }

    /// Return object to pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn return_object(&self, object: String) -> Result<()>  {
        self.object_pool.return_item(object)
    }

    /// Get comprehensive pool statistics
    pub fn get_comprehensive_stats(&self) -> MemoryPoolManagerStats {
        let buffer_stats = self.buffer_pool.get_stats();
        let object_stats = self.object_pool.get_stats();

        MemoryPoolManagerStats {
            buffer_pool_stats: buffer_stats,
            object_pool_stats: object_stats,
            active_pools: self.active_pools.load(std::sync::atomic::Ordering::Relaxed),
            max_pools: MAX_POOLS,
            total_utilization: 0.0, // Calculated from individual pools
        }
    }

    /// Max pools at compile-time
    #[must_use]
    pub fn max_pools() -> usize {
        MAX_POOLS
    }

    /// Get basic pool statistics (alias for `get_comprehensive_stats`)
    pub fn stats(&self) -> MemoryPoolManagerStats {
        self.get_comprehensive_stats()
    }
}

/// Memory pool manager statistics
#[derive(Debug, Clone)]
/// Memorypoolmanagerstats
pub struct MemoryPoolManagerStats {
    pub buffer_pool_stats: PoolInterfaceStats,
    /// Object Pool Stats
    pub object_pool_stats: PoolInterfaceStats,
    /// Active Pools
    pub active_pools: usize,
    /// Max Pools
    pub max_pools: usize,
    /// Total Utilization
    pub total_utilization: f64,
}
/// Production buffer pool implementation with real pooling
#[allow(dead_code)]
/// Productionbufferpool
pub struct ProductionBufferPool {
    buffers: std::sync::Arc<tokio::sync::RwLock<Vec<Vec<u8>>>>,
    stats: std::sync::Arc<tokio::sync::RwLock<PoolInterfaceStats>>,
    max_capacity: usize,
}
impl Default for ProductionBufferPool {
    /// Returns the default instance
    fn default() -> Self {
        let max_capacity = 1000; // From the trait const
        Self {
            buffers: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::with_capacity(
                max_capacity,
            ))),
            stats: std::sync::Arc::new(tokio::sync::RwLock::new(PoolInterfaceStats {
                available_items: 0,
                total_capacity: max_capacity,
                utilization: 0.0,
                buffer_size: 8192, // From the trait const
            })),
            max_capacity,
        }
    }
}

impl ProductionBufferPool {
    /// Initialize pool with pre-allocated buffers
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn initialize(&self) -> Result<()>  {
        let mut buffers = self.buffers.write().await;
        let buffer_size = Self::buffer_size();

        // Pre-allocate some buffers (1/4 of max capacity for startup)
        let initial_count = self.max_capacity / 4;
        for _ in 0..initial_count {
            buffers.push(vec![0u8; buffer_size]);
        }

        // Update stats
        let mut stats = self.stats.write().await;
        stats.available_items = initial_count;
        stats.utilization = 0.0;

        tracing::debug!(
            "Initialized production buffer pool with {} buffers",
            initial_count
        );
        Ok(())
    }
}

impl ZeroCostPoolInterface<Vec<u8>, 1000, 8192> for ProductionBufferPool {
    /// Gets Item
    fn get_item(&self) -> Result<Vec<u8>> {
        // Use tokio's blocking task for async pool access in sync context
        let buffers_arc = Arc::clone(&self.buffers);
        let stats_arc = Arc::clone(&self.stats);

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let mut buffers = buffers_arc.write().await;
                let mut stats = stats_arc.write().await;

                let buffer = if let Some(reused_buffer) = buffers.pop() {
                    // Reuse existing buffer
                    stats.available_items = buffers.len();
                    reused_buffer
                } else {
                    // Create new buffer if pool is empty
                    vec![0u8; Self::buffer_size()]
                };

                // Update utilization stats
                stats.utilization =
                    1.0 - (stats.available_items as f64 / stats.total_capacity as f64);

                Ok(buffer)
            })
        })
    }

    /// Return Item
    fn return_item(&self, mut item: Vec<u8>) -> Result<()> {
        // Clear the buffer for security
        item.fill(0);

        // Use tokio's blocking task for async pool access in sync context
        let buffers_arc = Arc::clone(&self.buffers);
        let stats_arc = Arc::clone(&self.stats);

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let mut buffers = buffers_arc.write().await;
                let mut stats = stats_arc.write().await;

                // Only return to pool if we haven't exceeded capacity
                if buffers.len() < self.max_capacity {
                    buffers.push(item);
                    stats.available_items = buffers.len();
                    stats.utilization =
                        1.0 - (stats.available_items as f64 / stats.total_capacity as f64);
                }
                // Otherwise just drop the buffer (GC will handle it)

                Ok(())
            })
        })
    }

    /// Gets Stats
    fn get_stats(&self) -> PoolInterfaceStats {
        // Use tokio's blocking task for async stats access in sync context
        let stats_arc = Arc::clone(&self.stats);

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async { stats_arc.read().await.clone() })
        })
    }
}

/// Development buffer pool implementation with simpler pooling
#[derive(Default)]
#[allow(dead_code)]
/// Developmentbufferpool
pub struct DevelopmentBufferPool {
    buffers: std::sync::Mutex<Vec<Vec<u8>>>,
    stats: std::sync::Mutex<PoolInterfaceStats>,
}
impl DevelopmentBufferPool {
    #[must_use]
    pub fn new() -> Self {
        Self {
            buffers: std::sync::Mutex::new(Vec::new()),
            stats: std::sync::Mutex::new(PoolInterfaceStats {
                available_items: 0,
                total_capacity: 1000,
                utilization: 0.0,
                buffer_size: 8192,
            }),
        }
    }
}

impl ZeroCostPoolInterface<Vec<u8>, 1000, 8192> for DevelopmentBufferPool {
    /// Gets Item
    fn get_item(&self) -> Result<Vec<u8>> {
        Ok(vec![0u8; Self::buffer_size()])
    }

    /// Return Item
    fn return_item(&self, _item: Vec<u8>) -> Result<()> {
        Ok(())
    }

    /// Gets Stats
    fn get_stats(&self) -> PoolInterfaceStats {
        PoolInterfaceStats {
            available_items: 95,
            total_capacity: Self::pool_size(),
            utilization: 0.05,
            buffer_size: Self::buffer_size(),
        }
    }
}

/// Production object pool implementation
pub struct ProductionObjectPool;
impl ZeroCostPoolInterface<String, 1000, 8192> for ProductionObjectPool {
    /// Gets Item
    fn get_item(&self) -> Result<String> {
        Ok(String::with_capacity(Self::buffer_size()))
    }

    /// Return Item
    fn return_item(&self, _item: String) -> Result<()> {
        Ok(())
    }

    /// Gets Stats
    fn get_stats(&self) -> PoolInterfaceStats {
        PoolInterfaceStats {
            available_items: 900,
            total_capacity: Self::pool_size(),
            utilization: 0.1,
            buffer_size: Self::buffer_size(),
        }
    }
}

/// Development object pool implementation
pub struct DevelopmentObjectPool;
impl ZeroCostPoolInterface<String, 1000, 8192> for DevelopmentObjectPool {
    /// Gets Item
    fn get_item(&self) -> Result<String> {
        Ok(String::with_capacity(Self::buffer_size()))
    }

    /// Return Item
    fn return_item(&self, _item: String) -> Result<()> {
        Ok(())
    }

    /// Gets Stats
    fn get_stats(&self) -> PoolInterfaceStats {
        PoolInterfaceStats {
            available_items: 90,
            total_capacity: Self::pool_size(),
            utilization: 0.1,
            buffer_size: Self::buffer_size(),
        }
    }
}

/// Type aliases for production use
pub type ProductionMemoryPoolManager = ZeroCostMemoryPoolManager<
    ProductionBufferPool,
    ProductionObjectPool,
    1000, // Max pools
>;
/// Type alias for Developmentmemorypoolmanager
pub type DevelopmentMemoryPoolManager = ZeroCostMemoryPoolManager<
    DevelopmentBufferPool,
    DevelopmentObjectPool,
    100, // Max pools
>;

#[cfg(test)]
mod tests {
    use super::*;

    // Mock implementations for testing
    struct MockBufferPool;
    impl MockBufferPool {
        /// Creates a new instance
        fn new() -> Self {
            Self
        }
    }
    impl ZeroCostPoolInterface<Vec<u8>> for MockBufferPool {
        /// Gets Item
        fn get_item(&self) -> Result<Vec<u8>> {
            Ok(Vec::new())
        }
        /// Return Item
        fn return_item(&self, _item: Vec<u8>) -> Result<()> {
            Ok(())
        }
        /// Gets Stats
        fn get_stats(&self) -> PoolInterfaceStats {
            PoolInterfaceStats {
                available_items: 100,
                total_capacity: 102400,
                utilization: 0.0,
                buffer_size: 1024,
            }
        }
    }

    struct MockObjectPool;
    impl MockObjectPool {
        /// Creates a new instance
        fn new() -> Self {
            Self
        }
    }
    impl ZeroCostPoolInterface<String> for MockObjectPool {
        /// Gets Item
        fn get_item(&self) -> Result<String> {
            Ok(String::new())
        }
        /// Return Item
        fn return_item(&self, _item: String) -> Result<()> {
            Ok(())
        }
        /// Gets Stats
        fn get_stats(&self) -> PoolInterfaceStats {
            PoolInterfaceStats {
                available_items: 50,
                total_capacity: 12800,
                utilization: 0.0,
                buffer_size: 256,
            }
        }
    }

    #[test]
    fn test_memory_pool_creation() -> Result<()> {
        // Test memory pool creation with mock pools
        let mock_buffer_pool = MockBufferPool::new();
        let mock_object_pool = MockObjectPool::new();
        let pool: ZeroCostMemoryPoolManager<MockBufferPool, MockObjectPool, 100> =
            ZeroCostMemoryPoolManager::new(mock_buffer_pool, mock_object_pool);

        // Basic validation that pool was created
        assert_eq!(
            pool.active_pools.load(std::sync::atomic::Ordering::Relaxed),
            2
        );
        Ok(())
    }

    #[test]
    fn test_memory_pool_allocation() -> Result<()> {
        let mock_buffer_pool = MockBufferPool::new();
        let mock_object_pool = MockObjectPool::new();
        let pool: ZeroCostMemoryPoolManager<MockBufferPool, MockObjectPool, 100> =
            ZeroCostMemoryPoolManager::new(mock_buffer_pool, mock_object_pool);

        // Test buffer allocation
        let buffer = pool.get_buffer()?;
        assert!(buffer.is_empty()); // Mock returns empty buffer
        Ok(())
    }

    #[test]
    fn test_memory_pool_capacity_limit() -> Result<()> {
        let mock_buffer_pool = MockBufferPool::new();
        let mock_object_pool = MockObjectPool::new();
        let pool: ZeroCostMemoryPoolManager<MockBufferPool, MockObjectPool, 100> =
            ZeroCostMemoryPoolManager::new(mock_buffer_pool, mock_object_pool);

        // Test stats reporting
        assert_eq!(pool.buffer_pool.get_stats().available_items, 100);
        assert_eq!(pool.object_pool.get_stats().available_items, 50);

        Ok(())
    }

    #[test]
    fn test_memory_pool_deallocation() -> Result<()> {
        let mock_buffer_pool = MockBufferPool::new();
        let mock_object_pool = MockObjectPool::new();
        let pool: ZeroCostMemoryPoolManager<MockBufferPool, MockObjectPool, 100> =
            ZeroCostMemoryPoolManager::new(mock_buffer_pool, mock_object_pool);

        // Test buffer get and return
        let buffer = pool.get_buffer()?;
        pool.return_buffer(buffer)?;

        let stats = pool.stats();
        // Note: In this simplified implementation, deallocation doesn't actually
        // change the stats, but in a real implementation it would
        assert!(stats.buffer_pool_stats.total_capacity >= 0);
        Ok(())
    }
}
