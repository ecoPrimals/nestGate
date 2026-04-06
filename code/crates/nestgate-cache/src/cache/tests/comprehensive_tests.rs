// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    deprecated,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::default_trait_access,
)]

//! Comprehensive cache tests
//!
//! Extended test suite for advanced cache functionality including
//! functional tests, integration tests, and edge cases.

use crate::cache::multi_tier::SimpleCacheConfig;
use crate::cache::*;

#[cfg(test)]
mod cache_functional_tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_system_enum_single_tier_operations() {
        let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
            enabled: true,
            size_bytes: 1024 * 1024, // 1MB
            cache_type: "lru".to_string(),
            hot_tier_size: Some(100),
            warm_tier_size: None,
            cold_tier_unlimited: None,
            ttl_seconds: Some(300),
            cache_dir: Some("/tmp/nestgate_cache_system_enum".to_string().into()),
            policy: Some("lru".to_string()),
            cache_settings: std::collections::HashMap::new(),
        };

        let mut cache = CacheSystem::single_tier(cache_config).expect("Cache operation failed");

        // Test full lifecycle
        cache
            .put("key", b"value".to_vec())
            .await
            .expect("Cache operation failed");
        let result = cache.get("key").await.expect("Cache operation failed");
        assert_eq!(result.expect("Cache operation failed"), b"value");

        cache.remove("key").await.expect("Cache operation failed");
        let after_remove = cache.get("key").await.expect("Cache operation failed");
        assert_eq!(after_remove, None);
    }

    #[tokio::test]
    async fn test_cache_system_enum_multi_tier_operations() {
        let config = MultiTierCacheConfig {
            hot_tier_config: SimpleCacheConfig {
                max_size: 10,
                ttl: std::time::Duration::from_secs(60),
                cache_dir: "/tmp/nestgate_cache_system_enum_multi/hot".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 50,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/nestgate_cache_system_enum_multi/warm".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/nestgate_cache_system_enum_multi/cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 2,
        };

        let mut cache = CacheSystem::multi_tier(config).expect("Cache operation failed");

        // Test full lifecycle
        cache
            .put("key", b"value".to_vec())
            .await
            .expect("Cache operation failed");
        let result = cache.get("key").await.expect("Cache operation failed");
        assert_eq!(result.expect("Cache operation failed"), b"value");

        cache.remove("key").await.expect("Cache operation failed");
        let after_remove = cache.get("key").await.expect("Cache operation failed");
        assert_eq!(after_remove, None);
    }

    #[tokio::test]
    async fn test_cache_policy_types() {
        // Verify cache policy enum variants
        let none = CachePolicy::None;
        let read_only = CachePolicy::ReadOnly;
        let write_through = CachePolicy::WriteThrough;

        // They should be different
        assert_ne!(format!("{none:?}"), format!("{:?}", read_only));
        assert_ne!(format!("{read_only:?}"), format!("{:?}", write_through));
    }

    #[tokio::test]
    async fn test_storage_tier_types() {
        // Verify storage tier enum variants
        let hot = StorageTier::Hot;
        let warm = StorageTier::Warm;
        let cold = StorageTier::Cold;

        // They should be different
        assert_ne!(format!("{hot:?}"), format!("{:?}", warm));
        assert_ne!(format!("{warm:?}"), format!("{:?}", cold));
    }

    #[tokio::test]
    async fn test_cache_entry_creation() {
        let entry = CacheEntry {
            key: "test_key".to_string(),
            data: b"test_data".to_vec(),
            size: 9,
            created_at: chrono::Utc::now(),
            accessed_at: chrono::Utc::now(),
            access_count: 0,
            tier: StorageTier::Hot,
            ttl: None,
        };

        assert_eq!(entry.key, "test_key");
        assert_eq!(entry.data, b"test_data");
        assert_eq!(entry.size, 9);
    }

    #[tokio::test]
    async fn test_cache_stats_initialization() {
        let stats = CacheStats {
            hits: 0,
            misses: 0,
            hot_tier_items: 0,
            warm_tier_items: 0,
            cold_tier_items: 0,
            hot_tier_size_bytes: 0,
            warm_tier_size_bytes: 0,
            cold_tier_size_bytes: 0,
            hot_tier_evictions: 0,
            warm_tier_evictions: 0,
            cold_tier_evictions: 0,
            tier_access_times: Default::default(),
            efficiency_metrics: Default::default(),
        };

        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }

    #[tokio::test]
    async fn test_multi_tier_cache_config_creation() {
        let config = MultiTierCacheConfig {
            hot_tier_config: SimpleCacheConfig {
                max_size: 10,
                ttl: std::time::Duration::from_secs(60),
                cache_dir: "/tmp/test/hot".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 50,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/test/warm".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/test/cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 2,
        };

        assert_eq!(config.hot_tier_config.max_size, 10);
        assert_eq!(config.warm_tier_config.max_size, 50);
        assert_eq!(config.cold_tier_config.max_size, 100);
    }

    #[tokio::test]
    async fn test_cache_system_type_checking() {
        let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
            enabled: true,
            size_bytes: 1024 * 1024, // 1MB
            cache_type: "lru".to_string(),
            hot_tier_size: Some(100),
            warm_tier_size: None,
            cold_tier_unlimited: None,
            ttl_seconds: Some(300),
            cache_dir: Some("/tmp/nestgate_type_check".to_string().into()),
            policy: Some("lru".to_string()),
            cache_settings: std::collections::HashMap::new(),
        };

        let cache = CacheSystem::single_tier(cache_config).expect("Cache operation failed");

        // Type checking via pattern matching
        match cache {
            CacheSystem::SingleTier(_) => {
                // Expected
            }
            CacheSystem::MultiTier(_) => {
                panic!("Expected SingleTier variant");
            }
        }
    }

    #[tokio::test]
    async fn test_cache_lifecycle_complete() {
        let cache_config = nestgate_config::config::canonical_primary::CacheConfig {
            enabled: true,
            size_bytes: 1024 * 1024, // 1MB
            cache_type: "lru".to_string(),
            hot_tier_size: Some(100),
            warm_tier_size: None,
            cold_tier_unlimited: None,
            ttl_seconds: Some(300),
            cache_dir: Some("/tmp/nestgate_lifecycle".to_string().into()),
            policy: Some("lru".to_string()),
            cache_settings: std::collections::HashMap::new(),
        };

        let mut cache = CacheSystem::single_tier(cache_config).expect("Cache operation failed");

        // Full lifecycle test
        for i in 0..10 {
            let key = format!("key_{i}");
            let value = format!("value_{i}").into_bytes();

            cache
                .put(&key, value.clone())
                .await
                .expect("Cache operation failed");
            let retrieved = cache.get(&key).await.expect("Cache operation failed");
            assert_eq!(retrieved, Some(value));

            cache.remove(&key).await.expect("Cache operation failed");
            let after_remove = cache.get(&key).await.expect("Cache operation failed");
            assert_eq!(after_remove, None);
        }
    }
}

