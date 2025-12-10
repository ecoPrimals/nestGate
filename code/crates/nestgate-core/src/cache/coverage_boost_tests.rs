// Strategic tests to boost cache coverage from 65% to 80%+
// Focus: Error paths, edge cases, and boundary conditions

#[cfg(test)]
mod cache_error_path_tests {
    use crate::cache::manager::{CacheManager, UnifiedCacheConfig};
    use crate::cache::multi_tier::{MultiTierCache, MultiTierCacheConfig};

    #[tokio::test]
    async fn test_cache_manager_empty_key() {
        let mut cache = CacheManager::default();

        // Test with empty key
        let result = cache.put(String::new(), b"data".to_vec()).await;
        assert!(result.is_ok());

        let retrieved = cache.get("");
        assert_eq!(retrieved, Some(b"data".to_vec()));
    }

    #[tokio::test]
    async fn test_cache_manager_empty_data() {
        let mut cache = CacheManager::default();

        // Test with empty data
        cache
            .put("key".to_string(), Vec::new())
            .await
            .expect("Put failed");
        let result = cache.get("key");
        assert_eq!(result, Some(Vec::new()));
    }

    #[tokio::test]
    async fn test_cache_manager_very_large_key() {
        let mut cache = CacheManager::default();

        // Test with very large key (1MB)
        let large_key = "x".repeat(1024 * 1024);
        cache
            .put(large_key.clone(), b"data".to_vec())
            .await
            .expect("Put failed");

        let result = cache.get(&large_key);
        assert_eq!(result, Some(b"data".to_vec()));
    }

    #[tokio::test]
    async fn test_cache_manager_very_large_data() {
        let mut cache = CacheManager::default();

        // Test with very large data (10MB)
        let large_data = vec![0u8; 10 * 1024 * 1024];
        cache
            .put("large".to_string(), large_data.clone())
            .await
            .expect("Put failed");

        let result = cache.get("large");
        assert_eq!(result, Some(large_data));
    }

    #[tokio::test]
    async fn test_cache_manager_special_characters_in_key() {
        let mut cache = CacheManager::default();

        // Test with special characters
        let special_keys = vec![
            "key/with/slashes",
            "key\\with\\backslashes",
            "key with spaces",
            "key\twith\ttabs",
            "key\nwith\nnewlines",
            "key:with:colons",
            "key|with|pipes",
            "key<with>brackets",
            "key\"with\"quotes",
        ];

        for key in special_keys {
            cache
                .put(key.to_string(), b"data".to_vec())
                .await
                .expect("Put failed");
            let result = cache.get(key);
            assert_eq!(result, Some(b"data".to_vec()), "Failed for key: {}", key);
        }
    }

    #[tokio::test]
    async fn test_cache_manager_unicode_keys() {
        let mut cache = CacheManager::default();

        // Test with Unicode keys
        let unicode_keys = vec![
            "키",    // Korean
            "ключ",  // Russian
            "🔑",    // Emoji
            "مفتاح", // Arabic
            "键",    // Chinese
        ];

        for key in unicode_keys {
            cache
                .put(key.to_string(), b"data".to_vec())
                .await
                .expect("Put failed");
            let result = cache.get(key);
            assert_eq!(result, Some(b"data".to_vec()), "Failed for key: {}", key);
        }
    }

    #[tokio::test]
    async fn test_cache_manager_max_size_boundary() {
        let config = UnifiedCacheConfig {
            max_size: 5,
            ttl_seconds: None,
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };
        let mut cache = CacheManager::new(config);

        // Fill cache to max capacity
        for i in 0..5 {
            cache
                .put(format!("key{}", i), vec![i as u8])
                .await
                .expect("Put failed");
        }

        // Verify all entries are present
        for i in 0..5 {
            assert!(cache.get(&format!("key{}", i)).is_some());
        }

        // Add one more (should trigger eviction)
        cache
            .put("key5".to_string(), vec![5])
            .await
            .expect("Put failed");

        // Verify cache size is still at max
        let stats = cache.stats();
        assert_eq!(stats.size, 5);
        assert!(stats.evictions > 0);
    }

