// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Direct storage/blob handler tests and JSON-RPC dispatch for storage methods.

use super::super::*;
use super::common::cleanup_family;

#[tokio::test]
async fn test_storage_store_retrieve() {
    let mut state = StorageState::new().expect("storage state");
    let family_id = format!("test-sr-{}", uuid::Uuid::new_v4());
    state.family_id = Some(family_id.clone());

    let store_params = json!({
        "key": "test_key",
        "data": {"value": "test_data"},
        "family_id": &family_id
    });
    let result = storage_handlers::storage_store(Some(&store_params), &state)
        .await
        .unwrap();
    assert_eq!(result["status"], "stored");

    let retrieve_params = json!({
        "key": "test_key",
        "family_id": &family_id
    });
    let result = storage_handlers::storage_retrieve(Some(&retrieve_params), &state)
        .await
        .unwrap();
    assert_eq!(result["data"]["value"], "test_data");

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn test_storage_delete() {
    let mut state = StorageState::new().expect("storage state");
    let family_id = format!("test-del-{}", uuid::Uuid::new_v4());
    state.family_id = Some(family_id.clone());

    let store_params = json!({
        "key": "delete_key",
        "data": {"value": "delete_me"},
        "family_id": &family_id
    });
    storage_handlers::storage_store(Some(&store_params), &state)
        .await
        .unwrap();

    let delete_params = json!({
        "key": "delete_key",
        "family_id": &family_id
    });
    let result = storage_handlers::storage_delete(Some(&delete_params), &state)
        .await
        .unwrap();
    assert_eq!(result["status"], "deleted");

    let retrieve_params = json!({
        "key": "delete_key",
        "family_id": &family_id
    });
    let result = storage_handlers::storage_retrieve(Some(&retrieve_params), &state)
        .await
        .unwrap();
    assert!(result["value"].is_null());

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn test_storage_list() {
    let mut state = StorageState::new().expect("storage state");
    let family_id = format!("test-list-{}", uuid::Uuid::new_v4());
    state.family_id = Some(family_id.clone());

    for i in 0..5 {
        let params = json!({
            "key": format!("key_{}", i),
            "data": {"index": i},
            "family_id": &family_id
        });
        storage_handlers::storage_store(Some(&params), &state)
            .await
            .unwrap();
    }

    let list_params = json!({"family_id": &family_id});
    let result = storage_handlers::storage_list(Some(&list_params), &state)
        .await
        .unwrap();
    assert_eq!(result["keys"].as_array().unwrap().len(), 5);

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn test_storage_stats() {
    let mut state = StorageState::new().expect("storage state");
    let family_id = format!("test-stats-{}", uuid::Uuid::new_v4());
    state.family_id = Some(family_id.clone());

    let store_params = json!({
        "key": "stats_key",
        "data": {"value": "stats"},
        "family_id": &family_id
    });
    storage_handlers::storage_store(Some(&store_params), &state)
        .await
        .unwrap();

    let stats_params = json!({"family_id": &family_id});
    let result = storage_handlers::storage_stats(Some(&stats_params), &state)
        .await
        .unwrap();
    assert_eq!(result["key_count"], 1);
    assert_eq!(result["blob_count"], 0);

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn test_blob_storage() {
    let mut state = StorageState::new().expect("storage state");
    let family_id = format!("test-blob-{}", uuid::Uuid::new_v4());
    state.family_id = Some(family_id.clone());

    let test_data = b"Hello, World!";
    use base64::Engine;
    let blob_base64 = base64::engine::general_purpose::STANDARD.encode(test_data);

    let store_params = json!({
        "key": "test_blob",
        "blob": blob_base64,
        "family_id": &family_id
    });
    let result = blob_handlers::storage_store_blob(Some(&store_params), &state)
        .await
        .unwrap();
    assert_eq!(result["status"], "stored");
    assert_eq!(result["size"], test_data.len());

    let retrieve_params = json!({
        "key": "test_blob",
        "family_id": &family_id
    });
    let result = blob_handlers::storage_retrieve_blob(Some(&retrieve_params), &state)
        .await
        .unwrap();
    let retrieved_blob = base64::engine::general_purpose::STANDARD
        .decode(result["blob"].as_str().unwrap())
        .unwrap();
    assert_eq!(retrieved_blob, test_data);

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn handle_request_storage_get_put_aliases_match_retrieve_store() {
    let mut state = StorageState::new().expect("storage state");
    let family_id = format!("test-aliases-{}", uuid::Uuid::new_v4());
    state.family_id = Some(family_id.clone());

    let get_alias = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "storage.get".into(),
        params: Some(json!({"key": "k", "family_id": &family_id})),
        id: Some(json!(1)),
    };
    let retrieve = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "storage.retrieve".into(),
        params: Some(json!({"key": "k", "family_id": &family_id})),
        id: Some(json!(2)),
    };
    let rg = handle_request(get_alias, &state).await;
    let rr = handle_request(retrieve, &state).await;
    // Both should succeed with null value (key doesn't exist yet)
    assert!(rg.error.is_none());
    assert!(rr.error.is_none());
    assert_eq!(
        rg.result.as_ref().unwrap()["value"],
        rr.result.as_ref().unwrap()["value"]
    );

    let put_alias = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "storage.put".into(),
        params: Some(json!({"key": "k", "data": {}, "family_id": &family_id})),
        id: Some(json!(3)),
    };
    let store = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "storage.store".into(),
        params: Some(json!({"key": "k2", "data": {}, "family_id": &family_id})),
        id: Some(json!(4)),
    };
    let rp = handle_request(put_alias, &state).await;
    let rs = handle_request(store, &state).await;
    // Both should succeed
    assert!(rp.error.is_none());
    assert!(rs.error.is_none());

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn handle_request_storage_stats_dispatch() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "storage.stats".into(),
        params: Some(json!({"family_id": "fam-stats"})),
        id: Some(json!(77)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("family_id")),
        Some(&json!("fam-stats"))
    );
}

