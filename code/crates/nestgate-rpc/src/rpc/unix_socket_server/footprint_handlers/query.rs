// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! footPrint query handlers — read-only access to projects and revisions.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

use super::super::StorageState;
use super::super::storage_paths::{content_cas_path, resolve_family_id};
use super::ingest::load_manifest;

/// `footprint.list` — list all projects (sorted by `updated_at` descending).
///
/// Optional params:
/// ```json
/// { "limit": 50, "offset": 0 }
/// ```
pub async fn footprint_list(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let p = params.unwrap_or(&default_params);
    let family_id = resolve_family_id(p, state)?;
    let manifest = load_manifest(family_id)?;

    let limit = usize::try_from(p["limit"].as_u64().unwrap_or(100)).unwrap_or(100);
    let offset = usize::try_from(p["offset"].as_u64().unwrap_or(0)).unwrap_or(0);

    let mut projects: Vec<&_> = manifest.projects.values().collect();
    projects.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    let page: Vec<Value> = projects
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|p| {
            json!({
                "project_id": p.project_id,
                "name": p.name,
                "created_at": p.created_at,
                "updated_at": p.updated_at,
                "current_revision": p.current_revision,
                "revision_count": p.revision_history.len(),
                "metadata": p.metadata,
            })
        })
        .collect();

    Ok(json!({
        "count": manifest.project_count,
        "limit": limit,
        "offset": offset,
        "projects": page,
    }))
}

/// `footprint.get` — retrieve a project and optionally its current content.
///
/// ```json
/// { "project_id": "my-portfolio", "include_content": true }
/// ```
pub async fn footprint_get(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;
    let manifest = load_manifest(family_id)?;

    let project_id = params["project_id"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("project_id", "project_id required")
        })?;

    let project = manifest.projects.get(project_id).ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "project_id",
            format!("project {project_id} not found"),
        )
    })?;

    let include_content = params["include_content"].as_bool().unwrap_or(false);
    let revision_hash = params["revision"]
        .as_str()
        .map(ToOwned::to_owned)
        .or_else(|| project.current_revision.clone());

    let mut response = json!({
        "project_id": project.project_id,
        "name": project.name,
        "created_at": project.created_at,
        "updated_at": project.updated_at,
        "current_revision": project.current_revision,
        "revision_count": project.revision_history.len(),
        "metadata": project.metadata,
    });

    if let Some(ref hash) = revision_hash {
        if let Some(rev) = project.revisions.get(hash) {
            response["revision"] = json!(rev);
        }

        if include_content {
            let cas_path = content_cas_path(family_id, hash);
            if cas_path.exists() {
                match std::fs::read(&cas_path) {
                    Ok(data) => {
                        response["content_base64"] = json!(STANDARD.encode(&data));
                    }
                    Err(e) => {
                        debug!(
                            "footprint.get: failed to read CAS object {}: {e}",
                            &hash[..12]
                        );
                        response["content_error"] =
                            json!(format!("failed to read revision content: {e}"));
                    }
                }
            } else {
                response["content_error"] =
                    json!(format!("CAS object {} not found on this gate", &hash[..12]));
            }
        }
    }

    Ok(response)
}

/// `footprint.history` — list revision history for a project.
///
/// ```json
/// { "project_id": "my-portfolio", "limit": 20 }
/// ```
pub async fn footprint_history(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;
    let manifest = load_manifest(family_id)?;

    let project_id = params["project_id"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("project_id", "project_id required")
        })?;

    let project = manifest.projects.get(project_id).ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "project_id",
            format!("project {project_id} not found"),
        )
    })?;

    let limit = usize::try_from(params["limit"].as_u64().unwrap_or(50)).unwrap_or(50);

    let revisions: Vec<Value> = project
        .revision_history
        .iter()
        .take(limit)
        .filter_map(|hash| {
            project.revisions.get(hash).map(|rev| {
                json!({
                    "hash": rev.hash,
                    "message": rev.message,
                    "saved_at": rev.saved_at,
                    "parent": rev.parent,
                    "size": rev.size,
                    "spine_index": rev.spine_index,
                    "braid_id": rev.braid_id,
                })
            })
        })
        .collect();

    Ok(json!({
        "project_id": project_id,
        "current_revision": project.current_revision,
        "revision_count": project.revision_history.len(),
        "limit": limit,
        "revisions": revisions,
    }))
}

#[cfg(test)]
mod tests {
    use super::super::types::{FootPrintManifest, FootPrintProject, ProjectRevision};
    use std::collections::BTreeMap;

    #[test]
    fn project_with_revisions_serializes() {
        let mut project = FootPrintProject::new(
            "test".into(),
            "Test Project".into(),
            serde_json::json!({}),
        );
        let rev = ProjectRevision {
            hash: "abc123".into(),
            message: "Initial".into(),
            saved_at: "2026-07-12T00:00:00Z".into(),
            parent: None,
            size: 42,
            spine_index: None,
            braid_id: None,
        };
        project.revisions.insert("abc123".into(), rev);
        project.revision_history.push("abc123".into());
        project.current_revision = Some("abc123".into());

        let json = serde_json::to_string(&project).expect("serialize");
        assert!(json.contains("abc123"));
        assert!(json.contains("Initial"));
    }

    #[test]
    fn empty_manifest_has_no_projects() {
        let m = FootPrintManifest::new();
        assert!(m.projects.is_empty());
        assert_eq!(m.project_count, 0);
    }

    #[test]
    fn multiple_projects_sort_by_id() {
        let mut m = FootPrintManifest::new();
        m.projects.insert(
            "beta".into(),
            FootPrintProject::new("beta".into(), "Beta".into(), serde_json::json!({})),
        );
        m.projects.insert(
            "alpha".into(),
            FootPrintProject::new("alpha".into(), "Alpha".into(), serde_json::json!({})),
        );
        let keys: Vec<&String> = m.projects.keys().collect();
        assert_eq!(keys, &["alpha", "beta"]);
    }
}
