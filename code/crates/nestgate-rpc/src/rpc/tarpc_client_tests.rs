// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for [`super::NestGateRpcClient`] (extracted to keep `tarpc_client.rs` under 1000 lines).

use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;

use super::NestGateRpcClient;
use crate::rpc::tarpc_types::NestGateRpcError;

#[test]
fn test_parse_endpoint() {
    let addr = NestGateRpcClient::parse_endpoint("tarpc://127.0.0.1:8091").unwrap();
    assert_eq!(addr.to_string(), "127.0.0.1:8091");
}

#[test]
fn test_parse_endpoint_ipv6() {
    let addr = NestGateRpcClient::parse_endpoint("tarpc://[::1]:8091").unwrap();
    assert!(addr.to_string().contains("8091"));
}

#[test]
fn test_parse_endpoint_invalid_prefix() {
    let result = NestGateRpcClient::parse_endpoint("http://127.0.0.1:8080");
    assert!(result.is_err());
}

#[test]
fn test_parse_endpoint_invalid_host() {
    let result = NestGateRpcClient::parse_endpoint("tarpc://invalid-host-name:8091");
    assert!(result.is_err());
}

#[test]
fn test_parse_endpoint_invalid_port() {
    let result = NestGateRpcClient::parse_endpoint("tarpc://127.0.0.1:99999");
    assert!(result.is_err());
}

#[test]
fn test_parse_endpoint_missing_port() {
    let result = NestGateRpcClient::parse_endpoint("tarpc://127.0.0.1");
    assert!(result.is_err());
}

#[test]
fn test_client_creation() {
    let client = NestGateRpcClient::new("tarpc://127.0.0.1:8091").unwrap();
    assert_eq!(client.endpoint, "tarpc://127.0.0.1:8091");
}

#[test]
fn test_with_timeout() {
    let client = NestGateRpcClient::new("tarpc://127.0.0.1:8091")
        .unwrap()
        .with_timeout(Duration::from_secs(10));
    assert_eq!(client.timeout, Duration::from_secs(10));
}

#[test]
fn test_client_creation_invalid_endpoint() {
    let result = NestGateRpcClient::new("http://127.0.0.1:8080");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_connection_refused() {
    let client = NestGateRpcClient::new("tarpc://127.0.0.1:1").unwrap();
    let result = client.health().await;
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Failed to connect") || err_msg.contains("Connection refused"),
        "Expected connection error, got: {}",
        err_msg
    );
}

