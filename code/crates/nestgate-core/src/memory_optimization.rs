//! # Memory Optimization
//!
//! Advanced memory optimization patterns and utilities for reducing memory
//! usage, preventing leaks, and improving allocation efficiency.

use std::collections::HashMap;
use std::sync::{Arc, Weak, Mutex};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::alloc::{GlobalAlloc, Layout};

/// **MEMORY USAGE STATISTICS**
///
/// Comprehensive memory usage tracking and analysis
#[derive(Debug, Default)]
pub struct MemoryStats {
    /// Total allocations made
    pub total_allocations: AtomicU64,
    /// Total deallocations made
    pub total_deallocations: AtomicU64,
    /// Current active allocations
    pub active_allocations: AtomicU64,
    /// Peak memory usage in bytes
    pub peak_memory_usage: AtomicU64,
    /// Current memory usage in bytes
    pub current_memory_usage: AtomicU64,
    /// Total bytes allocated
    pub total_bytes_allocated: AtomicU64,
    /// Total bytes deallocated
    pub total_bytes_deallocated: AtomicU64,
    /// Large allocation count (>1MB)
    pub large_allocation_count: AtomicU64,
    /// Small allocation count (<1KB)
    pub small_allocation_count: AtomicU64,
}

impl MemoryStats {
    pub const fn new() -> Self {
        Self::default()
    }
    
    /// Record an allocation
    pub fn record_allocation(&self, size: usize) {
        self.total_allocations.fetch_add(1, Ordering::Relaxed);
        self.active_allocations.fetch_add(1, Ordering::Relaxed);
        self.total_bytes_allocated.fetch_add(size as u64, Ordering::Relaxed);
        
        let current = self.current_memory_usage.fetch_add(size as u64, Ordering::Relaxed) + size as u64;
        let peak = self.peak_memory_usage.load(Ordering::Relaxed);
        if current > peak {
            self.peak_memory_usage.store(current, Ordering::Relaxed);
        }
        
        if size > 1_048_576 { // 1MB
            self.large_allocation_count.fetch_add(1, Ordering::Relaxed);
        } else if size < 1024 { // 1KB
            self.small_allocation_count.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    /// Record a deallocation
    pub fn record_deallocation(&self, size: usize) {
        self.total_deallocations.fetch_add(1, Ordering::Relaxed);
        self.active_allocations.fetch_sub(1, Ordering::Relaxed);
        self.total_bytes_deallocated.fetch_add(size as u64, Ordering::Relaxed);
        self.current_memory_usage.fetch_sub(size as u64, Ordering::Relaxed);
    }
    
    /// Get memory efficiency ratio (deallocations / allocations)
    pub const fn efficiency_ratio(&self) -> f64 {
        let allocs = self.total_allocations.load(Ordering::Relaxed) as f64;
        let deallocs = self.total_deallocations.load(Ordering::Relaxed) as f64;
        if allocs > 0.0 { deallocs / allocs } else { 0.0 }
    }
    
    /// Check if there are potential memory leaks
    pub const fn has_potential_leaks(&self) -> bool {
        let active = self.active_allocations.load(Ordering::Relaxed);
        let total = self.total_allocations.load(Ordering::Relaxed);
        
        // If more than 10% of allocations are still active, potential leak
        active > 0 && (f64::from(active) / f64::from(total)) > 0.1
    }
}

/// **OBJECT POOL**
///
/// Generic object pool for reusing expensive-to-create objects
pub struct ObjectPool<T> {
    objects: Mutex<Vec<T>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
    max_size: usize,
    stats: PoolStats,
}

#[derive(Debug, Default)]
struct PoolStats {
    hits: AtomicU64,
    misses: AtomicU64,
    creates: AtomicU64,
    returns: AtomicU64,
}

impl<T> ObjectPool<T> {
    /// Create new object pool with factory function
    pub fn new<F>(factory: F, max_size: usize) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            objects: Mutex::new(Vec::new()),
            factory: Box::new(factory),
            max_size,
            stats: PoolStats::default(),
        }
    }
    
    /// Get object from pool or create new one
    pub fn acquire(&self) -> T {
        let mut objects = self.objects.lock().unwrap();
        
        if let Some(obj) = objects.pop() {
            self.stats.hits.fetch_add(1, Ordering::Relaxed);
            obj
        } else {
            self.stats.misses.fetch_add(1, Ordering::Relaxed);
            self.stats.creates.fetch_add(1, Ordering::Relaxed);
            (self.factory)()
        }
    }
    
