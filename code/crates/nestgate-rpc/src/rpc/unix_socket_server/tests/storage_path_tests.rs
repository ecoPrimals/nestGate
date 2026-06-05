// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Direct tests for storage path builders and validators in `storage_paths.rs`.

use serde_json::json;

use crate::rpc::unix_socket_server::storage_paths::{
    blob_key_path, content_key_path, dataset_key_path, ensure_parent_dirs, extract_namespace,
    manifest_path, resolve_family_id,
};
use super::common::mock_state;

#[test]
fn content_key_path_uses_shard_prefix() {
    let path = content_key_path("myFamily", "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890");
    let path_str = path.to_string_lossy();
    assert!(path_str.contains("_content/ab/abcdef"));
    assert!(path_str.contains("datasets/myFamily"));
}

#[test]
fn content_key_path_different_prefix_for_different_hashes() {
    let a = content_key_path("f", &"a".repeat(64));
    let z = content_key_path("f", &"f".repeat(64));
    assert_ne!(a.parent(), z.parent());
}

#[test]
fn manifest_path_ends_with_json() {
    let path = manifest_path("fam", "site-v1");
    assert!(path.to_string_lossy().ends_with("site-v1.json"));
    assert!(path.to_string_lossy().contains("_manifests"));
}

#[test]
fn dataset_key_path_flat_without_namespace() {
    let path = dataset_key_path("fam", None, "my-key");
    let s = path.to_string_lossy();
    assert!(s.ends_with("datasets/fam/my-key"));
}

#[test]
fn dataset_key_path_namespaced() {
    let path = dataset_key_path("fam", Some("ns1"), "my-key");
    let s = path.to_string_lossy();
    assert!(s.contains("datasets/fam/ns1/my-key"));
}

#[test]
fn blob_key_path_flat_without_namespace() {
    let path = blob_key_path("fam", None, "blob-1");
    let s = path.to_string_lossy();
    assert!(s.contains("_blobs/blob-1"));
    assert!(!s.contains("ns"));
}

#[test]
fn blob_key_path_namespaced() {
    let path = blob_key_path("fam", Some("ns"), "blob-1");
    let s = path.to_string_lossy();
    assert!(s.contains("ns/_blobs/blob-1"));
}

#[test]
fn extract_namespace_none_when_absent() {
    let params = json!({"key": "test"});
    assert!(extract_namespace(&params).unwrap().is_none());
}

#[test]
fn extract_namespace_valid() {
    let params = json!({"namespace": "myns"});
    assert_eq!(extract_namespace(&params).unwrap(), Some("myns"));
}

#[test]
fn extract_namespace_rejects_slash() {
    let params = json!({"namespace": "a/b"});
    assert!(extract_namespace(&params).is_err());
}

#[test]
fn extract_namespace_rejects_backslash() {
    let params = json!({"namespace": "a\\b"});
    assert!(extract_namespace(&params).is_err());
}

#[test]
fn extract_namespace_rejects_dot_dot() {
    let params = json!({"namespace": "a..b"});
    assert!(extract_namespace(&params).is_err());
}

#[test]
fn extract_namespace_rejects_leading_dot() {
    let params = json!({"namespace": ".hidden"});
    assert!(extract_namespace(&params).is_err());
}

#[test]
fn extract_namespace_rejects_leading_underscore() {
    let params = json!({"namespace": "_reserved"});
    assert!(extract_namespace(&params).is_err());
}

#[test]
fn extract_namespace_rejects_empty() {
    let params = json!({"namespace": ""});
    assert!(extract_namespace(&params).is_err());
}

#[tokio::test]
async fn resolve_family_id_from_params() {
    let state = mock_state(Some("server-default")).await;
    let params = json!({"family_id": "from-params"});
    assert_eq!(resolve_family_id(&params, &state).unwrap(), "from-params");
}

#[tokio::test]
async fn resolve_family_id_falls_back_to_state() {
    let state = mock_state(Some("server-default")).await;
    let params = json!({});
    assert_eq!(
        resolve_family_id(&params, &state).unwrap(),
        "server-default"
    );
}

#[tokio::test]
async fn resolve_family_id_errors_when_both_missing() {
    let state = mock_state(None).await;
    let params = json!({});
    assert!(resolve_family_id(&params, &state).is_err());
}

#[tokio::test]
async fn ensure_parent_dirs_creates_nested() {
    let dir = std::env::temp_dir()
        .join(format!("nestgate-test-ensure-{}", uuid::Uuid::new_v4()));
    let path = dir.join("a").join("b").join("file.dat");
    ensure_parent_dirs(&path).await.unwrap();
    assert!(path.parent().unwrap().exists());
    let _ = tokio::fs::remove_dir_all(&dir).await;
}
