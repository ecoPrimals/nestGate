// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use axum::{http::StatusCode, response::Json};

use super::types::{StorageDatasetInfo, StorageMetrics, StoragePoolInfo, StorageSnapshotInfo};

/// **GET STORAGE POOLS HANDLER**
///
/// Retrieve information about all storage pools.
///
/// # Errors
///
/// This function currently always returns `Ok`, but returns `Result` for future error handling.
pub async fn get_storage_pools() -> Result<Json<Vec<StoragePoolInfo>>, StatusCode> {
    let pools = vec![
        StoragePoolInfo {
            name: "main-pool".to_string(),
            total_capacity_gb: 1000,
            used_capacity_gb: 400,
            available_capacity_gb: 600,
            health_status: "healthy".to_string(),
        },
        StoragePoolInfo {
            name: "backup-pool".to_string(),
            total_capacity_gb: 500,
            used_capacity_gb: 150,
            available_capacity_gb: 350,
            health_status: "healthy".to_string(),
        },
    ];

    Ok(Json(pools))
}

/// **GET STORAGE DATASETS HANDLER**
///
/// Retrieve information about all storage datasets.
///
/// # Errors
///
/// This function currently always returns `Ok`, but returns `Result` for future error handling.
pub async fn get_storage_datasets() -> Result<Json<Vec<StorageDatasetInfo>>, StatusCode> {
    let datasets = vec![
        StorageDatasetInfo {
            name: "main-pool/data".to_string(),
            pool_name: "main-pool".to_string(),
            used_space_gb: 200,
            compression_ratio: 1.5,
            dedup_ratio: 1.2,
        },
        StorageDatasetInfo {
            name: "main-pool/logs".to_string(),
            pool_name: "main-pool".to_string(),
            used_space_gb: 50,
            compression_ratio: 2.1,
            dedup_ratio: 1.8,
        },
    ];

    Ok(Json(datasets))
}

/// **GET STORAGE SNAPSHOTS HANDLER**
///
/// Retrieve information about all storage snapshots.
///
/// # Errors
///
/// This function currently always returns `Ok`, but returns `Result` for future error handling.
pub async fn get_storage_snapshots() -> Result<Json<Vec<StorageSnapshotInfo>>, StatusCode> {
    let snapshots = vec![
        StorageSnapshotInfo {
            name: "main-pool/data@backup-2024-01-15".to_string(),
            dataset_name: "main-pool/data".to_string(),
            created_at: std::time::SystemTime::now(),
            size_gb: 180,
        },
        StorageSnapshotInfo {
            name: "main-pool/logs@daily-2024-01-15".to_string(),
            dataset_name: "main-pool/logs".to_string(),
            created_at: std::time::SystemTime::now(),
            size_gb: 45,
        },
    ];

    Ok(Json(snapshots))
}

/// **GET STORAGE METRICS HANDLER**
///
/// Retrieve current storage performance metrics.
///
/// # Errors
///
/// This function currently always returns `Ok`, but returns `Result` for future error handling.
pub async fn get_storage_metrics() -> Result<Json<StorageMetrics>, StatusCode> {
    let metrics = StorageMetrics {
        total_pools: 2,
        total_datasets: 5,
        total_snapshots: 12,
        total_storage: 1_500_000_000_000,   // 1.5TB in bytes
        used_storage: 550_000_000_000,      // 550GB in bytes
        available_storage: 950_000_000_000, // 950GB in bytes
        iops: 1250.0,
        bandwidth_mbps: 450.5,
        health_status: "healthy".to_string(),
    };

    Ok(Json(metrics))
}
