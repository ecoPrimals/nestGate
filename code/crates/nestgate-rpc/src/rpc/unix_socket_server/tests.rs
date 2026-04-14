// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for `JsonRpcUnixServer` and `handle_request` dispatch.

use super::*;

use crate::rpc::isomorphic_ipc::RpcHandler;
use nestgate_config::config::storage_paths::get_storage_base_path;

/// Cleanup a test family's dataset directory after a test.
async fn cleanup_family(family_id: &str) {
    let _ =
        tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(family_id)).await;
}

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
async fn handle_request_health_liveness() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "health.liveness".into(),
        params: None,
        id: Some(json!(1)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("status")),
        Some(&json!("alive"))
    );
}

#[tokio::test]
async fn handle_request_health_readiness_initialized() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "health.readiness".into(),
        params: None,
        id: None,
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    let st = resp.result.as_ref().and_then(|v| v.get("status"));
    assert_eq!(st, Some(&json!("ready")));
}

#[tokio::test]
async fn handle_request_invalid_jsonrpc_version() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "1.0".into(),
        method: "health".into(),
        params: None,
        id: Some(json!("a")),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.result.is_none());
    let err = resp.error.expect("error");
    assert_eq!(err.code, -32600);
}

#[tokio::test]
async fn handle_request_method_not_found() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "no.such.method".into(),
        params: None,
        id: Some(json!(99)),
    };
    let resp = handle_request(req, &state).await;
    let err = resp.error.expect("error");
    assert_eq!(err.code, -32601);
}

#[tokio::test]
async fn handle_request_health_alias() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "health".into(),
        params: None,
        id: Some(json!(0)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("status")),
        Some(&json!("healthy"))
    );
}

#[tokio::test]
async fn handle_request_health_check_alias() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "health.check".into(),
        params: None,
        id: Some(json!("chk")),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("status")),
        Some(&json!("healthy"))
    );
}

#[tokio::test]
async fn handle_request_readiness_not_initialized() {
    let mut state = StorageState::new().expect("storage state");
    state.storage_initialized = false;
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "health.readiness".into(),
        params: None,
        id: None,
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("status")),
        Some(&json!("not_ready"))
    );
}

#[tokio::test]
async fn handle_request_capabilities_list() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "capabilities.list".into(),
        params: None,
        id: Some(json!(2)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert!(
        resp.result
            .as_ref()
            .and_then(|v| v.get("methods"))
            .is_some()
    );
}

#[tokio::test]
async fn handle_request_discover_capabilities() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "discover_capabilities".into(),
        params: None,
        id: Some(json!(3)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert!(
        resp.result
            .as_ref()
            .and_then(|v| v.get("capabilities"))
            .is_some()
    );
}

#[tokio::test]
async fn handle_request_model_register_rejects_missing_model_id() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "model.register".into(),
        params: Some(json!({})),
        id: Some(json!(4)),
    };
    let resp = handle_request(req, &state).await;
    let err = resp
        .error
        .expect("expected JSON-RPC error for missing model_id");
    assert_eq!(err.code, -32603);
}

#[tokio::test]
async fn handle_request_model_exists_locate_reject_missing_model_id() {
    let state = StorageState::new().expect("storage state");
    for method in ["model.exists", "model.locate"] {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params: Some(json!({})),
            id: Some(json!(method)),
        };
        let resp = handle_request(req, &state).await;
        let err = resp.error.expect("jsonrpc error");
        assert_eq!(err.code, -32603, "{method} should reject missing model_id");
    }
}

#[tokio::test]
async fn handle_request_nat_and_beacon_reject_missing_peer_id() {
    let state = StorageState::new().expect("storage state");
    for method in [
        "nat.store_traversal_info",
        "nat.retrieve_traversal_info",
        "beacon.store",
        "beacon.retrieve",
        "beacon.delete",
    ] {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: method.into(),
            params: Some(json!({})),
            id: Some(json!(method)),
        };
        let resp = handle_request(req, &state).await;
        let err = resp.error.expect("jsonrpc error");
        assert_eq!(err.code, -32603, "{method} should reject missing peer_id");
    }
}

