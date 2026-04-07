// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage JSON-RPC Handlers
//!
//! Extracted from `unix_socket_server` for domain-based refactoring.
//! Handles: `storage.store`, `storage.retrieve`, `storage.exists`, `storage.delete`,
//! `storage.list`, `storage.stats`, `storage.store_blob`, `storage.retrieve_blob`,
//! `storage.fetch_external`

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use tracing::debug;

use super::StorageState;

/// Build the filesystem path for a key in a family's dataset.
///
/// Layout matches `nestgate-core` `operations::objects`: `{base}/datasets/{family}/{key}`.
fn dataset_key_path(family_id: &str, key: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join(key)
}

/// Build the filesystem path for a binary blob.
///
/// Blobs are stored separately under `{base}/datasets/{family}/_blobs/{key}` so list
/// operations can distinguish JSON objects from raw blobs.
fn blob_key_path(family_id: &str, key: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_blobs")
        .join(key)
}

/// Ensure all parent directories of `path` exist.
async fn ensure_parent_dirs(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            NestGateError::io_error(format!(
                "Failed to create directories {}: {e}",
                parent.display()
            ))
        })?;
    }
    Ok(())
}

/// Resolve `family_id` from params, falling back to the server's socket-scoped family.
///
/// When callers connect via a family-scoped socket (`nestgate-{family}.sock`),
/// the server already knows the family context. This eliminates the #1 friction
/// point identified in primalSpring composition experiments (exp066/068).
pub fn resolve_family_id<'a>(params: &'a Value, state: &'a StorageState) -> Result<&'a str> {
    if let Some(fid) = params["family_id"].as_str() {
        return Ok(fid);
    }
    if let Some(ref fid) = state.family_id {
        return Ok(fid.as_str());
    }
    Err(NestGateError::invalid_input_with_field(
        "family_id",
        "family_id required (or connect via a family-scoped socket)",
    ))
}

/// storage.store - Store key-value data (filesystem-backed, durable)
pub(super) async fn storage_store(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;

    let data = if params.get("value").is_some() && !params["value"].is_null() {
        &params["value"]
    } else if params.get("data").is_some() && !params["data"].is_null() {
        &params["data"]
    } else {
        return Err(NestGateError::invalid_input_with_field(
            "value",
            "value or data (json) required",
        ));
    };

    let family_id = resolve_family_id(params, state)?;

    let bytes = serde_json::to_vec_pretty(data)
        .map_err(|e| NestGateError::io_error(format!("Failed to serialize value: {e}")))?;

    debug!(
        "storage.store: family_id='{}', key='{}', value_size={} bytes",
        family_id,
        key,
        bytes.len()
    );

    let object_path = dataset_key_path(family_id, key);
    ensure_parent_dirs(&object_path).await?;
    tokio::fs::write(&object_path, &bytes)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to write {family_id}/{key}: {e}")))?;

    Ok(json!({"status": "stored", "key": key, "family_id": family_id}))
}

/// storage.retrieve - Retrieve data by key (filesystem-backed, durable)
pub(super) async fn storage_retrieve(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;

    debug!("storage.retrieve: family_id='{}', key='{}'", family_id, key);

    let object_path = dataset_key_path(family_id, key);
    if !object_path.exists() {
        return Ok(json!({"value": null, "data": null, "key": key}));
    }

    let bytes = tokio::fs::read(&object_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read {family_id}/{key}: {e}")))?;
    let value: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));

    Ok(json!({"value": value, "data": value, "key": key, "family_id": family_id}))
}

/// storage.exists - Check if data exists by key (filesystem-backed)
pub(super) fn storage_exists(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;

    let object_path = dataset_key_path(family_id, key);
    Ok(json!({"exists": object_path.exists(), "key": key, "family_id": family_id}))
}

/// storage.delete - Delete data by key (filesystem-backed)
pub(super) async fn storage_delete(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;

    let object_path = dataset_key_path(family_id, key);
    if object_path.exists() {
        tokio::fs::remove_file(&object_path).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to delete {family_id}/{key}: {e}"))
        })?;
    }
    Ok(json!({"status": "deleted", "key": key, "family_id": family_id}))
}

/// storage.list - List all keys with optional prefix
pub(super) async fn storage_list(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = resolve_family_id(params, state)?;
    let prefix = params["prefix"].as_str();

    let dataset = family_id;

    // Scan the dataset directory — aligned with store_object's write path
    // which writes to .../datasets/{dataset}/{key} (no "objects/" segment).
    let dataset_path = get_storage_base_path().join("datasets").join(dataset);

    let keys = list_keys_recursive(&dataset_path, &dataset_path, prefix).await;

    debug!(
        "Listed {} keys for family '{}' (prefix: {:?})",
        keys.len(),
        family_id,
        prefix
    );

    Ok(json!({
        "keys": keys
    }))
}

