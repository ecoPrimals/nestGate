// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content-addressed storage handlers — BLAKE3 hash-as-key with automatic dedup.
//!
//! Objects are immutable: the BLAKE3 hash of the content *is* the key. Storing
//! the same bytes twice is a no-op that returns `deduplicated: true`.
//!
//! Filesystem layout:
//!   `{base}/datasets/{family}/_content/{hex[..2]}/{hex}`
//!   `{base}/datasets/{family}/_content/{hex[..2]}/{hex}.meta.json`

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

use super::StorageState;
use super::storage_paths::{
    content_key_path, ensure_parent_dirs, manifest_path, resolve_family_id,
};

/// `content.put` — store content-addressed data (BLAKE3 hash as key, automatic dedup).
pub(super) async fn content_put(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let data_b64 = params["data"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("data", "data (base64 string) required")
    })?;
    let content_type = params["content_type"].as_str();
    let family_id = resolve_family_id(params, state)?;

    let raw = STANDARD.decode(data_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("data", format!("invalid base64: {e}"))
    })?;

    let blake3_hex = blake3::hash(&raw).to_hex().to_string();
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

    let write_data = if let Some(ref enc) = state.encryption {
        enc.encrypt(&raw)?
    } else {
        raw.clone()
    };

    ensure_parent_dirs(&object_path).await?;
    tokio::fs::write(&object_path, &write_data)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to write content {blake3_hex}: {e}"))
        })?;

    let meta = json!({
        "hash": blake3_hex,
        "size": raw.len(),
        "content_type": content_type,
        "stored_at": chrono::Utc::now().to_rfc3339(),
    });
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
pub(super) async fn content_get(params: Option<&Value>, state: &StorageState) -> Result<Value> {
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

    let plain =
        if crate::rpc::storage_encryption::StorageEncryption::is_encrypted_envelope(&raw_data) {
            if let Some(ref enc) = state.encryption {
                enc.decrypt(&raw_data)?
            } else {
                raw_data
            }
        } else {
            raw_data
        };

    let meta_path = object_path.with_extension("meta.json");
    let content_type = if meta_path.exists() {
        tokio::fs::read(&meta_path)
            .await
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            .and_then(|v| v["content_type"].as_str().map(String::from))
    } else {
        None
    };

    let mut resp = json!({
        "data": STANDARD.encode(&plain),
        "hash": hash,
        "size": plain.len(),
        "family_id": family_id
    });
    if let Some(ct) = content_type {
        resp["content_type"] = json!(ct);
    }
    Ok(resp)
}

/// `content.exists` — check if content hash exists.
pub(super) async fn content_exists(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let hash = params["hash"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("hash", "hash (blake3 hex string) required")
    })?;
    validate_blake3_hex(hash)?;
    let family_id = resolve_family_id(params, state)?;

    let object_path = content_key_path(family_id, hash);
    if object_path.exists() {
        let meta = tokio::fs::metadata(&object_path)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to stat content {hash}: {e}")))?;
        Ok(json!({"exists": true, "hash": hash, "size": meta.len(), "family_id": family_id}))
    } else {
        Ok(json!({"exists": false, "hash": hash, "family_id": family_id}))
    }
}

