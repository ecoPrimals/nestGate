//! # 🗂️ Template Storage Integration Tests
//!
//! **Comprehensive Integration Tests for Template Storage via Unix Socket**
//!
//! Tests the template.* JSON-RPC methods through the Unix socket server,
//! verifying full integration with the collaborative intelligence system.

use nestgate_core::rpc::unix_socket_server::JsonRpcUnixServer;
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// Test helper: Send JSON-RPC request and get response
async fn send_jsonrpc_request(
    socket_path: &PathBuf,
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
async fn test_template_store_retrieve_workflow() {
    let family_id = format!("test_template_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Store template
    let store_params = json!({
        "name": "FastAPI CRUD Service",
        "description": "REST API with PostgreSQL database",
        "graph_data": {
            "nodes": [
                {"id": "api", "type": "fastapi"},
                {"id": "db", "type": "postgres"}
            ],
            "edges": [
                {"from": "api", "to": "db"}
            ]
        },
        "user_id": "user_abc123",
        "family_id": family_id,
        "metadata": {
            "tags": ["api", "rest", "database"],
            "niche_type": "web_service",
            "is_community": false
        }
    });

    let response = send_jsonrpc_request(&socket_path, "templates.store", store_params)
        .await
        .unwrap();

    assert!(
        response["result"].is_object(),
        "Expected result object, got: {:?}",
        response
    );
    assert_eq!(response["result"]["success"], true);
    let template_id = response["result"]["template_id"]
        .as_str()
        .expect("template_id should be a string");
    assert!(template_id.starts_with("tmpl_"));

    // Retrieve template
    let retrieve_params = json!({
        "template_id": template_id,
        "family_id": family_id
    });

    let response = send_jsonrpc_request(&socket_path, "templates.retrieve", retrieve_params)
        .await
        .unwrap();

    assert_eq!(response["result"]["name"], "FastAPI CRUD Service");
    assert_eq!(response["result"]["user_id"], "user_abc123");
    assert_eq!(response["result"]["version"], 1);

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_template_list_filtering() {
    let family_id = format!("test_list_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Store multiple templates
    for i in 0..3 {
        let params = json!({
            "name": format!("Template {}", i),
            "description": format!("Description {}", i),
            "graph_data": {},
            "user_id": format!("user_{}", i % 2),
            "family_id": family_id,
            "metadata": {
                "tags": if i == 0 { ["api"] } else { ["ml"] },
                "niche_type": if i < 2 { "web_service" } else { "ml_pipeline" },
                "is_community": i == 0
            }
        });
        send_jsonrpc_request(&socket_path, "templates.store", params)
            .await
            .unwrap();
    }

    // List all templates
    let list_params = json!({"family_id": family_id});
    let response = send_jsonrpc_request(&socket_path, "templates.list", list_params)
        .await
        .unwrap();

    assert!(
        response["result"].is_object(),
        "Expected result, got: {:?}",
        response
    );
    let total = response["result"]["total"]
        .as_u64()
        .expect("total should be a number");
    assert_eq!(total, 3);

    // Filter by user
    let list_params = json!({
        "family_id": family_id,
        "user_id": "user_0"
    });
    let response = send_jsonrpc_request(&socket_path, "templates.list", list_params)
        .await
        .unwrap();
    let total = response["result"]["total"]
        .as_u64()
        .expect("total should be a number");
    assert_eq!(total, 2);

    // Filter by niche_type
    let list_params = json!({
        "family_id": family_id,
        "niche_type": "web_service"
    });
    let response = send_jsonrpc_request(&socket_path, "templates.list", list_params)
        .await
        .unwrap();
    let total = response["result"]["total"]
        .as_u64()
        .expect("total should be a number");
    assert_eq!(total, 2);

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_community_top_ranking() {
    let family_id = format!("test_community_{}", uuid::Uuid::new_v4());
    let server = JsonRpcUnixServer::new(&family_id).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Store community templates with different stats
    let params1 = json!({
        "name": "Popular Template",
        "description": "High usage and rating",
        "graph_data": {},
        "user_id": "user_1",
        "family_id": family_id,
        "metadata": {
            "tags": ["popular"],
            "niche_type": "web_service",
            "usage_count": 100,
            "success_rate": 0.95,
            "is_community": true,
            "community_rating": 4.8,
            "rating_count": 50
        }
    });
    send_jsonrpc_request(&socket_path, "templates.store", params1)
        .await
        .unwrap();

    let params2 = json!({
        "name": "Less Popular",
        "description": "Lower stats",
        "graph_data": {},
        "user_id": "user_2",
        "family_id": family_id,
        "metadata": {
            "tags": ["test"],
            "niche_type": "web_service",
            "usage_count": 10,
            "success_rate": 0.80,
            "is_community": true,
            "community_rating": 3.0,
            "rating_count": 5
        }
    });
    send_jsonrpc_request(&socket_path, "templates.store", params2)
        .await
        .unwrap();

    // Get top community templates
    let top_params = json!({
        "niche_type": "web_service",
        "limit": 10,
        "min_usage": 5
    });
    let response = send_jsonrpc_request(&socket_path, "templates.community_top", top_params)
        .await
        .unwrap();

    let templates = response["result"]["templates"].as_array().unwrap();
    assert_eq!(templates.len(), 2);

    // First should be more popular
    assert_eq!(templates[0]["name"], "Popular Template");
    assert!(templates[0]["score"].as_f64().unwrap() > templates[1]["score"].as_f64().unwrap());

    std::fs::remove_file(socket_path).ok();
}

/// Requires running NestGate server
#[tokio::test]
#[ignore]
async fn test_template_family_isolation() {
    let family_id_1 = format!("test_family_1_{}", uuid::Uuid::new_v4());
    let family_id_2 = format!("test_family_2_{}", uuid::Uuid::new_v4());

    let server = JsonRpcUnixServer::new(&family_id_1).await.unwrap();
    let socket_path = server.socket_path().clone();

    tokio::spawn(async move {
        server.serve().await.ok();
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Store template for family 1
    let store_params = json!({
        "name": "Family1 Template",
        "description": "For family 1",
        "graph_data": {},
        "user_id": "user_123",
        "family_id": family_id_1,
        "metadata": {
            "tags": ["test"],
            "niche_type": "test"
        }
    });
    let response = send_jsonrpc_request(&socket_path, "templates.store", store_params)
        .await
        .unwrap();
    let template_id = response["result"]["template_id"]
        .as_str()
        .expect("template_id should be present");

    // Try to retrieve from family 2 (should fail)
    let retrieve_params = json!({
        "template_id": template_id,
        "family_id": family_id_2
    });
    let response = send_jsonrpc_request(&socket_path, "templates.retrieve", retrieve_params)
        .await
        .unwrap();
    assert!(response["error"].is_object());

    // Retrieve from correct family (should work)
    let retrieve_params = json!({
        "template_id": template_id,
        "family_id": family_id_1
    });
    let response = send_jsonrpc_request(&socket_path, "templates.retrieve", retrieve_params)
        .await
        .unwrap();
    assert_eq!(response["result"]["name"], "Family1 Template");

    std::fs::remove_file(socket_path).ok();
}
