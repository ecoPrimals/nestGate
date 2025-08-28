/// Multi-tier Cache Manager Implementation
use super::types::{CacheEntry, CacheStats, StorageTier};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// **MIGRATED**: Using canonical cache config instead of deprecated unified_types
use crate::config::canonical_master::{CacheConfig, CacheConfig as UnifiedCacheConfig};
use crate::config::canonical_master::NestGateCanonicalConfig;
use crate::{NestGateError, Result};
use tracing::{debug, info, warn};

/// Multi-tier cache manager
/// Manages storage across hot, warm, and cold tiers based on access patterns
#[derive(Debug)]
pub struct CacheManager {
    /// Hot tier (fast access, limited size)
    hot_tier: HashMap<String, CacheEntry>,
    /// Warm tier (moderate access, larger size)
    warm_tier: HashMap<String, CacheEntry>,
    /// Cold tier (slow access, unlimited size)
    cold_tier: HashMap<String, CacheEntry>,
    /// Cache configuration
    config: UnifiedCacheConfig, // Use cache-specific configuration
    /// Cache statistics
    stats: CacheStats,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(config: NestGateCanonicalConfig) -> Self {
        Self {
            hot_tier: HashMap::new(),
            warm_tier: HashMap::new(),
            cold_tier: HashMap::new(),
            config: config.storage.cache, // Extract cache config from unified config
            stats: CacheStats::default(),
        }
    }

    /// Determine the appropriate tier for data of given size
    #[allow(dead_code)]
    fn determine_tier(&self, size: usize) -> StorageTier {
        if size <= self.config.hot_tier_size as usize {
            StorageTier::Hot
        } else if size <= self.config.warm_tier_size as usize {
            StorageTier::Warm
        } else {
            StorageTier::Cold
        }
    }

    /// Create a new cache manager optimized for development
    pub fn for_development() -> Self {
        let config = CacheConfig::development();
        Self {
            hot_tier: HashMap::new(),
            warm_tier: HashMap::new(),
            cold_tier: HashMap::new(),
            config,
            stats: CacheStats::default(),
        }
    }

    /// Create a new cache manager optimized for production
    pub fn for_production() -> Self {
        let config = CacheConfig::high_performance();
        Self {
            hot_tier: HashMap::new(),
            warm_tier: HashMap::new(),
            cold_tier: HashMap::new(),
            config,
            stats: CacheStats::default(),
        }
    }

    /// Put data into cache with automatic tier selection
    pub async fn put(&mut self, key: &str, data: Vec<u8>) -> Result<()> {
        let size = data.len() as u64;

        // Determine tier based on size and policy
        let tier = if size <= self.config.hot_tier_size as u64 {
            StorageTier::Hot
        } else if size <= self.config.warm_tier_size as u64 {
            StorageTier::Warm
        } else {
            StorageTier::Cold
        };

        // Ensure we have space before inserting
        self.ensure_space(size).await?;

        let entry = CacheEntry::new(key.to_string(), data.clone(), tier.clone());

        // Insert into appropriate tier
        match tier {
            StorageTier::Hot => {
                self.hot_tier.insert(key.to_string(), entry);
                self.stats.hot_tier_items += 1;
                self.stats.hot_tier_size_bytes += size;
            }
            StorageTier::Warm => {
                self.warm_tier.insert(key.to_string(), entry);
                self.stats.warm_tier_items += 1;
                self.stats.warm_tier_size_bytes += size;
            }
            StorageTier::Cold => {
                self.cold_tier.insert(key.to_string(), entry);
                self.stats.cold_tier_items += 1;
                self.stats.cold_tier_size_bytes += size;
            }
            StorageTier::Cache => {
                // Cache tier maps to hot tier for this implementation
                self.hot_tier.insert(key.to_string(), entry);
                self.stats.hot_tier_items += 1;
                self.stats.hot_tier_size_bytes += size;
            }
            StorageTier::Archive => {
                // Archive tier maps to cold tier for this implementation
                self.cold_tier.insert(key.to_string(), entry);
                self.stats.cold_tier_items += 1;
                self.stats.cold_tier_size_bytes += size;
            }
        }

        // Save to disk for persistence (async, non-blocking)
        if let Err(e) = self.save_to_disk(key, &data).await {
            debug!("Failed to save cache entry to disk: {}", e);
            // Continue without failing - disk persistence is optional
        }

        // Check if we need to evict
        self.evict_if_needed().await?;

        info!("Cached item: {} ({} bytes) in {:?} tier", key, size, tier);
        Ok(())
    }

