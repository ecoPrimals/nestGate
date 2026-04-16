// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability discovery, model/nat/beacon parameter validation, templates, audit, and method aliases.

use super::super::*;

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
async fn handle_request_identity_get_includes_primal_and_license() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "identity.get".into(),
        params: None,
        id: Some(json!(7)),
    };
    let resp = handle_request(req, &state).await;
    assert!(resp.error.is_none());
    let r = resp.result.expect("result");
    assert_eq!(r.get("domain"), Some(&json!("storage")));
    assert_eq!(r.get("license"), Some(&json!("AGPL-3.0-or-later")));
}

#[tokio::test]
async fn handle_request_discovery_capability_register_missing_params() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "discovery.capability.register".into(),
        params: None,
        id: Some(json!(8)),
    };
    let resp = handle_request(req, &state).await;
    let err = resp.error.expect("error");
    assert_eq!(err.code, -32603);
}

#[tokio::test]
async fn handle_request_discovery_capability_register_missing_capability_field() {
    let state = StorageState::new().expect("storage state");
    let req = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        method: "discovery.capability.register".into(),
        params: Some(json!({"endpoint": "http://x"})),
        id: Some(json!(9)),
    };
    let resp = handle_request(req, &state).await;
    let err = resp.error.expect("error");
    assert_eq!(err.code, -32603);
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
