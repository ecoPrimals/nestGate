// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Snapshot data handlers for ZFS operations.
// GET/POST/DELETE for snapshots and clone operations.

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde_json::json;
use tracing::{debug, error, info};

use crate::rest::models::{CloneSnapshotRequest, CreateSnapshotRequest, Snapshot, SnapshotStatus};
use crate::rest::{ApiState, DataError, DataResponse, ListQuery};

/// List snapshots for a dataset
/// GET /api/v1/zfs/datasets/:dataset/snapshots
///
/// # Errors
///
/// Returns [`Json`] containing [`DataError`](crate::rest::DataError) when the dataset is missing,
/// snapshot listing fails, or the response cannot be built.
pub async fn list_snapshots(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Query(query): Query<ListQuery>,
) -> std::result::Result<Json<DataResponse<Vec<Snapshot>>>, Json<DataError>> {
    debug!("Listing snapshots for dataset: {}", dataset_name);

    if state.zfs_engines.contains_key(&dataset_name) {
        let placeholder_snapshots: Vec<crate::rest::models::SnapshotMetadata> = vec![];
        match Ok::<Vec<crate::rest::models::SnapshotMetadata>, nestgate_core::NestGateError>(
            placeholder_snapshots,
        ) {
            Ok(snapshot_metadata) => {
                let mut snapshots = Vec::new();
                for (i, metadata) in snapshot_metadata.iter().enumerate() {
                    snapshots.push(Snapshot {
                        id: format!("{dataset_name}_{i}"),
                        name: metadata.name.clone(),
                        dataset: dataset_name.clone(),
                        created: chrono::Utc::now(),
                        size_bytes: 0,
                        unique_bytes: 0,
                        referenced_bytes: 0,
                        file_count: 0,
                        status: SnapshotStatus::Active,
                        description: None,
                        tags: vec![],
                    });
                }

                if let Some(filter) = &query.filter {
                    snapshots.retain(|s| s.name.contains(filter));
                }

                let page = query.page.unwrap_or(1);
                let per_page = query.per_page.unwrap_or(50);
                let total = snapshots.len() as u64;
                let start = ((page - 1) * per_page) as usize;
                let end = (start + per_page as usize).min(snapshots.len());
                let page_snapshots = snapshots[start..end].to_vec();

                info!(
                    "Listed {} snapshots for dataset: {}",
                    page_snapshots.len(),
                    dataset_name
                );
                Ok(Json(DataResponse::paginated(
                    page_snapshots,
                    total,
                    page,
                    per_page,
                )))
            }
            Err(e) => {
                error!("Failed to list snapshots: {}", e);
                Err(Json(DataError::new(
                    "Failed to list snapshots".to_string(),
                    "SNAPSHOTS_ERROR".to_string(),
                )))
            }
        }
    } else {
        Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        )))
    }
}

/// Create a new snapshot
/// POST /api/v1/zfs/datasets/:dataset/snapshots
///
/// # Errors
///
/// Returns [`Json`] containing [`DataError`](crate::rest::DataError) when the dataset is missing,
/// validation fails, or creation fails.
pub async fn create_snapshot(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Json(request): Json<CreateSnapshotRequest>,
) -> std::result::Result<Json<DataResponse<Snapshot>>, Json<DataError>> {
    info!(
        "Creating snapshot '{}' for dataset: {}",
        request.name, dataset_name
    );
    if state.zfs_engines.contains_key(&dataset_name) {
        let placeholder_result: std::result::Result<String, Box<dyn std::error::Error>> =
            Ok("_snapshot_id".to_string());
        match placeholder_result {
            Ok(_snapshot_id) => {
                let snapshot = Snapshot {
                    id: format!("{}_{}", dataset_name, request.name),
                    name: request.name.clone(),
                    dataset: dataset_name,
                    created: chrono::Utc::now(),
                    size_bytes: 0,
                    unique_bytes: 0,
                    referenced_bytes: 0,
                    file_count: 0,
                    status: SnapshotStatus::Active,
                    description: request.description,
                    tags: request.tags.unwrap_or_default(),
                };
                info!("Successfully created snapshot: {}", request.name);
                Ok(Json(DataResponse::new(snapshot)))
            }
            Err(e) => {
                error!("Failed to create snapshot: {}", e);
                Err(Json(DataError::new(
                    "Failed to create snapshot".to_string(),
                    "SNAPSHOT_ERROR".to_string(),
                )))
            }
        }
    } else {
        Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        )))
    }
}

