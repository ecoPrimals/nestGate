// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! REST snapshot handlers — wired to [`ZfsOperations`] CLI.
//!
//! These handlers delegate to the system `zfs` CLI via [`ZfsOperations`] from
//! `nestgate-zfs`. If ZFS tools are not available on the host the handlers
//! return `503 Service Unavailable`.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use nestgate_zfs::command::{ZfsCommand, ZfsOperations};
use nestgate_zfs::native::is_zfs_available;
use serde_json::json;
use tracing::info;

use crate::rest::models::{CloneSnapshotRequest, CreateSnapshotRequest};
use crate::rest::{ApiState, DataError, ListQuery};

/// List snapshots for a dataset.
/// GET /api/v1/zfs/datasets/:dataset/snapshots
pub async fn list_snapshots(
    State(_state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Query(_query): Query<ListQuery>,
) -> impl IntoResponse {
    if !is_zfs_available().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "zfs_unavailable", "message": "zfs CLI not found on this host"})),
        )
            .into_response();
    }
    let ops = ZfsOperations::new();
    match ops.list_snapshots(Some(&dataset_name)).await {
        Ok(snapshots) => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "dataset": dataset_name,
                "snapshots": snapshots,
                "count": snapshots.len(),
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "zfs_error", "message": e.to_string()})),
        )
            .into_response(),
    }
}

/// Create a new snapshot.
/// POST /api/v1/zfs/datasets/:dataset/snapshots
pub async fn create_snapshot(
    State(_state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Json(request): Json<CreateSnapshotRequest>,
) -> impl IntoResponse {
    info!(
        dataset = %dataset_name,
        snapshot = %request.name,
        "Create snapshot requested"
    );
    if !is_zfs_available().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "zfs_unavailable", "message": "zfs CLI not found on this host"})),
        )
            .into_response();
    }
    let ops = ZfsOperations::new();
    match ops.create_snapshot(&dataset_name, &request.name).await {
        Ok(()) => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "dataset": dataset_name,
                "snapshot": request.name,
                "created": true,
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "zfs_error", "message": e.to_string()})),
        )
            .into_response(),
    }
}

/// Get a specific snapshot (metadata via `zfs list -t snapshot`).
/// GET /api/v1/zfs/datasets/:dataset/snapshots/:snapshot
pub async fn get_snapshot(
    State(_state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
) -> impl IntoResponse {
    if !is_zfs_available().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "zfs_unavailable", "message": "zfs CLI not found on this host"})),
        )
            .into_response();
    }
    let full_name = format!("{dataset_name}@{snapshot_name}");
    let ops = ZfsOperations::new();
    match ops.list_snapshots(Some(&dataset_name)).await {
        Ok(snapshots) => {
            let found = snapshots.iter().find(|s| s.name == full_name);
            if let Some(snap) = found {
                (StatusCode::OK, Json(json!({"status": "success", "snapshot": snap})))
                    .into_response()
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "not_found", "snapshot": full_name})),
                )
                    .into_response()
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "zfs_error", "message": e.to_string()})),
        )
            .into_response(),
    }
}

/// Delete a snapshot.
/// DELETE /api/v1/zfs/datasets/:dataset/snapshots/:snapshot
pub async fn delete_snapshot(
    State(_state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
) -> impl IntoResponse {
    info!(
        dataset = %dataset_name,
        snapshot = %snapshot_name,
        "Delete snapshot requested"
    );
    if !is_zfs_available().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "zfs_unavailable", "message": "zfs CLI not found on this host"})),
        )
            .into_response();
    }
    let full_name = format!("{dataset_name}@{snapshot_name}");
    let cmd = ZfsCommand::new();
    match cmd.zfs(&["destroy", &full_name]).await {
        Ok(result) if result.is_success() => (
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "snapshot": full_name,
                "destroyed": true,
            })),
        )
            .into_response(),
        Ok(result) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "zfs_error", "message": result.stderr})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "zfs_error", "message": e.to_string()})),
        )
            .into_response(),
    }
}

/// Clone a snapshot to create a new dataset.
/// POST /api/v1/zfs/datasets/:dataset/snapshots/:snapshot/clone
pub async fn clone_snapshot(
    State(_state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
    Json(request): Json<CloneSnapshotRequest>,
) -> impl IntoResponse {
    info!(
        dataset = %dataset_name,
        snapshot = %snapshot_name,
        clone_name = %request.clone_name,
        "Clone snapshot requested"
    );
    if request.clone_name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(DataError::new(
                "Clone name cannot be empty".into(),
                "400".into(),
            )),
        )
            .into_response();
    }
    if !is_zfs_available().await {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "zfs_unavailable", "message": "zfs CLI not found on this host"})),
        )
            .into_response();
    }
    let full_snap = format!("{dataset_name}@{snapshot_name}");
    let cmd = ZfsCommand::new();
    match cmd.zfs(&["clone", &full_snap, &request.clone_name]).await {
        Ok(result) if result.is_success() => (
            StatusCode::CREATED,
            Json(json!({
                "status": "success",
                "source_snapshot": full_snap,
                "clone_dataset": request.clone_name,
                "created": true,
            })),
        )
            .into_response(),
        Ok(result) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "zfs_error", "message": result.stderr})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "zfs_error", "message": e.to_string()})),
        )
            .into_response(),
    }
}
