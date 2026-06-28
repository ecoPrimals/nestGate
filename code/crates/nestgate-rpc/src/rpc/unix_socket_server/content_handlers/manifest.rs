// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Manifest / collection layer (NG-2): `content.publish`, `content.resolve`,
//! `content.promote`, `content.collections`.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

use super::super::StorageState;
use super::super::storage_paths::{
    content_key_path, ensure_parent_dirs, manifest_path, resolve_family_id,
};
use super::{validate_blake3_hex, validate_collection_name};

/// `content.publish` — store a manifest mapping URL paths to content hashes.
///
/// All referenced hashes must already exist in `_content/`. This is
/// validated before the manifest is written, ensuring referential integrity.
pub async fn content_publish(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let collection = params["collection"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("collection", "collection name (string) required")
    })?;
    validate_collection_name(collection)?;

    let manifest = params
        .get("manifest")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "manifest",
                "manifest (object mapping paths to hashes) required",
            )
        })?;
    let family_id = resolve_family_id(params, state)?;

    for (path, hash_val) in manifest {
        let hash = hash_val.as_str().ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "manifest",
                format!("manifest value for '{path}' must be a string hash"),
            )
        })?;
        validate_blake3_hex(hash)?;
        let content_path = content_key_path(family_id, hash);
        if !content_path.exists() {
            return Err(NestGateError::invalid_input_with_field(
                "manifest",
                format!("content hash for '{path}' not found: {hash}"),
            ));
        }
    }

    let manifest_file = manifest_path(family_id, collection);
    ensure_parent_dirs(&manifest_file).await?;

    let doc = json!({
        "collection": collection,
        "entries": manifest,
        "entry_count": manifest.len(),
        "published_at": chrono::Utc::now().to_rfc3339(),
    });
    tokio::fs::write(
        &manifest_file,
        serde_json::to_vec_pretty(&doc).unwrap_or_default(),
    )
    .await
    .map_err(|e| NestGateError::io_error(format!("Failed to write manifest {collection}: {e}")))?;

    debug!(
        "content.publish: collection={collection}, entries={}, family_id='{family_id}'",
        manifest.len()
    );

    Ok(json!({
        "collection": collection,
        "entry_count": manifest.len(),
        "stored": true,
        "family_id": family_id
    }))
}

/// `content.resolve` — look up a content hash by path within a collection.
///
/// Path normalization (static-site friendly): when an exact path match is not
/// found, the following fallbacks are tried in order:
///
/// 1. `{path}index.html` (path ends with `/`, e.g. `/` → `/index.html`)
/// 2. `{path}/index.html` (path does not end with `/`, e.g. `/about` → `/about/index.html`)
///
/// When `inline: true`, the content bytes are returned alongside the hash.
///
/// Response includes `resolved_path` when a fallback was used so the caller
/// can distinguish exact hits from normalized matches.
pub async fn content_resolve(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let t0 = std::time::Instant::now();

    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let collection = params["collection"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("collection", "collection name (string) required")
    })?;
    let path = params["path"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("path", "path (string) required"))?;
    let family_id = resolve_family_id(params, state)?;
    let inline = params["inline"].as_bool().unwrap_or(false);

    let manifest_file = resolve_manifest_target(family_id, collection).await?;
    let raw = tokio::fs::read(&manifest_file).await.map_err(|e| {
        NestGateError::io_error(format!("Failed to read manifest {collection}: {e}"))
    })?;
    let doc: Value = serde_json::from_slice(&raw)
        .map_err(|e| NestGateError::io_error(format!("Corrupt manifest {collection}: {e}")))?;

    let entries = &doc["entries"];

    let (hash, resolved_path) = resolve_path_with_index(entries, path);

    let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;

    let Some(hash) = hash else {
        return Ok(json!({
            "hash": null,
            "path": path,
            "collection": collection,
            "family_id": family_id,
            "resolved_in_ms": elapsed_ms
        }));
    };

    let mut resp = json!({
        "hash": hash,
        "path": path,
        "collection": collection,
        "family_id": family_id,
        "resolved_in_ms": elapsed_ms
    });

    if let Some(rp) = resolved_path {
        resp["resolved_path"] = json!(rp);
    }

    if inline {
        let get_params = json!({"hash": hash, "family_id": family_id});
        let content = super::cas::content_get(Some(&get_params), state).await?;
        resp["data"] = content["data"].clone();
        if let Some(ct) = content.get("content_type") {
            resp["content_type"] = ct.clone();
        }
    }

    Ok(resp)
}

