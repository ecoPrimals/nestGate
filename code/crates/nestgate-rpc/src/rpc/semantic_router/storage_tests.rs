use super::*;
use crate::rpc::NestGateRpcClient;
use crate::rpc::semantic_router::SemanticRouter;
use serde_json::json;
use std::sync::Arc;

fn router() -> SemanticRouter {
    let client = NestGateRpcClient::new("tarpc://127.0.0.1:65534").expect("client");
    SemanticRouter::new(Arc::new(client)).expect("router")
}

#[tokio::test]
async fn storage_put_missing_dataset_errors() {
    let r = router();
    let e = storage_put(&r, json!({"key": "k", "data": "YQ=="}))
        .await
        .expect_err("missing dataset");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_put_missing_key_errors() {
    let r = router();
    let e = storage_put(&r, json!({"dataset": "d", "data": "YQ=="}))
        .await
        .expect_err("missing key");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_put_missing_data_errors() {
    let r = router();
    let e = storage_put(&r, json!({"dataset": "d", "key": "k"}))
        .await
        .expect_err("missing data");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_put_invalid_base64_errors() {
    let r = router();
    let e = storage_put(
        &r,
        json!({"dataset": "d", "key": "k", "data": "not!!!valid-base64"}),
    )
    .await
    .expect_err("bad base64");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_get_missing_key_errors() {
    let r = router();
    let e = storage_get(&r, json!({"dataset": "d"}))
        .await
        .expect_err("missing key");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_get_missing_dataset_errors() {
    let r = router();
    let e = storage_get(&r, json!({"key": "k"}))
        .await
        .expect_err("missing dataset");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn dataset_create_missing_name_errors() {
    let r = router();
    let e = dataset_create(&r, json!({"description": "x"}))
        .await
        .expect_err("missing name");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_delete_missing_key_errors() {
    let r = router();
    let e = storage_delete(&r, json!({"dataset": "d"}))
        .await
        .expect_err("missing key");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_list_missing_dataset_errors() {
    let r = router();
    let e = storage_list(&r, json!({"prefix": "p"}))
        .await
        .expect_err("missing dataset");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_exists_missing_dataset_errors() {
    let r = router();
    let e = storage_exists(&r, json!({"key": "k"}))
        .await
        .expect_err("missing dataset");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_metadata_missing_key_errors() {
    let r = router();
    let e = storage_metadata(&r, json!({"dataset": "d"}))
        .await
        .expect_err("missing key");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn dataset_get_missing_name_errors() {
    let r = router();
    let e = dataset_get(&r, json!({})).await.expect_err("missing name");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn dataset_delete_missing_name_errors() {
    let r = router();
    let e = dataset_delete(&r, json!({}))
        .await
        .expect_err("missing name");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn store_blob_missing_key_errors() {
    let r = router();
    let e = storage_store_blob(&r, json!({"blob": "YQ=="}))
        .await
        .expect_err("missing key");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn store_blob_missing_blob_errors() {
    let r = router();
    let e = storage_store_blob(&r, json!({"key": "k"}))
        .await
        .expect_err("missing blob");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn store_blob_invalid_base64_errors() {
    let r = router();
    let e = storage_store_blob(&r, json!({"key": "k", "blob": "!!!"}))
        .await
        .expect_err("bad base64");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn retrieve_blob_missing_key_errors() {
    let r = router();
    let e = storage_retrieve_blob(&r, json!({}))
        .await
        .expect_err("missing key");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn retrieve_range_missing_key_errors() {
    let r = router();
    let e = storage_retrieve_range(&r, json!({"length": 10}))
        .await
        .expect_err("missing key");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn retrieve_range_missing_length_errors() {
    let r = router();
    let e = storage_retrieve_range(&r, json!({"key": "k"}))
        .await
        .expect_err("missing length");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn object_size_missing_key_errors() {
    let r = router();
    let e = storage_object_size(&r, json!({}))
        .await
        .expect_err("missing key");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn blob_roundtrip_via_semantic_router() {
    let r = router();
    let fid = format!("semtest-{}", uuid::Uuid::new_v4());
    let key = "test-blob.bin";
    let payload = STANDARD.encode(b"hello-semantic-blob");

    let store_result =
        storage_store_blob(&r, json!({"key": key, "blob": payload, "family_id": fid}))
            .await
            .expect("store should succeed");
    assert_eq!(store_result["status"], "stored");
    assert_eq!(store_result["size"], 19);

    let get_result = storage_retrieve_blob(&r, json!({"key": key, "family_id": fid}))
        .await
        .expect("retrieve should succeed");
    assert_eq!(get_result["size"], 19);
    let data_b64 = get_result["blob"]
        .as_str()
        .expect("retrieve_blob should return base64 string");
    let data = STANDARD
        .decode(data_b64)
        .expect("valid base64 from retrieve_blob");
    assert_eq!(data, b"hello-semantic-blob");

    let size_result = storage_object_size(&r, json!({"key": key, "family_id": fid}))
        .await
        .expect("size should succeed");
    assert_eq!(size_result["size"], 19);
    assert_eq!(size_result["exists"], true);

    let range_result = storage_retrieve_range(
        &r,
        json!({"key": key, "family_id": fid, "offset": 6, "length": 8}),
    )
    .await
    .expect("range should succeed");
    assert_eq!(range_result["length"], 8);
    let chunk = STANDARD
        .decode(
            range_result["data"]
                .as_str()
                .expect("range data should be base64"),
        )
        .expect("valid base64 in range result");
    assert_eq!(&chunk, b"semantic");

    let _ = tokio::fs::remove_dir_all(family_dir(&fid)).await;
}

#[tokio::test]
async fn namespaces_list_via_semantic_router() {
    let r = router();
    let fid = format!("semtest-ns-{}", uuid::Uuid::new_v4());
    let dir = family_dir(&fid);

    tokio::fs::create_dir_all(dir.join("shared"))
        .await
        .expect("create shared namespace dir");
    tokio::fs::create_dir_all(dir.join("private"))
        .await
        .expect("create private namespace dir");
    tokio::fs::create_dir_all(dir.join("_blobs"))
        .await
        .expect("create _blobs dir");

    let result = storage_namespaces_list(&r, json!({"family_id": fid}))
        .await
        .expect("list should succeed");
    let ns = result["namespaces"]
        .as_array()
        .expect("namespaces should be array");
    assert_eq!(ns.len(), 2);
    assert_eq!(ns[0], "private");
    assert_eq!(ns[1], "shared");
    assert_eq!(result["count"], 2);

    let _ = tokio::fs::remove_dir_all(&dir).await;
}

#[tokio::test]
async fn retrieve_blob_not_found_returns_null() {
    let r = router();
    let fid = format!("semtest-noexist-{}", uuid::Uuid::new_v4());
    let result = storage_retrieve_blob(&r, json!({"key": "nope", "family_id": fid}))
        .await
        .expect("should not error");
    assert!(result["blob"].is_null());
}

#[tokio::test]
async fn object_size_not_found_returns_false() {
    let r = router();
    let fid = format!("semtest-nosize-{}", uuid::Uuid::new_v4());
    let result = storage_object_size(&r, json!({"key": "nope", "family_id": fid}))
        .await
        .expect("should not error");
    assert_eq!(result["exists"], false);
    assert!(result["size"].is_null());
}

#[tokio::test]
async fn retrieve_range_not_found_returns_null() {
    let r = router();
    let fid = format!("semtest-norange-{}", uuid::Uuid::new_v4());
    let result = storage_retrieve_range(&r, json!({"key": "nope", "family_id": fid, "length": 10}))
        .await
        .expect("should not error");
    assert!(result["data"].is_null());
}

#[tokio::test]
async fn namespaces_list_empty_for_missing_family() {
    let r = router();
    let result = storage_namespaces_list(&r, json!({"family_id": "does-not-exist-12345"}))
        .await
        .expect("should not error");
    assert_eq!(result["count"], 0);
}

#[tokio::test]
async fn storage_store_blob_default_family_id() {
    let r = router();
    let key = format!("defam-{}", uuid::Uuid::new_v4());
    let out = storage_store_blob(&r, json!({"key": key, "blob": STANDARD.encode(b"x")}))
        .await
        .expect("store blob");
    assert_eq!(out["family_id"], "default");
    let path = blob_path("default", &key);
    let _ = tokio::fs::remove_file(&path).await;
}

#[tokio::test]
async fn storage_list_with_dataset_only_invokes_list_objects() {
    let r = router();
    let e = storage_list(&r, json!({"dataset": "ds"}))
        .await
        .expect_err("backend unavailable");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn dataset_list_invokes_list_datasets() {
    let r = router();
    let e = dataset_list(&r, json!({}))
        .await
        .expect_err("backend unavailable");
    assert!(!e.to_string().is_empty());
}

#[tokio::test]
async fn storage_put_valid_params_reaches_client() {
    let r = router();
    let e = storage_put(
        &r,
        json!({"dataset": "d", "key": "k", "data": STANDARD.encode(b"hi")}),
    )
    .await
    .expect_err("backend unavailable");
    assert!(!e.to_string().is_empty());
}
