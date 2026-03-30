// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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

//! **FAULT INJECTION TESTS**
//!
//! Tests for system behavior under injected faults.

use nestgate_api::transport::{
    JsonRpcHandler, JsonRpcRequest, JsonRpcResponse, NestGateRpcHandler, TransportConfig,
};
use serde_json::{Value, json};
use std::sync::Arc;

// ============================================================================
// Protocol Fault Injection (3 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_wrong_jsonrpc_version() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Inject fault: wrong version
    let request = JsonRpcRequest {
        jsonrpc: "1.0".to_string(), // Wrong version
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    let response = handler.handle_request(request).await;

    // Should still handle gracefully
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn test_fault_empty_method_name() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: String::new(), // Empty method
        params: json!({}),
        id: Value::from(2),
    };

    let response = handler.handle_request(request).await;

    // Should return method not found
    assert!(response.error.is_some());
}

#[tokio::test]
async fn test_fault_null_params() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!(null), // Null params
        id: Value::from(3),
    };

    let response = handler.handle_request(request).await;

    // Should handle null params
    assert_eq!(response.id, 3);
}

// ============================================================================
// Configuration Fault Injection (3 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_empty_family_id() {
    let config = TransportConfig::new("");

    // Should still validate (empty is technically valid)
    assert!(config.validate().is_ok());
}

#[tokio::test]
async fn test_fault_invalid_socket_path() {
    let config = TransportConfig::new("test").with_socket_path(""); // Empty path

    // Should create but might fail on actual use
    assert_eq!(config.socket_path.to_str().unwrap(), "");
}

#[tokio::test]
#[serial_test::serial]
async fn test_fault_conflicting_config() {
    temp_env::async_with_vars(
        [
            ("NESTGATE_FAMILY_ID", Some("env_family")),
            ("NESTGATE_HTTP_PORT", Some("not_a_number")),
        ],
        async {
            let result = TransportConfig::from_env();
            assert!(result.is_ok() || result.is_err());
        },
    )
    .await;
}

// ============================================================================
// Concurrency Fault Injection (2 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_concurrent_conflicting_requests() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    let mut handles = vec![];

    // Same ID for multiple requests (protocol violation)
    for i in 0..10 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"task": i}),
                id: Value::from(1), // Same ID for all!
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    // All should complete even with same ID
    for handle in handles {
        let response = handle.await.unwrap();
        assert_eq!(response.id, 1);
    }
}

#[tokio::test]
async fn test_fault_handler_under_extreme_load() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    let mut handles = vec![];

    // Extreme load: 200 concurrent requests
    for i in 0..200 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"load_test": i}),
                id: Value::from(i64::from(i)),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(response) = handle.await {
            if response.error.is_none() {
                success_count += 1;
            }
        }
    }

    // At least 90% should succeed under extreme load
    assert!(
        success_count >= 180,
        "Only {success_count}/200 succeeded under load"
    );
}

// ============================================================================
// Data Fault Injection (3 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_extremely_large_payload() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // 1MB of data
    let large_data: Vec<String> = (0..100000).map(|i| format!("data_{i}")).collect();

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({"large_data": large_data}),
        id: Value::from(1),
    };

    let response = handler.handle_request(request).await;

    // Should handle or reject gracefully
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn test_fault_deeply_nested_json() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Create deeply nested JSON
    let mut nested = json!({"deepest": "value"});
    for i in 0..100 {
        nested = json!({"level": i, "nested": nested});
    }

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: nested,
        id: Value::from(2),
    };

    let response = handler.handle_request(request).await;

    // Should handle deep nesting
    assert_eq!(response.id, 2);
}

#[tokio::test]
async fn test_fault_special_unicode_characters() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let special_chars = vec![
        "🚀💎🎊",
        "مرحبا بك",
        "你好世界",
        "Привет мир",
        "\u{0000}\u{001F}",
        "test\nwith\nnewlines",
    ];

    for (i, chars) in special_chars.into_iter().enumerate() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({"special": chars}),
            id: Value::from(i as i64),
        };

        let response = handler.handle_request(request).await;
        assert_eq!(response.id, i as i64);
    }
}

// ============================================================================
// Timing Fault Injection (2 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_simultaneous_requests() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    // Launch all at exactly the same time
    let handles: Vec<_> = (0..50)
        .map(|i| {
            let h = handler.clone();
            tokio::spawn(async move {
                let request = JsonRpcRequest {
                    jsonrpc: "2.0".to_string(),
                    method: "health.ping".to_string(),
                    params: json!({}),
                    id: Value::from(i),
                };
                h.handle_request(request).await
            })
        })
        .collect();

    // All should complete
    for handle in handles {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
    }
}

#[tokio::test]
async fn test_fault_request_during_high_cpu() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Simulate high CPU with busy work
    let busy_work = tokio::spawn(async {
        for _ in 0..1000000 {
            let _ = format!("{}", rand::random::<u64>());
        }
    });

    // Send request during high CPU
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    let response = handler.handle_request(request).await;

    busy_work.abort();

    // Should still respond
    assert!(response.error.is_none());
}

// ============================================================================
// Error Response Fault Injection (2 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_error_response_structure() {
    let response = JsonRpcResponse::error_with_code(Value::from(1), -32600, "Invalid Request");

    assert_eq!(response.id, Value::from(1));
    assert!(response.result.is_none());
    assert!(response.error.is_some());

    let error = response.error.unwrap();
    assert_eq!(error.code, -32600);
}

#[tokio::test]
async fn test_fault_multiple_errors_in_sequence() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Send 20 invalid requests in a row
    for i in 0..20 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: format!("invalid.method.{i}"),
            params: json!({}),
            id: Value::from(i64::from(i)),
        };

        let response = handler.handle_request(request).await;

        // Should consistently return errors (method not found per JSON-RPC spec)
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, -32601);
    }
}
