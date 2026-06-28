// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content-addressed store (CAS) handlers: `content.put`, `content.get`,
//! `content.exists`, `content.list`.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

use super::super::StorageState;
use super::super::storage_paths::{
    content_hash_hex, content_key_path, ensure_parent_dirs, resolve_family_id,
};
use super::{maybe_decrypt, merge_sidecar_fields, validate_blake3_hex};

/// `content.put` — store content-addressed data (BLAKE3 hash as key, automatic dedup).
///
/// Accepts `data` or `content_base64` as the base64-encoded payload (SP-4 compat).
/// Provenance fields (`source`, `pipeline`, `stored_by`) can appear at the top
/// level or nested inside a `metadata` object (SP-4 compat).
pub async fn content_put(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let data_b64 = params["data"]
        .as_str()
        .or_else(|| params["content_base64"].as_str())
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "data",
                "data or content_base64 (base64 string) required",
            )
        })?;
    let content_type = params["content_type"].as_str();
    let meta_obj = params.get("metadata");
    let source = params["source"]
        .as_str()
        .or_else(|| meta_obj.and_then(|m| m["source"].as_str()));
    let pipeline = params["pipeline"]
        .as_str()
        .or_else(|| meta_obj.and_then(|m| m["pipeline"].as_str()));
    let stored_by = params["stored_by"]
        .as_str()
        .or_else(|| meta_obj.and_then(|m| m["stored_by"].as_str()));
    let parent_hash = params["parent_hash"]
        .as_str()
        .or_else(|| meta_obj.and_then(|m| m["parent_hash"].as_str()));
    let derivation_depth: Option<u64> = params["derivation_depth"]
        .as_u64()
        .or_else(|| meta_obj.and_then(|m| m["derivation_depth"].as_u64()));
    let family_id = resolve_family_id(params, state)?;

    let raw = STANDARD.decode(data_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("data", format!("invalid base64: {e}"))
    })?;

    let blake3_hex = content_hash_hex(&raw);
    let object_path = content_key_path(family_id, &blake3_hex);

    if object_path.exists() {
        debug!(
            "content.put: dedup hit family_id='{}', hash={blake3_hex}, size={}",
            family_id,
            raw.len()
        );
        return Ok(json!({
            "hash": blake3_hex,
            "size": raw.len(),
            "stored": true,
            "deduplicated": true,
            "family_id": family_id
        }));
    }

    let write_data: std::borrow::Cow<'_, [u8]> = if let Some(ref enc) = state.encryption {
        std::borrow::Cow::Owned(enc.encrypt(&raw)?)
    } else {
        std::borrow::Cow::Borrowed(&raw)
    };

    ensure_parent_dirs(&object_path).await?;
    tokio::fs::write(&object_path, write_data.as_ref())
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to write content {blake3_hex}: {e}"))
        })?;

    let mut meta = json!({
        "hash": blake3_hex,
        "size": raw.len(),
        "content_type": content_type,
        "stored_at": chrono::Utc::now().to_rfc3339(),
    });
    if let Some(s) = source {
        meta["source"] = json!(s);
    }
    if let Some(p) = pipeline {
        meta["pipeline"] = json!(p);
    }
    if let Some(by) = stored_by {
        meta["stored_by"] = json!(by);
    }
    if let Some(ph) = parent_hash {
        meta["parent_hash"] = json!(ph);
    }
    let depth = derivation_depth.unwrap_or_else(|| u64::from(parent_hash.is_some()));
    meta["derivation_depth"] = json!(depth);
    let meta_path = object_path.with_extension("meta.json");
    tokio::fs::write(
        &meta_path,
        serde_json::to_vec_pretty(&meta).unwrap_or_default(),
    )
    .await
    .map_err(|e| {
        NestGateError::io_error(format!(
            "Failed to write content metadata {blake3_hex}: {e}"
        ))
    })?;

    debug!(
        "content.put: stored family_id='{}', hash={blake3_hex}, size={}",
        family_id,
        raw.len()
    );

    Ok(json!({
        "hash": blake3_hex,
        "size": raw.len(),
        "stored": true,
        "deduplicated": false,
        "content_type": content_type,
        "family_id": family_id
    }))
}

