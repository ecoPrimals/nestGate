// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! footPrint persistence — CAS-backed project save and delete.
//!
//! ## CAS layout
//!
//! ```text
//! {base}/datasets/{family}/_footprint/manifest.json
//! ```
//!
//! Revision content is stored in `_content/` via the existing CAS pipeline
//! (BLAKE3 hash, 2-char prefix sharding, `.meta.json` sidecars). This gives
//! footPrint projects automatic deduplication, federation, HTTP serving, and
//! provenance sidecars for free.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::{debug, info};

use super::super::StorageState;
use super::super::storage_paths::{content_cas_path, content_hash_hex, resolve_family_id};
use super::types::{FootPrintManifest, FootPrintProject, ProjectRevision};

/// Resolve the footprint storage root for a given family.
///
/// Priority:
/// 1. `PROJECTS_PATH` env var (footPrint composition wiring)
/// 2. Standard CAS layout: `{storage_base}/datasets/{family}/_footprint`
fn footprint_base_path(family_id: &str) -> std::path::PathBuf {
    if let Ok(projects_path) = std::env::var("PROJECTS_PATH") {
        if !projects_path.is_empty() {
            return std::path::PathBuf::from(projects_path)
                .join(family_id)
                .join("_footprint");
        }
    }
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_footprint")
}

pub(super) fn save_manifest(family_id: &str, manifest: &FootPrintManifest) -> Result<()> {
    let dir = footprint_base_path(family_id);
    std::fs::create_dir_all(&dir).map_err(|e| {
        NestGateError::io_error(format!("failed to create footprint directory: {e}"))
    })?;
    let path = dir.join("manifest.json");
    let data = serde_json::to_string_pretty(manifest).map_err(|e| {
        NestGateError::io_error(format!("failed to serialize footprint manifest: {e}"))
    })?;
    std::fs::write(&path, data).map_err(|e| {
        NestGateError::io_error(format!("failed to write footprint manifest: {e}"))
    })?;
    Ok(())
}

pub(super) fn load_manifest(family_id: &str) -> Result<FootPrintManifest> {
    let manifest_path = footprint_base_path(family_id).join("manifest.json");
    if !manifest_path.exists() {
        return Ok(FootPrintManifest::new());
    }
    let data = std::fs::read_to_string(&manifest_path).map_err(|e| {
        NestGateError::io_error(format!("failed to read footprint manifest: {e}"))
    })?;
    serde_json::from_str(&data).map_err(|e| {
        NestGateError::invalid_input_with_field(
            "manifest",
            format!("corrupt footprint manifest: {e}"),
        )
    })
}

fn store_content_cas(family_id: &str, data: &[u8]) -> Result<String> {
    let hash = content_hash_hex(data);
    let cas_path = content_cas_path(family_id, &hash);

    if cas_path.exists() {
        debug!("footprint.save: CAS dedup hit for {}", &hash[..12]);
        return Ok(hash);
    }

    if let Some(parent) = cas_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            NestGateError::io_error(format!("failed to create CAS shard directory: {e}"))
        })?;
    }
    std::fs::write(&cas_path, data).map_err(|e| {
        NestGateError::io_error(format!("failed to write CAS object {}: {e}", &hash[..12]))
    })?;

    let meta = json!({
        "content_type": "application/json",
        "source": "footprint.save",
        "stored_by": "nestgate",
        "size": data.len(),
    });
    let meta_path = cas_path.with_extension("meta.json");
    if let Ok(meta_str) = serde_json::to_string_pretty(&meta) {
        let _ = std::fs::write(&meta_path, meta_str);
    }

    Ok(hash)
}

