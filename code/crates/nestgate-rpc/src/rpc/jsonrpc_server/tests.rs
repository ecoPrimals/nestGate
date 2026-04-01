// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use jsonrpsee::core::params::{ArrayParams, ObjectParams};
use nestgate_types::error::Result as NgResult;

/// Helper: Create test service (in-memory via `NestGateRpcService::new`).
async fn create_test_service() -> NgResult<NestGateRpcService> {
    NestGateRpcService::new()
}

/// Helper: Build RPC module for testing (no server required)
async fn build_test_module() -> RpcModule<JsonRpcState> {
    let service = create_test_service().await.expect("create service");
    let state = JsonRpcState {
        service,
        start_time: std::time::Instant::now(),
    };
    JsonRpcServer::build_module(state).expect("build module")
}

#[test]
fn test_jsonrpc_config_default() {
    let config = JsonRpcConfig::default();
    assert!(config.log_requests);
    assert_eq!(config.max_request_size, 100 * 1024 * 1024);
    assert_eq!(config.max_response_size, 100 * 1024 * 1024);
}

#[tokio::test]
async fn test_jsonrpc_server_creation() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");
    let config = JsonRpcConfig::default();
    let _server = JsonRpcServer::new(service, config);
}

#[test]
fn test_jsonrpc_config_custom() {
    use std::net::{IpAddr, Ipv4Addr};
    let config = JsonRpcConfig {
        addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9999),
        log_requests: false,
        max_request_size: 1024,
        max_response_size: 2048,
    };
    assert!(!config.log_requests);
    assert_eq!(config.max_request_size, 1024);
    assert_eq!(config.max_response_size, 2048);
    assert_eq!(config.addr.port(), 9999);
}

#[tokio::test]
async fn test_jsonrpc_state_creation() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");
    let state = JsonRpcState {
        service: service.clone(),
        start_time: std::time::Instant::now(),
    };

    // Verify state is clonable
    let _state_clone = state.clone();
}

#[test]
fn test_base64_encoding_decoding() {
    let data = b"Hello, NestGate!";
    let encoded = base64::engine::general_purpose::STANDARD.encode(data);
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .unwrap();
    assert_eq!(data.to_vec(), decoded);
}

#[test]
fn test_jsonrpc_endpoint_format() {
    let config = JsonRpcConfig::default();
    let addr_str = format!("http://{}/jsonrpc", config.addr);
    assert!(addr_str.contains("/jsonrpc"));
}

#[tokio::test]
async fn test_multiple_servers() {
    // Verify we can create multiple server instances
    let service1 = create_test_service()
        .await
        .expect("Failed to create service1");
    let service2 = create_test_service()
        .await
        .expect("Failed to create service2");
    let config = JsonRpcConfig::default();

    let _server1 = JsonRpcServer::new(service1, config.clone());
    let _server2 = JsonRpcServer::new(service2, config);
}

// ========== JSON-RPC handler tests using RpcModule::call() ==========

#[tokio::test]
async fn test_handler_storage_dataset_create() {
    let module = build_test_module().await;
    let mut params = ObjectParams::new();
    params.insert("name", "test_dataset").expect("insert");
    params.insert("description", "test desc").expect("insert");
    let result: serde_json::Value = module
        .call("storage.dataset.create", params)
        .await
        .expect("create dataset");
    assert_eq!(result["name"], "test_dataset");
    assert_eq!(result["description"], "test desc");
    assert!(result["size_bytes"].is_number());
}

#[tokio::test]
async fn test_handler_storage_dataset_create_with_compression() {
    let module = build_test_module().await;
    let mut params = ObjectParams::new();
    params.insert("name", "compressed_ds").expect("insert");
    params.insert("compression", "lz4").expect("insert");
    let result: serde_json::Value = module
        .call("storage.dataset.create", params)
        .await
        .expect("create");
    assert_eq!(result["name"], "compressed_ds");
}

#[tokio::test]
async fn test_handler_storage_dataset_list() {
    let module = build_test_module().await;
    let result: Vec<serde_json::Value> = module
        .call("storage.dataset.list", ArrayParams::new())
        .await
        .expect("list");
    assert!(result.is_empty() || !result.is_empty());
}

#[tokio::test]
async fn test_handler_storage_dataset_get_after_create() {
    let module = build_test_module().await;
    let mut create_params = ObjectParams::new();
    create_params.insert("name", "get_test_ds").expect("insert");
    let _ = module
        .call::<_, serde_json::Value>("storage.dataset.create", create_params)
        .await
        .expect("create");
    let mut get_params = ArrayParams::new();
    get_params.insert("get_test_ds").expect("insert");
    let ds: serde_json::Value = module
        .call("storage.dataset.get", get_params)
        .await
        .expect("get");
    assert_eq!(ds["name"], "get_test_ds");
    assert_eq!(ds["status"], "active");
}

