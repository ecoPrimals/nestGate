//! Response Parsing Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: RESPONSE PARSING TESTS

use super::super::client::*;
use serde::Deserialize;
use std::collections::HashMap;

// ==================== RESPONSE PARSING TESTS ====================
#[test]
fn test_response_text_empty() {
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: vec![],
    };

    let text = response.text().expect("Network operation failed");
    assert_eq!(text, "");
}

#[test]
fn test_response_text_with_content() {
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: b"Hello, World!".to_vec(),
    };

    let text = response.text().expect("Network operation failed");
    assert_eq!(text, "Hello, World!");
}

#[test]
fn test_response_json_array() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        id: i32,
        name: String,
    }

    let json_str = r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#;
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: json_str.as_bytes().to_vec(),
    };

    let result = response.json::<Vec<User>>();
    assert!(result.is_ok());

    let users = result.expect("Network operation failed");
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[1].name, "Bob");
}

#[test]
fn test_response_json_nested() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct ApiResponse {
        success: bool,
        data: Data,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        count: i32,
        items: Vec<String>,
    }

    let json_str = r#"{"success":true,"data":{"count":3,"items":["a","b","c"]}}"#;
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: json_str.as_bytes().to_vec(),
    };

    let result = response.json::<ApiResponse>();
    assert!(result.is_ok());

    let api_response = result.expect("Network operation failed");
    assert!(api_response.success);
    assert_eq!(api_response.data.count, 3);
    assert_eq!(api_response.data.items.len(), 3);
}
