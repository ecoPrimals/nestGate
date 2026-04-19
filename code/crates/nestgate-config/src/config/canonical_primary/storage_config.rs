// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **STORAGE CONFIGURATION**
///
/// Storage and ZFS configuration types.
/// This module contains all storage-related settings including ZFS pools,
/// caching, replication, and backend configurations.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== SECTION ====================

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Storage
pub struct StorageConfig {
    /// Enable storage
    pub enabled: bool,
    /// Default storage backend
    pub default_backend: String,
    /// Storage backends
    pub backends: HashMap<String, StorageBackend>,
    /// ZFS configuration
    pub zfs: ZfsConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Storage-specific settings
    pub storage_settings: HashMap<String, serde_json::Value>,
}
/// Storage backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagebackend
pub struct StorageBackend {
    /// Backend type
    pub backend_type: String,
    /// Backend configuration
    pub config: HashMap<String, serde_json::Value>,
}
/// ZFS configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Zfs
pub struct ZfsConfig {
    /// Enable ZFS
    pub enabled: bool,
    /// ZFS pools
    pub pools: Vec<ZfsPool>,
    /// ZFS settings
    pub zfs_settings: HashMap<String, serde_json::Value>,
}
/// ZFS pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfspool
pub struct ZfsPool {
    /// Pool name
    pub name: String,
    /// Pool devices
    pub devices: Vec<String>,
    /// Pool properties
    pub properties: HashMap<String, String>,
}
/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Cache
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    /// Cache size (bytes)
    pub size_bytes: u64,
    /// Cache type
    pub cache_type: String,
    /// Cache directory path
    pub cache_dir: Option<std::path::PathBuf>,
    /// Cache policy (LRU, LFU, etc.)
    pub policy: Option<String>,
    /// Hot tier size in bytes
    pub hot_tier_size: Option<u64>,
    /// Warm tier size in bytes
    pub warm_tier_size: Option<u64>,
    /// Whether cold tier has unlimited size
    pub cold_tier_unlimited: Option<bool>,
    /// TTL in seconds
    pub ttl_seconds: Option<u64>,
    /// Cache settings
    pub cache_settings: HashMap<String, serde_json::Value>,
}
// ==================== SECTION ====================

impl Default for StorageConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            default_backend: "filesystem".to_string(),
            backends: HashMap::new(),
            zfs: ZfsConfig::default(),
            cache: CacheConfig::default(),
            storage_settings: HashMap::new(),
        }
    }
}

impl Default for CacheConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            size_bytes: 1024 * 1024 * 1024, // 1GB
            cache_type: "lru".to_string(),
            cache_dir: None,
            policy: None,
            hot_tier_size: None,
            warm_tier_size: None,
            cold_tier_unlimited: None,
            ttl_seconds: None,
            cache_settings: HashMap::new(),
        }
    }
}

impl CacheConfig {
    /// Development cache configuration
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: true,
            size_bytes: 256 * 1024 * 1024, // 256MB
            cache_type: "lru".to_string(),
            cache_dir: Some("/tmp/nestgate/cache".to_string().into()),
            policy: Some("lru".to_string()),
            hot_tier_size: Some(64 * 1024 * 1024),   // 64MB
            warm_tier_size: Some(128 * 1024 * 1024), // 128MB
            cold_tier_unlimited: Some(false),
            ttl_seconds: Some(3600), // 1 hour
            cache_settings: HashMap::new(),
        }
    }

    /// High performance cache configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self {
            enabled: true,
            size_bytes: 4 * 1024 * 1024 * 1024, // 4GB
            cache_type: "lru".to_string(),
            cache_dir: Some("/var/cache/nestgate".to_string().into()),
            policy: Some("lru".to_string()),
            hot_tier_size: Some(1024 * 1024 * 1024), // 1GB
            warm_tier_size: Some(2 * 1024 * 1024 * 1024), // 2GB
            cold_tier_unlimited: Some(true),
            ttl_seconds: Some(86400), // 24 hours
            cache_settings: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_config_default_fields() {
        let s = StorageConfig::default();
        assert!(s.enabled);
        assert_eq!(s.default_backend, "filesystem");
        assert!(s.backends.is_empty());
        assert!(s.zfs.pools.is_empty());
        assert!(!s.zfs.enabled);
        assert!(s.cache.enabled);
        assert_eq!(s.cache.size_bytes, 1024 * 1024 * 1024);
    }

    #[test]
    fn zfs_config_default_is_empty_disabled() {
        let z = ZfsConfig::default();
        assert!(!z.enabled);
        assert!(z.pools.is_empty());
        assert!(z.zfs_settings.is_empty());
    }

    #[test]
    fn cache_config_development_sets_dir_and_tiers() {
        let c = CacheConfig::development();
        assert!(c.cache_dir.is_some());
        assert_eq!(c.policy.as_deref(), Some("lru"));
        assert_eq!(c.hot_tier_size, Some(64 * 1024 * 1024));
    }

    #[test]
    fn cache_config_high_performance_enables_cold_unlimited() {
        let c = CacheConfig::high_performance();
        assert_eq!(c.cold_tier_unlimited, Some(true));
        assert!(c.cache_dir.is_some());
    }

    #[test]
    fn storage_config_serde_roundtrip() {
        let original = StorageConfig::default();
        let json = serde_json::to_string(&original).expect("serialize StorageConfig");
        let parsed: StorageConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original.enabled, parsed.enabled);
        assert_eq!(original.default_backend, parsed.default_backend);
    }

    #[test]
    fn storage_backend_roundtrip() {
        let mut backends = HashMap::new();
        backends.insert(
            "main".to_string(),
            StorageBackend {
                backend_type: "fs".to_string(),
                config: HashMap::new(),
            },
        );
        let s = StorageConfig {
            backends,
            ..StorageConfig::default()
        };
        assert!(s.backends.contains_key("main"));
    }
}
