// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Template creation and application: JSON templates are stored on local disk under
// `NESTGATE_WORKSPACE_TEMPLATES_DIR`. Applying to live ZFS datasets remains orchestration.

use axum::{Json, extract::Path, http::StatusCode};
use nestgate_types::{EnvSource, ProcessEnv, env_var_or_default};
use serde_json::{Value, json};
use std::path::PathBuf;
use tracing::info;

const APPLY_MSG: &str = "Applying a template to live ZFS datasets (properties, quotas, clones, \
    zfs receive) requires workspace lifecycle orchestration and validated inputs. Use lifecycle \
    APIs and automation once wired; template JSON is stored locally via create_workspace_template.";

/// Returns `None` if `id` is unsafe for use as a single path segment.
fn sanitize_workspace_template_id(id: &str) -> Option<&str> {
    if id.is_empty() || id.len() > 256 {
        return None;
    }
    if id.contains("..") || id.contains('/') || id.contains('\\') {
        return None;
    }
    if !id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
    {
        return None;
    }
    Some(id)
}

/// Create workspace template (JSON on disk)
pub fn create_workspace_template(path: Path<String>) -> (StatusCode, Json<Value>) {
    create_workspace_template_from_env_source(&ProcessEnv, path)
}

/// Like [`create_workspace_template`], but resolves `NESTGATE_WORKSPACE_TEMPLATES_DIR` from `env`.
pub fn create_workspace_template_from_env_source(
    env: &(impl EnvSource + ?Sized),
    Path(workspace_id): Path<String>,
) -> (StatusCode, Json<Value>) {
    info!(
        "Template creation requested for workspace: {}",
        workspace_id
    );

    let Some(safe) = sanitize_workspace_template_id(&workspace_id) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "invalid_workspace_id",
                "message": "workspace_id must be non-empty, at most 256 characters, and use only [a-zA-Z0-9._-] with no path separators",
            })),
        );
    };

    let base = env_var_or_default(
        env,
        "NESTGATE_WORKSPACE_TEMPLATES_DIR",
        "/var/lib/nestgate/workspace_templates",
    );
    let dir = PathBuf::from(base);

    if let Err(e) = std::fs::create_dir_all(&dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "template_storage_unavailable",
                "message": format!("could not create template directory: {e}"),
            })),
        );
    }

    let path = dir.join(format!("{safe}.json"));
    let payload = json!({
        "schema_version": 1,
        "workspace_id": workspace_id,
        "template": {
            "description": "NestGate workspace template scaffold (JSON on local disk)",
            "defaults": {
                "compression": "lz4",
                "quota": "10G"
            }
        }
    });

    let text = match serde_json::to_string_pretty(&payload) {
        Ok(t) => t,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "serialization",
                    "message": e.to_string(),
                })),
            );
        }
    };

    if let Err(e) = std::fs::write(&path, text) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "template_write_failed",
                "message": e.to_string(),
            })),
        );
    }

    (
        StatusCode::OK,
        Json(json!({
            "status": "created",
            "workspace_id": workspace_id,
            "path": path.to_string_lossy(),
            "template": payload["template"],
        })),
    )
}

/// Apply workspace template (orchestration not implemented here)
pub fn apply_workspace_template(Path(workspace_id): Path<String>) -> (StatusCode, Json<Value>) {
    info!(
        "Template application requested for workspace: {}",
        workspace_id
    );
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "workspace_template_apply_not_available",
            "message": APPLY_MSG,
            "workspace_id": workspace_id,
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use nestgate_types::MapEnv;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_create_workspace_template_rejects_invalid_id() {
        let (status, _) = create_workspace_template(Path("../escape".to_string()));
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_create_workspace_template_writes_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().to_str().expect("utf8 path");
        let env = MapEnv::from([("NESTGATE_WORKSPACE_TEMPLATES_DIR", path)]);
        let (status, Json(body)) =
            create_workspace_template_from_env_source(&env, Path("my-ws-01".to_string()));
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"].as_str(), Some("created"));
        let p = PathBuf::from(path).join("my-ws-01.json");
        assert!(p.exists());
        let raw = fs::read_to_string(p).expect("read template");
        assert!(raw.contains("schema_version"));
    }

    #[test]
    fn test_apply_workspace_template_not_implemented() {
        let (status, Json(body)) = apply_workspace_template(Path("test-workspace".to_string()));
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(
            body["error"].as_str(),
            Some("workspace_template_apply_not_available")
        );
    }
}
