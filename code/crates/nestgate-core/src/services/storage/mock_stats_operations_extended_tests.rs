// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Edge cases and extended coverage for mock storage types (pools, quotas, stats, operations).

#![cfg(test)]

use super::types::*;
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

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
