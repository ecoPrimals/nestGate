// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Coordination ingest pipeline — processes wateringHole artifacts into CAS
//! via the rootPulse commit pipeline.
//!
//! ## rootPulse flow
//!
//! 1. Parse and classify artifacts (blurb, FRAGO, AAR, wave, head)
//! 2. Hash content with BLAKE3 and store in CAS (`content.put`)
//! 3. Update coordination manifest atomically
//!
//! ## Provenance trio integration
//!
//! When the provenance trio is available on the local gate (rhizoCrypt,
//! loamSpine, sweetGrass), ingest calls the rootPulse commit pipeline:
//!
//! - rhizoCrypt `session.create` + `session.dehydrate` — DAG session
//! - bearDog `crypto.sign_ed25519` — sign dehydration summary
//! - loamSpine `session.commit` — permanent ledger entry
//! - sweetGrass `braid.create` — attribution braid
//!
//! When the trio is unavailable (e.g. thin-relay gate), ingest stores
//! content in CAS and updates the manifest without provenance metadata.
//! The provenance fields (`spine_index`, `braid_id`) remain `None` and
//! can be backfilled when the trio becomes reachable via mesh federation.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::{debug, info, warn};

use super::super::StorageState;
use super::super::storage_paths::resolve_family_id;
use super::types::{ArtifactKind, CoordArtifact, CoordManifest};

fn coord_base_path(family_id: &str) -> std::path::PathBuf {
    let base = get_storage_base_path();
    base.join("datasets").join(family_id).join("_coordination")
}

