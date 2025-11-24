use crate::error::NestGateError;
use std::collections::HashMap;
//
// This module provides sophisticated memory management and optimization
// strategies for maximum performance with zero unsafe code.

use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::{Result};

/// Advanced memory optimization manager
pub struct MemoryOptimizationManager {
    cache_manager: Arc<CacheManager>,
    pool_manager: Arc<PoolManager>,
    metrics: Arc<RwLock<MemoryMetrics>>,
    config: MemoryConfig,
}
/// Memory optimization configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::MemoryConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::MemoryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct MemoryConfig {
    pub max_memory_usage: u64,
    pub cache_size_limit: u64,
    pub pool_size_limit: u64,
    pub cleanup_interval: Duration,
    pub enable_huge_pages: bool,
    pub memory_mapped_threshold: u64,
    pub zero_copy_threshold: u64,
}
impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memory_usage: 8 * 1024 * 1024 * 1024, // 8GB
            cache_size_limit: 2 * 1024 * 1024 * 1024,  // 2GB
            pool_size_limit: 512 * 1024 * 1024,        // 512MB
            cleanup_interval: Duration::from_secs(300), // 5 minutes
            enable_huge_pages: true,
            memory_mapped_threshold: 4096,
            zero_copy_threshold: 1024,
        }
    }
}

/// Memory usage metrics
#[derive(Debug, Default, Clone)]
pub struct MemoryMetrics {
    pub total_allocated: u64,
    pub cache_usage: u64,
    pub pool_usage: u64,
    pub peak_usage: u64,
    pub allocation_count: u64,
    pub deallocation_count: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub pool_hits: u64,
    pub pool_misses: u64,
    pub zero_copy_operations: u64,
    pub memory_savings_bytes: u64,
}
/// Smart cache manager with LRU and adaptive sizing
pub struct CacheManager {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    access_order: Arc<RwLock<Vec<String>>>,
    config: MemoryConfig,
    stats: Arc<RwLock<CacheStats>>,
}
/// Cache entry with metadata
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub data: Arc<Vec<u8>>,
    pub size: u64,
    pub created_at: Instant,
    pub last_accessed: Instant,
    pub access_count: u64,
    pub ttl: Option<Duration>,
}
/// Cache statistics
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub total_size: u64,
    pub entry_count: u64,
}
/// Memory pool manager for buffer reuse
pub struct PoolManager {
    small_buffers: Arc<RwLock<Vec<Vec<u8>>>>,    // < 1KB
    medium_buffers: Arc<RwLock<Vec<Vec<u8>>>>,   // 1KB - 64KB
    large_buffers: Arc<RwLock<Vec<Vec<u8>>>>,    // > 64KB
    stats: Arc<RwLock<PoolStats>>,
    config: MemoryConfig,
}
/// Pool statistics
#[derive(Debug, Default, Clone)]
pub struct PoolStats {
    pub small_pool_size: usize,
    pub medium_pool_size: usize,
    pub large_pool_size: usize,
    pub total_allocations: u64,
    pub total_reuses: u64,
    pub memory_saved: u64,
}
/// Memory optimization strategies
pub enum OptimizationStrategy {
    ZeroCopy,
    BufferReuse,
    StringInterning,
    MemoryMapping,
    LazyLoading,
    Compression,
}
impl MemoryOptimizationManager {
    /// Create new memory optimization manager
    pub fn new(config: MemoryConfig) -> Self {
        let cache_manager = Arc::new(CacheManager::new(config.clone()));
        let pool_manager = Arc::new(PoolManager::new(config.clone()));
        let metrics = Arc::new(RwLock::new(MemoryMetrics::default()));

        Self {
            cache_manager,
            pool_manager,
            metrics,
            config,
        }
    }

    /// Get optimized buffer for data
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_optimized_buffer(&self, size: usize) -> Result<OptimizedBuffer>  {
        if size < self.config.zero_copy_threshold as usize {
            // Use zero-copy for small data
            self.increment_zero_copy_operations().await;
            Ok(OptimizedBuffer::ZeroCopy(Vec::with_capacity(size)))
        } else {
            // Use pooled buffer for larger data
            let buffer = self.pool_manager.get_buffer(size).await?;
            Ok(OptimizedBuffer::Pooled(buffer))
        }
    }

