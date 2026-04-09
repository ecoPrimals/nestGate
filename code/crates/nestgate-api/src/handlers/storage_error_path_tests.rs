// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **STORAGE COMPREHENSIVE ERROR PATH TESTS**
//!
//! Testing strategy:
//! - Storage operations error handling
//! - Edge cases for capacity and limits
//! - Data validation and integrity
//! - Concurrent storage operations
//! - Pool/dataset/snapshot validation

use super::storage::*;

#[tokio::test]
async fn test_get_storage_pools_returns_ok() {
    let result = get_storage_pools().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_storage_pools_non_empty() {
    let result = get_storage_pools().await.unwrap();
    let pools = result.0;

    assert!(!pools.is_empty(), "Storage pools should not be empty");
    assert!(pools.len() > 0);
}

#[tokio::test]
async fn test_storage_pool_names_valid() {
    let result = get_storage_pools().await.unwrap();
    let pools = result.0;

    for pool in pools {
        assert!(!pool.name.is_empty(), "Pool name should not be empty");
        assert!(pool.name.len() > 2, "Pool name should be meaningful");
    }
}

#[tokio::test]
async fn test_storage_pool_capacity_consistency() {
    let result = get_storage_pools().await.unwrap();
    let pools = result.0;

    for pool in pools {
        // Used + Available should equal Total
        assert_eq!(
            pool.used_capacity_gb + pool.available_capacity_gb,
            pool.total_capacity_gb,
            "Pool capacity should be consistent: used + available = total"
        );
    }
}

#[tokio::test]
async fn test_storage_pool_capacity_non_negative() {
    let result = get_storage_pools().await.unwrap();
    let pools = result.0;

    for pool in pools {
        assert!(
            pool.total_capacity_gb > 0,
            "Total capacity should be positive"
        );
        assert!(
            pool.used_capacity_gb >= 0,
            "Used capacity should be non-negative"
        );
        assert!(
            pool.available_capacity_gb >= 0,
            "Available capacity should be non-negative"
        );
    }
}

#[tokio::test]
async fn test_storage_pool_health_status_valid() {
    let result = get_storage_pools().await.unwrap();
    let pools = result.0;

    let valid_statuses = ["healthy", "degraded", "faulted", "offline", "online"];

    for pool in pools {
        assert!(
            !pool.health_status.is_empty(),
            "Health status should not be empty"
        );
        // Health status should be one of the known values (case-insensitive)
        assert!(
            valid_statuses
                .iter()
                .any(|&s| s.eq_ignore_ascii_case(&pool.health_status)),
            "Health status '{}' should be valid",
            pool.health_status
        );
    }
}

#[tokio::test]
async fn test_get_storage_datasets_returns_ok() {
    let result = get_storage_datasets().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_storage_datasets_non_empty() {
    let result = get_storage_datasets().await.unwrap();
    let datasets = result.0;

    assert!(!datasets.is_empty(), "Storage datasets should not be empty");
}

#[tokio::test]
async fn test_storage_dataset_names_valid() {
    let result = get_storage_datasets().await.unwrap();
    let datasets = result.0;

    for dataset in datasets {
        assert!(!dataset.name.is_empty(), "Dataset name should not be empty");
        assert!(
            !dataset.pool_name.is_empty(),
            "Pool name should not be empty"
        );
        // Dataset names should typically have a pool prefix
        assert!(
            dataset.name.contains(&dataset.pool_name) || dataset.name.contains('/'),
            "Dataset name should reference its pool"
        );
    }
}

#[tokio::test]
async fn test_storage_dataset_ratios_valid() {
    let result = get_storage_datasets().await.unwrap();
    let datasets = result.0;

    for dataset in datasets {
        // Compression ratio should be >= 1.0
        assert!(
            dataset.compression_ratio >= 1.0,
            "Compression ratio should be >= 1.0 (got {})",
            dataset.compression_ratio
        );
        // Dedup ratio should be >= 1.0
        assert!(
            dataset.dedup_ratio >= 1.0,
            "Dedup ratio should be >= 1.0 (got {})",
            dataset.dedup_ratio
        );
        // Ratios should be reasonable (< 10x typically)
        assert!(
            dataset.compression_ratio < 10.0,
            "Compression ratio seems unrealistic: {}",
            dataset.compression_ratio
        );
        assert!(
            dataset.dedup_ratio < 10.0,
            "Dedup ratio seems unrealistic: {}",
            dataset.dedup_ratio
        );
    }
}

#[tokio::test]
async fn test_storage_dataset_space_non_negative() {
    let result = get_storage_datasets().await.unwrap();
    let datasets = result.0;

    for dataset in datasets {
        assert!(
            dataset.used_space_gb >= 0,
            "Used space should be non-negative"
        );
    }
}

#[tokio::test]
async fn test_concurrent_storage_pool_requests() {
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = tokio::spawn(async { get_storage_pools().await });
        handles.push(handle);
    }

    let results = futures_util::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok(), "Task should complete");
        assert!(
            result.unwrap().is_ok(),
            "Storage pools request should succeed"
        );
    }
}