    /// Return object to pool
    pub fn release(&self, obj: T) {
        let mut objects = self.objects.lock().unwrap();
        
        if objects.len() < self.max_size {
            objects.push(obj);
            self.stats.returns.fetch_add(1, Ordering::Relaxed);
        }
        // If pool is full, object is dropped (deallocated)
    }
    
    /// Get pool statistics
    pub const fn stats(&self) -> (u64, u64, f64) {
        let hits = self.stats.hits.load(Ordering::Relaxed);
        let misses = self.stats.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 { f64::from(hits) / f64::from(total) } else { 0.0 };
        (hits, misses, hit_rate)
    }
}

/// **WEAK REFERENCE CACHE**
///
/// Cache that uses weak references to avoid memory leaks
pub struct WeakCache<K, V> {
    cache: Mutex<HashMap<K, Weak<V>>>,
    stats: CacheStats,
}

#[derive(Debug, Default)]
struct CacheStats {
    hits: AtomicU64,
    misses: AtomicU64,
    evictions: AtomicU64,
}

impl<K, V> WeakCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            stats: CacheStats::default(),
        }
    }
    
    /// Get value from cache
    #[must_use]
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let mut cache = self.cache.lock().unwrap();
        
        if let Some(weak_ref) = cache.get(key) {
            if let Some(strong_ref) = weak_ref.upgrade() {
                self.stats.hits.fetch_add(1, Ordering::Relaxed);
                return Some(strong_ref);
            } else {
                // Weak reference is dead, remove it
                cache.remove(key);
                self.stats.evictions.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        self.stats.misses.fetch_add(1, Ordering::Relaxed);
        None
    }
    
    /// Insert value into cache
    pub fn insert(&self, key: K, value: Arc<V>) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, Arc::downgrade(&value));
    }
    
    /// Clean up dead weak references
    pub fn cleanup(&self) {
        let mut cache = self.cache.lock().unwrap();
        let mut to_remove = Vec::new();
        
        for (key, weak_ref) in cache.iter() {
            if weak_ref.strong_count() == 0 {
                to_remove.push(key.clone());
            }
        }
        
        let removed = to_remove.len();
        for key in to_remove {
            cache.remove(&key);
        }
        
        self.stats.evictions.fetch_add(removed as u64, Ordering::Relaxed);
    }
    
    /// Get cache statistics
    pub const fn stats(&self) -> (u64, u64, f64, usize) {
        let hits = self.stats.hits.load(Ordering::Relaxed);
        let misses = self.stats.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 { f64::from(hits) / f64::from(total) } else { 0.0 };
        let size = self.cache.lock().unwrap().len();
        (hits, misses, hit_rate, size)
    }
}

/// **MEMORY PRESSURE DETECTOR**
///
/// Detects memory pressure and triggers cleanup actions
pub struct MemoryPressureDetector {
    memory_threshold: f64, // Percentage (0.0 to 1.0)
    check_interval: Duration,
    last_check: Mutex<Instant>,
    callbacks: Mutex<Vec<Box<dyn Fn() + Send + Sync>>>,
}

impl MemoryPressureDetector {
    #[must_use]
    pub fn new(memory_threshold: f64, check_interval: Duration) -> Self {
        Self {
            memory_threshold,
            check_interval,
            last_check: Mutex::new(Instant::now()),
            callbacks: Mutex::new(Vec::new()),
        }
    }
    
    /// Register callback for memory pressure events
    pub fn register_callback<F>(&self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
    }
    
    /// Check for memory pressure and trigger callbacks if needed
    pub fn check_pressure(&self) {
        let mut last_check = self.last_check.lock().unwrap();
        let now = Instant::now();
        
        if now.duration_since(*last_check) < self.check_interval {
            return;
        }
        *last_check = now;
        drop(last_check);
        
        if let Ok(memory_usage) = self.get_memory_usage_percentage() {
            if memory_usage > self.memory_threshold {
                let callbacks = self.callbacks.lock().unwrap();
                for callback in callbacks.iter() {
                    callback();
                }
            }
        }
    }
    
    fn get_memory_usage_percentage(&self) -> Result<f64, &'static str> {
        // In a real implementation, this would query system memory usage
        // For this example, we'll return a simulated value
        Ok(0.65) // 65% memory usage
    }
}