/// Try exact path, then index.html fallbacks for static-site resolution.
///
/// Returns `(Some(hash), None)` for exact matches, `(Some(hash), Some(resolved))`
/// for fallback matches, or `(None, None)` for miss.
fn resolve_path_with_index<'a>(
    entries: &'a Value,
    path: &str,
) -> (Option<&'a str>, Option<String>) {
    if let Some(hash) = entries.get(path).and_then(Value::as_str) {
        return (Some(hash), None);
    }

    let index_candidate = if path.ends_with('/') {
        format!("{path}index.html")
    } else {
        format!("{path}/index.html")
    };

    if let Some(hash) = entries.get(&index_candidate).and_then(Value::as_str) {
        return (Some(hash), Some(index_candidate));
    }

    (None, None)
}

/// `content.promote` — alias one collection name to another (atomic deploy).
///
/// Creates a thin alias manifest at `_manifests/{alias}.json` that points
/// to the target collection. `content.resolve` follows the indirection.
pub async fn content_promote(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let collection = params["collection"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("collection", "collection name (string) required")
    })?;
    let alias = params["alias"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("alias", "alias name (string) required")
    })?;
    validate_collection_name(collection)?;
    validate_collection_name(alias)?;
    let family_id = resolve_family_id(params, state)?;

    let target = manifest_path(family_id, collection);
    if !target.exists() {
        return Err(NestGateError::invalid_input_with_field(
            "collection",
            format!("target collection '{collection}' does not exist"),
        ));
    }

    let alias_file = manifest_path(family_id, alias);
    ensure_parent_dirs(&alias_file).await?;
    let doc = json!({
        "alias": true,
        "target": collection,
        "promoted_at": chrono::Utc::now().to_rfc3339(),
    });
    tokio::fs::write(
        &alias_file,
        serde_json::to_vec_pretty(&doc).unwrap_or_default(),
    )
    .await
    .map_err(|e| NestGateError::io_error(format!("Failed to write alias {alias}: {e}")))?;

    debug!("content.promote: alias={alias} -> target={collection}, family_id='{family_id}'");

    Ok(json!({
        "alias": alias,
        "target": collection,
        "promoted": true,
        "family_id": family_id
    }))
}

/// `content.collections` — list all manifests/aliases within a family.
pub async fn content_collections(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let empty = json!({});
    let params = params.unwrap_or(&empty);
    let family_id = resolve_family_id(params, state)?;
    let prefix_filter = params["prefix"].as_str();

    let manifests_dir = get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_manifests");

    let mut collections: Vec<String> = Vec::new();
    if manifests_dir.exists() {
        let mut entries = tokio::fs::read_dir(&manifests_dir).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to list manifests for {family_id}: {e}"))
        })?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            let Some(stem) = name.strip_suffix(".json") else {
                continue;
            };
            if let Some(pfx) = prefix_filter
                && !stem.starts_with(pfx)
            {
                continue;
            }
            collections.push(stem.to_string());
        }
    }

    let count = collections.len();
    Ok(json!({
        "collections": collections,
        "count": count,
        "family_id": family_id
    }))
}

/// Resolve a manifest file, following one level of alias indirection.
async fn resolve_manifest_target(
    family_id: &str,
    collection: &str,
) -> std::result::Result<std::path::PathBuf, NestGateError> {
    let path = manifest_path(family_id, collection);
    if !path.exists() {
        return Err(NestGateError::invalid_input_with_field(
            "collection",
            format!("collection '{collection}' not found"),
        ));
    }
    let raw = tokio::fs::read(&path).await.map_err(|e| {
        NestGateError::io_error(format!("Failed to read manifest {collection}: {e}"))
    })?;
    let doc: Value = serde_json::from_slice(&raw)
        .map_err(|e| NestGateError::io_error(format!("Corrupt manifest {collection}: {e}")))?;

    if doc.get("alias").and_then(Value::as_bool) == Some(true) {
        let target = doc["target"].as_str().ok_or_else(|| {
            NestGateError::io_error(format!(
                "Alias manifest '{collection}' missing target field"
            ))
        })?;
        let target_path = manifest_path(family_id, target);
        if !target_path.exists() {
            return Err(NestGateError::invalid_input_with_field(
                "collection",
                format!("alias '{collection}' points to missing collection '{target}'"),
            ));
        }
        return Ok(target_path);
    }

    Ok(path)
}
