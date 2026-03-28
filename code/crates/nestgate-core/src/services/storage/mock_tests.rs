// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Mock-based storage tests (no ZFS required)
//! Added: November 21, 2025 - Storage Testing Sprint
//!
//! Target: Complete coverage of storage service logic without system dependencies

#[cfg(test)]
mod storage_mock_tests {
    use super::super::types::*;
    use std::collections::HashMap;
    use std::time::SystemTime;
    use uuid::Uuid;

    // ==================== Cache Type Tests ====================

    #[test]
    fn test_cache_type_variants() {
        let _memory = CacheType::Memory;
        let _redis = CacheType::Redis;
        let _disk = CacheType::Disk;
        let _hybrid = CacheType::Hybrid;
    }

    #[test]
    fn test_eviction_policy_variants() {
        let _lru = EvictionPolicy::Lru;
        let _lfu = EvictionPolicy::Lfu;
        let _fifo = EvictionPolicy::Fifo;
        let _random = EvictionPolicy::Random;
    }

    // ==================== Storage Pool Tests ====================

    #[test]
    fn test_storage_pool_creation() {
        let pool = StoragePool {
            id: "pool_001".to_string(),
            name: "test_pool".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 1_000_000_000,
            used_size: 500_000_000,
            available_size: 500_000_000,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Hot,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.name, "test_pool");
        assert_eq!(pool.pool_type, StoragePoolType::Zfs);
        assert_eq!(pool.health, PoolHealth::Online);
    }