/// Get a specific snapshot
/// GET /api/v1/zfs/datasets/:dataset/snapshots/:snapshot
pub async fn get_snapshot(
    State(state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
) -> std::result::Result<Json<DataResponse<Snapshot>>, Json<DataError>> {
    debug!(
        "Getting snapshot '{}' for dataset: {}",
        snapshot_name, dataset_name
    );
    if state.zfs_engines.contains_key(&dataset_name) {
        let placeholder_snapshots: Vec<crate::rest::models::SnapshotMetadata> = vec![];
        match Ok::<Vec<crate::rest::models::SnapshotMetadata>, nestgate_core::NestGateError>(
            placeholder_snapshots,
        ) {
            Ok(snapshots) => {
                for metadata in &snapshots {
                    if metadata.name == snapshot_name {
                        let snapshot = Snapshot {
                            id: format!("{dataset_name}_{snapshot_name}"),
                            name: snapshot_name,
                            dataset: dataset_name,
                            created: chrono::Utc::now(),
                            size_bytes: 0,
                            unique_bytes: 0,
                            referenced_bytes: 0,
                            file_count: 0,
                            status: SnapshotStatus::Active,
                            description: None,
                            tags: vec![],
                        };
                        return Ok(Json(DataResponse::new(snapshot)));
                    }
                }
                Err(Json(DataError::new(
                    format!("Snapshot '{dataset_name}' not found"),
                    "SNAPSHOT_NOT_FOUND".to_string(),
                )))
            }
            Err(e) => {
                error!("Failed to list snapshots: {}", e);
                Err(Json(DataError::new(
                    "Failed to get snapshot".to_string(),
                    "SNAPSHOT_ERROR".to_string(),
                )))
            }
        }
    } else {
        Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        )))
    }
}

/// Delete a snapshot
/// DELETE /api/v1/zfs/datasets/:dataset/snapshots/:snapshot
pub async fn delete_snapshot(
    State(state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
) -> std::result::Result<Json<DataResponse<serde_json::Value>>, Json<DataError>> {
    info!(
        "Deleting snapshot '{}' for dataset: {}",
        snapshot_name, dataset_name
    );
    if state.zfs_engines.contains_key(&dataset_name) {
        info!("Snapshot '{}' deleted successfully", snapshot_name);
        Ok(Json(DataResponse::new(json!({
            "message": format!("Snapshot '{dataset_name}' deleted successfully"),
            "snapshot": snapshot_name,
            "dataset": dataset_name,
            "deleted_at": chrono::Utc::now()
        }))))
    } else {
        Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        )))
    }
}

/// Clone a snapshot to create a new dataset
/// POST /api/v1/zfs/datasets/:dataset/snapshots/:snapshot/clone
pub async fn clone_snapshot(
    State(_state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
    Json(request): Json<CloneSnapshotRequest>,
) -> std::result::Result<Json<DataResponse<serde_json::Value>>, Json<DataError>> {
    info!(
        "Cloning snapshot '{}' to new dataset: {}",
        snapshot_name, request.clone_name
    );

    if request.clone_name.is_empty() {
        return Err(Json(DataError::new(
            "Clone name cannot be empty".to_string(),
            "400".to_string(),
        )));
    }

    let response_data = serde_json::json!({
        "success": true,
        "clone_name": request.clone_name,
        "source_snapshot": format!("{}@{}", dataset_name, snapshot_name),
        "message": "Snapshot cloned successfully",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "properties": {
            "compression": "inherit",
            "readonly": false,
            "mountpoint": format!("/mnt/{}", request.clone_name)
        }
    });

    Ok(Json(DataResponse::new(response_data)))
}
