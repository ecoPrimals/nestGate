// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! List workspaces from ZFS.

use axum::Json;
use axum::http::StatusCode;
use nestgate_types::{EnvSource, ProcessEnv};
use serde_json::{Value, json};
use tokio::process::Command;
use tracing::{error, info, warn};

use super::crud_helpers::{get_workspace_properties, workspace_pool_name, zfs_executable};

/// Get all workspaces with real ZFS integration
///
/// # Errors
///
/// Returns `StatusCode::INTERNAL_SERVER_ERROR` if ZFS command fails or output cannot be parsed.
pub async fn get_workspaces() -> Result<Json<Value>, StatusCode> {
    get_workspaces_from_env_source(&ProcessEnv).await
}

/// Like [`get_workspaces`], but uses an injectable [`EnvSource`] (e.g. [`nestgate_types::MapEnv`] in tests).
pub async fn get_workspaces_from_env_source(
    env: &(impl EnvSource + ?Sized),
) -> Result<Json<Value>, StatusCode> {
    info!("Getting all workspaces from ZFS datasets");
    let zfs_bin = zfs_executable(env);
    let pool_name = workspace_pool_name(env);
    let workspaces_path = format!("{pool_name}/workspaces");

    // Query ZFS for workspace datasets
    let list_output = Command::new(&zfs_bin)
        .args([
            "list",
            "-H",
            "-o",
            "name,used,avail,referenced,mountpoint,creation",
            "-t",
            "filesystem",
            "-d",
            "1",
            &workspaces_path,
        ])
        .output()
        .await;

    match list_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut workspaces = Vec::new();

            for line in stdout.lines() {
                if line.trim().is_empty() {
                    continue;
                }

                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() >= 6 {
                    let full_name = fields[0];
                    let used = fields[1];
                    let available = fields[2];
                    let referenced = fields[3];
                    let mountpoint = fields[4];
                    let creation = fields[5];

                    // Extract workspace ID from dataset name (e.g., "zfspool/workspaces/ws-123" -> "ws-123")
                    if let Some(workspace_id) = full_name.split('/').next_back() {
                        // Skip the parent dataset itself
                        if workspace_id == "workspaces" {
                            continue;
                        }

                        // Get additional properties
                        let (compression, quota, status) =
                            get_workspace_properties(&zfs_bin, full_name).await;

                        workspaces.push(json!({
                            "id": workspace_id,
                            "name": workspace_id.replace(['-', '_'], " "),
                            "dataset_name": full_name,
                            "status": status,
                            "used": used,
                            "available": available,
                            "referenced": referenced,
                            "mountpoint": mountpoint,
                            "compression": compression,
                            "quota": quota,
                            "created": creation,
                            "type": "zfs_dataset"
                        }));
                    }
                }
            }

            info!("Found {} workspaces", workspaces.len());
            Ok(Json(json!({
                "status": "success",
                "workspaces": workspaces,
                "count": workspaces.len(),
                "pool": pool_name
            })))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            warn!("ZFS list command failed: {}", error_msg);

            // Return empty list if workspaces dataset doesn't exist yet
            Ok(Json(json!({
                "status": "success",
                "workspaces": [],
                "count": 0,
                "message": "No workspaces found - workspace pool may not be initialized",
                "pool": pool_name
            })))
        }
        Err(e) => {
            error!("Failed to execute ZFS list command: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
