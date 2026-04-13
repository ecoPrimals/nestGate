// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::UnixSocketRpcHandler;
use crate::rpc::isomorphic_ipc::tcp_fallback::RpcHandler;
use base64::Engine as _;
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
    assert!(list["result"]["namespace"].is_string());
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

// ── Streaming / blob tests ──────────────────────────────────────────

#[tokio::test]
async fn test_blob_store_retrieve_roundtrip() {
    let h = test_handler();
    let key = format!("blob-{}", uuid::Uuid::new_v4());
    let payload = b"binary tensor data \x00\x01\x02\xff";
    let encoded = base64::engine::general_purpose::STANDARD.encode(payload);

    let store = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.store_blob",
            "params":{"key": key, "blob": encoded}, "id":1
        }))
        .await;
    assert_eq!(store["result"]["status"], "stored");
    assert_eq!(store["result"]["size"], payload.len());

    let retrieve = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.retrieve_blob",
            "params":{"key": key}, "id":2
        }))
        .await;
    assert_eq!(retrieve["result"]["encoding"], "base64");
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(retrieve["result"]["data"].as_str().unwrap())
        .unwrap();
    assert_eq!(decoded, payload);
}

#[tokio::test]
async fn test_object_size() {
    let h = test_handler();
    let key = format!("size-{}", uuid::Uuid::new_v4());
    let payload = vec![0u8; 1024];
    let encoded = base64::engine::general_purpose::STANDARD.encode(&payload);

    h.handle_request(json!({
        "jsonrpc":"2.0","method":"storage.store_blob",
        "params":{"key": key, "blob": encoded}, "id":1
    }))
    .await;

    let size = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.object.size",
            "params":{"key": key}, "id":2
        }))
        .await;
    assert_eq!(size["result"]["exists"], true);
    assert_eq!(size["result"]["size"], 1024);
    assert_eq!(size["result"]["storage_type"], "blob");
}

#[tokio::test]
async fn test_object_size_not_found() {
    let h = test_handler();
    let size = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.object.size",
            "params":{"key": "nonexistent-key"}, "id":1
        }))
        .await;
    assert_eq!(size["result"]["exists"], false);
    assert_eq!(size["result"]["storage_type"], "none");
}

#[tokio::test]
async fn test_retrieve_range_chunked_reassembly() {
    let h = test_handler();
    let key = format!("range-{}", uuid::Uuid::new_v4());
    let payload: Vec<u8> = (0..200).map(|i| (i % 256) as u8).collect();
    let encoded = base64::engine::general_purpose::STANDARD.encode(&payload);

    h.handle_request(json!({
        "jsonrpc":"2.0","method":"storage.store_blob",
        "params":{"key": key, "blob": encoded}, "id":1
    }))
    .await;

    // Read first 100 bytes
    let chunk1 = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.retrieve_range",
            "params":{"key": key, "offset": 0, "length": 100}, "id":2
        }))
        .await;
    assert_eq!(chunk1["result"]["total_size"], 200);
    assert_eq!(chunk1["result"]["length"], 100);

    // Read second 100 bytes
    let chunk2 = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.retrieve_range",
            "params":{"key": key, "offset": 100, "length": 100}, "id":3
        }))
        .await;
    assert_eq!(chunk2["result"]["length"], 100);

    // Reassemble and verify
    let mut reassembled = base64::engine::general_purpose::STANDARD
        .decode(chunk1["result"]["data"].as_str().unwrap())
        .unwrap();
    reassembled.extend(
        base64::engine::general_purpose::STANDARD
            .decode(chunk2["result"]["data"].as_str().unwrap())
            .unwrap(),
    );
    assert_eq!(reassembled, payload);
}

#[tokio::test]
async fn test_retrieve_range_not_found() {
    let h = test_handler();
    let r = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.retrieve_range",
            "params":{"key": "ghost", "offset": 0, "length": 10}, "id":1
        }))
        .await;
    assert!(r["result"]["data"].is_null());
    assert_eq!(r["result"]["error"], "not_found");
}

// ── Namespace isolation tests ───────────────────────────────────────

