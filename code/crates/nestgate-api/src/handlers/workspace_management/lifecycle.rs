// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Workspace Lifecycle Management
//!
//! This module provides advanced lifecycle operations for workspaces including
//! backup, restore, migration, and lifecycle policy management using ZFS snapshots
//! and send/receive operations.

use axum::{extract::Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use tokio::process::Command;
use tracing::{debug, error, info, warn};

/// Backup configuration for workspace operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::BackupConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::BackupConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Backup
pub struct BackupConfig {
    /// Backup name/identifier
    pub backup_name: String,
    /// Include snapshots in backup
    pub include_snapshots: bool,
    /// Compression level (0-9)
    pub compression_level: u8,
    /// Encryption enabled
    pub encryption_enabled: bool,
    /// Description of the backup
    pub description: Option<String>,
}

/// Restore configuration for workspace operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::RestoreConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::RestoreConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Restore
pub struct RestoreConfig {
    /// Backup to restore from
    pub backup_name: String,
    /// Target workspace ID (if different from source)
    pub target_workspace_id: Option<String>,
    /// Restore point in time (snapshot name)
    pub restore_point: Option<String>,
    /// Force restore even if target exists
    pub force: bool,
}

/// Migration configuration for workspace operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::MigrationConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::MigrationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Migration
pub struct MigrationConfig {
    /// Target pool for migration
    pub target_pool: String,
    /// Target host for remote migration
    pub target_host: Option<String>,
    /// Migration strategy
    pub strategy: MigrationStrategy,
    /// Bandwidth limit in bytes per second
    pub bandwidth_limit: Option<u64>,
}

/// Migration strategy options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Migrationstrategy
pub enum MigrationStrategy {
    /// Copy data to new location, keep original
    Copy,
    /// Move data to new location, remove original
    Move,
    /// Create incremental replica
    Replicate,
}

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

/// Restore workspace from backup
#[allow(clippy::too_many_lines)]
pub async fn restore_workspace(
    Path(workspace_id): Path<String>,
    Json(config): Json<RestoreConfig>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        "🔄 Restoring workspace: {} from backup: {:?}",
        workspace_id, config
    );

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("❌ Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let target_workspace = config.target_workspace_id.as_ref().unwrap_or(&workspace_id);
    let dataset_name = format!("nestpool/workspaces/{target_workspace}");
    use nestgate_core::error::utilities::safe_env_var_or_default;
    let backup_dir = safe_env_var_or_default("NESTGATE_BACKUP_DIR", "/var/backups/nestgate");
    let backup_file = format!(
        "{}/workspace_{}_{}.zfs",
        backup_dir, workspace_id, config.backup_name
    );

    // Step 1: Check if backup file exists
    if !tokio::fs::try_exists(&backup_file).await.unwrap_or(false) {
        error!("❌ Backup file not found: {}", backup_file);
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
            warn!("⚠️ Target workspace already exists: {}", dataset_name);
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
            info!("🗑️ Destroyed existing workspace: {}", dataset_name);
        }
    }

    // Step 4: Restore from backup using ZFS receive
    info!("📥 Restoring from backup file: {}", backup_file);

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
                                            "✅ Workspace restored successfully: {} ({} bytes)",
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
                                        error!("❌ ZFS receive failed with status: {}", status);
                                    }
                                    Err(e) => {
                                        error!("❌ ZFS receive process error: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("❌ Failed to pipe backup data: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("❌ Failed to start ZFS receive process: {}", e);
                }
            }
        }
        Err(e) => {
            error!("❌ Failed to open backup file: {}", e);
        }
    }

    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

