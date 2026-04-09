// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::UnixSocketRpcHandler;
use crate::rpc::isomorphic_ipc::tcp_fallback::RpcHandler;
use serde_json::json;

fn test_handler() -> UnixSocketRpcHandler {
    UnixSocketRpcHandler::new().expect("handler creation should succeed")
}

#[tokio::test]
async fn test_handler_creation() {
    assert!(UnixSocketRpcHandler::new().is_ok());
}

#[tokio::test]
async fn test_health_request() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"health","id":1}))
        .await;
    assert_eq!(r["result"]["status"], "healthy");
    assert_eq!(r["result"]["isomorphic"], true);
}

#[tokio::test]
async fn test_health_liveness() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"health.liveness","id":1}))
        .await;
    assert_eq!(r["result"]["alive"], true);
}

#[tokio::test]
async fn test_health_readiness() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"health.readiness","id":1}))
        .await;
    assert_eq!(r["result"]["ready"], true);
    assert!(r["result"]["storage_path"].is_string());
}

#[tokio::test]
async fn test_capabilities_list() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"capabilities.list","id":1}))
        .await;
    assert_eq!(r["result"]["primal"], "nestgate");
    let methods = r["result"]["methods"].as_array().unwrap();
    assert!(methods.iter().any(|c| c == "health.liveness"));
    assert!(methods.iter().any(|c| c == "health.readiness"));
    assert!(methods.iter().any(|c| c == "session.save"));
    assert!(methods.iter().any(|c| c == "identity.get"));
}

#[tokio::test]
async fn test_version_request() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"version","id":1}))
        .await;
    assert!(r["result"]["version"].is_string());
    assert_eq!(r["result"]["ipc"], "isomorphic");
}

#[tokio::test]
async fn test_unknown_method() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"unknown.method","id":1}))
        .await;
    assert_eq!(r["error"]["code"], -32601);
}

#[tokio::test]
async fn test_invalid_request() {
    let h = test_handler();
    let r = h.handle_request(json!({"not": "valid"})).await;
    assert_eq!(r["error"]["code"], -32700);
}

#[tokio::test]
async fn test_storage_store_retrieve_roundtrip() {
    let h = test_handler();
    let key = format!("rt-{}", uuid::Uuid::new_v4());

    let store = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.store",
            "params":{"key": key, "value":{"test":"data"}}, "id":1
        }))
        .await;
    assert_eq!(store["result"]["status"], "stored");

    let retrieve = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.retrieve",
            "params":{"key": key}, "id":2
        }))
        .await;
    assert_eq!(retrieve["result"]["value"]["test"], "data");

    let exists = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.exists",
            "params":{"key": key}, "id":3
        }))
        .await;
    assert_eq!(exists["result"]["exists"], true);

    let delete = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.delete",
            "params":{"key": key}, "id":4
        }))
        .await;
    assert_eq!(delete["result"]["status"], "deleted");

    let gone = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.exists",
            "params":{"key": key}, "id":5
        }))
        .await;
    assert_eq!(gone["result"]["exists"], false);
}

#[tokio::test]
async fn test_session_save_load_round_trip() {
    let h = test_handler();
    let sid = format!("sess-{}", uuid::Uuid::new_v4());

    let save = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"session.save",
            "params":{"session_id": sid, "data":{"level":3}}, "id":1
        }))
        .await;
    assert_eq!(save["result"]["status"], "saved");

    let load = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"session.load",
            "params":{"session_id": sid}, "id":2
        }))
        .await;
    assert_eq!(load["result"]["found"], true);
    assert_eq!(load["result"]["data"]["level"], 3);
}

#[tokio::test]
async fn test_storage_list_returns_keys() {
    let h = test_handler();
    let key = format!("list-{}", uuid::Uuid::new_v4());

    h.handle_request(json!({
        "jsonrpc":"2.0","method":"storage.store",
        "params":{"key": key, "value": 42}, "id":1
    }))
    .await;

    let list = h
        .handle_request(json!({"jsonrpc":"2.0","method":"storage.list","id":2}))
        .await;
    assert!(list["result"]["datasets"].is_array());
    let keys = list["result"]["keys"].as_array().unwrap();
    assert!(keys.iter().any(|k| k.as_str() == Some(&key)));
}

#[tokio::test]
async fn test_storage_store_missing_params() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"storage.store","id":1}))
        .await;
    assert_eq!(r["error"]["code"], -32602);
}

#[tokio::test]
async fn test_storage_retrieve_missing_key() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"storage.retrieve","params":{},"id":1}))
        .await;
    assert!(r["error"].is_object());
}

#[tokio::test]
async fn test_nat_store_retrieve_roundtrip() {
    let h = test_handler();
    let peer = format!("peer-{}", uuid::Uuid::new_v4());

    let store = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"nat.store_traversal_info",
            "params":{"peer_id": peer, "info":{"endpoint":"1.2.3.4:9000"}}, "id":1
        }))
        .await;
    assert_eq!(store["result"]["status"], "stored");

    let retrieve = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"nat.retrieve_traversal_info",
            "params":{"peer_id": peer}, "id":2
        }))
        .await;
    assert_eq!(retrieve["result"]["info"]["endpoint"], "1.2.3.4:9000");
}

#[tokio::test]
async fn test_beacon_crud() {
    let h = test_handler();
    let peer = format!("beacon-{}", uuid::Uuid::new_v4());

    let store = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"beacon.store",
            "params":{"peer_id": peer, "beacon":{"ts":12345}}, "id":1
        }))
        .await;
    assert_eq!(store["result"]["status"], "stored");

    let retrieve = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"beacon.retrieve",
            "params":{"peer_id": peer}, "id":2
        }))
        .await;
    assert_eq!(retrieve["result"]["beacon"]["ts"], 12345);

    let delete = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"beacon.delete",
            "params":{"peer_id": peer}, "id":3
        }))
        .await;
    assert_eq!(delete["result"]["status"], "deleted");

    let list = h
        .handle_request(json!({"jsonrpc":"2.0","method":"beacon.list","id":4}))
        .await;
    let ids = list["result"]["peer_ids"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|v| v.as_str())
        .collect::<Vec<_>>();
    assert!(!ids.iter().any(|id| id.contains(&peer)));
}

#[tokio::test]
async fn test_key_path_traversal_rejected() {
    let h = test_handler();
    let r = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.store",
            "params":{"key": "../etc/passwd", "value": "bad"}, "id":1
        }))
        .await;
    assert_eq!(r["error"]["code"], -32602);
}

#[tokio::test]
async fn test_discover_capabilities_legacy_method() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"discover_capabilities","id":1}))
        .await;
    assert_eq!(r["result"]["primal"], "nestgate");
    assert!(r["result"]["methods"].is_array());
}
