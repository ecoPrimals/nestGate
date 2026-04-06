// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines,
        clippy::cognitive_complexity,
    )
)]
#![allow(
    deprecated,
    missing_docs,
    dead_code,
    unfulfilled_lint_expectations,
    unused_doc_comments,
    unused_imports,
    unused_variables,
    unused_comparisons,
    unused_must_use,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::uninlined_format_args,
    clippy::similar_names,
    clippy::items_after_statements,
    clippy::empty_line_after_doc_comments,
    clippy::unreadable_literal,
    clippy::manual_clamp,
    clippy::pub_underscore_fields,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::wildcard_in_or_patterns,
    clippy::type_complexity,
    clippy::field_reassign_with_default,
    clippy::module_inception,
    clippy::unnecessary_get_then_check,
    clippy::cmp_null,
    clippy::redundant_clone,
    clippy::absurd_extreme_comparisons,
    clippy::no_effect_underscore_binding,
    clippy::default_constructed_unit_structs,
    clippy::manual_string_new,
    clippy::assertions_on_constants,
    clippy::unnecessary_unwrap,
    clippy::needless_collect,
    clippy::drop_non_drop,
    clippy::zero_sized_map_values,
    clippy::match_single_binding,
    clippy::match_same_arms,
    clippy::overly_complex_bool_expr,
    clippy::needless_character_iteration,
    clippy::manual_range_contains,
    clippy::bool_assert_comparison,
    clippy::single_component_path_imports,
    clippy::used_underscore_binding
)]

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
