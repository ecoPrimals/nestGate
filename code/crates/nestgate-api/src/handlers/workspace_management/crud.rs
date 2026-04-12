// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Core workspace lifecycle management including creation, reading,
// updating, and listing workspace resources with real ZFS integration.

//! Crud module

#[path = "crud_helpers.rs"]
mod crud_helpers;
#[path = "crud_list.rs"]
mod crud_list;
#[path = "crud_properties.rs"]
mod crud_properties;

pub(crate) use crud_helpers::parse_size;
pub use crud_list::{get_workspaces, get_workspaces_from_env_source};

use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use nestgate_types::{EnvSource, ProcessEnv};
use serde_json::{Value, json};
use tokio::process::Command;
use tracing::{error, info, warn};

use crud_helpers::{
    get_snapshot_count, get_workspace_details, workspace_pool_name, zfs_executable,
};
use crud_properties::{workspace_apply_compression, workspace_apply_name, workspace_apply_quota};

/// Create a new workspace with real ZFS dataset creation
///
/// # Errors
///
/// Returns `StatusCode::BAD_REQUEST` if workspace name is missing or invalid,
/// or `StatusCode::INTERNAL_SERVER_ERROR` if ZFS dataset creation fails.
pub async fn create_workspace(Json(request): Json<Value>) -> Result<Json<Value>, StatusCode> {
    create_workspace_from_env_source(&ProcessEnv, Json(request)).await
}