/// storage.stats - Get storage statistics
pub(super) async fn storage_stats(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = resolve_family_id(params, state)?;

    let dataset = family_id;

    let dataset_path = get_storage_base_path().join("datasets").join(dataset);

    let keys = list_keys_recursive(&dataset_path, &dataset_path, None).await;
    let key_count = keys.len();

    debug!("Stats for family '{}': {} objects", family_id, key_count);

    Ok(json!({
        "key_count": key_count,
        "blob_count": 0,
        "family_id": family_id
    }))
}

/// `storage.store_blob` - Store binary blob (base64 encoded, filesystem-backed)
pub(super) async fn storage_store_blob(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let blob_base64 = params["blob"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("blob", "blob (base64 string) required")
    })?;
    let family_id = resolve_family_id(params, state)?;

    let blob_data = STANDARD.decode(blob_base64).map_err(|e| {
        NestGateError::invalid_input_with_field("blob", format!("Invalid base64: {e}"))
    })?;

    debug!(
        "storage.store_blob: family_id='{}', key='{}', blob_size={} bytes",
        family_id,
        key,
        blob_data.len()
    );

    let blob_path = blob_key_path(family_id, key);
    ensure_parent_dirs(&blob_path).await?;
    tokio::fs::write(&blob_path, &blob_data)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to write blob {family_id}/{key}: {e}"))
        })?;

    Ok(json!({
        "status": "stored",
        "key": key,
        "family_id": family_id,
        "size": blob_data.len()
    }))
}

/// `storage.retrieve_blob` - Retrieve binary blob (base64 encoded, filesystem-backed)
pub(super) async fn storage_retrieve_blob(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;

    let blob_path = blob_key_path(family_id, key);
    if !blob_path.exists() {
        return Ok(json!({"blob": null, "key": key}));
    }

    let blob_data = tokio::fs::read(&blob_path).await.map_err(|e| {
        NestGateError::io_error(format!("Failed to read blob {family_id}/{key}: {e}"))
    })?;

    Ok(json!({
        "blob": STANDARD.encode(&blob_data),
        "key": key,
        "family_id": family_id,
        "size": blob_data.len()
    }))
}

/// Build the filesystem path for an external-fetch cache entry.
///
/// Layout: `{base}/datasets/{family}/_external/{cache_key}` — separate from
/// user-created objects so `storage.list` can distinguish fetched content.
fn external_cache_path(family_id: &str, cache_key: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_external")
        .join(cache_key)
}

/// Metadata sidecar path for an external-fetch cache entry.
fn external_meta_path(family_id: &str, cache_key: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_external")
        .join(format!("{cache_key}.meta.json"))
}

/// Validate the URL scheme for external fetch — only HTTPS unless HTTP is explicitly allowed.
fn validate_fetch_url(url: &str, allow_http: bool) -> Result<()> {
    let parsed = url::Url::parse(url)
        .map_err(|e| NestGateError::invalid_input_with_field("url", format!("invalid URL: {e}")))?;
    match parsed.scheme() {
        "https" => Ok(()),
        "http" if allow_http => {
            tracing::warn!("storage.fetch_external: HTTP (insecure) requested for {url}");
            Ok(())
        }
        scheme => Err(NestGateError::invalid_input_with_field(
            "url",
            format!("scheme '{scheme}' not allowed — use https"),
        )),
    }
}

/// Return a cached external-fetch response if both payload and metadata exist on disk.
async fn try_cached_external(url: &str, cache_key: &str, family_id: &str) -> Result<Option<Value>> {
    let cache_path = external_cache_path(family_id, cache_key);
    let meta_path = external_meta_path(family_id, cache_key);
    if !cache_path.exists() || !meta_path.exists() {
        return Ok(None);
    }
    debug!("storage.fetch_external: cache hit for '{cache_key}' (family={family_id})");
    let meta_bytes = tokio::fs::read(&meta_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read cache metadata: {e}")))?;
    let meta: Value = serde_json::from_slice(&meta_bytes).unwrap_or_else(|_| json!({}));
    let payload_bytes = tokio::fs::read(&cache_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read cached payload: {e}")))?;
    let value: Value = serde_json::from_slice(&payload_bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&payload_bytes).into_owned()));
    Ok(Some(json!({
        "value": value, "data": value, "key": cache_key,
        "blake3": meta["blake3"], "url": url, "size": payload_bytes.len(),
        "cached": true, "family_id": family_id,
    })))
}

