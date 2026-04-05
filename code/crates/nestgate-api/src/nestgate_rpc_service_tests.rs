// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for `NestGateRpcServer` and `NestGateJsonRpcHandler`.

use super::nestgate_rpc_service::*;
use nestgate_zfs::command::ZfsOperations;
use std::sync::Arc;
use tarpc::context::Context;

#[test]
fn parse_zfs_size_units() {
    assert_eq!(parse_zfs_size("2T"), 2048);
    assert_eq!(parse_zfs_size("10G"), 10);
    assert_eq!(parse_zfs_size("1024M"), 1);
    assert_eq!(parse_zfs_size("1048576K"), 1);
    assert_eq!(parse_zfs_size(&format!("{}", 2u64 * 1024 * 1024 * 1024)), 2);
    assert_eq!(parse_zfs_size("  5G  "), 5);
    assert_eq!(parse_zfs_size("bad"), 0);
}

#[test]
fn parse_pool_capacity_from_zfs_pool() {
    let pool = nestgate_zfs::command::ZfsPool {
        name: "tank".to_string(),
        size: "4T".to_string(),
        allocated: "500G".to_string(),
        free: "3.5T".to_string(),
        health: "ONLINE".to_string(),
    };
    let (t, u, f) = parse_pool_capacity(&pool);
    assert_eq!(t, 4 * 1024);
    assert_eq!(u, 500);
    assert_eq!(f, 3072);
}

#[tokio::test]
async fn json_rpc_unknown_method_returns_err() {
    let handler = NestGateJsonRpcHandler::new();
    let err = handler
        .handle("not_a_real_method", serde_json::Value::Null)
        .await
        .unwrap_err();
    assert!(err.contains("Unknown method"));
}

#[tokio::test]
async fn json_rpc_list_datasets_requires_pool_string() {
    let handler = NestGateJsonRpcHandler::new();
    let err = handler
        .handle("list_datasets", serde_json::json!(123))
        .await
        .unwrap_err();
    assert!(!err.is_empty());
}

#[tokio::test]
async fn json_rpc_list_datasets_with_pool_serializes() {
    let handler = NestGateJsonRpcHandler::new();
    let v = handler
        .handle("list_datasets", serde_json::json!("my_pool"))
        .await
        .expect("list_datasets");
    assert!(v.is_array());
}

#[tokio::test]
async fn json_rpc_capabilities_aliases() {
    let handler = NestGateJsonRpcHandler::new();
    let a = handler
        .handle("capabilities", serde_json::Value::Null)
        .await
        .unwrap();
    let b = handler
        .handle("capabilities.list", serde_json::Value::Null)
        .await
        .unwrap();
    assert_eq!(a, b);
}

#[tokio::test]
async fn json_rpc_version_round_trip() {
    let handler = NestGateJsonRpcHandler::new();
    let v = handler
        .handle("version", serde_json::Value::Null)
        .await
        .expect("version");
    assert!(v.get("protocol").and_then(|x| x.as_str()) == Some("tarpc"));
}

#[tokio::test]
async fn json_rpc_health_readiness_shape() {
    let handler = NestGateJsonRpcHandler::new();
    let v = handler
        .handle("health.readiness", serde_json::Value::Null)
        .await
        .expect("readiness");
    assert!(v.get("ready").is_some());
    assert!(v.get("status").is_some());
}

#[tokio::test]
async fn test_rpc_server_creation() {
    let zfs_backend = Arc::new(ZfsOperations::new());
    let server = NestGateRpcServer::new(zfs_backend);
    let ctx = Context::current();

    let health = server.clone().health(ctx).await;
    assert!(
        !health.status.is_empty(),
        "Health status should not be empty"
    );
}

#[tokio::test]
async fn test_list_pools() {
    let zfs_backend = Arc::new(ZfsOperations::new());
    let server = NestGateRpcServer::new(zfs_backend);
    let ctx = Context::current();
    let _pools = server.list_pools(ctx).await;
}

#[tokio::test]
async fn test_capabilities() {
    let zfs_backend = Arc::new(ZfsOperations::new());
    let server = NestGateRpcServer::new(zfs_backend);
    let ctx = Context::current();

    let caps = server.capabilities(ctx).await;
    assert!(caps.contains(&"storage".to_string()));
    assert!(caps.contains(&"zfs".to_string()));
}

#[tokio::test]
async fn test_json_rpc_handler() {
    let handler = NestGateJsonRpcHandler::new();
    let result = handler.handle("health", serde_json::Value::Null).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn json_rpc_health_liveness_matches_health_core_fields() {
    let handler = NestGateJsonRpcHandler::new();
    let h = handler
        .handle("health", serde_json::Value::Null)
        .await
        .unwrap();
    let l = handler
        .handle("health.liveness", serde_json::Value::Null)
        .await
        .unwrap();
    assert_eq!(h.get("status"), l.get("status"));
    assert_eq!(h.get("pools_total"), l.get("pools_total"));
}

#[tokio::test]
async fn json_rpc_health_check_alias_is_health() {
    let handler = NestGateJsonRpcHandler::new();
    let a = handler
        .handle("health.check", serde_json::Value::Null)
        .await
        .unwrap();
    let b = handler
        .handle("health", serde_json::Value::Null)
        .await
        .unwrap();
    assert_eq!(a, b);
}

#[tokio::test]
async fn json_rpc_get_metrics_has_capacity_fields() {
    let handler = NestGateJsonRpcHandler::new();
    let v = handler
        .handle("get_metrics", serde_json::Value::Null)
        .await
        .unwrap();
    assert!(v.get("total_capacity_gb").is_some());
    assert!(v.get("dataset_count").is_some());
}

#[test]
fn rpc_pool_info_and_health_status_serde_roundtrip() {
    let p = PoolInfo {
        name: "t".to_string(),
        total_capacity_gb: 10,
        used_capacity_gb: 1,
        available_capacity_gb: 9,
        health_status: "ONLINE".to_string(),
        backend: "zfs".to_string(),
    };
    let v = serde_json::to_value(&p).unwrap();
    let back: PoolInfo = serde_json::from_value(v).unwrap();
    assert_eq!(back.name, p.name);

    let h = HealthStatus {
        status: "healthy".to_string(),
        version: "0".to_string(),
        uptime_seconds: 1,
        pools_healthy: 1,
        pools_total: 1,
    };
    let v2 = serde_json::to_string(&h).unwrap();
    let h2: HealthStatus = serde_json::from_str(&v2).unwrap();
    assert_eq!(h2.status, "healthy");
}

#[test]
fn operation_result_and_version_info_serde() {
    let r = OperationResult {
        success: true,
        message: "ok".to_string(),
        data: Some(serde_json::json!({"a": 1})),
    };
    let s = serde_json::to_string(&r).unwrap();
    let r2: OperationResult = serde_json::from_str(&s).unwrap();
    assert!(r2.success);

    let vi = VersionInfo {
        version: "1".to_string(),
        protocol: "tarpc".to_string(),
        capabilities: vec!["storage".to_string()],
    };
    let v = serde_json::to_value(&vi).unwrap();
    assert_eq!(v["protocol"], "tarpc");
}
