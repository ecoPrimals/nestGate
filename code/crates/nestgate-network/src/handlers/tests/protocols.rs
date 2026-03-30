// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::collections::HashMap;

use crate::handlers::{HttpProtocolHandler, HttpRequest, TcpProtocolHandler};

use super::{create_test_config, create_test_connection};

// ==================== HttpProtocolHandler Tests ====================

#[tokio::test]
async fn test_http_handler_get_request() {
    let config = create_test_config();
    let handler = HttpProtocolHandler::new(config);

    let request = HttpRequest {
        method: "GET".to_string(),
        path: "/test".to_string(),
        headers: HashMap::new(),
        body: Vec::new(),
    };

    let response = handler.handle_request(request);
    assert!(response.is_ok());

    let response = response.expect("GET request should succeed");
    assert_eq!(response.status_code, 200);
}

#[tokio::test]
async fn test_http_handler_post_request() {
    let config = create_test_config();
    let handler = HttpProtocolHandler::new(config);

    let request = HttpRequest {
        method: "POST".to_string(),
        path: "/test".to_string(),
        headers: HashMap::new(),
        body: Vec::new(),
    };

    let response = handler.handle_request(request);
    assert!(response.is_ok());

    let response = response.expect("POST request should succeed");
    assert_eq!(response.status_code, 201);
}

#[tokio::test]
async fn test_http_handler_put_request() {
    let config = create_test_config();
    let handler = HttpProtocolHandler::new(config);

    let request = HttpRequest {
        method: "PUT".to_string(),
        path: "/test".to_string(),
        headers: HashMap::new(),
        body: Vec::new(),
    };

    let response = handler.handle_request(request);
    assert!(response.is_ok());

    let response = response.expect("PUT request should succeed");
    assert_eq!(response.status_code, 200);
}

#[tokio::test]
async fn test_http_handler_delete_request() {
    let config = create_test_config();
    let handler = HttpProtocolHandler::new(config);

    let request = HttpRequest {
        method: "DELETE".to_string(),
        path: "/test".to_string(),
        headers: HashMap::new(),
        body: Vec::new(),
    };

    let response = handler.handle_request(request);
    assert!(response.is_ok());

    let response = response.expect("DELETE request should succeed");
    assert_eq!(response.status_code, 204);
    assert!(response.body.is_empty());
}

#[tokio::test]
async fn test_http_handler_unsupported_method() {
    let config = create_test_config();
    let handler = HttpProtocolHandler::new(config);

    let request = HttpRequest {
        method: "PATCH".to_string(),
        path: "/test".to_string(),
        headers: HashMap::new(),
        body: Vec::new(),
    };

    let response = handler.handle_request(request);
    assert!(response.is_ok());

    let response = response.expect("PATCH request should return response");
    assert_eq!(response.status_code, 405);
}

// ==================== TcpProtocolHandler Tests ====================

#[test]
fn test_tcp_handler_creation() {
    let config = create_test_config();
    let _handler = TcpProtocolHandler::new(config);
    // Just verify it constructs without panic
}

#[test]
fn test_tcp_handler_handle_connection() {
    let config = create_test_config();
    let handler = TcpProtocolHandler::new(config);

    let mut conn = create_test_connection("conn1", true);
    let result = handler.handle_connection(&mut conn);
    assert!(result.is_ok());
}

#[test]
fn test_tcp_handler_send_data() {
    let config = create_test_config();
    let handler = TcpProtocolHandler::new(config);

    let data = b"test data";
    let result = handler.send_data("conn1", data);
    assert!(result.is_ok());
    assert_eq!(result.expect("Send data should succeed"), data.len());
}

#[test]
fn test_tcp_handler_receive_data() {
    let config = create_test_config();
    let handler = TcpProtocolHandler::new(config);

    let mut buffer = vec![0u8; 1024];
    let result = handler.receive_data("conn1", &mut buffer);
    assert!(result.is_ok());
}