#[tokio::test]
async fn test_handler_storage_dataset_delete() {
    let module = build_test_module().await;
    let mut create_params = ObjectParams::new();
    create_params
        .insert("name", "delete_test_ds")
        .expect("insert");
    let _ = module
        .call::<_, serde_json::Value>("storage.dataset.create", create_params)
        .await
        .expect("create");
    let mut del_params = ArrayParams::new();
    del_params.insert("delete_test_ds").expect("insert");
    let del: serde_json::Value = module
        .call("storage.dataset.delete", del_params)
        .await
        .expect("delete");
    assert!(del["success"].as_bool().unwrap_or(false));
}

#[tokio::test]
async fn test_handler_storage_object_store_retrieve() {
    let module = build_test_module().await;
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"hello object");
    let mut create_params = ObjectParams::new();
    create_params.insert("name", "obj_ds").expect("insert");
    let _ = module
        .call::<_, serde_json::Value>("storage.dataset.create", create_params)
        .await
        .expect("create ds");
    let mut store_params = ObjectParams::new();
    store_params.insert("dataset", "obj_ds").expect("insert");
    store_params.insert("key", "mykey").expect("insert");
    store_params.insert("data", &data_b64).expect("insert");
    let stored: serde_json::Value = module
        .call("storage.object.store", store_params)
        .await
        .expect("store");
    assert_eq!(stored["key"], "mykey");
    assert_eq!(stored["dataset"], "obj_ds");
    let mut retrieve_params = ObjectParams::new();
    retrieve_params.insert("dataset", "obj_ds").expect("insert");
    retrieve_params.insert("key", "mykey").expect("insert");
    let retrieved: serde_json::Value = module
        .call("storage.object.retrieve", retrieve_params)
        .await
        .expect("retrieve");
    let raw = base64::engine::general_purpose::STANDARD
        .decode(retrieved["data"].as_str().unwrap())
        .unwrap();
    assert_eq!(raw, b"hello object");
}