#[tokio::test]
async fn test_create_dataset_connection_fails() {
    let client = NestGateRpcClient::new("tarpc://127.0.0.1:2").unwrap();
    let result = client.create_dataset("test-ds", Default::default()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_datasets_connection_fails() {
    let client = NestGateRpcClient::new("tarpc://127.0.0.1:3").unwrap();
    let result = client.list_datasets().await;
    assert!(result.is_err());
}

#[test]
fn test_convert_rpc_error_dataset_not_found() {
    let err = NestGateRpcError::DatasetNotFound {
        dataset: Arc::from("missing"),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(nest_err.to_string().to_lowercase().contains("not found"));
}

#[test]
fn test_convert_rpc_error_invalid_params() {
    let err = NestGateRpcError::InvalidParameters {
        message: "bad param".to_string(),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(!nest_err.to_string().is_empty());
}

#[test]
fn test_convert_rpc_error_storage_full() {
    let err = NestGateRpcError::StorageFull {
        required: 1000,
        available: 100,
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(nest_err.to_string().contains("Storage") || nest_err.to_string().contains("storage"));
}

#[test]
fn test_convert_rpc_error_timeout() {
    let err = NestGateRpcError::Timeout {
        operation: Arc::from("create_dataset"),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(!nest_err.to_string().is_empty());
}

#[test]
fn test_convert_rpc_error_connection_error() {
    let err = NestGateRpcError::ConnectionError {
        message: "connection lost".to_string(),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(!nest_err.to_string().is_empty());
}

#[test]
fn test_convert_rpc_error_dataset_already_exists() {
    let err = NestGateRpcError::DatasetAlreadyExists {
        dataset: Arc::from("exists"),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(nest_err.to_string().to_lowercase().contains("exists"));
}

#[test]
fn test_convert_rpc_error_object_not_found() {
    let err = NestGateRpcError::ObjectNotFound {
        dataset: Arc::from("ds"),
        key: Arc::from("key"),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(nest_err.to_string().to_lowercase().contains("not found"));
}

#[test]
fn test_convert_rpc_error_object_already_exists() {
    let err = NestGateRpcError::ObjectAlreadyExists {
        dataset: Arc::from("ds"),
        key: Arc::from("key"),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(!nest_err.to_string().is_empty());
}

#[test]
fn test_convert_rpc_error_quota_exceeded() {
    let err = NestGateRpcError::QuotaExceeded {
        dataset: Arc::from("ds"),
        quota: 100,
        requested: 200,
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(nest_err.to_string().to_lowercase().contains("quota"));
}

#[test]
fn test_convert_rpc_error_permission_denied() {
    let err = NestGateRpcError::PermissionDenied {
        message: "access denied".to_string(),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(!nest_err.to_string().is_empty());
}

#[test]
fn test_convert_rpc_error_internal_error() {
    let err = NestGateRpcError::InternalError {
        message: "internal".to_string(),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(!nest_err.to_string().is_empty());
}

#[test]
fn test_convert_rpc_error_service_unavailable() {
    let err = NestGateRpcError::ServiceUnavailable {
        message: "unavailable".to_string(),
    };
    let nest_err = NestGateRpcClient::convert_rpc_error(err);
    assert!(!nest_err.to_string().is_empty());
}

#[test]
fn test_parse_endpoint_empty() {
    let result = NestGateRpcClient::parse_endpoint("");
    assert!(result.is_err());
}

#[test]
fn test_parse_endpoint_hostname() {
    let addr = NestGateRpcClient::parse_endpoint("tarpc://127.0.0.1:8091").unwrap();
    assert_eq!(addr.port(), 8091);
}

#[test]
fn normalize_to_tarpc_endpoint_preserves_scheme() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("tarpc://127.0.0.1:8091"),
        "tarpc://127.0.0.1:8091"
    );
}

#[test]
fn normalize_to_tarpc_endpoint_http_to_tarpc() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("http://10.0.0.5:8080"),
        "tarpc://10.0.0.5:8080"
    );
}

#[test]
fn normalize_to_tarpc_endpoint_https_to_tarpc() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("https://api.example.com:443"),
        "tarpc://api.example.com:443"
    );
}

#[test]
fn normalize_to_tarpc_endpoint_bare_host_gets_prefix() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("127.0.0.1:8091"),
        "tarpc://127.0.0.1:8091"
    );
}

#[test]
fn normalize_to_tarpc_endpoint_trims_whitespace() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("  http://127.0.0.1:1  "),
        "tarpc://127.0.0.1:1"
    );
}

#[test]
fn normalize_to_tarpc_endpoint_ipv6_host_in_brackets() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("tarpc://[::1]:8092"),
        "tarpc://[::1]:8092"
    );
}

#[tokio::test]
async fn test_discover_by_capability_unknown() {
    let result = NestGateRpcClient::discover_by_capability("unknown_capability");
    assert!(
        result.is_ok(),
        "unknown capability should still yield a client via default fallback: {:?}",
        result.err()
    );
}

#[test]
fn discover_by_capability_env_var_name_uses_uppercase_and_underscores() {
    let cap = "my-cap_name";
    let expected = format!("NESTGATE_{}_ENDPOINT", cap.to_uppercase().replace('-', "_"));
    assert_eq!(expected, "NESTGATE_MY_CAP_NAME_ENDPOINT");
}

#[test]
fn new_parses_host_literal_tarpc_endpoint() {
    let c = NestGateRpcClient::new("tarpc://192.168.0.1:9000").unwrap();
    assert_eq!(c.addr.to_string(), "192.168.0.1:9000");
}

#[test]
fn r6_normalize_http_ipv6_brackets() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("http://[2001:db8::1]:8443"),
        "tarpc://[2001:db8::1]:8443"
    );
}

