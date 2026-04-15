// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for semantic router

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
    // Verify semantic method naming conventions
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
    let client =
        Arc::new(NestGateRpcClient::new("tarpc://127.0.0.1:65534").expect("valid endpoint"));
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
    let v = router
        .call_method("capabilities.list", json!({}))
        .await
        .expect("capabilities.list");
    let methods = v["methods"].as_array().expect("methods array");
    assert!(methods.iter().any(|m| m == "capabilities.list"));
    assert!(methods.iter().any(|m| m == "storage.put"));
    assert!(
        methods
            .iter()
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
        let err = router
            .call_method(method, json!({}))
            .await
            .expect_err(method);
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
        let err = router
            .call_method(method, json!({}))
            .await
            .expect_err(method);
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
    // `storage.exists` reports false when the object cannot be reached (no error to caller).
    let ex = router
        .call_method("storage.exists", json!({"dataset":"d","key":"k"}))
        .await
        .expect("exists");
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
    let err = router
        .call_method("storage.nonexistent", json!({}))
        .await
        .expect_err("unknown method");
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
        let l = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
        l.local_addr().expect("addr")
    };
    let service = NestGateRpcService::new().expect("service");
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

    let announce = router
        .call_method("discovery.announce", json!({}))
        .await
        .expect("discovery.announce");
    assert_eq!(announce["status"], "registered_locally");

    let list = router
        .call_method("discovery.list", json!({}))
        .await
        .expect("discovery.list");
    assert!(list["services"].as_array().is_some());

    let cap = router
        .call_method("discovery.capabilities", json!({}))
        .await
        .expect("discovery.capabilities");
    assert!(cap.get("capabilities").is_some());

    // query requires a capability param; missing should error
    router
        .call_method("discovery.query", json!({}))
        .await
        .expect_err("discovery.query missing param");
    // query with valid param returns Ok
    let query = router
        .call_method("discovery.query", json!({"capability": "storage"}))
        .await
        .expect("discovery.query");
    assert!(query["providers"].as_array().is_some());
}

#[tokio::test]
async fn metadata_methods_wire_through_backend() {
    let router = test_router();

    // Store succeeds
    let stored = router
        .call_method(
            "metadata.store",
            json!({"name": "test-svc", "capabilities": ["storage"]}),
        )
        .await
        .expect("metadata.store");
    assert_eq!(stored["status"], "stored");

    // Retrieve succeeds after store
    let retrieved = router
        .call_method("metadata.retrieve", json!({"name": "test-svc"}))
        .await
        .expect("metadata.retrieve");
    assert_eq!(retrieved["name"], "test-svc");

    // Search by capability
    let searched = router
        .call_method("metadata.search", json!({"capability": "storage"}))
        .await
        .expect("metadata.search");
    assert_eq!(searched["count"], 1);

    // Missing name errors
    router
        .call_method("metadata.retrieve", json!({}))
        .await
        .expect_err("metadata.retrieve missing name");
    router
        .call_method("metadata.search", json!({}))
        .await
        .expect_err("metadata.search missing capability");
}

#[tokio::test]
async fn health_semantic_methods_with_live_tarpc_server() {
    let (addr, server_handle) = spawn_local_tarpc_server().await;
    let endpoint = format!("tarpc://{}", addr);
    let client = Arc::new(NestGateRpcClient::new(&endpoint).expect("client"));
    let router = SemanticRouter::new(client).expect("router");

    let check = router
        .call_method("health.check", json!({}))
        .await
        .expect("health.check");
    assert_eq!(check["status"], "healthy");

    let live = router
        .call_method("health.liveness", json!({}))
        .await
        .expect("liveness");
    assert_eq!(live["alive"], true);
    assert_eq!(live["status"], "ok");

    let ready = router
        .call_method("health.readiness", json!({}))
        .await
        .expect("readiness");
    assert_eq!(ready["ready"], true);
    assert_eq!(ready["backends"]["storage"], "ready");

    let metrics = router
        .call_method("health.metrics", json!({}))
        .await
        .expect("metrics");
    assert!(metrics.get("used_space_bytes").is_some() || metrics.is_object());

    let info = router
        .call_method("health.info", json!({}))
        .await
        .expect("info");
    assert!(info.get("version").is_some());

    server_handle.abort();
}

#[tokio::test]
async fn storage_put_success_through_router_with_server() {
    let (addr, server_handle) = spawn_local_tarpc_server().await;
    let endpoint = format!("tarpc://{}", addr);
    let client = Arc::new(NestGateRpcClient::new(&endpoint).expect("client"));
    let router = SemanticRouter::new(client).expect("router");

    router
        .call_method(
            "storage.dataset.create",
            json!({"name": "ds-semantic", "description": "t"}),
        )
        .await
        .expect("create dataset");

    let put = router
        .call_method(
            "storage.put",
            json!({
                "dataset": "ds-semantic",
                "key": "k1",
                "data": "aGVsbG8="
            }),
        )
        .await
        .expect("put");
    assert!(put.get("key").is_some() || put.get("size").is_some());

    let get = router
        .call_method(
            "storage.get",
            json!({"dataset": "ds-semantic", "key": "k1"}),
        )
        .await
        .expect("get");
    assert_eq!(get["data"], "aGVsbG8=");

    server_handle.abort();
}