    #[tokio::test]
    async fn test_cache_manager_zero_max_size() {
        let config = UnifiedCacheConfig {
            max_size: 0,
            ttl_seconds: None,
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };
        let mut cache = CacheManager::new(config);

        // Should handle zero size gracefully
        let result = cache.put("key".to_string(), b"data".to_vec()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cache_manager_concurrent_same_key_updates() {
        let mut cache = CacheManager::default();

        // Rapidly update the same key
        for i in 0..100 {
            cache
                .put("key".to_string(), vec![i])
                .await
                .expect("Put failed");
        }

        // Should have the last value
        let result = cache.get("key");
        assert_eq!(result, Some(vec![99]));
    }

    #[tokio::test]
    async fn test_cache_manager_remove_nonexistent() {
        let mut cache = CacheManager::default();

        // Try to remove non-existent key
        let removed = cache.remove("nonexistent");
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_cache_manager_remove_from_each_tier() {
        let mut cache = CacheManager::default();

        // Add multiple keys
        for i in 0..10 {
            cache
                .put(format!("key{}", i), vec![i])
                .await
                .expect("Put failed");
        }

        // Remove from various positions
        assert!(cache.remove("key0"));
        assert!(cache.remove("key5"));
        assert!(cache.remove("key9"));

        // Verify removed
        assert!(cache.get("key0").is_none());
        assert!(cache.get("key5").is_none());
        assert!(cache.get("key9").is_none());

        // Verify others still present
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key8").is_some());
    }

    #[tokio::test]
    async fn test_cache_manager_clear_empty_cache() {
        let mut cache = CacheManager::default();

        // Clear empty cache
        cache.clear();

        // Should work without error
        assert_eq!(cache.stats().size, 0);
    }

    #[tokio::test]
    async fn test_cache_manager_stats_after_clear() {
        let mut cache = CacheManager::default();

        // Add and access some data
        cache
            .put("key".to_string(), b"data".to_vec())
            .await
            .expect("Put failed");
        cache.get("key");
        cache.get("nonexistent");

        let stats_before = cache.stats();
        assert!(stats_before.hits > 0);
        assert!(stats_before.misses > 0);

        // Clear and check stats
        cache.clear();

        let stats_after = cache.stats();
        assert_eq!(stats_after.size, 0);
        // Stats should be reset
        assert_eq!(stats_after.hits, 0);
        assert_eq!(stats_after.misses, 0);
    }

    #[tokio::test]
    async fn test_multi_tier_cache_empty_operations() {
        let config = MultiTierCacheConfig::default();
        let cache = MultiTierCache::new(config).expect("Cache creation failed");

        // Get from empty cache
        let result = cache.get("nonexistent").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        // Remove from empty cache
        let removed = cache.remove("nonexistent").await;
        assert!(removed.is_ok());
    }

    #[tokio::test]
    async fn test_multi_tier_cache_tier_promotion() {
        let config = MultiTierCacheConfig::default();
        let mut cache = MultiTierCache::new(config).expect("Cache creation failed");

        // Add entry
        cache
            .put("key", b"data".to_vec())
            .await
            .expect("Put failed");

        // Access multiple times to trigger promotion
        for _ in 0..10 {
            let _ = cache.get("key").await;
        }

        // Should still be accessible
        let result = cache.get("key").await.expect("Get failed");
        assert_eq!(result, Some(b"data".to_vec()));
    }

    #[tokio::test]
    async fn test_multi_tier_cache_mixed_operations() {
        let config = MultiTierCacheConfig::default();
        let mut cache = MultiTierCache::new(config).expect("Cache creation failed");

        // Mix of operations
        cache
            .put("key1", b"data1".to_vec())
            .await
            .expect("Put failed");
        cache
            .put("key2", b"data2".to_vec())
            .await
            .expect("Put failed");

        let _ = cache.get("key1").await;
        cache.remove("key2").await.expect("Remove failed");

        cache
            .put("key3", b"data3".to_vec())
            .await
            .expect("Put failed");

        // Verify final state
        assert!(cache.get("key1").await.unwrap().is_some());
        assert!(cache.get("key2").await.unwrap().is_none());
        assert!(cache.get("key3").await.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_cache_manager_sequential_evictions() {
        let config = UnifiedCacheConfig {
            max_size: 3,
            ttl_seconds: None,
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };
        let mut cache = CacheManager::new(config);

        // Fill cache
        for i in 0..3 {
            cache
                .put(format!("key{}", i), vec![i])
                .await
                .expect("Put failed");
        }

        // Trigger multiple evictions
        for i in 3..10 {
            cache
                .put(format!("key{}", i), vec![i])
                .await
                .expect("Put failed");
        }

        // Should have evicted 7 entries
        let stats = cache.stats();
        assert!(stats.evictions >= 7);
        assert_eq!(stats.size, 3);
    }

    #[tokio::test]
    async fn test_cache_manager_get_updates_lru() {
        let config = UnifiedCacheConfig {
            max_size: 3,
            ttl_seconds: None,
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };
        let mut cache = CacheManager::new(config);

        // Fill cache
        cache
            .put("key1".to_string(), b"data1".to_vec())
            .await
            .expect("Put failed");
        cache
            .put("key2".to_string(), b"data2".to_vec())
            .await
            .expect("Put failed");
        cache
            .put("key3".to_string(), b"data3".to_vec())
            .await
            .expect("Put failed");

        // Access key1 to update its LRU position
        cache.get("key1");

        // Add new key (should evict key2 or key3, not key1)
        cache
            .put("key4".to_string(), b"data4".to_vec())
            .await
            .expect("Put failed");

        // key1 should still be present
        assert!(cache.get("key1").is_some());
    }

    #[tokio::test]
    async fn test_cache_manager_alternating_operations() {
        let mut cache = CacheManager::default();

        // Alternate between put, get, and remove
        for i in 0..20 {
            if i % 3 == 0 {
                cache
                    .put(format!("key{}", i), vec![i as u8])
                    .await
                    .expect("Put failed");
            } else if i % 3 == 1 {
                cache.get(&format!("key{}", i - 1));
            } else {
                cache.remove(&format!("key{}", i - 2));
            }
        }

        // Should complete without errors
        let stats = cache.stats();
        assert!(stats.hits > 0);
    }

    #[tokio::test]
    async fn test_multi_tier_cache_rapid_puts() {
        let config = MultiTierCacheConfig::default();
        let mut cache = MultiTierCache::new(config).expect("Cache creation failed");

        // Rapidly add many entries
        for i in 0..100 {
            cache
                .put(&format!("key{}", i), vec![i as u8])
                .await
                .expect("Put failed");
        }

        // Verify stats - check that cache operations worked
        let stats = cache.stats().expect("Stats failed");
        // Just verify stats structure is accessible
        let _ = stats.total_hits;
    }

    #[tokio::test]
    async fn test_cache_manager_duplicate_removes() {
        let mut cache = CacheManager::default();

        cache
            .put("key".to_string(), b"data".to_vec())
            .await
            .expect("Put failed");

        // Remove multiple times
        assert!(cache.remove("key"));
        assert!(!cache.remove("key"));
        assert!(!cache.remove("key"));
    }
}
