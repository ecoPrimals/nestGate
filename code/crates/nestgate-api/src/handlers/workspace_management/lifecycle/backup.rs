// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use axum::{extract::Json, extract::Path, http::StatusCode};
use serde_json::{Value, json};
use tokio::process::Command;
use tracing::{debug, error, info, warn};

use super::types::BackupConfig;

/// Backup workspace with ZFS snapshots
#[allow(clippy::too_many_lines)]
pub async fn backup_workspace(
    Path(workspace_id): Path<String>,
    Json(config): Json<BackupConfig>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        "💾 Creating backup for workspace: {} with config: {:?}",
        workspace_id, config
    );

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("❌ Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let dataset_name = format!("nestpool/workspaces/{workspace_id}");
    let snapshot_name = format!("{}@backup_{}", dataset_name, config.backup_name);
    use nestgate_core::error::utilities::safe_env_var_or_default;
    let backup_dir = safe_env_var_or_default("NESTGATE_BACKUP_DIR", "/var/backups/nestgate");
    let backup_file = format!(
        "{}/workspace_{}_{}.zfs",
        backup_dir, workspace_id, config.backup_name
    );

    // Step 1: Create snapshot
    info!("📸 Creating snapshot: {}", snapshot_name);
    let snapshot_result = Command::new("zfs")
        .args(["snapshot", &snapshot_name])
        .output()
        .await;

    match snapshot_result {
        Ok(output) if output.status.success() => {
            info!("✅ Snapshot created successfully: {}", snapshot_name);
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create snapshot: {}", stderr);
            return Ok(Json(json!({
                "status": "error",
                "message": format!("Failed to create snapshot: {stderr}"),
                "workspace_id": workspace_id,
                "backup_name": config.backup_name
            })));
        }
        Err(e) => {
            error!("❌ Failed to execute snapshot command: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Step 2: Create backup directory if it doesn't exist
    if let Err(e) = tokio::fs::create_dir_all(&backup_dir).await {
        error!("❌ Failed to create backup directory: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Step 3: Send snapshot to backup file
    info!("💾 Sending snapshot to backup file: {}", backup_file);
    let mut send_args = vec!["send"];

    if config.compression_level > 0 {
        send_args.extend(["-c", "-L"]);
    }

    send_args.push(&snapshot_name);

    let send_result = Command::new("zfs")
        .args(&send_args)
        .stdout(std::process::Stdio::piped())
        .spawn();

    match send_result {
        Ok(mut send_process) => {
            // Pipe the output to a file
            let backup_file_handle = tokio::fs::File::create(&backup_file).await;
            match backup_file_handle {
                Ok(mut file) => {
                    if let Some(stdout) = send_process.stdout.take() {
                        let mut reader = tokio::io::BufReader::new(stdout);
                        match tokio::io::copy(&mut reader, &mut file).await {
                            Ok(bytes_written) => {
                                info!(
                                    "✅ Backup completed: {} bytes written to {}",
                                    bytes_written, backup_file
                                );

                                // Wait for the process to complete
                                match send_process.wait().await {
                                    Ok(status) if status.success() => {
                                        // Optionally remove the snapshot after successful backup
                                        if !config.include_snapshots {
                                            let _ = Command::new("zfs")
                                                .args(["destroy", &snapshot_name])
                                                .output()
                                                .await;
                                            debug!(
                                                "🧹 Cleaned up temporary snapshot: {}",
                                                snapshot_name
                                            );
                                        }

                                        return Ok(Json(json!({
                                            "status": "success",
                                            "message": "Workspace backup completed successfully",
                                            "workspace_id": workspace_id,
                                            "backup_name": config.backup_name,
                                            "backup_file": backup_file,
                                            "backup_size_bytes": bytes_written,
                                            "snapshot_name": snapshot_name,
                                            "compression_enabled": config.compression_level > 0
                                        })));
                                    }
                                    Ok(_) | Err(_) => {
                                        error!("❌ ZFS send process failed");
                                    }
                                }
                            }
                            Err(e) => {
                                error!("❌ Failed to write backup data: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("❌ Failed to create backup file: {}", e);
                }
            }
        }
        Err(e) => {
            error!("❌ Failed to start ZFS send process: {}", e);
        }
    }

    // Cleanup on failure
    let _ = Command::new("zfs")
        .args(["destroy", &snapshot_name])
        .output()
        .await;

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
