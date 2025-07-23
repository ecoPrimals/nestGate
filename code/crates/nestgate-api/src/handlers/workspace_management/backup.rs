//! Backup and Recovery Operations
//!
//! ZFS snapshot-based backup and restore functionality for workspace data protection.

use axum::{extract::Json, extract::Path, http::StatusCode};
use serde_json::{json, Value};
use tokio::process::Command;
use tracing::error;
use tracing::info;
use tracing::warn;
// Removed unused tracing import

/// Create workspace backup (CORE STORAGE FUNCTION)
pub async fn create_workspace_backup(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("💾 Creating workspace backup: {}", workspace_id);

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("❌ Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let dataset_name = format!("nestpool/workspaces/{workspace_id}");
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let snapshot_name = format!("{dataset_name}@backup_{timestamp}");

    // Create ZFS snapshot
    let snapshot_output = Command::new("zfs")
        .args(["snapshot", &snapshot_name])
        .output()
        .await;

    match snapshot_output {
        Ok(output) if output.status.success() => {
            info!("✅ Created backup snapshot: {}", snapshot_name);

            // Get snapshot size
            let size_output = Command::new("zfs")
                .args(["get", "-H", "-p", "-o", "value", "used", &snapshot_name])
                .output()
                .await;

            let backup_size = if let Ok(size_result) = size_output {
                if size_result.status.success() {
                    String::from_utf8_lossy(&size_result.stdout)
                        .trim()
                        .parse::<u64>()
                        .unwrap_or(0)
                } else {
                    0
                }
            } else {
                0
            };

            Ok(Json(json!({
                "status": "success",
                "message": "Workspace backup created successfully",
                "workspace_id": workspace_id,
                "backup_id": format!("backup_{}", timestamp),
                "snapshot_name": snapshot_name,
                "backup_size_bytes": backup_size,
                "created_at": timestamp
            })))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create ZFS snapshot: {}", error_msg);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Restore workspace from backup (CORE STORAGE FUNCTION)
pub async fn restore_workspace(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("🔄 Restoring workspace from backup: {}", workspace_id);

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("❌ Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let dataset_name = format!("nestpool/workspaces/{workspace_id}");

    // Find the most recent backup snapshot
    let snapshot_output = Command::new("zfs")
        .args([
            "list",
            "-H",
            "-t",
            "snapshot",
            "-o",
            "name",
            "-s",
            "creation",
            "-d",
            "1",
            &dataset_name,
        ])
        .output()
        .await;

    let snapshot_name = match snapshot_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let snapshots: Vec<&str> = stdout
                .lines()
                .filter(|line| line.contains("@backup_"))
                .collect();

            if let Some(latest_snapshot) = snapshots.last() {
                latest_snapshot.to_string()
            } else {
                warn!(
                    "⚠️ No backup snapshots found for workspace: {}",
                    workspace_id
                );
                return Ok(Json(json!({
                    "status": "error",
                    "message": "No backup snapshots found for workspace",
                    "workspace_id": workspace_id
                })));
            }
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to list snapshots: {}", error_msg);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Rollback to the snapshot
    let rollback_output = Command::new("zfs")
        .args(["rollback", "-r", &snapshot_name])
        .output()
        .await;

    match rollback_output {
        Ok(output) if output.status.success() => {
            info!(
                "✅ Successfully restored workspace from backup: {}",
                workspace_id
            );
            Ok(Json(json!({
                "status": "success",
                "message": "Workspace restored successfully",
                "workspace_id": workspace_id,
                "restored_from": snapshot_name
            })))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to rollback ZFS dataset: {}", error_msg);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
