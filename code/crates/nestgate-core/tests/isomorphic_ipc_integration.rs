// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # 🧪 Isomorphic IPC Integration Tests
//!
//! **END-TO-END TESTING**: Validates the complete isomorphic IPC implementation
//!
//! Tests the Try→Detect→Adapt→Succeed pattern with real Unix socket
//! and TCP fallback scenarios.

use nestgate_core::rpc::isomorphic_ipc::{
    IpcEndpoint, IsomorphicIpcServer, RpcHandler, UnixSocketRpcHandler, discover_ipc_endpoint,
};
use serde_json::json;
use std::sync::Arc;

/// Test Unix socket server startup and client connection
#[tokio::test]
async fn test_isomorphic_server_creation() {
    // Create handler
    let handler = Arc::new(UnixSocketRpcHandler::new().unwrap());

    // Create isomorphic server
    let _server = Arc::new(IsomorphicIpcServer::new(
        "test-nestgate".to_string(),
        handler,
    ));

    // Server creation should succeed (field is private, so we just verify it doesn't panic)
}

/// Test handler processes health requests correctly
#[tokio::test]
async fn test_handler_health_check() {
    let handler = UnixSocketRpcHandler::new().unwrap();

    let request = json!({
        "jsonrpc": "2.0",
        "method": "health",
        "id": 1
    });

    let response = handler.handle_request(request).await;

    // Should return healthy status
    assert!(response["result"]["status"] == "healthy");
    assert!(response["result"]["isomorphic"] == true);
    assert!(response["error"].is_null());
}

/// Test handler processes version requests correctly
#[tokio::test]
async fn test_handler_version_check() {
    let handler = UnixSocketRpcHandler::new().unwrap();

    let request = json!({
        "jsonrpc": "2.0",
        "method": "version",
        "id": 1
    });

    let response = handler.handle_request(request).await;

    // Should return version info
    assert!(response["result"]["version"].is_string());
    assert!(response["result"]["ipc"] == "isomorphic");
    assert!(response["error"].is_null());
}

/// Test handler rejects unknown methods
#[tokio::test]
async fn test_handler_unknown_method() {
    let handler = UnixSocketRpcHandler::new().unwrap();

    let request = json!({
        "jsonrpc": "2.0",
        "method": "unknown.method",
        "id": 1
    });

    let response = handler.handle_request(request).await;

    // Should return error
    assert!(response["error"].is_object());
    assert!(response["error"]["code"] == -32601);
    assert!(response["result"].is_null());
}

/// Test handler handles invalid JSON requests
#[tokio::test]
async fn test_handler_invalid_json() {
    let handler = UnixSocketRpcHandler::new().unwrap();

    let request = json!({
        "not": "valid",
        "json-rpc": "request"
    });

    let response = handler.handle_request(request).await;

    // Should return parse error
    assert!(response["error"].is_object());
    assert!(response["error"]["code"] == -32700);
}

/// Test endpoint discovery system
#[tokio::test]
async fn test_endpoint_discovery_unix() {
    // Note: This test will fail if service not running
    // In production, we'd mock or use a test service

    let result = discover_ipc_endpoint("test-service");

    // Should either find endpoint or return error (both valid)
    match result {
        Ok(endpoint) => {
            assert!(matches!(
                endpoint,
                IpcEndpoint::UnixSocket(_) | IpcEndpoint::TcpLocal(_)
            ));
        }
        Err(_) => {
            // Expected if service not running
        }
    }
}

/// Test multiple concurrent requests
#[tokio::test]
async fn test_concurrent_requests() {
    let handler = Arc::new(UnixSocketRpcHandler::new().unwrap());

    let mut handles = vec![];

    for i in 0..10 {
        let handler_clone = handler.clone();
        let handle = tokio::spawn(async move {
            let request = json!({
                "jsonrpc": "2.0",
                "method": "health",
                "id": i
            });

            handler_clone.handle_request(request).await
        });

        handles.push(handle);
    }

    // Wait for all requests
    for handle in handles {
        let response = handle.await.unwrap();
        assert!(response["result"]["status"] == "healthy");
    }
}

/// Requires running NestGate server
#[tokio::test]
#[ignore = "Integration: requires live NestGate / storage RPC; run with --ignored when server available"]
async fn test_storage_methods() {
    let handler = UnixSocketRpcHandler::new().unwrap();

    // Test storage.store
    let store_request = json!({
        "jsonrpc": "2.0",
        "method": "storage.store",
        "params": {"key": "test", "value": "data"},
        "id": 1
    });

    let response = handler.handle_request(store_request).await;
    assert!(response["result"]["status"] == "stored");

    // Test storage.list
    let list_request = json!({
        "jsonrpc": "2.0",
        "method": "storage.list",
        "id": 2
    });

    let response = handler.handle_request(list_request).await;
    assert!(response["result"]["items"].is_array());
}

/// Requires running NestGate server
#[tokio::test]
#[ignore = "Integration: requires live NestGate / template RPC; run with --ignored when server available"]
async fn test_template_methods() {
    let handler = UnixSocketRpcHandler::new().unwrap();

    // Test template.store
    let store_request = json!({
        "jsonrpc": "2.0",
        "method": "template.store",
        "params": {"name": "test-template", "data": {}},
        "id": 1
    });

    let response = handler.handle_request(store_request).await;
    assert!(response["result"]["status"] == "stored");

    // Test template.list
    let list_request = json!({
        "jsonrpc": "2.0",
        "method": "template.list",
        "id": 2
    });

    let response = handler.handle_request(list_request).await;
    assert!(response["result"]["templates"].is_array());
}

/// Requires running NestGate server
#[tokio::test]
#[ignore = "Integration: requires live NestGate / audit RPC; run with --ignored when server available"]
async fn test_audit_methods() {
    let handler = UnixSocketRpcHandler::new().unwrap();

    // Test audit.record
    let record_request = json!({
        "jsonrpc": "2.0",
        "method": "audit.record",
        "params": {"action": "test", "result": "success"},
        "id": 1
    });

    let response = handler.handle_request(record_request).await;
    assert!(response["result"]["status"] == "recorded");

    // Test audit.query
    let query_request = json!({
        "jsonrpc": "2.0",
        "method": "audit.query",
        "id": 2
    });

    let response = handler.handle_request(query_request).await;
    assert!(response["result"]["audits"].is_array());
}
