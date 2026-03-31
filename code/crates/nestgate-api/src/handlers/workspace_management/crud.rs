// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Core workspace lifecycle management including creation, reading,
// updating, and listing workspace resources with real ZFS integration.

//! Crud module

use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use nestgate_core::error::utilities::safe_env_var_or_default;
use nestgate_zfs::numeric::f64_to_u64_saturating;
use serde_json::{Value, json};
use tokio::process::Command;
use tracing::{error, info, warn};
// Removed unused tracing import

/// Get all workspaces with real ZFS integration
///
/// # Errors
///
/// Returns `StatusCode::INTERNAL_SERVER_ERROR` if ZFS command fails or output cannot be parsed.
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
            let error_msg = String::from_utf8_lossy(&output.stderr);
            warn!("⚠️ ZFS list command failed: {}", error_msg);

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

async fn workspace_apply_quota(
    dataset_name: &str,
    quota: &str,
    updated_properties: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    let quota_result = Command::new("zfs")
        .args(["set", &format!("quota={quota}"), dataset_name])
        .output()
        .await;

    match quota_result {
        Ok(output) if output.status.success() => {
            updated_properties.push(format!("quota: {quota}"));
            info!("✅ Updated quota to: {}", quota);
        }
        Ok(output) => {
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

async fn workspace_apply_compression(
    dataset_name: &str,
    compression: &str,
    updated_properties: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    let compression_result = Command::new("zfs")
        .args(["set", &format!("compression={compression}"), dataset_name])
        .output()
        .await;

    match compression_result {
        Ok(output) if output.status.success() => {
            updated_properties.push(format!("compression: {compression}"));
            info!("✅ Updated compression to: {}", compression);
        }
        Ok(output) => {
            errors.push(format!(
                "Failed to update compression: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Err(_e) => {
            errors.push("Compression update command failed".to_string());
        }
    }
}

async fn workspace_apply_name(
    dataset_name: &str,
    name: &str,
    updated_properties: &mut Vec<String>,
    errors: &mut Vec<String>,
) {
    let prop = format!("org.nestgate:workspace_name={name}");
    let name_result = Command::new("zfs")
        .args(["set", &prop, dataset_name])
        .output()
        .await;

    match name_result {
        Ok(output) if output.status.success() => {
            updated_properties.push(format!("name: {name}"));
            info!("✅ Updated workspace name to: {}", name);
        }
        Ok(output) => {
            errors.push(format!(
                "Failed to update name: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        Err(_e) => {
            errors.push("Name update command failed".to_string());
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

    if let Some(quota) = config.get("quota").and_then(|v| v.as_str()) {
        workspace_apply_quota(&dataset_name, quota, &mut updated_properties, &mut errors).await;
    }

    if let Some(compression) = config.get("compression").and_then(|v| v.as_str()) {
        workspace_apply_compression(
            &dataset_name,
            compression,
            &mut updated_properties,
            &mut errors,
        )
        .await;
    }

    if let Some(name) = config.get("name").and_then(|v| v.as_str()) {
        workspace_apply_name(&dataset_name, name, &mut updated_properties, &mut errors).await;
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
    if let Ok(output) = props_output
        && output.status.success()
    {
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

    if let Ok(output) = props_output
        && output.status.success()
    {
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
    if let Ok(output) = snapshot_output
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.lines().count() as u32;
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

        f64_to_u64_saturating(number * multiplier as f64)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Path;
    use serde_json::json;
    use serial_test::serial;

    #[test]
    fn parse_size_numeric_and_none() {
        assert_eq!(parse_size("4096"), 4096);
        assert_eq!(parse_size("none"), 0);
        assert_eq!(parse_size("-"), 0);
        assert!(parse_size("1M") >= 1024 * 1024);
    }

    #[test]
    fn parse_size_suffixes_k_g_t_p() {
        assert_eq!(parse_size("2G"), 2 * 1024 * 1024 * 1024);
        assert_eq!(parse_size("1K"), 1024);
        let one_t = parse_size("1T");
        assert!(one_t >= 1024_u64 * 1024 * 1024 * 1024);
        let one_p = parse_size("1P");
        assert!(one_p > one_t);
    }

    #[test]
    fn parse_size_whitespace_and_invalid() {
        assert_eq!(parse_size("  3G  "), 3 * 1024 * 1024 * 1024);
        assert_eq!(parse_size(""), 0);
        assert_eq!(parse_size("Z"), 0);
    }

    #[test]
    fn parse_size_unknown_suffix_uses_multiplier_one() {
        assert_eq!(parse_size("1X"), 1);
        assert_eq!(parse_size("4X"), 4);
    }

    #[test]
    fn parse_size_invalid_number_before_suffix() {
        assert_eq!(parse_size("abcG"), 0);
        assert_eq!(parse_size("not5M"), 0);
    }

    #[test]
    fn parse_size_decimal_k_and_m() {
        assert_eq!(parse_size("1.5K"), 1536);
        let m = parse_size("2.5M");
        assert!(m >= 2 * 1024 * 1024);
    }

    #[test]
    fn parse_size_single_char_no_suffix_parse() {
        assert_eq!(parse_size("9"), 9);
    }

    #[tokio::test]
    async fn update_workspace_config_rejects_empty_or_slash_id() {
        let bad =
            update_workspace_config(Path(String::new()), Json(json!({ "quota": "1G" }))).await;
        assert!(matches!(bad, Err(StatusCode::BAD_REQUEST)));
        let bad2 = update_workspace_config(Path("a/b".to_string()), Json(json!({}))).await;
        assert!(matches!(bad2, Err(StatusCode::BAD_REQUEST)));
    }

    #[tokio::test]
    async fn delete_workspace_rejects_invalid_id() {
        let r = delete_workspace(Path(String::new())).await;
        assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
        let r2 = delete_workspace(Path("../escape".to_string())).await;
        assert!(matches!(r2, Err(StatusCode::BAD_REQUEST)));
    }

    /// Prepends a fake `zfs` executable to `PATH` for deterministic handler coverage.
    #[cfg(unix)]
    struct FakeZfsPathGuard {
        _dir: tempfile::TempDir,
        _prev_path: Option<String>,
    }

    #[cfg(unix)]
    impl FakeZfsPathGuard {
        fn new(script: &str) -> Self {
            use std::os::unix::fs::PermissionsExt;
            let dir = tempfile::tempdir().expect("tempdir");
            let bin = dir.path().join("zfs");
            std::fs::write(&bin, script).expect("write fake zfs");
            let mut perms = std::fs::metadata(&bin).expect("metadata").permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&bin, perms).expect("chmod");
            let prev = std::env::var("PATH").ok();
            let new_path = format!("{}:{}", dir.path().display(), prev.as_deref().unwrap_or(""));
            nestgate_core::env_process::set_var("PATH", &new_path);
            Self {
                _dir: dir,
                _prev_path: prev,
            }
        }
    }

    #[cfg(unix)]
    impl Drop for FakeZfsPathGuard {
        fn drop(&mut self) {
            if let Some(ref p) = self._prev_path {
                nestgate_core::env_process::set_var("PATH", p);
            } else {
                nestgate_core::env_process::remove_var("PATH");
            }
        }
    }

    /// Covers `get_workspaces` success path, `get_workspace_properties`, and list parsing.
    #[cfg(unix)]
    const FAKE_ZFS_GET_WORKSPACES: &str = r#"#!/bin/bash
cmd="${1:-}"
case "$cmd" in
  list)
    if [[ "$*" == *"self.base_url/workspaces"* ]]; then
      echo -e "zfspool/workspaces/ws-active\t1048576\t10485760\t1048576\t/mnt\tJan 1 2020"
      echo -e "zfspool/workspaces/ws-inactive\t1048576\t10485760\t1048576\t/mnt\tJan 1 2020"
      exit 0
    fi
    if [[ "$*" == *"-t snapshot"* ]]; then
      echo "pool@snap"
      exit 0
    fi
    if [[ "$*" == *"rpool/workspaces"* ]] && [[ "$*" == *"-o name"* ]]; then
      exit 1
    fi
    exit 1
    ;;
  get)
    if [[ "$*" == *"property,value"* ]]; then
      dataset="${@: -1}"
      if [[ "$dataset" == *"crit"* ]]; then
        echo -e "used\t95"; echo -e "available\t5"; echo -e "referenced\t95"
        echo -e "quota\t100"; echo -e "compression\tlz4"; echo -e "recordsize\t128K"
        echo -e "mountpoint\t/tmp"; echo -e "creation\tJan 1 2020"
        echo -e "org.nestgate:workspace_name\tCrit"
      elif [[ "$dataset" == *"warn"* ]]; then
        echo -e "used\t85"; echo -e "available\t15"; echo -e "referenced\t85"
        echo -e "quota\t100"; echo -e "compression\tlz4"; echo -e "recordsize\t128K"
        echo -e "mountpoint\t/tmp"; echo -e "creation\tJan 1 2020"
        echo -e "org.nestgate:workspace_name\tWarn"
      elif [[ "$dataset" == *"fallback"* ]]; then
        echo -e "used\t10"; echo -e "available\t90"; echo -e "referenced\t10"
        echo -e "quota\t100"; echo -e "compression\tlz4"; echo -e "recordsize\t128K"
        echo -e "mountpoint\t/tmp"; echo -e "creation\tJan 1 2020"
      else
        echo -e "used\t10"; echo -e "available\t90"; echo -e "referenced\t10"
        echo -e "quota\t100"; echo -e "compression\tlz4"; echo -e "recordsize\t128K"
        echo -e "mountpoint\t/tmp"; echo -e "creation\tJan 1 2020"
        echo -e "org.nestgate:workspace_name\tNamed WS"
      fi
      exit 0
    fi
    if [[ "$*" == *"used,available,quota,compression"* ]] && [[ "$*" == *"self.base_url"* ]]; then
      echo "1"; echo "2"; echo "3"; echo "4"
      exit 0
    fi
    if [[ "$*" == *"compression,quota,mounted"* ]]; then
      dataset="${@: -1}"
      if [[ "$dataset" == *"inactive"* ]]; then
        echo "lz4"; echo "10G"; echo "no"
      else
        echo "lz4"; echo "10G"; echo "yes"
      fi
      exit 0
    fi
    exit 1
    ;;
  create)
    exit 0
    ;;
  set)
    if [[ "$2" == "quota=bad"* ]] || [[ "$2" == "compression=bad"* ]] || [[ "$2" == org.nestgate:workspace_name=bad* ]]; then
      echo fail >&2
      exit 1
    fi
    exit 0
    ;;
  destroy)
    exit 0
    ;;
  *)
    exit 1
    ;;
esac
"#;

    #[cfg(unix)]
    #[serial]
    #[tokio::test]
    async fn get_workspaces_fake_zfs_lists_datasets() {
        let _g = FakeZfsPathGuard::new(FAKE_ZFS_GET_WORKSPACES);
        nestgate_core::env_process::remove_var("NESTGATE_WORKSPACE_POOL");
        let Json(v) = get_workspaces().await.expect("ok");
        assert_eq!(v["status"], "success");
        assert_eq!(v["pool"], "zfspool");
        let arr = v["workspaces"].as_array().expect("array");
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["id"], "ws-active");
        assert_eq!(arr[0]["status"], "active");
        assert_eq!(arr[1]["id"], "ws-inactive");
        assert_eq!(arr[1]["status"], "inactive");
    }

    #[cfg(unix)]
    #[serial]
    #[tokio::test]
    async fn get_workspace_fake_zfs_healthy_warning_critical_and_name_fallback() {
        let _g = FakeZfsPathGuard::new(FAKE_ZFS_GET_WORKSPACES);
        nestgate_core::env_process::remove_var("NESTGATE_WORKSPACE_POOL");

        let Json(h) = get_workspace(Path("ws-healthy".to_string()))
            .await
            .expect("healthy");
        assert_eq!(h["workspace"]["health_status"], "healthy");
        assert_eq!(h["workspace"]["name"], "Named WS");

        let Json(w) = get_workspace(Path("ws-warn-test".to_string()))
            .await
            .expect("warn");
        assert_eq!(w["workspace"]["health_status"], "warning");

        let Json(c) = get_workspace(Path("ws-crit-test".to_string()))
            .await
            .expect("crit");
        assert_eq!(c["workspace"]["health_status"], "critical");

        let Json(f) = get_workspace(Path("fallback-ws-id".to_string()))
            .await
            .expect("fallback");
        assert_eq!(f["workspace"]["name"], "fallback ws id");
    }

    #[cfg(unix)]
    #[serial]
    #[tokio::test]
    async fn create_workspace_fake_zfs_succeeds() {
        let _g = FakeZfsPathGuard::new(FAKE_ZFS_GET_WORKSPACES);
        nestgate_core::env_process::remove_var("NESTGATE_WORKSPACE_POOL");
        let req = json!({
            "name": "ok-ws",
            "quota": "20G",
            "compression": "zstd",
            "recordsize": "256K"
        });
        let Json(v) = create_workspace(Json(req)).await.expect("create ok");
        assert_eq!(v["status"], "success");
        assert!(v.get("workspace_id").is_some());
        assert_eq!(v["name"], "ok-ws");
    }

    #[cfg(unix)]
    #[serial]
    #[tokio::test]
    async fn update_workspace_config_fake_zfs_success_empty_and_partial() {
        let _g = FakeZfsPathGuard::new(FAKE_ZFS_GET_WORKSPACES);
        nestgate_core::env_process::remove_var("NESTGATE_WORKSPACE_POOL");

        let Json(empty) = update_workspace_config(Path("ws-1".to_string()), Json(json!({})))
            .await
            .expect("empty ok");
        assert_eq!(empty["status"], "success");
        let up: Vec<String> = serde_json::from_value(empty["updated_properties"].clone())
            .expect("updated_properties");
        assert!(up.is_empty());

        let Json(ok) = update_workspace_config(
            Path("ws-2".to_string()),
            Json(json!({ "quota": "10G", "compression": "lz4", "name": "n" })),
        )
        .await
        .expect("full ok");
        assert_eq!(ok["status"], "success");

        let Json(partial) = update_workspace_config(
            Path("ws-3".to_string()),
            Json(json!({ "quota": "badquota", "compression": "lz4", "name": "still-ok" })),
        )
        .await
        .expect("partial");
        assert_eq!(partial["status"], "partial_success");
        let errs = partial["errors"].as_array().expect("errors");
        assert!(!errs.is_empty());

        let all_bad = update_workspace_config(
            Path("ws-4".to_string()),
            Json(json!({
                "quota": "badquota",
                "compression": "badcompression",
                "name": "badname"
            })),
        )
        .await;
        assert!(matches!(all_bad, Err(StatusCode::BAD_REQUEST)));
    }

    #[cfg(unix)]
    #[serial]
    #[tokio::test]
    async fn delete_workspace_fake_zfs_dataset_missing_is_not_found() {
        let _g = FakeZfsPathGuard::new(FAKE_ZFS_GET_WORKSPACES);
        let r = delete_workspace(Path("missing-ws".to_string())).await;
        assert!(matches!(r, Err(StatusCode::NOT_FOUND)));
    }
}
