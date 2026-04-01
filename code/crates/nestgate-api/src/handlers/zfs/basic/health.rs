// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS aggregate health endpoint.

use super::service::get_zfs_service;
use super::types::{ZfsHealthResponse, evaluate_zfs_health};
use crate::dev_stubs::zfs::PoolOperations;
use crate::routes::AppState;
use axum::{extract::State, http::StatusCode, response::Json};
use tracing::{error, info};

/// Get ZFS health status
pub async fn get_zfs_health(
    State(state): State<AppState>,
) -> Result<Json<ZfsHealthResponse>, StatusCode> {
    info!("API: Getting ZFS health status");

    match get_zfs_service(&state).await {
        Ok(service) => match service.list_pools() {
            Ok(pools) => {
                let response = evaluate_zfs_health(pools);

                info!("ZFS health check completed - healthy: {}", response.healthy);
                Ok(Json(response))
            }
            Err(e) => {
                error!("Failed to get ZFS health: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Err(e) => {
            error!("ZFS service unavailable: {}", e);
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