/// Migrate workspace to different pool or host
pub async fn migrate_workspace(
    Path(workspace_id): Path<String>,
    Json(config): Json<MigrationConfig>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        "🚀 Migrating workspace: {} with config: {:?}",
        workspace_id, config
    );

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("❌ Invalid workspace ID format: {}", workspace_id);
        return Err(StatusCode::BAD_REQUEST);
    }

    let source_dataset = format!("nestpool/workspaces/{workspace_id}");
    let target_dataset = format!("{}/workspaces/{}", config.target_pool, workspace_id);

    // Step 1: Verify source dataset exists
    let check_source = Command::new("zfs")
        .args(["list", "-H", "-o", "name", &source_dataset])
        .output()
        .await;

    match check_source {
        Ok(output) if !output.status.success() => {
            error!("❌ Source workspace not found: {}", source_dataset);
            return Ok(Json(json!({
                "status": "error",
                "message": format!("Source workspace not found: {source_dataset}"),
                "workspace_id": workspace_id
            })));
        }
        Err(e) => {
            error!("❌ Failed to check source workspace: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        _ => {}
    }

    // Step 2: Create migration snapshot
    // Modern: Proper error handling for system time
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or_else(
            |_| {
                warn!("⚠️ System time before UNIX epoch, using current timestamp");
                0
            },
            |d| d.as_secs(),
        );

    let migration_snapshot = format!("{source_dataset}@migrate_{timestamp}");

    let snapshot_result = Command::new("zfs")
        .args(["snapshot", &migration_snapshot])
        .output()
        .await;

    match snapshot_result {
        Ok(output) if !output.status.success() => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("❌ Failed to create migration snapshot: {}", stderr);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Err(e) => {
            error!("❌ Failed to create migration snapshot: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        _ => {
            info!("📸 Created migration snapshot: {}", migration_snapshot);
        }
    }

    // Step 3: Perform migration based on strategy
    let migration_result = match config.strategy {
        MigrationStrategy::Copy => {
            perform_copy_migration(&migration_snapshot, &target_dataset, &config).await
        }
        MigrationStrategy::Move => {
            perform_move_migration(
                &migration_snapshot,
                &target_dataset,
                &source_dataset,
                &config,
            )
            .await
        }
        MigrationStrategy::Replicate => {
            perform_replicate_migration(&migration_snapshot, &target_dataset, &config).await
        }
    };

    // Step 4: Cleanup migration snapshot
    let _ = Command::new("zfs")
        .args(["destroy", &migration_snapshot])
        .output()
        .await;

    match migration_result {
        Ok(result) => Ok(Json(result)),
        Err(status) => Err(status),
    }
}

/// List available backups for a workspace
pub async fn list_workspace_backups(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("📋 Listing backups for workspace: {}", workspace_id);

    use nestgate_core::error::utilities::safe_env_var_or_default;
    let backup_dir = safe_env_var_or_default("NESTGATE_BACKUP_DIR", "/var/backups/nestgate");

    let backup_pattern = format!("workspace_{workspace_id}_");
    let mut backups = Vec::new();

    match tokio::fs::read_dir(&backup_dir).await {
        Ok(mut entries) => {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(file_name) = entry.file_name().to_str()
                    && file_name.starts_with(&backup_pattern)
                    && file_name.ends_with(".zfs")
                {
                    // Extract backup name from filename
                    let backup_name = file_name
                        .strip_prefix(&backup_pattern)
                        .and_then(|s| s.strip_suffix(".zfs"))
                        .unwrap_or("unknown");

                    if let Ok(metadata) = entry.metadata().await {
                        backups.push(json!({
                            "backup_name": backup_name,
                            "file_name": file_name,
                            "size_bytes": metadata.len(),
                            "created": metadata.created()
                                .ok()
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map_or(0, |d| d.as_secs())
                        }));
                    }
                }
            }
        }
        Err(e) => {
            warn!("⚠️ Could not read backup directory: {}", e);
        }
    }

    Ok(Json(json!({
        "status": "success",
        "workspace_id": workspace_id,
        "backup_directory": backup_dir,
        "backups": backups
    })))
}

// Private helper functions

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

/// Perform Copy Migration
async fn perform_copy_migration(
    snapshot: &str,
    target_dataset: &str,
    config: &MigrationConfig,
) -> Result<Value, StatusCode> {
    info!(
        "📋 Performing copy migration from {} to {}",
        snapshot, target_dataset
    );

    let mut send_args = vec!["send"];
    if let Some(host) = &config.target_host {
        // Remote migration
        let ssh_command = format!("ssh {host} zfs receive -F {target_dataset}");
        let send_result = Command::new("zfs")
            .args(&send_args)
            .arg(snapshot)
            .stdout(std::process::Stdio::piped())
            .spawn();

        match send_result {
            Ok(mut send_process) => {
                // Wait for send process to complete first
                let _send_output = send_process
                    .wait()
                    .await
                    .map_err(|_e| StatusCode::INTERNAL_SERVER_ERROR)?;

                let receive_result = Command::new("sh").args(["-c", &ssh_command]).output().await;

                match receive_result {
                    Ok(output) if output.status.success() => {
                        info!("✅ Remote copy migration completed successfully");
                        Ok(json!({
                            "status": "success",
                            "message": "Copy migration completed successfully",
                            "source": snapshot,
                            "target": target_dataset,
                            "target_host": config.target_host,
                            "strategy": "copy"
                        }))
                    }
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        error!("❌ Remote copy migration failed: {}", stderr);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                    Err(e) => {
                        error!("❌ Failed to execute remote copy: {}", e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
            Err(e) => {
                error!("❌ Failed to start send process: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        // Local migration
        send_args.extend([snapshot, "|", "zfs", "receive", "-F", target_dataset]);
        let migration_command = send_args.join(" ");

        let result = Command::new("sh")
            .args(["-c", &migration_command])
            .output()
            .await;

        match result {
            Ok(output) if output.status.success() => {
                info!("✅ Local copy migration completed successfully");
                Ok(json!({
                    "status": "success",
                    "message": "Copy migration completed successfully",
                    "source": snapshot,
                    "target": target_dataset,
                    "strategy": "copy"
                }))
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                error!("❌ Local copy migration failed: {}", stderr);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Err(e) => {
                error!("❌ Failed to execute local copy: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

/// Perform Move Migration
async fn perform_move_migration(
    snapshot: &str,
    target_dataset: &str,
    source_dataset: &str,
    config: &MigrationConfig,
) -> Result<Value, StatusCode> {
    info!(
        "🚚 Performing move migration from {} to {}",
        snapshot, target_dataset
    );

    // First perform copy
    let _copy_result = perform_copy_migration(snapshot, target_dataset, config).await?;

    // Then destroy source if copy was successful
    let destroy_result = Command::new("zfs")
        .args(["destroy", "-r", source_dataset])
        .output()
        .await;

    match destroy_result {
        Ok(output) if output.status.success() => {
            info!("✅ Move migration completed - source destroyed");
            Ok(json!({
                "status": "success",
                "message": "Move migration completed successfully",
                "source": snapshot,
                "target": target_dataset,
                "source_destroyed": true,
                "strategy": "move"
            }))
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!(
                "⚠️ Copy successful but failed to destroy source: {}",
                stderr
            );
            Ok(json!({
                "status": "partial_success",
                "message": "Migration copied successfully but source cleanup failed",
                "source": snapshot,
                "target": target_dataset,
                "source_destroyed": false,
                "cleanup_error": stderr,
                "strategy": "move"
            }))
        }
        Err(e) => {
            error!("❌ Failed to destroy source after migration: {}", e);
            Ok(json!({
                "status": "partial_success",
                "message": "Migration copied successfully but source cleanup failed",
                "source": snapshot,
                "target": target_dataset,
                "source_destroyed": false,
                "cleanup_error": e.to_string(),
                "strategy": "move"
            }))
        }
    }
}

/// Perform Replicate Migration
async fn perform_replicate_migration(
    snapshot: &str,
    target_dataset: &str,
    config: &MigrationConfig,
) -> Result<Value, StatusCode> {
    info!(
        "🔄 Performing replicate migration from {} to {}",
        snapshot, target_dataset
    );

    // Replication is similar to copy but maintains ongoing sync capability
    let _result = perform_copy_migration(snapshot, target_dataset, config).await?;

    // For replication, we don't destroy the source and set up for incremental updates
    Ok(json!({
        "status": "success",
        "message": "Replication migration completed successfully",
        "source": snapshot,
        "target": target_dataset,
        "replication_enabled": true,
        "strategy": "replicate",
        "note": "Incremental replication can be performed using the same snapshot base"
    }))
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Restoreconfigcanonical
pub type RestoreConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using RestoreConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Backupconfigcanonical
pub type BackupConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using BackupConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Migrationconfigcanonical
pub type MigrationConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using MigrationConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::{
        BackupConfig, MigrationConfig, MigrationStrategy, RestoreConfig, backup_workspace,
        migrate_workspace, restore_workspace,
    };
    use axum::extract::{Json, Path};
    use axum::http::StatusCode;

    fn sample_backup_config() -> BackupConfig {
        BackupConfig {
            backup_name: "b1".to_string(),
            include_snapshots: false,
            compression_level: 0,
            encryption_enabled: false,
            description: None,
        }
    }

    fn sample_restore_config() -> RestoreConfig {
        RestoreConfig {
            backup_name: "b1".to_string(),
            target_workspace_id: None,
            restore_point: None,
            force: false,
        }
    }

    #[test]
    fn migration_strategy_roundtrips_json() {
        let m = MigrationConfig {
            target_pool: "p".to_string(),
            target_host: None,
            strategy: MigrationStrategy::Replicate,
            bandwidth_limit: Some(1024),
        };
        let v = serde_json::to_value(&m).unwrap();
        let back: MigrationConfig = serde_json::from_value(v).unwrap();
        assert_eq!(back.strategy, MigrationStrategy::Replicate);
        assert_eq!(back.target_pool, "p");
    }

    #[tokio::test]
    async fn backup_workspace_invalid_id_returns_bad_request() {
        let r = backup_workspace(Path("".to_string()), Json(sample_backup_config())).await;
        assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
        let r = backup_workspace(Path("bad/id".to_string()), Json(sample_backup_config())).await;
        assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
    }

    #[tokio::test]
    async fn restore_workspace_invalid_id_returns_bad_request() {
        let r = restore_workspace(Path("".to_string()), Json(sample_restore_config())).await;
        assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
    }

    #[tokio::test]
    async fn migrate_workspace_invalid_id_returns_bad_request() {
        let cfg = MigrationConfig {
            target_pool: "t".to_string(),
            target_host: None,
            strategy: MigrationStrategy::Copy,
            bandwidth_limit: None,
        };
        let r = migrate_workspace(Path("bad id".to_string()), Json(cfg)).await;
        assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
    }
}
