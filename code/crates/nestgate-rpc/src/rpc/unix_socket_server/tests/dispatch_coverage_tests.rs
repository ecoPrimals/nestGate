// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Extra dispatch, transport, and `nestgate.` method-prefix coverage for `unix_socket_server`.

use super::super::connection::handle_connection;
use super::super::*;
use super::common::cleanup_family;

#[tokio::test]
async fn handle_request_strips_nestgate_method_prefix() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "nestgate.health.check".into(),
        params: None,
        id: Some(json!(1)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("status")),
        Some(&json!("healthy"))
    );
}

#[tokio::test]
async fn handle_request_discovery_capability_register_success() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "discovery.capability.register".into(),
        params: Some(json!({
            "capability": "test.cap",
            "endpoint": "http://127.0.0.1:9"
        })),
        id: Some(json!(55)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none(), "expected success");
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("success")),
        Some(&json!(true))
    );
}

#[tokio::test]
async fn handle_request_session_save_and_load_roundtrip() {
    let mut state = StorageState::new().expect("storage state");
    let family_id = format!("test-sess-{}", uuid::Uuid::new_v4());
    state.family_id = Some(family_id.clone());

    let save = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "session.save".into(),
        params: Some(json!({
            "session_id": "s1",
            "data": {"level": 3},
            "family_id": &family_id
        })),
        id: Some(json!(1)),
    };
    let resp = handle_request(save, &state).await;
    assert!(resp.error.is_none(), "expected save ok");
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("status")),
        Some(&json!("saved"))
    );

    let load = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "session.load".into(),
        params: Some(json!({
            "session_id": "s1",
            "family_id": &family_id
        })),
        id: Some(json!(2)),
    };
    let resp = handle_request(load, &state).await;
    assert!(resp.error.is_none(), "expected load ok");
    assert_eq!(
        resp.result.as_ref().and_then(|v| v.get("data")),
        Some(&json!({"level": 3}))
    );

    cleanup_family(&family_id).await;
}

/// Blank lines are ignored; a later JSON-RPC line is still handled.
#[tokio::test]
#[cfg(unix)]
async fn handle_connection_skips_empty_lines_before_request() {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let state = StorageState::new().expect("storage state");
    let state = Arc::new(state);
    let (client, server) = UnixStream::pair().expect("unix pair");
    let h = tokio::spawn(handle_connection(server, Arc::clone(&state)));
    let (mut c_read, mut c_write) = client.into_split();
    c_write.write_all(b"\n\n").await.expect("write blanks");
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "health.liveness",
        "id": 1
    });
    c_write
        .write_all(serde_json::to_string(&req).unwrap().as_bytes())
        .await
        .unwrap();
    c_write.write_all(b"\n").await.unwrap();
    c_write.flush().await.unwrap();

    let mut line = String::new();
    BufReader::new(&mut c_read)
        .read_line(&mut line)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
    assert_eq!(v["result"]["status"], "alive");
    drop(c_write);
    let _ = h.await;
}

#[tokio::test]
async fn handle_request_route_register_returns_gate_id() {
    let mut state = StorageState::new().expect("storage state");
    state.socket_path = Some(String::from("/tmp/nestgate-test.sock"));
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "route.register".into(),
        params: Some(json!({"gate_id": "eastGate", "ttl_seconds": 60})),
        id: Some(json!(99)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none(), "expected success: {:?}", resp.error);
    let result = resp.result.unwrap();
    assert_eq!(result["registered"], true);
    assert_eq!(result["gate_id"], "eastGate");
    assert!(result["capabilities"].is_array());
    assert!(result["federation_methods"].is_array());
    assert_eq!(result["ttl_seconds"], 60);
}

#[tokio::test]
async fn handle_request_route_register_defaults() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "route.register".into(),
        params: None,
        id: Some(json!(100)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none(), "expected success: {:?}", resp.error);
    let result = resp.result.unwrap();
    assert_eq!(result["registered"], true);
    assert_eq!(result["ttl_seconds"], 300);
}

// ── Content pipeline dispatch coverage ──────────────────────────────

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serial_test::serial;
use super::common::mock_state;

fn content_rpc(method: &str, params: serde_json::Value) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: method.into(),
        params: Some(params),
        id: Some(json!(1)),
    }
}