/// `footprint.save` — save or update a project with a new revision.
///
/// Creates the project if it doesn't exist, or appends a new revision.
/// Revision content is stored in `_content/` via BLAKE3 CAS.
///
/// ```json
/// {
///   "project_id": "my-portfolio",
///   "name": "My Portfolio",
///   "content_base64": "eyJwYWdlcyI6W119",
///   "message": "Add contact page",
///   "metadata": {"tags": ["portfolio", "personal"]}
/// }
/// ```
pub async fn footprint_save(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;

    let project_id = params["project_id"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("project_id", "project_id required")
        })?;

    if project_id.is_empty() || project_id.len() > 128 {
        return Err(NestGateError::invalid_input_with_field(
            "project_id",
            "project_id must be 1-128 characters",
        ));
    }

    let content_b64 = params["content_base64"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "content_base64",
                "content_base64 required (base64-encoded project snapshot)",
            )
        })?;

    let raw = STANDARD.decode(content_b64).map_err(|e| {
        NestGateError::invalid_input_with_field(
            "content_base64",
            format!("invalid base64: {e}"),
        )
    })?;

    let message = params["message"]
        .as_str()
        .unwrap_or("Save project")
        .to_owned();

    let name = params["name"]
        .as_str()
        .unwrap_or(project_id)
        .to_owned();

    let metadata = params.get("metadata").cloned().unwrap_or_else(|| json!({}));

    let hash = store_content_cas(family_id, &raw)?;

    let mut manifest = load_manifest(family_id)?;
    let now = chrono::Utc::now().to_rfc3339();

    let project = manifest
        .projects
        .entry(project_id.to_owned())
        .or_insert_with(|| {
            manifest.project_count += 1;
            FootPrintProject::new(project_id.to_owned(), name.clone(), metadata.clone())
        });

    if params["name"].is_string() {
        project.name = name;
    }
    if params.get("metadata").is_some() {
        project.metadata = metadata;
    }

    let parent = project.current_revision.clone();

    let revision = ProjectRevision {
        hash: hash.clone(),
        message: message.clone(),
        saved_at: now.clone(),
        parent,
        size: raw.len() as u64,
        spine_index: None,
        braid_id: None,
    };

    let is_new_revision = !project.revisions.contains_key(&hash);
    project.revisions.insert(hash.clone(), revision);
    project.current_revision = Some(hash.clone());
    if is_new_revision {
        project.revision_history.insert(0, hash.clone());
    }
    project.updated_at = now;

    manifest.updated_at = chrono::Utc::now().to_rfc3339();
    manifest.version += 1;
    save_manifest(family_id, &manifest)?;

    info!(
        "footprint.save: project={} revision={} size={}B",
        project_id,
        &hash[..12],
        raw.len()
    );

    Ok(json!({
        "project_id": project_id,
        "hash": hash,
        "message": message,
        "size": raw.len(),
        "revision_count": manifest.projects.get(project_id).map_or(0, |p| p.revision_history.len()),
        "manifest_version": manifest.version,
    }))
}

/// `footprint.delete` — soft-delete a project from the manifest.
///
/// The project is removed from the manifest index. Revision content
/// remains in CAS (immutable) and can be recovered if the hash is known.
///
/// ```json
/// { "project_id": "my-portfolio" }
/// ```
pub async fn footprint_delete(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;

    let project_id = params["project_id"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("project_id", "project_id required")
        })?;

    let mut manifest = load_manifest(family_id)?;
    let removed = manifest.projects.remove(project_id);
    if removed.is_none() {
        return Err(NestGateError::invalid_input_with_field(
            "project_id",
            format!("project {project_id} not found"),
        ));
    }

    manifest.project_count = u32::try_from(manifest.projects.len()).unwrap_or(u32::MAX);
    manifest.updated_at = chrono::Utc::now().to_rfc3339();
    manifest.version += 1;
    save_manifest(family_id, &manifest)?;

    info!("footprint.delete: removed project={}", project_id);

    Ok(json!({
        "project_id": project_id,
        "deleted": true,
        "manifest_version": manifest.version,
        "note": "Revision content remains in CAS (immutable). Recover via content.get if hash is known."
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn footprint_base_path_layout() {
        let path = footprint_base_path("test-fam");
        assert!(path.ends_with("_footprint"));
        assert!(path.to_string_lossy().contains("test-fam"));
    }

    #[test]
    #[serial]
    fn footprint_base_path_respects_projects_path_env() {
        temp_env::with_vars([("PROJECTS_PATH", Some("/custom/projects"))], || {
            let path = footprint_base_path("my-family");
            assert_eq!(
                path,
                std::path::PathBuf::from("/custom/projects/my-family/_footprint")
            );
        });
    }

    #[test]
    #[serial]
    fn footprint_base_path_ignores_empty_projects_path() {
        temp_env::with_vars([("PROJECTS_PATH", Some(""))], || {
            let path = footprint_base_path("fam");
            assert!(!path.starts_with("/_footprint"));
            assert!(path.to_string_lossy().contains("datasets"));
        });
    }

    #[test]
    #[serial]
    fn footprint_base_path_falls_back_when_projects_path_unset() {
        temp_env::with_vars([("PROJECTS_PATH", None::<&str>)], || {
            let path = footprint_base_path("fam");
            assert!(path.to_string_lossy().contains("datasets"));
            assert!(path.to_string_lossy().contains("fam"));
            assert!(path.ends_with("_footprint"));
        });
    }

    #[test]
    fn manifest_roundtrip() {
        let mut m = FootPrintManifest::new();
        let project = FootPrintProject::new(
            "test".into(),
            "Test".into(),
            json!({}),
        );
        m.projects.insert("test".into(), project);
        m.project_count = 1;

        let json_str = serde_json::to_string(&m).expect("serialize");
        let back: FootPrintManifest = serde_json::from_str(&json_str).expect("deserialize");
        assert_eq!(back.project_count, 1);
        assert!(back.projects.contains_key("test"));
    }
}
