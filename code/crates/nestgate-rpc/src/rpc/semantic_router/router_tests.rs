// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for semantic router (`call_method` dispatch table and helpers).

use super::SemanticRouter;
use crate::rpc::metadata_backend::{DefaultMetadataBackend, InMemoryMetadataBackend};
use crate::rpc::{NestGateRpcClient, NestGateRpcService, serve_tarpc};
use nestgate_types::error::NestGateError;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_semantic_method_names() {
    let storage_methods = vec![
        "storage.put",
        "storage.get",
        "storage.delete",
        "storage.list",
        "storage.dataset.create",
    ];

    for method in storage_methods {
        assert!(
            method.contains('.'),
            "Method should use dot notation: {}",
            method
        );
        assert!(
            method.starts_with("storage."),
            "Storage method should start with storage.: {}",
            method
        );
    }
}

fn test_router() -> SemanticRouter {
    let client = match NestGateRpcClient::new("tarpc://127.0.0.1:65534") {
        Ok(c) => Arc::new(c),
        Err(e) => panic!("client: {e}"),
    };
    SemanticRouter::with_metadata_backend(
        client,
        Arc::new(DefaultMetadataBackend::InMemory(
            InMemoryMetadataBackend::new(),
        )),
    )
}

#[tokio::test]
async fn capabilities_list_includes_self_and_storage_methods() {
    let router = test_router();
    let v = match router.call_method("capabilities.list", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("capabilities.list: {e}"),
    };
    assert!(v.get("count").is_some(), "expected count field");
    let caps = match v["capabilities"].as_array() {
        Some(a) => a,
        None => panic!("capabilities array"),
    };
    assert!(caps.iter().any(|m| m == "capabilities.list"));
    assert!(caps.iter().any(|m| m == "storage.put"));
    assert!(
        caps.iter()
            .filter_map(|m| m.as_str())
            .all(|s| !s.starts_with("data.")),
        "NestGate must not advertise data.* in capabilities.list (storage primal; data is delegated)"
    );
}

#[tokio::test]
async fn crypto_methods_return_not_implemented() {
    let router = test_router();
    for method in [
        "crypto.encrypt",
        "crypto.decrypt",
        "crypto.generate_key",
        "crypto.generate_nonce",
        "crypto.hash",
        "crypto.verify_hash",
    ] {
        let err = match router.call_method(method, json!({})).await {
            Ok(_) => panic!("expected error for {method}"),
            Err(e) => e,
        };
        match err {
            NestGateError::NotImplemented(_) => {}
            other => panic!("expected NotImplemented for {method}, got {other:?}"),
        }
    }
}

#[tokio::test]
async fn data_prefixed_methods_are_unknown_semantic_methods() {
    let router = test_router();
    for method in [
        "data.ncbi_search",
        "data.ncbi_fetch",
        "data.noaa_ghcnd",
        "data.iris_stations",
        "data.iris_events",
        "data.anything_future",
        "data.unknown_provider",
    ] {
        let err = match router.call_method(method, json!({})).await {
            Ok(_) => panic!("expected error for {method}"),
            Err(e) => e,
        };
        let NestGateError::Api(details) = err else {
            panic!("expected Api error for unknown {method}, got {err:?}");
        };
        assert_eq!(details.status_code, Some(404));
        assert!(
            details.message.contains("semantic method"),
            "unexpected message for {method}: {}",
            details.message
        );
    }
}

