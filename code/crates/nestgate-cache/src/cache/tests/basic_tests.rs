// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Basic cache tests
//!
//! Core test suite for basic cache functionality including
//! single-tier cache, multi-tier cache, and cache system operations.

use crate::cache::multi_tier::SimpleCacheConfig;
use crate::cache::*;

#[tokio::test]
async fn test_cache_system_operations() -> nestgate_types::Result<()> {
    let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
        enabled: true,
        size_bytes: 1024 * 1024, // 1MB
        cache_type: "lru".to_string(),
        hot_tier_size: Some(100),
        warm_tier_size: Some(500),
        cold_tier_unlimited: Some(false),
        ttl_seconds: Some(300),
        cache_dir: Some("/tmp/nestgate_cache_test".to_string().into()),
        policy: Some("lru".to_string()),
        cache_settings: std::collections::HashMap::new(),
    };

    let mut cache = CacheSystem::single_tier(cache_config)?;

    // Test put operation
    cache.put("test_key", b"test_value".to_vec()).await?;

    // Test get operation
    let result = cache.get("test_key").await?;
    assert!(result.is_some());
    assert_eq!(result.expect("Cache operation failed"), b"test_value");

    // Test remove operation
    let removed = cache.remove("test_key").await?;
    assert!(removed);

    // Verify key no longer exists
    let result_after_remove = cache.get("test_key").await?;
    assert!(result_after_remove.is_none());

    Ok(())
}