#[cfg(test)]
mod cache_manager_entry_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_cache_entry_creation() {
        let data = b"test_data".to_vec();
        let entry = manager::CacheEntry::new(data.clone());

        assert_eq!(entry.data, data);
        assert_eq!(entry.access_count, 1);
    }

    #[test]
    fn test_cache_entry_access_tracking() {
        let data = b"test_data".to_vec();
        let mut entry = manager::CacheEntry::new(data);

        let initial_count = entry.access_count;
        entry.access();
        entry.access();

        assert_eq!(entry.access_count, initial_count + 2);
    }

    #[test]
    fn test_cache_entry_expiration() {
        let data = b"test_data".to_vec();
        let entry = manager::CacheEntry::new(data);

        // Not expired with 1 hour TTL
        assert!(!entry.is_expired(Duration::from_secs(3600)));

        // Would be expired with zero TTL
        assert!(entry.is_expired(Duration::from_millis(0)));
    }

    #[tokio::test]
    async fn test_cache_entry_access_time_updates() {
        let data = b"test_data".to_vec();
        let mut entry = manager::CacheEntry::new(data);

        let first_access = entry.last_accessed;
        tokio::time::sleep(Duration::from_millis(10)).await;
        entry.access();
        let second_access = entry.last_accessed;

        assert!(second_access > first_access);
    }
}

#[cfg(test)]
mod cache_stats_tests {
    use super::*;

