// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use axum::{extract::Json, extract::Path, http::StatusCode};
use serde_json::{Value, json};
use tokio::process::Command;
use tracing::{error, info, warn};

use nestgate_core::error::utilities::safe_env_var_or_default;

use super::types::RestoreConfig;

/// Restore workspace from backup
pub async fn restore_workspace(
    Path(workspace_id): Path<String>,
    Json(config): Json<RestoreConfig>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        "Restoring workspace: {} from backup: {:?}",
        workspace_id, config
    );

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let target_workspace = config.target_workspace_id.as_ref().unwrap_or(&workspace_id);
    let dataset_name = format!("nestpool/workspaces/{target_workspace}");
    let backup_dir = safe_env_var_or_default("NESTGATE_BACKUP_DIR", "/var/backups/nestgate");
    let backup_file = format!(
        "{}/workspace_{}_{}.zfs",
        backup_dir, workspace_id, config.backup_name
    );

    // Step 1: Check if backup file exists
    if !tokio::fs::try_exists(&backup_file).await.unwrap_or(false) {
        error!("Backup file not found: {}", backup_file);
        return Ok(Json(json!({
            "status": "error",
            "message": format!("Backup file not found: {backup_file}"),
            "workspace_id": workspace_id,
            "backup_name": config.backup_name
        })));
    }

    // Step 2: Check if target workspace exists (unless force is enabled)
    if !config.force {
        let check_result = Command::new("zfs")
            .args(["list", "-H", "-o", "name", &dataset_name])
            .output()
            .await;

        if let Ok(output) = check_result
            && output.status.success()
        {
            warn!("Target workspace already exists: {}", dataset_name);
            return Ok(Json(json!({
                "status": "error",
                "message": format!("Target workspace already exists. Use force=true to overwrite."),
                "workspace_id": target_workspace
            })));
        }
    }

    // Step 3: If force is enabled and target exists, destroy it first
    if config.force {
        let destroy_result = Command::new("zfs")
            .args(["destroy", "-r", &dataset_name])
            .output()
            .await;

        if let Ok(output) = destroy_result
            && output.status.success()
        {
            info!("Destroyed existing workspace: {}", dataset_name);
        }
    }

    // Step 4: Restore from backup using ZFS receive
    info!("Restoring from backup file: {}", backup_file);

    let backup_file_handle = tokio::fs::File::open(&backup_file).await;
    match backup_file_handle {
        Ok(file) => {
            let receive_result = Command::new("zfs")
                .args(["receive", "-F", &dataset_name])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn();

            match receive_result {
                Ok(mut receive_process) => {
                    if let Some(mut stdin) = receive_process.stdin.take() {
                        let mut reader = tokio::io::BufReader::new(file);

                        match tokio::io::copy(&mut reader, &mut stdin).await {
                            Ok(bytes_read) => {
                                drop(stdin); // Close stdin to signal end of data

                                match receive_process.wait().await {
                                    Ok(status) if status.success() => {
                                        info!(
                                            "Workspace restored successfully: {} ({} bytes)",
                                            dataset_name, bytes_read
                                        );

                                        // Get workspace info after restore
                                        let info_result = get_workspace_info(&dataset_name).await;

                                        return Ok(Json(json!({
                                            "status": "success",
                                            "message": "Workspace restored successfully",
                                            "workspace_id": target_workspace,
                                            "backup_name": config.backup_name,
                                            "backup_file": backup_file,
                                            "restored_bytes": bytes_read,
                                            "dataset_name": dataset_name,
                                            "workspace_info": info_result.unwrap_or_default()
                                        })));
                                    }
                                    Ok(status) => {
                                        error!("ZFS receive failed with status: {}", status);
                                    }
                                    Err(e) => {
                                        error!("ZFS receive process error: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to pipe backup data: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to start ZFS receive process: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to open backup file: {}", e);
        }
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

/// Gets Workspace Info
async fn get_workspace_info(dataset_name: &str) -> Result<Value, ()> {
    let info_result = Command::new("zfs")
        .args([
            "list",
            "-H",
            "-o",
            "name,used,avail,refer,mountpoint",
            dataset_name,
        ])
        .output()
        .await;

    match info_result {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = output_str.trim().split('\t').collect();

            if parts.len() >= 5 {
                Ok(json!({
                    "name": parts[0],
                    "used": parts[1],
                    "available": parts[2],
                    "referenced": parts[3],
                    "mountpoint": parts[4]
                }))
            } else {
                Err(())
            }
        }
        _ => Err(()),
    }
}
