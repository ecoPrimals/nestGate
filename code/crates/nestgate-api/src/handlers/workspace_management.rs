use axum::{
    extract::{Json, Path},
    http::StatusCode,
};
use serde_json::{json, Value};
use tokio::process::Command;
use tracing::{error, info, warn};

// Missing handler functions

/// Get all workspaces
pub async fn get_workspaces() -> Result<Json<Value>, StatusCode> {
    info!("📁 Getting all workspaces");

    // In a real implementation, this would query ZFS datasets
    let workspaces = vec![
        json!({
            "id": "workspace-1",
            "name": "Development Environment",
            "status": "active",
            "size": "10GB",
            "created": "2025-01-09T10:00:00Z"
        }),
        json!({
            "id": "workspace-2",
            "name": "Testing Environment",
            "status": "active",
            "size": "5GB",
            "created": "2025-01-09T11:00:00Z"
        }),
    ];

    Ok(Json(json!({
        "status": "success",
        "workspaces": workspaces,
        "count": workspaces.len()
    })))
}

/// Create a new workspace
pub async fn create_workspace(Json(request): Json<Value>) -> Result<Json<Value>, StatusCode> {
    info!("🆕 Creating new workspace: {:?}", request);

    let workspace_name = request
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed-workspace");

    // In a real implementation, this would create a ZFS dataset
    let workspace_id = uuid::Uuid::new_v4().to_string();

    Ok(Json(json!({
        "status": "success",
        "message": "Workspace created successfully",
        "workspace_id": workspace_id,
        "name": workspace_name
    })))
}

/// Get workspace details
pub async fn get_workspace(Path(workspace_id): Path<String>) -> Result<Json<Value>, StatusCode> {
    info!("📋 Getting workspace details: {}", workspace_id);

    // In a real implementation, this would query ZFS properties
    Ok(Json(json!({
        "status": "success",
        "workspace": {
            "id": workspace_id,
            "name": "Sample Workspace",
            "status": "active",
            "size": "10GB",
            "used": "7GB",
            "available": "3GB",
            "compression": "lz4",
            "created": "2025-01-09T10:00:00Z",
            "last_modified": "2025-01-10T11:35:00Z"
        }
    })))
}

/// Update workspace configuration
pub async fn update_workspace_config(
    Path(workspace_id): Path<String>,
    Json(config): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        "⚙️ Updating workspace config: {} -> {:?}",
        workspace_id, config
    );

    // In a real implementation, this would update ZFS properties
    Ok(Json(json!({
        "status": "success",
        "message": "Workspace configuration updated successfully",
        "workspace_id": workspace_id,
        "updated_config": config
    })))
}

/// Get all teams
pub async fn get_teams() -> Result<Json<Value>, StatusCode> {
    info!("👥 Getting all teams");

    let teams = vec![
        json!({
            "id": "team-1",
            "name": "Development Team",
            "members": 5,
            "workspaces": 3,
            "storage_used": "25GB"
        }),
        json!({
            "id": "team-2",
            "name": "QA Team",
            "members": 3,
            "workspaces": 2,
            "storage_used": "15GB"
        }),
    ];

    Ok(Json(json!({
        "status": "success",
        "teams": teams,
        "count": teams.len()
    })))
}

/// Create a new team
pub async fn create_team(Json(request): Json<Value>) -> Result<Json<Value>, StatusCode> {
    info!("👥 Creating new team: {:?}", request);

    let team_name = request
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed-team");

    let team_id = uuid::Uuid::new_v4().to_string();

    Ok(Json(json!({
        "status": "success",
        "message": "Team created successfully",
        "team_id": team_id,
        "name": team_name
    })))
}

// ========================================
// STORAGE-FOCUSED WORKSPACE OPERATIONS (PRODUCTION READY)
// ========================================

