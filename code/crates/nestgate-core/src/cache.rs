//! Cache management module for NestGate
//!
//! This module provides caching functionality for the NestGate system,
//! supporting hot, warm, and cold storage tiers.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use crate::error::{NestGateError, Result};
use crate::StorageTier;

/// Cache policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CachePolicy {
    /// No caching
    None,
    
    /// Read-only caching
    ReadOnly,
    
    /// Write-through caching (writes go to both cache and backing store)
    WriteThrough,
    
    /// Write-back caching (writes go to cache, then are flushed to backing store)
    WriteBack,
}

impl std::fmt::Display for CachePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CachePolicy::None => write!(f, "none"),
            CachePolicy::ReadOnly => write!(f, "read-only"),
            CachePolicy::WriteThrough => write!(f, "write-through"),
            CachePolicy::WriteBack => write!(f, "write-back"),
        }
    }
}

impl Default for CachePolicy {
    fn default() -> Self {
        Self::WriteThrough
    }
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Cache policy
    pub policy: CachePolicy,
    
    /// Cache storage tier
    pub tier: StorageTier,
    
    /// Cache size limit in bytes (0 = unlimited)
    pub size_limit: u64,
    
    /// Time-to-live for cached items in seconds (0 = unlimited)
    pub ttl: u64,
    
    /// Whether to cache metadata
    pub cache_metadata: bool,
    
    /// Path to cache directory
    pub cache_dir: PathBuf,
    
    /// Flush interval for write-back cache in seconds
    pub flush_interval: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            policy: CachePolicy::default(),
            tier: StorageTier::Warm,
            size_limit: 1024 * 1024 * 1024, // 1 GB
            ttl: 3600, // 1 hour
            cache_metadata: true,
            cache_dir: PathBuf::from("/var/cache/nestgate"),
            flush_interval: 60, // 1 minute
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total hits
    pub hits: u64,
    
    /// Total misses
    pub misses: u64,
    
    /// Total evictions
    pub evictions: u64,
    
    /// Total writes
    pub writes: u64,
    
    /// Total flushes
    pub flushes: u64,
    
    /// Current size in bytes
    pub current_size: u64,
    
    /// Maximum size reached in bytes
    pub max_size: u64,
    
    /// Hit ratio (hits / (hits + misses))
    pub hit_ratio: f64,
}

/// Cached item
#[derive(Debug)]
struct CachedItem {
    /// Creation time
    created_at: Instant,
    
    /// Last accessed time
    accessed_at: Instant,
    
    /// Size in bytes
    size: u64,
    
    /// Storage tier
    tier: StorageTier,
    
    /// Path to cached file
    path: PathBuf,
    
    /// Path in original storage
    original_path: PathBuf,
    
    /// Whether the item is dirty (needs to be flushed)
    dirty: bool,
}

/// Cache manager for handling caching across storage tiers
#[derive(Debug)]
pub struct CacheManager {
    /// Configuration
    config: CacheConfig,
    
    /// Cache items
    items: Arc<RwLock<HashMap<String, CachedItem>>>,
    
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
}

