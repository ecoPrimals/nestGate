// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Request Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: REQUEST TESTS

use super::super::client::*;

// ==================== REQUEST TESTS ====================
#[test]
fn test_request_get() {
    let request = Request::get("/api/users");

    assert_eq!(request.method, Method::Get);
    assert_eq!(request.path, "/api/users");
    assert!(request.headers.is_empty());
}

#[test]
fn test_request_post_json() {
    let body = r#"{"name":"test"}"#;
    let request = Request::post_json("/api/users", body);

    assert_eq!(request.method, Method::Post);
    assert_eq!(request.path, "/api/users");
    assert!(request.headers.contains_key("content-type"));
}

#[test]
fn test_request_with_header() {
    let request = Request::get("/api/users")
        .with_header("authorization".to_string(), "Bearer token123".to_string());

    assert_eq!(
        request.headers.get("authorization"),
        Some(&"Bearer token123".to_string())
    );
}

#[test]
fn test_request_multiple_headers() {
    let request = Request::get("/api/users")
        .with_header("authorization".to_string(), "Bearer token".to_string())
        .with_header("accept".to_string(), "application/json".to_string());

    assert_eq!(request.headers.len(), 2);
}