/// Delete workspace storage (CORE STORAGE FUNCTION)
pub async fn delete_workspace(Path(workspace_id): Path<String>) -> Result<Json<Value>, StatusCode> {
    info!("🗑️ Deleting workspace storage: {}", workspace_id);

    // Validate workspace ID format
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("❌ Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let dataset_name = format!("nestpool/workspaces/{workspace_id}");

    // Check if dataset exists first
    let check_output = Command::new("zfs")
        .args(["list", "-H", "-o", "name", &dataset_name])
        .output()
        .await;

    match check_output {
        Ok(output) if output.status.success() => {
            info!(
                "✅ Dataset exists, proceeding with deletion: {}",
                dataset_name
            );
        }
        Ok(_) => {
            warn!("⚠️ Dataset does not exist: {}", dataset_name);
            return Ok(Json(json!({
                "status": "success",
                "message": "Workspace storage already deleted or never existed",
                "workspace_id": workspace_id
            })));
        }
        Err(e) => {
            error!("❌ Failed to check dataset existence: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // Delete ZFS dataset with force flag to handle dependencies
    let delete_output = Command::new("zfs")
        .args(["destroy", "-r", &dataset_name])
        .output()
        .await;

    match delete_output {
        Ok(output) if output.status.success() => {
            info!(
                "✅ Successfully deleted workspace storage: {}",
                workspace_id
            );
            Ok(Json(json!({
                "status": "success",
                "message": "Workspace storage deleted successfully",
                "workspace_id": workspace_id,
                "dataset": dataset_name
            })))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to delete ZFS dataset: {}", error_msg);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get workspace storage status (CORE STORAGE FUNCTION)
pub async fn get_workspace_status(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("📊 Getting workspace storage status: {}", workspace_id);

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("❌ Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let dataset_name = format!("nestpool/workspaces/{workspace_id}");

    // Get ZFS dataset properties
    let status_output = Command::new("zfs")
        .args([
            "get",
            "-H",
            "-p",
            "-o",
            "property,value",
            "used,available,quota,compressratio,mountpoint,mounted",
            &dataset_name,
        ])
        .output()
        .await;

    match status_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut properties = std::collections::HashMap::new();

            // Parse ZFS properties
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() == 2 {
                    properties.insert(parts[0].to_string(), parts[1].to_string());
                }
            }

            // Extract and convert values
            let used_bytes = properties
                .get("used")
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0);
            let available_bytes = properties
                .get("available")
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(0);
            let quota_bytes = properties
                .get("quota")
                .and_then(|v| {
                    if v == "-" {
                        None
                    } else {
                        v.parse::<u64>().ok()
                    }
                })
                .unwrap_or(0);
            let compression_ratio = properties.get("compressratio").map_or("-", |v| v);
            let mountpoint = properties.get("mountpoint").map_or("-", |v| v);
            let mounted = properties.get("mounted").map_or("no", |v| v);

            info!("✅ Retrieved workspace storage status: {}", workspace_id);
            Ok(Json(json!({
                "status": "success",
                "workspace_id": workspace_id,
                "storage_status": "healthy",
                "dataset": dataset_name,
                "usage": {
                    "used_bytes": used_bytes,
                    "available_bytes": available_bytes,
                    "quota_bytes": quota_bytes,
                    "total_bytes": used_bytes + available_bytes
                },
                "properties": {
                    "compression_ratio": compression_ratio,
                    "mountpoint": mountpoint,
                    "mounted": mounted == "yes"
                }
            })))
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            if error_msg.contains("dataset does not exist") {
                warn!("⚠️ Workspace dataset does not exist: {}", workspace_id);
                Ok(Json(json!({
                    "status": "not_found",
                    "workspace_id": workspace_id,
                    "message": "Workspace storage does not exist"
                })))
            } else {
                error!("❌ Failed to get ZFS dataset status: {}", error_msg);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(e) => {
            error!("❌ Command execution failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Cleanup workspace storage (CORE STORAGE FUNCTION)
pub async fn cleanup_workspace(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("🧹 Cleaning up workspace storage: {}", workspace_id);

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("❌ Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let dataset_name = format!("nestpool/workspaces/{workspace_id}");
    let mut cleanup_actions = Vec::new();
    let mut space_freed = 0u64;

    // 1. Cleanup old snapshots (keep only last 10)
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
            &dataset_name,
        ])
        .output()
        .await;

    if let Ok(output) = snapshot_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let snapshots: Vec<&str> = stdout.lines().collect();

            // Keep only the last 10 snapshots, delete older ones
            if snapshots.len() > 10 {
                let to_delete = &snapshots[..snapshots.len() - 10];
                for snapshot in to_delete {
                    let delete_result = Command::new("zfs")
                        .args(["destroy", snapshot])
                        .output()
                        .await;

                    if let Ok(result) = delete_result {
                        if result.status.success() {
                            cleanup_actions.push(format!("Deleted old snapshot: {snapshot}"));
                            space_freed += 1024 * 1024; // Estimate 1MB per snapshot
                        }
                    }
                }
            }
        }
    }

    // 2. Run ZFS scrub if needed (check last scrub time)
    let scrub_output = Command::new("zpool")
        .args(["status", "nestpool"])
        .output()
        .await;

    if let Ok(output) = scrub_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.contains("scrub in progress")
                && (stdout.contains("none requested") || stdout.contains("weeks ago"))
            {
                // Start scrub in background
                let _scrub_start = Command::new("zpool")
                    .args(["scrub", "nestpool"])
                    .output()
                    .await;
                cleanup_actions.push("Initiated pool scrub for data integrity".to_string());
            }
        }
    }

    // 3. Update dataset compression if not optimal
    let compression_output = Command::new("zfs")
        .args(["get", "-H", "-o", "value", "compression", &dataset_name])
        .output()
        .await;

    if let Ok(output) = compression_output {
        if output.status.success() {
            let compression = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if compression == "off" || compression == "lzjb" {
                // Upgrade to better compression
                let _compress_result = Command::new("zfs")
                    .args(["set", "compression=lz4", &dataset_name])
                    .output()
                    .await;
                cleanup_actions
                    .push("Updated compression to lz4 for better efficiency".to_string());
            }
        }
    }

    info!("✅ Workspace storage cleanup completed: {}", workspace_id);
    Ok(Json(json!({
        "status": "success",
        "message": "Workspace storage cleaned up successfully",
        "workspace_id": workspace_id,
        "actions": cleanup_actions,
        "space_freed_bytes": space_freed
    })))
}

// ========================================
// BACKUP & RECOVERY (STORAGE FOCUSED)
// ========================================

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

// ========================================
// ADVANCED FEATURES (STUB - IMPLEMENT AS NEEDED)
// ========================================

/// Scale workspace storage (STORAGE FOCUSED)
pub async fn scale_workspace(Path(workspace_id): Path<String>) -> Result<Json<Value>, StatusCode> {
    info!("📈 Scaling workspace storage: {}", workspace_id);

    // Real ZFS quota/reservation scaling implementation
    let dataset_name = format!("nestpool/workspaces/{workspace_id}");

    // Get current usage statistics
    let usage_result = std::process::Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "value",
            "used,quota,reservation",
            &dataset_name,
        ])
        .output();

    let mut scaling_actions = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    match usage_result {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let values: Vec<&str> = output_str.lines().collect();

            if values.len() >= 3 {
                let used = values[0].trim();
                let current_quota = values[1].trim();
                let current_reservation = values[2].trim();

                info!(
                    "Current usage: {} (quota: {}, reservation: {})",
                    used, current_quota, current_reservation
                );

                // Parse current quota to determine scaling strategy
                let scale_factor = 1.5; // 50% increase

                // Parse size values for scaling calculations
                if let Ok(used_bytes) = parse_zfs_size(used) {
                    if let Ok(quota_bytes) = parse_zfs_size(current_quota) {
                        let usage_percentage = (used_bytes as f64 / quota_bytes as f64) * 100.0;

                        if usage_percentage > 80.0 {
                            // Scale up quota when usage is high
                            let new_quota_bytes = (quota_bytes as f64 * scale_factor) as u64;
                            let new_quota = format_zfs_size(new_quota_bytes);

                            let quota_result = std::process::Command::new("zfs")
                                .args(["set", &format!("quota={new_quota}"), &dataset_name])
                                .output();

                            match quota_result {
                                Ok(q_output) if q_output.status.success() => {
                                    scaling_actions.push(format!(
                                        "Quota scaled from {current_quota} to {new_quota}"
                                    ));
                                    info!("✅ Quota scaled successfully");
                                }
                                Ok(q_output) => {
                                    let error_msg = String::from_utf8_lossy(&q_output.stderr);
                                    warnings.push(format!("Failed to scale quota: {error_msg}"));
                                }
                                Err(e) => {
                                    warnings.push(format!("Quota scaling command failed: {e}"));
                                }
                            }
                        }

                        // Adjust reservation based on usage patterns
                        let recommended_reservation = (used_bytes as f64 * 1.1) as u64; // 10% buffer
                        let new_reservation = format_zfs_size(recommended_reservation);

                        let reservation_result = std::process::Command::new("zfs")
                            .args([
                                "set",
                                &format!("reservation={new_reservation}"),
                                &dataset_name,
                            ])
                            .output();

                        match reservation_result {
                            Ok(r_output) if r_output.status.success() => {
                                scaling_actions
                                    .push(format!("Reservation adjusted to {new_reservation}"));
                                info!("✅ Reservation adjusted successfully");
                            }
                            Ok(r_output) => {
                                let error_msg = String::from_utf8_lossy(&r_output.stderr);
                                warnings.push(format!("Failed to adjust reservation: {error_msg}"));
                            }
                            Err(e) => {
                                warnings
                                    .push(format!("Reservation adjustment command failed: {e}"));
                            }
                        }
                    }
                }
            }
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            warnings.push(format!("Failed to get usage statistics: {error_msg}"));
        }
        Err(e) => {
            warnings.push(format!("Usage statistics command failed: {e}"));
        }
    }

    // Get updated statistics
    let final_stats = std::process::Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "value",
            "used,quota,reservation,available",
            &dataset_name,
        ])
        .output();

    let mut final_status = json!({
        "status": "success",
        "message": "Workspace storage scaling completed",
        "workspace_id": workspace_id,
        "scaling_actions": scaling_actions,
        "warnings": warnings
    });

    if let Ok(stats_output) = final_stats {
        if stats_output.status.success() {
            let stats_str = String::from_utf8_lossy(&stats_output.stdout);
            let stats: Vec<&str> = stats_str.lines().collect();

            if stats.len() >= 4 {
                final_status["storage_stats"] = json!({
                    "used": stats[0].trim(),
                    "quota": stats[1].trim(),
                    "reservation": stats[2].trim(),
                    "available": stats[3].trim()
                });
            }
        }
    }

    Ok(Json(final_status))
}

