// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for `storage_handlers` — core key/value CRUD, namespace isolation,
//! encrypt-at-rest compatibility. Extracted from inline `#[cfg(test)]` module.

use nestgate_config::config::storage_paths::get_storage_base_path;
use serde_json::json;

use super::super::storage_handlers::*;
use super::super::storage_paths::{dataset_key_path, extract_namespace, resolve_family_id};

use super::common::{cleanup_family, encrypted_state, mock_state};

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

    cleanup_family(&family_id).await;
}

/// GAP-21: family_id omitted in params — server uses its own family as default.
#[tokio::test]
async fn store_retrieve_without_family_id_uses_server_default() {
    let server_family = format!("gap21-test-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&server_family)).await;

    let store_params = Some(json!({"key": "gap21-key", "value": {"msg": "no family in params"}}));
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

    cleanup_family(&server_family).await;
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

    cleanup_family(&family_id).await;
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

    cleanup_family(&family_id).await;
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

    cleanup_family(&family_id).await;
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

    cleanup_family(&family_id).await;
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

    cleanup_family(&family_id).await;
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

    cleanup_family(&family_id).await;
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

    cleanup_family(&family_id).await;
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