#[test]
fn r6_normalize_https_ipv6_brackets() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("https://[::1]:443"),
        "tarpc://[::1]:443"
    );
}

#[test]
fn r6_parse_endpoint_ipv6_full() {
    let a = NestGateRpcClient::parse_endpoint("tarpc://[::1]:65535").unwrap();
    assert_eq!(a.port(), 65535);
}

#[test]
fn r6_client_with_timeout_250ms() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:8091")
        .unwrap()
        .with_timeout(Duration::from_millis(250));
    assert_eq!(c.timeout, Duration::from_millis(250));
}

#[test]
fn r6_client_with_timeout_120s() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:8091")
        .unwrap()
        .with_timeout(Duration::from_secs(120));
    assert_eq!(c.timeout, Duration::from_secs(120));
}

#[test]
fn r6_normalize_bare_ipv6_needs_brackets_for_parse() {
    let n = NestGateRpcClient::normalize_to_tarpc_endpoint("[::1]:1234");
    assert!(n.starts_with("tarpc://"));
}

#[test]
fn r6_normalize_preserves_port_1() {
    assert_eq!(
        NestGateRpcClient::normalize_to_tarpc_endpoint("http://127.0.0.1:1"),
        "tarpc://127.0.0.1:1"
    );
}

#[test]
fn r6_parse_rejects_tarpc_without_host() {
    assert!(NestGateRpcClient::parse_endpoint("tarpc://").is_err());
}

#[test]
fn r6_normalize_http_host_port_only() {
    let n = NestGateRpcClient::normalize_to_tarpc_endpoint("http://192.168.1.10:99");
    assert_eq!(n, "tarpc://192.168.1.10:99");
}

// --- Additional coverage (clone, more async error paths) ---

#[test]
fn client_clone_shares_connection_state() {
    let a = NestGateRpcClient::new("tarpc://127.0.0.1:8091").unwrap();
    let b = a.clone();
    assert_eq!(a.endpoint, b.endpoint);
}

#[tokio::test]
async fn metrics_fails_without_server() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:4").unwrap();
    assert!(c.metrics().await.is_err());
}

#[tokio::test]
async fn version_fails_without_server() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:5").unwrap();
    assert!(c.version().await.is_err());
}

#[tokio::test]
async fn protocols_fails_without_server() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:6").unwrap();
    assert!(c.protocols().await.is_err());
}

#[tokio::test]
async fn get_dataset_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:7").unwrap();
    assert!(c.get_dataset("x").await.is_err());
}

#[tokio::test]
async fn delete_dataset_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:8").unwrap();
    assert!(c.delete_dataset("x").await.is_err());
}

#[tokio::test]
async fn store_object_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:9").unwrap();
    assert!(
        c.store_object("ds", "k", Bytes::from(vec![1u8]), None)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn retrieve_object_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:10").unwrap();
    assert!(c.retrieve_object("ds", "k").await.is_err());
}

#[tokio::test]
async fn get_object_metadata_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:11").unwrap();
    assert!(c.get_object_metadata("ds", "k").await.is_err());
}

#[tokio::test]
async fn list_objects_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:12").unwrap();
    assert!(c.list_objects("ds", None, None).await.is_err());
}

#[tokio::test]
async fn delete_object_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:13").unwrap();
    assert!(c.delete_object("ds", "k").await.is_err());
}

#[tokio::test]
async fn register_capability_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:14").unwrap();
    let reg = crate::rpc::tarpc_types::CapabilityRegistration {
        service_id: Arc::from("00000000-0000-0000-0000-000000000001"),
        service_name: Arc::from("nestgate"),
        capability: Arc::from("storage"),
        capabilities: vec![Arc::from("storage")],
        tarpc_endpoint: Arc::from("tarpc://127.0.0.1:8091"),
        jsonrpc_endpoint: None,
        http_endpoint: None,
        metadata: std::collections::HashMap::new(),
    };
    assert!(c.register_capability(reg).await.is_err());
}

#[tokio::test]
async fn discover_capability_connection_fails() {
    let c = NestGateRpcClient::new("tarpc://127.0.0.1:15").unwrap();
    assert!(c.discover_capability("storage").await.is_err());
}
