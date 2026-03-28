// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Discovery Cache Module
/// Handles caching and persistence of discovery results including:
/// - Discovery result caching with TTL
/// - Cache invalidation and refresh strategies
/// - Persistent cache storage
/// - Cache statistics and monitoring
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
/// Cache entry with TTL
#[derive(Debug, Clone)]
/// Cacheentry
pub struct CacheEntry {
    /// Value
    pub value: String,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Ttl
    pub ttl: Duration,
    /// Count of access
    pub access_count: u64,
    /// Last Accessed
    pub last_accessed: SystemTime,
}
impl CacheEntry {
    #[must_use]
    /// Creates a new cached entry with the specified value and time-to-live
    pub fn new(value: String, ttl: Duration) -> Self {
        let now = SystemTime::now();
        Self {
            value,
            created_at: now,
            ttl,
            access_count: 0,
            last_accessed: now,
        }
    }

    #[must_use]
    /// Checks if this cached entry has expired based on its TTL
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().unwrap_or(Duration::ZERO) > self.ttl
    }

    /// Access
    pub fn access(&mut self) -> String {
        self.access_count += 1;
        self.last_accessed = SystemTime::now();
        self.value.clone()
    }
}

/// Discovery result cache
#[derive(Debug)]
/// Discoverycache
pub struct DiscoveryCache {
    /// Port discoveries cache
    port_cache: HashMap<String, CacheEntry>,
    /// Endpoint discoveries cache
    endpoint_cache: HashMap<String, CacheEntry>,
    /// Timeout discoveries cache
    timeout_cache: HashMap<String, CacheEntry>,
    /// General cache for other discoveries
    general_cache: HashMap<String, CacheEntry>,
    /// Default TTL for cache entries
    default_ttl: Duration,
    /// Maximum cache size
    max_cache_size: usize,
}
impl Default for DiscoveryCache {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryCache {
    /// Create new discovery cache
    #[must_use]
    pub fn new() -> Self {
        Self {
            port_cache: HashMap::new(),
            endpoint_cache: HashMap::new(),
            timeout_cache: HashMap::new(),
            general_cache: HashMap::new(),
            default_ttl: Duration::from_secs(300), // 5 minutes default
            max_cache_size: 1000,
        }
    }

    /// **PORT CACHING**: Store port discovery result
    pub fn store_port_discovery(&mut self, service_name: &str, port: u16) {
        let key = format!("port:{service_name}");
        let entry = CacheEntry::new(port.to_string(), self.default_ttl);

        self.port_cache.insert(key, entry);
        self.enforce_cache_limits();

        tracing::debug!("Cached port discovery: {} -> {}", service_name, port);
    }

    /// **PORT RETRIEVAL**: Get cached port discovery
    #[must_use]
    pub fn get_port_discovery(&mut self, service_name: &str) -> Option<u16> {
        let key = format!("port:{service_name}");

        if let Some(entry) = self.port_cache.get_mut(&key) {
            if !entry.is_expired() {
                let value = entry.access();
                return value.parse::<u16>().ok();
            }
            self.port_cache.remove(&key);
        }

        None
    }

    /// **ENDPOINT CACHING**: Store endpoint discovery result
    pub fn store_endpoint_discovery(&mut self, service_name: &str, endpoint: &str) {
        let key = format!("endpoint:{service_name}");
        let entry = CacheEntry::new(endpoint.to_string(), self.default_ttl);

        self.endpoint_cache.insert(key, entry);
        self.enforce_cache_limits();

        tracing::debug!(
            "Cached endpoint discovery: {} -> {}",
            service_name,
            endpoint
        );
    }

    /// **ENDPOINT RETRIEVAL**: Get cached endpoint discovery
    #[must_use]
    pub fn get_endpoint_discovery(&mut self, service_name: &str) -> Option<String> {
        let key = format!("endpoint:{service_name}");

        if let Some(entry) = self.endpoint_cache.get_mut(&key) {
            if !entry.is_expired() {
                return Some(entry.access());
            }
            self.endpoint_cache.remove(&key);
        }

        None
    }

    /// **TIMEOUT CACHING**: Store timeout discovery result
    pub fn store_timeout_discovery(&mut self, service_name: &str, timeout: Duration) {
        let key = format!("timeout:{service_name}");
        let entry = CacheEntry::new(format!("{timeout:?}"), self.default_ttl);

        self.timeout_cache.insert(key, entry);
        self.enforce_cache_limits();

        tracing::debug!(
            "Cached timeout discovery: {} -> {:?}",
            service_name,
            timeout
        );
    }

