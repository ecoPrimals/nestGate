// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Snapshot list and create handlers.

use super::service::get_zfs_service;
use super::types::CreateSnapshotRequest;
use crate::dev_stubs::zfs::{
    DatasetOperations, PoolOperations, SnapshotOperations, ZeroCostSnapshotInfo,
};
use crate::routes::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tracing::{error, info, warn};

/// List snapshots for a dataset
pub async fn list_snapshots(
    State(state): State<AppState>,
    Path((pool_name, dataset_name)): Path<(String, String)>,
) -> Result<Json<Vec<ZeroCostSnapshotInfo>>, StatusCode> {
    info!(
        "API: Listing snapshots for dataset '{}/{}''",
        pool_name, dataset_name
    );

    match get_zfs_service(&state).await {
        Ok(service) => match service.list_pools() {
            Ok(pools) => {
                if let Some(pool) = pools.into_iter().find(|p| p.name == pool_name) {
                    match service.list_datasets(&pool.name) {
                        Ok(datasets) => {
                            if let Some(dataset) =
                                datasets.into_iter().find(|d| d.name == dataset_name)
                            {
                                match service.list_snapshots(&dataset.name) {
                                    Ok(snapshots) => {
                                        info!(
                                            "Found {} snapshots for dataset '{}/{}''",
                                            snapshots.len(),
                                            pool_name,
                                            dataset_name
                                        );
                                        Ok(Json(snapshots))
                                    }
                                    Err(e) => {
                                        error!("Failed to list snapshots: {}", e);
                                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                                    }
                                }
                            } else {
                                warn!("Dataset not found: {}/{}", pool_name, dataset_name);
                                Err(StatusCode::NOT_FOUND)
                            }
                        }
                        Err(e) => {
                            error!("Failed to list datasets: {}", e);
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                } else {
                    warn!("Pool not found: {}", pool_name);
                    Err(StatusCode::NOT_FOUND)
                }
            }
            Err(e) => {
                error!("Failed to access pools: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            error!("ZFS service unavailable: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Create a snapshot
pub async fn create_snapshot(
    State(state): State<AppState>,
    Path((pool_name, dataset_name)): Path<(String, String)>,
    Json(request): Json<CreateSnapshotRequest>,
) -> Result<Json<ZeroCostSnapshotInfo>, StatusCode> {
    info!(
        "API: Creating snapshot '{}/{}@{}'",
        pool_name, dataset_name, request.name
    );

    match get_zfs_service(&state).await {
        Ok(service) => match service.list_pools() {
            Ok(pools) => {
                if let Some(pool) = pools.into_iter().find(|p| p.name == pool_name) {
                    match service.list_datasets(&pool.name) {
                        Ok(datasets) => {
                            if let Some(dataset) =
                                datasets.into_iter().find(|d| d.name == dataset_name)
                            {
                                match service.create_snapshot(&dataset.name, &request.name) {
                                    Ok(()) => {
                                        info!(
                                            "Snapshot created successfully: {}/{}@{}",
                                            pool_name, dataset_name, request.name
                                        );
                                        Ok(Json(ZeroCostSnapshotInfo {
                                            name: format!("{}@{}", dataset.name, request.name),
                                            creation_time: chrono::Utc::now().to_rfc3339(),
                                            used: 0,
                                            referenced: 0,
                                        }))
                                    }
                                    Err(e) => {
                                        error!("Failed to create snapshot: {}", e);
                                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                                    }
                                }
                            } else {
                                warn!("Dataset not found: {}/{}", pool_name, dataset_name);
                                Err(StatusCode::NOT_FOUND)
                            }
                        }
                        Err(e) => {
                            error!("Failed to list datasets: {}", e);
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                } else {
                    warn!("Pool not found: {}", pool_name);
                    Err(StatusCode::NOT_FOUND)
                }
            }
            Err(e) => {
                error!("Failed to access pools: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            error!("ZFS service unavailable: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