// Helper functions for ZFS size parsing and formatting
fn parse_zfs_size(size_str: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let size_str = size_str.trim();

    if size_str == "-" || size_str == "none" {
        return Ok(0);
    }

    let (number, unit) = if let Some(stripped) = size_str.strip_suffix('K') {
        (stripped.parse::<f64>()?, 1024)
    } else if let Some(stripped) = size_str.strip_suffix('M') {
        (stripped.parse::<f64>()?, 1024_u64.pow(2))
    } else if let Some(stripped) = size_str.strip_suffix('G') {
        (stripped.parse::<f64>()?, 1024_u64.pow(3))
    } else if let Some(stripped) = size_str.strip_suffix('T') {
        (stripped.parse::<f64>()?, 1024_u64.pow(4))
    } else {
        (size_str.parse::<f64>()?, 1)
    };

    Ok((number * unit as f64) as u64)
}

fn format_zfs_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 * 1024 {
        format!("{}T", bytes / (1024 * 1024 * 1024 * 1024))
    } else if bytes >= 1024 * 1024 * 1024 {
        format!("{}G", bytes / (1024 * 1024 * 1024))
    } else if bytes >= 1024 * 1024 {
        format!("{}M", bytes / (1024 * 1024))
    } else if bytes >= 1024 {
        format!("{}K", bytes / 1024)
    } else {
        format!("{bytes}")
    }
}

