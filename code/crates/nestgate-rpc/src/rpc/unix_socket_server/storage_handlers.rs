// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage JSON-RPC Handlers — core key/value CRUD on the dataset filesystem layout.
//!
//! Path helpers and shared utilities live in [`super::storage_paths`].
//! Blob helpers live in [`super::blob_handlers`]. External fetch and object metadata in
//! [`super::external_handlers`]. See [`super::bonding_handlers`] for `bonding.ledger.*`.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::Path;
use tracing::debug;

use super::StorageState;
use super::storage_paths::{
    dataset_key_path, ensure_parent_dirs, extract_namespace, resolve_family_id,
};

/// storage.store - Store key-value data (filesystem-backed, durable)
///
/// Accepts optional `namespace` for cross-spring scoped storage.
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
    let namespace = extract_namespace(params)?;

    let serialized = serde_json::to_vec_pretty(data)
        .map_err(|e| NestGateError::io_error(format!("Failed to serialize value: {e}")))?;

    let bytes = if let Some(ref enc) = state.encryption {
        enc.encrypt(&serialized)?
    } else {
        serialized
    };

    debug!(
        "storage.store: family_id='{}', namespace={:?}, key='{}', value_size={} bytes",
        family_id,
        namespace,
        key,
        bytes.len()
    );

    let object_path = dataset_key_path(family_id, namespace, key);
    ensure_parent_dirs(&object_path).await?;
    tokio::fs::write(&object_path, &bytes)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to write {family_id}/{key}: {e}")))?;

    let mut resp = json!({"status": "stored", "key": key, "family_id": family_id});
    if let Some(ns) = namespace {
        resp["namespace"] = json!(ns);
    }
    Ok(resp)
}

/// Maximum payload size for in-memory `storage.retrieve` (64 MiB). Objects
/// larger than this must be read via `storage.retrieve_stream` /
/// `storage.retrieve_range` to avoid unbounded memory growth.
const RETRIEVE_MAX_INLINE: u64 = 64 * 1024 * 1024;

/// storage.retrieve - Retrieve data by key (filesystem-backed, durable)
///
/// Accepts optional `namespace` for cross-spring scoped storage.
/// When namespace is provided and the namespaced path doesn't exist, falls
/// back to the flat legacy path for migration compatibility.
///
/// Returns an error for objects exceeding [`RETRIEVE_MAX_INLINE`] with
/// guidance to use `storage.retrieve_stream` or `storage.retrieve_range`.
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
    let namespace = extract_namespace(params)?;

    debug!(
        "storage.retrieve: family_id='{}', namespace={:?}, key='{}'",
        family_id, namespace, key
    );

    let object_path = dataset_key_path(family_id, namespace, key);
    let resolved_path = if object_path.exists() {
        object_path
    } else if namespace.is_some() {
        let flat = dataset_key_path(family_id, None, key);
        if flat.exists() {
            flat
        } else {
            return Ok(json!({"value": null, "data": null, "key": key}));
        }
    } else {
        return Ok(json!({"value": null, "data": null, "key": key}));
    };

    let metadata = tokio::fs::metadata(&resolved_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to stat {family_id}/{key}: {e}")))?;
    if metadata.len() > RETRIEVE_MAX_INLINE {
        return Err(NestGateError::validation_error(format!(
            "Object {key} is {} bytes — exceeds inline limit ({RETRIEVE_MAX_INLINE}). \
             Use storage.retrieve_stream or storage.retrieve_range for large payloads.",
            metadata.len()
        )));
    }

    let raw_bytes = tokio::fs::read(&resolved_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read {family_id}/{key}: {e}")))?;

    let bytes =
        if crate::rpc::storage_encryption::StorageEncryption::is_encrypted_envelope(&raw_bytes) {
            if let Some(ref enc) = state.encryption {
                enc.decrypt(&raw_bytes)?
            } else {
                raw_bytes
            }
        } else {
            raw_bytes
        };

    let value: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));

    let mut resp = json!({"value": value, "data": value, "key": key, "family_id": family_id});
    if let Some(ns) = namespace {
        resp["namespace"] = json!(ns);
    }
    Ok(resp)
}

