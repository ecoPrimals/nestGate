// Strategic tests to boost cache/mod.rs coverage from 30.77% to 75%+
// Focus: CacheSystem public API, factory functions, and operations

#[cfg(test)]
mod cache_system_api_tests {
    use crate::cache::{CacheSystem, MultiTierCacheConfig};
    use crate::config::canonical_primary::CacheConfig;

    // ==================== FACTORY FUNCTION TESTS ====================

    #[tokio::test]
    async fn test_cache_system_single_tier_creation() {
        let config = CacheConfig::default();
        let system = CacheSystem::single_tier(config);
        assert!(system.is_ok());
    }

    #[tokio::test]
    async fn test_cache_system_single_tier_with_custom_config() {
        let config = CacheConfig {
            hot_tier_size: Some(500),
            ttl_seconds: Some(1800),
            cache_dir: Some("/tmp/test_cache".into()),
            policy: Some("lru".to_string()),
            ..Default::default()
        };
        let system = CacheSystem::single_tier(config);
        assert!(system.is_ok());
    }

    #[tokio::test]
    async fn test_cache_system_single_tier_minimal_config() {
        let config = CacheConfig {
            hot_tier_size: Some(10),
            ttl_seconds: None,
            cache_dir: None,
            policy: None,
            ..Default::default()
        };
        let system = CacheSystem::single_tier(config);
        assert!(system.is_ok());
    }

    #[tokio::test]
    async fn test_cache_system_multi_tier_creation() {
        let config = MultiTierCacheConfig::default();
        let system = CacheSystem::multi_tier(config);
        assert!(system.is_ok());
    }

    // ==================== SINGLE TIER OPERATIONS ====================

    #[tokio::test]
    async fn test_single_tier_put_and_get() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Put data
        let result = system.put("test_key", b"test_value".to_vec()).await;
        assert!(result.is_ok());

