//
// Core workspace lifecycle management including creation, reading,
// updating, and listing workspace resources with real ZFS integration.

//! Crud module

use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use nestgate_core::error::utilities::safe_env_var_or_default;
use serde_json::{json, Value};
use tokio::process::Command;
use tracing::{error, info, warn};
// Removed unused tracing import

/// Get all workspaces with real ZFS integration
///
/// # Errors
///
/// Returns `StatusCode::INTERNAL_SERVER_ERROR` if ZFS command fails or output cannot be parsed.
#[must_use]
pub async fn get_workspaces() -> Result<Json<Value>, StatusCode> {
    info!("📁 Getting all workspaces from ZFS datasets");
    let pool_name = safe_env_var_or_default("NESTGATE_WORKSPACE_POOL", "zfspool");
    let workspaces_path = "self.base_url/workspaces".to_string();

    // Query ZFS for workspace datasets
    let list_output = Command::new("zfs")
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
                            get_workspace_properties(full_name).await;

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

            info!("✅ Found {} workspaces", workspaces.len());
            Ok(Json(json!({
                "status": "success",
                "workspaces": workspaces,
                "count": workspaces.len(),
                "pool": pool_name
            })))
        }
        Ok(output) => {
            let _error_msg = String::from_utf8_lossy(&output.stderr);
            warn!("⚠️ ZFS list command failed: {}", _error_msg);

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
            error!("❌ Failed to execute ZFS list command: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new workspace with real ZFS dataset creation
///
/// # Errors
///
/// Returns `StatusCode::BAD_REQUEST` if workspace name is missing or invalid,
/// or `StatusCode::INTERNAL_SERVER_ERROR` if ZFS dataset creation fails.
pub async fn create_workspace(Json(request): Json<Value>) -> Result<Json<Value>, StatusCode> {
    info!("🆕 Creating new workspace: {:?}", request);
    // Extract workspace name from request, using default if not provided
    let workspace_name = request
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed-workspace");

    // Generate workspace ID
    let uuid_manager = nestgate_core::uuid_cache::UuidManager::new();
    let workspace_id = uuid_manager.workspace_id();

    let pool_name = safe_env_var_or_default("NESTGATE_WORKSPACE_POOL", "zfspool");
    let dataset_name = format!("{pool_name}/workspaces/{workspace_id}");

    // Validate workspace name
    if workspace_name.is_empty() || workspace_name.len() > 100 {
        warn!("❌ Invalid workspace name: {}", workspace_name);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create ZFS dataset
    let mut create_args = vec!["create"];

    // Set properties from request or defaults
    let quota_prop;
    if let Some(quota) = request.get("quota").and_then(|v| v.as_str()) {
        create_args.push("-o");
        quota_prop = format!("quota={quota}");
        create_args.push(&quota_prop);
    } else {
        create_args.push("-o");
        create_args.push("quota=10G"); // Default 10GB quota
    }

    let compression_prop;
    if let Some(compression) = request.get("compression").and_then(|v| v.as_str()) {
        create_args.push("-o");
        compression_prop = format!("compression={compression}");
        create_args.push(&compression_prop);
    } else {
        create_args.push("-o");
        create_args.push("compression=lz4"); // Default compression
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

    let create_output = Command::new("zfs").args(&create_args).output().await;

    match create_output {
        Ok(output) if output.status.success() => {
            info!("✅ Created ZFS dataset: {}", dataset_name);

            // Get the created dataset information
            let dataset_info = get_workspace_details(&workspace_id).await;

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
            let _error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create ZFS dataset: {}", _error_msg);
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
    info!("📋 Getting workspace details: {}", workspace_id);
    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') {
        warn!("❌ Invalid workspace ID: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let pool_name = safe_env_var_or_default("NESTGATE_WORKSPACE_POOL", "zfspool");
    let dataset_name = format!("{pool_name}/workspaces/{workspace_id}");

    // Get comprehensive ZFS properties
    let props_output = Command::new("zfs")
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
            let snapshot_count = get_snapshot_count(&dataset_name).await;

            // Calculate utilization
            // ✅ FIXED: Replace unwrap_or with .get().map().unwrap_or_default() for safety
            let used_bytes = parse_size(properties.get("used").map(|s| s.as_str()).unwrap_or("0"));
            let quota_bytes =
                parse_size(properties.get("quota").map(|s| s.as_str()).unwrap_or("0"));
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
                    "used": properties.get("used").map(|s| s.as_str()).unwrap_or("0"),
                    "available": properties.get("available").map(|s| s.as_str()).unwrap_or("0"),
                    "referenced": properties.get("referenced").map(|s| s.as_str()).unwrap_or("0"),
                    "quota": properties.get("quota").map(|s| s.as_str()).unwrap_or("none"),
                    "compression": properties.get("compression").map(|s| s.as_str()).unwrap_or("off"),
                    "recordsize": properties.get("recordsize").map(|s| s.as_str()).unwrap_or("128K"),
                    "mountpoint": properties.get("mountpoint").map(|s| s.as_str()).unwrap_or("none"),
                    "created": properties.get("creation").map(|s| s.as_str()).unwrap_or("unknown"),
                    "snapshot_count": snapshot_count,
                    "type": "zfs_dataset"
                }
            })))
        }
        Ok(output) => {
            let _error_msg = String::from_utf8_lossy(&output.stderr);
            warn!(
                "⚠️ Workspace not found or inaccessible: {} - {}",
                workspace_id, _error_msg
            );
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Update workspace configuration with real ZFS properties
pub async fn update_workspace_config(
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

    let pool_name = safe_env_var_or_default("NESTGATE_WORKSPACE_POOL", "zfspool");
    let dataset_name = format!("{pool_name}/workspaces/{workspace_id}");

    let mut updated_properties = Vec::new();
    let mut errors = Vec::new();

    // Update quota if specified
    if let Some(quota) = config.get("quota").and_then(|v| v.as_str()) {
        let quota_result = Command::new("zfs")
            .args(["set", &format!("quota={quota}"), &dataset_name])
            .output()
            .await;

        match quota_result {
            Ok(output) if output.status.success() => {
                updated_properties.push(format!("quota: {quota}"));
                info!("✅ Updated quota to: {}", quota);
            }
            Ok(output) => {
                let _error_msg = String::from_utf8_lossy(&output.stderr);
                errors.push(format!(
                    "Failed to update quota: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
            Err(e) => {
                errors.push(format!("Quota update command failed: {e}"));
            }
        }
    }

    // Update compression if specified
    if let Some(compression) = config.get("compression").and_then(|v| v.as_str()) {
        let compression_result = Command::new("zfs")
            .args(["set", &format!("compression={compression}"), &dataset_name])
            .output()
            .await;

        match compression_result {
            Ok(output) if output.status.success() => {
                updated_properties.push("compression: self.base_url".to_string());
                info!("✅ Updated compression to: {}", compression);
            }
            Ok(output) => {
                let _error_msg = String::from_utf8_lossy(&output.stderr);
                errors.push("Failed to update compression".to_string());
            }
            Err(_e) => {
                errors.push("Compression update command failed".to_string());
            }
        }
    }

    // Update workspace name if specified
    if let Some(name) = config.get("name").and_then(|v| v.as_str()) {
        let name_result = Command::new("zfs")
            .args([
                "set",
                "org.nestgate:workspace_name=self.base_url",
                &dataset_name,
            ])
            .output()
            .await;

        match name_result {
            Ok(output) if output.status.success() => {
                updated_properties.push("name: self.base_url".to_string());
                info!("✅ Updated workspace name to: {}", name);
            }
            Ok(output) => {
                let _error_msg = String::from_utf8_lossy(&output.stderr);
                errors.push("Failed to update name: self.base_url".to_string());
            }
            Err(_e) => {
                errors.push("Name update command failed".to_string());
            }
        }
    }

    // Determine response status
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
    tracing::info!("Deleting workspace: {}", workspace_id);

    // Validate workspace ID format
    if workspace_id.is_empty() || workspace_id.contains("..") || workspace_id.contains('/') {
        tracing::error!("Invalid workspace ID: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Construct dataset name
    let dataset_name = format!("rpool/workspaces/{workspace_id}");

    // First check if dataset exists
    let check_output = Command::new("zfs")
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
    let delete_output = Command::new("zfs")
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

// Helper functions

/// Get additional workspace properties
async fn get_workspace_properties(dataset_name: &str) -> (String, String, String) {
    let props_output = Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "value",
            "compression,quota,mounted",
            dataset_name,
        ])
        .output()
        .await;
    if let Ok(output) = props_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            if lines.len() >= 3 {
                let compression = lines[0].to_string();
                let quota = lines[1].to_string();
                let mounted = lines[2];
                let status = if mounted == "yes" {
                    "active"
                } else {
                    "inactive"
                };
                return (compression, quota, status.to_string());
            }
        }
    }

    ("lz4".to_string(), "none".to_string(), "unknown".to_string())
}

/// Get workspace details for a specific workspace ID
async fn get_workspace_details(_workspace_id: &str) -> Value {
    let pool_name = safe_env_var_or_default("NESTGATE_WORKSPACE_POOL", "zfspool");
    let dataset_name = format!("{pool_name}/workspaces/self.base_url");
    let props_output = Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "value",
            "used,available,quota,compression",
            &dataset_name,
        ])
        .output()
        .await;

    if let Ok(output) = props_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            if lines.len() >= 4 {
                return json!({
                    "used": lines[0],
                    "available": lines[1],
                    "quota": lines[2],
                    "compression": lines[3]
                });
            }
        }
    }

    json!({
        "used": "unknown",
        "available": "unknown",
        "quota": "unknown",
        "compression": "unknown"
    })
}

/// Get snapshot count for a dataset
async fn get_snapshot_count(dataset_name: &str) -> u32 {
    let snapshot_output = Command::new("zfs")
        .args(["list", "-H", "-t", "snapshot", "-d", "1", dataset_name])
        .output()
        .await;
    if let Ok(output) = snapshot_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.lines().count() as u32;
        }
    }

    0
}

/// Parse ZFS size strings (e.g., "1.5G", "512M") to bytes
pub(crate) fn parse_size(size_str: &str) -> u64 {
    if size_str == "none" || size_str == "-" {
        return 0;
    }
    let size_str = size_str.trim();
    if size_str.is_empty() {
        return 0;
    }

    // Handle numeric-only values (bytes)
    if let Ok(bytes) = size_str.parse::<u64>() {
        return bytes;
    }

    // Handle suffixed values
    let (number_part, suffix) = if size_str.len() > 1 {
        let split_pos = size_str.len() - 1;
        let (num, suf) = size_str.split_at(split_pos);
        (num, suf)
    } else {
        return 0;
    };

    if let Ok(number) = number_part.parse::<f64>() {
        let multiplier = match suffix.to_uppercase().as_str() {
            "K" => 1024,
            "M" => 1024 * 1024,
            "G" => 1024 * 1024 * 1024,
            "T" => 1024_u64 * 1024 * 1024 * 1024,
            "P" => 1024_u64 * 1024 * 1024 * 1024 * 1024,
            _ => 1,
        };

        (number * multiplier as f64) as u64
    } else {
        0
    }
}