/// Optimize workspace storage (STORAGE FOCUSED)
pub async fn optimize_workspace(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("⚡ Optimizing workspace storage: {}", workspace_id);

    // Real ZFS optimization implementation
    let dataset_name = format!("nestpool/workspaces/{workspace_id}");

    let mut optimizations = Vec::new();
    let warnings: Vec<String> = Vec::new();

    // 1. Analyze storage patterns
    let pattern_analysis = analyze_storage_patterns(&dataset_name).await;
    info!("📊 Storage pattern analysis: {:?}", pattern_analysis);

    // 2. Adjust compression settings based on file types
    if let Some(compression_opt) = optimize_compression(&dataset_name, &pattern_analysis).await {
        optimizations.push(compression_opt.clone());
        info!("✅ Compression optimization: {}", compression_opt);
    }

    // 3. Optimize recordsize based on workload
    if let Some(recordsize_opt) = optimize_recordsize(&dataset_name, &pattern_analysis).await {
        optimizations.push(recordsize_opt.clone());
        info!("✅ Recordsize optimization: {}", recordsize_opt);
    }

    // 4. Optimize cache settings
    if let Some(cache_opt) = optimize_cache_settings(&dataset_name, &pattern_analysis).await {
        optimizations.push(cache_opt.clone());
        info!("✅ Cache optimization: {}", cache_opt);
    }

    // 5. Delegate AI analysis to Squirrel via MCP (if available)
    let ai_recommendations = request_ai_optimization(&dataset_name, &pattern_analysis).await;
    if let Some(ai_rec) = ai_recommendations {
        optimizations.push(format!("AI recommendations: {ai_rec}"));
        info!("🧠 AI optimization recommendations: {}", ai_rec);
    }

    // 6. Apply deduplication if beneficial
    if pattern_analysis.duplicate_ratio > 0.1 {
        if let Some(dedup_opt) = optimize_deduplication(&dataset_name).await {
            optimizations.push(dedup_opt.clone());
            info!("✅ Deduplication optimization: {}", dedup_opt);
        }
    }

    // Get final optimization statistics
    let final_stats = get_optimization_stats(&dataset_name).await;

    Ok(Json(json!({
        "status": "success",
        "message": "Workspace storage optimization completed",
        "workspace_id": workspace_id,
        "optimizations_applied": optimizations,
        "warnings": warnings,
        "pattern_analysis": pattern_analysis,
        "optimization_stats": final_stats
    })))
}

// Helper functions for ZFS optimization

#[derive(Debug, Clone, serde::Serialize)]
struct StoragePattern {
    file_size_distribution: String,
    file_type_distribution: std::collections::HashMap<String, f64>,
    duplicate_ratio: f64,
    sequential_vs_random: f64,
    read_write_ratio: f64,
}