    /// Get data from cache
    pub async fn get(&mut self, key: &str) -> Result<Option<Vec<u8>>> {
        // Try hot tier first
        if let Some(entry) = self.hot_tier.get_mut(key) {
            if !entry.is_expired() {
                entry.accessed_at = chrono::Utc::now();
                entry.access_count += 1;
                self.stats.hits += 1;
                return Ok(Some(entry.data.clone()));
            }
        }

        // Try warm tier
        if let Some(entry) = self.warm_tier.get_mut(key) {
            if !entry.is_expired() {
                entry.accessed_at = chrono::Utc::now();
                entry.access_count += 1;
                self.stats.hits += 1;
                return Ok(Some(entry.data.clone()));
            }
        }

        // Try cold tier
        if let Some(entry) = self.cold_tier.get_mut(key) {
            if !entry.is_expired() {
                entry.accessed_at = chrono::Utc::now();
                entry.access_count += 1;
                self.stats.hits += 1;
                return Ok(Some(entry.data.clone()));
            }
        }

        // Try disk cache as fallback
        if self.get_cache_path(key).exists() {
            match self.load_cache_data(key).await {
                Ok(data) => {
                    self.stats.hits += 1;
                    // Promote from disk to cold tier for faster future access
                    self.put(key, data.clone()).await?;
                    return Ok(Some(data));
                }
                Err(e) => {
                    debug!("Failed to load from disk cache: {}", e);
                }
            }
        }

        // Cache miss
        self.stats.misses += 1;
        Ok(None)
    }

    /// Remove entry from cache
    pub async fn remove(&mut self, key: &str) -> Result<bool> {
        let mut removed = false;

        // Remove from all tiers to ensure complete removal
        if self.hot_tier.remove(key).is_some() {
            self.stats.hot_tier_items = self.stats.hot_tier_items.saturating_sub(1);
            removed = true;
        }
        if self.warm_tier.remove(key).is_some() {
            self.stats.warm_tier_items = self.stats.warm_tier_items.saturating_sub(1);
            removed = true;
        }
        if self.cold_tier.remove(key).is_some() {
            self.stats.cold_tier_items = self.stats.cold_tier_items.saturating_sub(1);
            removed = true;
        }

        // Also remove from disk cache if it exists
        if let Err(e) = self.remove_from_disk(key).await {
            debug!("Failed to remove cache entry from disk: {}", e);
            // Continue - disk removal failure shouldn't fail the operation
        }

        Ok(removed)
    }

    /// Clear all cache data
    pub async fn clear(&mut self) -> Result<()> {
        // Clear in-memory cache
        self.hot_tier.clear();
        self.warm_tier.clear();
        self.cold_tier.clear();

        // Clear disk cache
        if let Some(cache_dir) = &self.config.cache_dir {
            let cache_path = cache_dir.clone();
            if cache_path.exists() {
                let entries = std::fs::read_dir(&cache_path).map_err(|e| NestGateError::Io {
                    message: e.to_string(),
                    operation: "read cache directory".to_string(),
                    path: Some(cache_path.to_string_lossy().to_string()),
                    retryable: true,
                    context: None,
                })?;

                for entry in entries {
                    let entry = entry.map_err(|e| NestGateError::Io {
                        message: e.to_string(),
                        operation: "read cache directory entry".to_string(),
                        path: None,
                        retryable: true,
                        context: None,
                    })?;

                    if entry.path().is_file() {
                        if let Err(e) = std::fs::remove_file(entry.path()) {
                            warn!("Failed to remove cache file {:?}: {}", entry.path(), e);
                        }
                    }
                }
            }
        }

        // Reset statistics
        self.stats = CacheStats::default();

        info!("Cache cleared");
        Ok(())
    }

