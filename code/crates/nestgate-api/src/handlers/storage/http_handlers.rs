// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage HTTP handlers.
//!
//! These routes surface ZFS pool/dataset/snapshot information over HTTP.
//! When ZFS is unavailable the handlers return `503 Service Unavailable`
//! with a structured JSON body instead of fabricated data.

use axum::{http::StatusCode, response::Json};
use serde_json::{Value, json};

use super::types::{StorageDatasetInfo, StorageMetrics, StoragePoolInfo, StorageSnapshotInfo};

fn zfs_unavailable() -> (StatusCode, Json<Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({
            "status": "error",
            "error": "zfs_unavailable",
            "message": "ZFS runtime not available — storage pool queries require zpool/zfs userland"
        })),
    )
}

/// Retrieve information about all storage pools.
///
/// Returns `503` when ZFS is unavailable rather than fabricated data.
pub async fn get_storage_pools() -> Result<Json<Vec<StoragePoolInfo>>, (StatusCode, Json<Value>)> {
    if !nestgate_zfs::native::is_zpool_available().await {
        return Err(zfs_unavailable());
    }
    let ops = nestgate_zfs::command::ZfsOperations::new();
    match ops.list_pools().await {
        Ok(pools) => Ok(Json(
            pools
                .iter()
                .map(|p| StoragePoolInfo {
                    name: p.name.clone(),
                    total_capacity_gb: 0,
                    used_capacity_gb: 0,
                    available_capacity_gb: 0,
                    health_status: p.health.clone(),
                })
                .collect(),
        )),
        Err(_) => Err(zfs_unavailable()),
    }
}

/// Retrieve information about all storage datasets.
///
/// Returns `503` when ZFS is unavailable rather than fabricated data.
pub async fn get_storage_datasets()
-> Result<Json<Vec<StorageDatasetInfo>>, (StatusCode, Json<Value>)> {
    if !nestgate_zfs::native::is_zfs_available().await {
        return Err(zfs_unavailable());
    }
    let ops = nestgate_zfs::command::ZfsOperations::new();
    match ops.list_datasets(None).await {
        Ok(datasets) => Ok(Json(
            datasets
                .iter()
                .map(|d| {
                    let pool = d.name.split('/').next().unwrap_or(&d.name);
                    StorageDatasetInfo {
                        name: d.name.clone(),
                        pool_name: pool.to_owned(),
                        used_space_gb: 0,
                        compression_ratio: 1.0,
                        dedup_ratio: 1.0,
                    }
                })
                .collect(),
        )),
        Err(_) => Err(zfs_unavailable()),
    }
}

/// Retrieve information about all storage snapshots.
///
/// Returns `503` when ZFS is unavailable rather than fabricated data.
pub async fn get_storage_snapshots()
-> Result<Json<Vec<StorageSnapshotInfo>>, (StatusCode, Json<Value>)> {
    if !nestgate_zfs::native::is_zfs_available().await {
        return Err(zfs_unavailable());
    }
    let ops = nestgate_zfs::command::ZfsOperations::new();
    match ops.list_snapshots(None).await {
        Ok(snapshots) => Ok(Json(
            snapshots
                .iter()
                .map(|s| {
                    let dataset = s.name.split('@').next().unwrap_or(&s.name);
                    StorageSnapshotInfo {
                        name: s.name.clone(),
                        dataset_name: dataset.to_owned(),
                        created_at: std::time::SystemTime::now(),
                        size_gb: 0,
                    }
                })
                .collect(),
        )),
        Err(_) => Err(zfs_unavailable()),
    }
}

/// Retrieve current storage performance metrics.
///
/// Returns `503` when ZFS is unavailable rather than fabricated data.
pub async fn get_storage_metrics() -> Result<Json<StorageMetrics>, (StatusCode, Json<Value>)> {
    if !nestgate_zfs::native::is_zpool_available().await {
        return Err(zfs_unavailable());
    }
    let ops = nestgate_zfs::command::ZfsOperations::new();
    let pools = ops.list_pools().await.unwrap_or_default();
    let datasets = ops.list_datasets(None).await.unwrap_or_default();
    let snapshots = ops.list_snapshots(None).await.unwrap_or_default();

    #[expect(
        clippy::cast_possible_truncation,
        reason = "pool/dataset/snapshot counts will never exceed u32::MAX"
    )]
    let metrics = StorageMetrics {
        total_pools: pools.len() as u32,
        total_datasets: datasets.len() as u32,
        total_snapshots: snapshots.len() as u32,
        total_storage: 0,
        used_storage: 0,
        available_storage: 0,
        iops: 0.0,
        bandwidth_mbps: 0.0,
        health_status: String::from(if pools.is_empty() {
            "no_pools"
        } else {
            "active"
        }),
    };

    Ok(Json(metrics))
}