    #[test]
    fn test_storage_pool_capacity_percentage() {
        let pool = StoragePool {
            id: "pool_002".to_string(),
            name: "capacity_test".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 1_000_000,
            used_size: 750_000,
            available_size: 250_000,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Warm,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        let capacity_used = (pool.used_size as f64 / pool.total_size as f64) * 100.0;
        assert!((capacity_used - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_storage_pool_types() {
        assert_eq!(StoragePoolType::Zfs, StoragePoolType::Zfs);
        assert_eq!(StoragePoolType::Filesystem, StoragePoolType::Filesystem);
        assert_eq!(StoragePoolType::Block, StoragePoolType::Block);
        assert_eq!(StoragePoolType::Object, StoragePoolType::Object);
        assert_ne!(StoragePoolType::Zfs, StoragePoolType::Block);
    }

    #[test]
    fn test_pool_health_states() {
        assert_eq!(PoolHealth::Online, PoolHealth::Online);
        assert_eq!(PoolHealth::Degraded, PoolHealth::Degraded);
        assert_eq!(PoolHealth::Faulted, PoolHealth::Faulted);
        assert_eq!(PoolHealth::Offline, PoolHealth::Offline);
        assert_eq!(PoolHealth::Unavailable, PoolHealth::Unavailable);
        assert_eq!(PoolHealth::Removed, PoolHealth::Removed);
        assert_ne!(PoolHealth::Online, PoolHealth::Degraded);
    }

    #[test]
    fn test_storage_pool_with_properties() {
        let mut props = HashMap::new();
        props.insert("compression".to_string(), "lz4".to_string());
        props.insert("dedup".to_string(), "off".to_string());

        let pool = StoragePool {
            id: "pool_003".to_string(),
            name: "props_pool".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 1_000_000,
            used_size: 100_000,
            available_size: 900_000,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Hot,
            properties: props.clone(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.properties.len(), 2);
        assert_eq!(pool.properties.get("compression"), Some(&"lz4".to_string()));
    }

    #[test]
    fn test_storage_pool_with_datasets() {
        let datasets = vec![
            "tank/data".to_string(),
            "tank/backup".to_string(),
            "tank/archive".to_string(),
        ];

        let pool = StoragePool {
            id: "pool_004".to_string(),
            name: "multi_dataset".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 5_000_000,
            used_size: 1_000_000,
            available_size: 4_000_000,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Hot,
            properties: HashMap::new(),
            datasets: datasets.clone(),
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.datasets.len(), 3);
        assert!(pool.datasets.contains(&"tank/data".to_string()));
    }

    #[test]
    fn test_storage_pool_degraded() {
        let pool = StoragePool {
            id: "pool_005".to_string(),
            name: "degraded_pool".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 1_000_000,
            used_size: 500_000,
            available_size: 500_000,
            health: PoolHealth::Degraded,
            tier: crate::canonical_types::storage::StorageTier::Cold,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.health, PoolHealth::Degraded);
    }

    // ==================== Storage Quota Tests ====================

    #[test]
    fn test_storage_quota_creation() {
        let quota = StorageQuota {
            id: "quota_001".to_string(),
            soft_limit: Some(8_000_000),
            hard_limit: Some(10_000_000),
            current_usage: 3_000_000,
            last_checked: SystemTime::now(),
            enforcement: QuotaEnforcement::Block,
        };

        assert_eq!(quota.id, "quota_001");
        assert_eq!(quota.soft_limit, Some(8_000_000));
        assert_eq!(quota.hard_limit, Some(10_000_000));
        assert_eq!(quota.enforcement, QuotaEnforcement::Block);
    }

    #[test]
    fn test_storage_quota_no_limits() {
        let quota = StorageQuota {
            id: "quota_002".to_string(),
            soft_limit: None,
            hard_limit: None,
            current_usage: 5_000_000,
            last_checked: SystemTime::now(),
            enforcement: QuotaEnforcement::None,
        };

        assert!(quota.soft_limit.is_none());
        assert!(quota.hard_limit.is_none());
        assert_eq!(quota.enforcement, QuotaEnforcement::None);
    }

    #[test]
    fn test_quota_enforcement_types() {
        assert_eq!(QuotaEnforcement::None, QuotaEnforcement::None);
        assert_eq!(QuotaEnforcement::Warn, QuotaEnforcement::Warn);
        assert_eq!(QuotaEnforcement::Block, QuotaEnforcement::Block);
        assert_ne!(QuotaEnforcement::Warn, QuotaEnforcement::Block);
    }

    #[test]
    fn test_storage_quota_soft_limit_exceeded() {
        let quota = StorageQuota {
            id: "quota_003".to_string(),
            soft_limit: Some(5_000_000),
            hard_limit: Some(10_000_000),
            current_usage: 7_000_000, // Over soft limit
            last_checked: SystemTime::now(),
            enforcement: QuotaEnforcement::Warn,
        };

        assert!(quota.current_usage > quota.soft_limit.unwrap());
        assert!(quota.current_usage < quota.hard_limit.unwrap());
    }

    #[test]
    fn test_storage_quota_hard_limit_exceeded() {
        let quota = StorageQuota {
            id: "quota_004".to_string(),
            soft_limit: Some(8_000_000),
            hard_limit: Some(10_000_000),
            current_usage: 11_000_000, // Over hard limit
            last_checked: SystemTime::now(),
            enforcement: QuotaEnforcement::Block,
        };

        assert!(quota.current_usage > quota.hard_limit.unwrap());
    }

    // ==================== Storage Service Stats Tests ====================

    #[test]
    fn test_storage_service_stats_default() {
        let stats = StorageServiceStats::default();

        assert_eq!(stats.total_operations, 0);
        assert_eq!(stats.read_operations, 0);
        assert_eq!(stats.write_operations, 0);
        assert_eq!(stats.delete_operations, 0);
        assert_eq!(stats.bytes_read, 0);
        assert_eq!(stats.bytes_written, 0);
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.cache_misses, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn test_storage_service_stats_with_operations() {
        let stats = StorageServiceStats {
            total_operations: 1000,
            read_operations: 600,
            write_operations: 350,
            delete_operations: 50,
            bytes_read: 10_000_000,
            bytes_written: 5_000_000,
            cache_hits: 800,
            cache_misses: 200,
            errors: 5,
            last_reset: SystemTime::now(),
        };

        assert_eq!(stats.total_operations, 1000);
        assert_eq!(
            stats.read_operations + stats.write_operations + stats.delete_operations,
            1000
        );
    }

    #[test]
    fn test_storage_service_stats_cache_hit_rate() {
        let stats = StorageServiceStats {
            total_operations: 1000,
            read_operations: 600,
            write_operations: 400,
            delete_operations: 0,
            bytes_read: 5_000_000,
            bytes_written: 3_000_000,
            cache_hits: 850,
            cache_misses: 150,
            errors: 0,
            last_reset: SystemTime::now(),
        };

        let total_cache_ops = stats.cache_hits + stats.cache_misses;
        let hit_rate = stats.cache_hits as f64 / total_cache_ops as f64;
        assert!((hit_rate - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_storage_service_stats_error_rate() {
        let stats = StorageServiceStats {
            total_operations: 10000,
            read_operations: 6000,
            write_operations: 4000,
            delete_operations: 0,
            bytes_read: 50_000_000,
            bytes_written: 30_000_000,
            cache_hits: 8000,
            cache_misses: 2000,
            errors: 100,
            last_reset: SystemTime::now(),
        };

        let error_rate = stats.errors as f64 / stats.total_operations as f64;
        assert!((error_rate - 0.01).abs() < 0.001);
    }

    // ==================== Storage Operation Result Tests ====================

    #[test]
    fn test_storage_operation_result_success() {
        let result = StorageOperationResult {
            operation_id: Uuid::new_v4(),
            operation_type: StorageOperationType::CreateDataset,
            success: true,
            error_message: None,
            bytes_processed: Some(1024),
            timestamp: SystemTime::now(),
        };

        assert!(result.success);
        assert_eq!(result.operation_type, StorageOperationType::CreateDataset);
        assert!(result.error_message.is_none());
        assert!(result.bytes_processed.is_some());
    }

    #[test]
    fn test_storage_operation_result_failure() {
        let result = StorageOperationResult {
            operation_id: Uuid::new_v4(),
            operation_type: StorageOperationType::Delete,
            success: false,
            error_message: Some("Permission denied".to_string()),
            bytes_processed: None,
            timestamp: SystemTime::now(),
        };

        assert!(!result.success);
        assert!(result.error_message.is_some());
        assert!(result.bytes_processed.is_none());
    }

    #[test]
    fn test_storage_operation_types() {
        assert_eq!(StorageOperationType::Read, StorageOperationType::Read);
        assert_eq!(StorageOperationType::Write, StorageOperationType::Write);
        assert_eq!(StorageOperationType::Delete, StorageOperationType::Delete);
        assert_eq!(
            StorageOperationType::CreatePool,
            StorageOperationType::CreatePool
        );
        assert_eq!(
            StorageOperationType::CreateDataset,
            StorageOperationType::CreateDataset
        );
        assert_eq!(
            StorageOperationType::CreateSnapshot,
            StorageOperationType::CreateSnapshot
        );
        assert_ne!(StorageOperationType::Read, StorageOperationType::Write);
    }

    #[test]
    fn test_storage_operation_result_with_bytes() {
        let result = StorageOperationResult {
            operation_id: Uuid::new_v4(),
            operation_type: StorageOperationType::Write,
            success: true,
            error_message: None,
            bytes_processed: Some(2048),
            timestamp: SystemTime::now(),
        };

        assert_eq!(result.bytes_processed, Some(2048));
    }

    #[test]
    fn test_storage_operation_result_timestamp() {
        let before = SystemTime::now();
        let result = StorageOperationResult {
            operation_id: Uuid::new_v4(),
            operation_type: StorageOperationType::Read,
            success: true,
            error_message: None,
            bytes_processed: Some(512),
            timestamp: SystemTime::now(),
        };
        let after = SystemTime::now();

        assert!(result.timestamp >= before);
        assert!(result.timestamp <= after);
    }

    // ==================== Storage Pool Edge Cases ====================

    #[test]
    fn test_storage_pool_zero_capacity() {
        let pool = StoragePool {
            id: "pool_empty".to_string(),
            name: "empty_pool".to_string(),
            pool_type: StoragePoolType::Filesystem,
            total_size: 0,
            used_size: 0,
            available_size: 0,
            health: PoolHealth::Offline,
            tier: crate::canonical_types::storage::StorageTier::Archive,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.total_size, 0);
        assert_eq!(pool.health, PoolHealth::Offline);
    }

    #[test]
    fn test_storage_pool_full_capacity() {
        let pool = StoragePool {
            id: "pool_full".to_string(),
            name: "full_pool".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 1_000_000,
            used_size: 1_000_000,
            available_size: 0,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Hot,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.used_size, pool.total_size);
        assert_eq!(pool.available_size, 0);
    }

    #[test]
    fn test_storage_pool_large_capacity() {
        let pool = StoragePool {
            id: "pool_large".to_string(),
            name: "large_pool".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 10_000_000_000_000, // 10TB
            used_size: 5_000_000_000_000,   // 5TB
            available_size: 5_000_000_000_000,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Warm,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert!(pool.total_size > 1_000_000_000_000);
    }

    // ==================== Quota Edge Cases ====================

    #[test]
    fn test_storage_quota_zero_usage() {
        let quota = StorageQuota {
            id: "quota_zero".to_string(),
            soft_limit: Some(10_000_000),
            hard_limit: Some(15_000_000),
            current_usage: 0,
            last_checked: SystemTime::now(),
            enforcement: QuotaEnforcement::Warn,
        };

        assert_eq!(quota.current_usage, 0);
    }

    #[test]
    fn test_storage_quota_only_soft_limit() {
        let quota = StorageQuota {
            id: "quota_soft".to_string(),
            soft_limit: Some(10_000_000),
            hard_limit: None,
            current_usage: 5_000_000,
            last_checked: SystemTime::now(),
            enforcement: QuotaEnforcement::Warn,
        };

        assert!(quota.soft_limit.is_some());
        assert!(quota.hard_limit.is_none());
    }

    #[test]
    fn test_storage_quota_only_hard_limit() {
        let quota = StorageQuota {
            id: "quota_hard".to_string(),
            soft_limit: None,
            hard_limit: Some(10_000_000),
            current_usage: 5_000_000,
            last_checked: SystemTime::now(),
            enforcement: QuotaEnforcement::Block,
        };

        assert!(quota.soft_limit.is_none());
        assert!(quota.hard_limit.is_some());
    }

    // ==================== Stats Edge Cases ====================

    #[test]
    fn test_storage_service_stats_perfect_cache_hit_rate() {
        let stats = StorageServiceStats {
            total_operations: 1000,
            read_operations: 1000,
            write_operations: 0,
            delete_operations: 0,
            bytes_read: 5_000_000,
            bytes_written: 0,
            cache_hits: 1000,
            cache_misses: 0,
            errors: 0,
            last_reset: SystemTime::now(),
        };

        let hit_rate = if stats.cache_hits + stats.cache_misses > 0 {
            stats.cache_hits as f64 / (stats.cache_hits + stats.cache_misses) as f64
        } else {
            0.0
        };
        assert!((hit_rate - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_storage_service_stats_zero_cache_hit_rate() {
        let stats = StorageServiceStats {
            total_operations: 1000,
            read_operations: 1000,
            write_operations: 0,
            delete_operations: 0,
            bytes_read: 5_000_000,
            bytes_written: 0,
            cache_hits: 0,
            cache_misses: 1000,
            errors: 0,
            last_reset: SystemTime::now(),
        };

        let hit_rate = if stats.cache_hits + stats.cache_misses > 0 {
            stats.cache_hits as f64 / (stats.cache_hits + stats.cache_misses) as f64
        } else {
            0.0
        };
        assert!((hit_rate - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_storage_service_stats_large_operations() {
        let stats = StorageServiceStats {
            total_operations: 1_000_000_000,
            read_operations: 600_000_000,
            write_operations: 400_000_000,
            delete_operations: 0,
            bytes_read: 500_000_000_000_000,
            bytes_written: 300_000_000_000_000,
            cache_hits: 800_000_000,
            cache_misses: 200_000_000,
            errors: 1_000,
            last_reset: SystemTime::now(),
        };

        assert!(stats.total_operations > 100_000_000);
        assert!(stats.bytes_read > 100_000_000_000);
    }

    // ==================== Additional Operation Type Tests ====================

    #[test]
    fn test_storage_operation_type_list() {
        let op = StorageOperationType::List;
        assert_eq!(op, StorageOperationType::List);
    }

    #[test]
    fn test_storage_operation_type_create_pool() {
        let op = StorageOperationType::CreatePool;
        assert_eq!(op, StorageOperationType::CreatePool);
    }

    #[test]
    fn test_storage_operation_type_set_quota() {
        let op = StorageOperationType::SetQuota;
        assert_eq!(op, StorageOperationType::SetQuota);
    }

    #[test]
    fn test_storage_operation_type_cache_operation() {
        let op = StorageOperationType::CacheOperation;
        assert_eq!(op, StorageOperationType::CacheOperation);
    }

    #[test]
    fn test_storage_operation_type_create_snapshot() {
        let op = StorageOperationType::CreateSnapshot;
        assert_eq!(op, StorageOperationType::CreateSnapshot);
    }

    // ==================== Additional Pool Type Tests ====================

    #[test]
    fn test_storage_pool_type_filesystem() {
        let pool = StoragePool {
            id: "fs_pool".to_string(),
            name: "filesystem_pool".to_string(),
            pool_type: StoragePoolType::Filesystem,
            total_size: 1_000_000,
            used_size: 100_000,
            available_size: 900_000,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Warm,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.pool_type, StoragePoolType::Filesystem);
    }

    #[test]
    fn test_storage_pool_type_block() {
        let pool = StoragePool {
            id: "block_pool".to_string(),
            name: "block_storage".to_string(),
            pool_type: StoragePoolType::Block,
            total_size: 2_000_000,
            used_size: 500_000,
            available_size: 1_500_000,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Hot,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.pool_type, StoragePoolType::Block);
    }

    #[test]
    fn test_storage_pool_type_object() {
        let pool = StoragePool {
            id: "obj_pool".to_string(),
            name: "object_storage".to_string(),
            pool_type: StoragePoolType::Object,
            total_size: 5_000_000,
            used_size: 1_000_000,
            available_size: 4_000_000,
            health: PoolHealth::Online,
            tier: crate::canonical_types::storage::StorageTier::Cold,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.pool_type, StoragePoolType::Object);
    }

    // ==================== Additional Health State Tests ====================

    #[test]
    fn test_pool_health_unavailable() {
        let pool = StoragePool {
            id: "unavail_pool".to_string(),
            name: "unavailable_pool".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 1_000_000,
            used_size: 0,
            available_size: 0,
            health: PoolHealth::Unavailable,
            tier: crate::canonical_types::storage::StorageTier::Archive,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.health, PoolHealth::Unavailable);
    }

    #[test]
    fn test_pool_health_removed() {
        let pool = StoragePool {
            id: "removed_pool".to_string(),
            name: "removed_pool".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 0,
            used_size: 0,
            available_size: 0,
            health: PoolHealth::Removed,
            tier: crate::canonical_types::storage::StorageTier::Archive,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.health, PoolHealth::Removed);
    }

    #[test]
    fn test_pool_health_faulted() {
        let pool = StoragePool {
            id: "faulted_pool".to_string(),
            name: "faulted_pool".to_string(),
            pool_type: StoragePoolType::Zfs,
            total_size: 1_000_000,
            used_size: 500_000,
            available_size: 500_000,
            health: PoolHealth::Faulted,
            tier: crate::canonical_types::storage::StorageTier::Hot,
            properties: HashMap::new(),
            datasets: vec![],
            last_updated: SystemTime::now(),
        };

        assert_eq!(pool.health, PoolHealth::Faulted);
    }

    // ==================== Stats Operation Breakdown Tests ====================

    #[test]
    fn test_storage_service_stats_read_heavy() {
        let stats = StorageServiceStats {
            total_operations: 1000,
            read_operations: 900,
            write_operations: 90,
            delete_operations: 10,
            bytes_read: 10_000_000,
            bytes_written: 1_000_000,
            cache_hits: 850,
            cache_misses: 150,
            errors: 2,
            last_reset: SystemTime::now(),
        };

        assert!(stats.read_operations > stats.write_operations);
        assert_eq!(stats.read_operations, 900);
    }

    #[test]
    fn test_storage_service_stats_write_heavy() {
        let stats = StorageServiceStats {
            total_operations: 1000,
            read_operations: 100,
            write_operations: 850,
            delete_operations: 50,
            bytes_read: 1_000_000,
            bytes_written: 10_000_000,
            cache_hits: 100,
            cache_misses: 900,
            errors: 5,
            last_reset: SystemTime::now(),
        };

        assert!(stats.write_operations > stats.read_operations);
        assert_eq!(stats.write_operations, 850);
    }

    #[test]
    fn test_storage_service_stats_balanced() {
        let stats = StorageServiceStats {
            total_operations: 1000,
            read_operations: 500,
            write_operations: 450,
            delete_operations: 50,
            bytes_read: 5_000_000,
            bytes_written: 4_500_000,
            cache_hits: 500,
            cache_misses: 500,
            errors: 0,
            last_reset: SystemTime::now(),
        };

        assert_eq!(stats.read_operations, 500);
        assert_eq!(stats.write_operations, 450);
        assert_eq!(stats.delete_operations, 50);
    }

    #[test]
    fn test_storage_operation_result_zero_bytes() {
        let result = StorageOperationResult {
            operation_id: Uuid::new_v4(),
            operation_type: StorageOperationType::Delete,
            success: true,
            error_message: None,
            bytes_processed: Some(0),
            timestamp: SystemTime::now(),
        };

        assert_eq!(result.bytes_processed, Some(0));
    }

    #[test]
    fn test_storage_operation_result_large_bytes() {
        let result = StorageOperationResult {
            operation_id: Uuid::new_v4(),
            operation_type: StorageOperationType::Write,
            success: true,
            error_message: None,
            bytes_processed: Some(1_000_000_000_000), // 1TB
            timestamp: SystemTime::now(),
        };

        assert!(result.bytes_processed.unwrap() > 100_000_000_000);
    }
}