    /// **TIMEOUT RETRIEVAL**: Get cached timeout discovery
    #[must_use]
    pub fn get_timeout_discovery(&mut self, service_name: &str) -> Option<Duration> {
        let key = format!("timeout:{service_name}");

        if let Some(entry) = self.timeout_cache.get_mut(&key) {
            if !entry.is_expired() {
                let value = entry.access();
                // Parse duration string back to Duration (simplified)
                if let Ok(secs) = value
                    .trim_matches(|c: char| !c.is_ascii_digit())
                    .parse::<u64>()
                {
                    return Some(Duration::from_secs(secs));
                }
            } else {
                self.timeout_cache.remove(&key);
            }
        }

        None
    }

    /// **GENERAL CACHING**: Store general discovery result
    pub async fn store_discovery(&mut self, key: &str, value: &str, ttl: Option<Duration>) {
        let entry = CacheEntry::new(value.to_string(), ttl.unwrap_or(self.default_ttl));

        self.general_cache.insert(key.to_string(), entry);
        self.enforce_cache_limits();

        tracing::debug!("Cached discovery: {} -> {}", key, value);
    }

    /// **GENERAL RETRIEVAL**: Get cached discovery result
    #[must_use]
    pub fn get_discovery(&mut self, key: &str) -> Option<String> {
        if let Some(entry) = self.general_cache.get_mut(key) {
            if !entry.is_expired() {
                return Some(entry.access());
            }
            self.general_cache.remove(key);
        }

        None
    }

    /// **CACHE INVALIDATION**: Invalidate specific cache entries
    pub fn invalidate(&mut self, pattern: &str) {
        let keys_to_remove: Vec<String> = self
            .general_cache
            .keys()
            .filter(|key| key.contains(pattern))
            .cloned()
            .collect();

        for key in keys_to_remove {
            self.general_cache.remove(&key);
            tracing::debug!("Invalidated cache entry: {}", key);
        }
    }

    /// **CACHE CLEANUP**: Remove expired entries
    pub fn cleanup_expired(&mut self) {
        // Cleanup port cache
        self.port_cache.retain(|_, entry| !entry.is_expired());

        // Cleanup endpoint cache
        self.endpoint_cache.retain(|_, entry| !entry.is_expired());

        // Cleanup timeout cache
        self.timeout_cache.retain(|_, entry| !entry.is_expired());

        // Cleanup general cache
        self.general_cache.retain(|_, entry| !entry.is_expired());

        tracing::debug!("Cleaned up expired cache entries");
    }

    /// **CACHE LIMITS**: Enforce cache size limits using LRU eviction
    fn enforce_cache_limits(&mut self) {
        let total_entries = self.port_cache.len()
            + self.endpoint_cache.len()
            + self.timeout_cache.len()
            + self.general_cache.len();

        if total_entries > self.max_cache_size {
            // Simple LRU: remove least recently accessed entries from general cache first
            let to_remove_count = total_entries - self.max_cache_size;
            let keys_to_remove: Vec<String> = {
                let mut entries: Vec<_> = self
                    .general_cache
                    .iter()
                    .map(|(k, v)| (k.clone(), v.last_accessed))
                    .collect();
                entries.sort_by(|a, b| a.1.cmp(&b.1));

                entries
                    .into_iter()
                    .take(to_remove_count)
                    .map(|(key, _)| key)
                    .collect()
            };

            for key in keys_to_remove {
                self.general_cache.remove(&key);
            }

            tracing::debug!(
                "Evicted {} cache entries to enforce limits",
                to_remove_count
            );
        }
    }

    /// **CACHE STATISTICS**: Get cache statistics
    #[must_use]
    pub fn get_cache_stats(&self) -> usize {
        self.port_cache.len()
            + self.endpoint_cache.len()
            + self.timeout_cache.len()
            + self.general_cache.len()
    }

    /// **DETAILED STATISTICS**: Get detailed cache statistics
    #[must_use]
    pub fn get_detailed_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();

        stats.insert("port_cache_size".to_string(), self.port_cache.len());
        stats.insert("endpoint_cache_size".to_string(), self.endpoint_cache.len());
        stats.insert("timeout_cache_size".to_string(), self.timeout_cache.len());
        stats.insert("general_cache_size".to_string(), self.general_cache.len());
        stats.insert(
            "total_cache_size".to_string(),
            self.port_cache.len()
                + self.endpoint_cache.len()
                + self.timeout_cache.len()
                + self.general_cache.len(),
        );
        stats.insert("max_cache_size".to_string(), self.max_cache_size);

        stats
    }

    /// **CACHE CONFIGURATION**: Update cache configuration
    pub async fn configure(&mut self, default_ttl: Duration, max_size: usize) {
        self.default_ttl = default_ttl;
        self.max_cache_size = max_size;

        // Enforce new limits immediately
        self.enforce_cache_limits();

        tracing::info!(
            "Updated cache configuration: TTL={:?}, MaxSize={}",
            default_ttl,
            max_size
        );
    }
}
