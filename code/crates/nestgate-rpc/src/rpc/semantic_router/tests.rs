// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for semantic router

use super::SemanticRouter;
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
    SemanticRouter::new(client)
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
            NestGateError::NotImplemented { .. } => {}
            other => panic!("expected NotImplemented for {method}, got {other:?}"),
        }
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
async fn spawn_local_tarpc_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
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
async fn discovery_and_metadata_methods_via_router_match_direct_handlers() {
    let router = test_router();
    for method in [
        "discovery.announce",
        "discovery.query",
        "discovery.list",
        "metadata.store",
        "metadata.retrieve",
        "metadata.search",
    ] {
        let err = router
            .call_method(method, json!({}))
            .await
            .expect_err(method);
        match err {
            NestGateError::NotImplemented { .. } => {}
            other => panic!("{method}: expected NotImplemented, got {other:?}"),
        }
    }
    let cap = router
        .call_method("discovery.capabilities", json!({}))
        .await
        .expect("capabilities");
    assert!(cap.get("capabilities").is_some());
}

#[tokio::test]
async fn health_semantic_methods_with_live_tarpc_server() {
    let (addr, server_handle) = spawn_local_tarpc_server().await;
    let endpoint = format!("tarpc://{}", addr);
    let client = Arc::new(NestGateRpcClient::new(&endpoint).expect("client"));
    let router = SemanticRouter::new(client);

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

    let ready = router
        .call_method("health.readiness", json!({}))
        .await
        .expect("readiness");
    assert_eq!(ready["ready"], true);

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
    let router = SemanticRouter::new(client);

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
