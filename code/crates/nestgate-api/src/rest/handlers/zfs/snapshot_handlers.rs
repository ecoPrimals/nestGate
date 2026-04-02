// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Snapshot data handlers for ZFS operations.
// GET/POST/DELETE for snapshots and clone operations.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use tracing::info;

use crate::rest::models::{CloneSnapshotRequest, CreateSnapshotRequest};
use crate::rest::{ApiState, DataError, ListQuery};

/// Deprecated REST snapshot routes do not implement real ZFS snapshot I/O (same JSON body shape as
/// ZFS production placeholder `501` responses).
fn zfs_snapshot_rest_not_implemented() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "not_implemented",
            "feature": "zfs_http",
            "details": null,
        })),
    )
}

/// List snapshots for a dataset
/// GET /api/v1/zfs/datasets/:dataset/snapshots
///
/// # Errors
///
/// Returns [`Json`] containing [`DataError`] when the dataset is missing,
/// snapshot listing fails, or the response cannot be built.
#[deprecated(
    since = "0.2.0",
    note = "Use JSON-RPC via nestgate-rpc semantic router"
)]
pub async fn list_snapshots(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Query(_query): Query<ListQuery>,
) -> impl IntoResponse {
    if state.zfs_engines.contains_key(&dataset_name) {
        zfs_snapshot_rest_not_implemented().into_response()
    } else {
        Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        ))
        .into_response()
    }
}

/// Create a new snapshot
/// POST /api/v1/zfs/datasets/:dataset/snapshots
///
/// # Errors
///
/// Returns [`Json`] containing [`DataError`] when the dataset is missing,
/// validation fails, or creation fails.
#[deprecated(
    since = "0.2.0",
    note = "Use JSON-RPC via nestgate-rpc semantic router"
)]
pub async fn create_snapshot(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Json(_request): Json<CreateSnapshotRequest>,
) -> impl IntoResponse {
    info!("Create snapshot requested for dataset: {}", dataset_name);
    if state.zfs_engines.contains_key(&dataset_name) {
        zfs_snapshot_rest_not_implemented().into_response()
    } else {
        Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        ))
        .into_response()
    }
}

/// Get a specific snapshot
/// GET /api/v1/zfs/datasets/:dataset/snapshots/:snapshot
#[deprecated(
    since = "0.2.0",
    note = "Use JSON-RPC via nestgate-rpc semantic router"
)]
pub async fn get_snapshot(
    State(state): State<ApiState>,
    Path((dataset_name, _snapshot_name)): Path<(String, String)>,
) -> impl IntoResponse {
    if state.zfs_engines.contains_key(&dataset_name) {
        zfs_snapshot_rest_not_implemented().into_response()
    } else {
        Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        ))
        .into_response()
    }
}

/// Delete a snapshot
/// DELETE /api/v1/zfs/datasets/:dataset/snapshots/:snapshot
#[deprecated(
    since = "0.2.0",
    note = "Use JSON-RPC via nestgate-rpc semantic router"
)]
pub async fn delete_snapshot(
    State(state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
) -> impl IntoResponse {
    info!(
        "Delete snapshot requested for '{}' on dataset {}",
        snapshot_name, dataset_name
    );
    if state.zfs_engines.contains_key(&dataset_name) {
        zfs_snapshot_rest_not_implemented().into_response()
    } else {
        Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        ))
        .into_response()
    }
}

/// Clone a snapshot to create a new dataset
/// POST /api/v1/zfs/datasets/:dataset/snapshots/:snapshot/clone
#[deprecated(
    since = "0.2.0",
    note = "Use JSON-RPC via nestgate-rpc semantic router"
)]
pub async fn clone_snapshot(
    State(_state): State<ApiState>,
    Path((_dataset_name, _snapshot_name)): Path<(String, String)>,
    Json(request): Json<CloneSnapshotRequest>,
) -> impl IntoResponse {
    info!(
        "Clone snapshot requested to new dataset: {}",
        request.clone_name
    );

    if request.clone_name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(DataError::new(
                "Clone name cannot be empty".to_string(),
                "400".to_string(),
            )),
        )
            .into_response();
    }

    zfs_snapshot_rest_not_implemented().into_response()
}