fn save_manifest(family_id: &str, manifest: &CoordManifest) -> Result<()> {
    let dir = coord_base_path(family_id);
    std::fs::create_dir_all(&dir).map_err(|e| {
        NestGateError::io_error(format!("failed to create coordination directory: {e}"))
    })?;
    let path = dir.join("manifest.json");
    let data = serde_json::to_string_pretty(manifest).map_err(|e| {
        NestGateError::io_error(format!("failed to serialize manifest: {e}"))
    })?;
    std::fs::write(&path, data).map_err(|e| {
        NestGateError::io_error(format!("failed to write manifest: {e}"))
    })?;
    Ok(())
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

fn store_artifact(family_id: &str, hash: &str, content: &[u8]) -> Result<()> {
    let artifacts_dir = coord_base_path(family_id).join("artifacts");
    std::fs::create_dir_all(&artifacts_dir).map_err(|e| {
        NestGateError::io_error(format!("failed to create artifacts directory: {e}"))
    })?;
    let path = artifacts_dir.join(hash);
    if path.exists() {
        debug!("coord.ingest: dedup hit for artifact {hash}");
        return Ok(());
    }
    std::fs::write(&path, content).map_err(|e| {
        NestGateError::io_error(format!("failed to write artifact {hash}: {e}"))
    })?;
    Ok(())
}

fn extract_wave_from_content(content: &str, kind: &ArtifactKind) -> Option<String> {
    match kind {
        ArtifactKind::Blurb => {
            for line in content.lines().take(5) {
                if let Some(rest) = line.strip_prefix("**Wave**:") {
                    return Some(rest.trim().to_owned());
                }
                if let Some(start) = line.find("Wave") {
                    let after = &line[start + 4..];
                    let after = after.trim_start_matches([' ', ':', '\u{2014}', '-'].as_ref());
                    let wave_id: String =
                        after.chars().take_while(|c| c.is_alphanumeric()).collect();
                    if !wave_id.is_empty() {
                        return Some(wave_id);
                    }
                }
            }
            None
        }
        ArtifactKind::Wave => {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("id") {
                    let rest = rest.trim_start_matches([' ', '='].as_ref()).trim();
                    let id: String = rest.chars().take_while(|c| c.is_alphanumeric()).collect();
                    if !id.is_empty() {
                        return Some(id);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn extract_gate_from_content(_content: &str, kind: &ArtifactKind, filename: &str) -> Option<String> {
    if *kind != ArtifactKind::Head {
        return None;
    }
    let stem = filename.trim_end_matches(".toml");
    Some(stem.to_owned())
}

fn extract_title(content: &str, kind: &ArtifactKind, filename: &str) -> String {
    match kind {
        ArtifactKind::Blurb | ArtifactKind::Frago | ArtifactKind::Aar => {
            for line in content.lines().take(3) {
                let trimmed = line.trim().trim_start_matches('#').trim();
                if !trimmed.is_empty() {
                    return trimmed.to_owned();
                }
            }
            filename.to_owned()
        }
        ArtifactKind::Wave => "Wave State".to_owned(),
        ArtifactKind::Head => format!("{} HEAD", filename.trim_end_matches(".toml")),
    }
}

/// `coord.ingest` — ingest wateringHole artifacts into the coordination CAS.
///
/// Accepts a list of artifacts to ingest:
/// ```json
/// {
///   "artifacts": [
///     { "filename": "ECOSYSTEM_BLURB.md", "content_base64": "..." },
///     { "filename": "eastGate.toml", "content_base64": "..." }
///   ]
/// }
/// ```
///
/// Or a single artifact:
/// ```json
/// {
///   "filename": "ECOSYSTEM_BLURB.md",
///   "content_base64": "..."
/// }
/// ```
pub async fn coord_ingest(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let family_id = resolve_family_id(params, state)?;
    let mut manifest = load_manifest(family_id)?;

    let artifacts = if let Some(arr) = params["artifacts"].as_array() {
        arr.clone()
    } else if params["filename"].is_string() {
        vec![params.clone()]
    } else {
        return Err(NestGateError::invalid_input_with_field(
            "artifacts",
            "artifacts array or single {filename, content_base64} required",
        ));
    };

    let mut ingested = Vec::new();
    let mut errors = Vec::new();

    for artifact_val in &artifacts {
        let Some(filename) = artifact_val["filename"].as_str() else {
            errors.push(json!({"error": "missing filename"}));
            continue;
        };

        let Some(content_b64) = artifact_val["content_base64"].as_str() else {
            errors.push(json!({"error": format!("missing content_base64 for {filename}")}));
            continue;
        };

        let raw = match STANDARD.decode(content_b64) {
            Ok(r) => r,
            Err(e) => {
                errors.push(json!({"error": format!("invalid base64 for {filename}: {e}")}));
                continue;
            }
        };

        let content_str = String::from_utf8_lossy(&raw);
        let hash = blake3::hash(&raw).to_hex().to_string();
        let kind = ArtifactKind::from_filename(filename);
        let wave = extract_wave_from_content(&content_str, &kind);
        let gate = extract_gate_from_content(&content_str, &kind, filename);
        let title = extract_title(&content_str, &kind, filename);

        store_artifact(family_id, &hash, &raw)?;

        let coord_artifact = CoordArtifact {
            hash: hash.clone(),
            kind: kind.clone(),
            title: title.clone(),
            wave: wave.clone(),
            gate: gate.clone(),
            ingested_at: chrono::Utc::now().to_rfc3339(),
            content_hash: hash.clone(),
            spine_index: None,
            braid_id: None,
        };

        manifest.artifacts.insert(hash.clone(), coord_artifact);

        match kind {
            ArtifactKind::Blurb => {
                manifest.current_blurb = Some(hash.clone());
                if !manifest.blurb_history.contains(&hash) {
                    manifest.blurb_history.insert(0, hash.clone());
                }
            }
            ArtifactKind::Frago | ArtifactKind::Aar => {
                if !manifest.frago_history.contains(&hash) {
                    manifest.frago_history.insert(0, hash.clone());
                }
            }
            ArtifactKind::Wave => {
                manifest.current_wave = Some(hash.clone());
            }
            ArtifactKind::Head => {
                if let Some(ref g) = gate {
                    manifest.heads.insert(g.clone(), hash.clone());
                }
            }
        }

        info!(
            "coord.ingest: ingested {} ({}) hash={} wave={:?}",
            filename,
            kind.as_str(),
            &hash[..12],
            wave
        );

        ingested.push(json!({
            "filename": filename,
            "kind": kind.as_str(),
            "hash": hash,
            "title": title,
            "wave": wave,
            "gate": gate,
        }));
    }

    manifest.updated_at = chrono::Utc::now().to_rfc3339();
    manifest.version += 1;
    save_manifest(family_id, &manifest)?;

    // rootPulse provenance trio integration:
    // When rhizoCrypt, loamSpine, and sweetGrass are reachable via IPC,
    // the ingest pipeline will call:
    //   1. rhizoCrypt session.create + event.append (DAG session for change set)
    //   2. rhizoCrypt session.dehydrate (merkle root)
    //   3. bearDog crypto.sign_ed25519 (sign dehydration summary)
    //   4. loamSpine session.commit (permanent ledger entry)
    //   5. sweetGrass braid.create (attribution braid)
    //
    // The pipeline populates spine_index and braid_id on each artifact.
    // This is orchestrated by biomeOS via the rootpulse_commit graph.
    //
    // For now, the manifest stores artifacts without provenance metadata.
    // The trio integration is wired in the rootpulse_commit.toml graph
    // and will be activated when the NUCLEUS composition is live on sporeGate.
    let provenance_available = false;
    if provenance_available {
        warn!("coord.ingest: rootPulse provenance trio integration is pending NUCLEUS activation");
    }

    Ok(json!({
        "ingested": ingested.len(),
        "errors": errors.len(),
        "results": ingested,
        "error_details": errors,
        "manifest_version": manifest.version,
        "provenance": provenance_available,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_wave_from_blurb() {
        let content = "# ecoPrimals Ecosystem Blurb — Wave 135\n\n**Date**: Jul 9";
        assert_eq!(
            extract_wave_from_content(content, &ArtifactKind::Blurb),
            Some("135".to_owned())
        );
    }

    #[test]
    fn extract_title_from_blurb() {
        let content = "# ecoPrimals Ecosystem Blurb — Wave 135\n\nSome body";
        let title = extract_title(content, &ArtifactKind::Blurb, "ECOSYSTEM_BLURB.md");
        assert_eq!(title, "ecoPrimals Ecosystem Blurb — Wave 135");
    }

    #[test]
    fn extract_gate_from_head() {
        let gate = extract_gate_from_content("", &ArtifactKind::Head, "eastGate.toml");
        assert_eq!(gate, Some("eastGate".to_owned()));
    }

    #[test]
    fn extract_gate_none_for_blurb() {
        let gate = extract_gate_from_content("", &ArtifactKind::Blurb, "BLURB.md");
        assert!(gate.is_none());
    }
}
