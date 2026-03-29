//! # biomeOS Integration Tests
//!
//! Integration tests for biomeOS client compatibility.
//! Tests the JSON-RPC Unix socket server with patterns matching
//! the biomeOS `NestGateClient` expectations.
//!
//! **Note**: Uses the legacy `JsonRpcUnixServer` API pending migration to
//! Songbird IPC service-based patterns.
//!
//! ## Test Coverage
//! - Unix socket server lifecycle
//! - All 7 storage.* methods

#![allow(deprecated, dead_code, unused_imports)]

use nestgate_core::rpc::unix_socket_server::JsonRpcUnixServer;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

mod common;
use common::sync_utils::wait_for_condition;

/// Helper: Wait for socket to be ready with retries
async fn wait_for_socket_ready(socket_path: &Path) {
    wait_for_condition(|| socket_path.exists(), std::time::Duration::from_secs(5))
        .await
        .expect("Server socket should be created");
}

/// Test helper: Send JSON-RPC request and get response
async fn send_jsonrpc_request(
    socket_path: &Path,
    method: &str,
    params: Value,
) -> Result<Value, String> {
    let stream = UnixStream::connect(socket_path)
        .await
        .map_err(|e| format!("Connect error: {}", e))?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1
    });

    let request_json =
        serde_json::to_string(&request).map_err(|e| format!("Serialize error: {}", e))?;
    writer
        .write_all(request_json.as_bytes())
        .await
        .map_err(|e| format!("Write error: {}", e))?;
    writer
        .write_all(b"\n")
        .await
        .map_err(|e| format!("Write newline error: {}", e))?;

    let mut response_line = String::new();
    reader
        .read_line(&mut response_line)
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    let response: Value =
        serde_json::from_str(&response_line).map_err(|e| format!("Parse error: {}", e))?;
    Ok(response)
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_store_retrieve() {
    let family_id = format!("test_biomeos_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    // Start server in background
    tokio::spawn(async move {
        server.serve().await.ok();
    });

    // Wait for server to start
    wait_for_socket_ready(&socket_path).await;

    // Store data (biomeOS pattern)
    let store_params = json!({
        "key": "user:123",
        "data": {"name": "Alice", "email": "alice@example.com"},
        "family_id": family_id
    });

    let response = send_jsonrpc_request(&socket_path, "storage.store", store_params)
        .await
        .unwrap();

    assert_eq!(response["result"]["success"], true);

    // Retrieve data (biomeOS pattern)
    let retrieve_params = json!({
        "key": "user:123",
        "family_id": family_id
    });

    let response = send_jsonrpc_request(&socket_path, "storage.retrieve", retrieve_params)
        .await
        .unwrap();

    assert_eq!(response["result"]["data"]["name"], "Alice");
    assert_eq!(response["result"]["data"]["email"], "alice@example.com");

    // Cleanup
    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_list_keys() {
    let family_id = format!("test_list_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Store multiple keys with prefix
    for i in 0..5 {
        let params = json!({
            "key": format!("config:{}", i),
            "data": {"index": i},
            "family_id": family_id
        });
        send_jsonrpc_request(&socket_path, "storage.store", params)
            .await
            .unwrap();
    }

    // List all keys
    let list_params = json!({"family_id": family_id});
    let response = send_jsonrpc_request(&socket_path, "storage.list", list_params)
        .await
        .unwrap();

    let keys = response["result"]["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 5);

    // List with prefix
    let list_params = json!({
        "family_id": family_id,
        "prefix": "config:"
    });
    let response = send_jsonrpc_request(&socket_path, "storage.list", list_params)
        .await
        .unwrap();

    let keys = response["result"]["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 5);

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_stats() {
    let family_id = format!("test_stats_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Store some data
    let params = json!({
        "key": "metric:cpu",
        "data": {"value": 42},
        "family_id": family_id
    });
    send_jsonrpc_request(&socket_path, "storage.store", params)
        .await
        .unwrap();

    // Get stats
    let stats_params = json!({"family_id": family_id});
    let response = send_jsonrpc_request(&socket_path, "storage.stats", stats_params)
        .await
        .unwrap();

    assert_eq!(response["result"]["key_count"], 1);
    assert_eq!(response["result"]["blob_count"], 0);
    assert_eq!(response["result"]["family_id"], family_id);

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_blob_storage() {
    let family_id = format!("test_blob_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Store blob (biomeOS pattern)
    let test_data = b"Binary data from biomeOS client";
    use base64::Engine;
    let blob_base64 = base64::engine::general_purpose::STANDARD.encode(test_data);

    let store_params = json!({
        "key": "file:document.pdf",
        "blob": blob_base64,
        "family_id": family_id
    });

    let response = send_jsonrpc_request(&socket_path, "storage.store_blob", store_params)
        .await
        .unwrap();

    assert_eq!(response["result"]["success"], true);
    assert_eq!(response["result"]["size"], test_data.len());

    // Retrieve blob (biomeOS pattern)
    let retrieve_params = json!({
        "key": "file:document.pdf",
        "family_id": family_id
    });

    let response = send_jsonrpc_request(&socket_path, "storage.retrieve_blob", retrieve_params)
        .await
        .unwrap();

    let retrieved_blob = base64::engine::general_purpose::STANDARD
        .decode(response["result"]["blob"].as_str().unwrap())
        .unwrap();
    assert_eq!(retrieved_blob, test_data);

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_delete() {
    let family_id = format!("test_delete_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Store data
    let store_params = json!({
        "key": "temp:cache",
        "data": {"value": "temporary"},
        "family_id": family_id
    });
    send_jsonrpc_request(&socket_path, "storage.store", store_params)
        .await
        .unwrap();

    // Delete (biomeOS pattern)
    let delete_params = json!({
        "key": "temp:cache",
        "family_id": family_id
    });
    let response = send_jsonrpc_request(&socket_path, "storage.delete", delete_params)
        .await
        .unwrap();

    assert_eq!(response["result"]["success"], true);

    // Verify deleted
    let retrieve_params = json!({
        "key": "temp:cache",
        "family_id": family_id
    });
    let response = send_jsonrpc_request(&socket_path, "storage.retrieve", retrieve_params)
        .await
        .unwrap();

    assert!(response["error"].is_object());

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_family_isolation() {
    let family_id_1 = format!("test_family_1_{}", uuid::Uuid::new_v4());
    let family_id_2 = format!("test_family_2_{}", uuid::Uuid::new_v4());

    let server = JsonRpcUnixServer::new(&family_id_1).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Store data for family 1
    let store_params_1 = json!({
        "key": "shared_key",
        "data": {"owner": "family1"},
        "family_id": family_id_1
    });
    send_jsonrpc_request(&socket_path, "storage.store", store_params_1)
        .await
        .unwrap();

    // Store data for family 2 (same key, different family)
    let store_params_2 = json!({
        "key": "shared_key",
        "data": {"owner": "family2"},
        "family_id": family_id_2
    });
    send_jsonrpc_request(&socket_path, "storage.store", store_params_2)
        .await
        .unwrap();

    // Retrieve for family 1
    let retrieve_params_1 = json!({
        "key": "shared_key",
        "family_id": family_id_1
    });
    let response = send_jsonrpc_request(&socket_path, "storage.retrieve", retrieve_params_1)
        .await
        .unwrap();
    assert_eq!(response["result"]["data"]["owner"], "family1");

    // Retrieve for family 2
    let retrieve_params_2 = json!({
        "key": "shared_key",
        "family_id": family_id_2
    });
    let response = send_jsonrpc_request(&socket_path, "storage.retrieve", retrieve_params_2)
        .await
        .unwrap();
    assert_eq!(response["result"]["data"]["owner"], "family2");

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_concurrent_operations() {
    let family_id = format!("test_concurrent_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Spawn multiple concurrent operations (biomeOS pattern)
    let mut handles = vec![];
    for i in 0..10 {
        let socket_path = socket_path.clone();
        let family_id = family_id.clone();
        let handle = tokio::spawn(async move {
            let params = json!({
                "key": format!("concurrent:{}", i),
                "data": {"index": i},
                "family_id": family_id
            });
            send_jsonrpc_request(&socket_path, "storage.store", params).await
        });
        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    // Verify all stored
    let list_params = json!({"family_id": family_id});
    let response = send_jsonrpc_request(&socket_path, "storage.list", list_params)
        .await
        .unwrap();

    let keys = response["result"]["keys"].as_array().unwrap();
    assert_eq!(keys.len(), 10);

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_error_handling() {
    let family_id = format!("test_errors_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Test 1: Retrieve non-existent key
    let retrieve_params = json!({
        "key": "nonexistent",
        "family_id": family_id
    });
    let response = send_jsonrpc_request(&socket_path, "storage.retrieve", retrieve_params)
        .await
        .unwrap();
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32603);

    // Test 2: Missing required parameter
    let invalid_params = json!({"key": "test"});
    let response = send_jsonrpc_request(&socket_path, "storage.store", invalid_params)
        .await
        .unwrap();
    assert!(response["error"].is_object());

    // Test 3: Invalid method
    let params = json!({"family_id": family_id});
    let response = send_jsonrpc_request(&socket_path, "invalid.method", params)
        .await
        .unwrap();
    assert!(response["error"].is_object());
    assert_eq!(response["error"]["code"], -32601);

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_json_rpc_compliance() {
    let family_id = format!("test_jsonrpc_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Test JSON-RPC 2.0 compliance
    let params = json!({
        "key": "test",
        "data": {"value": "test"},
        "family_id": family_id
    });
    let response = send_jsonrpc_request(&socket_path, "storage.store", params)
        .await
        .unwrap();

    // Check JSON-RPC 2.0 structure
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"].is_object() || response["error"].is_object());
    assert!(response["id"].is_number());

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_biomeos_pattern_large_data() {
    let family_id = format!("test_large_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    wait_for_socket_ready(&socket_path).await;

    // Store large JSON object
    let large_data = json!({
        "records": (0..1000).map(|i| json!({
            "id": i,
            "name": format!("Record {}", i),
            "data": vec![i; 100]
        })).collect::<Vec<_>>()
    });

    let store_params = json!({
        "key": "large:dataset",
        "data": large_data,
        "family_id": family_id
    });

    let response = send_jsonrpc_request(&socket_path, "storage.store", store_params)
        .await
        .unwrap();
    assert_eq!(response["result"]["success"], true);

    // Retrieve and verify
    let retrieve_params = json!({
        "key": "large:dataset",
        "family_id": family_id
    });
    let response = send_jsonrpc_request(&socket_path, "storage.retrieve", retrieve_params)
        .await
        .unwrap();

    let records = response["result"]["data"]["records"].as_array().unwrap();
    assert_eq!(records.len(), 1000);

    std::fs::remove_file(socket_path).ok();
}
