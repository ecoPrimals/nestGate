// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Coordination query handlers — read-only access to ingested artifacts.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

use super::super::StorageState;
use super::super::storage_paths::resolve_family_id;
use super::types::{ArtifactKind, CoordManifest};

fn coord_base_path(family_id: &str) -> std::path::PathBuf {
    let base = get_storage_base_path();
    base.join("datasets").join(family_id).join("_coordination")
}

fn load_manifest(family_id: &str) -> Result<CoordManifest> {
    let manifest_path = coord_base_path(family_id).join("manifest.json");
    if !manifest_path.exists() {
        return Ok(CoordManifest::new());
    }
    let data = std::fs::read_to_string(&manifest_path).map_err(|e| {
        NestGateError::io_error(format!("failed to read coordination manifest: {e}"))
    })?;
    serde_json::from_str(&data).map_err(|e| {
        NestGateError::invalid_input_with_field(
            "manifest",
            format!("corrupt coordination manifest: {e}"),
        )
    })
}

fn load_artifact_content(family_id: &str, hash: &str) -> Result<String> {
    let artifact_path = coord_base_path(family_id).join("artifacts").join(hash);
    if !artifact_path.exists() {
        return Err(NestGateError::invalid_input_with_field(
            "hash",
            format!("artifact {hash} not found"),
        ));
    }
    std::fs::read_to_string(&artifact_path).map_err(|e| {
        NestGateError::io_error(format!("failed to read artifact {hash}: {e}"))
    })
}

/// `coord.blurbs.current` — return the current wave blurb.
pub async fn coord_blurbs_current(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&default_params), state)?;
    let manifest = load_manifest(family_id)?;

    let Some(ref hash) = manifest.current_blurb else {
        return Ok(json!({
            "status": "no_blurb",
            "message": "No blurb has been ingested yet. Run coord.ingest to populate."
        }));
    };

    let artifact = manifest.artifacts.get(hash);
    let content = load_artifact_content(family_id, hash)?;

    Ok(json!({
        "hash": hash,
        "content": content,
        "artifact": artifact,
        "wave": artifact.and_then(|a| a.wave.as_deref()),
    }))
}

/// `coord.blurbs.list` — list all ingested blurbs (newest first).
pub async fn coord_blurbs_list(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&default_params), state)?;
    let manifest = load_manifest(family_id)?;
    let blurbs: Vec<&_> = manifest
        .blurb_history
        .iter()
        .filter_map(|h| manifest.artifacts.get(h))
        .collect();

    Ok(json!({
        "count": blurbs.len(),
        "current": manifest.current_blurb,
        "blurbs": blurbs,
    }))
}

/// `coord.blurbs.get` — retrieve a specific blurb by hash or wave number.
pub async fn coord_blurbs_get(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;
    let manifest = load_manifest(family_id)?;

    let hash = if let Some(h) = params["hash"].as_str() {
        h.to_owned()
    } else if let Some(wave) = params["wave"].as_str() {
        manifest
            .artifacts
            .values()
            .find(|a| a.kind == ArtifactKind::Blurb && a.wave.as_deref() == Some(wave))
            .map(|a| a.hash.clone())
            .ok_or_else(|| {
                NestGateError::invalid_input_with_field("wave", format!("no blurb for wave {wave}"))
            })?
    } else {
        return Err(NestGateError::invalid_input_with_field(
            "hash",
            "hash or wave parameter required",
        ));
    };

    let content = load_artifact_content(family_id, &hash)?;
    let artifact = manifest.artifacts.get(&hash);

    Ok(json!({
        "hash": hash,
        "content": content,
        "artifact": artifact,
    }))
}

/// `coord.fragos.list` — list all FRAGOs and AARs (newest first).
pub async fn coord_fragos_list(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&default_params), state)?;
    let manifest = load_manifest(family_id)?;
    let fragos: Vec<&_> = manifest
        .frago_history
        .iter()
        .filter_map(|h| manifest.artifacts.get(h))
        .collect();

    Ok(json!({
        "count": fragos.len(),
        "fragos": fragos,
    }))
}

/// `coord.fragos.get` — retrieve a specific FRAGO/AAR by hash.
pub async fn coord_fragos_get(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;
    let hash = params["hash"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("hash", "hash parameter required")
    })?;

    let content = load_artifact_content(family_id, hash)?;
    let manifest = load_manifest(family_id)?;
    let artifact = manifest.artifacts.get(hash);

    Ok(json!({
        "hash": hash,
        "content": content,
        "artifact": artifact,
    }))
}

/// `coord.waves.current` — return the current wave state.
pub async fn coord_waves_current(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&default_params), state)?;
    let manifest = load_manifest(family_id)?;

    let Some(ref hash) = manifest.current_wave else {
        return Ok(json!({
            "status": "no_wave",
            "message": "No wave state has been ingested yet."
        }));
    };

    let content = load_artifact_content(family_id, hash)?;
    let artifact = manifest.artifacts.get(hash);

    Ok(json!({
        "hash": hash,
        "content": content,
        "artifact": artifact,
    }))
}

/// `coord.waves.history` — return wave history from the coordination manifest.
///
/// Full provenance history requires loamSpine IPC (`entry.list` on the
/// coordination spine). When loamSpine is unavailable, falls back to the
/// local manifest's blurb history as a wave timeline.
pub async fn coord_waves_history(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&default_params), state)?;
    let manifest = load_manifest(family_id)?;

    let waves: Vec<Value> = manifest
        .blurb_history
        .iter()
        .filter_map(|h| {
            manifest.artifacts.get(h).map(|a| {
                json!({
                    "wave": a.wave,
                    "hash": a.hash,
                    "title": a.title,
                    "ingested_at": a.ingested_at,
                    "spine_index": a.spine_index,
                    "braid_id": a.braid_id,
                })
            })
        })
        .collect();

    Ok(json!({
        "count": waves.len(),
        "waves": waves,
        "source": "manifest",
        "note": "Full provenance requires a ledger capability provider (spine.list IPC)"
    }))
}