#[tokio::test]
#[serial]
async fn dispatch_content_put_get_roundtrip() {
    let fam = format!("test-disp-putget-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&fam)).await;
    let data = STANDARD.encode(b"dispatch routing test");

    let resp = handle_request(
        content_rpc("content.put", json!({"data": data, "family_id": fam})),
        &state,
    )
    .await;
    assert!(resp.error.is_none(), "put: {:?}", resp.error);
    let hash = resp.result.unwrap()["hash"].as_str().unwrap().to_owned();

    let resp = handle_request(
        content_rpc("content.get", json!({"hash": hash, "family_id": fam})),
        &state,
    )
    .await;
    assert!(resp.error.is_none(), "get: {:?}", resp.error);
    assert_eq!(resp.result.unwrap()["hash"], hash);

    cleanup_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn dispatch_content_exists_for_stored_blob() {
    let fam = format!("test-disp-exists-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&fam)).await;
    let data = STANDARD.encode(b"exists dispatch test");

    let resp = handle_request(
        content_rpc("content.put", json!({"data": data, "family_id": fam})),
        &state,
    )
    .await;
    let hash = resp.result.unwrap()["hash"].as_str().unwrap().to_owned();

    let resp = handle_request(
        content_rpc("content.exists", json!({"hash": hash, "family_id": fam})),
        &state,
    )
    .await;
    assert!(resp.error.is_none());
    assert_eq!(resp.result.unwrap()["exists"], true);

    cleanup_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn dispatch_content_list_returns_array() {
    let fam = format!("test-disp-list-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&fam)).await;

    let resp = handle_request(
        content_rpc("content.list", json!({"family_id": fam})),
        &state,
    )
    .await;
    assert!(resp.error.is_none());
    assert!(resp.result.unwrap()["hashes"].is_array());

    cleanup_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn dispatch_content_publish_resolve_roundtrip() {
    let fam = format!("test-disp-manifest-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&fam)).await;

    let data = STANDARD.encode(b"manifest content");
    let put = handle_request(
        content_rpc("content.put", json!({"data": data, "family_id": fam})),
        &state,
    )
    .await;
    let hash = put.result.unwrap()["hash"].as_str().unwrap().to_owned();

    let resp = handle_request(
        content_rpc(
            "content.publish",
            json!({"collection": "site-v1", "manifest": {"/index.html": hash}, "family_id": fam}),
        ),
        &state,
    )
    .await;
    assert!(resp.error.is_none(), "publish: {:?}", resp.error);

    let resp = handle_request(
        content_rpc(
            "content.resolve",
            json!({"collection": "site-v1", "path": "/index.html", "family_id": fam}),
        ),
        &state,
    )
    .await;
    assert!(resp.error.is_none(), "resolve: {:?}", resp.error);
    assert_eq!(resp.result.unwrap()["hash"], hash);

    cleanup_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn dispatch_content_collections_returns_array() {
    let fam = format!("test-disp-colls-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&fam)).await;

    let resp = handle_request(
        content_rpc("content.collections", json!({"family_id": fam})),
        &state,
    )
    .await;
    assert!(resp.error.is_none());
    assert!(resp.result.unwrap()["collections"].is_array());

    cleanup_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn dispatch_content_store_stream_begin() {
    let fam = format!("test-disp-stream-{}", uuid::Uuid::new_v4());
    let resp = handle_request(
        content_rpc(
            "content.store_stream",
            json!({"total_size": 256, "family_id": fam}),
        ),
        &StorageState::new().unwrap(),
    )
    .await;
    assert!(resp.error.is_none(), "store_stream: {:?}", resp.error);
    assert!(resp.result.unwrap()["stream_id"].is_string());

    cleanup_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn dispatch_content_replicate_pull_skips_local() {
    let fam = format!("test-disp-pull-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&fam)).await;
    let data = STANDARD.encode(b"pull skip test");

    let put = handle_request(
        content_rpc("content.put", json!({"data": data, "family_id": fam})),
        &state,
    )
    .await;
    let cid = put.result.unwrap()["hash"].as_str().unwrap().to_owned();

    let resp = handle_request(
        content_rpc(
            "content.replicate.pull",
            json!({"cids": [cid], "source": "/nonexistent.sock", "family_id": fam}),
        ),
        &state,
    )
    .await;
    assert!(resp.error.is_none(), "pull: {:?}", resp.error);
    assert_eq!(resp.result.unwrap()["skipped_count"], 1);

    cleanup_family(&fam).await;
}

#[tokio::test]
async fn dispatch_unknown_method_returns_error() {
    let state = StorageState::new().unwrap();
    let resp = handle_request(
        content_rpc("nonexistent.method.xyz", json!({})),
        &state,
    )
    .await;
    assert!(resp.error.is_some());
}

#[tokio::test]
async fn dispatch_nestgate_prefix_strips_for_content() {
    let fam = format!("test-disp-prefix-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&fam)).await;

    let resp = handle_request(
        content_rpc("nestgate.content.list", json!({"family_id": fam})),
        &state,
    )
    .await;
    assert!(resp.error.is_none(), "prefix strip: {:?}", resp.error);
    assert!(resp.result.unwrap()["hashes"].is_array());

    cleanup_family(&fam).await;
}

#[tokio::test]
async fn dispatch_content_get_missing_returns_null_data() {
    let fam = format!("test-disp-miss-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&fam)).await;
    let fake_hash = "c".repeat(64);

    let resp = handle_request(
        content_rpc("content.get", json!({"hash": fake_hash, "family_id": fam})),
        &state,
    )
    .await;
    assert!(resp.error.is_none());
    assert!(resp.result.unwrap()["data"].is_null());

    cleanup_family(&fam).await;
}