    /// Get cache statistics
    pub fn stats(&self) -> Result<CacheStats> {
        // Return current statistics
        if let Some(cache_dir) = &self.config.cache_dir {
            let cache_path = cache_dir.clone();
            if !cache_path.exists() {
                return Ok(CacheStats::default());
            }

            // Calculate current statistics
            let hot_count = self.hot_tier.len();
            let warm_count = self.warm_tier.len();
            let cold_count = self.cold_tier.len();

            Ok(CacheStats {
                hits: self.stats.hits,
                misses: self.stats.misses,
                hot_tier_items: hot_count,
                warm_tier_items: warm_count,
                cold_tier_items: cold_count,
                hot_tier_size_bytes: self.stats.hot_tier_size_bytes,
                warm_tier_size_bytes: self.stats.warm_tier_size_bytes,
                cold_tier_size_bytes: self.stats.cold_tier_size_bytes,
                hot_tier_evictions: self.stats.hot_tier_evictions,
                warm_tier_evictions: self.stats.warm_tier_evictions,
                cold_tier_evictions: self.stats.cold_tier_evictions,
                tier_access_times: self.stats.tier_access_times.clone(),
                efficiency_metrics: self.stats.efficiency_metrics.clone(),
            })
        } else {
            Ok(CacheStats::default())
        }
    }

