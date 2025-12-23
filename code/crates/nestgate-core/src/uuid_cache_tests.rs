//! Comprehensive tests for UUID cache module
//! Added: November 14, 2025 - Coverage Sprint

#[cfg(test)]
mod uuid_cache_tests {
    use crate::uuid_cache::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_uuid_cache_creation() {
        let cache = UuidCache::new();
        assert!(cache.cache.lock().unwrap().is_empty());
    }

    #[test]
    fn test_get_or_create_uuid_first_time() {
        let cache = UuidCache::new();
        let key = "test_key";
        
        let uuid1 = cache.get_or_create(key);
        let uuid2 = cache.get_or_create(key);
        
        // Should return the same UUID for the same key
        assert_eq!(uuid1, uuid2);
    }

    #[test]
    fn test_get_or_create_uuid_different_keys() {
        let cache = UuidCache::new();
        
        let uuid1 = cache.get_or_create("key1");
        let uuid2 = cache.get_or_create("key2");
        
        // Different keys should have different UUIDs
        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_uuid_cache_persistence() {
        let cache = UuidCache::new();
        let key = "persistent_key";
        
        let uuid = cache.get_or_create(key);
        
        // Call multiple times, should always return the same UUID
        for _ in 0..100 {
            assert_eq!(cache.get_or_create(key), uuid);
        }
    }

    #[test]
    fn test_uuid_cache_concurrent_access() {
        let cache = Arc::new(UuidCache::new());
        let key = "concurrent_key";
        let mut handles = vec![];
        
        // Spawn multiple threads accessing the same key
        for _ in 0..10 {
            let cache_clone = Arc::clone(&cache);
            let handle = thread::spawn(move || {
                cache_clone.get_or_create(key)
            });
            handles.push(handle);
        }
        
        // Collect all UUIDs
        let mut uuids = vec![];
        for handle in handles {
            uuids.push(handle.join().unwrap());
        }
        
        // All should be the same
        let first_uuid = uuids[0];
        for uuid in uuids {
            assert_eq!(uuid, first_uuid);
        }
    }

    #[test]
    fn test_uuid_format_validity() {
        let cache = UuidCache::new();
        let uuid = cache.get_or_create("test");
        
        // UUID should be in the correct format
        assert_eq!(uuid.len(), 36); // Standard UUID length with hyphens
        assert_eq!(uuid.chars().filter(|&c| c == '-').count(), 4); // 4 hyphens
    }

    #[test]
    fn test_clear_cache() {
        let cache = UuidCache::new();
        
        cache.get_or_create("key1");
        cache.get_or_create("key2");
        cache.get_or_create("key3");
        
        cache.clear();
        
        assert!(cache.cache.lock().unwrap().is_empty());
    }

    #[test]
    fn test_cache_size() {
        let cache = UuidCache::new();
        
        assert_eq!(cache.len(), 0);
        
        cache.get_or_create("key1");
        assert_eq!(cache.len(), 1);
        
        cache.get_or_create("key2");
        assert_eq!(cache.len(), 2);
        
        cache.get_or_create("key1"); // Accessing existing key
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_contains_key() {
        let cache = UuidCache::new();
        
        assert!(!cache.contains("nonexistent"));
        
        cache.get_or_create("exists");
        assert!(cache.contains("exists"));
    }

    #[test]
    fn test_remove_key() {
        let cache = UuidCache::new();
        
        let uuid = cache.get_or_create("to_remove");
        assert!(cache.contains("to_remove"));
        
        let removed = cache.remove("to_remove");
        assert_eq!(removed, Some(uuid));
        assert!(!cache.contains("to_remove"));
        
        // Removing non-existent key
        assert_eq!(cache.remove("never_existed"), None);
    }

    #[test]
    fn test_get_without_create() {
        let cache = UuidCache::new();
        
        // Get before creating
        assert_eq!(cache.get("nonexistent"), None);
        
        // Create then get
        let uuid = cache.get_or_create("exists");
        assert_eq!(cache.get("exists"), Some(uuid));
    }

    #[test]
    fn test_cache_with_empty_string_key() {
        let cache = UuidCache::new();
        
        let uuid1 = cache.get_or_create("");
        let uuid2 = cache.get_or_create("");
        
        assert_eq!(uuid1, uuid2);
    }

    #[test]
    fn test_cache_with_special_characters() {
        let cache = UuidCache::new();
        
        let keys = vec!["key@123", "key#456", "key$789", "key%abc"];
        let mut uuids = std::collections::HashMap::new();
        
        for key in &keys {
            let uuid = cache.get_or_create(key);
            uuids.insert(key, uuid);
        }
        
        // Verify all keys still map to correct UUIDs
        for key in &keys {
            assert_eq!(cache.get_or_create(key), *uuids.get(key).unwrap());
        }
    }

    #[test]
    fn test_cache_with_long_keys() {
        let cache = UuidCache::new();
        
        let long_key = "a".repeat(1000);
        let uuid1 = cache.get_or_create(&long_key);
        let uuid2 = cache.get_or_create(&long_key);
        
        assert_eq!(uuid1, uuid2);
    }

    #[test]
    fn test_cache_statistics() {
        let cache = UuidCache::new();
        
        // Create some entries
        for i in 0..10 {
            cache.get_or_create(&format!("key_{}", i));
        }
        
        let stats = cache.stats();
        assert_eq!(stats.total_entries, 10);
    }
}