/// Like [`create_workspace`], but uses an injectable [`EnvSource`].
pub async fn create_workspace_from_env_source(
    env: &(impl EnvSource + ?Sized),
    Json(request): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!("🆕 Creating new workspace: {:?}", request);
    // Extract workspace name from request, using default if not provided
    let workspace_name = request
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed-workspace");

    // Generate workspace ID
    let uuid_manager = nestgate_core::uuid_cache::UuidManager::new();
    let workspace_id = uuid_manager.workspace_id();

    let zfs_bin = zfs_executable(env);
    let pool_name = workspace_pool_name(env);
    let dataset_name = format!("{pool_name}/workspaces/{workspace_id}");

    // Validate workspace name
    if workspace_name.is_empty() || workspace_name.len() > 100 {
        warn!("❌ Invalid workspace name: {}", workspace_name);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create ZFS dataset
    let mut create_args = vec!["create"];

    let quota_prop;
    create_args.push("-o");
    if let Some(quota) = request.get("quota").and_then(|v| v.as_str()) {
        quota_prop = format!("quota={quota}");
        create_args.push(&quota_prop);
    } else {
        create_args.push("quota=10G");
    }

    let compression_prop;
    create_args.push("-o");
    if let Some(compression) = request.get("compression").and_then(|v| v.as_str()) {
        compression_prop = format!("compression={compression}");
        create_args.push(&compression_prop);
    } else {
        create_args.push("compression=lz4");
    }

    // Set recordsize based on expected workload (default: 128K for mixed workloads)
    let recordsize = request
        .get("recordsize")
        .and_then(|v| v.as_str())
        .unwrap_or("128K");
    create_args.push("-o");
    let recordsize_prop = format!("recordsize={recordsize}");
    create_args.push(&recordsize_prop);

    // Set description
    create_args.push("-o");
    let workspace_name_prop = format!("org.nestgate:workspace_name={workspace_name}");
    create_args.push(&workspace_name_prop);
    create_args.push("-o");
    create_args.push("org.nestgate:created_by=api");

    create_args.push(&dataset_name);

    let create_output = Command::new(&zfs_bin).args(&create_args).output().await;

    match create_output {
        Ok(output) if output.status.success() => {
            info!("✅ Created ZFS dataset: {}", dataset_name);

            // Get the created dataset information
            let dataset_info = get_workspace_details(&zfs_bin, env, &workspace_id).await;

            Ok(Json(json!({
                "status": "success",
                "message": "Workspace created successfully",
                "workspace_id": workspace_id,
                "name": workspace_name,
                "dataset_name": dataset_name,
                "dataset_info": dataset_info
            })))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create ZFS dataset: {}", error_msg);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get workspace details with real ZFS properties
///
/// # Errors
///
/// Returns `StatusCode::BAD_REQUEST` if workspace ID format is invalid,
/// or `StatusCode::INTERNAL_SERVER_ERROR` if ZFS command fails or dataset not found.
pub async fn get_workspace(Path(workspace_id): Path<String>) -> Result<Json<Value>, StatusCode> {
    get_workspace_from_env_source(&ProcessEnv, Path(workspace_id)).await
}

/// Like [`get_workspace`], but uses an injectable [`EnvSource`].
pub async fn get_workspace_from_env_source(
    env: &(impl EnvSource + ?Sized),
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("📋 Getting workspace details: {}", workspace_id);
    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') {
        warn!("❌ Invalid workspace ID: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let zfs_bin = zfs_executable(env);
    let pool_name = workspace_pool_name(env);
    let dataset_name = format!("{pool_name}/workspaces/{workspace_id}");

    // Get comprehensive ZFS properties
    let props_output = Command::new(&zfs_bin)
        .args([
            "get",
            "-H",
            "-o", "property,value",
            "used,available,referenced,quota,compression,recordsize,mountpoint,creation,org.nestgate:workspace_name",
            &dataset_name
        ])
        .output()
        .await;

    match props_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut properties = std::collections::HashMap::new();

            for line in stdout.lines() {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() >= 2 {
                    properties.insert(fields[0].to_string(), fields[1].to_string());
                }
            }

            // Get snapshot count
            let snapshot_count = get_snapshot_count(&zfs_bin, &dataset_name).await;

            // Calculate utilization
            // ✅ FIXED: Replace unwrap_or with .get().map().unwrap_or_default() for safety
            let used_bytes = parse_size(
                properties
                    .get("used")
                    .map_or("0", std::string::String::as_str),
            );
            let quota_bytes = parse_size(
                properties
                    .get("quota")
                    .map_or("0", std::string::String::as_str),
            );
            // Calculate utilization percentage safely (avoid division by zero)
            let utilization = if quota_bytes > 0 {
                (used_bytes as f64 / quota_bytes as f64) * 100.0
            } else {
                0.0
            };

            // Get health status
            let health_status = if utilization > 90.0 {
                "critical"
            } else if utilization > 80.0 {
                "warning"
            } else {
                "healthy"
            };

            // ✅ EVOLVED: Proper error handling with safe fallback
            let workspace_name = properties
                .get("org.nestgate:workspace_name")
                .cloned()
                .unwrap_or_else(|| workspace_id.replace('-', " "));

            Ok(Json(json!({
                "status": "success",
                "workspace": {
                    "id": workspace_id,
                    "name": workspace_name,
                    "dataset_name": dataset_name,
                    "health_status": health_status,
                    "utilization_percent": utilization,
                    // ✅ FIXED: Replace unwrap_or with safe map().unwrap_or() pattern
                    "used": properties.get("used").map_or("0", std::string::String::as_str),
                    "available": properties.get("available").map_or("0", std::string::String::as_str),
                    "referenced": properties.get("referenced").map_or("0", std::string::String::as_str),
                    "quota": properties.get("quota").map_or("none", std::string::String::as_str),
                    "compression": properties.get("compression").map_or("off", std::string::String::as_str),
                    "recordsize": properties.get("recordsize").map_or("128K", std::string::String::as_str),
                    "mountpoint": properties.get("mountpoint").map_or("none", std::string::String::as_str),
                    "created": properties.get("creation").map_or("unknown", std::string::String::as_str),
                    "snapshot_count": snapshot_count,
                    "type": "zfs_dataset"
                }
            })))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            warn!(
                "⚠️ Workspace not found or inaccessible: {} - {}",
                workspace_id, error_msg
            );
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update workspace configuration with real ZFS properties.
///
/// # Errors
///
/// Returns [`StatusCode::BAD_REQUEST`] if the workspace ID is invalid or every property update fails.
/// When some updates succeed, returns HTTP 200 with JSON `status` set to `partial_success` and an `errors` list.
pub async fn update_workspace_config(
    Path(workspace_id): Path<String>,
    Json(config): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    update_workspace_config_from_env_source(&ProcessEnv, Path(workspace_id), Json(config)).await
}

/// Like [`update_workspace_config`], but uses an injectable [`EnvSource`].
pub async fn update_workspace_config_from_env_source(
    env: &(impl EnvSource + ?Sized),
    Path(workspace_id): Path<String>,
    Json(config): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        "⚙️ Updating workspace config: {} -> {:?}",
        workspace_id, config
    );
    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') {
        warn!("❌ Invalid workspace ID: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let zfs_bin = zfs_executable(env);
    let pool_name = workspace_pool_name(env);
    let dataset_name = format!("{pool_name}/workspaces/{workspace_id}");

    let mut updated_properties = Vec::new();
    let mut errors = Vec::new();

    if let Some(quota) = config.get("quota").and_then(|v| v.as_str()) {
        workspace_apply_quota(
            &zfs_bin,
            &dataset_name,
            quota,
            &mut updated_properties,
            &mut errors,
        )
        .await;
    }

    if let Some(compression) = config.get("compression").and_then(|v| v.as_str()) {
        workspace_apply_compression(
            &zfs_bin,
            &dataset_name,
            compression,
            &mut updated_properties,
            &mut errors,
        )
        .await;
    }

    if let Some(name) = config.get("name").and_then(|v| v.as_str()) {
        workspace_apply_name(
            &zfs_bin,
            &dataset_name,
            name,
            &mut updated_properties,
            &mut errors,
        )
        .await;
    }

    if errors.is_empty() {
        info!(
            "✅ Workspace configuration updated successfully: {}",
            workspace_id
        );
        Ok(Json(json!({
            "status": "success",
            "message": "Workspace configuration updated successfully",
            "workspace_id": workspace_id,
            "updated_properties": updated_properties
        })))
    } else if updated_properties.is_empty() {
        warn!("❌ No properties were updated due to errors: {:?}", errors);
        Err(StatusCode::BAD_REQUEST)
    } else {
        warn!("⚠️ Partial update completed with some errors: {:?}", errors);
        Ok(Json(json!({
            "status": "partial_success",
            "message": "Some configuration updates succeeded, others failed",
            "workspace_id": workspace_id,
            "updated_properties": updated_properties,
            "errors": errors
        })))
    }
}

/// **DELETE WORKSPACE**
///
/// Delete an existing workspace by ID.
///
/// # Errors
///
/// Returns `StatusCode::BAD_REQUEST` if workspace ID format is invalid,
/// or `StatusCode::INTERNAL_SERVER_ERROR` if ZFS deletion fails.
pub async fn delete_workspace(Path(workspace_id): Path<String>) -> Result<StatusCode, StatusCode> {
    delete_workspace_from_env_source(&ProcessEnv, Path(workspace_id)).await
}

/// Like [`delete_workspace`], but uses an injectable [`EnvSource`].
pub async fn delete_workspace_from_env_source(
    env: &(impl EnvSource + ?Sized),
    Path(workspace_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Deleting workspace: {}", workspace_id);

    // Validate workspace ID format
    if workspace_id.is_empty() || workspace_id.contains("..") || workspace_id.contains('/') {
        tracing::error!("Invalid workspace ID: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let zfs_bin = zfs_executable(env);

    // Construct dataset name
    let dataset_name = format!("rpool/workspaces/{workspace_id}");

    // First check if dataset exists
    let check_output = Command::new(&zfs_bin)
        .args(["list", "-H", "-o", "name", &dataset_name])
        .output()
        .await;

    match check_output {
        Ok(output) if output.status.success() => {
            // Dataset exists, proceed with deletion
            tracing::info!("Found workspace dataset: {}", dataset_name);
        }
        Ok(_) => {
            tracing::warn!("Workspace dataset not found: {}", dataset_name);
            return Err(StatusCode::NOT_FOUND);
        }
        Err(e) => {
            tracing::error!("Failed to check workspace existence: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Delete the ZFS dataset (recursive to handle any child datasets/snapshots)
    let delete_output = Command::new(&zfs_bin)
        .args(["destroy", "-r", &dataset_name])
        .output()
        .await;

    match delete_output {
        Ok(output) if output.status.success() => {
            tracing::info!("Successfully deleted workspace: {}", workspace_id);
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            tracing::error!("Failed to delete workspace {}: {}", workspace_id, error_msg);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(e) => {
            tracing::error!("Failed to execute zfs destroy command: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
#[path = "crud_inline_tests.rs"]
mod tests;
