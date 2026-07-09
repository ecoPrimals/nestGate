// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! HTTP coordination handlers — REST surface for the coordination backend.
//!
//! Parallel to the `coord.*` JSON-RPC methods in `nestgate-rpc`. These
//! handlers read the coordination manifest and artifacts from the same
//! filesystem paths that the RPC handlers use.

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::debug;

use nestgate_core::config::storage_paths::get_storage_base_path;

/// Query parameters shared across coordination endpoints.
#[derive(Debug, Deserialize)]
pub struct CoordQuery {
    /// Override family context (defaults to `NESTGATE_FAMILY_ID`).
    pub family_id: Option<String>,
}

fn resolve_family(query: &CoordQuery) -> String {
    query
        .family_id
        .clone()
        .or_else(|| std::env::var("NESTGATE_FAMILY_ID").ok())
        .unwrap_or_else(|| String::from("default"))
}

fn coord_base_path(family_id: &str) -> std::path::PathBuf {
    let base = get_storage_base_path();
    base.join("datasets").join(family_id).join("_coordination")
}

fn load_manifest(family_id: &str) -> Result<Value, (StatusCode, Json<Value>)> {
    let manifest_path = coord_base_path(family_id).join("manifest.json");
    if !manifest_path.exists() {
        return Ok(json!({
            "version": 0,
            "artifacts": {},
            "heads": {},
            "blurb_history": [],
            "frago_history": [],
            "note": "No coordination data ingested yet. Use coord.ingest via JSON-RPC."
        }));
    }
    let data = std::fs::read_to_string(&manifest_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("failed to read manifest: {e}")})),
        )
    })?;
    serde_json::from_str(&data).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("corrupt manifest: {e}")})),
        )
    })
}

fn load_artifact(family_id: &str, hash: &str) -> Result<String, (StatusCode, Json<Value>)> {
    let path = coord_base_path(family_id).join("artifacts").join(hash);
    if !path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": format!("artifact {hash} not found")})),
        ));
    }
    std::fs::read_to_string(&path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("failed to read artifact: {e}")})),
        )
    })
}

/// `GET /coord/blurbs` — list blurbs with current indicator.
pub async fn coord_blurbs(
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    let manifest = load_manifest(&fid)?;
    debug!(family_id = %fid, "GET /coord/blurbs");

    let current = manifest["current_blurb"].as_str();
    let blurbs = manifest["blurb_history"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let artifacts: Vec<&Value> = blurbs
        .iter()
        .filter_map(|h| h.as_str().and_then(|s| manifest["artifacts"].get(s)))
        .collect();

    Ok(Json(json!({
        "count": artifacts.len(),
        "current": current,
        "blurbs": artifacts,
    })))
}

/// `GET /coord/blurbs/:wave` — get a specific blurb by wave identifier.
pub async fn coord_blurb_by_wave(
    Path(wave): Path<String>,
    Query(query): Query<CoordQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    let manifest = load_manifest(&fid)?;
    debug!(family_id = %fid, wave = %wave, "GET /coord/blurbs/:wave");

    let artifacts = manifest["artifacts"].as_object();
    let hash = artifacts
        .and_then(|arts| {
            arts.values().find(|a| {
                a["kind"].as_str() == Some("blurb") && a["wave"].as_str() == Some(&wave)
            })
        })
        .and_then(|a| a["hash"].as_str())
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": format!("no blurb for wave {wave}")})),
            )
        })?
        .to_owned();

    let content = load_artifact(&fid, &hash)?;

    Ok(Json(json!({
        "wave": wave,
        "hash": hash,
        "content": content,
        "artifact": manifest["artifacts"].get(&hash),
    })))
}

