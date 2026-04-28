// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Health/readiness JSON-RPC paths, ecosystem legacy handler, and isomorphic keep-alive.

use super::super::*;

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
async fn legacy_ecosystem_rpc_handler_dispatches_health_check() {
    let handler = legacy_ecosystem_rpc_handler("cov-family", None).expect("handler");
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

/// LD-03: verify IsomorphicIpcServer keep-alive through the LegacyUnixJsonRpcHandler.
#[tokio::test]
#[cfg(unix)]
async fn isomorphic_keep_alive_multiple_requests_one_connection() {
    use crate::rpc::isomorphic_ipc::server::IsomorphicIpcServer;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    let family_id = format!("test-iso-keepalive-{}", uuid::Uuid::new_v4());
    let handler = legacy_ecosystem_rpc_handler(&family_id, None).expect("handler");

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
