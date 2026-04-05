// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dataset list, get, and create handlers.

use super::service::get_zfs_service;
use super::types::CreateDatasetRequest;
use crate::dev_stubs::zfs::{DatasetOperations, PoolOperations, ZeroCostDatasetInfo};
use crate::routes::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use nestgate_core::canonical_types::StorageTier;
use tracing::{error, info, warn};

/// List datasets in a pool
pub async fn list_datasets(
    State(state): State<AppState>,
    Path(pool_name): Path<String>,
) -> Result<Json<Vec<ZeroCostDatasetInfo>>, StatusCode> {
    info!("API: Listing datasets in pool '{}'", pool_name);

    match get_zfs_service(&state).await {
        Ok(service) => match service.list_pools() {
            Ok(pools) => {
                if let Some(pool) = pools.into_iter().find(|p| p.name == pool_name) {
                    match service.list_datasets(&pool.name) {
                        Ok(datasets) => {
                            info!("Found {} datasets in pool '{}'", datasets.len(), pool_name);
                            Ok(Json(datasets))
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
                error!("Failed to access pool: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            error!("ZFS service unavailable: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Get dataset information
pub async fn get_dataset(
    State(state): State<AppState>,
    Path((pool_name, dataset_name)): Path<(String, String)>,
) -> Result<Json<ZeroCostDatasetInfo>, StatusCode> {
    info!(
        "API: Getting dataset info for '{}/{}''",
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
                                info!("Found dataset: {}/{}", pool_name, dataset_name);
                                Ok(Json(dataset))
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

/// Create a new dataset
pub async fn create_dataset(
    State(state): State<AppState>,
    Path(pool_name): Path<String>,
    Json(request): Json<CreateDatasetRequest>,
) -> Result<Json<ZeroCostDatasetInfo>, StatusCode> {
    info!("API: Creating dataset '{}/{}''", pool_name, request.name);

    match get_zfs_service(&state).await {
        Ok(service) => match service.list_pools() {
            Ok(pools) => {
                if let Some(pool) = pools.into_iter().find(|p| p.name == pool_name) {
                    match service.create_dataset_with_tier(
                        &pool.name,
                        &request.name,
                        StorageTier::Warm,
                    ) {
                        Ok(dataset) => {
                            info!("Dataset {} created successfully", request.name);
                            Ok(Json(dataset))
                        }
                        Err(e) => {
                            error!("Failed to create dataset: {}", e);
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
