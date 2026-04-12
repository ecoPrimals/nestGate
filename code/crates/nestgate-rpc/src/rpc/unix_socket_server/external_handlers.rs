// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! External storage operations: HTTPS `storage.fetch_external` and `storage.object.size`.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::PathBuf;
use tracing::debug;

use super::StorageState;
use super::storage_handlers::{
    blob_key_path, dataset_key_path, ensure_parent_dirs, resolve_family_id,
};

/// `storage.object.size` — get size of a stored object without reading its content.
///
/// Returns `{size, exists, key, family_id, storage_type}` where `storage_type` is
/// `"blob"`, `"object"`, or `"none"`.
pub(super) async fn storage_object_size(
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
    let object_path = dataset_key_path(family_id, key);

    let (exists, size, storage_type) = if blob_path.exists() {
        let meta = tokio::fs::metadata(&blob_path).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to stat blob {family_id}/{key}: {e}"))
        })?;
        (true, meta.len(), "blob")
    } else if object_path.exists() {
        let meta = tokio::fs::metadata(&object_path).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to stat object {family_id}/{key}: {e}"))
        })?;
        (true, meta.len(), "object")
    } else {
        (false, 0, "none")
    };

    Ok(json!({
        "exists": exists,
        "size": size,
        "key": key,
        "family_id": family_id,
        "storage_type": storage_type
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
///
/// Uses `ureq` (pure-Rust HTTP) with `rustls-rustcrypto` (pure-Rust TLS) —
/// zero C/ASM dependencies, fully ecoBin-compliant.  The sync HTTP call runs
/// inside `spawn_blocking` so the tokio runtime stays unblocked.
async fn do_external_fetch(
    url: &str,
    cache_key: &str,
    family_id: &str,
    timeout_secs: u64,
) -> Result<Value> {
    debug!("storage.fetch_external: fetching '{url}' as '{cache_key}' (family={family_id})");

    let url_owned = url.to_owned();
    let timeout = std::time::Duration::from_secs(timeout_secs);
    let user_agent = format!("NestGate/{}", env!("CARGO_PKG_VERSION"));

    let (content_type, payload_bytes) = tokio::task::spawn_blocking(move || -> Result<_> {
        let _ = rustls_rustcrypto::provider().install_default();

        let config = ureq::Agent::config_builder()
            .timeout_global(Some(timeout))
            .https_only(false)
            .build();
        let agent = ureq::Agent::new_with_config(config);

        let mut response = agent
            .get(&url_owned)
            .header("User-Agent", &user_agent)
            .call()
            .map_err(|e| NestGateError::io_error(format!("Fetch failed for {url_owned}: {e}")))?;

        let ct = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/octet-stream")
            .to_owned();

        let body = response
            .body_mut()
            .read_to_vec()
            .map_err(|e| NestGateError::io_error(format!("Failed to read body: {e}")))?;

        Ok::<_, NestGateError>((ct, body))
    })
    .await
    .map_err(|e| NestGateError::io_error(format!("Fetch task failed: {e}")))??;

    let payload_bytes = bytes::Bytes::from(payload_bytes);

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

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use nestgate_config::config::storage_paths::get_storage_base_path;
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

    #[tokio::test]
    async fn object_size_requires_params() {
        let state = mock_state(Some("test")).await;
        assert!(storage_object_size(None, &state).await.is_err());
    }

    #[tokio::test]
    async fn object_size_returns_none_for_missing() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"key": "ghost-key-999", "family_id": "test"}));
        let result = storage_object_size(params.as_ref(), &state).await.unwrap();
        assert_eq!(result["exists"], false);
        assert_eq!(result["size"], 0);
        assert_eq!(result["storage_type"], "none");
    }

    #[tokio::test]
    async fn object_size_returns_correct_size_for_blob() {
        let state = mock_state(Some("test-size")).await;
        let family_id = format!("test-size-{}", uuid::Uuid::new_v4());
        let payload = vec![0u8; 2048];
        let encoded = base64::engine::general_purpose::STANDARD.encode(&payload);

        let store_p = json!({"family_id": &family_id, "key": "sized", "blob": encoded});
        assert!(
            super::super::blob_handlers::storage_store_blob(Some(&store_p), &state)
                .await
                .is_ok()
        );

        let size_p = json!({"family_id": &family_id, "key": "sized"});
        let result = storage_object_size(Some(&size_p), &state).await.unwrap();
        assert_eq!(result["exists"], true);
        assert_eq!(result["size"], 2048);
        assert_eq!(result["storage_type"], "blob");

        // Cleanup
        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }
}
