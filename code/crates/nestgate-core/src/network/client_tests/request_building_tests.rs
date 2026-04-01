// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Request Building Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: REQUEST BUILDING TESTS

use super::super::client::*;

// ==================== REQUEST BUILDING TESTS ====================
#[test]
fn test_request_get_with_path() {
    let request = Request::get("/api/v1/users");
    assert_eq!(request.method, Method::Get);
    assert_eq!(request.path, "/api/v1/users");
}

#[test]
fn test_request_post_json_with_body() {
    let json_body = r#"{"email":"user@example.com","password":"secure123"}"#;
    let request = Request::post_json("/api/v1/login", json_body);

    assert_eq!(request.method, Method::Post);
    assert_eq!(request.path, "/api/v1/login");
    assert!(request.headers.contains_key("content-type"));

    match &request.body {
        Some(RequestBody::Json(s)) => assert!(s.contains("email")),
        _ => panic!("Expected JSON body"),
    }
}

#[test]
fn test_request_with_multiple_headers() {
    let request = Request::get("/api/data")
        .with_header("authorization".to_string(), "Bearer xyz".to_string())
        .with_header("accept".to_string(), "application/json".to_string())
        .with_header("user-agent".to_string(), "NestGate/2.0".to_string());

    assert_eq!(request.headers.len(), 3);
    assert!(request.headers.contains_key("authorization"));
    assert!(request.headers.contains_key("accept"));
    assert!(request.headers.contains_key("user-agent"));
}

#[test]
fn test_request_header_overwrite() {
    let request = Request::get("/api/test")
        .with_header("x-custom".to_string(), "value1".to_string())
        .with_header("x-custom".to_string(), "value2".to_string());

    assert_eq!(request.headers.get("x-custom"), Some(&"value2".to_string()));
}