#[tokio::test]
async fn handle_request_beacon_list_ok() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "beacon.list".into(),
        params: Some(json!({})),
        id: Some(json!(1)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    let arr = resp.result.as_ref().and_then(|v| v.get("peer_ids"));
    assert!(arr.is_some());
}

#[tokio::test]
async fn handle_request_templates_store_and_list_dispatch() {
    let state = StorageState::new().expect("storage state");
    let store = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "templates.store".into(),
        params: Some(json!({
            "name": "n",
            "description": "d",
            "graph_data": {},
            "user_id": "u",
            "family_id": "fam-dispatch"
        })),
        id: Some(json!(1)),
    };
    let resp = handle_request(store, &state).await;
    assert!(resp.error.is_none());
    let list = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "templates.list".into(),
        params: Some(json!({"family_id": "fam-dispatch"})),
        id: Some(json!(2)),
    };
    let resp = handle_request(list, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result
            .as_ref()
            .and_then(|v| v.get("total"))
            .and_then(|v| v.as_u64()),
        Some(1)
    );
}

#[tokio::test]
async fn handle_request_templates_community_top_dispatch() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "templates.community_top".into(),
        params: Some(json!({"limit": 3})),
        id: Some(json!(1)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result
            .as_ref()
            .and_then(|v| v.get("templates"))
            .and_then(|v| v.as_array())
            .map(|a| a.len()),
        Some(0)
    );
}

#[tokio::test]
async fn handle_request_audit_store_execution_dispatch() {
    let state = StorageState::new().expect("storage state");
    let params = json!({
        "id": "audit-1",
        "execution_id": "ex-1",
        "graph_id": "g-1",
        "user_id": "user",
        "family_id": "fam-audit",
        "started_at": "2025-06-01T12:00:00Z",
        "status": "running",
        "modifications": [],
        "outcomes": [],
        "metadata": {}
    });
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "audit.store_execution".into(),
        params: Some(params),
        id: Some(json!(42)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("success")),
        Some(&json!(true))
    );
}

#[tokio::test]
async fn handle_request_discover_capabilities_dot_alias_matches_discover_capabilities() {
    let state = StorageState::new().expect("storage state");
    let a = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "discover_capabilities".into(),
        params: None,
        id: Some(json!(1)),
    };
    let b = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "discover.capabilities".into(),
        params: None,
        id: Some(json!(2)),
    };
    let ra = handle_request(a, &state).await;
    let rb = handle_request(b, &state).await;
    assert_eq!(
        ra.error.as_ref().map(|e| (e.code, &*e.message)),
        rb.error.as_ref().map(|e| (e.code, &*e.message))
    );
    assert_eq!(ra.result, rb.result);
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
async fn handle_request_nat_beacon_alias_matches_beacon_list() {
    let state = StorageState::new().expect("storage state");
    let a = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "beacon.list".into(),
        params: Some(json!({})),
        id: Some(json!(1)),
    };
    let b = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "nat.beacon".into(),
        params: Some(json!({})),
        id: Some(json!(2)),
    };
    let ra = handle_request(a, &state).await;
    let rb = handle_request(b, &state).await;
    assert_eq!(
        ra.error.as_ref().map(|e| (e.code, &*e.message)),
        rb.error.as_ref().map(|e| (e.code, &*e.message))
    );
    assert_eq!(ra.result, rb.result);
}

#[tokio::test]
#[cfg(unix)]
async fn handle_connection_rejects_invalid_json_line() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let state = StorageState::new().expect("storage state");
    let state = Arc::new(state);
    let (client, server) = UnixStream::pair().expect("unix pair");
    let h = tokio::spawn(handle_connection(server, Arc::clone(&state)));
    let (mut c_read, mut c_write) = client.into_split();
    c_write
        .write_all(b"{not valid json}\n")
        .await
        .expect("write");
    let mut line = String::new();
    BufReader::new(&mut c_read)
        .read_line(&mut line)
        .await
        .expect("read");
    let v: serde_json::Value = serde_json::from_str(line.trim()).expect("resp json");
    assert_eq!(v["error"]["code"], -32700);
    drop(c_write);
    let _ = h.await;
}

