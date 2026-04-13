// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use axum::{extract::Json, extract::Path, http::StatusCode};
use serde_json::{Value, json};
use tokio::process::Command;
use tracing::{error, info, warn};

use super::types::{MigrationConfig, MigrationStrategy};

/// Migrate workspace to different pool or host
pub async fn migrate_workspace(
    Path(workspace_id): Path<String>,
    Json(config): Json<MigrationConfig>,
) -> Result<Json<Value>, StatusCode> {
    info!(
        "Migrating workspace: {} with config: {:?}",
        workspace_id, config
    );

    // Validate workspace ID
    if workspace_id.is_empty() || workspace_id.contains('/') || workspace_id.contains(' ') {
        warn!("Invalid workspace ID format: {}", workspace_id);
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
            error!("Source workspace not found: {}", source_dataset);
            return Ok(Json(json!({
                "status": "error",
                "message": format!("Source workspace not found: {source_dataset}"),
                "workspace_id": workspace_id
            })));
        }
        Err(e) => {
            error!("Failed to check source workspace: {}", e);
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
                warn!("System time before UNIX epoch, using current timestamp");
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
            error!("Failed to create migration snapshot: {}", stderr);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Err(e) => {
            error!("Failed to create migration snapshot: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        _ => {
            info!("Created migration snapshot: {}", migration_snapshot);
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

/// Perform Copy Migration
async fn perform_copy_migration(
    snapshot: &str,
    target_dataset: &str,
    config: &MigrationConfig,
) -> Result<Value, StatusCode> {
    info!(
        "Performing copy migration from {} to {}",
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
                        info!("Remote copy migration completed successfully");
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
                        error!("Remote copy migration failed: {}", stderr);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                    Err(e) => {
                        error!("Failed to execute remote copy: {}", e);
                        Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
            }
            Err(e) => {
                error!("Failed to start send process: {}", e);
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
                info!("Local copy migration completed successfully");
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
                error!("Local copy migration failed: {}", stderr);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Err(e) => {
                error!("Failed to execute local copy: {}", e);
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
        "Performing move migration from {} to {}",
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
            info!("Move migration completed - source destroyed");
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
            warn!("Copy successful but failed to destroy source: {}", stderr);
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
            error!("Failed to destroy source after migration: {}", e);
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
        "Performing replicate migration from {} to {}",
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

#[cfg(test)]
mod tests {
    #![expect(deprecated)]

    use super::*;
    use axum::extract::{Json, Path};

    fn sample_migration_config(strategy: MigrationStrategy) -> MigrationConfig {
        MigrationConfig {
            target_pool: "targetpool".to_string(),
            target_host: None,
            strategy,
            bandwidth_limit: Some(4096),
        }
    }

    #[test]
    fn migration_strategy_serde_roundtrip_all_variants() {
        for strategy in [
            MigrationStrategy::Copy,
            MigrationStrategy::Move,
            MigrationStrategy::Replicate,
        ] {
            let v = serde_json::to_value(&strategy).expect("serialize strategy");
            let back: MigrationStrategy = serde_json::from_value(v).expect("deserialize strategy");
            assert_eq!(back, strategy);
        }
    }

    #[test]
    fn migration_config_serde_roundtrip() {
        let cfg = sample_migration_config(MigrationStrategy::Replicate);
        let s = serde_json::to_string(&cfg).expect("serialize MigrationConfig");
        let back: MigrationConfig = serde_json::from_str(&s).expect("deserialize MigrationConfig");
        assert_eq!(back.target_pool, cfg.target_pool);
        assert_eq!(back.strategy, MigrationStrategy::Replicate);
        assert_eq!(back.bandwidth_limit, cfg.bandwidth_limit);
    }

    #[tokio::test]
    async fn migrate_workspace_rejects_empty_workspace_id() {
        let r = migrate_workspace(
            Path(String::new()),
            Json(sample_migration_config(MigrationStrategy::Copy)),
        )
        .await;
        assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
    }

    #[tokio::test]
    async fn migrate_workspace_rejects_slash_in_workspace_id() {
        let r = migrate_workspace(
            Path("x/y".to_string()),
            Json(sample_migration_config(MigrationStrategy::Copy)),
        )
        .await;
        assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
    }

    #[tokio::test]
    async fn migrate_workspace_rejects_space_in_workspace_id() {
        let r = migrate_workspace(
            Path("x y".to_string()),
            Json(sample_migration_config(MigrationStrategy::Copy)),
        )
        .await;
        assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
    }
}
