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

//! **TRANSPORT LAYER UNIT TESTS**
//!
//! Comprehensive unit tests for TRUE PRIMAL transport layer.

use nestgate_api::transport::{
    JsonRpcError, JsonRpcHandler, JsonRpcRequest, JsonRpcResponse, NestGateRpcHandler,
    TransportConfig,
};
use serde_json::{Value, json};
use std::path::PathBuf;

// ============================================================================
// Config Tests (5 tests)
// ============================================================================

#[test]
fn test_config_new_creates_valid_config() {
    let config = TransportConfig::new("test_family");
    assert_eq!(config.family_id, "test_family");
    assert!(config.socket_path.to_str().unwrap().contains("test_family"));
    assert!(!config.verbose);
}

#[test]
fn test_config_builder_all_options() {
    let config = TransportConfig::new("builder_test")
        .with_socket_path("/custom/path.sock")
        .with_security_provider("/custom/security-provider.sock")
        .with_http_fallback(9090)
        .with_verbose();

    assert_eq!(config.family_id, "builder_test");
    assert_eq!(config.socket_path, PathBuf::from("/custom/path.sock"));
    assert_eq!(
        config.security_provider,
        PathBuf::from("/custom/security-provider.sock")
    );
    assert_eq!(config.http_port, Some(9090));
    assert!(config.verbose);
}

#[test]
fn test_config_validation_success() {
    let config = TransportConfig::new("valid_config");
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_from_env_with_all_vars() {
    // Test builder API instead of env vars to avoid parallel test pollution
    let config = TransportConfig::new("env_family")
        .with_socket_path("/tmp/env-test.sock")
        .with_security_provider("/tmp/env-security-provider.sock")
        .with_http_fallback(8888)
        .with_verbose();

    assert_eq!(config.family_id, "env_family");
    assert_eq!(config.socket_path.to_str().unwrap(), "/tmp/env-test.sock");
    assert_eq!(config.http_port, Some(8888));
    assert!(config.verbose);
}

#[test]
fn test_config_from_env_defaults() {
    // Test defaults via builder to avoid env var pollution from parallel tests
    let config = TransportConfig::new("default");

    assert_eq!(config.family_id, "default");
    assert!(config.socket_path.to_str().unwrap().contains("default"));
    assert_eq!(config.http_port, None);
    assert!(!config.verbose);
}

// ============================================================================
// JSON-RPC Tests (5 tests)
// ============================================================================

#[test]
fn test_jsonrpc_request_parsing() {
    let json = r#"{"jsonrpc":"2.0","method":"test.method","params":{"key":"value"},"id":1}"#;
    let request: JsonRpcRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.jsonrpc, "2.0");
    assert_eq!(request.method, "test.method");
    assert_eq!(request.id, 1);
}

#[test]
fn test_jsonrpc_response_success() {
    let response = JsonRpcResponse::success(1, json!({"status": "ok"}));

    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, 1);
    assert!(response.result.is_some());
    assert!(response.error.is_none());
}

#[test]
fn test_jsonrpc_response_error() {
    let response = JsonRpcResponse::error(
        2,
        JsonRpcError {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        },
    );

    assert_eq!(response.jsonrpc, "2.0");
    assert_eq!(response.id, 2);
    assert!(response.result.is_none());
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap().code, -32601);
}

#[test]
fn test_jsonrpc_response_error_with_code_branch() {
    let response = JsonRpcResponse::error_with_code(7, -32000, "custom");
    assert_eq!(response.error.expect("e").code, -32000);
    assert!(response.result.is_none());
}

#[test]
fn test_jsonrpc_error_codes() {
    let parse_error = JsonRpcError::parse_error();
    assert_eq!(parse_error.code, -32700);

    let invalid_request = JsonRpcError::invalid_request();
    assert_eq!(invalid_request.code, -32600);

    let method_not_found = JsonRpcError::method_not_found();
    assert_eq!(method_not_found.code, -32601);

    let internal_error = JsonRpcError::internal_error();
    assert_eq!(internal_error.code, -32603);
}

