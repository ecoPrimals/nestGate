// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Extra dispatch, transport, and `nestgate.` method-prefix coverage for `unix_socket_server`.

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
