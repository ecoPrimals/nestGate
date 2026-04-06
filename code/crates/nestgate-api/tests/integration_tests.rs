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

//! **INTEGRATION TESTS**
//!
//! Tests for transport + protocol + handlers integration.

use nestgate_api::transport::{
    JsonRpcHandler, JsonRpcRequest, NestGateRpcHandler, TransportConfig,
};
use serde_json::{Value, json};
use std::sync::Arc;

// ============================================================================
// Transport + Handler Integration (3 tests)
// ============================================================================

#[tokio::test]
async fn test_handler_request_response_flow() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Create request
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    // Handle request
    let response = handler.handle_request(request).await;

    // Verify response
    assert!(response.error.is_none());
    assert!(response.result.is_some());
    assert_eq!(response.id, 1);

    if let Some(result) = response.result {
        assert_eq!(result["status"], "pong");
    }
}

#[tokio::test]
async fn test_config_to_handler_integration() {
    let config = TransportConfig::new("integration_test")
        .with_socket_path("/tmp/integration_test.sock")
        .with_verbose();

    assert!(config.validate().is_ok());

    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Handler should work with any config
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "identity.get".to_string(),
        params: json!({}),
        id: Value::from(2),
    };

    let response = handler.handle_request(request).await;
    assert!(response.error.is_none());
}

#[tokio::test]
async fn test_multiple_handlers_concurrent() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    let mut handles = vec![];

    for i in 0..20 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"id": i}),
                id: Value::from(i64::from(i)),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
        assert_eq!(response.id, i as i64);
    }
}

// ============================================================================
// Config + Environment Integration (2 tests)
// ============================================================================

#[tokio::test]
async fn test_config_env_to_transport() {
    let orig_fid = std::env::var("NESTGATE_FAMILY_ID").ok();
    let orig_sock = std::env::var("NESTGATE_SOCKET_PATH").ok();
    let orig_verb = std::env::var("NESTGATE_VERBOSE").ok();
    nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", "integration");
    nestgate_core::env_process::set_var("NESTGATE_SOCKET_PATH", "/tmp/integration.sock");
    nestgate_core::env_process::set_var("NESTGATE_VERBOSE", "true");

    let config = TransportConfig::from_env().unwrap();

    match orig_fid {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_FAMILY_ID"),
    }
    match orig_sock {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_SOCKET_PATH", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_SOCKET_PATH"),
    }
    match orig_verb {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_VERBOSE", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_VERBOSE"),
    }
    assert!(!config.family_id.is_empty());
    assert!(config.validate().is_ok());
}

#[tokio::test]
async fn test_config_precedence() {
    let orig = std::env::var("NESTGATE_FAMILY_ID").ok();
    nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", "env_family");

    let manual_config = TransportConfig::new("manual_family").with_socket_path("/tmp/manual.sock");

    match orig {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_FAMILY_ID"),
    }
    assert_eq!(manual_config.family_id, "manual_family");
    assert!(
        manual_config
            .socket_path
            .to_str()
            .unwrap()
            .contains("manual")
    );
}

// ============================================================================
// Error Path Integration (2 tests)
// ============================================================================

#[tokio::test]
async fn test_invalid_method_error_flow() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "invalid.nonexistent.method".to_string(),
        params: json!({}),
        id: Value::from(100),
    };

    let response = handler.handle_request(request).await;

    assert!(response.result.is_none());
    assert!(response.error.is_some());

    let error = response.error.unwrap();
    assert_eq!(error.code, -32601); // Method not found (JSON-RPC spec)
}

#[tokio::test]
async fn test_error_recovery_and_next_request() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // First request fails
    let bad_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "bad.method".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    let error_response = handler.handle_request(bad_request).await;
    assert!(error_response.error.is_some());

    // Second request should succeed
    let good_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(2),
    };

    let success_response = handler.handle_request(good_request).await;
    assert!(success_response.error.is_none());
    assert!(success_response.result.is_some());
}

// ============================================================================
// End-to-End Scenarios (3 tests)
// ============================================================================

#[tokio::test]
async fn test_e2e_health_check_flow() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // 1. Ping
    let ping_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };
    let ping_resp = handler.handle_request(ping_req).await;
    assert_eq!(ping_resp.result.unwrap()["status"], "pong");

    // 2. Status
    let status_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.status".to_string(),
        params: json!({}),
        id: Value::from(2),
    };
    let status_resp = handler.handle_request(status_req).await;
    assert!(status_resp.result.is_some());

    // 3. Identity
    let identity_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "identity.get".to_string(),
        params: json!({}),
        id: Value::from(3),
    };
    let identity_resp = handler.handle_request(identity_req).await;
    assert!(identity_resp.result.is_some());
}

#[tokio::test]
async fn test_e2e_config_handler_lifecycle() {
    let orig = std::env::var("NESTGATE_FAMILY_ID").ok();
    nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", "e2e_test");
    let config = TransportConfig::from_env().unwrap();
    match orig {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_FAMILY_ID"),
    }

    assert!(config.validate().is_ok());

    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Send request
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    // 5. Get response
    let response = handler.handle_request(request).await;

    // 6. Verify success
    assert!(response.error.is_none());
    assert!(response.result.is_some());
}

#[tokio::test]
async fn test_e2e_stress_sequential_requests() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    for i in 0..100 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({"iteration": i}),
            id: Value::from(i64::from(i)),
        };

        let response = handler.handle_request(request).await;

        assert!(response.error.is_none(), "Request {i} failed");
        assert_eq!(response.id, i64::from(i));
    }
}

// ============================================================================
// Performance & Load Tests (2 tests)
// ============================================================================

#[tokio::test]
async fn test_concurrent_load() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    let mut handles = vec![];

    // Spawn 50 concurrent requests
    for i in 0..50 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: if i % 3 == 0 {
                    "health.ping".to_string()
                } else if i % 3 == 1 {
                    "health.status".to_string()
                } else {
                    "identity.get".to_string()
                },
                params: json!({"id": i}),
                id: Value::from(i64::from(i)),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
    }
}

#[tokio::test]
async fn test_rapid_fire_requests() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let start = std::time::Instant::now();

    for i in 0..1000 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({}),
            id: Value::from(i),
        };

        let response = handler.handle_request(request).await;
        assert!(response.error.is_none());
    }

    let duration = start.elapsed();

    // Should complete 1000 requests in reasonable time
    assert!(duration.as_secs() < 10, "Too slow: {duration:?}");
}
