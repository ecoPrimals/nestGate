// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use crate::rest::rpc::config::NestGateRpcConfig;
use crate::rest::rpc::types::{DynRpcService, RequestPriority, UnifiedRpcRequest};
use std::time::Duration;
use uuid::Uuid;

#[test]
fn unified_rpc_manager_display() {
    let m = UnifiedRpcManager::new();
    assert!(format!("{m}").contains("UnifiedRpcManager"));
}

#[test]
fn with_config_builds_manager() {
    let c = NestGateRpcConfig::default();
    let _m = UnifiedRpcManager::with_config(c);
}

#[tokio::test]
async fn call_unknown_service_returns_error_payload() {
    let m = UnifiedRpcManager::new();
    let req = UnifiedRpcRequest {
        id: Uuid::nil(),
        source: "a".to_string(),
        target: "missing".to_string(),
        method: "m".to_string(),
        _params: serde_json::json!({}),
        _metadata: std::collections::HashMap::new(),
        timestamp: chrono::Utc::now(),
        streaming: false,
        priority: RequestPriority::Normal,
        timeout: Some(Duration::from_secs(1)),
    };
    let res = m.call("no_such", req).await.expect("call");
    assert!(!res.success);
    assert!(res.error.unwrap().contains("not found"));
}

#[test]
fn init_tarpc_rejects_empty_endpoint() {
    let mut m = UnifiedRpcManager::new();
    assert!(m.init_tarpc_service("").is_err());
    assert!(m.init_tarpc_service("127.0.0.1:9000").is_ok());
}

#[test]
fn init_json_rpc_validates_empty_and_http_url() {
    let mut m = UnifiedRpcManager::new();
    assert!(m.init_json_rpc_service("").is_err());
    assert!(m.init_json_rpc_service("http://127.0.0.1:8080/rpc").is_ok());
    assert!(m.init_json_rpc_service("/unix/socket").is_ok());
}

#[test]
fn get_health_status_returns_json() {
    let m = UnifiedRpcManager::new();
    let v = m.get_health_status().unwrap();
    assert_eq!(v["status"], "healthy");
}

#[test]
fn connection_pool_new_reads_config() {
    let mut c = NestGateRpcConfig::default();
    c.connection_pool.max_connections = 7;
    c.connection_pool.connection_timeout = Duration::from_millis(500);
    let p = ConnectionPool::new(&c.connection_pool);
    assert!(format!("{p:?}").contains("ConnectionPool"));
}

#[test]
#[allow(deprecated)]
fn metrics_config_default() {
    let m = MetricsConfig::default();
    assert!(m.enabled);
}

#[test]
fn init_security_capability_ok() {
    let mgr = UnifiedRpcManager::new();
    assert!(mgr.init_security_capability("x").is_ok());
}

#[tokio::test]
async fn register_and_call_json_rpc_service() {
    use crate::rest::rpc::json_rpc_service::JsonRpcService;

    let mgr = UnifiedRpcManager::new();
    let svc = JsonRpcService::new("http://127.0.0.1:9/jsonrpc".to_string());
    mgr.register_service("j".to_string(), DynRpcService::JsonRpc(svc))
        .await
        .unwrap();

    let req = UnifiedRpcRequest {
        id: Uuid::nil(),
        source: "s".to_string(),
        target: "t".to_string(),
        method: "m".to_string(),
        _params: serde_json::json!({}),
        _metadata: std::collections::HashMap::new(),
        timestamp: chrono::Utc::now(),
        streaming: false,
        priority: RequestPriority::Normal,
        timeout: None,
    };
    let _ = mgr.call("j", req).await;
}

#[test]
fn start_and_shutdown_roundtrip() {
    let mut m = UnifiedRpcManager::new();
    assert!(m.start().is_ok());
    assert!(m.shutdown().is_ok());
}

#[test]
fn start_bidirectional_stream_returns_channels() {
    let m = UnifiedRpcManager::new();
    let req = UnifiedRpcRequest {
        id: Uuid::nil(),
        source: "s".to_string(),
        target: "t".to_string(),
        method: "m".to_string(),
        _params: serde_json::json!({}),
        _metadata: std::collections::HashMap::new(),
        timestamp: chrono::Utc::now(),
        streaming: true,
        priority: RequestPriority::High,
        timeout: None,
    };
    let (tx, _rx) = m.start_bidirectional_stream(req).unwrap();
    drop(tx);
}
