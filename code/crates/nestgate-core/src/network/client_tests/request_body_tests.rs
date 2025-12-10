//! Request Body Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: REQUEST BODY TESTS, REQUEST BODY ADVANCED TESTS

use super::super::client::*;

// ==================== REQUEST BODY TESTS ====================
#[test]
fn test_request_body_empty() {
    let body = RequestBody::Empty;
    match body {
        RequestBody::Empty => (),
        _ => panic!("Expected Empty body"),
    }
}

#[test]
fn test_request_body_bytes() {
    let data = b"test data";
    let body = RequestBody::Bytes(data);

    match body {
        RequestBody::Bytes(b) => assert_eq!(b, data),
        _ => panic!("Expected Bytes body"),
    }
}

#[test]
fn test_request_body_string() {
    let data = "test string";
    let body = RequestBody::String(data);

    match body {
        RequestBody::String(s) => assert_eq!(s, data),
        _ => panic!("Expected String body"),
    }
}

// ==================== REQUEST BODY ADVANCED TESTS ====================
#[test]
fn test_request_body_sizes() {
    let small_data = b"hello";
    let medium_data = vec![0u8; 1024]; // 1KB
    let large_data = vec![0u8; 1024 * 1024]; // 1MB

    let body1 = RequestBody::Bytes(small_data);
    let body2 = RequestBody::Bytes(&medium_data);
    let body3 = RequestBody::Bytes(&large_data);

    match body1 {
        RequestBody::Bytes(b) => assert_eq!(b.len(), 5),
        _ => panic!("Expected Bytes"),
    }

    match body2 {
        RequestBody::Bytes(b) => assert_eq!(b.len(), 1024),
        _ => panic!("Expected Bytes"),
    }

    match body3 {
        RequestBody::Bytes(b) => assert_eq!(b.len(), 1024 * 1024),
        _ => panic!("Expected Bytes"),
    }
}

#[test]
fn test_request_body_string_content_types() {
    let json_body = r#"{"key":"value"}"#;
    let xml_body = r#"<root><key>value</key></root>"#;
    let plain_body = "plain text content";

    let body1 = RequestBody::String(json_body);
    let body2 = RequestBody::String(xml_body);
    let body3 = RequestBody::String(plain_body);

    match body1 {
        RequestBody::String(s) => assert!(s.contains("key")),
        _ => panic!("Expected String"),
    }

    match body2 {
        RequestBody::String(s) => assert!(s.contains("<root>")),
        _ => panic!("Expected String"),
    }

    match body3 {
        RequestBody::String(s) => assert!(s.contains("plain")),
        _ => panic!("Expected String"),
    }
}