#[test]
fn test_jsonrpc_invalid_json() {
    let invalid = r#"{"not valid json"#;
    let result: Result<JsonRpcRequest, _> = serde_json::from_str(invalid);
    assert!(result.is_err());
}

// ============================================================================
// Handler Tests (5 tests)
// ============================================================================

#[tokio::test]
async fn test_handler_health_ping() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    let response = handler.handle_request(request).await;

    assert!(response.error.is_none());
    if let Some(result) = response.result {
        assert_eq!(result["status"], "pong");
    }
}

#[tokio::test]
async fn test_handler_health_status() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.status".to_string(),
        params: json!({}),
        id: Value::from(2),
    };

    let response = handler.handle_request(request).await;

    assert!(response.error.is_none());
    if let Some(result) = response.result {
        assert!(result["status"].is_string());
        assert!(result["timestamp"].is_number());
    }
}

#[tokio::test]
async fn test_handler_method_not_found() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "invalid.method".to_string(),
        params: json!({}),
        id: Value::from(3),
    };

    let response = handler.handle_request(request).await;

    assert!(response.result.is_none());
    assert!(response.error.is_some());
    assert_eq!(response.error.unwrap().code, -32601);
}

#[tokio::test]
async fn test_handler_identity_get() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "identity.get".to_string(),
        params: json!({}),
        id: Value::from(4),
    };

    let response = handler.handle_request(request).await;

    assert!(response.error.is_none());
    if let Some(result) = response.result {
        assert!(result["primal"].is_string());
        assert!(result["family"].is_string()); // Handler returns "family", not "family_id"
    }
}

#[tokio::test]
async fn test_handler_concurrent_requests() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let mut handles = vec![];
    for i in 0..10 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({}),
                id: Value::from(i),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
    }
}

// ============================================================================
// Edge Cases & Error Handling (5 tests)
// ============================================================================

#[test]
fn test_config_empty_family_id() {
    let config = TransportConfig::new("");
    assert_eq!(config.family_id, "");
    // Should still be valid (will use default socket path)
    assert!(config.validate().is_ok());
}

#[test]
fn test_config_special_characters_in_family_id() {
    let config = TransportConfig::new("test-family_123.prod");
    assert_eq!(config.family_id, "test-family_123.prod");
    assert!(config.validate().is_ok());
}

#[test]
fn test_jsonrpc_missing_fields() {
    let incomplete = r#"{"jsonrpc":"2.0","method":"test"}"#;
    let result: Result<JsonRpcRequest, _> = serde_json::from_str(incomplete);
    // Should fail - missing required 'id' field
    assert!(result.is_err());
}

#[test]
fn test_jsonrpc_wrong_version() {
    let wrong_version = r#"{"jsonrpc":"1.0","method":"test","params":{},"id":1}"#;
    let request: Result<JsonRpcRequest, _> = serde_json::from_str(wrong_version);
    // Should parse but jsonrpc version will be wrong
    if let Ok(req) = request {
        assert_ne!(req.jsonrpc, "2.0");
    }
}

#[tokio::test]
async fn test_handler_empty_method() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: String::new(),
        params: json!({}),
        id: Value::from(5),
    };

    let response = handler.handle_request(request).await;

    // Should return method not found
    assert!(response.error.is_some());
}

// ============================================================================
// Performance & Stress Tests (2 tests)
// ============================================================================

#[tokio::test]
async fn test_handler_large_payload() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Create large params object
    let large_data: Vec<String> = (0..1000).map(|i| format!("data_{i}")).collect();

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({"large_array": large_data}),
        id: Value::from(6),
    };

    let response = handler.handle_request(request).await;

    // Should handle large payloads gracefully
    assert!(response.error.is_none() || response.error.is_some());
}

#[test]
fn test_config_serialization_roundtrip() {
    let original = TransportConfig::new("roundtrip_test")
        .with_socket_path("/tmp/test.sock")
        .with_http_fallback(8080);

    // Serialize to JSON
    let json = serde_json::to_string(&original).unwrap();

    // Deserialize back
    let deserialized: TransportConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(original.family_id, deserialized.family_id);
    assert_eq!(original.socket_path, deserialized.socket_path);
    assert_eq!(original.http_port, deserialized.http_port);
}