/// Fetch a URL, BLAKE3-hash the payload, cache it, and return provenance metadata.
async fn do_external_fetch(
    url: &str,
    cache_key: &str,
    family_id: &str,
    timeout_secs: u64,
) -> Result<Value> {
    debug!("storage.fetch_external: fetching '{url}' as '{cache_key}' (family={family_id})");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs))
        .user_agent(format!("NestGate/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| NestGateError::io_error(format!("HTTP client error: {e}")))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| NestGateError::io_error(format!("Fetch failed for {url}: {e}")))?;
    let status = response.status();
    if !status.is_success() {
        return Err(NestGateError::io_error(format!(
            "Fetch returned HTTP {status} for {url}"
        )));
    }
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_owned();
    let payload_bytes = response.bytes().await.map_err(|e| {
        NestGateError::io_error(format!("Failed to read response body from {url}: {e}"))
    })?;

    let blake3_hex = blake3::hash(&payload_bytes).to_hex().to_string();
    debug!(
        "storage.fetch_external: fetched {} bytes, blake3={blake3_hex}",
        payload_bytes.len()
    );

    let cache_path = external_cache_path(family_id, cache_key);
    ensure_parent_dirs(&cache_path).await?;
    tokio::fs::write(&cache_path, &payload_bytes)
        .await
        .map_err(|e| {
            NestGateError::io_error(format!("Failed to cache payload for {cache_key}: {e}"))
        })?;

    let meta = json!({
        "url": url, "cache_key": cache_key, "blake3": blake3_hex,
        "size": payload_bytes.len(), "content_type": content_type,
        "fetched_at": chrono::Utc::now().to_rfc3339(), "family_id": family_id,
    });
    tokio::fs::write(
        external_meta_path(family_id, cache_key),
        serde_json::to_vec_pretty(&meta).unwrap_or_default(),
    )
    .await
    .map_err(|e| {
        NestGateError::io_error(format!(
            "Failed to write cache metadata for {cache_key}: {e}"
        ))
    })?;

    let value: Value = serde_json::from_slice(&payload_bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&payload_bytes).into_owned()));
    Ok(json!({
        "value": value, "data": value, "key": cache_key,
        "blake3": blake3_hex, "url": url, "size": payload_bytes.len(),
        "content_type": content_type, "cached": false, "family_id": family_id,
    }))
}

/// `storage.fetch_external` — `NestGate` owns the TLS boundary.
///
/// Accepts a URL, fetches it over HTTPS, content-addresses the payload with
/// BLAKE3, caches it on disk, and returns the payload with provenance metadata.
/// Springs never open network connections — this capability is the single
/// point of external data ingestion for the ecosystem.
///
/// # Errors
///
/// Returns an error if required params are missing, the URL scheme is not
/// HTTPS (or HTTP when explicitly allowed), the fetch fails, or disk I/O fails.
pub(super) async fn storage_fetch_external(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let url = params["url"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("url", "url (string) required"))?;
    let cache_key = params["cache_key"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("cache_key", "cache_key (string) required")
    })?;

    validate_fetch_url(
        url,
        params["allow_insecure_http"].as_bool().unwrap_or(false),
    )?;
    let family_id = resolve_family_id(params, state)?;

    if let Some(cached) = try_cached_external(url, cache_key, family_id).await? {
        return Ok(cached);
    }
    let timeout = params["timeout_secs"].as_u64().unwrap_or(60);
    do_external_fetch(url, cache_key, family_id, timeout).await
}

