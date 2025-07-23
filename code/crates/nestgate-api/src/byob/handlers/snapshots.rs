//! Snapshot Management Handler Functions
//!
//! This module contains all the HTTP handlers for snapshot-related operations
//! including creating, reading, and deleting snapshots.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::OnceLock;
use tracing::{error, info};
use uuid::Uuid;

use super::super::types::{ByobStorageProvider, CreateSnapshotRequest};
use super::super::ZfsStorageProvider;
use crate::routes::AppState;

// Global storage provider instance
static STORAGE_PROVIDER: OnceLock<ZfsStorageProvider> = OnceLock::new();

fn get_storage_provider() -> &'static ZfsStorageProvider {
    STORAGE_PROVIDER.get_or_init(ZfsStorageProvider::new)
}

/// Get all snapshots
pub async fn get_snapshots(State(_state): State<AppState>) -> impl IntoResponse {
    info!("📋 Getting all snapshots");

    // Query ZFS for all snapshots under nestpool
    let mut cmd = tokio::process::Command::new("zfs");
    cmd.args([
        "list",
        "-t",
        "snapshot",
        "-H",
        "-o",
        "name,creation,used,refer",
        "-r",
        "nestpool",
    ]);

    match cmd.output().await {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let snapshots: Vec<serde_json::Value> = stdout
                .lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split('\t').collect();
                    if parts.len() >= 4 {
                        let name_parts: Vec<&str> = parts[0].split('@').collect();
                        if name_parts.len() == 2 {
                            Some(json!({
                                "name": parts[0],
                                "dataset": name_parts[0],
                                "snapshot_name": name_parts[1],
                                "creation_time": parts[1],
                                "used_space": parts[2],
                                "referenced_data": parts[3]
                            }))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            Json(json!({
                "snapshots": snapshots,
                "count": snapshots.len(),
                "timestamp": chrono::Utc::now()
            }))
        }
        Ok(output) => {
            error!(
                "❌ Failed to list snapshots: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            Json(json!({
                "error": "Failed to list snapshots",
                "message": String::from_utf8_lossy(&output.stderr),
                "snapshots": [],
                "count": 0,
                "timestamp": chrono::Utc::now()
            }))
        }
        Err(e) => {
            error!("❌ Failed to execute zfs command: {}", e);
            Json(json!({
                "error": "Failed to execute zfs command",
                "message": e.to_string(),
                "snapshots": [],
                "count": 0,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Get a specific snapshot
pub async fn get_snapshot(
    State(_state): State<AppState>,
    Path(snapshot_id): Path<String>,
) -> impl IntoResponse {
    info!("📖 Getting snapshot details: {}", snapshot_id);

    // For now, this is a placeholder - in a real implementation,
    // we would query the storage provider for snapshot details
    Json(json!({
        "snapshot_id": snapshot_id,
        "status": "active",
        "message": "Snapshot details retrieved",
        "timestamp": chrono::Utc::now()
    }))
}

/// Create a new snapshot
pub async fn create_snapshot(
    State(_state): State<AppState>,
    Path(deployment_id): Path<Uuid>,
    Json(request): Json<CreateSnapshotRequest>,
) -> impl IntoResponse {
    info!(
        "📸 Creating snapshot: {} for deployment: {}",
        request.name, deployment_id
    );

    let provider = get_storage_provider();
    match provider.create_snapshot(&deployment_id, &request).await {
        Ok(snapshot_name) => {
            info!("✅ Successfully created snapshot: {}", snapshot_name);

            (
                StatusCode::CREATED,
                Json(json!({
                    "deployment_id": deployment_id,
                    "snapshot_id": Uuid::new_v4(),
                    "snapshot_name": snapshot_name,
                    "status": "created",
                    "message": "Snapshot created successfully",
                    "timestamp": chrono::Utc::now()
                })),
            )
        }
        Err(e) => {
            error!("❌ Failed to create snapshot: {}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create snapshot",
                    "message": e,
                    "deployment_id": deployment_id,
                    "timestamp": chrono::Utc::now()
                })),
            )
        }
    }
}

/// Restore from a snapshot
pub async fn restore_snapshot(
    State(_state): State<AppState>,
    Path((deployment_id, snapshot_name)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "🔄 Restoring snapshot: {} for deployment: {}",
        snapshot_name, deployment_id
    );

    let dataset_name = format!("nestpool/deployments/{}", deployment_id);
    let full_snapshot_name = format!("{}@{}", dataset_name, snapshot_name);

    // First, check if the snapshot exists
    let mut check_cmd = tokio::process::Command::new("zfs");
    check_cmd.args(["list", "-t", "snapshot", &full_snapshot_name]);

    match check_cmd.output().await {
        Ok(check_output) if check_output.status.success() => {
            // Snapshot exists, proceed with rollback
            let mut rollback_cmd = tokio::process::Command::new("zfs");
            rollback_cmd.args(["rollback", "-r", &full_snapshot_name]);

            match rollback_cmd.output().await {
                Ok(rollback_output) if rollback_output.status.success() => {
                    info!(
                        "✅ Successfully restored snapshot: {} for deployment: {}",
                        snapshot_name, deployment_id
                    );
                    Json(json!({
                        "deployment_id": deployment_id,
                        "snapshot_name": snapshot_name,
                        "dataset_name": dataset_name,
                        "status": "restored",
                        "message": "Snapshot has been successfully restored. All data after the snapshot has been lost.",
                        "timestamp": chrono::Utc::now()
                    }))
                }
                Ok(rollback_output) => {
                    error!(
                        "❌ Failed to restore snapshot: {}",
                        String::from_utf8_lossy(&rollback_output.stderr)
                    );
                    Json(json!({
                        "error": "Failed to restore snapshot",
                        "message": String::from_utf8_lossy(&rollback_output.stderr),
                        "deployment_id": deployment_id,
                        "snapshot_name": snapshot_name,
                        "timestamp": chrono::Utc::now()
                    }))
                }
                Err(e) => {
                    error!("❌ Failed to execute zfs rollback: {}", e);
                    Json(json!({
                        "error": "Failed to execute zfs rollback",
                        "message": e.to_string(),
                        "deployment_id": deployment_id,
                        "snapshot_name": snapshot_name,
                        "timestamp": chrono::Utc::now()
                    }))
                }
            }
        }
        Ok(_) => {
            // Snapshot doesn't exist
            Json(json!({
                "error": "Snapshot not found",
                "message": format!("Snapshot '{}' not found for deployment '{}'", snapshot_name, deployment_id),
                "deployment_id": deployment_id,
                "snapshot_name": snapshot_name,
                "timestamp": chrono::Utc::now()
            }))
        }
        Err(e) => {
            error!("❌ Failed to check snapshot existence: {}", e);
            Json(json!({
                "error": "Failed to check snapshot existence",
                "message": e.to_string(),
                "deployment_id": deployment_id,
                "snapshot_name": snapshot_name,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}