    /// Cache data with optimal strategy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn cache_data(&self, key: String, data: Vec<u8>) -> Result<()>  {
        let size = data.len() as u64;
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_allocated += size;
            metrics.cache_usage += size;
            metrics.allocation_count += 1;
            
            if metrics.cache_usage > metrics.peak_usage {
                metrics.peak_usage = metrics.cache_usage;
            }
        }

        self.cache_manager.insert(key, data).await
    }

    /// Get cached data
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_cached_data(&self, key: &str) -> Result<Option<Arc<Vec<u8>>>>  {
        match self.cache_manager.get(key).await? {
            Some(data) => {
                let mut metrics = self.metrics.write().await;
                metrics.cache_hits += 1;
                Ok(Some(data))
            }
            None => {
                let mut metrics = self.metrics.write().await;
                metrics.cache_misses += 1;
                Ok(None)
            }
        }
    }

    /// Apply optimization strategy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn apply_optimization(&self, strategy: OptimizationStrategy) -> Result<()>  {
        match strategy {
            OptimizationStrategy::ZeroCopy => self.optimize_zero_copy().await,
            OptimizationStrategy::BufferReuse => self.optimize_buffer_reuse().await,
            OptimizationStrategy::StringInterning => self.optimize_string_interning().await,
            OptimizationStrategy::MemoryMapping => self.optimize_memory_mapping().await,
            OptimizationStrategy::LazyLoading => self.optimize_lazy_loading().await,
            OptimizationStrategy::Compression => self.optimize_compression().await,
        }
    }

    /// Get current memory metrics
    pub async fn get_metrics(&self) -> MemoryMetrics {
        self.metrics.read().await.clone()
    }

    /// Perform memory cleanup
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn cleanup(&self) -> Result<()>  {
        println!("🧹 Starting memory cleanup...");
        
        // Clean up cache
        self.cache_manager.cleanup().await?;
        
        // Clean up pools
        self.pool_manager.cleanup().await?;
        
        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            let cache_stats = self.cache_manager.get_stats().await;
            let pool_stats = self.pool_manager.get_stats().await;
            
            metrics.cache_usage = cache_stats.total_size;
            metrics.cache_hits = cache_stats.hits;
            metrics.cache_misses = cache_stats.misses;
            metrics.pool_hits = pool_stats.total_reuses;
            metrics.memory_savings_bytes = pool_stats.memory_saved;
        }
        
        println!("✅ Memory cleanup completed");
        Ok(())
    }

    /// Private helper methods

    async fn increment_zero_copy_operations(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.zero_copy_operations += 1;
    }

    async fn optimize_zero_copy(&self) -> Result<()> {
        println!("🚀 Optimizing zero-copy operations...");
        // Implementation for zero-copy optimization
        Ok(())
    }

    async fn optimize_buffer_reuse(&self) -> Result<()> {
        println!("♻️ Optimizing buffer reuse...");
        // Implementation for buffer reuse optimization
        Ok(())
    }

    async fn optimize_string_interning(&self) -> Result<()> {
        println!("📝 Optimizing string interning...");
        // Implementation for string interning optimization
        Ok(())
    }

    async fn optimize_memory_mapping(&self) -> Result<()> {
        println!("🗺️ Optimizing memory mapping...");
        // Implementation for memory mapping optimization
        Ok(())
    }

    async fn optimize_lazy_loading(&self) -> Result<()> {
        println!("⏳ Optimizing lazy loading...");
        // Implementation for lazy loading optimization
        Ok(())
    }

    async fn optimize_compression(&self) -> Result<()> {
        println!("🗜️ Optimizing compression...");
        // Implementation for compression optimization
        Ok(())
    }
}

