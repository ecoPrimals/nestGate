// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Round 5: HTTP JSON-RPC handler branches (`handle_jsonrpc`) and related error paths.

use axum::Json;
use nestgate_api::handlers::rpc_handlers::{JsonRpcRequest, handle_jsonrpc};
use nestgate_api::nestgate_rpc_service::NestGateJsonRpcHandler;
use std::sync::Arc;

#[tokio::test]
async fn handle_jsonrpc_rejects_non_2_0_version() {
    let req = JsonRpcRequest {
        jsonrpc: Arc::from("1.0"),
        id: "a".into(),
        method: Arc::from("list_pools"),
        params: serde_json::json!(null),
    };
    let err = handle_jsonrpc(Json(req)).await.expect_err("bad version");
    assert_eq!(err.0, axum::http::StatusCode::BAD_REQUEST);
    assert_eq!(err.1.0.error.as_ref().expect("err").code, -32600);
}

#[tokio::test]
async fn handle_jsonrpc_accepts_list_pools() {
    let req = JsonRpcRequest {
        jsonrpc: Arc::from("2.0"),
        id: "1".into(),
        method: Arc::from("list_pools"),
        params: serde_json::json!(null),
    };
    let ok = handle_jsonrpc(Json(req)).await.expect("ok");
    assert!(ok.0.result.is_some());
    assert!(ok.0.error.is_none());
}

#[tokio::test]
async fn handle_jsonrpc_unknown_method_returns_500() {
    let req = JsonRpcRequest {
        jsonrpc: Arc::from("2.0"),
        id: "2".into(),
        method: Arc::from("definitely_unknown_method_xyz"),
        params: serde_json::json!(null),
    };
    let err = handle_jsonrpc(Json(req)).await.expect_err("unknown");
    assert_eq!(err.0, axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(err.1.0.error.as_ref().expect("e").code, -32603);
}

#[tokio::test]
async fn nestgate_json_rpc_handler_list_datasets_param_error() {
    let h = NestGateJsonRpcHandler::new();
    let e = h
        .handle("list_datasets", serde_json::json!({"not": "string"}))
        .await
        .expect_err("type error");
    assert!(!e.is_empty());
}

#[tokio::test]
async fn nestgate_json_rpc_handler_serializes_list_pools() {
    let h = NestGateJsonRpcHandler::new();
    let v = h
        .handle("list_pools", serde_json::Value::Null)
        .await
        .expect("pools");
    assert!(v.is_array() || v.is_null());
}

#[tokio::test]
async fn nestgate_json_rpc_handler_health_aliases_equivalent() {
    let h = NestGateJsonRpcHandler::new();
    let a = h
        .handle("health.liveness", serde_json::Value::Null)
        .await
        .unwrap();
    let b = h
        .handle("health.check", serde_json::Value::Null)
        .await
        .unwrap();
    assert_eq!(a, b);
}
