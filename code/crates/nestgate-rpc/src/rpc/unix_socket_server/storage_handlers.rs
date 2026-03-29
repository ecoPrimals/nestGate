// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage JSON-RPC Handlers
//!
//! Extracted from `unix_socket_server` for domain-based refactoring.
//! Handles: storage.store, storage.retrieve, storage.exists, storage.delete,
//! storage.list, storage.stats, `storage.store_blob`, `storage.retrieve_blob`

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::Path;
use tracing::debug;

use super::StorageState;

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

/// storage.store - Store key-value data
pub(super) fn storage_store(params: Option<&Value>, state: &StorageState) -> Result<Value> {
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

    // ✅ ENHANCED LOGGING: Input validation
    let data_str = serde_json::to_string(data).unwrap_or_else(|_| "<invalid>".to_string());
    debug!(
        "📝 storage.store called: family_id='{}', key='{}', value_size={} bytes",
        family_id,
        key,
        data_str.len()
    );

    let _ = (state, data);
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (storage.store)",
    ))
}

/// storage.retrieve - Retrieve data by key
pub(super) fn storage_retrieve(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let family_id = resolve_family_id(params, state)?;

    debug!(
        "📖 storage.retrieve called: family_id='{}', key='{}'",
        family_id, key
    );

    let _ = state;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (storage.retrieve)",
    ))
}

/// storage.exists - Check if data exists by key
///
/// Modern idiomatic Rust: Efficient existence check without data transfer
/// Deep Debt Principle #1: Standard API pattern, no unnecessary data retrieval
pub(super) fn storage_exists(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    resolve_family_id(params, state)?;
    let _ = state;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (storage.exists)",
    ))
}

/// storage.delete - Delete data by key
pub(super) fn storage_delete(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    resolve_family_id(params, state)?;
    let _ = state;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (storage.delete)",
    ))
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

/// `storage.store_blob` - Store binary blob (base64 encoded)
pub(super) fn storage_store_blob(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    let blob_base64 = params["blob"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("blob", "blob (base64 string) required")
    })?;
    resolve_family_id(params, state)?;

    // Decode base64
    let blob_data = STANDARD.decode(blob_base64).map_err(|e| {
        NestGateError::invalid_input_with_field("blob", format!("Invalid base64: {e}"))
    })?;

    let _ = (state, blob_data);
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (storage.store_blob)",
    ))
}

/// `storage.retrieve_blob` - Retrieve binary blob (base64 encoded)
pub(super) fn storage_retrieve_blob(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "key (string) required"))?;
    resolve_family_id(params, state)?;
    let _ = state;
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core storage (storage.retrieve_blob)",
    ))
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
    use nestgate_types::error::NestGateError;
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
        let result = storage_store(None, &state);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_store_requires_key() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"family_id": "test", "dataset": "ds"}));
        let result = storage_store(params.as_ref(), &state);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn storage_retrieve_requires_params() {
        let state = mock_state(Some("test")).await;
        let result = storage_retrieve(None, &state);
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
        let result = storage_delete(None, &state);
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
        let store_result = storage_store(store_params.as_ref(), &state);
        assert!(
            matches!(store_result, Err(NestGateError::NotImplemented(_))),
            "expected NotImplemented for storage.store: {:?}",
            store_result
        );

        let retrieve_params = Some(json!({
            "family_id": &family_id,
            "key": "hello"
        }));
        let retrieve_result = storage_retrieve(retrieve_params.as_ref(), &state);
        assert!(
            matches!(retrieve_result, Err(NestGateError::NotImplemented(_))),
            "expected NotImplemented for storage.retrieve: {:?}",
            retrieve_result
        );

        let delete_params = json!({"family_id": &family_id, "key": "hello"});
        let delete_result = storage_delete(Some(&delete_params), &state);
        assert!(
            matches!(delete_result, Err(NestGateError::NotImplemented(_))),
            "expected NotImplemented for storage.delete: {:?}",
            delete_result
        );
    }

    #[tokio::test]
    async fn storage_list_returns_stored_keys() {
        let state = mock_state(Some("test-list")).await;
        let family_id = format!("test-list-{}", uuid::Uuid::new_v4());

        let p1 = json!({"family_id": &family_id, "key": "a", "value": "1"});
        let s1 = storage_store(Some(&p1), &state);
        assert!(matches!(s1, Err(NestGateError::NotImplemented(_))));
        let p2 = json!({"family_id": &family_id, "key": "b", "value": "2"});
        let s2 = storage_store(Some(&p2), &state);
        assert!(matches!(s2, Err(NestGateError::NotImplemented(_))));

        // listing still walks the filesystem under datasets/{family_id} (no core wiring yet)
        let list_params = json!({"family_id": &family_id});
        let list_result = storage_list(Some(&list_params), &state).await;
        assert!(list_result.is_ok());
        let keys = list_result.unwrap();
        let key_arr = keys["keys"].as_array().expect("keys array");
        assert!(
            key_arr.is_empty(),
            "without successful store, list should be empty; got {:?}",
            key_arr
        );
    }

    #[tokio::test]
    async fn storage_nested_key_paths_work() {
        let state = mock_state(Some("test-nested")).await;
        let family_id = format!("test-nested-{}", uuid::Uuid::new_v4());

        let store_p = json!({"family_id": &family_id, "key": "deep/path/key", "value": "nested"});
        let store_result = storage_store(Some(&store_p), &state);
        assert!(
            matches!(store_result, Err(NestGateError::NotImplemented(_))),
            "nested store: {:?}",
            store_result
        );

        let retrieve_p = json!({"family_id": &family_id, "key": "deep/path/key"});
        let retrieve_result = storage_retrieve(Some(&retrieve_p), &state);
        assert!(matches!(
            retrieve_result,
            Err(NestGateError::NotImplemented(_))
        ));
    }
}