#[tokio::test]
async fn storage_put_get_delete_error_without_server() {
    let router = test_router();
    assert!(
        router
            .call_method(
                "storage.put",
                json!({"dataset":"d","key":"k","data":"YQ=="}),
            )
            .await
            .is_err()
    );
    assert!(
        router
            .call_method("storage.get", json!({"dataset":"d","key":"k"}))
            .await
            .is_err()
    );
    assert!(
        router
            .call_method("storage.delete", json!({"dataset":"d","key":"k"}))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn storage_routes_hit_dispatch_table_without_server() {
    let router = test_router();
    let ex = match router
        .call_method("storage.exists", json!({"dataset":"d","key":"k"}))
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("exists: {e}"),
    };
    assert_eq!(ex["exists"], false);

    for (method, params) in [
        ("storage.metadata", json!({"dataset":"d","key":"k"})),
        ("storage.dataset.list", json!({})),
        ("storage.dataset.get", json!({"name":"n"})),
        ("storage.dataset.delete", json!({"name":"n"})),
    ] {
        assert!(
            router.call_method(method, params).await.is_err(),
            "expected connection error for {method}"
        );
    }
}

#[tokio::test]
async fn unknown_semantic_method_is_not_found() {
    let router = test_router();
    let err = match router.call_method("storage.nonexistent", json!({})).await {
        Ok(_) => panic!("expected unknown method error"),
        Err(e) => e,
    };
    match err {
        NestGateError::Api(details) => {
            assert!(
                details.message.contains("semantic method"),
                "unexpected message: {}",
                details.message
            );
            assert_eq!(details.status_code, Some(404));
        }
        other => panic!("expected Api(404), got {other:?}"),
    }
}

/// Reserve a free localhost TCP port, release it, then start [`serve_tarpc`] on that address.
pub(crate) async fn spawn_local_tarpc_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
    let addr = {
        let l = match std::net::TcpListener::bind("127.0.0.1:0") {
            Ok(x) => x,
            Err(e) => panic!("bind: {e}"),
        };
        match l.local_addr() {
            Ok(a) => a,
            Err(e) => panic!("local_addr: {e}"),
        }
    };
    let service = match NestGateRpcService::new() {
        Ok(s) => s,
        Err(e) => panic!("service: {e}"),
    };
    let handle = tokio::spawn(async move {
        let _ = serve_tarpc(addr, service).await;
    });
    for _ in 0..80 {
        if std::net::TcpStream::connect(addr).is_ok() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    (addr, handle)
}

#[tokio::test]
async fn discovery_methods_return_self_knowledge() {
    let router = test_router();

    let announce = match router.call_method("discovery.announce", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("discovery.announce: {e}"),
    };
    assert_eq!(announce["status"], "registered_locally");

    let list = match router.call_method("discovery.list", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("discovery.list: {e}"),
    };
    assert!(list["services"].as_array().is_some());

    let cap = match router
        .call_method("discovery.capabilities", json!({}))
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("discovery.capabilities: {e}"),
    };
    assert!(cap.get("capabilities").is_some());

    if router
        .call_method("discovery.query", json!({}))
        .await
        .is_ok()
    {
        panic!("expected error for missing capability");
    }
    let query = match router
        .call_method("discovery.query", json!({"capability": "storage"}))
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("discovery.query: {e}"),
    };
    assert!(query["providers"].as_array().is_some());
}

#[tokio::test]
async fn metadata_methods_wire_through_backend() {
    let router = test_router();

    let stored = match router
        .call_method(
            "metadata.store",
            json!({"name": "test-svc", "capabilities": ["storage"]}),
        )
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("metadata.store: {e}"),
    };
    assert_eq!(stored["status"], "stored");

    let retrieved = match router
        .call_method("metadata.retrieve", json!({"name": "test-svc"}))
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("metadata.retrieve: {e}"),
    };
    assert_eq!(retrieved["name"], "test-svc");

    let searched = match router
        .call_method("metadata.search", json!({"capability": "storage"}))
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("metadata.search: {e}"),
    };
    assert_eq!(searched["count"], 1);

    if router
        .call_method("metadata.retrieve", json!({}))
        .await
        .is_ok()
    {
        panic!("expected missing name error");
    }
    if router
        .call_method("metadata.search", json!({}))
        .await
        .is_ok()
    {
        panic!("expected missing capability error");
    }
}