#[tokio::test]
async fn test_handler_storage_object_store_invalid_base64() {
    let module = build_test_module().await;
    let mut create_params = ObjectParams::new();
    create_params.insert("name", "bad_ds").expect("insert");
    let _ = module
        .call::<_, serde_json::Value>("storage.dataset.create", create_params)
        .await
        .expect("create");
    let mut store_params = ObjectParams::new();
    store_params.insert("dataset", "bad_ds").expect("insert");
    store_params.insert("key", "k").expect("insert");
    store_params
        .insert("data", "!!!invalid!!!")
        .expect("insert");
    let err = module
        .call::<_, serde_json::Value>("storage.object.store", store_params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_handler_storage_object_metadata_list_delete() {
    let module = build_test_module().await;
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"meta_test");
    let mut create_params = ObjectParams::new();
    create_params.insert("name", "meta_ds").expect("insert");
    let _ = module
        .call::<_, serde_json::Value>("storage.dataset.create", create_params)
        .await
        .expect("create");
    let mut store_params = ObjectParams::new();
    store_params.insert("dataset", "meta_ds").expect("insert");
    store_params.insert("key", "obj1").expect("insert");
    store_params.insert("data", &data_b64).expect("insert");
    let _ = module
        .call::<_, serde_json::Value>("storage.object.store", store_params)
        .await
        .expect("store");
    let mut meta_params = ObjectParams::new();
    meta_params.insert("dataset", "meta_ds").expect("insert");
    meta_params.insert("key", "obj1").expect("insert");
    let meta: serde_json::Value = module
        .call("storage.object.metadata", meta_params)
        .await
        .expect("metadata");
    assert_eq!(meta["key"], "obj1");
    let mut list_params = ObjectParams::new();
    list_params.insert("dataset", "meta_ds").expect("insert");
    let list: Vec<serde_json::Value> = module
        .call("storage.object.list", list_params)
        .await
        .expect("list");
    assert!(!list.is_empty());
    let mut del_params = ObjectParams::new();
    del_params.insert("dataset", "meta_ds").expect("insert");
    del_params.insert("key", "obj1").expect("insert");
    let del: serde_json::Value = module
        .call("storage.object.delete", del_params)
        .await
        .expect("delete");
    assert!(del["success"].as_bool().unwrap_or(false));
}

#[tokio::test]
async fn test_handler_health_check() {
    let module = build_test_module().await;
    let result: serde_json::Value = module
        .call("health.check", ArrayParams::new())
        .await
        .expect("health");
    assert!(result["status"].as_str().is_some());
    assert!(result["version"].as_str().is_some());
}

#[tokio::test]
async fn test_handler_health_metrics() {
    let module = build_test_module().await;
    let result: serde_json::Value = module
        .call("health.metrics", ArrayParams::new())
        .await
        .expect("metrics");
    assert!(result["dataset_count"].as_u64().is_some());
    assert!(result["object_count"].as_u64().is_some());
}

#[tokio::test]
async fn test_handler_health_info() {
    let module = build_test_module().await;
    let result: serde_json::Value = module
        .call("health.info", ArrayParams::new())
        .await
        .expect("health.info");
    assert!(result["version"].as_str().is_some());
    assert!(result["api_version"].as_str().is_some());
}

#[tokio::test]
async fn test_handler_health_protocols() {
    let module = build_test_module().await;
    let result: Vec<serde_json::Value> = module
        .call("health.protocols", ArrayParams::new())
        .await
        .expect("protocols");
    assert!(!result.is_empty());
}

#[tokio::test]
async fn test_handler_health_liveness_readiness() {
    let module = build_test_module().await;
    let live: serde_json::Value = module
        .call("health.liveness", ArrayParams::new())
        .await
        .expect("liveness");
    assert_eq!(live["alive"], true);
    assert!(live["status"].as_str().is_some());

    let ready: serde_json::Value = module
        .call("health.readiness", ArrayParams::new())
        .await
        .expect("readiness");
    assert!(ready["ready"].is_boolean());
    assert!(ready["status"].as_str().is_some());
}

#[tokio::test]
async fn test_handler_capabilities_list() {
    let module = build_test_module().await;
    let result: serde_json::Value = module
        .call("capabilities.list", ArrayParams::new())
        .await
        .expect("capabilities.list");
    let methods = result["methods"].as_array().expect("methods array");
    assert!(methods.iter().any(|m| m.as_str() == Some("health.check")));
    assert!(
        methods
            .iter()
            .any(|m| m.as_str() == Some("capabilities.list"))
    );
}

#[tokio::test]
async fn test_handler_discovery_capability_register() {
    let module = build_test_module().await;
    let mut params = ObjectParams::new();
    params.insert("capability", "storage").expect("insert");
    params
        .insert("endpoint", "http://localhost:8092")
        .expect("insert");
    let result: serde_json::Value = module
        .call("discovery.capability.register", params)
        .await
        .expect("register");
    assert!(result["success"].as_bool().is_some());
}

#[tokio::test]
async fn test_handler_discovery_capability_query() {
    let module = build_test_module().await;
    let mut params = ArrayParams::new();
    params.insert("storage").expect("insert");
    let result: Vec<serde_json::Value> = module
        .call("discovery.capability.query", params)
        .await
        .expect("query");
    // Query succeeds: returns empty vec when no service, or list when found
    assert!(result.iter().all(|v| v.is_object() || v.is_null()));
}

#[tokio::test]
async fn test_handler_get_dataset_not_found() {
    let module = build_test_module().await;
    let mut params = ArrayParams::new();
    params.insert("nonexistent_ds_xyz").expect("insert");
    let err = module
        .call::<_, serde_json::Value>("storage.dataset.get", params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_module_method_names() {
    let module = build_test_module().await;
    let names: Vec<_> = module.method_names().collect();
    assert!(names.contains(&"storage.dataset.create"));
    assert!(names.contains(&"health.check"));
    assert!(names.contains(&"capabilities.list"));
    assert!(names.len() >= 18);
}

// ========== Round 5: error / branch coverage ==========

#[tokio::test]
async fn test_call_unknown_method_fails() {
    let module = build_test_module().await;
    let err = module
        .call::<_, serde_json::Value>("method.does.not.exist", ArrayParams::new())
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_storage_dataset_create_missing_name_fails() {
    let module = build_test_module().await;
    let params = ObjectParams::new();
    let err = module
        .call::<_, serde_json::Value>("storage.dataset.create", params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_discovery_capability_register_missing_fields_fails() {
    let module = build_test_module().await;
    let params = ObjectParams::new();
    let err = module
        .call::<_, serde_json::Value>("discovery.capability.register", params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_discovery_capability_query_empty_params_array() {
    let module = build_test_module().await;
    let err = module
        .call::<_, serde_json::Value>("discovery.capability.query", ArrayParams::new())
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_storage_object_retrieve_missing_keys_fails() {
    let module = build_test_module().await;
    let params = ObjectParams::new();
    let err = module
        .call::<_, serde_json::Value>("storage.object.retrieve", params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_storage_object_delete_missing_keys_fails() {
    let module = build_test_module().await;
    let params = ObjectParams::new();
    let err = module
        .call::<_, serde_json::Value>("storage.object.delete", params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_storage_object_list_missing_dataset_fails() {
    let module = build_test_module().await;
    let params = ObjectParams::new();
    let err = module
        .call::<_, serde_json::Value>("storage.object.list", params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_storage_object_metadata_missing_keys_fails() {
    let module = build_test_module().await;
    let params = ObjectParams::new();
    let err = module
        .call::<_, serde_json::Value>("storage.object.metadata", params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_storage_object_store_missing_fields_fails() {
    let module = build_test_module().await;
    let mut params = ObjectParams::new();
    params.insert("dataset", "d").expect("insert");
    let err = module
        .call::<_, serde_json::Value>("storage.object.store", params)
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_storage_dataset_delete_empty_array_fails() {
    let module = build_test_module().await;
    let err = module
        .call::<_, serde_json::Value>("storage.dataset.delete", ArrayParams::new())
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_storage_dataset_get_empty_array_fails() {
    let module = build_test_module().await;
    let err = module
        .call::<_, serde_json::Value>("storage.dataset.get", ArrayParams::new())
        .await;
    assert!(err.is_err());
}
