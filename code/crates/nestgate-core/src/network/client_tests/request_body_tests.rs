// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Request Body Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: REQUEST BODY TESTS, REQUEST BODY ADVANCED TESTS

use super::super::client::*;

// ==================== REQUEST BODY TESTS ====================
#[test]
fn test_request_body_empty() {
    let data: &[u8] = &[];
    let body = RequestBody::Raw(data);
    match body {
        RequestBody::Raw(b) => assert_eq!(b.len(), 0),
        _ => panic!("Expected Raw empty body"),
    }
}

#[test]
fn test_request_body_raw() {
    let data = b"test data";
    let body = RequestBody::Raw(data);

    match body {
        RequestBody::Raw(b) => assert_eq!(b, data),
        _ => panic!("Expected Raw body"),
    }
}

#[test]
fn test_request_body_json() {
    let data = r#"{"test":"string"}"#;
    let body = RequestBody::Json(data);

    match body {
        RequestBody::Json(s) => assert_eq!(s, data),
        _ => panic!("Expected JSON body"),
    }
}

// ==================== REQUEST BODY ADVANCED TESTS ====================
#[test]
fn test_request_body_sizes() {
    let small_data = b"hello";
    let medium_data = vec![0u8; 1024]; // 1KB
    let large_data = vec![0u8; 1024 * 1024]; // 1MB

    let body1 = RequestBody::Raw(small_data);
    let body2 = RequestBody::Raw(&medium_data);
    let body3 = RequestBody::Raw(&large_data);

    match body1 {
        RequestBody::Raw(b) => assert_eq!(b.len(), 5),
        _ => panic!("Expected Raw"),
    }

    match body2 {
        RequestBody::Raw(b) => assert_eq!(b.len(), 1024),
        _ => panic!("Expected Raw"),
    }

    match body3 {
        RequestBody::Raw(b) => assert_eq!(b.len(), 1024 * 1024),
        _ => panic!("Expected Raw"),
    }
}

#[test]
fn test_request_body_string_content_types() {
    let json_body = r#"{"key":"value"}"#;
    let xml_body = r"<root><key>value</key></root>";
    let plain_body = "plain text content";

    let body1 = RequestBody::Json(json_body);
    let body2 = RequestBody::Json(xml_body);
    let body3 = RequestBody::Json(plain_body);

    match body1 {
        RequestBody::Json(s) => assert!(s.contains("key")),
        _ => panic!("Expected Json"),
    }

    match body2 {
        RequestBody::Json(s) => assert!(s.contains("<root>")),
        _ => panic!("Expected Json"),
    }

    match body3 {
        RequestBody::Json(s) => assert!(s.contains("plain")),
        _ => panic!("Expected Json"),
    }
}
