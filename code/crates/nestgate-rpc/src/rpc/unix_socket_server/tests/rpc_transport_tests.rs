// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unix line framing, parse errors, legacy JSON-RPC adapter, and idle-timer behavior (transport-level).

use super::super::*;
use crate::rpc::isomorphic_ipc::RpcHandler;

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
async fn legacy_health_readiness_reflects_storage_initialized_flag() {
    let mut ready = StorageState::new().expect("storage state");
    ready.storage_initialized = true;
    let handler = LegacyUnixJsonRpcHandler::new(Arc::new(ready));
    let v = handler
        .handle_request(json!({
            "jsonrpc": "2.0",
            "method": "health.readiness",
            "id": 1
        }))
        .await;
    assert_eq!(v["result"]["status"], "ready");

    let mut not_ready = StorageState::new().expect("storage state");
    not_ready.storage_initialized = false;
    let handler2 = LegacyUnixJsonRpcHandler::new(Arc::new(not_ready));
    let v2 = handler2
        .handle_request(json!({
            "jsonrpc": "2.0",
            "method": "health.readiness",
            "id": 2
        }))
        .await;
    assert_eq!(v2["result"]["status"], "not_ready");
}

#[tokio::test]
async fn legacy_jsonrpc_version_must_be_2_0() {
    let state = StorageState::new().expect("storage state");
    let handler = LegacyUnixJsonRpcHandler::new(Arc::new(state));
    let v = handler
        .handle_request(json!({
            "jsonrpc": "1.0",
            "method": "health.liveness",
            "id": 9
        }))
        .await;
    assert_eq!(v["error"]["code"], -32600);
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