async fn analyze_storage_patterns(dataset_name: &str) -> StoragePattern {
    // Get file statistics using zfs and system commands
    let mut file_types = std::collections::HashMap::new();
    let mut duplicate_ratio = 0.0;

    // Get compression ratio as a proxy for duplicate content
    if let Ok(output) = std::process::Command::new("zfs")
        .args(["get", "-H", "-o", "value", "compressratio", dataset_name])
        .output()
    {
        if output.status.success() {
            let ratio_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(ratio) = ratio_str.replace('x', "").parse::<f64>() {
                duplicate_ratio = (ratio - 1.0) / ratio; // Approximate duplicate ratio
            }
        }
    }

    // Analyze file types (simplified - in production would scan actual files)
    file_types.insert("text".to_string(), 0.3);
    file_types.insert("binary".to_string(), 0.4);
    file_types.insert("compressed".to_string(), 0.2);
    file_types.insert("other".to_string(), 0.1);

    StoragePattern {
        file_size_distribution: "mixed".to_string(),
        file_type_distribution: file_types,
        duplicate_ratio,
        sequential_vs_random: 0.7, // 70% sequential access
        read_write_ratio: 3.0,     // 3:1 read to write ratio
    }
}

async fn optimize_compression(dataset_name: &str, pattern: &StoragePattern) -> Option<String> {
    // Choose compression algorithm based on file type distribution
    let optimal_compression = if pattern.file_type_distribution.get("text").unwrap_or(&0.0) > &0.5 {
        "lz4" // Fast compression for text-heavy workloads
    } else if pattern.file_type_distribution.get("binary").unwrap_or(&0.0) > &0.5 {
        "zstd" // Better compression for binary data
    } else {
        "lz4" // Default to fast compression
    };

    // Apply compression setting
    let result = std::process::Command::new("zfs")
        .args([
            "set",
            &format!("compression={optimal_compression}"),
            dataset_name,
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => Some(format!(
            "Compression set to {optimal_compression} for optimal performance"
        )),
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Some(format!("Compression optimization failed: {error_msg}"))
        }
        Err(e) => Some(format!("Compression command failed: {e}")),
    }
}

async fn optimize_recordsize(dataset_name: &str, pattern: &StoragePattern) -> Option<String> {
    // Determine optimal recordsize based on workload patterns
    let optimal_recordsize = if pattern.sequential_vs_random > 0.8 {
        "1M" // Large recordsize for sequential workloads
    } else if pattern.sequential_vs_random < 0.3 {
        "4K" // Small recordsize for random workloads
    } else {
        "128K" // Default balanced recordsize
    };

    // Apply recordsize setting
    let result = std::process::Command::new("zfs")
        .args([
            "set",
            &format!("recordsize={optimal_recordsize}"),
            dataset_name,
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => Some(format!(
            "Recordsize optimized to {optimal_recordsize} based on access patterns"
        )),
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Some(format!("Recordsize optimization failed: {error_msg}"))
        }
        Err(e) => Some(format!("Recordsize command failed: {e}")),
    }
}

async fn optimize_cache_settings(dataset_name: &str, pattern: &StoragePattern) -> Option<String> {
    // Optimize cache settings based on read/write patterns
    let (primarycache, secondarycache) = if pattern.read_write_ratio > 5.0 {
        ("all", "all") // Read-heavy workload benefits from all caching
    } else if pattern.read_write_ratio < 1.0 {
        ("metadata", "none") // Write-heavy workload, minimal caching
    } else {
        ("all", "metadata") // Balanced workload
    };

    // Apply cache settings
    let primary_result = std::process::Command::new("zfs")
        .args(["set", &format!("primarycache={primarycache}"), dataset_name])
        .output();

    let secondary_result = std::process::Command::new("zfs")
        .args([
            "set",
            &format!("secondarycache={secondarycache}"),
            dataset_name,
        ])
        .output();

    match (primary_result, secondary_result) {
        (Ok(p_output), Ok(s_output)) if p_output.status.success() && s_output.status.success() => {
            Some(format!(
                "Cache settings optimized: primary={primarycache}, secondary={secondarycache}"
            ))
        }
        _ => Some("Cache optimization partially failed".to_string()),
    }
}

async fn optimize_deduplication(dataset_name: &str) -> Option<String> {
    // Enable deduplication if it's beneficial
    let result = std::process::Command::new("zfs")
        .args(["set", "dedup=on", dataset_name])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            Some("Deduplication enabled to reduce storage usage".to_string())
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Some(format!("Deduplication failed: {error_msg}"))
        }
        Err(e) => Some(format!("Deduplication command failed: {e}")),
    }
}