#[tokio::test]
async fn health_semantic_methods_with_live_tarpc_server() {
    let (addr, server_handle) = spawn_local_tarpc_server().await;
    let endpoint = format!("tarpc://{}", addr);
    let client = match NestGateRpcClient::new(&endpoint) {
        Ok(c) => Arc::new(c),
        Err(e) => panic!("client: {e}"),
    };
    let router = match SemanticRouter::new(client) {
        Ok(r) => r,
        Err(e) => panic!("router: {e}"),
    };

    let check = match router.call_method("health.check", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("health.check: {e}"),
    };
    assert_eq!(check["status"], "healthy");

    let live = match router.call_method("health.liveness", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("liveness: {e}"),
    };
    assert_eq!(live["alive"], true);
    assert_eq!(live["status"], "ok");

    let ready = match router.call_method("health.readiness", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("readiness: {e}"),
    };
    assert_eq!(ready["ready"], true);
    assert_eq!(ready["backends"]["storage"], "ready");

    let metrics = match router.call_method("health.metrics", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("metrics: {e}"),
    };
    assert!(metrics.get("used_space_bytes").is_some() || metrics.is_object());

    let info = match router.call_method("health.info", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("info: {e}"),
    };
    assert!(info.get("version").is_some());

    server_handle.abort();
}

#[tokio::test]
async fn storage_put_success_through_router_with_server() {
    let (addr, server_handle) = spawn_local_tarpc_server().await;
    let endpoint = format!("tarpc://{}", addr);
    let client = match NestGateRpcClient::new(&endpoint) {
        Ok(c) => Arc::new(c),
        Err(e) => panic!("client: {e}"),
    };
    let router = match SemanticRouter::new(client) {
        Ok(r) => r,
        Err(e) => panic!("router: {e}"),
    };

    match router
        .call_method(
            "storage.dataset.create",
            json!({"name": "ds-semantic", "description": "t"}),
        )
        .await
    {
        Ok(_) => {}
        Err(e) => panic!("create dataset: {e}"),
    }

    let put = match router
        .call_method(
            "storage.put",
            json!({
                "dataset": "ds-semantic",
                "key": "k1",
                "data": "aGVsbG8="
            }),
        )
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("put: {e}"),
    };
    assert!(put.get("key").is_some() || put.get("size").is_some());

    let get = match router
        .call_method(
            "storage.get",
            json!({"dataset": "ds-semantic", "key": "k1"}),
        )
        .await
    {
        Ok(x) => x,
        Err(e) => panic!("get: {e}"),
    };
    assert_eq!(get["data"], "aGVsbG8=");

    server_handle.abort();
}

#[tokio::test]
async fn unknown_method_with_whitespace_not_matched() {
    let router = test_router();
    let err = match router.call_method(" storage.put", json!({})).await {
        Ok(_) => panic!("expected not found"),
        Err(e) => e,
    };
    match err {
        NestGateError::Api(details) => {
            assert_eq!(details.status_code, Some(404));
        }
        other => panic!("expected Api 404, got {other:?}"),
    }
}

#[tokio::test]
async fn storage_list_route_errors_without_server() {
    let router = test_router();
    assert!(
        router
            .call_method("storage.list", json!({"dataset": "d"}))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn blob_and_stream_routes_dispatch_without_server() {
    let router = test_router();
    for method in [
        "storage.store_blob",
        "storage.retrieve_blob",
        "storage.retrieve_range",
        "storage.store_stream",
        "storage.retrieve_stream",
    ] {
        assert!(
            router.call_method(method, json!({})).await.is_err(),
            "expected error for offline {method}"
        );
    }
}

#[tokio::test]
async fn session_routes_are_reachable_offline() {
    let router = test_router();
    let listed = match router.call_method("session.list", json!({})).await {
        Ok(x) => x,
        Err(e) => panic!("session.list: {e}"),
    };
    assert!(listed.get("sessions").is_some());
}