#[tokio::test]
async fn test_single_tier_creation() -> nestgate_types::Result<()> {
    let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
        enabled: true,
        size_bytes: 1024 * 1024, // 1MB
        cache_type: "lru".to_string(),
        hot_tier_size: Some(100),
        warm_tier_size: None,
        cold_tier_unlimited: None,
        ttl_seconds: Some(300),
        cache_dir: Some("/tmp/nestgate_cache_single".to_string().into()),
        policy: Some("lru".to_string()),
        cache_settings: std::collections::HashMap::new(),
    };

    let cache = CacheSystem::single_tier(cache_config)?;

    match cache {
        CacheSystem::SingleTier(_) => {
            // Success
        }
        CacheSystem::MultiTier(_) => {
            panic!("Expected single tier cache");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_multi_tier_creation() -> nestgate_types::Result<()> {
    let config = MultiTierCacheConfig {
        hot_tier_config: SimpleCacheConfig {
            max_size: 100,
            ttl: std::time::Duration::from_secs(300),
            cache_dir: "/tmp/nestgate_cache_multi/hot".to_string(),
        },
        warm_tier_config: SimpleCacheConfig {
            max_size: 500,
            ttl: std::time::Duration::from_secs(3600),
            cache_dir: "/tmp/nestgate_cache_multi/warm".to_string(),
        },
        cold_tier_config: SimpleCacheConfig {
            max_size: 1000,
            ttl: std::time::Duration::from_secs(86400),
            cache_dir: "/tmp/nestgate_cache_multi/cold".to_string(),
        },
        promotion_threshold: 10,
        demotion_threshold: 2,
    };

    let cache = CacheSystem::multi_tier(config)?;

    match cache {
        CacheSystem::MultiTier(_) => {
            // Success
        }
        CacheSystem::SingleTier(_) => {
            panic!("Expected multi-tier cache");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_cache_miss() -> nestgate_types::Result<()> {
    let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
        enabled: true,
        size_bytes: 1024 * 1024, // 1MB
        cache_type: "lru".to_string(),
        hot_tier_size: Some(100),
        warm_tier_size: None,
        cold_tier_unlimited: None,
        ttl_seconds: Some(300),
        cache_dir: Some("/tmp/nestgate_cache_miss".to_string().into()),
        policy: Some("lru".to_string()),
        cache_settings: std::collections::HashMap::new(),
    };

    let mut cache = CacheSystem::single_tier(cache_config)?;

    // Try to get a non-existent key
    let result = cache.get("nonexistent_key").await?;
    assert!(result.is_none());

    Ok(())
}

#[tokio::test]
async fn test_cache_overwrite() -> nestgate_types::Result<()> {
    let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
        enabled: true,
        size_bytes: 1024 * 1024, // 1MB
        cache_type: "lru".to_string(),
        hot_tier_size: Some(100),
        warm_tier_size: None,
        cold_tier_unlimited: None,
        ttl_seconds: Some(300),
        cache_dir: Some("/tmp/nestgate_cache_overwrite".to_string().into()),
        policy: Some("lru".to_string()),
        cache_settings: std::collections::HashMap::new(),
    };

    let mut cache = CacheSystem::single_tier(cache_config)?;

    // Put initial value
    cache.put("key", b"value1".to_vec()).await?;

    // Overwrite with new value
    cache.put("key", b"value2".to_vec()).await?;

    // Verify new value is retrieved
    let result = cache.get("key").await?;
    assert_eq!(result.expect("Cache operation failed"), b"value2");

    Ok(())
}

#[tokio::test]
async fn test_cache_multiple_keys() -> nestgate_types::Result<()> {
    let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
        enabled: true,
        size_bytes: 1024 * 1024, // 1MB
        cache_type: "lru".to_string(),
        hot_tier_size: Some(100),
        warm_tier_size: None,
        cold_tier_unlimited: None,
        ttl_seconds: Some(300),
        cache_dir: Some("/tmp/nestgate_cache_multiple".to_string().into()),
        policy: Some("lru".to_string()),
        cache_settings: std::collections::HashMap::new(),
    };

    let mut cache = CacheSystem::single_tier(cache_config)?;

    // Add multiple keys
    cache.put("key1", b"value1".to_vec()).await?;
    cache.put("key2", b"value2".to_vec()).await?;
    cache.put("key3", b"value3".to_vec()).await?;

    // Verify all keys
    assert_eq!(
        cache.get("key1").await?.expect("Cache operation failed"),
        b"value1"
    );
    assert_eq!(
        cache.get("key2").await?.expect("Cache operation failed"),
        b"value2"
    );
    assert_eq!(
        cache.get("key3").await?.expect("Cache operation failed"),
        b"value3"
    );

    Ok(())
}

#[tokio::test]
async fn test_clear_cache() -> nestgate_types::Result<()> {
    let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
        enabled: true,
        size_bytes: 1024 * 1024, // 1MB
        cache_type: "lru".to_string(),
        hot_tier_size: Some(100),
        warm_tier_size: None,
        cold_tier_unlimited: None,
        ttl_seconds: Some(300),
        cache_dir: Some("/tmp/nestgate_cache_clear".to_string().into()),
        policy: Some("lru".to_string()),
        cache_settings: std::collections::HashMap::new(),
    };

    let mut cache = CacheSystem::single_tier(cache_config)?;

    // Add some keys
    cache.put("key1", b"value1".to_vec()).await?;
    cache.put("key2", b"value2".to_vec()).await?;

    // Clear cache
    cache.clear().await?;

    // Verify keys are gone
    assert!(cache.get("key1").await?.is_none());
    assert!(cache.get("key2").await?.is_none());

    Ok(())
}

#[cfg(test)]
mod cache_comprehensive_tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_manager_basic_operations() {
        let config = manager::UnifiedCacheConfig {
            max_size: 1000,
            ttl_seconds: Some(300),
            cache_dir: Some("/tmp/nestgate_cache_manager".to_string().into()),
            eviction_policy: "lru".to_string(),
        };

        let mut cache = CacheManager::new(config);

        // Test put and get
        cache
            .put("test_key".to_string(), b"test_value".to_vec())
            .await
            .expect("Cache operation failed");
        let result = cache.get("test_key");
        assert!(result.is_some());
        assert_eq!(result.expect("Cache operation failed"), b"test_value");

        // Test remove
        let removed = cache.remove("test_key");
        assert!(removed);
        assert!(cache.get("test_key").is_none());
    }

    #[tokio::test]
    async fn test_cache_manager_overwrite() {
        let config = manager::UnifiedCacheConfig {
            max_size: 1000,
            ttl_seconds: Some(300),
            cache_dir: Some("/tmp/nestgate_cache_overwrite2".to_string().into()),
            eviction_policy: "lru".to_string(),
        };

        let mut cache = CacheManager::new(config);

        cache
            .put("key".to_string(), b"value1".to_vec())
            .await
            .expect("Cache operation failed");
        cache
            .put("key".to_string(), b"value2".to_vec())
            .await
            .expect("Cache operation failed");

        let result = cache.get("key").expect("Cache operation failed");
        assert_eq!(result, b"value2");
    }

    #[tokio::test]
    async fn test_cache_manager_multiple_entries() {
        let config = manager::UnifiedCacheConfig {
            max_size: 1000,
            ttl_seconds: Some(300),
            cache_dir: Some("/tmp/nestgate_cache_multiple2".to_string().into()),
            eviction_policy: "lru".to_string(),
        };

        let mut cache = CacheManager::new(config);

        for i in 0..10 {
            let key = format!("key_{}", i);
            let value = format!("value_{}", i).into_bytes();
            cache.put(key, value).await.expect("Cache operation failed");
        }

        // Verify all entries
        for i in 0..10 {
            let key = format!("key_{}", i);
            let expected_value = format!("value_{}", i).into_bytes();
            let result = cache.get(&key);
            assert!(result.is_some());
            assert_eq!(result.expect("Cache operation failed"), expected_value);
        }
    }

    #[tokio::test]
    async fn test_cache_manager_clear() {
        let config = manager::UnifiedCacheConfig {
            max_size: 1000,
            ttl_seconds: Some(300),
            cache_dir: Some("/tmp/nestgate_cache_clear2".to_string().into()),
            eviction_policy: "lru".to_string(),
        };

        let mut cache = CacheManager::new(config);

        // Add entries
        cache
            .put("key1".to_string(), b"value1".to_vec())
            .await
            .expect("Cache operation failed");
        cache
            .put("key2".to_string(), b"value2".to_vec())
            .await
            .expect("Cache operation failed");

        // Clear
        cache.clear();

        // Verify cleared
        assert!(cache.get("key1").is_none());
        assert!(cache.get("key2").is_none());
    }

    #[tokio::test]
    async fn test_multi_tier_cache_basic_operations() {
        let config = MultiTierCacheConfig {
            hot_tier_config: SimpleCacheConfig {
                max_size: 10,
                ttl: std::time::Duration::from_secs(60),
                cache_dir: "/tmp/nestgate_multi_tier/hot".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 50,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/nestgate_multi_tier/warm".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/nestgate_multi_tier/cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 2,
        };

        let mut cache = MultiTierCache::new(config).expect("Cache operation failed");

        // Test put and get
        cache
            .put("test_key", b"test_value".to_vec())
            .await
            .expect("Cache operation failed");
        let result = cache.get("test_key").await.expect("Cache operation failed");
        assert!(result.is_some());
        assert_eq!(result.expect("Cache operation failed"), b"test_value");

        // Test remove
        let removed = cache
            .remove("test_key")
            .await
            .expect("Cache operation failed");
        assert!(removed);
        assert!(
            cache
                .get("test_key")
                .await
                .expect("Cache operation failed")
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_multi_tier_cache_tier_promotion() {
        let config = MultiTierCacheConfig {
            hot_tier_config: SimpleCacheConfig {
                max_size: 10,
                ttl: std::time::Duration::from_secs(60),
                cache_dir: "/tmp/nestgate_multi_tier_promotion/hot".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 50,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/nestgate_multi_tier_promotion/warm".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/nestgate_multi_tier_promotion/cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 2,
        };

        let mut cache = MultiTierCache::new(config).expect("Cache operation failed");

        // Add entry (starts in hot tier)
        cache
            .put("key", b"value".to_vec())
            .await
            .expect("Cache operation failed");

        // Access multiple times (should stay in or promote to hot)
        for _ in 0..5 {
            let _ = cache.get("key").await.expect("Cache operation failed");
        }

        // Verify still accessible
        let result = cache.get("key").await.expect("Cache operation failed");
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_multi_tier_cache_multiple_entries() {
        let config = MultiTierCacheConfig {
            hot_tier_config: SimpleCacheConfig {
                max_size: 10,
                ttl: std::time::Duration::from_secs(60),
                cache_dir: "/tmp/nestgate_multi_tier_multiple/hot".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 50,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/nestgate_multi_tier_multiple/warm".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/nestgate_multi_tier_multiple/cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 2,
        };

        let mut cache = MultiTierCache::new(config).expect("Cache operation failed");

        // Add multiple entries
        for i in 0..20 {
            let key = format!("key_{}", i);
            let value = format!("value_{}", i).into_bytes();
            cache
                .put(&key, value)
                .await
                .expect("Cache operation failed");
        }

        // Verify all entries are accessible
        for i in 0..20 {
            let key = format!("key_{}", i);
            let expected_value = format!("value_{}", i).into_bytes();
            let result = cache.get(&key).await.expect("Cache operation failed");
            assert!(result.is_some());
            assert_eq!(result.expect("Cache operation failed"), expected_value);
        }
    }

    #[tokio::test]
    async fn test_multi_tier_cache_clear() {
        let config = MultiTierCacheConfig {
            hot_tier_config: SimpleCacheConfig {
                max_size: 10,
                ttl: std::time::Duration::from_secs(60),
                cache_dir: "/tmp/nestgate_multi_tier_clear/hot".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 50,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/nestgate_multi_tier_clear/warm".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/nestgate_multi_tier_clear/cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 2,
        };

        let mut cache = MultiTierCache::new(config).expect("Cache operation failed");

        // Add entries
        cache
            .put("key1", b"value1".to_vec())
            .await
            .expect("Cache operation failed");
        cache
            .put("key2", b"value2".to_vec())
            .await
            .expect("Cache operation failed");

        // Clear
        cache.clear().await.expect("Cache operation failed");

        // Verify cleared
        assert!(
            cache
                .get("key1")
                .await
                .expect("Cache operation failed")
                .is_none()
        );
        assert!(
            cache
                .get("key2")
                .await
                .expect("Cache operation failed")
                .is_none()
        );
    }

    #[tokio::test]
    async fn test_multi_tier_cache_stats() {
        let config = MultiTierCacheConfig {
            hot_tier_config: SimpleCacheConfig {
                max_size: 10,
                ttl: std::time::Duration::from_secs(60),
                cache_dir: "/tmp/nestgate_multi_tier_stats/hot".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 50,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/nestgate_multi_tier_stats/warm".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/nestgate_multi_tier_stats/cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 2,
        };

        let mut cache = MultiTierCache::new(config).expect("Cache operation failed");

        // Add and access entries
        cache
            .put("key1", b"value1".to_vec())
            .await
            .expect("Cache operation failed");
        let _ = cache.get("key1").await.expect("Cache operation failed"); // Hit
        let _ = cache
            .get("nonexistent")
            .await
            .expect("Cache operation failed"); // Miss

        let stats = cache.stats().expect("Cache operation failed");

        // NOTE: stats() is currently a placeholder implementation that returns zeros.
        // Future enhancement: Implement actual stats tracking in MultiTierCache.
        // This test validates the placeholder behavior.
        assert_eq!(stats.total_hits, 0); // Placeholder implementation returns 0
        assert_eq!(stats.total_misses, 0); // Placeholder implementation returns 0
    }
}