/// **MEMORY ARENA**
///
/// Arena allocator for reducing allocation overhead
pub struct MemoryArena {
    chunks: Mutex<Vec<Vec<u8>>>,
    current_chunk: Mutex<Option<Vec<u8>>>,
    chunk_size: usize,
    allocation_count: AtomicUsize,
}

impl MemoryArena {
    #[must_use]
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunks: Mutex::new(Vec::new()),
            current_chunk: Mutex::new(None),
            chunk_size,
            allocation_count: AtomicUsize::new(0),
        }
    }
    
    /// Allocate memory from arena
    #[must_use]
    pub fn allocate(&self, size: usize) -> Option<*mut u8> {
        if size > self.chunk_size {
            return None; // Too large for arena
        }
        
        let mut current_chunk = self.current_chunk.lock().unwrap();
        
        // Check if current chunk has enough space
        if let Some(ref mut chunk) = *current_chunk {
            if chunk.capacity() - chunk.len() >= size {
                let ptr = unsafe { chunk.as_mut_ptr().add(chunk.len()) };
                unsafe { chunk.set_len(chunk.len() + size) };
                self.allocation_count.fetch_add(1, Ordering::Relaxed);
                return Some(ptr);
            }
        }
        
        // Need new chunk
        let mut new_chunk = Vec::with_capacity(self.chunk_size);
        let ptr = new_chunk.as_mut_ptr();
        unsafe { new_chunk.set_len(size) };
        
        // Store old chunk if exists
        if let Some(old_chunk) = current_chunk.replace(new_chunk) {
            let mut chunks = self.chunks.lock().unwrap();
            chunks.push(old_chunk);
        }
        
        self.allocation_count.fetch_add(1, Ordering::Relaxed);
        Some(ptr)
    }
    
    /// Reset arena (deallocate all memory)
    pub fn reset(&self) {
        let mut chunks = self.chunks.lock().unwrap();
        chunks.clear();
        
        let mut current_chunk = self.current_chunk.lock().unwrap();
        *current_chunk = None;
        
        self.allocation_count.store(0, Ordering::Relaxed);
    }
    
    /// Get total allocations made
    pub const fn allocation_count(&self) -> usize {
        self.allocation_count.load(Ordering::Relaxed)
    }
}

/// **MEMORY COMPACTOR**
///
/// Compacts fragmented data structures to reduce memory usage
pub struct MemoryCompactor;

impl MemoryCompactor {
    /// Compact a HashMap by rebuilding it
    pub fn compact_hashmap<K, V>(map: &mut HashMap<K, V>)
    where
        K: Clone + std::hash::Hash + Eq,
        V: Clone,
    {
        if map.len() == 0 {
            return;
        }
        
        // Create new map with exact capacity
        let mut new_map = HashMap::with_capacity(map.len());
        
        // Move all entries to new map
        for (key, value) in map.drain() {
            new_map.insert(key, value);
        }
        
        // Replace old map
        *map = new_map;
    }
    
    /// Compact a Vec by removing unused capacity
    pub fn compact_vec<T>(vec: &mut Vec<T>) {
        vec.shrink_to_fit();
    }
    
    /// Compact a String by removing unused capacity
    pub fn compact_string(s: &mut String) {
        s.shrink_to_fit();
    }
}

/// **MEMORY PROFILER**
///
/// Profiles memory usage patterns and provides optimization suggestions
pub struct MemoryProfiler {
    allocations: Mutex<HashMap<String, AllocationInfo>>,
    start_time: Instant,
}

#[derive(Debug, Clone)]
struct AllocationInfo {
    count: u64,
    total_size: u64,
    average_size: f64,
    last_allocation: Instant,
}

impl MemoryProfiler {
    #[must_use]
    pub fn new() -> Self {
        Self {
            allocations: Mutex::new(HashMap::new()),
            start_time: Instant::now(),
        }
    }
    
    /// Record an allocation for profiling
    pub fn record_allocation(&self, category: &str, size: usize) {
        let mut allocations = self.allocations.lock().unwrap();
        let info = allocations.entry(category.to_string()).or_insert(AllocationInfo {
            count: 0,
            total_size: 0,
            average_size: 0.0,
            last_allocation: Instant::now(),
        });
        
        info.count += 1;
        info.total_size += size as u64;
        info.average_size = info.f64::from(total_size) / info.f64::from(count);
        info.last_allocation = Instant::now();
    }
    
