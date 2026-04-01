// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use axum::{extract::Json, extract::Path, http::StatusCode};
use nestgate_core::error::utilities::safe_env_var_or_default;
use serde_json::{Value, json};
use tracing::{info, warn};

/// List available backups for a workspace
pub async fn list_workspace_backups(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("📋 Listing backups for workspace: {}", workspace_id);

    let backup_dir = safe_env_var_or_default("NESTGATE_BACKUP_DIR", "/var/backups/nestgate");

    let backup_pattern = format!("workspace_{workspace_id}_");
    let mut backups = Vec::new();

    match tokio::fs::read_dir(&backup_dir).await {
        Ok(mut entries) => {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(file_name) = entry.file_name().to_str()
                    && file_name.starts_with(&backup_pattern)
                    && std::path::Path::new(file_name)
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("zfs"))
                {
                    // Extract backup name from filename
                    let backup_name = file_name
                        .strip_prefix(&backup_pattern)
                        .and_then(|rest| {
                            std::path::Path::new(rest)
                                .file_stem()
                                .and_then(|stem| stem.to_str())
                        })
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
