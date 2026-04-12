// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage JSON-RPC Handlers — core key/value CRUD on the dataset filesystem layout.
//!
//! Blob helpers live in [`super::blob_handlers`]. External fetch and object metadata in
//! [`super::external_handlers`]. See [`super::bonding_handlers`] for `bonding.ledger.*`.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use tracing::debug;

use super::StorageState;

/// Build the filesystem path for a key in a family's dataset.
///
/// Layout matches `nestgate-core` `operations::objects`: `{base}/datasets/{family}/{key}`.
pub(in crate::rpc::unix_socket_server) fn dataset_key_path(family_id: &str, key: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join(key)
}

/// Build the filesystem path for a binary blob.
///
/// Blobs are stored separately under `{base}/datasets/{family}/_blobs/{key}` so list
/// operations can distinguish JSON objects from raw blobs.
pub(in crate::rpc::unix_socket_server) fn blob_key_path(family_id: &str, key: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_blobs")
        .join(key)
}

/// Ensure all parent directories of `path` exist.
pub(in crate::rpc::unix_socket_server) async fn ensure_parent_dirs(path: &Path) -> Result<()> {
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
/// point identified in downstream composition experiments.
pub(in crate::rpc::unix_socket_server) fn resolve_family_id<'a>(
    params: &'a Value,
    state: &'a StorageState,
) -> Result<&'a str> {
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
}