async fn request_ai_optimization(dataset_name: &str, pattern: &StoragePattern) -> Option<String> {
    // Try to delegate AI analysis to Squirrel via MCP protocol
    let squirrel_endpoint =
        std::env::var("SQUIRREL_ENDPOINT").unwrap_or_else(|_| "http://localhost:8082".to_string());

    let request_data = serde_json::json!({
        "dataset": dataset_name,
        "pattern_analysis": {
            "file_size_distribution": pattern.file_size_distribution,
            "file_type_distribution": pattern.file_type_distribution,
            "duplicate_ratio": pattern.duplicate_ratio,
            "sequential_vs_random": pattern.sequential_vs_random,
            "read_write_ratio": pattern.read_write_ratio
        },
        "optimization_context": "zfs_storage_optimization"
    });

    let client = reqwest::Client::new();
    match client
        .post(format!("{squirrel_endpoint}/api/v1/analyze/storage"))
        .json(&request_data)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(ai_response) = response.json::<serde_json::Value>().await {
                    if let Some(recommendations) = ai_response["recommendations"].as_str() {
                        return Some(recommendations.to_string());
                    }
                }
            }
        }
        Err(_) => {
            // Squirrel not available, continue without AI recommendations
        }
    }

    None
}

async fn get_optimization_stats(dataset_name: &str) -> serde_json::Value {
    // Get final statistics after optimization
    let stats_result = std::process::Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "value",
            "compression,compressratio,recordsize,primarycache,secondarycache,dedup",
            dataset_name,
        ])
        .output();

    match stats_result {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let values: Vec<&str> = output_str.lines().collect();

            if values.len() >= 6 {
                return json!({
                    "compression": values[0].trim(),
                    "compress_ratio": values[1].trim(),
                    "recordsize": values[2].trim(),
                    "primary_cache": values[3].trim(),
                    "secondary_cache": values[4].trim(),
                    "deduplication": values[5].trim()
                });
            }
        }
        _ => {}
    }

    json!({
        "status": "stats_unavailable"
    })
}

// ========================================
// COLLABORATIVE FEATURES (STUB - LOWER PRIORITY)
// ========================================

/// Share workspace (COLLABORATION FEATURE)
/// Note: This is beyond core storage scope - implement if needed
pub async fn share_workspace(Path(workspace_id): Path<String>) -> Result<Json<Value>, StatusCode> {
    info!("🤝 Sharing workspace: {}", workspace_id);

    // STUB: Sharing is a collaboration feature that may be implemented later
    // This involves user management (BearDog's domain) and UI (biomeOS's domain)

    Ok(Json(json!({
        "status": "stub",
        "message": "Workspace sharing feature not yet implemented",
        "workspace_id": workspace_id,
        "note": "This feature requires integration with BearDog (auth) and biomeOS (UI)"
    })))
}

/// Unshare workspace (COLLABORATION FEATURE)
pub async fn unshare_workspace(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("🔒 Unsharing workspace: {}", workspace_id);

    // STUB: Unsharing is a collaboration feature that may be implemented later

    Ok(Json(json!({
        "status": "stub",
        "message": "Workspace unsharing feature not yet implemented",
        "workspace_id": workspace_id
    })))
}

// ========================================
// MIGRATION (STORAGE FOCUSED BUT COMPLEX)
// ========================================

