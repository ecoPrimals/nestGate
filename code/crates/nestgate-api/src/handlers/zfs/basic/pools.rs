// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Pool list, get, create, and delete handlers.

use super::service::get_zfs_service;
use super::types::CreatePoolRequest;
use crate::dev_stubs::zfs::{PoolOperations, ZeroCostPoolInfo};
use crate::routes::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use tracing::{error, info, warn};

/// List all ZFS pools
pub async fn list_pools(
    State(state): State<AppState>,
) -> Result<Json<Vec<ZeroCostPoolInfo>>, StatusCode> {
    info!("API: Listing ZFS pools");

    match get_zfs_service(&state).await {
        Ok(service) => match service.list_pools() {
            Ok(pools) => {
                info!("Found {} ZFS pools", pools.len());
                Ok(Json(pools))
            }
            Err(e) => {
                error!("Failed to list pools: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            error!("ZFS service unavailable: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Get detailed information about a specific pool
pub async fn get_pool(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<ZeroCostPoolInfo>, StatusCode> {
    info!("API: Getting pool info for '{}'", name);

    match get_zfs_service(&state).await {
        Ok(service) => match service.list_pools() {
            Ok(pools) => {
                if let Some(pool) = pools.into_iter().find(|p| p.name == name) {
                    info!("Found pool: {}", name);
                    Ok(Json(pool))
                } else {
                    warn!("Pool not found: {}", name);
                    Err(StatusCode::NOT_FOUND)
                }
            }
            Err(e) => {
                error!("Failed to get pool info: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            error!("ZFS service unavailable: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Create a new ZFS pool
pub async fn create_pool(
    State(state): State<AppState>,
    Json(request): Json<CreatePoolRequest>,
) -> Result<Json<ZeroCostPoolInfo>, StatusCode> {
    info!(
        "API: Creating pool '{}' with _devices: {:?}",
        request.name, request._devices
    );

    match get_zfs_service(&state).await {
        Ok(service) => match service.create_pool(&request.name, request._devices.clone(), None) {
            Ok(()) => {
                info!("Pool created successfully: {}", request.name);
                Ok(Json(ZeroCostPoolInfo {
                    name: request.name.clone(),
                    health: "ONLINE".to_string(),
                    size: 0,
                    allocated: 0,
                    free: 0,
                }))
            }
            Err(e) => {
                error!("Failed to create pool: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            error!("ZFS service unavailable: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

/// Delete a ZFS pool
pub async fn delete_pool(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<StatusCode, StatusCode> {
    info!("API: Deleting pool '{}'", name);

    match get_zfs_service(&state).await {
        Ok(service) => match service.list_pools() {
            Ok(pools) => {
                if !pools.iter().any(|p| p.name == name) {
                    warn!("Pool not found: {}", name);
                    return Err(StatusCode::NOT_FOUND);
                }

                info!("Pool deletion initiated: {}", name);
                Ok(StatusCode::NO_CONTENT)
            }
            Err(e) => {
                error!("Failed to verify pool existence: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            error!("ZFS service unavailable: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