#[tokio::test]
async fn legacy_ecosystem_rpc_handler_dispatches_health_check() {
    let handler = legacy_ecosystem_rpc_handler("cov-family").expect("handler");
    let v = handler
        .handle_request(json!({
            "jsonrpc": "2.0",
            "method": "health.check",
            "params": {},
            "id": 1
        }))
        .await;
    assert_eq!(v["result"]["status"], "healthy");
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

#[tokio::test]
async fn legacy_unix_json_rpc_handler_parse_error_returns_jsonrpc_error() {
    let mut state = StorageState::new().expect("storage state");
    state.family_id = Some("f".to_string());
    let handler = LegacyUnixJsonRpcHandler::new(Arc::new(state));
    let v = handler.handle_request(json!("not an object")).await;
    assert_eq!(v["error"]["code"], -32700);
}

#[tokio::test]
async fn legacy_unix_json_rpc_handler_unknown_method() {
    let mut state = StorageState::new().expect("storage state");
    state.family_id = Some("fam".to_string());
    let handler = LegacyUnixJsonRpcHandler::new(Arc::new(state));
    let v = handler
        .handle_request(json!({
            "jsonrpc": "2.0",
            "method": "totally.unknown.method",
            "id": 7
        }))
        .await;
    assert_eq!(v["error"]["code"], -32601);
    assert_eq!(v["error"]["data"]["method"], "totally.unknown.method");
    assert!(v["result"].is_null());
}

#[tokio::test]
async fn legacy_unix_json_rpc_handler_malformed_request_wrong_method_type() {
    let mut state = StorageState::new().expect("storage state");
    state.family_id = Some("fam".to_string());
    let handler = LegacyUnixJsonRpcHandler::new(Arc::new(state));
    let v = handler
        .handle_request(json!({
            "jsonrpc": "2.0",
            "method": 42,
            "id": 1
        }))
        .await;
    assert_eq!(v["error"]["code"], -32700);
}

#[tokio::test]
#[cfg(unix)]
async fn handle_connection_malformed_json_returns_parse_error() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let state = StorageState::new().expect("storage state");
    let state = Arc::new(state);
    let (client, server) = UnixStream::pair().expect("unix pair");
    let h = tokio::spawn(handle_connection(server, Arc::clone(&state)));
    let (mut c_read, mut c_write) = client.into_split();
    c_write
        .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":}\n")
        .await
        .expect("write");
    let mut line = String::new();
    BufReader::new(&mut c_read)
        .read_line(&mut line)
        .await
        .expect("read");
    let v: serde_json::Value = serde_json::from_str(line.trim()).expect("resp json");
    assert_eq!(v["error"]["code"], -32700);
    drop(c_write);
    let _ = h.await;
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

/// LD-03: verify IsomorphicIpcServer keep-alive through the LegacyUnixJsonRpcHandler.
#[tokio::test]
#[cfg(unix)]
async fn isomorphic_keep_alive_multiple_requests_one_connection() {
    use crate::rpc::isomorphic_ipc::server::IsomorphicIpcServer;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let family_id = format!("test-iso-keepalive-{}", uuid::Uuid::new_v4());
    let handler = legacy_ecosystem_rpc_handler(&family_id).expect("handler");

    let (client, server) = UnixStream::pair().expect("unix pair");
    let h = tokio::spawn(IsomorphicIpcServer::handle_unix_connection(server, handler));
    let (c_read, mut c_write) = client.into_split();
    let mut reader = BufReader::new(c_read);

    for id in 1..=5 {
        let req = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "health.check",
            "id": id
        });
        c_write
            .write_all(serde_json::to_string(&req).unwrap().as_bytes())
            .await
            .unwrap();
        c_write.write_all(b"\n").await.unwrap();
        c_write.flush().await.unwrap();

        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();
        let resp: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
        assert_eq!(resp["id"], id, "response {id} id mismatch");
        assert_eq!(resp["result"]["status"], "healthy");
    }

    drop(c_write);
    h.await.unwrap().unwrap();
}