/// `content.get` — retrieve content by BLAKE3 hash.
///
/// Response includes `retrieved_in_ms` for shadow-run latency measurement.
pub async fn content_get(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let t0 = std::time::Instant::now();

    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let hash = params["hash"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("hash", "hash (blake3 hex string) required")
    })?;
    validate_blake3_hex(hash)?;
    let family_id = resolve_family_id(params, state)?;

    let object_path = content_key_path(family_id, hash);
    if !object_path.exists() {
        return Ok(json!({"data": null, "hash": hash, "family_id": family_id}));
    }

    let raw_data = tokio::fs::read(&object_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read content {hash}: {e}")))?;

    let plain = maybe_decrypt(raw_data, state)?;

    let meta_path = object_path.with_extension("meta.json");
    let sidecar: Option<Value> = if meta_path.exists() {
        tokio::fs::read(&meta_path)
            .await
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
    } else {
        None
    };

    let mut resp = json!({
        "data": STANDARD.encode(&plain),
        "hash": hash,
        "size": plain.len(),
        "family_id": family_id,
        "retrieved_in_ms": t0.elapsed().as_secs_f64() * 1000.0
    });
    if let Some(ref meta) = sidecar {
        merge_sidecar_fields(&mut resp, meta);
    }
    Ok(resp)
}

/// `content.exists` — check if content hash exists.
pub async fn content_exists(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let hash = params["hash"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("hash", "hash (blake3 hex string) required")
    })?;
    validate_blake3_hex(hash)?;
    let family_id = resolve_family_id(params, state)?;

    let object_path = content_key_path(family_id, hash);
    if object_path.exists() {
        let file_meta = tokio::fs::metadata(&object_path)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to stat content {hash}: {e}")))?;
        let mut resp =
            json!({"exists": true, "hash": hash, "size": file_meta.len(), "family_id": family_id});

        let sidecar_path = object_path.with_extension("meta.json");
        if let Ok(raw) = tokio::fs::read(&sidecar_path).await
            && let Ok(sidecar) = serde_json::from_slice::<Value>(&raw)
        {
            merge_sidecar_fields(&mut resp, &sidecar);
        }
        Ok(resp)
    } else {
        Ok(json!({"exists": false, "hash": hash, "family_id": family_id}))
    }
}

/// `content.list` — enumerate content-addressed objects.
pub async fn content_list(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let empty = json!({});
    let params = params.unwrap_or(&empty);
    let family_id = resolve_family_id(params, state)?;

    let content_dir = get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_content");

    let mut hashes: Vec<Value> = Vec::new();
    if content_dir.exists() {
        let mut prefix_dirs = tokio::fs::read_dir(&content_dir).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to list content for {family_id}: {e}"))
        })?;
        while let Ok(Some(prefix_entry)) = prefix_dirs.next_entry().await {
            if !prefix_entry
                .file_type()
                .await
                .map(|ft| ft.is_dir())
                .unwrap_or(false)
            {
                continue;
            }
            let mut entries = tokio::fs::read_dir(prefix_entry.path())
                .await
                .map_err(|e| {
                    NestGateError::io_error(format!("Failed to read content prefix dir: {e}"))
                })?;
            while let Ok(Some(entry)) = entries.next_entry().await {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.ends_with(".meta.json") {
                    continue;
                }
                let size = entry.metadata().await.map(|m| m.len()).unwrap_or(0);
                hashes.push(json!({"hash": &*name, "size": size}));
            }
        }
    }

    let count = hashes.len();
    Ok(json!({
        "hashes": hashes,
        "count": count,
        "family_id": family_id
    }))
}
