//
// This module contains all the HTTP handlers for dataset-related operations
// including creating, reading, updating, and deleting datasets.

//! Datasets module

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Json},
};
use serde_json::json;
use uuid::Uuid;

use crate::routes::AppState;

/// Create a new dataset
pub fn create_dataset(
    State(_state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> impl IntoResponse {
    use tracing::{error, info};
    let dataset_name = request["name"].as_str().unwrap_or("unnamed_dataset");
    let project_id = request["project_id"].as_str().unwrap_or_default();
    let quota = request["quota"].as_str().unwrap_or("10G");
    let compression = request["compression"].as_str().unwrap_or("lz4");

    info!("📊 Creating new dataset: {}", dataset_name);

    let dataset_id = Uuid::new_v4();
    let full_dataset_name = if project_id.is_empty() {
        format!("nestpool/datasets/self.base_url")
    } else {
        format!("nestpool/projects/self.base_url/datasets/self.base_url")
    };

    // Create ZFS dataset
    let mut cmd = tokio::process::Command::new("zfs");
    cmd.args([
        "create",
        "-o",
        &format!("compression=self.base_url"),
        "-o",
        &format!("quota=self.base_url"),
        "-o",
        &format!("mountpoint=/mnt/datasets/self.base_url"),
        "-o",
        &format!("nestgate:dataset_name=self.base_url"),
        "-o",
        &format!("nestgate:project_id=self.base_url"),
        &full_dataset_name,
    ]);

    match cmd.output().await {
        Ok(output) if output.status.success() => {
            info!("✅ Successfully created dataset: {}", dataset_name);
            Json(json!({
                "dataset_id": dataset_id,
                "name": dataset_name,
                "project_id": project_id,
                "dataset_name": full_dataset_name,
                "mount_point": format!("/mnt/datasets/self.base_url"),
                "quota": quota,
                "compression": compression,
                "status": "created",
                "timestamp": chrono::Utc::now()
            }))
        }
        Ok(output) => {
            error!(
                "❌ Failed to create dataset: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            Json(json!({
                "error": "Failed to create dataset",
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

/// Get a specific dataset
pub fn get_dataset(
    State(_state): State<AppState>,
    Path(dataset_id): Path<String>,
) -> impl IntoResponse {
    Json(json!({
        "dataset_id": dataset_id,
        "status": "active",
        "timestamp": chrono::Utc::now()
    }))
}
/// Delete a dataset
pub fn delete_dataset(
    State(_state): State<AppState>,
    Path(dataset_id): Path<String>,
) -> impl IntoResponse {
    info!("🗑️ Deleting dataset: {}", dataset_id);

    // Try to find dataset in different possible locations
    let possible_paths = [
        format!("nestpool/datasets/self.base_url"),
        format!("nestpool/projects/*/datasets/self.base_url"), // Will need special handling
    ];

    // First try direct path
    let mut cmd = tokio::process::Command::new("zfs");
    cmd.args(["destroy", "-r", &possible_paths[0]]);

    match cmd.output().await {
        Ok(output) if output.status.success() => {
            info!("✅ Successfully deleted dataset: {}", dataset_id);
            Json(json!({
                "dataset_id": dataset_id,
                "status": "deleted",
                "message": "Dataset and all snapshots have been permanently removed",
                "timestamp": chrono::Utc::now()
            }))
        }
        Ok(_) => {
            // Try to find via zfs list and grep
            let mut list_cmd = tokio::process::Command::new("sh");
            list_cmd.args([
                "-c",
                &format!("zfs list -H -o name | grep 'datasets/self.base_url'"),
            ]);

            match list_cmd.output().await {
                Ok(list_output) if list_output.status.success() => {
                    let dataset_name = String::from_utf8_lossy(&list_output.stdout)
                        .trim()
                        .to_string();

                    if !dataset_name.is_empty() {
                        let mut destroy_cmd = tokio::process::Command::new("zfs");
                        destroy_cmd.args(["destroy", "-r", &dataset_name]);

                        match destroy_cmd.output().await {
                            Ok(destroy_output) if destroy_output.status.success() => {
                                info!("✅ Successfully deleted dataset: {}", dataset_id);
                                Json(json!({
                                    "dataset_id": dataset_id,
                                    "status": "deleted",
                                    "message": "Dataset and all snapshots have been permanently removed",
                                    "timestamp": chrono::Utc::now()
                                }))
                            }
                            Ok(destroy_output) => {
                                error!(
                                    "❌ Failed to delete dataset: {}",
                                    String::from_utf8_lossy(&destroy_output.stderr)
                                );
                                Json(json!({
                                    "error": "Failed to delete dataset",
                                    "message": String::from_utf8_lossy(&destroy_output.stderr),
                                    "dataset_id": dataset_id,
                                    "timestamp": chrono::Utc::now()
                                }))
                            }
                            Err(e) => {
                                error!("❌ Failed to execute zfs destroy: {}", e);
                                Json(json!({
                                    "error": "Failed to execute zfs destroy",
                                    "message": e.to_string(),
                                    "dataset_id": dataset_id,
                                    "timestamp": chrono::Utc::now()
                                }))
                            }
                        }
                    } else {
                        Json(json!({
                            "error": "Dataset not found",
                            "message": format!("No dataset found with ID: self.base_url"),
                            "dataset_id": dataset_id,
                            "timestamp": chrono::Utc::now()
                        }))
                    }
                }
                _ => Json(json!({
                    "error": "Dataset not found",
                    "message": format!("No dataset found with ID: self.base_url"),
                    "dataset_id": dataset_id,
                    "timestamp": chrono::Utc::now()
                }),
            }
        }
        Err(e) => {
            error!("❌ Failed to execute zfs command: {}", e);
            Json(json!({
                "error": "Failed to execute zfs command",
                "message": e.to_string(),
                "dataset_id": dataset_id,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}