    /// Generate memory usage report
    pub fn generate_report(&self) -> MemoryReport {
        let allocations = self.allocations.lock().unwrap();
        let mut categories = Vec::new();
        
        for (category, info) in allocations.iter() {
            categories.push(CategoryReport {
                category: category.clone(),
                allocation_count: info.count,
                total_bytes: info.total_size,
                average_bytes: info.average_size,
                last_allocation_ago: info.last_allocation.elapsed(),
            });
        }
        
        // Sort by total bytes descending
        categories.sort_by(|a, b| b.total_bytes.cmp(&a.total_bytes));
        
        MemoryReport {
            categories,
            total_runtime: self.start_time.elapsed(),
            suggestions: self.generate_suggestions(&categories),
        }
    }
    
    fn generate_suggestions(&self, categories: &[CategoryReport]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for category in categories.iter().take(5) { // Top 5 consumers
            if category.average_bytes > 1024.0 * 1024.0 { // >1MB average
                suggestions.push(format!(
                    "Consider object pooling for '{}' (avg size: {:.1}KB)",
                    category.category,
                    category.average_bytes / 1024.0
                ));
            }
            
            if category.allocation_count > 10000 {
                suggestions.push(format!(
                    "High allocation frequency in '{}' ({} allocations) - consider batching",
                    category.category,
                    category.allocation_count
                ));
            }
        }
        
        if suggestions.is_empty() {
            suggestions.push("Memory usage appears optimal".to_string());
        }
        
        suggestions
    }
}

/// **MEMORY REPORT**
///
/// Comprehensive memory usage report
#[derive(Debug)]
pub struct MemoryReport {
    pub categories: Vec<CategoryReport>,
    pub total_runtime: Duration,
    pub suggestions: Vec<String>,
}

#[derive(Debug)]
pub struct CategoryReport {
    pub category: String,
    pub allocation_count: u64,
    pub total_bytes: u64,
    pub average_bytes: f64,
    pub last_allocation_ago: Duration,
}

impl MemoryReport {
    /// Print formatted report
    pub fn print(&self) {
        println!("🧠 Memory Usage Report");
        println!("Runtime: {:.2}s"));
        println!();
        
        println!("Top Memory Consumers:");
        for (i, category) in self.categories.iter().enumerate().take(10) {
            println!(
                "{}. {} - {} allocations, {:.1}KB total, {:.1}KB avg",
                i + 1,
                category.category,
                category.allocation_count,
                category.f64::from(total_bytes) / 1024.0,
                category.average_bytes / 1024.0
            );
        }
        