/// Recursively list all file keys under a dataset directory.
///
/// Keys are returned as relative paths from `root` (e.g. `test/primalspring/hello`
/// for a file at `{root}/test/primalspring/hello`). Directories are traversed but
/// not returned as keys themselves — only files are keys.
fn list_keys_recursive<'a>(
    dir: &'a Path,
    root: &'a Path,
    prefix: Option<&'a str>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Vec<String>> + Send + 'a>> {
    Box::pin(async move {
        let mut keys = Vec::new();
        let Ok(mut entries) = tokio::fs::read_dir(dir).await else {
            return keys;
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.is_dir() {
                keys.extend(list_keys_recursive(&path, root, prefix).await);
            } else if let Ok(relative) = path.strip_prefix(root) {
                let key = relative.to_string_lossy().to_string();
                if let Some(p) = prefix {
                    if key.starts_with(p) {
                        keys.push(key);
                    }
                } else {
                    keys.push(key);
                }
            }
        }
        keys
    })
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
        }
    }

    #[tokio::test]
    async fn resolve_family_id_from_params() {
        let state = mock_state(Some("server-family")).await;
        let params = json!({"family_id": "explicit-family"});
        let result = resolve_family_id(&params, &state).unwrap();
        assert_eq!(result, "explicit-family");
    }

    #[tokio::test]
    async fn resolve_family_id_falls_back_to_state() {
        let state = mock_state(Some("server-family")).await;
        let params = json!({"key": "some-key"});
        let result = resolve_family_id(&params, &state).unwrap();
        assert_eq!(result, "server-family");
    }

    #[tokio::test]
    async fn resolve_family_id_errors_when_missing() {
        let state = mock_state(None).await;
        let params = json!({"key": "some-key"});
        let result = resolve_family_id(&params, &state);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_family_id_param_overrides_state() {
        let state = mock_state(Some("default")).await;
        let params = json!({"family_id": "override"});
        let result = resolve_family_id(&params, &state).unwrap();
        assert_eq!(result, "override");
    }

    #[tokio::test]
    async fn storage_store_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_store(None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_store_requires_key() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"family_id": "test", "dataset": "ds"}));
        let result = storage_store(params.as_ref(), &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_retrieve_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_retrieve(None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_exists_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_exists(None, &state);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_delete_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_delete(None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_list_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_list(None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_store_and_retrieve_round_trip() {
        let state = mock_state(Some("test-rt")).await;
        let family_id = format!("test-dataset-{}", uuid::Uuid::new_v4());

        let store_params = Some(json!({
            "family_id": &family_id,
            "key": "hello",
            "value": "world"
        }));
        let store_result = storage_store(store_params.as_ref(), &state).await;
        assert!(store_result.is_ok(), "store failed: {store_result:?}");
        assert_eq!(store_result.unwrap()["status"], "stored");

        let retrieve_params = Some(json!({
            "family_id": &family_id,
            "key": "hello"
        }));
        let retrieve_result = storage_retrieve(retrieve_params.as_ref(), &state).await;
        assert!(
            retrieve_result.is_ok(),
            "retrieve failed: {retrieve_result:?}"
        );
        assert_eq!(retrieve_result.unwrap()["value"], "world");

        let exists_params = json!({"family_id": &family_id, "key": "hello"});
        let exists_result = storage_exists(Some(&exists_params), &state);
        assert!(exists_result.is_ok());
        assert_eq!(exists_result.unwrap()["exists"], true);

        let delete_params = json!({"family_id": &family_id, "key": "hello"});
        let delete_result = storage_delete(Some(&delete_params), &state).await;
        assert!(delete_result.is_ok(), "delete failed: {delete_result:?}");
        assert_eq!(delete_result.unwrap()["status"], "deleted");

        let gone = storage_exists(
            Some(&json!({"family_id": &family_id, "key": "hello"})),
            &state,
        );
        assert_eq!(gone.unwrap()["exists"], false);

        // Cleanup
        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn storage_list_returns_stored_keys() {
        let state = mock_state(Some("test-list")).await;
        let family_id = format!("test-list-{}", uuid::Uuid::new_v4());

        let p1 = json!({"family_id": &family_id, "key": "a", "value": "1"});
        assert!(storage_store(Some(&p1), &state).await.is_ok());
        let p2 = json!({"family_id": &family_id, "key": "b", "value": "2"});
        assert!(storage_store(Some(&p2), &state).await.is_ok());

        let list_params = json!({"family_id": &family_id});
        let list_result = storage_list(Some(&list_params), &state).await;
        assert!(list_result.is_ok());
        let keys = list_result.unwrap();
        let key_arr = keys["keys"].as_array().expect("keys array");
        assert_eq!(key_arr.len(), 2, "expected 2 keys; got {key_arr:?}");

        // Cleanup
        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn storage_nested_key_paths_work() {
        let state = mock_state(Some("test-nested")).await;
        let family_id = format!("test-nested-{}", uuid::Uuid::new_v4());

        let store_p = json!({"family_id": &family_id, "key": "deep/path/key", "value": "nested"});
        let store_result = storage_store(Some(&store_p), &state).await;
        assert!(store_result.is_ok(), "nested store: {store_result:?}");

        let retrieve_p = json!({"family_id": &family_id, "key": "deep/path/key"});
        let retrieve_result = storage_retrieve(Some(&retrieve_p), &state).await;
        assert!(retrieve_result.is_ok());
        assert_eq!(retrieve_result.unwrap()["value"], "nested");

        let list_p = json!({"family_id": &family_id, "prefix": "deep/"});
        let list_result = storage_list(Some(&list_p), &state).await;
        assert!(list_result.is_ok());
        let keys = list_result.unwrap()["keys"].as_array().unwrap().clone();
        assert!(keys.iter().any(|k| k == "deep/path/key"));

        // Cleanup
        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn storage_blob_round_trip() {
        let state = mock_state(Some("test-blob")).await;
        let family_id = format!("test-blob-{}", uuid::Uuid::new_v4());
        let raw_data = b"binary payload \x00\xff\xfe";
        let encoded = base64::engine::general_purpose::STANDARD.encode(raw_data);

        let store_p = json!({"family_id": &family_id, "key": "binfile", "blob": encoded});
        let store_result = storage_store_blob(Some(&store_p), &state).await;
        assert!(store_result.is_ok(), "blob store: {store_result:?}");

        let retrieve_p = json!({"family_id": &family_id, "key": "binfile"});
        let retrieve_result = storage_retrieve_blob(Some(&retrieve_p), &state).await;
        assert!(retrieve_result.is_ok());
        let blob_b64 = retrieve_result.unwrap()["blob"]
            .as_str()
            .unwrap()
            .to_string();
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&blob_b64)
            .unwrap();
        assert_eq!(decoded, raw_data);

        // Cleanup
        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn fetch_external_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_fetch_external(None, &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_external_requires_url() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"cache_key": "ck", "family_id": "test"}));
        let result = storage_fetch_external(params.as_ref(), &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_external_requires_cache_key() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"url": "https://example.com", "family_id": "test"}));
        let result = storage_fetch_external(params.as_ref(), &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_external_rejects_ftp_scheme() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({
            "url": "ftp://example.com/file",
            "cache_key": "ck",
            "family_id": "test"
        }));
        let result = storage_fetch_external(params.as_ref(), &state).await;
        assert!(result.is_err());
        let err = format!("{}", result.unwrap_err());
        assert!(
            err.contains("not allowed"),
            "expected scheme error, got: {err}"
        );
    }

    #[tokio::test]
    async fn fetch_external_rejects_http_by_default() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({
            "url": "http://example.com/file",
            "cache_key": "ck",
            "family_id": "test"
        }));
        let result = storage_fetch_external(params.as_ref(), &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_external_allows_http_when_opted_in() {
        let _state = mock_state(Some("test")).await;
        let params = json!({
            "url": "http://example.com/file",
            "cache_key": "ck",
            "family_id": "test",
            "allow_insecure_http": true
        });
        // URL validation should pass (actual fetch may fail due to network)
        let result = validate_fetch_url(
            params["url"].as_str().unwrap(),
            params["allow_insecure_http"].as_bool().unwrap_or(false),
        );
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn fetch_external_validates_url_format() {
        let result = validate_fetch_url("not a url", false);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_external_cache_miss_then_hit() {
        let family_id = format!("test-fetch-{}", uuid::Uuid::new_v4());

        // Pre-populate a cache entry manually to test cache hit path
        let cache_path = external_cache_path(&family_id, "manual-cache");
        let meta_path = external_meta_path(&family_id, "manual-cache");
        ensure_parent_dirs(&cache_path).await.unwrap();

        let payload = br#"{"result": "cached_data"}"#;
        tokio::fs::write(&cache_path, payload).await.unwrap();

        let blake3_hex = blake3::hash(payload).to_hex().to_string();
        let meta = json!({
            "url": "https://example.com/cached",
            "blake3": blake3_hex,
            "size": payload.len(),
        });
        tokio::fs::write(&meta_path, serde_json::to_vec_pretty(&meta).unwrap())
            .await
            .unwrap();

        // Cache hit should return without network
        let state = mock_state(Some(&family_id)).await;
        let params = json!({
            "url": "https://example.com/cached",
            "cache_key": "manual-cache",
            "family_id": &family_id,
        });
        let result = storage_fetch_external(Some(&params), &state).await;
        assert!(result.is_ok(), "cache hit failed: {result:?}");
        let val = result.unwrap();
        assert_eq!(val["cached"], true);
        assert_eq!(val["blake3"], blake3_hex);
        assert_eq!(val["value"]["result"], "cached_data");

        // Cleanup
        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }
}