/// `GET /coord/fragos` — list FRAGOs and AARs.
pub async fn coord_fragos(
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    let manifest = load_manifest(&fid)?;
    debug!(family_id = %fid, "GET /coord/fragos");

    let fragos = manifest["frago_history"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let artifacts: Vec<&Value> = fragos
        .iter()
        .filter_map(|h| h.as_str().and_then(|s| manifest["artifacts"].get(s)))
        .collect();

    Ok(Json(json!({
        "count": artifacts.len(),
        "fragos": artifacts,
    })))
}

/// `GET /coord/fragos/:id` — get a specific FRAGO/AAR by hash.
pub async fn coord_frago_by_id(
    Path(id): Path<String>,
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    debug!(family_id = %fid, id = %id, "GET /coord/fragos/:id");

    let content = load_artifact(&fid, &id)?;
    let manifest = load_manifest(&fid)?;

    Ok(Json(json!({
        "hash": id,
        "content": content,
        "artifact": manifest["artifacts"].get(&id),
    })))
}

/// `GET /coord/waves` — current wave state + history summary.
pub async fn coord_waves(
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    let manifest = load_manifest(&fid)?;
    debug!(family_id = %fid, "GET /coord/waves");

    let current_hash = manifest["current_wave"].as_str();
    let current_content = current_hash
        .and_then(|h| load_artifact(&fid, h).ok());

    let history: Vec<Value> = manifest["blurb_history"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|h| {
            let hash = h.as_str()?;
            let art = manifest["artifacts"].get(hash)?;
            Some(json!({
                "wave": art["wave"],
                "hash": hash,
                "title": art["title"],
                "ingested_at": art["ingested_at"],
            }))
        })
        .collect();

    Ok(Json(json!({
        "current_wave": current_hash,
        "current_content": current_content,
        "history": history,
    })))
}

/// `GET /coord/heads` — all gate HEAD states.
pub async fn coord_heads(
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    let manifest = load_manifest(&fid)?;
    debug!(family_id = %fid, "GET /coord/heads");

    let heads = manifest["heads"].as_object().cloned().unwrap_or_default();
    let entries: Vec<Value> = heads
        .iter()
        .map(|(gate, hash_val)| {
            let hash = hash_val.as_str().unwrap_or("");
            json!({
                "gate": gate,
                "hash": hash,
                "artifact": manifest["artifacts"].get(hash),
            })
        })
        .collect();

    Ok(Json(json!({
        "count": entries.len(),
        "heads": entries,
    })))
}

/// `GET /coord/heads/:gate` — specific gate's HEAD state.
pub async fn coord_head_by_gate(
    Path(gate): Path<String>,
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    let manifest = load_manifest(&fid)?;
    debug!(family_id = %fid, gate = %gate, "GET /coord/heads/:gate");

    let hash = manifest["heads"]
        .get(&gate)
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": format!("no HEAD for gate {gate}")})),
            )
        })?;

    let content = load_artifact(&fid, hash)?;

    Ok(Json(json!({
        "gate": gate,
        "hash": hash,
        "content": content,
        "artifact": manifest["artifacts"].get(hash),
    })))
}

/// `GET /coord/topology` — mesh topology from coordination state.
pub async fn coord_topology(
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    let manifest = load_manifest(&fid)?;
    debug!(family_id = %fid, "GET /coord/topology");

    let gates: Vec<&str> = manifest["heads"]
        .as_object()
        .into_iter()
        .flat_map(|m| m.keys().map(|s| s.as_str()))
        .collect();

    Ok(Json(json!({
        "gates": gates,
        "head_count": gates.len(),
        "source": "coordination_manifest",
        "note": "Live mesh topology requires songBird mesh.peers IPC"
    })))
}

/// `GET /coord/depot` — depot status with binary inventory.
pub async fn coord_depot(
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    debug!(family_id = %fid, "GET /coord/depot");

    let depot_path = get_storage_base_path()
        .join("datasets")
        .join(&fid)
        .join("_depot");

    let scan_path = if depot_path.exists() {
        depot_path
    } else {
        let alt = std::path::PathBuf::from("/opt/ecoPrimals/depot");
        if alt.exists() {
            alt
        } else {
            return Ok(Json(json!({
                "status": "no_depot",
                "message": "No depot directory found"
            })));
        }
    };

    let mut binaries = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&scan_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let meta = std::fs::metadata(&path).ok();
                binaries.push(json!({
                    "name": name,
                    "size": meta.as_ref().map(|m| m.len()),
                    "modified": meta.and_then(|m| m.modified().ok())
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

    Ok(Json(json!({
        "path": scan_path.display().to_string(),
        "binary_count": binaries.len(),
        "binaries": binaries,
    })))
}

/// `GET /coord/provenance/:hash` — provenance trail for an artifact.
pub async fn coord_provenance(
    Path(hash): Path<String>,
    Query(query): Query<CoordQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let fid = resolve_family(&query);
    let manifest = load_manifest(&fid)?;
    debug!(family_id = %fid, hash = %hash, "GET /coord/provenance/:hash");

    let artifact = manifest["artifacts"].get(&hash).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": format!("artifact {hash} not found")})),
        )
    })?;

    Ok(Json(json!({
        "hash": hash,
        "kind": artifact["kind"],
        "title": artifact["title"],
        "ingested_at": artifact["ingested_at"],
        "content_hash": artifact["content_hash"],
        "spine_index": artifact["spine_index"],
        "braid_id": artifact["braid_id"],
        "provenance_source": "manifest",
        "note": "Full rootPulse provenance requires loamSpine + sweetGrass IPC"
    })))
}