/// `content.list` — enumerate content-addressed objects.
pub(super) async fn content_list(params: Option<&Value>, state: &StorageState) -> Result<Value> {
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

// ── Manifest / collection handlers (NG-2) ───────────────────────────

/// `content.publish` — store a manifest mapping URL paths to content hashes.
///
/// All referenced hashes must already exist in `_content/`. This is
/// validated before the manifest is written, ensuring referential integrity.
pub(super) async fn content_publish(params: Option<&Value>, state: &StorageState) -> Result<Value> {
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
/// When `inline: true`, the content bytes are returned alongside the hash.
pub(super) async fn content_resolve(params: Option<&Value>, state: &StorageState) -> Result<Value> {
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

    let hash = doc["entries"].get(path).and_then(Value::as_str);

    let Some(hash) = hash else {
        return Ok(json!({
            "hash": null,
            "path": path,
            "collection": collection,
            "family_id": family_id
        }));
    };

    let mut resp = json!({
        "hash": hash,
        "path": path,
        "collection": collection,
        "family_id": family_id
    });

    if inline {
        let get_params = json!({"hash": hash, "family_id": family_id});
        let content = content_get(Some(&get_params), state).await?;
        resp["data"] = content["data"].clone();
        if let Some(ct) = content.get("content_type") {
            resp["content_type"] = ct.clone();
        }
    }

    Ok(resp)
}

/// `content.promote` — alias one collection name to another (atomic deploy).
///
/// Creates a thin alias manifest at `_manifests/{alias}.json` that points
/// to the target collection. `content.resolve` follows the indirection.
pub(super) async fn content_promote(params: Option<&Value>, state: &StorageState) -> Result<Value> {
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
pub(super) async fn content_collections(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
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

// ── Internal helpers ─────────────────────────────────────────────────

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

fn validate_blake3_hex(hash: &str) -> Result<()> {
    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(NestGateError::invalid_input_with_field(
            "hash",
            "must be a 64-character lowercase hex BLAKE3 digest",
        ));
    }
    Ok(())
}

fn validate_collection_name(name: &str) -> Result<()> {
    if name.is_empty()
        || name.contains('/')
        || name.contains('\\')
        || name.contains("..")
        || name.starts_with('.')
    {
        return Err(NestGateError::invalid_input_with_field(
            "collection",
            "must be a non-empty simple name without path separators",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    async fn mock_state(family_id: Option<&str>) -> StorageState {
        StorageState {
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
            family_id: family_id.map(String::from),
            storage_initialized: true,
            encryption: None,
        }
    }

    #[tokio::test]
    async fn content_put_and_get_round_trip() {
        let state = mock_state(Some("test-content")).await;
        let family_id = format!("test-cas-{}", uuid::Uuid::new_v4());
        let raw = b"hello content-addressed world";
        let encoded = STANDARD.encode(raw);
        let expected_hash = blake3::hash(raw).to_hex().to_string();

        let put_params = json!({
            "family_id": &family_id,
            "data": encoded,
            "content_type": "text/plain"
        });
        let put_result = content_put(Some(&put_params), &state).await;
        assert!(put_result.is_ok(), "put failed: {put_result:?}");
        let put_val = put_result.unwrap();
        assert_eq!(put_val["hash"], expected_hash);
        assert_eq!(put_val["deduplicated"], false);
        assert_eq!(put_val["stored"], true);
        assert_eq!(put_val["size"], raw.len());

        let get_params = json!({"family_id": &family_id, "hash": &expected_hash});
        let get_result = content_get(Some(&get_params), &state).await;
        assert!(get_result.is_ok(), "get failed: {get_result:?}");
        let get_val = get_result.unwrap();
        let decoded = STANDARD.decode(get_val["data"].as_str().unwrap()).unwrap();
        assert_eq!(decoded, raw);
        assert_eq!(get_val["content_type"], "text/plain");

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn content_put_deduplicates() {
        let state = mock_state(Some("test-dedup")).await;
        let family_id = format!("test-dedup-{}", uuid::Uuid::new_v4());
        let raw = b"duplicate content";
        let encoded = STANDARD.encode(raw);

        let params = json!({"family_id": &family_id, "data": &encoded});
        let first = content_put(Some(&params), &state).await.unwrap();
        assert_eq!(first["deduplicated"], false);

        let second = content_put(Some(&params), &state).await.unwrap();
        assert_eq!(second["deduplicated"], true);
        assert_eq!(second["hash"], first["hash"]);

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn content_exists_returns_correct_state() {
        let state = mock_state(Some("test-exists")).await;
        let family_id = format!("test-exists-{}", uuid::Uuid::new_v4());
        let raw = b"existence check";
        let encoded = STANDARD.encode(raw);
        let hash = blake3::hash(raw).to_hex().to_string();

        let missing = json!({"family_id": &family_id, "hash": &hash});
        let r = content_exists(Some(&missing), &state).await.unwrap();
        assert_eq!(r["exists"], false);

        let put_p = json!({"family_id": &family_id, "data": &encoded});
        content_put(Some(&put_p), &state).await.unwrap();

        let found = json!({"family_id": &family_id, "hash": &hash});
        let r = content_exists(Some(&found), &state).await.unwrap();
        assert_eq!(r["exists"], true);
        assert!(r["size"].as_u64().unwrap() > 0);

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn content_list_returns_stored_hashes() {
        let state = mock_state(Some("test-list")).await;
        let family_id = format!("test-list-{}", uuid::Uuid::new_v4());

        for i in 0..3 {
            let data = format!("item-{i}");
            let encoded = STANDARD.encode(data.as_bytes());
            let p = json!({"family_id": &family_id, "data": &encoded});
            content_put(Some(&p), &state).await.unwrap();
        }

        let list_p = json!({"family_id": &family_id});
        let result = content_list(Some(&list_p), &state).await.unwrap();
        assert_eq!(result["count"], 3);
        assert_eq!(result["hashes"].as_array().unwrap().len(), 3);

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn content_get_returns_null_for_missing() {
        let state = mock_state(Some("test-miss")).await;
        let hash = "a".repeat(64);
        let params = json!({"family_id": "test-miss", "hash": &hash});
        let result = content_get(Some(&params), &state).await.unwrap();
        assert!(result["data"].is_null());
    }

    #[tokio::test]
    async fn content_put_requires_data() {
        let state = mock_state(Some("test")).await;
        let params = json!({"family_id": "test"});
        assert!(content_put(Some(&params), &state).await.is_err());
    }

    #[tokio::test]
    async fn content_get_rejects_invalid_hash() {
        let state = mock_state(Some("test")).await;
        let params = json!({"family_id": "test", "hash": "tooshort"});
        assert!(content_get(Some(&params), &state).await.is_err());
    }

    // ── NG-2: Manifest / collection tests ────────────────────────────

    async fn put_test_content(state: &StorageState, family_id: &str, data: &[u8]) -> String {
        let encoded = STANDARD.encode(data);
        let p = json!({"family_id": family_id, "data": &encoded, "content_type": "text/html"});
        let r = content_put(Some(&p), state).await.unwrap();
        r["hash"].as_str().unwrap().to_string()
    }

    #[tokio::test]
    async fn publish_and_resolve_round_trip() {
        let state = mock_state(Some("test-pub")).await;
        let family_id = format!("test-pub-{}", uuid::Uuid::new_v4());

        let h1 = put_test_content(&state, &family_id, b"<html>index</html>").await;
        let h2 = put_test_content(&state, &family_id, b"body{color:red}").await;

        let pub_p = json!({
            "family_id": &family_id,
            "collection": "site-v1",
            "manifest": {"/": &h1, "/css/main.css": &h2}
        });
        let r = content_publish(Some(&pub_p), &state).await.unwrap();
        assert_eq!(r["stored"], true);
        assert_eq!(r["entry_count"], 2);

        let resolve_p = json!({
            "family_id": &family_id,
            "collection": "site-v1",
            "path": "/css/main.css"
        });
        let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
        assert_eq!(r["hash"], h2);

        let resolve_missing = json!({
            "family_id": &family_id,
            "collection": "site-v1",
            "path": "/not-here"
        });
        let r = content_resolve(Some(&resolve_missing), &state)
            .await
            .unwrap();
        assert!(r["hash"].is_null());

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn promote_alias_resolves_correctly() {
        let state = mock_state(Some("test-promo")).await;
        let family_id = format!("test-promo-{}", uuid::Uuid::new_v4());

        let h1 = put_test_content(&state, &family_id, b"hello").await;

        let pub_p = json!({
            "family_id": &family_id,
            "collection": "release-v1",
            "manifest": {"/": &h1}
        });
        content_publish(Some(&pub_p), &state).await.unwrap();

        let promote_p = json!({
            "family_id": &family_id,
            "collection": "release-v1",
            "alias": "latest"
        });
        let r = content_promote(Some(&promote_p), &state).await.unwrap();
        assert_eq!(r["promoted"], true);
        assert_eq!(r["target"], "release-v1");

        let resolve_p = json!({
            "family_id": &family_id,
            "collection": "latest",
            "path": "/"
        });
        let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
        assert_eq!(r["hash"], h1);

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn collections_lists_manifests() {
        let state = mock_state(Some("test-coll")).await;
        let family_id = format!("test-coll-{}", uuid::Uuid::new_v4());

        let h = put_test_content(&state, &family_id, b"x").await;

        for name in &["alpha", "beta", "gamma"] {
            let p = json!({
                "family_id": &family_id,
                "collection": name,
                "manifest": {"/": &h}
            });
            content_publish(Some(&p), &state).await.unwrap();
        }

        let list_p = json!({"family_id": &family_id});
        let r = content_collections(Some(&list_p), &state).await.unwrap();
        assert_eq!(r["count"], 3);

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn publish_rejects_missing_content_hash() {
        let state = mock_state(Some("test-reject")).await;
        let family_id = format!("test-reject-{}", uuid::Uuid::new_v4());
        let fake_hash = "b".repeat(64);

        let p = json!({
            "family_id": &family_id,
            "collection": "bad",
            "manifest": {"/": &fake_hash}
        });
        assert!(content_publish(Some(&p), &state).await.is_err());
    }

    #[tokio::test]
    async fn resolve_inline_returns_content() {
        let state = mock_state(Some("test-inline")).await;
        let family_id = format!("test-inline-{}", uuid::Uuid::new_v4());

        let content = b"<html>inline</html>";
        let h = put_test_content(&state, &family_id, content).await;

        let pub_p = json!({
            "family_id": &family_id,
            "collection": "inline-test",
            "manifest": {"/": &h}
        });
        content_publish(Some(&pub_p), &state).await.unwrap();

        let resolve_p = json!({
            "family_id": &family_id,
            "collection": "inline-test",
            "path": "/",
            "inline": true
        });
        let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
        assert_eq!(r["hash"], h);
        let decoded = STANDARD.decode(r["data"].as_str().unwrap()).unwrap();
        assert_eq!(decoded, content);
        assert_eq!(r["content_type"], "text/html");

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }
}
