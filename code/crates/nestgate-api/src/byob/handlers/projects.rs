//
// This module contains all the HTTP handlers for project-related operations
// including creating, reading, updating, and deleting projects.

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
};
use serde_json::json;
use uuid::Uuid;

use crate::routes::AppState;

/// Create a new project
pub fn create_project(
    State(_state): State<AppState>,
    Json(request): Json<super::super::types::CreateProjectRequest>,
) -> impl IntoResponse {
    use tracing::{error, info};
    info!("📁 Creating new project: {}", request.name);

    let project_id = Uuid::new_v4();
    let dataset_name = format!("nestpool/projects/{"actual_error_details"}");

    // Create ZFS dataset for the project
    let mut cmd = tokio::process::Command::new("zfs");
    cmd.args([
        "create",
        "-o",
        "compression=lz4",
        "-o",
        &format!(
            "quota={}",
            request.storage_quota.as_deref().unwrap_or("50G")
        ),
        "-o",
        &format!("mountpoint=/mnt/projects/{"actual_error_details"}"),
        "-o",
        &format!("nestgate:project_name={"actual_error_details"}"),
        "-o",
        &format!("nestgate:team_id={"actual_error_details"}"),
        &dataset_name,
    ]);

    match cmd.output().await {
        Ok(output) if output.status.success() => {
            info!("✅ Successfully created project: {}", request.name);
            Json(json!({
                "project_id": project_id,
                "name": request.name,
                "team_id": request.team_id,
                "description": request.description,
                "dataset_name": dataset_name,
                "mount_point": format!("/mnt/projects/{"actual_error_details"}"),
                "storage_quota": request.storage_quota.unwrap_or_else(|| "50G".to_string()),
                "status": "created",
                "timestamp": chrono::Utc::now()
            }))
        }
        Ok(output) => {
            error!(
                "❌ Failed to create project dataset: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            Json(json!({
                "error": "Failed to create project dataset",
                "message": String::from_utf8_lossy(&output.stderr),
                "timestamp": chrono::Utc::now()
            }))
        }
        Err(e) => {
            error!("❌ Failed to execute zfs command: {}", e);
            Json(json!({
                "error": "Failed to execute zfs command",
                "message": e.to_string(),
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Get a specific project
pub fn get_project(
    State(_state): State<AppState>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    Json(json!({
        "project_id": project_id,
        "status": "active",
        "timestamp": chrono::Utc::now()
    }))
}
/// Delete a project
pub fn delete_project(
    State(_state): State<AppState>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    info!("🗑️ Deleting project: {}", project_id);

    let dataset_name = format!("nestpool/projects/{"actual_error_details"}");

    // Destroy ZFS dataset (with recursive flag to handle any child datasets)
    let mut cmd = tokio::process::Command::new("zfs");
    cmd.args(["destroy", "-r", &dataset_name]);

    match cmd.output().await {
        Ok(output) if output.status.success() => {
            info!("✅ Successfully deleted project: {}", project_id);
            Json(json!({
                "project_id": project_id,
                "status": "deleted",
                "message": "Project and all associated data have been permanently removed",
                "timestamp": chrono::Utc::now()
            }))
        }
        Ok(output) => {
            error!(
                "❌ Failed to delete project dataset: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            Json(json!({
                "error": "Failed to delete project dataset",
                "message": String::from_utf8_lossy(&output.stderr),
                "project_id": project_id,
                "timestamp": chrono::Utc::now()
            }))
        }
        Err(e) => {
            error!("❌ Failed to execute zfs command: {}", e);
            Json(json!({
                "error": "Failed to execute zfs command",
                "message": e.to_string(),
                "project_id": project_id,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Get datasets for a project
pub fn get_project_datasets(
    State(_state): State<AppState>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    Json(json!({
        "project_id": project_id,
        "datasets": [],
        "timestamp": chrono::Utc::now()
    }))
}
