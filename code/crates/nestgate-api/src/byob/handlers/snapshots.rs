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

    // For now, this is a placeholder - in a real implementation,
    // we would query the storage provider for snapshots
    Json(json!({
        "snapshots": [],
        "count": 0,
        "timestamp": chrono::Utc::now()
    }))
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

    // For now, this is a placeholder - in a real implementation,
    // we would implement snapshot restoration logic
    Json(json!({
        "deployment_id": deployment_id,
        "snapshot_name": snapshot_name,
        "status": "restored",
        "message": "Snapshot restoration requested",
        "timestamp": chrono::Utc::now()
    }))
}
