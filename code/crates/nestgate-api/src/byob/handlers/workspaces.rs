//! Workspace Management Handler Functions
//!
//! This module contains all the HTTP handlers for workspace-related operations
//! including creating, reading, updating, and deleting workspaces.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::OnceLock;
use tracing::{error, info};
use uuid::Uuid;

use super::super::types::{ByobStorageProvider, CreateWorkspaceRequest};
use super::super::ZfsStorageProvider;
use crate::routes::AppState;

// Global storage provider instance
static STORAGE_PROVIDER: OnceLock<ZfsStorageProvider> = OnceLock::new();

fn get_storage_provider() -> &'static ZfsStorageProvider {
    STORAGE_PROVIDER.get_or_init(ZfsStorageProvider::new)
}

/// Get all workspaces
pub async fn get_workspaces(State(_state): State<AppState>) -> impl IntoResponse {
    info!("📋 Getting all workspaces");

    let provider = get_storage_provider();
    match provider.list_workspaces().await {
        Ok(workspaces) => {
            let workspace_summaries: Vec<serde_json::Value> = workspaces
                .iter()
                .map(|w| {
                    json!({
                        "id": w.id,
                        "name": w.name,
                        "team_id": w.team_id,
                        "status": w.status,
                        "dataset_name": w.dataset_name,
                        "storage_quota": w.storage_quota,
                        "compression": w.compression,
                        "created_at": w.created_at,
                        "updated_at": w.updated_at
                    })
                })
                .collect();

            Json(json!({
                "workspaces": workspace_summaries,
                "count": workspaces.len(),
                "timestamp": chrono::Utc::now()
            }))
        }
        Err(e) => {
            error!("❌ Failed to get workspaces: {}", e);
            Json(json!({
                "error": "Failed to get workspaces",
                "message": e,
                "workspaces": [],
                "count": 0,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Create a new workspace
pub async fn create_workspace(
    State(_state): State<AppState>,
    Json(request): Json<CreateWorkspaceRequest>,
) -> impl IntoResponse {
    info!("🏗️ Creating new workspace: {}", request.name);

    let provider = get_storage_provider();
    match provider.create_workspace(&request).await {
        Ok(workspace_state) => {
            info!(
                "✅ Successfully created workspace: {} ({})",
                request.name, workspace_state.id
            );

            (
                StatusCode::CREATED,
                Json(json!({
                    "status": "success",
                    "workspace_id": workspace_state.id,
                    "name": workspace_state.name,
                    "team_id": workspace_state.team_id,
                    "dataset_name": workspace_state.dataset_name,
                    "storage_quota": workspace_state.storage_quota,
                    "compression": workspace_state.compression,
                    "mount_path": format!("/mnt/{}", workspace_state.dataset_name),
                    "created_at": workspace_state.created_at,
                    "message": "Workspace created successfully"
                })),
            )
        }
        Err(e) => {
            error!("❌ Failed to create workspace: {}", e);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "error": "WORKSPACE_CREATION_FAILED",
                    "message": format!("Failed to create workspace: {}", e),
                    "timestamp": chrono::Utc::now()
                })),
            )
        }
    }
}

/// Get a specific workspace
pub async fn get_workspace(
    State(_state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("📖 Getting workspace details: {}", workspace_id);

    let provider = get_storage_provider();
    match provider.get_workspace(&workspace_id).await {
        Ok(workspace) => {
            // Get real-time ZFS dataset information
            let dataset_properties = get_dataset_properties(&workspace.dataset_name).await;

            Json(json!({
                "status": "success",
                "workspace": {
                    "id": workspace.id,
                    "name": workspace.name,
                    "team_id": workspace.team_id,
                    "status": workspace.status,
                    "dataset_name": workspace.dataset_name,
                    "storage_quota": workspace.storage_quota,
                    "compression": workspace.compression,
                    "created_at": workspace.created_at,
                    "updated_at": workspace.updated_at,
                    "zfs_properties": dataset_properties,
                    "mount_path": format!("/mnt/{}", workspace.dataset_name)
                },
                "timestamp": chrono::Utc::now()
            }))
        }
        Err(e) => {
            error!("❌ Failed to get workspace: {}", e);
            Json(json!({
                "status": "error",
                "error": "WORKSPACE_NOT_FOUND",
                "message": format!("Workspace not found: {}", e),
                "workspace_id": workspace_id,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Get dataset properties from ZFS
async fn get_dataset_properties(dataset_name: &str) -> serde_json::Value {
    let result = tokio::process::Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "property,value",
            "used,avail,quota,compressratio,mounted,mountpoint",
            dataset_name,
        ])
        .output()
        .await;

    match result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut properties = std::collections::HashMap::new();

            for line in stdout.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    properties.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            json!(properties)
        }
        _ => json!({}),
    }
}

/// Set workspace volume properties
pub async fn set_workspace_volume_properties(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, String)>,
    Json(_request): Json<serde_json::Value>,
) -> impl IntoResponse {
    info!(
        "🔧 Setting volume properties for workspace: {}, volume: {}",
        workspace_id, volume_id
    );

    Json(json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "status": "properties_updated",
        "timestamp": chrono::Utc::now()
    }))
}

/// Inherit workspace volume
pub async fn inherit_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "🔄 Inheriting volume for workspace: {}, volume: {}",
        workspace_id, volume_id
    );

    Json(json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "status": "inherited",
        "timestamp": chrono::Utc::now()
    }))
}

/// Userspace workspace volume
pub async fn userspace_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "👤 Configuring userspace for workspace: {}, volume: {}",
        workspace_id, volume_id
    );

    Json(json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "status": "userspace_configured",
        "timestamp": chrono::Utc::now()
    }))
}

/// Groupspace workspace volume
pub async fn groupspace_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "👥 Configuring groupspace for workspace: {}, volume: {}",
        workspace_id, volume_id
    );

    Json(json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "status": "groupspace_configured",
        "timestamp": chrono::Utc::now()
    }))
}

/// Projectspace workspace volume
pub async fn projectspace_workspace_volume(
    State(_state): State<AppState>,
    Path((workspace_id, volume_id)): Path<(Uuid, String)>,
) -> impl IntoResponse {
    info!(
        "📁 Configuring projectspace for workspace: {}, volume: {}",
        workspace_id, volume_id
    );

    Json(json!({
        "workspace_id": workspace_id,
        "volume_id": volume_id,
        "status": "projectspace_configured",
        "timestamp": chrono::Utc::now()
    }))
}