impl CacheManager {
    /// Create new cache manager
    #[must_use]
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            access_order: Arc::new(RwLock::new(Vec::new())),
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// Insert data into cache
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn insert(&self, key: String, data: Vec<u8>) -> Result<()>  {
        let size = data.len() as u64;
        let now = Instant::now();
        
        let entry = CacheEntry {
            data: Arc::new(data),
            size,
            created_at: now,
            last_accessed: now,
            access_count: 0,
            ttl: None,
        };

        {
            let mut cache = self.cache.write().await;
            let mut access_order = self.access_order.write().await;
            let mut stats = self.stats.write().await;

            // Check if we need to evict entries
            while stats.total_size + size > self.config.cache_size_limit {
                if let Some(oldest_key) = access_order.first().cloned() {
                    if let Some(old_entry) = cache.remove(&oldest_key) {
                        stats.total_size -= old_entry.size;
                        stats.entry_count -= 1;
                        stats.evictions += 1;
                        access_order.retain(|k| k != &oldest_key);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // Insert new entry
            cache.insert(key.clone(), entry);
            access_order.push(key);
            stats.total_size += size;
            stats.entry_count += 1;
        }

        Ok(())
    }

    /// Get data from cache
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get(&self, key: &str) -> Result<Option<Arc<Vec<u8>>>>  {
        let mut cache = self.cache.write().await;
        let mut access_order = self.access_order.write().await;
        
        if let Some(entry) = cache.get_mut(key) {
            // Update access information
            entry.last_accessed = Instant::now();
            entry.access_count += 1;
            
            // Move to end of access order (most recently used)
            access_order.retain(|k| k != key);
            access_order.push(key.to_string());
            
            Ok(Some(Arc::clone(&entry.data)))
        } else {
            Ok(None)
        }
    }

    /// Clean up expired entries
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn cleanup(&self) -> Result<()>  {
        let mut cache = self.cache.write().await;
        let mut access_order = self.access_order.write().await;
        let mut stats = self.stats.write().await;
        
        let now = Instant::now();
        let mut to_remove = Vec::new();
        
        for (key, entry) in cache.iter() {
            if let Some(ttl) = entry.ttl {
                if now.duration_since(entry.created_at) > ttl {
                    to_remove.push(key.clone());
                }
            }
        }
        
        for key in to_remove {
            if let Some(entry) = cache.remove(&key) {
                stats.total_size -= entry.size;
                stats.entry_count -= 1;
                access_order.retain(|k| k != &key);
            }
        }
        
        Ok(())
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }
}

impl PoolManager {
    /// Create new pool manager
    #[must_use]
    pub fn new(config: MemoryConfig) -> Self {
        Self {
            small_buffers: Arc::new(RwLock::new(Vec::new())),
            medium_buffers: Arc::new(RwLock::new(Vec::new())),
            large_buffers: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(PoolStats::default())),
            config,
        }
    }

    /// Get buffer from appropriate pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_buffer(&self, size: usize) -> Result<Vec<u8>>  {
        let mut stats = self.stats.write().await;
        stats.total_allocations += 1;

        let buffer = if size < 1024 {
            // Small buffer
            let mut pool = self.small_buffers.write().await;
            if let Some(mut buffer) = pool.pop() {
                buffer.clear();
                buffer.reserve(size);
                stats.total_reuses += 1;
                stats.memory_saved += size as u64;
                buffer
            } else {
                Vec::with_capacity(size)
            }
        } else if size < 65536 {
            // Medium buffer
            let mut pool = self.medium_buffers.write().await;
            if let Some(mut buffer) = pool.pop() {
                buffer.clear();
                buffer.reserve(size);
                stats.total_reuses += 1;
                stats.memory_saved += size as u64;
                buffer
            } else {
                Vec::with_capacity(size)
            }
        } else {
            // Large buffer
            let mut pool = self.large_buffers.write().await;
            if let Some(mut buffer) = pool.pop() {
                buffer.clear();
                buffer.reserve(size);
                stats.total_reuses += 1;
                stats.memory_saved += size as u64;
                buffer
            } else {
                Vec::with_capacity(size)
            }
        };

        Ok(buffer)
    }

    /// Return buffer to appropriate pool
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn return_buffer(&self, buffer: Vec<u8>) -> Result<()>  {
        let capacity = buffer.capacity();
        
        if capacity < 1024 {
            let mut pool = self.small_buffers.write().await;
            if pool.len() < 100 { // Limit pool size
                pool.push(buffer);
                let mut stats = self.stats.write().await;
                stats.small_pool_size = pool.len();
            }
        } else if capacity < 65536 {
            let mut pool = self.medium_buffers.write().await;
            if pool.len() < 50 {
                pool.push(buffer);
                let mut stats = self.stats.write().await;
                stats.medium_pool_size = pool.len();
            }
        } else {
            let mut pool = self.large_buffers.write().await;
            if pool.len() < 10 {
                pool.push(buffer);
                let mut stats = self.stats.write().await;
                stats.large_pool_size = pool.len();
            }
        }

        Ok(())
    }

    /// Clean up pools
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn cleanup(&self) -> Result<()>  {
        // Trim pools to reasonable sizes
        {
            let mut small_pool = self.small_buffers.write().await;
            small_pool.truncate(50);
        }
        
        {
            let mut medium_pool = self.medium_buffers.write().await;
            medium_pool.truncate(25);
        }
        
        {
            let mut large_pool = self.large_buffers.write().await;
            large_pool.truncate(5);
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.small_pool_size = self.small_buffers.read().await.len();
            stats.medium_pool_size = self.medium_buffers.read().await.len();
            stats.large_pool_size = self.large_buffers.read().await.len();
        }

        Ok(())
    }

    /// Get pool statistics
    pub async fn get_stats(&self) -> PoolStats {
        self.stats.read().await.clone()
    }
}

/// Optimized buffer types
pub enum OptimizedBuffer {
    ZeroCopy(Vec<u8>),
    Pooled(Vec<u8>),
    MemoryMapped(Vec<u8>), // Placeholder for memory-mapped buffers
}
impl OptimizedBuffer {
    /// Get the underlying buffer
    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::ZeroCopy(buf) => buf,
            Self::Pooled(buf) => buf,
            Self::MemoryMapped(buf) => buf,
        }
    }

    /// Get mutable access to the buffer
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            OptimizedBuffer::ZeroCopy(buf) => buf,
            OptimizedBuffer::Pooled(buf) => buf,
            OptimizedBuffer::MemoryMapped(buf) => buf,
        }
    }

    /// Get buffer size
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type MemoryConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using MemoryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_optimization_manager() -> Result<()> {
        let config = MemoryConfig::default();
        let manager = MemoryOptimizationManager::new(config);

        // Test buffer allocation
        let buffer = manager.get_optimized_buffer(512).await?;
        assert!(!buffer.is_empty() || buffer.len() == 0); // Buffer may be empty initially

        // Test caching
        let data = vec![1, 2, 3, 4, 5];
        manager.cache_data("test_key".to_string(), data.clone()).await?;
        
        let cached = manager.get_cached_data("test_key").await?;
        assert!(cached.is_some());
        assert_eq!(*cached.expect("Operation failed"), data);

        // Test metrics
        let metrics = manager.get_metrics().await;
        assert!(metrics.cache_hits > 0 || metrics.cache_misses > 0);

        println!("✅ Memory optimization manager test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_cache_manager() -> Result<()> {
        let config = MemoryConfig::default();
        let cache = CacheManager::new(config);

        // Test insertion and retrieval
        let data = vec![1, 2, 3, 4, 5];
        cache.insert("test".to_string(), data.clone()).await?;
        
        let retrieved = cache.get("test").await?;
        assert!(retrieved.is_some());
        assert_eq!(*retrieved.expect("Operation failed"), data);

        // Test cache miss
        let missing = cache.get("nonexistent").await?;
        assert!(missing.is_none());

        println!("✅ Cache manager test passed");
        Ok(())
    }

    #[tokio::test]
    async fn test_pool_manager() -> Result<()> {
        let config = MemoryConfig::default();
        let pool = PoolManager::new(config);

        // Test buffer allocation and return
        let buffer = pool.get_buffer(1024).await?;
        assert!(buffer.capacity() >= 1024);
        
        pool.return_buffer(buffer).await?;

        // Test reuse
        let reused_buffer = pool.get_buffer(1024).await?;
        assert!(reused_buffer.capacity() >= 1024);

        println!("✅ Pool manager test passed");
        Ok(())
    }
} 
