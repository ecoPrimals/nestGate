// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Response Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: RESPONSE TESTS, RESPONSE ADVANCED TESTS

use super::super::client::*;
use serde::Deserialize;
use std::collections::HashMap;

// ==================== RESPONSE TESTS ====================
#[test]
fn test_response_is_success_200() {
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: vec![],
    };

    assert!(response.is_success());
}

#[test]
fn test_response_is_not_success_404() {
    let response = Response {
        status: StatusCode::NOT_FOUND,
        headers: HashMap::new(),
        body: vec![],
    };

    assert!(!response.is_success());
}

#[test]
fn test_response_text() {
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: b"test response".to_vec(),
    };

    let text = response.text();
    assert!(text.is_ok());
    assert_eq!(text.expect("Network operation failed"), "test response");
}

#[test]
fn test_response_json() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    let json_str = r#"{"name":"test","value":42}"#;
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: json_str.as_bytes().to_vec(),
    };

    let result = response.json::<TestData>();
    assert!(result.is_ok());

    let data = result.expect("Network operation failed");
    assert_eq!(data.name, "test");
    assert_eq!(data.value, 42);
}

// ==================== RESPONSE ADVANCED TESTS ====================
#[test]
fn test_response_with_headers() {
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "application/json".to_string());
    headers.insert("x-request-id".to_string(), "123-456".to_string());

    let response = Response {
        status: StatusCode::OK,
        headers,
        body: vec![],
    };

    assert!(response.is_success());
    assert_eq!(response.headers.len(), 2);
}

#[test]
fn test_response_large_body() {
    let large_body = vec![0u8; 1024 * 1024]; // 1MB

    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: large_body,
    };

    assert!(response.is_success());
    assert_eq!(response.body.len(), 1024 * 1024);
}