#[tokio::test]
async fn test_concurrent_storage_dataset_requests() {
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = tokio::spawn(async { get_storage_datasets().await });
        handles.push(handle);
    }

    let results = futures_util::future::join_all(handles).await;

    for result in results {
        assert!(result.is_ok(), "Task should complete");
        assert!(
            result.unwrap().is_ok(),
            "Storage datasets request should succeed"
        );
    }
}

#[tokio::test]
async fn test_storage_pools_consistency_across_calls() {
    let result1 = get_storage_pools().await.unwrap().0;
    let result2 = get_storage_pools().await.unwrap().0;

    // Should return same number of pools
    assert_eq!(result1.len(), result2.len());

    // Pool names should match
    for (pool1, pool2) in result1.iter().zip(result2.iter()) {
        assert_eq!(pool1.name, pool2.name);
    }
}

#[tokio::test]
async fn test_storage_datasets_consistency_across_calls() {
    let result1 = get_storage_datasets().await.unwrap().0;
    let result2 = get_storage_datasets().await.unwrap().0;

    // Should return same number of datasets
    assert_eq!(result1.len(), result2.len());

    // Dataset names should match
    for (ds1, ds2) in result1.iter().zip(result2.iter()) {
        assert_eq!(ds1.name, ds2.name);
        assert_eq!(ds1.pool_name, ds2.pool_name);
    }
}

#[test]
fn test_storage_handler_creation() {
    let handler = StorageHandler::new();
    assert!(format!("{:?}", handler).contains("StorageHandler"));
}

#[test]
fn test_storage_handler_default() {
    let handler = StorageHandler::default();
    assert!(format!("{:?}", handler).contains("StorageHandler"));
}

#[test]
fn test_storage_pool_struct_creation() {
    let pool = StoragePool {
        name: "test-pool".to_string(),
        status: "online".to_string(),
        size: 1000000,
        used: 500000,
        available: 500000,
        health: "healthy".to_string(),
        pool_type: "raidz".to_string(),
    };

    assert_eq!(pool.name, "test-pool");
    assert_eq!(pool.size, pool.used + pool.available);
}

#[test]
fn test_storage_dataset_struct_creation() {
    let dataset = StorageDataset {
        name: "test-pool/data".to_string(),
        pool: "test-pool".to_string(),
        size: 1000000,
        used: 400000,
        available: 600000,
        mount_point: "/mnt/data".to_string(),
        compression: "lz4".to_string(),
    };

    assert!(dataset.name.contains(&dataset.pool));
    assert_eq!(dataset.size, dataset.used + dataset.available);
}

#[test]
fn test_storage_snapshot_struct_creation() {
    let snapshot = StorageSnapshot {
        name: "snapshot-1".to_string(),
        dataset: "test-pool/data".to_string(),
        size: 100000,
        created: "2025-11-26".to_string(),
        referenced: 95000,
    };

    assert!(!snapshot.name.is_empty());
    assert!(!snapshot.dataset.is_empty());
    assert!(snapshot.size > 0);
}

#[test]
fn test_storage_metrics_struct_creation() {
    let metrics = StorageMetrics {
        total_pools: 2,
        total_datasets: 5,
        total_snapshots: 10,
        total_storage: 1000000000,
        used_storage: 600000000,
        available_storage: 400000000,
        iops: 1000.0,
        bandwidth_mbps: 500.0,
        health_status: "healthy".to_string(),
    };

    assert_eq!(
        metrics.total_storage,
        metrics.used_storage + metrics.available_storage
    );
    assert!(metrics.iops > 0.0);
    assert!(metrics.bandwidth_mbps > 0.0);
}

#[tokio::test]
async fn test_storage_pool_capacity_percentages() {
    let result = get_storage_pools().await.unwrap();
    let pools = result.0;

    for pool in pools {
        let usage_percent = (pool.used_capacity_gb as f64 / pool.total_capacity_gb as f64) * 100.0;
        assert!(
            usage_percent >= 0.0 && usage_percent <= 100.0,
            "Usage percentage should be between 0 and 100, got {}",
            usage_percent
        );
    }
}

#[tokio::test]
async fn test_rapid_sequential_storage_calls() {
    for _ in 0..20 {
        let pools = get_storage_pools().await;
        assert!(pools.is_ok());

        let datasets = get_storage_datasets().await;
        assert!(datasets.is_ok());
    }
}

#[test]
fn test_storage_structs_are_cloneable() {
    let pool = StoragePool {
        name: "test".to_string(),
        status: "online".to_string(),
        size: 1000,
        used: 500,
        available: 500,
        health: "healthy".to_string(),
        pool_type: "mirror".to_string(),
    };

    let pool_clone = pool.clone();
    assert_eq!(pool.name, pool_clone.name);
    assert_eq!(pool.size, pool_clone.size);
}

#[test]
fn test_storage_structs_are_debug() {
    let dataset = StorageDataset {
        name: "test".to_string(),
        pool: "pool".to_string(),
        size: 1000,
        used: 400,
        available: 600,
        mount_point: "/mnt/test".to_string(),
        compression: "lz4".to_string(),
    };

    let debug_output = format!("{:?}", dataset);
    assert!(debug_output.contains("StorageDataset"));
    assert!(debug_output.contains("test"));
}