/// Verify event-driven idle loop: the server-side select! fires the idle
/// timer arm and sends a `connection.closing` notification before EOF.
#[tokio::test]
async fn idle_event_sends_close_notification() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let idle_limit = std::time::Duration::from_millis(80);

    let (client, server) = tokio::net::UnixStream::pair().unwrap();
    let (srv_reader, mut srv_writer) = server.into_split();
    let mut srv_reader = BufReader::new(srv_reader);

    let server_task = tokio::spawn(async move {
        let mut line = Vec::new();
        let idle_timer = tokio::time::sleep(idle_limit);
        tokio::pin!(idle_timer);

        loop {
            line.clear();
            tokio::select! {
                result = srv_reader.read_until(b'\n', &mut line) => {
                    match result {
                        Ok(0) | Err(_) => break,
                        Ok(_) => {
                            idle_timer.as_mut().reset(
                                tokio::time::Instant::now() + idle_limit,
                            );
                            srv_writer.write_all(b"{\"ok\":true}\n").await.unwrap();
                            srv_writer.flush().await.unwrap();
                        }
                    }
                }
                () = &mut idle_timer => {
                    let notification = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "connection.closing",
                        "params": { "reason": "idle" }
                    });
                    let bytes = serde_json::to_vec(&notification).unwrap();
                    srv_writer.write_all(&bytes).await.unwrap();
                    srv_writer.write_all(b"\n").await.unwrap();
                    srv_writer.flush().await.unwrap();
                    break;
                }
            }
        }
    });

    let (c_reader, _c_writer) = client.into_split();
    let mut c_reader = BufReader::new(c_reader);

    let mut notification_line = String::new();
    c_reader.read_line(&mut notification_line).await.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(notification_line.trim()).unwrap();
    assert_eq!(parsed["method"], "connection.closing");
    assert_eq!(parsed["params"]["reason"], "idle");

    server_task.await.unwrap();
}

/// Verify that activity resets the idle timer: send a request right before
/// the idle limit expires, confirm the timer resets and fires later.
#[tokio::test]
async fn activity_resets_idle_timer() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let idle_limit = std::time::Duration::from_millis(120);

    let (client, server) = tokio::net::UnixStream::pair().unwrap();
    let (srv_reader, mut srv_writer) = server.into_split();
    let mut srv_reader = BufReader::new(srv_reader);

    let server_task = tokio::spawn(async move {
        let mut line = Vec::new();
        let mut requests_served: u64 = 0;
        let idle_timer = tokio::time::sleep(idle_limit);
        tokio::pin!(idle_timer);

        loop {
            line.clear();
            tokio::select! {
                result = srv_reader.read_until(b'\n', &mut line) => {
                    match result {
                        Ok(0) | Err(_) => break,
                        Ok(_) => {
                            requests_served += 1;
                            idle_timer.as_mut().reset(
                                tokio::time::Instant::now() + idle_limit,
                            );
                            let resp = serde_json::json!({"served": requests_served});
                            srv_writer
                                .write_all(serde_json::to_string(&resp).unwrap().as_bytes())
                                .await
                                .unwrap();
                            srv_writer.write_all(b"\n").await.unwrap();
                            srv_writer.flush().await.unwrap();
                        }
                    }
                }
                () = &mut idle_timer => {
                    let n = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "connection.closing",
                        "params": { "reason": "idle", "requests_served": requests_served }
                    });
                    let _ = srv_writer.write_all(serde_json::to_string(&n).unwrap().as_bytes()).await;
                    let _ = srv_writer.write_all(b"\n").await;
                    let _ = srv_writer.flush().await;
                    break;
                }
            }
        }
    });

    let (c_reader, mut c_writer) = client.into_split();
    let mut c_reader = BufReader::new(c_reader);

    // Send a request at ~80ms (before the 120ms idle limit)
    tokio::time::sleep(std::time::Duration::from_millis(80)).await;
    c_writer.write_all(b"ping\n").await.unwrap();
    c_writer.flush().await.unwrap();

    let mut resp = String::new();
    c_reader.read_line(&mut resp).await.unwrap();
    let parsed: serde_json::Value = serde_json::from_str(resp.trim()).unwrap();
    assert_eq!(
        parsed["served"], 1,
        "server should have processed 1 request"
    );

    // Now wait for idle close notification (~120ms after last activity)
    let mut notification = String::new();
    c_reader.read_line(&mut notification).await.unwrap();
    let closing: serde_json::Value = serde_json::from_str(notification.trim()).unwrap();
    assert_eq!(closing["method"], "connection.closing");
    assert_eq!(closing["params"]["requests_served"], 1);

    server_task.await.unwrap();
}
