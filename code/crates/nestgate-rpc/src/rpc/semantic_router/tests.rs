// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unit tests for semantic router

use super::SemanticRouter;
use crate::rpc::NestGateRpcClient;
use nestgate_types::error::NestGateError;
use serde_json::json;
use std::sync::Arc;

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
