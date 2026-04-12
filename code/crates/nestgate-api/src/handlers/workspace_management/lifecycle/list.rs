// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use axum::{extract::Json, extract::Path, http::StatusCode};
use nestgate_types::{EnvSource, ProcessEnv, env_var_or_default};
use serde_json::{Value, json};
use tracing::{info, warn};

/// List available backups for a workspace
pub async fn list_workspace_backups(path: Path<String>) -> Result<Json<Value>, StatusCode> {
    list_workspace_backups_from_env_source(&ProcessEnv, path).await
}

/// Like [`list_workspace_backups`], but resolves `NESTGATE_BACKUP_DIR` from `env`.
pub async fn list_workspace_backups_from_env_source(
    env: &(impl EnvSource + ?Sized),
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("📋 Listing backups for workspace: {}", workspace_id);

    let backup_dir = env_var_or_default(env, "NESTGATE_BACKUP_DIR", "/var/backups/nestgate");

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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Path;
    use nestgate_types::MapEnv;

    #[tokio::test]
    async fn list_workspace_backups_empty_when_backup_dir_exists_but_has_no_matches() {
        let dir = tempfile::tempdir().expect("temp backup dir");
        let path = dir.path().to_str().expect("utf8 temp path");
        let env = MapEnv::from([("NESTGATE_BACKUP_DIR", path)]);
        let Json(v) = list_workspace_backups_from_env_source(&env, Path("ws1".to_string()))
            .await
            .expect("list_workspace_backups");
        assert_eq!(v.get("status").and_then(|s| s.as_str()), Some("success"));
        assert_eq!(v.get("workspace_id").and_then(|s| s.as_str()), Some("ws1"));
        let backups = v.get("backups").and_then(|b| b.as_array());
        assert!(backups.is_some_and(Vec::is_empty));
    }

    #[tokio::test]
    async fn list_workspace_backups_finds_matching_zfs_files() {
        let dir = tempfile::tempdir().expect("temp backup dir");
        let path = dir.path().to_str().expect("utf8 temp path");
        let file_path = dir.path().join("workspace_acme_mybackup.zfs");
        tokio::fs::write(&file_path, b"snapshot-data")
            .await
            .expect("write fake backup");
        let env = MapEnv::from([("NESTGATE_BACKUP_DIR", path)]);
        let Json(v) = list_workspace_backups_from_env_source(&env, Path("acme".to_string()))
            .await
            .expect("list_workspace_backups");
        assert_eq!(v.get("status").and_then(|s| s.as_str()), Some("success"));
        let backups = v
            .get("backups")
            .and_then(|b| b.as_array())
            .expect("backups array");
        assert_eq!(backups.len(), 1);
        let first = backups.first().expect("one backup");
        assert_eq!(
            first.get("backup_name").and_then(|s| s.as_str()),
            Some("mybackup")
        );
        assert_eq!(
            first.get("file_name").and_then(|s| s.as_str()),
            Some("workspace_acme_mybackup.zfs")
        );
    }

    #[tokio::test]
    async fn list_workspace_backups_ok_with_default_backup_dir_env() {
        let Json(v) = list_workspace_backups(Path("any-id".to_string()))
            .await
            .expect("list ok");
        assert_eq!(v.get("status").and_then(|s| s.as_str()), Some("success"));
        assert_eq!(
            v.get("workspace_id").and_then(|s| s.as_str()),
            Some("any-id")
        );
    }
}