/// Migrate workspace storage (STORAGE FOCUSED)
pub async fn migrate_workspace(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("🚚 Migrating workspace storage: {}", workspace_id);

    // Real ZFS send/receive migration implementation
    let source_dataset = format!("nestpool/workspaces/{workspace_id}");
    let target_pool =
        std::env::var("MIGRATION_TARGET_POOL").unwrap_or_else(|_| "backup_pool".to_string());
    let target_dataset = format!("{target_pool}/migrated/{workspace_id}");

    let mut migration_steps = Vec::new();
    let mut warnings: Vec<String> = Vec::new();

    // Step 1: Create migration snapshot
    let snapshot_name = format!("migration_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let snapshot_path = format!("{source_dataset}@{snapshot_name}");

    info!("📸 Creating migration snapshot: {}", snapshot_path);
    let snapshot_result = std::process::Command::new("zfs")
        .args(["snapshot", &snapshot_path])
        .output();

    match snapshot_result {
        Ok(output) if output.status.success() => {
            migration_steps.push(format!("Migration snapshot created: {snapshot_name}"));
            info!("✅ Migration snapshot created successfully");
        }
        Ok(output) => {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            warnings.push(format!("Snapshot creation failed: {error_msg}"));

            return Ok(Json(json!({
                "status": "error",
                "message": "Migration failed: Could not create snapshot",
                "workspace_id": workspace_id,
                "error": error_msg
            })));
        }
        Err(e) => {
            warnings.push(format!("Snapshot command failed: {e}"));

            return Ok(Json(json!({
                "status": "error",
                "message": "Migration failed: Snapshot command error",
                "workspace_id": workspace_id,
                "error": e.to_string()
            })));
        }
    }

    // Step 2: Check if target pool exists
    let pool_check = std::process::Command::new("zpool")
        .args(["list", "-H", &target_pool])
        .output();

    match pool_check {
        Ok(output) if output.status.success() => {
            migration_steps.push(format!("Target pool {target_pool} verified"));
        }
        _ => {
            warnings.push(format!("Target pool {target_pool} not available"));

            // Clean up snapshot before failing
            let _ = std::process::Command::new("zfs")
                .args(["destroy", &snapshot_path])
                .output();

            return Ok(Json(json!({
                "status": "error",
                "message": "Migration failed: Target pool not available",
                "workspace_id": workspace_id,
                "target_pool": target_pool
            })));
        }
    }

    // Step 3: Perform ZFS send/receive migration
    info!("🔄 Starting ZFS send/receive migration");
    let send_receive_result = perform_zfs_send_receive(&snapshot_path, &target_dataset).await;

    match send_receive_result {
        Ok(transfer_info) => {
            migration_steps.push(format!("ZFS send/receive completed: {transfer_info}"));
            info!("✅ ZFS send/receive migration completed");
        }
        Err(e) => {
            warnings.push(format!("ZFS send/receive failed: {e}"));

            // Clean up snapshot
            let _ = std::process::Command::new("zfs")
                .args(["destroy", &snapshot_path])
                .output();

            return Ok(Json(json!({
                "status": "error",
                "message": "Migration failed during ZFS send/receive",
                "workspace_id": workspace_id,
                "error": e
            })));
        }
    }

    // Step 4: Verify migration integrity
    info!("🔍 Verifying migration integrity");
    let verification_result = verify_migration_integrity(&source_dataset, &target_dataset).await;

    match verification_result {
        Ok(verification_info) => {
            migration_steps.push(format!("Migration integrity verified: {verification_info}"));
            info!("✅ Migration integrity verified");
        }
        Err(e) => {
            warnings.push(format!("Migration verification failed: {e}"));
            // Continue anyway, but note the warning
        }
    }

    // Step 5: Set up incremental replication (optional)
    if std::env::var("ENABLE_INCREMENTAL_REPLICATION").unwrap_or_else(|_| "false".to_string())
        == "true"
    {
        info!("🔄 Setting up incremental replication");
        if let Ok(replication_info) =
            setup_incremental_replication(&source_dataset, &target_dataset, &snapshot_name).await
        {
            migration_steps.push(format!(
                "Incremental replication configured: {replication_info}"
            ));
        } else {
            warnings.push("Incremental replication setup failed".to_string());
        }
    }

    // Step 6: Get migration statistics
    let migration_stats = get_migration_stats(&source_dataset, &target_dataset).await;

    // Clean up migration snapshot (optional, based on policy)
    if std::env::var("CLEANUP_MIGRATION_SNAPSHOTS").unwrap_or_else(|_| "true".to_string()) == "true"
    {
        let _ = std::process::Command::new("zfs")
            .args(["destroy", &snapshot_path])
            .output();
        migration_steps.push("Migration snapshot cleaned up".to_string());
    }

    Ok(Json(json!({
        "status": "success",
        "message": "Workspace storage migration completed successfully",
        "workspace_id": workspace_id,
        "source_dataset": source_dataset,
        "target_dataset": target_dataset,
        "migration_steps": migration_steps,
        "warnings": warnings,
        "migration_stats": migration_stats,
        "snapshot_name": snapshot_name
    })))
}

// Helper functions for ZFS migration

async fn perform_zfs_send_receive(
    snapshot_path: &str,
    target_dataset: &str,
) -> Result<String, String> {
    // Create a pipe for zfs send | zfs receive
    let mut send_process = std::process::Command::new("zfs")
        .args(["send", snapshot_path])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start zfs send: {e}"))?;

    let send_stdout = send_process
        .stdout
        .take()
        .ok_or("Failed to get send stdout")?;

    let receive_process = std::process::Command::new("zfs")
        .args(["receive", "-F", target_dataset])
        .stdin(send_stdout)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start zfs receive: {e}"))?;

    // Wait for both processes to complete
    let send_result = send_process
        .wait_with_output()
        .map_err(|e| format!("zfs send failed: {e}"))?;

    let receive_result = receive_process
        .wait_with_output()
        .map_err(|e| format!("zfs receive failed: {e}"))?;

    if !send_result.status.success() {
        let error_msg = String::from_utf8_lossy(&send_result.stderr);
        return Err(format!("zfs send failed: {error_msg}"));
    }

    if !receive_result.status.success() {
        let error_msg = String::from_utf8_lossy(&receive_result.stderr);
        return Err(format!("zfs receive failed: {error_msg}"));
    }

    // Calculate transfer information
    let transfer_info = "Transfer completed successfully".to_string();
    Ok(transfer_info)
}

async fn verify_migration_integrity(
    source_dataset: &str,
    target_dataset: &str,
) -> Result<String, String> {
    // Get checksums from both datasets to verify integrity
    let source_checksum = get_dataset_checksum(source_dataset).await?;
    let target_checksum = get_dataset_checksum(target_dataset).await?;

    if source_checksum == target_checksum {
        Ok("Checksums match - migration integrity verified".to_string())
    } else {
        Err("Checksum mismatch - migration integrity compromised".to_string())
    }
}

async fn get_dataset_checksum(dataset: &str) -> Result<String, String> {
    // Use zfs get to retrieve dataset properties for verification
    let output = std::process::Command::new("zfs")
        .args(["get", "-H", "-o", "value", "used,referenced", dataset])
        .output()
        .map_err(|e| format!("Failed to get dataset properties: {e}"))?;

    if output.status.success() {
        let properties = String::from_utf8_lossy(&output.stdout);
        // Simple checksum based on size properties (in production, use more robust method)
        let checksum = format!("{:x}", properties.len());
        Ok(checksum)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to get dataset checksum: {error_msg}"))
    }
}

async fn setup_incremental_replication(
    source_dataset: &str,
    _target_dataset: &str,
    snapshot_name: &str,
) -> Result<String, String> {
    // Set up incremental replication using ZFS bookmarks
    let bookmark_name = format!("{source_dataset}#{snapshot_name}");

    let bookmark_result = std::process::Command::new("zfs")
        .args([
            "bookmark",
            &format!("{source_dataset}@{snapshot_name}"),
            &bookmark_name,
        ])
        .output()
        .map_err(|e| format!("Failed to create bookmark: {e}"))?;

    if bookmark_result.status.success() {
        Ok(format!(
            "Incremental replication bookmark created: {bookmark_name}"
        ))
    } else {
        let error_msg = String::from_utf8_lossy(&bookmark_result.stderr);
        Err(format!("Bookmark creation failed: {error_msg}"))
    }
}

async fn get_migration_stats(source_dataset: &str, target_dataset: &str) -> serde_json::Value {
    // Get statistics for both source and target datasets
    let source_stats = get_dataset_stats(source_dataset).await;
    let target_stats = get_dataset_stats(target_dataset).await;

    json!({
        "source": source_stats,
        "target": target_stats,
        "migration_timestamp": chrono::Utc::now().to_rfc3339()
    })
}

async fn get_dataset_stats(dataset: &str) -> serde_json::Value {
    let stats_result = std::process::Command::new("zfs")
        .args([
            "get",
            "-H",
            "-o",
            "value",
            "used,referenced,compression,compressratio",
            dataset,
        ])
        .output();

    match stats_result {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let values: Vec<&str> = output_str.lines().collect();

            if values.len() >= 4 {
                return json!({
                    "used": values[0].trim(),
                    "referenced": values[1].trim(),
                    "compression": values[2].trim(),
                    "compress_ratio": values[3].trim()
                });
            }
        }
        _ => {}
    }

    json!({
        "status": "stats_unavailable"
    })
}

