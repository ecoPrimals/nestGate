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

//! **CHAOS ENGINEERING TESTS**
//!
//! Tests for system resilience under chaotic conditions.

use nestgate_api::transport::{
    JsonRpcHandler, JsonRpcRequest, NestGateRpcHandler, TransportConfig,
};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::time::{Duration, sleep};

// ============================================================================
// Network Chaos (3 tests)
// ============================================================================

#[tokio::test]
async fn test_chaos_delayed_responses() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let mut handles = vec![];

    // Send requests with random delays
    for i in 0..20 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            // Random delay between requests
            if i % 3 == 0 {
                sleep(Duration::from_millis(50)).await;
            }

            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"delay_test": i}),
                id: Value::from(i64::from(i)),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    // All should eventually succeed
    for (i, handle) in handles.into_iter().enumerate() {
        let response = handle.await.unwrap();
        assert!(response.error.is_none(), "Request {i} failed under delay");
    }
}

#[tokio::test]
async fn test_chaos_burst_traffic() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());
    let handler = Arc::new(handler);

    // Simulate burst of 100 requests at once
    let mut handles = vec![];

    for i in 0..100 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"burst_id": i}),
                id: Value::from(i64::from(i)),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    // All should handle burst gracefully
    let mut success_count = 0;
    for handle in handles {
        let response = handle.await.unwrap();
        if response.error.is_none() {
            success_count += 1;
        }
    }

    // At least 95% should succeed
    assert!(success_count >= 95, "Only {success_count}/100 succeeded");
}

#[tokio::test]
async fn test_chaos_interleaved_methods() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let methods = ["health.ping", "health.status", "identity.get"];

    for i in 0..30 {
        let method = methods[i % methods.len()];

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: json!({"test_id": i}),
            id: Value::from(i as i64),
        };

        let response = handler.handle_request(request).await;
        assert!(response.error.is_none(), "Method {method} failed");
    }
}

// ============================================================================
// Resource Chaos (3 tests)
// ============================================================================

#[tokio::test]
async fn test_chaos_memory_pressure() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Create large payloads to simulate memory pressure
    let large_data: Vec<String> = (0..10000).map(|i| format!("data_{i}")).collect();

    for i in 0..10 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({"large_payload": &large_data, "iteration": i}),
            id: Value::from(i),
        };

        let response = handler.handle_request(request).await;
        assert!(
            response.error.is_none(),
            "Failed under memory pressure at iteration {i}"
        );
    }
}

#[tokio::test]
async fn test_chaos_rapid_config_changes() {
    // Simulate rapid config creation and validation
    for i in 0..100 {
        let config = TransportConfig::new(format!("chaos_family_{i}"))
            .with_socket_path(format!("/tmp/chaos_{i}.sock"));

        assert!(config.validate().is_ok(), "Config {i} validation failed");
    }
}

#[tokio::test]
async fn test_chaos_concurrent_handler_creation() {
    let mut handles = vec![];

    // Create multiple handlers concurrently
    for i in 0..50 {
        let handle = tokio::spawn(async move {
            let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"handler_id": i}),
                id: Value::from(i64::from(i)),
            };

            handler.handle_request(request).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
    }
}

// ============================================================================
// Timing Chaos (2 tests)
// ============================================================================

#[tokio::test]
async fn test_chaos_timeout_simulation() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Send requests with varying "simulated timeout" durations
    for i in 0..20 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({"timeout_sim": i * 10}),
            id: Value::from(i64::from(i)),
        };

        // Even with "timeout" params, should still respond
        let response = handler.handle_request(request).await;
        assert!(response.error.is_none());
    }
}

#[tokio::test]
async fn test_chaos_race_conditions() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());
    let handler = Arc::new(handler);

    let mut handles = vec![];

    // Create race condition by having multiple tasks access same handler
    for i in 0..50 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            // Some tasks start immediately, some delay
            if i % 2 == 0 {
                sleep(Duration::from_micros(100)).await;
            }

            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.status".to_string(),
                params: json!({"race_id": i}),
                id: Value::from(i64::from(i)),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    // All should complete without race issues
    for handle in handles {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
    }
}

// ============================================================================
// Error Injection Chaos (3 tests)
// ============================================================================

#[tokio::test]
async fn test_chaos_mixed_valid_invalid_requests() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    for i in 0..50 {
        let method = if i % 3 == 0 {
            "invalid.method".to_string()
        } else {
            "health.ping".to_string()
        };

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method,
            params: json!({}),
            id: Value::from(i64::from(i)),
        };

        let response = handler.handle_request(request).await;

        // Should always return a response, even for invalid methods
        assert_eq!(response.id, Value::from(i64::from(i)));
    }
}

#[tokio::test]
async fn test_chaos_malformed_params() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let malformed_params = vec![
        json!(null),
        json!([]),
        json!({"random": "data"}),
        json!({"nested": {"deeply": {"data": true}}}),
    ];

    for (i, params) in malformed_params.into_iter().enumerate() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params,
            id: Value::from(i as i64),
        };

        let response = handler.handle_request(request).await;

        // Should handle gracefully, not panic
        assert_eq!(response.id, Value::from(i as i64));
    }
}

#[tokio::test]
async fn test_chaos_extreme_request_ids() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let extreme_ids = vec![i64::MIN, i64::MAX, 0, -1, 1000000000];

    for id in extreme_ids {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({}),
            id: Value::from(id),
        };

        let response = handler.handle_request(request).await;
        assert_eq!(response.id, Value::from(id));
    }
}

// ============================================================================
// Recovery Chaos (2 tests)
// ============================================================================

#[tokio::test]
async fn test_chaos_error_recovery() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Pattern: fail, succeed, fail, succeed
    for i in 0..20 {
        let method = if i % 2 == 0 {
            "invalid.method"
        } else {
            "health.ping"
        };

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: json!({}),
            id: Value::from(i64::from(i)),
        };

        let response = handler.handle_request(request).await;

        if i % 2 == 0 {
            assert!(response.error.is_some(), "Expected error at iteration {i}");
        } else {
            assert!(
                response.error.is_none(),
                "Expected success at iteration {i}"
            );
        }
    }
}

#[tokio::test]
async fn test_chaos_sustained_failure_recovery() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // 10 failures in a row
    for i in 0..10 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "invalid.method".to_string(),
            params: json!({}),
            id: Value::from(i),
        };

        let response = handler.handle_request(request).await;
        assert!(response.error.is_some());
    }

    // Then should recover for valid requests
    for i in 10..20 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({}),
            id: Value::from(i),
        };

        let response = handler.handle_request(request).await;
        assert!(
            response.error.is_none(),
            "Failed to recover at iteration {i}"
        );
    }
}