        // Get data
        let retrieved = system.get("test_key").await.expect("Failed to get");
        assert_eq!(retrieved, Some(b"test_value".to_vec()));
    }

    #[tokio::test]
    async fn test_single_tier_get_nonexistent() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        let result = system.get("nonexistent").await.expect("Failed to get");
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_single_tier_remove() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Put and remove
        system
            .put("key", b"value".to_vec())
            .await
            .expect("Put failed");
        let removed = system.remove("key").await.expect("Remove failed");
        assert!(removed);

        // Verify removed
        let result = system.get("key").await.expect("Get failed");
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_single_tier_remove_nonexistent() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        let removed = system.remove("nonexistent").await.expect("Remove failed");
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_single_tier_clear() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Add multiple items
        system
            .put("key1", b"value1".to_vec())
            .await
            .expect("Put failed");
        system
            .put("key2", b"value2".to_vec())
            .await
            .expect("Put failed");
        system
            .put("key3", b"value3".to_vec())
            .await
            .expect("Put failed");

        // Clear
        system.clear().await.expect("Clear failed");

        // Verify cleared
        assert_eq!(system.get("key1").await.expect("Get failed"), None);
        assert_eq!(system.get("key2").await.expect("Get failed"), None);
        assert_eq!(system.get("key3").await.expect("Get failed"), None);
    }

    #[tokio::test]
    async fn test_single_tier_contains_key() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Initially doesn't contain
        assert!(!system.contains_key("key").await);

        // After put, contains
        system
            .put("key", b"value".to_vec())
            .await
            .expect("Put failed");
        assert!(system.contains_key("key").await);

        // After remove, doesn't contain
        system.remove("key").await.expect("Remove failed");
        assert!(!system.contains_key("key").await);
    }

    #[tokio::test]
    async fn test_single_tier_multiple_operations() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Series of operations
        system
            .put("key1", b"value1".to_vec())
            .await
            .expect("Put failed");
        system
            .put("key2", b"value2".to_vec())
            .await
            .expect("Put failed");

        assert!(system.contains_key("key1").await);
        assert_eq!(
            system.get("key1").await.expect("Get failed"),
            Some(b"value1".to_vec())
        );

        system.remove("key1").await.expect("Remove failed");
        assert!(!system.contains_key("key1").await);

        system
            .put("key3", b"value3".to_vec())
            .await
            .expect("Put failed");
        assert!(system.contains_key("key3").await);
    }

    // ==================== MULTI TIER OPERATIONS ====================

    #[tokio::test]
    async fn test_multi_tier_put_and_get() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        // Put data
        let result = system.put("test_key", b"test_value".to_vec()).await;
        assert!(result.is_ok());

        // Get data
        let retrieved = system.get("test_key").await.expect("Failed to get");
        assert_eq!(retrieved, Some(b"test_value".to_vec()));
    }

    #[tokio::test]
    async fn test_multi_tier_get_nonexistent() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        let result = system.get("nonexistent").await.expect("Failed to get");
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_multi_tier_remove() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        // Put and remove
        system
            .put("key", b"value".to_vec())
            .await
            .expect("Put failed");
        let removed = system.remove("key").await.expect("Remove failed");
        assert!(removed);

        // Verify removed
        let result = system.get("key").await.expect("Get failed");
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_multi_tier_clear() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        // Add multiple items
        system
            .put("key1", b"value1".to_vec())
            .await
            .expect("Put failed");
        system
            .put("key2", b"value2".to_vec())
            .await
            .expect("Put failed");

        // Clear
        system.clear().await.expect("Clear failed");

        // Verify cleared
        assert_eq!(system.get("key1").await.expect("Get failed"), None);
        assert_eq!(system.get("key2").await.expect("Get failed"), None);
    }

    #[tokio::test]
    async fn test_multi_tier_contains_key() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        // Initially doesn't contain
        assert!(!system.contains_key("key").await);

        // After put, contains
        system
            .put("key", b"value".to_vec())
            .await
            .expect("Put failed");
        assert!(system.contains_key("key").await);
    }

    // ==================== EDGE CASES ====================

    #[tokio::test]
    async fn test_single_tier_empty_key() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        system.put("", b"value".to_vec()).await.expect("Put failed");
        let result = system.get("").await.expect("Get failed");
        assert_eq!(result, Some(b"value".to_vec()));
    }

    #[tokio::test]
    async fn test_single_tier_empty_value() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        system.put("key", Vec::new()).await.expect("Put failed");
        let result = system.get("key").await.expect("Get failed");
        assert_eq!(result, Some(Vec::new()));
    }

    #[tokio::test]
    async fn test_single_tier_overwrite() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Put initial value
        system
            .put("key", b"value1".to_vec())
            .await
            .expect("Put failed");

        // Overwrite with new value
        system
            .put("key", b"value2".to_vec())
            .await
            .expect("Put failed");

        // Should have new value
        let result = system.get("key").await.expect("Get failed");
        assert_eq!(result, Some(b"value2".to_vec()));
    }

    #[tokio::test]
    async fn test_multi_tier_empty_key() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        system.put("", b"value".to_vec()).await.expect("Put failed");
        let result = system.get("").await.expect("Get failed");
        assert_eq!(result, Some(b"value".to_vec()));
    }

    #[tokio::test]
    async fn test_multi_tier_overwrite() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        // Put initial value
        system
            .put("key", b"value1".to_vec())
            .await
            .expect("Put failed");

        // Overwrite
        system
            .put("key", b"value2".to_vec())
            .await
            .expect("Put failed");

        // Should have new value
        let result = system.get("key").await.expect("Get failed");
        assert_eq!(result, Some(b"value2".to_vec()));
    }

    #[tokio::test]
    async fn test_single_tier_many_keys() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Add many keys
        for i in 0..50 {
            system
                .put(&format!("key{}", i), vec![i as u8])
                .await
                .expect("Put failed");
        }

        // Verify some are still accessible
        assert!(system.contains_key("key0").await);
        assert!(system.contains_key("key49").await);
    }

    #[tokio::test]
    async fn test_multi_tier_many_keys() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        // Add many keys
        for i in 0..50 {
            system
                .put(&format!("key{}", i), vec![i as u8])
                .await
                .expect("Put failed");
        }

        // Verify some are accessible
        assert!(system.contains_key("key0").await);
        assert!(system.contains_key("key49").await);
    }

    #[tokio::test]
    async fn test_single_tier_clear_empty() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Clear empty cache
        let result = system.clear().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multi_tier_clear_empty() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        // Clear empty cache
        let result = system.clear().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_single_tier_repeated_operations() {
        let config = CacheConfig::default();
        let mut system = CacheSystem::single_tier(config).expect("Failed to create cache");

        // Repeated put/get on same key
        for i in 0..10 {
            system.put("key", vec![i]).await.expect("Put failed");
            let result = system.get("key").await.expect("Get failed");
            assert_eq!(result, Some(vec![i]));
        }
    }

    #[tokio::test]
    async fn test_multi_tier_repeated_operations() {
        let config = MultiTierCacheConfig::default();
        let mut system = CacheSystem::multi_tier(config).expect("Failed to create cache");

        // Repeated put/get on same key
        for i in 0..10 {
            system.put("key", vec![i]).await.expect("Put failed");
            let result = system.get("key").await.expect("Get failed");
            assert_eq!(result, Some(vec![i]));
        }
    }
}