        println!();
        println!("Optimization Suggestions:");
        for suggestion in &self.suggestions {
            println!("  • {suggestion}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_stats() {
        let stats = MemoryStats::new();
        
        stats.record_allocation(1024);
        stats.record_allocation(2048);
        
        assert_eq!(stats.total_allocations.load(Ordering::Relaxed), 2);
        assert_eq!(stats.current_memory_usage.load(Ordering::Relaxed), 3072);
        
        stats.record_deallocation(1024);
        
        assert_eq!(stats.active_allocations.load(Ordering::Relaxed), 1);
        assert_eq!(stats.current_memory_usage.load(Ordering::Relaxed), 2048);
    }
    
    #[test]
    fn test_object_pool() {
        let pool = ObjectPool::new(|| Vec::<u8>::new(), 5);
        
        let obj1 = pool.acquire();
        let obj2 = pool.acquire();
        
        pool.release(obj1);
        pool.release(obj2);
        
        let (hits, misses, hit_rate) = pool.stats();
        assert_eq!(misses, 2); // First two acquisitions were misses
        
        let _obj3 = pool.acquire(); // Should be a hit
        let (hits, _misses, _hit_rate) = pool.stats();
        assert_eq!(hits, 1);
    }
    
    #[test]
    fn test_weak_cache() {
        let cache = WeakCache::new();
        let value = Arc::new("test_value".to_string());
        
        cache.insert("key1".to_string(), value.clone());
        
        // Should hit
        let retrieved = cache.get(&"key1".to_string());
        assert!(retrieved.is_some());
        
        // Drop strong reference
        drop(value);
        
        // Should miss (weak reference is dead)
        let retrieved = cache.get(&"key1".to_string());
        assert!(retrieved.is_none());
    }
    
    #[test]
    fn test_memory_arena() {
        let arena = MemoryArena::new(1024);
        
        let ptr1 = arena.allocate(100);
        let ptr2 = arena.allocate(200);
        
        assert!(ptr1.is_some());
        assert!(ptr2.is_some());
        assert_eq!(arena.allocation_count(), 2);
        
        arena.reset();
        assert_eq!(arena.allocation_count(), 0);
    }
    
    #[test]
    fn test_memory_profiler() {
        let profiler = MemoryProfiler::new();
        
        profiler.record_allocation("strings", 1024);
        profiler.record_allocation("vectors", 2048);
        profiler.record_allocation("strings", 512);
        
        let report = profiler.generate_report();
        
        assert_eq!(report.categories.len(), 2);
        assert!(!report.suggestions.is_empty());
    }

    #[test]
    fn test_memory_stats_efficiency_ratio() {
        let stats = MemoryStats::new();
        
        // Test with no allocations
        assert_eq!(stats.efficiency_ratio(), 0.0);
        
        // Test with allocations but no deallocations
        stats.record_allocation(1024);
        stats.record_allocation(2048);
        assert_eq!(stats.efficiency_ratio(), 0.0);
        
        // Test with partial deallocations
        stats.record_deallocation(1024);
        assert_eq!(stats.efficiency_ratio(), 0.5);
        
        // Test with equal allocations and deallocations
        stats.record_deallocation(2048);
        assert_eq!(stats.efficiency_ratio(), 1.0);
    }

    #[test]
    fn test_memory_stats_potential_leaks() {
        let stats = MemoryStats::new();
        
        // No allocations - no leaks
        assert!(!stats.has_potential_leaks());
        
        // Small number of active allocations - no leak
        stats.record_allocation(1024);
        stats.record_allocation(2048);
        stats.record_deallocation(1024);
        assert!(!stats.has_potential_leaks());
        
        // Large number of active allocations - potential leak
        for _ in 0..10 {
            stats.record_allocation(1024);
        }
        assert!(stats.has_potential_leaks());
    }

    #[test]
    fn test_memory_stats_allocation_sizes() {
        let stats = MemoryStats::new();
        
        // Test large allocation (>1MB)
        stats.record_allocation(2_097_152); // 2MB
        assert_eq!(stats.large_allocation_count.load(Ordering::Relaxed), 1);
        
        // Test small allocation (<1KB)
        stats.record_allocation(512);
        assert_eq!(stats.small_allocation_count.load(Ordering::Relaxed), 1);
        
        // Test medium allocation (between 1KB and 1MB)
        stats.record_allocation(50_000);
        assert_eq!(stats.large_allocation_count.load(Ordering::Relaxed), 1);
        assert_eq!(stats.small_allocation_count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_memory_stats_peak_usage() {
        let stats = MemoryStats::new();
        
        stats.record_allocation(1024);
        assert_eq!(stats.peak_memory_usage.load(Ordering::Relaxed), 1024);
        
        stats.record_allocation(2048);
        assert_eq!(stats.peak_memory_usage.load(Ordering::Relaxed), 3072);
        
        // Deallocation shouldn't affect peak
        stats.record_deallocation(1024);
        assert_eq!(stats.peak_memory_usage.load(Ordering::Relaxed), 3072);
        assert_eq!(stats.current_memory_usage.load(Ordering::Relaxed), 2048);
    }

    #[test]
    fn test_object_pool_max_size() {
        let pool = ObjectPool::new(|| String::new(), 2);
        
        let obj1 = pool.acquire();
        let obj2 = pool.acquire();
        let obj3 = pool.acquire();
        
        pool.release(obj1);
        pool.release(obj2);
        pool.release(obj3); // This should be dropped (pool is full)
        
        let (hits, misses, _) = pool.stats();
        assert_eq!(misses, 3); // All acquisitions were misses
        
        let _obj4 = pool.acquire(); // Should be a hit
        let _obj5 = pool.acquire(); // Should be a hit
        let _obj6 = pool.acquire(); // Should be a miss (pool empty)
        
        let (hits, misses, _) = pool.stats();
        assert_eq!(hits, 2);
        assert_eq!(misses, 4);
    }

    #[test]
    fn test_weak_cache_cleanup() {
        let cache = WeakCache::new();
        let value1 = Arc::new("test1".to_string());
        let value2 = Arc::new("test2".to_string());
        
        cache.insert("key1".to_string(), value1.clone());
        cache.insert("key2".to_string(), value2.clone());
        
        // Both should be accessible
        assert!(cache.get(&"key1".to_string()).is_some());
        assert!(cache.get(&"key2".to_string()).is_some());
        
        // Drop one reference
        drop(value1);
        
        // Cleanup should remove dead references
        cache.cleanup();
        
        assert!(cache.get(&"key1".to_string()).is_none());
        assert!(cache.get(&"key2".to_string()).is_some());
    }

    #[test]
    fn test_weak_cache_stats() {
        let cache = WeakCache::new();
        let value = Arc::new("test".to_string());
        
        cache.insert("key".to_string(), value.clone());
        
        // Hit
        let _retrieved = cache.get(&"key".to_string());
        
        // Miss
        let _not_found = cache.get(&"nonexistent".to_string());
        
        let (hits, misses, hit_rate, size) = cache.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
        assert_eq!(hit_rate, 0.5);
        assert_eq!(size, 1);
    }

    #[test]
    fn test_memory_arena_allocation_failure() {
        let arena = MemoryArena::new(100); // Small arena
        
        // Should succeed
        let ptr1 = arena.allocate(50);
        assert!(ptr1.is_some());
        
        // Should succeed
        let ptr2 = arena.allocate(40);
        assert!(ptr2.is_some());
        
        // Should fail (not enough space)
        let ptr3 = arena.allocate(20);
        assert!(ptr3.is_none());
    }

    #[test]
    fn test_memory_arena_reset() {
        let arena = MemoryArena::new(1000);
        
        let _ptr1 = arena.allocate(100);
        let _ptr2 = arena.allocate(200);
        
        assert_eq!(arena.allocation_count(), 2);
        
        arena.reset();
        
        assert_eq!(arena.allocation_count(), 0);
        
        // Should be able to allocate again after reset
        let ptr3 = arena.allocate(500);
        assert!(ptr3.is_some());
        assert_eq!(arena.allocation_count(), 1);
    }

    #[test]
    fn test_memory_compactor() {
        // Test HashMap compaction
        let mut map: HashMap<String, String> = HashMap::new();
        for i in 0..100 {
            map.insert(format!("key{i}"), format!("value{i}"));
        }
        
        // Remove most entries
        for i in 50..100 {
            map.remove(&format!("key{i}"));
        }
        
        MemoryCompactor::compact_hashmap(&mut map);
        assert_eq!(map.len(), 50);
        
        // Test Vec compaction
        let mut vec: Vec<i32> = (0..100).collect();
        vec.truncate(10);
        
        MemoryCompactor::compact_vec(&mut vec);
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);
        
        // Test String compaction
        let mut s = String::with_capacity(1000);
        s.push_str("small");
        
        MemoryCompactor::compact_string(&mut s);
        assert_eq!(s.len(), 5);
        assert!(s.capacity() <= 10); // Should be much smaller
    }

    #[test]
    fn test_memory_profiler_comprehensive() {
        let profiler = MemoryProfiler::new();
        
        // Record various allocations
        profiler.record_allocation("strings", 1000);
        profiler.record_allocation("strings", 2000);
        profiler.record_allocation("vectors", 5000);
        profiler.record_allocation("hashmaps", 10000);
        profiler.record_allocation("hashmaps", 15000);
        
        let report = profiler.generate_report();
        
        assert_eq!(report.categories.len(), 3);
        
        // Find the strings category
        let strings_category = report.categories.iter()
            .find(|c| c.name == "strings")
            .unwrap();
        
        assert_eq!(strings_category.allocation_count, 2);
        assert_eq!(strings_category.total_size, 3000);
        assert_eq!(strings_category.average_size, 1500);
        
        // Check that report has suggestions
        assert!(!report.suggestions.is_empty());
        
        // Test report printing (just ensure it doesn't panic)
        report.print();
    }

    #[test]
    fn test_memory_pressure_detector() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        
        let detector = MemoryPressureDetector::new(0.8, Duration::from_millis(10));
        
        let callback_triggered = Arc::new(AtomicBool::new(false));
        let callback_flag = callback_triggered.clone();
        
        detector.register_callback(move || {
            callback_flag.store(true, Ordering::Relaxed);
        });
        
        // Manually trigger pressure check
        detector.check_pressure();
        
        // Note: In a real test environment, we can't easily trigger high memory pressure,
        // so we just verify the detector was created and callback was registered
        assert!(!callback_triggered.load(Ordering::Relaxed)); // Should not trigger in normal conditions
    }
} 