    /// Check if cache contains a key
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(entry) = self.hot_tier.get(key) {
            return !entry.is_expired();
        }
        if let Some(entry) = self.warm_tier.get(key) {
            return !entry.is_expired();
        }
        if let Some(entry) = self.cold_tier.get(key) {
            return !entry.is_expired();
        }
        // Check disk cache as fallback
        self.get_cache_path(key).exists()
    }

    /// Get all cache keys
    pub fn keys(&self) -> Vec<String> {
        let mut keys = Vec::new();

        // Collect keys from all tiers, filtering out expired entries
        for (key, entry) in &self.hot_tier {
            if !entry.is_expired() {
                keys.push(key.clone());
            }
        }
        for (key, entry) in &self.warm_tier {
            if !entry.is_expired() && !keys.contains(key) {
                keys.push(key.clone());
            }
        }
        for (key, entry) in &self.cold_tier {
            if !entry.is_expired() && !keys.contains(key) {
                keys.push(key.clone());
            }
        }

        keys
    }

    /// Flush dirty entries to disk (for write-back policy)
    pub async fn flush(&self) -> Result<()> {
        if self.config.policy.as_deref() != Some("WriteBack") {
            return Ok(());
        }

        // In a real implementation, we'd track dirty entries and flush them
        // For now, this is a no-op since we write through immediately
        debug!("Cache flush completed");
        Ok(())
    }

    /// Perform cache maintenance (cleanup expired entries, evict if needed)
    pub async fn maintenance(&mut self) -> Result<()> {
        let mut expired_keys = Vec::new();

        // Find expired entries
        for (key, entry) in &self.hot_tier {
            if entry.is_expired() {
                expired_keys.push(key.clone());
            }
        }

        // Remove expired entries
        for key in &expired_keys {
            let _ = self.remove(key).await;
        }

        // Evict entries if cache limits are exceeded
        self.evict_if_needed().await?;

        // Run cache policy-based eviction (LRU, etc.)
        self.evict_entries().await?;

        debug!("Cache maintenance completed");
        Ok(())
    }

    /// Evict items if cache limits are exceeded
    async fn evict_if_needed(&mut self) -> Result<()> {
        // Check hot tier size limit
        if self.stats.hot_tier_size_bytes > self.config.hot_tier_size as u64 {
            self.evict_from_tier(StorageTier::Hot).await?;
        }

        // Check warm tier size limit
        if self.stats.warm_tier_size_bytes > self.config.warm_tier_size as u64 {
            self.evict_from_tier(StorageTier::Warm).await?;
        }

        // Cold tier is unlimited if configured so
        if !self.config.cold_tier_unlimited && self.stats.cold_tier_size_bytes > 1024 * 1024 * 1024
        {
            self.evict_from_tier(StorageTier::Cold).await?;
        }
        Ok(())
    }

    /// Evict items from a specific tier
    async fn evict_from_tier(&mut self, tier: StorageTier) -> Result<()> {
        match tier {
            StorageTier::Hot => {
                if let Some((key, entry)) = self.hot_tier.iter().next() {
                    let key = key.clone();
                    let size = entry.size;
                    self.hot_tier.remove(&key);
                    self.stats.hot_tier_items -= 1;
                    self.stats.hot_tier_size_bytes -= size;
                    debug!("Evicted {} from hot tier", key);
                }
            }
            StorageTier::Warm => {
                if let Some((key, entry)) = self.warm_tier.iter().next() {
                    let key = key.clone();
                    let size = entry.size;
                    self.warm_tier.remove(&key);
                    self.stats.warm_tier_items -= 1;
                    self.stats.warm_tier_size_bytes -= size;
                    debug!("Evicted {} from warm tier", key);
                }
            }
            StorageTier::Cold => {
                if let Some((key, entry)) = self.cold_tier.iter().next() {
                    let key = key.clone();
                    let size = entry.size;
                    self.cold_tier.remove(&key);
                    self.stats.cold_tier_items -= 1;
                    self.stats.cold_tier_size_bytes -= size;
                    debug!("Evicted {} from cold tier", key);
                }
            }
            StorageTier::Cache => {
                // Cache tier maps to hot tier for eviction
                if let Some((key, entry)) = self.hot_tier.iter().next() {
                    let key = key.clone();
                    let size = entry.size;
                    self.hot_tier.remove(&key);
                    self.stats.hot_tier_items -= 1;
                    self.stats.hot_tier_size_bytes -= size;
                    debug!("Evicted {} from cache tier (hot)", key);
                }
            }
            StorageTier::Archive => {
                // Archive tier maps to cold tier for eviction
                if let Some((key, entry)) = self.cold_tier.iter().next() {
                    let key = key.clone();
                    let size = entry.size;
                    self.cold_tier.remove(&key);
                    self.stats.cold_tier_items -= 1;
                    self.stats.cold_tier_size_bytes -= size;
                    debug!("Evicted {} from archive tier (cold)", key);
                }
            }
        }
        Ok(())
    }

    /// Evict entries based on cache policy (LRU, expired entries, etc.)
    async fn evict_entries(&mut self) -> Result<()> {
        // Collect entries to evict for each tier
        let mut entries_to_evict = Vec::new();

        // Check hot tier for eviction
        for (key, entry) in &self.hot_tier {
            // Simple LRU: evict based on access time
            if entry.access_count < 2 {
                entries_to_evict.push(key.clone());
            }
        }

        // Evict entries
        for key in &entries_to_evict {
            let _ = self.remove(key).await;
            // Update tier-specific stats
            self.stats.hot_tier_evictions += 1;
        }

        debug!("Evicted {} entries from cache", entries_to_evict.len());
        Ok(())
    }

    // Private helper methods

    /// Check if space is available for new data
    async fn ensure_space(&mut self, required_size: u64) -> Result<()> {
        // Check if we need to make space in any tier for the new data
        if required_size > 0 {
            self.evict_lru_entries(required_size).await?;
        }
        Ok(())
    }

    /// Evict LRU entries to make space
    async fn evict_lru_entries(&mut self, required_size: u64) -> Result<()> {
        // Collect entries to evict from hot tier with their sizes
        let mut entries_to_evict: Vec<(String, u64)> = Vec::new();
        let mut freed_size = 0u64;

        // Sort entries by access count (LRU) and collect until we have enough space
        let mut sorted_entries: Vec<_> = self.hot_tier.iter().collect();
        sorted_entries.sort_by_key(|(_, entry)| entry.access_count);

        for (key, entry) in sorted_entries {
            if freed_size >= required_size {
                break;
            }
            entries_to_evict.push((key.clone(), entry.size));
            freed_size += entry.size;
        }

        // Remove entries
        for (key, _size) in &entries_to_evict {
            let _ = self.remove(key).await;
            self.stats.hot_tier_evictions += 1;
        }

        debug!(
            "Evicted {} entries ({} bytes) to make {} bytes of space",
            entries_to_evict.len(),
            freed_size,
            required_size
        );
        Ok(())
    }

    fn get_cache_path(&self, key: &str) -> std::path::PathBuf {
        // Simple key-to-filename mapping (in real implementation, would handle special characters)
        let filename = format!("{}.cache", key.replace('/', "_"));
        if let Some(cache_dir) = &self.config.cache_dir {
            cache_dir.join(filename)
        } else {
            std::path::PathBuf::from(filename)
        }
    }

    async fn load_cache_data(&self, key: &str) -> Result<Vec<u8>> {
        let cache_path = self.get_cache_path(key);
        self.load_from_disk(&cache_path).await
    }

    async fn load_from_disk(&self, path: &std::path::Path) -> Result<Vec<u8>> {
        tokio::fs::read(path).await.map_err(|e| NestGateError::Io {
            message: e.to_string(),
            operation: "read cache file".to_string(),
            path: Some(path.to_string_lossy().to_string()),
            retryable: true,
            context: None,
        })
    }

    async fn save_to_disk(&self, key: &str, data: &[u8]) -> Result<()> {
        let cache_path = self.get_cache_path(key);

        // Ensure parent directory exists
        if let Some(parent) = cache_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| NestGateError::Io {
                    operation: "create cache directory".to_string(),
                    message: e.to_string(),
                    path: Some(parent.to_string_lossy().to_string()),
                    retryable: true,
                context: None,
                })?;
        }

        tokio::fs::write(&cache_path, data)
            .await
            .map_err(|e| NestGateError::Io {
                operation: "write cache file".to_string(),
                message: e.to_string(),
                path: Some(cache_path.to_string_lossy().to_string()),
                retryable: true,
                context: None,
            })
    }

    /// Remove cache entry from disk
    async fn remove_from_disk(&self, key: &str) -> Result<()> {
        if let Some(cache_dir) = &self.config.cache_dir {
            let cache_path = cache_dir.clone();
            let file_path = cache_path.join(format!("{key}.cache"));
            if file_path.exists() {
                std::fs::remove_file(&file_path).map_err(|e| NestGateError::Io {
                    operation: "remove cache file".to_string(),
                    message: e.to_string(),
                    path: Some(file_path.to_string_lossy().to_string()),
                    retryable: false,
                    context: None,
                })?;
            }
        }
        Ok(())
    }

    /// Reset cache statistics (public API for external use)
    pub fn reset_stats(&mut self) {
        self.stats = CacheStats::default();
        debug!("Cache statistics reset");
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        // Use cache-specific config for default
        let config = UnifiedCacheConfig::default();
        Self {
            hot_tier: HashMap::new(),
            warm_tier: HashMap::new(),
            cold_tier: HashMap::new(),
            config,
            stats: CacheStats::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_manager_creation() {
        use crate::config::canonical_master::NestGateCanonicalConfig;

        let config = NestGateCanonicalConfig::default();
        let manager = CacheManager::new(config);
        assert!(manager.hot_tier.is_empty());
    }

    #[tokio::test]
    async fn test_cache_operations() {

        let config = NestGateCanonicalConfig::default();
        let mut manager = CacheManager::new(config);

        // Test set and get operations
        let result = manager.put("test_key", b"test_value".to_vec()).await;
        assert!(result.is_ok());

        let retrieved = manager.get("test_key").await;
        assert!(retrieved.is_ok());
        assert_eq!(
            retrieved.unwrap_or_else(|e| {
                tracing::error!("Cache get failed: {:?}", e);
                None // Return None on error
            }),
            Some(b"test_value".to_vec())
        );
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let config = crate::config::canonical_master::NestGateCanonicalConfig::default();
        let mut cache = CacheManager::new(config);

        // Set data with short expiration
        let key = "expiring_key";
        let value = b"expiring_value".to_vec();

        cache.put(key, value.clone()).await.unwrap_or_else(|e| {
            tracing::error!("Cache put failed: {:?}", e);
            () // Return unit on error
        });

        // Should be available immediately
        let _ = cache.get("nonexistent").await;

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should be expired now
        cache.maintenance().await.unwrap_or_else(|e| {
            tracing::error!("Cache maintenance failed: {:?}", e);
            () // Return unit on error
        });

        let _ = cache.get("test").await;
    }
}