#[tokio::test]
async fn handle_request_storage_store_blob_succeeds() {
    let mut state = StorageState::new().expect("storage state");
    let family_id = format!("test-blob-dispatch-{}", uuid::Uuid::new_v4());
    state.family_id = Some(family_id.clone());

    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "storage.store_blob".into(),
        params: Some(json!({
            "key": "k",
            "blob": "YQ==",
            "family_id": &family_id
        })),
        id: Some(json!(1)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("status")),
        Some(&json!("stored"))
    );

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn handle_request_storage_retrieve_blob_null_for_missing_key() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "storage.retrieve_blob".into(),
        params: Some(json!({"key": "nonexistent", "family_id": "fam-blob2"})),
        id: Some(json!(2)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert!(resp.result.as_ref().unwrap()["blob"].is_null());
}

/// LD-03 composition parity: multiple sequential requests on one connection.
///
/// This is the exact pattern primalSpring needs for Nest atomic storage
/// validation: `storage.store` then `storage.retrieve` on the same socket
/// without reconnecting.
#[tokio::test]
#[cfg(unix)]
async fn keep_alive_store_then_retrieve_on_same_connection() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let family_id = format!("test-keepalive-{}", uuid::Uuid::new_v4());
    let mut state = StorageState::new().expect("storage state");
    state.family_id = Some(family_id.clone());
    let state = Arc::new(state);

    let (client, server) = UnixStream::pair().expect("unix pair");
    let h = tokio::spawn(handle_connection(server, Arc::clone(&state)));
    let (c_read, mut c_write) = client.into_split();
    let mut reader = BufReader::new(c_read);

    // Request 1: store
    let store_req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "storage.store",
        "params": { "key": "keepalive_key", "data": {"msg": "hello"}, "family_id": &family_id },
        "id": 1
    });
    c_write
        .write_all(serde_json::to_string(&store_req).unwrap().as_bytes())
        .await
        .unwrap();
    c_write.write_all(b"\n").await.unwrap();
    c_write.flush().await.unwrap();

    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();
    let resp1: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(resp1["id"], 1);
    assert!(resp1["error"].is_null(), "store failed: {resp1}");
    assert_eq!(resp1["result"]["status"], "stored");

    // Request 2: retrieve (same connection — keep-alive)
    line.clear();
    let retrieve_req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "storage.retrieve",
        "params": { "key": "keepalive_key", "family_id": &family_id },
        "id": 2
    });
    c_write
        .write_all(serde_json::to_string(&retrieve_req).unwrap().as_bytes())
        .await
        .unwrap();
    c_write.write_all(b"\n").await.unwrap();
    c_write.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let resp2: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(resp2["id"], 2);
    assert!(resp2["error"].is_null(), "retrieve failed: {resp2}");
    assert_eq!(resp2["result"]["data"]["msg"], "hello");

    // Request 3: health check (proving the loop handles N requests)
    line.clear();
    let health_req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "health.check",
        "id": 3
    });
    c_write
        .write_all(serde_json::to_string(&health_req).unwrap().as_bytes())
        .await
        .unwrap();
    c_write.write_all(b"\n").await.unwrap();
    c_write.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let resp3: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(resp3["id"], 3);
    assert_eq!(resp3["result"]["status"], "healthy");

    // Client closes — server should exit loop cleanly
    drop(c_write);
    h.await.unwrap().unwrap();

    cleanup_family(&family_id).await;
}
