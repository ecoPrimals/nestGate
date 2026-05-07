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
        if flat.exists() { flat } else {
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

    let family_root = get_storage_base_path()
        .join("datasets")
        .join(family_id);
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

    let family_root = get_storage_base_path()
        .join("datasets")
        .join(family_id);
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

    /// GAP-21: family_id omitted in params — server uses its own family as default.
    /// This is the primary composition pattern: springs connect to a family-scoped
    /// socket and should not need to repeat the family in every request.
    #[tokio::test]
    async fn store_retrieve_without_family_id_uses_server_default() {
        let server_family = format!("gap21-test-{}", uuid::Uuid::new_v4());
        let state = mock_state(Some(&server_family)).await;

        let store_params =
            Some(json!({"key": "gap21-key", "value": {"msg": "no family in params"}}));
        let store_result = storage_store(store_params.as_ref(), &state).await;
        assert!(
            store_result.is_ok(),
            "store without family_id failed: {store_result:?}"
        );
        let store_val = store_result.unwrap();
        assert_eq!(store_val["status"], "stored");
        assert_eq!(store_val["family_id"], server_family.as_str());

        let retrieve_params = Some(json!({"key": "gap21-key"}));
        let retrieve_result = storage_retrieve(retrieve_params.as_ref(), &state).await;
        assert!(
            retrieve_result.is_ok(),
            "retrieve without family_id failed: {retrieve_result:?}"
        );
        assert_eq!(
            retrieve_result.unwrap()["value"]["msg"],
            "no family in params"
        );

        let exists_params = json!({"key": "gap21-key"});
        let exists_result = storage_exists(Some(&exists_params), &state);
        assert!(exists_result.is_ok());
        assert_eq!(exists_result.unwrap()["exists"], true);

        let list_params = json!({});
        let list_result = storage_list(Some(&list_params), &state).await;
        assert!(
            list_result.is_ok(),
            "list without family_id failed: {list_result:?}"
        );

        let delete_params = json!({"key": "gap21-key"});
        let delete_result = storage_delete(Some(&delete_params), &state).await;
        assert!(
            delete_result.is_ok(),
            "delete without family_id failed: {delete_result:?}"
        );

        let _ = tokio::fs::remove_dir_all(
            get_storage_base_path()
                .join("datasets")
                .join(&server_family),
        )
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
    async fn namespaces_list_returns_dirs_only() {
        let state = mock_state(Some("test-ns")).await;
        let family_id = format!("test-ns-{}", uuid::Uuid::new_v4());
        let base = get_storage_base_path().join("datasets").join(&family_id);

        tokio::fs::create_dir_all(base.join("shared"))
            .await
            .unwrap();
        tokio::fs::create_dir_all(base.join("private"))
            .await
            .unwrap();
        tokio::fs::create_dir_all(base.join("_blobs"))
            .await
            .unwrap();

        let params = json!({"family_id": &family_id});
        let result = storage_namespaces_list(Some(&params), &state)
            .await
            .unwrap();
        let ns = result["namespaces"].as_array().unwrap();
        assert_eq!(ns.len(), 2, "expected 2 namespaces (not _blobs): {ns:?}");
        assert_eq!(ns[0], "private");
        assert_eq!(ns[1], "shared");
        assert_eq!(result["count"], 2);

        let _ = tokio::fs::remove_dir_all(&base).await;
    }

    #[tokio::test]
    async fn namespaces_list_empty_for_missing_family() {
        let state = mock_state(Some("test-ns-missing")).await;
        let params = json!({"family_id": "nonexistent-family-12345"});
        let result = storage_namespaces_list(Some(&params), &state)
            .await
            .unwrap();
        assert_eq!(result["count"], 0);
        assert!(result["namespaces"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn namespaces_list_uses_state_family_id() {
        let family_id = format!("test-ns-state-{}", uuid::Uuid::new_v4());
        let state = mock_state(Some(&family_id)).await;
        let base = get_storage_base_path().join("datasets").join(&family_id);

        tokio::fs::create_dir_all(base.join("default"))
            .await
            .unwrap();

        let result = storage_namespaces_list(None, &state).await.unwrap();
        assert_eq!(result["count"], 1);
        assert_eq!(result["family_id"], family_id);

        let _ = tokio::fs::remove_dir_all(&base).await;
    }

    fn encrypted_state(family_id: &str) -> StorageState {
        let mut key = [0u8; 32];
        for (i, b) in key.iter_mut().enumerate() {
            *b = i as u8;
        }
        StorageState {
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
            family_id: Some(family_id.to_string()),
            storage_initialized: true,
            encryption: Some(std::sync::Arc::new(
                crate::rpc::storage_encryption::StorageEncryption::new(key),
            )),
        }
    }

    #[tokio::test]
    async fn encrypted_store_and_retrieve_round_trip() {
        let family_id = format!("test-enc-rt-{}", uuid::Uuid::new_v4());
        let state = encrypted_state(&family_id);

        let store_params = Some(json!({
            "family_id": &family_id,
            "key": "secret",
            "value": {"msg": "classified data"}
        }));
        let store_result = storage_store(store_params.as_ref(), &state).await;
        assert!(store_result.is_ok(), "encrypted store: {store_result:?}");

        let retrieve_result = storage_retrieve(
            Some(&json!({"family_id": &family_id, "key": "secret"})),
            &state,
        )
        .await;
        assert!(
            retrieve_result.is_ok(),
            "encrypted retrieve: {retrieve_result:?}"
        );
        let val = retrieve_result.unwrap();
        assert_eq!(val["value"]["msg"], "classified data");

        // Verify on-disk data is an encrypted envelope, not plaintext
        let on_disk = tokio::fs::read(dataset_key_path(&family_id, None, "secret"))
            .await
            .expect("read disk");
        assert!(
            crate::rpc::storage_encryption::StorageEncryption::is_encrypted_envelope(&on_disk),
            "on-disk data should be an encrypted envelope"
        );
        let disk_str = String::from_utf8_lossy(&on_disk);
        assert!(
            !disk_str.contains("classified"),
            "plaintext must not appear on disk"
        );

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn encrypted_state_reads_unencrypted_data() {
        let family_id = format!("test-enc-compat-{}", uuid::Uuid::new_v4());
        let plain_state = mock_state(Some(&family_id)).await;

        let p = json!({"family_id": &family_id, "key": "plain", "value": "hello"});
        assert!(storage_store(Some(&p), &plain_state).await.is_ok());

        let enc_state = encrypted_state(&family_id);
        let result = storage_retrieve(
            Some(&json!({"family_id": &family_id, "key": "plain"})),
            &enc_state,
        )
        .await;
        assert!(result.is_ok(), "backward-compat retrieve: {result:?}");
        assert_eq!(result.unwrap()["value"], "hello");

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn namespace_store_and_retrieve_round_trip() {
        let state = mock_state(Some("test-ns-rt")).await;
        let family_id = format!("test-ns-rt-{}", uuid::Uuid::new_v4());

        let store = json!({
            "family_id": &family_id,
            "namespace": "experiments",
            "key": "run-42",
            "value": {"accuracy": 0.95}
        });
        let result = storage_store(Some(&store), &state).await.unwrap();
        assert_eq!(result["status"], "stored");
        assert_eq!(result["namespace"], "experiments");

        let retrieve = json!({
            "family_id": &family_id,
            "namespace": "experiments",
            "key": "run-42"
        });
        let val = storage_retrieve(Some(&retrieve), &state).await.unwrap();
        assert_eq!(val["value"]["accuracy"], 0.95);
        assert_eq!(val["namespace"], "experiments");

        let flat = json!({"family_id": &family_id, "key": "run-42"});
        let flat_val = storage_retrieve(Some(&flat), &state).await.unwrap();
        assert!(
            flat_val["value"].is_null(),
            "flat path should NOT find namespaced data"
        );

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn namespace_retrieve_falls_back_to_flat() {
        let state = mock_state(Some("test-ns-fb")).await;
        let family_id = format!("test-ns-fb-{}", uuid::Uuid::new_v4());

        let store_flat = json!({
            "family_id": &family_id,
            "key": "legacy-data",
            "value": "old"
        });
        assert!(storage_store(Some(&store_flat), &state).await.is_ok());

        let retrieve_ns = json!({
            "family_id": &family_id,
            "namespace": "shared",
            "key": "legacy-data"
        });
        let val = storage_retrieve(Some(&retrieve_ns), &state).await.unwrap();
        assert_eq!(
            val["value"], "old",
            "namespace retrieve should fall back to flat path"
        );

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn namespace_list_scopes_to_namespace() {
        let state = mock_state(Some("test-ns-list")).await;
        let family_id = format!("test-ns-list-{}", uuid::Uuid::new_v4());

        let s1 = json!({
            "family_id": &family_id, "namespace": "alpha",
            "key": "a1", "value": "x"
        });
        let s2 = json!({
            "family_id": &family_id, "namespace": "beta",
            "key": "b1", "value": "y"
        });
        assert!(storage_store(Some(&s1), &state).await.is_ok());
        assert!(storage_store(Some(&s2), &state).await.is_ok());

        let list_alpha = json!({"family_id": &family_id, "namespace": "alpha"});
        let keys = storage_list(Some(&list_alpha), &state).await.unwrap();
        let arr = keys["keys"].as_array().unwrap();
        assert_eq!(arr.len(), 1, "alpha should have 1 key");
        assert_eq!(arr[0], "a1");
        assert_eq!(keys["namespace"], "alpha");

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn namespace_rejects_path_traversal() {
        let params = json!({"namespace": "../escape"});
        assert!(extract_namespace(&params).is_err());

        let params = json!({"namespace": "_internal"});
        assert!(extract_namespace(&params).is_err());

        let params = json!({"namespace": "valid-name"});
        assert_eq!(extract_namespace(&params).unwrap(), Some("valid-name"));

        let params = json!({});
        assert_eq!(extract_namespace(&params).unwrap(), None);
    }
}