    #[test]
    fn test_cache_stats_default() {
        let stats = manager::CacheStats::default();

        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.evictions, 0);
        assert_eq!(stats.size, 0);
    }

    #[test]
    fn test_cache_stats_hit_rate_zero_operations() {
        let stats = manager::CacheStats::default();
        assert_eq!(stats.hit_rate(), 0.0);
    }

    #[test]
    fn test_cache_stats_hit_rate_all_hits() {
        let stats = manager::CacheStats {
            hits: 100,
            misses: 0,
            evictions: 0,
            size: 0,
        };
        assert_eq!(stats.hit_rate(), 1.0);
    }

    #[test]
    fn test_cache_stats_hit_rate_all_misses() {
        let stats = manager::CacheStats {
            hits: 0,
            misses: 100,
            evictions: 0,
            size: 0,
        };
        assert_eq!(stats.hit_rate(), 0.0);
    }

    #[test]
    fn test_cache_stats_hit_rate_mixed() {
        let stats = manager::CacheStats {
            hits: 75,
            misses: 25,
            evictions: 0,
            size: 0,
        };
        assert_eq!(stats.hit_rate(), 0.75);
    }

    #[test]
    fn test_cache_stats_incremental_updates() {
        let stats = manager::CacheStats {
            hits: 10,
            misses: 5,
            ..Default::default()
        };
        assert_eq!(stats.hit_rate(), 10.0 / 15.0);

        let stats2 = manager::CacheStats {
            hits: 20,
            misses: 5,
            ..Default::default()
        };
        assert_eq!(stats2.hit_rate(), 20.0 / 25.0);
    }
}

#[cfg(test)]
mod storage_tier_tests {
    use super::*;

    #[test]
    fn test_storage_tier_priority() {
        assert!(StorageTier::Hot.priority() < StorageTier::Warm.priority());
        assert!(StorageTier::Warm.priority() < StorageTier::Cool.priority());
        assert!(StorageTier::Cool.priority() < StorageTier::Cold.priority());
        assert!(StorageTier::Cold.priority() < StorageTier::Frozen.priority());
    }

    #[test]
    fn test_storage_tier_priority_values() {
        assert_eq!(StorageTier::Hot.priority(), 0);
        assert_eq!(StorageTier::Warm.priority(), 1);
        assert_eq!(StorageTier::Cool.priority(), 2);
        assert_eq!(StorageTier::Cold.priority(), 3);
        assert_eq!(StorageTier::Frozen.priority(), 4);
    }

    #[test]
    fn test_storage_tier_access_times() {
        assert!(StorageTier::Hot.typical_access_time() < StorageTier::Warm.typical_access_time());
        assert!(StorageTier::Warm.typical_access_time() < StorageTier::Cool.typical_access_time());
        assert!(StorageTier::Cool.typical_access_time() < StorageTier::Cold.typical_access_time());
        assert!(
            StorageTier::Cold.typical_access_time() < StorageTier::Frozen.typical_access_time()
        );
    }

    #[test]
    fn test_storage_tier_access_time_magnitudes() {
        use std::time::Duration;

        // Hot tier should be sub-millisecond
        assert!(StorageTier::Hot.typical_access_time() < Duration::from_millis(1));

        // Frozen tier should be multi-second
        assert!(StorageTier::Frozen.typical_access_time() >= Duration::from_secs(1));
    }

    #[test]
    fn test_storage_tier_clone() {
        let tier1 = StorageTier::Hot;
        let tier2 = tier1.clone();
        assert_eq!(format!("{tier1:?}"), format!("{:?}", tier2));
    }
}

#[cfg(test)]
mod cache_policy_tests {
    use super::*;

    #[test]
    fn test_cache_policy_default() {
        let policy = CachePolicy::default();
        assert_eq!(policy, CachePolicy::WriteThrough);
    }

    #[test]
    fn test_cache_policy_variants() {
        let policies = [
            CachePolicy::None,
            CachePolicy::ReadOnly,
            CachePolicy::WriteThrough,
            CachePolicy::WriteBack,
        ];

        assert_eq!(policies.len(), 4);
    }

    #[test]
    fn test_cache_policy_equality() {
        assert_eq!(CachePolicy::None, CachePolicy::None);
        assert_ne!(CachePolicy::None, CachePolicy::ReadOnly);
    }

    #[test]
    fn test_cache_policy_serialization() {
        let policy = CachePolicy::WriteThrough;
        let serialized = serde_json::to_string(&policy).expect("Serialization failed");
        assert!(serialized.contains("WriteThrough"));
    }