/// storage.exists - Check if data exists by key (filesystem-backed)
///
/// Accepts optional `namespace`. Falls back to flat layout when namespaced
/// path does not exist.
pub(super) fn storage_exists(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;
    let namespace = extract_namespace(params)?;

    let ns_path = dataset_key_path(family_id, namespace, key);
    let exists = ns_path.exists()
        || (namespace.is_some() && dataset_key_path(family_id, None, key).exists());

    let mut resp = json!({"exists": exists, "key": key, "family_id": family_id});
    if let Some(ns) = namespace {
        resp["namespace"] = json!(ns);
    }
    Ok(resp)
}

/// storage.delete - Delete data by key (filesystem-backed)
///
/// Accepts optional `namespace`. When namespace is provided, deletes from
/// the namespaced path; the flat legacy path is NOT deleted automatically.
pub(super) async fn storage_delete(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;
    let namespace = extract_namespace(params)?;

    let object_path = dataset_key_path(family_id, namespace, key);
    if object_path.exists() {
        tokio::fs::remove_file(&object_path).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to delete {family_id}/{key}: {e}"))
        })?;
    }
    let mut resp = json!({"status": "deleted", "key": key, "family_id": family_id});
    if let Some(ns) = namespace {
        resp["namespace"] = json!(ns);
    }
    Ok(resp)
}

/// storage.list - List all keys with optional prefix
///
/// Accepts optional `namespace` to scope listing. When provided, lists only
/// keys under `{family}/{namespace}/`. When omitted, lists all keys under
/// `{family}/` (legacy behavior).
pub(super) async fn storage_list(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = resolve_family_id(params, state)?;
    let namespace = extract_namespace(params)?;
    let prefix = params["prefix"].as_str();

    let family_root = get_storage_base_path().join("datasets").join(family_id);
    let scan_root = namespace.map_or_else(|| family_root.clone(), |ns| family_root.join(ns));

    let keys = list_keys_recursive(&scan_root, &scan_root, prefix).await;

    debug!(
        "Listed {} keys for family '{}', namespace={:?} (prefix: {:?})",
        keys.len(),
        family_id,
        namespace,
        prefix
    );

    let mut resp = json!({"keys": keys});
    if let Some(ns) = namespace {
        resp["namespace"] = json!(ns);
    }
    Ok(resp)
}

/// storage.stats - Get storage statistics
///
/// Accepts optional `namespace` to scope stats. When provided, counts only
/// keys under `{family}/{namespace}/`.
pub(super) async fn storage_stats(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = resolve_family_id(params, state)?;
    let namespace = extract_namespace(params)?;

    let family_root = get_storage_base_path().join("datasets").join(family_id);
    let scan_root = namespace.map_or_else(|| family_root.clone(), |ns| family_root.join(ns));

    let keys = list_keys_recursive(&scan_root, &scan_root, None).await;
    let key_count = keys.len();

    debug!(
        "Stats for family '{}', namespace={:?}: {} objects",
        family_id, namespace, key_count
    );

    let mut resp = json!({
        "key_count": key_count,
        "blob_count": 0,
        "family_id": family_id
    });
    if let Some(ns) = namespace {
        resp["namespace"] = json!(ns);
    }
    Ok(resp)
}

/// Recursively list all file keys under a dataset directory.
///
/// Keys are returned as relative paths from `root` (e.g. `test/myapp/hello`
/// for a file at `{root}/test/myapp/hello`). Directories are traversed but
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

/// `storage.namespaces.list` — enumerate namespaces under a family's dataset directory.
///
/// Returns all subdirectories (excluding underscore-prefixed internals like `_blobs`).
pub(super) async fn storage_namespaces_list(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let empty_params = json!({});
    let family_id = resolve_family_id(params.unwrap_or(&empty_params), state)?;

    let family_dir = get_storage_base_path().join("datasets").join(family_id);
    let mut namespaces = Vec::new();
    if family_dir.exists() {
        let mut entries = tokio::fs::read_dir(&family_dir).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to list namespaces for {family_id}: {e}"))
        })?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with('_')
                && entry
                    .file_type()
                    .await
                    .map(|ft| ft.is_dir())
                    .unwrap_or(false)
            {
                namespaces.push(name.to_string());
            }
        }
    }
    namespaces.sort();
    Ok(json!({
        "namespaces": namespaces,
        "family_id": family_id,
        "count": namespaces.len()
    }))
}