#[tokio::test]
async fn test_namespace_isolation() {
    let h = test_handler();
    let key = format!("iso-{}", uuid::Uuid::new_v4());

    // Store in namespace "springA"
    h.handle_request(json!({
        "jsonrpc":"2.0","method":"storage.store",
        "params":{"key": key, "value": "alpha", "namespace": "springA"}, "id":1
    }))
    .await;

    // Not visible in namespace "springB"
    let r = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.retrieve",
            "params":{"key": key, "namespace": "springB"}, "id":2
        }))
        .await;
    assert!(r["result"]["value"].is_null());

    // Visible in "springA"
    let r = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.retrieve",
            "params":{"key": key, "namespace": "springA"}, "id":3
        }))
        .await;
    assert_eq!(r["result"]["value"], "alpha");
}

#[tokio::test]
async fn test_shared_namespace_default() {
    let h = test_handler();
    let key = format!("shared-{}", uuid::Uuid::new_v4());

    // Store without namespace (defaults to "shared")
    h.handle_request(json!({
        "jsonrpc":"2.0","method":"storage.store",
        "params":{"key": key, "value": "common"}, "id":1
    }))
    .await;

    // Retrieve with explicit "shared" namespace
    let r = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.retrieve",
            "params":{"key": key, "namespace": "shared"}, "id":2
        }))
        .await;
    assert_eq!(r["result"]["value"], "common");
    assert_eq!(r["result"]["namespace"], "shared");
}

#[tokio::test]
async fn test_namespace_list() {
    let h = test_handler();
    let key = format!("ns-{}", uuid::Uuid::new_v4());

    // Create data in two namespaces
    h.handle_request(json!({
        "jsonrpc":"2.0","method":"storage.store",
        "params":{"key": key, "value": 1, "namespace": "nsA"}, "id":1
    }))
    .await;
    h.handle_request(json!({
        "jsonrpc":"2.0","method":"storage.store",
        "params":{"key": key, "value": 2, "namespace": "nsB"}, "id":2
    }))
    .await;

    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"storage.namespaces.list","id":3}))
        .await;
    let namespaces = r["result"]["namespaces"].as_array().unwrap();
    let ns: Vec<&str> = namespaces.iter().filter_map(|v| v.as_str()).collect();
    assert!(ns.contains(&"nsA"));
    assert!(ns.contains(&"nsB"));
    assert!(ns.contains(&"shared"));
}

#[tokio::test]
async fn test_namespace_path_traversal_rejected() {
    let h = test_handler();
    let r = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.store",
            "params":{"key": "test", "value": "x", "namespace": "../etc"}, "id":1
        }))
        .await;
    assert_eq!(r["error"]["code"], -32602);
}

#[tokio::test]
async fn test_blob_exists_via_storage_exists() {
    let h = test_handler();
    let key = format!("blobex-{}", uuid::Uuid::new_v4());
    let encoded = base64::engine::general_purpose::STANDARD.encode(b"data");

    h.handle_request(json!({
        "jsonrpc":"2.0","method":"storage.store_blob",
        "params":{"key": key, "blob": encoded}, "id":1
    }))
    .await;

    let r = h
        .handle_request(json!({
            "jsonrpc":"2.0","method":"storage.exists",
            "params":{"key": key}, "id":2
        }))
        .await;
    assert_eq!(r["result"]["exists"], true);
}

#[tokio::test]
async fn test_capabilities_include_streaming_methods() {
    let h = test_handler();
    let r = h
        .handle_request(json!({"jsonrpc":"2.0","method":"capabilities.list","id":1}))
        .await;
    let methods = r["result"]["methods"].as_array().unwrap();
    let m: Vec<&str> = methods.iter().filter_map(|v| v.as_str()).collect();
    assert!(m.contains(&"storage.store_blob"));
    assert!(m.contains(&"storage.retrieve_blob"));
    assert!(m.contains(&"storage.retrieve_range"));
    assert!(m.contains(&"storage.object.size"));
    assert!(m.contains(&"storage.namespaces.list"));
}