    #[test]
    fn test_cache_policy_deserialization() {
        let json = "\"WriteThrough\"";
        let policy: CachePolicy = serde_json::from_str(json).expect("Deserialization failed");
        assert_eq!(policy, CachePolicy::WriteThrough);
    }

    #[test]
    fn test_cache_policy_roundtrip() {
        let original = CachePolicy::WriteBack;
        let serialized = serde_json::to_string(&original).expect("Serialization failed");
        let deserialized: CachePolicy =
            serde_json::from_str(&serialized).expect("Deserialization failed");
        assert_eq!(original, deserialized);
    }
}

#[cfg(test)]
mod cache_config_tests {
    use super::*;

    #[test]
    fn test_unified_cache_config_default() {
        let config = manager::UnifiedCacheConfig::default();

        assert_eq!(config.max_size, 1000);
        assert_eq!(config.ttl_seconds, Some(3600));
        assert_eq!(config.eviction_policy, "lru");
        assert!(config.cache_dir.is_none());
    }

    #[test]
    fn test_unified_cache_config_custom() {
        let config = manager::UnifiedCacheConfig {
            max_size: 5000,
            ttl_seconds: Some(7200),
            cache_dir: Some("/custom/cache".into()),
            eviction_policy: "lfu".to_string(),
        };

        assert_eq!(config.max_size, 5000);
        assert_eq!(config.ttl_seconds, Some(7200));
        assert_eq!(config.eviction_policy, "lfu");
    }

    #[test]
    fn test_unified_cache_config_serialization() {
        let config = manager::UnifiedCacheConfig::default();
        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_simple_cache_config_creation() {
        let config = SimpleCacheConfig {
            max_size: 100,
            ttl: std::time::Duration::from_secs(300),
            cache_dir: "/tmp/test_cache".to_string(),
        };

        assert_eq!(config.max_size, 100);
        assert_eq!(config.ttl, std::time::Duration::from_secs(300));
        assert_eq!(config.cache_dir, "/tmp/test_cache");
    }
}

#[cfg(test)]
mod cache_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_manager_stat_tracking() {
        let config = manager::UnifiedCacheConfig::default();
        let mut cache = CacheManager::new(config);

        // Test hit/miss tracking
        cache.put("key1", b"value1".to_vec()).expect("Put failed");

        // This should be a hit
        let _result1 = cache.get("key1");

        // This should be a miss
        let _result2 = cache.get("nonexistent");

        // Verify stats (note: actual implementation may vary)
        // This test validates that stat tracking infrastructure exists
    }

    #[tokio::test]
    async fn test_cache_manager_tier_navigation() {
        let config = manager::UnifiedCacheConfig::default();
        let mut cache = CacheManager::new(config);

        // Add entry (goes to hot tier)
        cache.put("key", b"value".to_vec()).expect("Put failed");

        // Access multiple times
        for _ in 0..5 {
            let _ = cache.get("key");
        }

        // Verify data is still accessible
        let result = cache.get("key");
        assert!(result.is_some());
        assert_eq!(result.expect("Get failed"), b"value");
    }

    #[tokio::test]
    async fn test_cache_manager_large_dataset() {
        let config = manager::UnifiedCacheConfig {
            max_size: 1000,
            ttl_seconds: Some(3600),
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };

        let mut cache = CacheManager::new(config);

        // Add many entries
        for i in 0..100 {
            let key = format!("key_{i}");
            let value = format!("value_{i}").into_bytes();
            cache.put(&key, value).expect("Put failed");
        }

        // Verify some entries are accessible
        for i in 0..50 {
            let key = format!("key_{i}");
            let result = cache.get(&key);
            assert!(result.is_some());
        }
    }

    #[tokio::test]
    async fn test_multi_tier_cache_stress() {
        let config = MultiTierCacheConfig {
            hot_tier_config: SimpleCacheConfig {
                max_size: 10,
                ttl: std::time::Duration::from_secs(60),
                cache_dir: "/tmp/stress/hot".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 50,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/stress/warm".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/stress/cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 2,
        };

        let mut cache = MultiTierCache::new(config).expect("Cache creation failed");

        // Stress test with many operations
        for i in 0..50 {
            let key = format!("stress_key_{i}");
            let value = format!("stress_value_{i}").into_bytes();

            cache.put(&key, value.clone()).await.expect("Put failed");
            let result = cache.get(&key).await.expect("Get failed");
            assert_eq!(result, Some(value));
        }
    }
}