impl CacheManager {
    /// Create a new cache manager with default configuration
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }
    
    /// Create a new cache manager with specific configuration
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            config,
            items: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }
    
    /// Get an item from the cache
    pub fn get(&self, key: &str) -> Result<Option<PathBuf>> {
        let mut items = match self.items.write() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        let mut stats = match self.stats.write() {
            Ok(stats) => stats,
            Err(_) => return Err(NestGateError::Internal("Stats lock poisoned".to_string())),
        };
        
        if let Some(item) = items.get_mut(key) {
            // Check if item has expired
            if self.config.ttl > 0 {
                let age = item.created_at.elapsed().as_secs();
                if age > self.config.ttl {
                    // Item has expired, remove it
                    items.remove(key);
                    stats.evictions += 1;
                    return Ok(None);
                }
            }
            
            // Update access time
            item.accessed_at = Instant::now();
            
            // Increment hits
            stats.hits += 1;
            stats.hit_ratio = stats.hits as f64 / (stats.hits + stats.misses) as f64;
            
            Ok(Some(item.path.clone()))
        } else {
            // Increment misses
            stats.misses += 1;
            stats.hit_ratio = stats.hits as f64 / (stats.hits + stats.misses) as f64;
            
            Ok(None)
        }
    }
    
    /// Put an item in the cache
    pub fn put(&self, key: &str, path: PathBuf, size: u64, original_path: PathBuf) -> Result<()> {
        // If cache policy is None, don't cache
        if self.config.policy == CachePolicy::None {
            return Ok(());
        }
        
        let mut items = match self.items.write() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        let mut stats = match self.stats.write() {
            Ok(stats) => stats,
            Err(_) => return Err(NestGateError::Internal("Stats lock poisoned".to_string())),
        };
        
        // Check if we need to evict items to make room
        if self.config.size_limit > 0 {
            let current_size: u64 = items.values().map(|item| item.size).sum();
            if current_size + size > self.config.size_limit {
                self.evict_items(current_size + size - self.config.size_limit)?;
            }
        }
        
        // Add the new item
        let item = CachedItem {
            created_at: Instant::now(),
            accessed_at: Instant::now(),
            size,
            tier: self.config.tier.clone(),
            path,
            original_path,
            dirty: false,
        };
        
        items.insert(key.to_string(), item);
        
        // Update stats
        stats.writes += 1;
        stats.current_size += size;
        if stats.current_size > stats.max_size {
            stats.max_size = stats.current_size;
        }
        
        Ok(())
    }
    
    /// Remove an item from the cache
    pub fn remove(&self, key: &str) -> Result<bool> {
        let mut items = match self.items.write() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        let mut stats = match self.stats.write() {
            Ok(stats) => stats,
            Err(_) => return Err(NestGateError::Internal("Stats lock poisoned".to_string())),
        };
        
        if let Some(item) = items.remove(key) {
            // Update stats
            stats.evictions += 1;
            stats.current_size -= item.size;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Mark an item as dirty (needs to be flushed)
    pub fn mark_dirty(&self, key: &str) -> Result<bool> {
        // Only applicable for write-back cache
        if self.config.policy != CachePolicy::WriteBack {
            return Ok(false);
        }
        
        let mut items = match self.items.write() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        if let Some(item) = items.get_mut(key) {
            item.dirty = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Flush dirty items to backing store
    pub fn flush(&self) -> Result<u64> {
        // Only applicable for write-back cache
        if self.config.policy != CachePolicy::WriteBack {
            return Ok(0);
        }
        
        let mut items = match self.items.write() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        let mut stats = match self.stats.write() {
            Ok(stats) => stats,
            Err(_) => return Err(NestGateError::Internal("Stats lock poisoned".to_string())),
        };
        
        let mut flushed = 0;
        
        for item in items.values_mut() {
            if item.dirty {
                // In a real implementation, this would copy the cached file
                // back to the original location, but for now we just mark it as clean
                item.dirty = false;
                flushed += 1;
            }
        }
        
        // Update stats
        stats.flushes += 1;
        
        Ok(flushed)
    }
    
    /// Clear all items from the cache
    pub fn clear(&self) -> Result<u64> {
        let mut items = match self.items.write() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        let mut stats = match self.stats.write() {
            Ok(stats) => stats,
            Err(_) => return Err(NestGateError::Internal("Stats lock poisoned".to_string())),
        };
        
        let count = items.len() as u64;
        
        // Clear items
        items.clear();
        
        // Reset stats
        stats.current_size = 0;
        stats.evictions += count;
        
        Ok(count)
    }
    
    /// Get cache statistics
    pub fn stats(&self) -> Result<CacheStats> {
        let stats = match self.stats.read() {
            Ok(stats) => stats.clone(),
            Err(_) => return Err(NestGateError::Internal("Stats lock poisoned".to_string())),
        };
        
        Ok(stats)
    }
    
    /// Get current cache size
    pub fn size(&self) -> Result<u64> {
        let items = match self.items.read() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        let size = items.values().map(|item| item.size).sum();
        
        Ok(size)
    }
    
    /// Get number of items in cache
    pub fn item_count(&self) -> Result<usize> {
        let items = match self.items.read() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        Ok(items.len())
    }
    
    /// Evict items from cache to free up space
    fn evict_items(&self, bytes_needed: u64) -> Result<u64> {
        let mut items = match self.items.write() {
            Ok(items) => items,
            Err(_) => return Err(NestGateError::Internal("Cache lock poisoned".to_string())),
        };
        
        let mut stats = match self.stats.write() {
            Ok(stats) => stats,
            Err(_) => return Err(NestGateError::Internal("Stats lock poisoned".to_string())),
        };
        
        // Build a list of items sorted by access time (LRU)
        let mut item_keys: Vec<(String, Instant)> = items
            .iter()
            .map(|(key, item)| (key.clone(), item.accessed_at))
            .collect();
        
        // Sort by access time (oldest first)
        item_keys.sort_by(|a, b| a.1.cmp(&b.1));
        
        let mut bytes_freed = 0;
        let mut evicted = 0;
        
        // Evict items until we've freed enough space
        for (key, _) in item_keys {
            if bytes_freed >= bytes_needed {
                break;
            }
            
            if let Some(item) = items.remove(&key) {
                bytes_freed += item.size;
                evicted += 1;
                
                // Update stats
                stats.evictions += 1;
                stats.current_size -= item.size;
            }
        }
        
        Ok(evicted)
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-tier cache manager
///
/// Manages multiple cache tiers (hot, warm, cold) with automatic promotion/demotion
#[derive(Debug)]
pub struct MultiTierCache {
    /// Hot tier cache (fastest, smallest)
    hot: CacheManager,
    
    /// Warm tier cache (balanced)
    warm: CacheManager,
    
    /// Cold tier cache (slowest, largest)
    cold: CacheManager,
    
    /// Access counts for promotion/demotion
    access_counts: Arc<RwLock<HashMap<String, u64>>>,
    
    /// Access threshold for promotion
    promotion_threshold: u64,
    
    /// Access count decay interval
    decay_interval: Duration,
}

impl MultiTierCache {
    /// Create a new multi-tier cache with default configurations
    pub fn new() -> Self {
        // Create hot tier config
        let hot_config = CacheConfig {
            tier: StorageTier::Hot,
            size_limit: 256 * 1024 * 1024, // 256 MB
            ttl: 3600, // 1 hour
            ..Default::default()
        };
        
        // Create warm tier config
        let warm_config = CacheConfig {
            tier: StorageTier::Warm,
            size_limit: 1024 * 1024 * 1024, // 1 GB
            ttl: 86400, // 24 hours
            ..Default::default()
        };
        
        // Create cold tier config
        let cold_config = CacheConfig {
            tier: StorageTier::Cold,
            size_limit: 10 * 1024 * 1024 * 1024, // 10 GB
            ttl: 7 * 86400, // 7 days
            ..Default::default()
        };
        
        Self {
            hot: CacheManager::with_config(hot_config),
            warm: CacheManager::with_config(warm_config),
            cold: CacheManager::with_config(cold_config),
            access_counts: Arc::new(RwLock::new(HashMap::new())),
            promotion_threshold: 5,
            decay_interval: Duration::from_secs(3600), // 1 hour
        }
    }
    
    /// Get an item from the cache
    pub fn get(&self, key: &str) -> Result<Option<PathBuf>> {
        // Try hot tier first
        if let Some(path) = self.hot.get(key)? {
            self.increment_access_count(key)?;
            return Ok(Some(path));
        }
        
        // Try warm tier next
        if let Some(path) = self.warm.get(key)? {
            self.increment_access_count(key)?;
            
            // Check if item should be promoted to hot tier
            if self.should_promote(key, StorageTier::Warm)? {
                // Promotion logic would go here in a real implementation
            }
            
            return Ok(Some(path));
        }
        
        // Try cold tier last
        if let Some(path) = self.cold.get(key)? {
            self.increment_access_count(key)?;
            
            // Check if item should be promoted to warm tier
            if self.should_promote(key, StorageTier::Cold)? {
                // Promotion logic would go here in a real implementation
            }
            
            return Ok(Some(path));
        }
        
        // Item not found in any tier
        Ok(None)
    }
    
    /// Put an item in the cache
    pub fn put(&self, key: &str, path: PathBuf, size: u64, original_path: PathBuf, tier: StorageTier) -> Result<()> {
        match tier {
            StorageTier::Hot => self.hot.put(key, path, size, original_path),
            StorageTier::Warm => self.warm.put(key, path, size, original_path),
            StorageTier::Cold => self.cold.put(key, path, size, original_path),
            StorageTier::Cache => self.hot.put(key, path, size, original_path),
        }
    }
    
    /// Increment access count for an item
    fn increment_access_count(&self, key: &str) -> Result<u64> {
        let mut access_counts = match self.access_counts.write() {
            Ok(counts) => counts,
            Err(_) => return Err(NestGateError::Internal("Access counts lock poisoned".to_string())),
        };
        
        let count = access_counts.entry(key.to_string()).or_insert(0);
        *count += 1;
        
        Ok(*count)
    }
    
    /// Check if an item should be promoted to a higher tier
    fn should_promote(&self, key: &str, current_tier: StorageTier) -> Result<bool> {
        let access_counts = match self.access_counts.read() {
            Ok(counts) => counts,
            Err(_) => return Err(NestGateError::Internal("Access counts lock poisoned".to_string())),
        };
        
        if let Some(count) = access_counts.get(key) {
            Ok(match current_tier {
                StorageTier::Cold => *count >= self.promotion_threshold,
                StorageTier::Warm => *count >= self.promotion_threshold * 2,
                _ => false,
            })
        } else {
            Ok(false)
        }
    }
    
    /// Clear all tiers
    pub fn clear(&self) -> Result<u64> {
        let hot_count = self.hot.clear()?;
        let warm_count = self.warm.clear()?;
        let cold_count = self.cold.clear()?;
        
        let mut access_counts = match self.access_counts.write() {
            Ok(counts) => counts,
            Err(_) => return Err(NestGateError::Internal("Access counts lock poisoned".to_string())),
        };
        
        access_counts.clear();
        
        Ok(hot_count + warm_count + cold_count)
    }
    
    /// Flush all tiers
    pub fn flush(&self) -> Result<u64> {
        let hot_count = self.hot.flush()?;
        let warm_count = self.warm.flush()?;
        let cold_count = self.cold.flush()?;
        
        Ok(hot_count + warm_count + cold_count)
    }
    
    /// Get combined statistics
    pub fn stats(&self) -> Result<HashMap<StorageTier, CacheStats>> {
        let mut result = HashMap::new();
        
        result.insert(StorageTier::Hot, self.hot.stats()?);
        result.insert(StorageTier::Warm, self.warm.stats()?);
        result.insert(StorageTier::Cold, self.cold.stats()?);
        
        Ok(result)
    }
}

impl Default for MultiTierCache {
    fn default() -> Self {
        Self::new()
    }
} 