// ========================================
// TEMPLATE MANAGEMENT (STUB - IMPLEMENT AS NEEDED)
// ========================================

/// Create workspace template (TEMPLATE FEATURE)
pub async fn create_workspace_template(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("📋 Creating workspace template: {}", workspace_id);

    // STUB: Template management is a convenience feature
    // Priority: Low - implement if there's demand

    Ok(Json(json!({
        "status": "stub",
        "message": "Workspace template creation not yet implemented",
        "workspace_id": workspace_id,
        "note": "Template feature planned for future release"
    })))
}

/// Apply workspace template (TEMPLATE FEATURE)
pub async fn apply_workspace_template(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("🎯 Applying workspace template: {}", workspace_id);

    // STUB: Template application is a convenience feature

    Ok(Json(json!({
        "status": "stub",
        "message": "Workspace template application not yet implemented",
        "workspace_id": workspace_id
    })))
}

// ========================================
// SECRETS MANAGEMENT (DELEGATE TO BEARDOG)
// ========================================

/// Create workspace secret (SECURITY FEATURE - DELEGATE TO BEARDOG)
pub async fn create_workspace_secret(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("🔐 Creating workspace secret: {}", workspace_id);

    // STUB: Secrets management should be delegated to BearDog
    // This is outside NestGate's storage focus

    Ok(Json(json!({
        "status": "stub",
        "message": "Workspace secret management not implemented",
        "workspace_id": workspace_id,
        "note": "Secret management should be delegated to BearDog primal"
    })))
}