/// `coord.heads.get` — return a specific gate's HEAD state.
pub async fn coord_heads_get(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;
    let gate = params["gate"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("gate", "gate parameter required")
    })?;

    let manifest = load_manifest(family_id)?;
    let Some(hash) = manifest.heads.get(gate) else {
        return Ok(json!({
            "gate": gate,
            "status": "not_found",
            "message": format!("No HEAD state for gate {gate}")
        }));
    };

    let content = load_artifact_content(family_id, hash)?;
    let artifact = manifest.artifacts.get(hash);

    Ok(json!({
        "gate": gate,
        "hash": hash,
        "content": content,
        "artifact": artifact,
    }))
}

/// `coord.heads.all` — return all gate HEAD states.
pub async fn coord_heads_all(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&default_params), state)?;
    let manifest = load_manifest(family_id)?;

    let heads: Vec<Value> = manifest
        .heads
        .iter()
        .map(|(gate, hash)| {
            let artifact = manifest.artifacts.get(hash);
            json!({
                "gate": gate,
                "hash": hash,
                "artifact": artifact,
            })
        })
        .collect();

    Ok(json!({
        "count": heads.len(),
        "heads": heads,
    }))
}

/// `coord.topology` — return mesh topology from gate heads + static config.
pub async fn coord_topology(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&default_params), state)?;
    let manifest = load_manifest(family_id)?;

    let gates: Vec<&str> = manifest.heads.keys().map(String::as_str).collect();

    debug!("coord.topology: {} gates with HEAD state", gates.len());

    Ok(json!({
        "gates": gates,
        "head_count": manifest.heads.len(),
        "source": "coordination_manifest",
        "note": "Live mesh topology requires a mesh capability provider (mesh.peers IPC)"
    }))
}

/// `coord.depot.status` — return depot staleness and binary info.
pub async fn coord_depot_status(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let default_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&default_params), state)?;
    let depot_path = get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_depot");

    if !depot_path.exists() {
        let alt_path = std::env::var("ECOPRIMALS_DEPOT_PATH").map_or_else(
            |_| std::path::PathBuf::from("/opt/ecoPrimals/depot"),
            std::path::PathBuf::from,
        );
        if alt_path.exists() {
            return scan_depot(&alt_path);
        }
        return Ok(json!({
            "status": "no_depot",
            "message": "No depot directory found"
        }));
    }
    scan_depot(&depot_path)
}

fn scan_depot(depot_path: &std::path::Path) -> Result<Value> {
    let mut binaries = Vec::new();
    if let Ok(entries) = std::fs::read_dir(depot_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let metadata = std::fs::metadata(&path).ok();
                binaries.push(json!({
                    "name": name,
                    "size": metadata.as_ref().map(std::fs::Metadata::len),
                    "modified": metadata.and_then(|m| m.modified().ok())
                        .map(|t| {
                            let dt: chrono::DateTime<chrono::Utc> = t.into();
                            dt.to_rfc3339()
                        }),
                }));
            }
        }
    }
    binaries.sort_by(|a, b| {
        let an = a["name"].as_str().unwrap_or("");
        let bn = b["name"].as_str().unwrap_or("");
        an.cmp(bn)
    });

    Ok(json!({
        "path": depot_path.display().to_string(),
        "binary_count": binaries.len(),
        "binaries": binaries,
    }))
}

/// `coord.provenance` — return provenance trail for an artifact.
///
/// Full provenance requires `loamSpine` + `sweetGrass` IPC. Returns what is
/// available from the local manifest (`spine_index`, `braid_id`).
pub async fn coord_provenance(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;
    let hash = params["hash"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("hash", "hash parameter required")
    })?;

    let manifest = load_manifest(family_id)?;
    let Some(artifact) = manifest.artifacts.get(hash) else {
        return Ok(json!({
            "hash": hash,
            "status": "not_found",
        }));
    };

    Ok(json!({
        "hash": hash,
        "kind": artifact.kind,
        "title": artifact.title,
        "ingested_at": artifact.ingested_at,
        "content_hash": artifact.content_hash,
        "spine_index": artifact.spine_index,
        "braid_id": artifact.braid_id,
        "provenance_source": "manifest",
        "note": "Full provenance requires ledger (entry.get) + braid (braid.get) capability providers via IPC"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::CoordManifest;

    #[test]
    fn empty_manifest_returns_no_blurb() {
        let manifest = CoordManifest::new();
        assert!(manifest.current_blurb.is_none());
        assert!(manifest.artifacts.is_empty());
    }

    #[test]
    fn artifact_kind_from_filename() {
        assert_eq!(ArtifactKind::from_filename("ECOSYSTEM_BLURB.md"), ArtifactKind::Blurb);
        assert_eq!(ArtifactKind::from_filename("AAR_FLOCKGATE.md"), ArtifactKind::Aar);
        assert_eq!(ArtifactKind::from_filename("wave.toml"), ArtifactKind::Wave);
        assert_eq!(ArtifactKind::from_filename("eastGate.toml"), ArtifactKind::Head);
        assert_eq!(ArtifactKind::from_filename("POSTMORTEM_134h.md"), ArtifactKind::Aar);
    }

    #[test]
    fn scan_depot_nonexistent_returns_empty() {
        let result = scan_depot(std::path::Path::new("/tmp/nonexistent_depot_12345"));
        let val = result.unwrap();
        assert_eq!(val["binary_count"], 0);
    }
}
