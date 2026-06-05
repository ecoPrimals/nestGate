// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for [`super::handlers::NestGateRpcHandler`] — storage backend dispatch,
//! identity/health/system introspection, and content pipeline routing.

use std::future::Future;
use std::sync::Arc;

use nestgate_types::error::Result;
use serde_json::Value;

use super::handlers::{NestGateRpcHandler, NoopStorage, StorageBackend};

struct MockStorage;

impl StorageBackend for MockStorage {
    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn store(&self, _key: &str, _value: &[u8]) -> impl Future<Output = Result<()>> + Send + '_ {
        async { Ok(()) }
    }

    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn retrieve(&self, key: &str) -> impl Future<Output = Result<Vec<u8>>> + Send + '_ {
        let key = key.to_owned();
        async move { Ok(format!("mock_value_{key}").into_bytes()) }
    }

    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn delete(&self, _key: &str) -> impl Future<Output = Result<()>> + Send + '_ {
        async { Ok(()) }
    }

    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn list(
        &self,
        _prefix: &Option<String>,
    ) -> impl Future<Output = Result<Vec<String>>> + Send + '_ {
        async { Ok(vec![String::from("key1"), String::from("key2")]) }
    }
}

#[test]
fn test_ping() {
    let handler = NestGateRpcHandler::new();
    let result = handler.handle_ping(Value::Null);
    assert!(result.is_ok());
}

#[test]
fn test_identity() {
    let handler = NestGateRpcHandler::new();
    let result = handler.handle_identity(Value::Null);
    assert!(result.is_ok());
    let identity = result.unwrap();
    assert_eq!(identity["primal"], "nestgate");
    assert_eq!(identity["domain"], "storage");
    assert_eq!(identity["license"], "AGPL-3.0-or-later");
    assert!(identity.get("version").is_some());
    assert!(identity.get("family_id").is_some());
}

#[test]
fn test_capabilities() {
    let handler = NestGateRpcHandler::new();
    let result = handler.handle_capabilities(Value::Null);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_store_without_backend() {
    let handler = NestGateRpcHandler::new();
    let params = serde_json::json!({"key": "test", "value": [1, 2, 3]});
    let result = handler.handle_store(params).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_store_with_backend() {
    let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
    let params = serde_json::json!({"key": "test", "value": [1, 2, 3]});
    let result = handler.handle_store(params).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_retrieve_invalid_params() {
    let handler = NestGateRpcHandler::new();
    let result = handler
        .handle_retrieve(serde_json::json!("not_object"))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_retrieve_without_backend() {
    let handler = NestGateRpcHandler::new();
    let result = handler
        .handle_retrieve(serde_json::json!({"key": "k"}))
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_retrieve_with_backend_ok() {
    let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
    let result = handler
        .handle_retrieve(serde_json::json!({"key": "k"}))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_invalid_params() {
    let handler = NestGateRpcHandler::new();
    assert!(handler.handle_delete(serde_json::json!([])).await.is_err());
}

#[tokio::test]
async fn test_delete_without_backend() {
    let handler = NestGateRpcHandler::new();
    assert!(
        handler
            .handle_delete(serde_json::json!({"key": "k"}))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_delete_with_backend_ok() {
    let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
    assert!(
        handler
            .handle_delete(serde_json::json!({"key": "k"}))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn test_list_invalid_params_type() {
    let handler = NestGateRpcHandler::new();
    assert!(handler.handle_list(serde_json::json!("x")).await.is_err());
}

#[tokio::test]
async fn test_list_without_backend() {
    let handler = NestGateRpcHandler::new();
    assert!(handler.handle_list(serde_json::json!({})).await.is_err());
}

#[tokio::test]
async fn test_list_with_backend_ok() {
    let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
    assert!(
        handler
            .handle_list(serde_json::json!({"prefix": "p"}))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn test_store_invalid_params() {
    let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
    assert!(
        handler
            .handle_store(serde_json::json!({"key": 1}))
            .await
            .is_err()
    );
}

#[test]
fn test_system_info_ok() {
    let handler = NestGateRpcHandler::new();
    let v = handler.handle_system_info(serde_json::Value::Null);
    assert!(v.is_ok());
}

#[tokio::test]
async fn test_handle_method_dispatch_branches() {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler = NestGateRpcHandler::new();
    assert!(
        handler
            .handle_method("storage.list", serde_json::json!({}))
            .await
            .is_err()
    );
    assert!(
        handler
            .handle_method("health.ping", serde_json::json!({}))
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn dispatch_unknown_method_rejected() {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler = NestGateRpcHandler::new();
    let err = handler
        .handle_method("unknown.method", serde_json::json!({}))
        .await
        .expect_err("unknown RPC method should error");
    assert!(
        err.to_string().contains("Unknown method"),
        "unexpected: {err}"
    );
}

#[tokio::test]
async fn dispatch_health_status() {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler = NestGateRpcHandler::new();
    let v = handler
        .handle_method("health.status", serde_json::json!({}))
        .await
        .expect("health.status should succeed");
    assert_eq!(v["status"], "healthy");
    assert_eq!(v["protocol"], "jsonrpc-2.0");
}

#[tokio::test]
async fn dispatch_identity_get() {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler = NestGateRpcHandler::new();
    let v = handler
        .handle_method("identity.get", serde_json::json!({}))
        .await
        .expect("identity.get");
    assert_eq!(v["domain"], "storage");
    assert_eq!(v["license"], "AGPL-3.0-or-later");
}

#[tokio::test]
async fn dispatch_identity_capabilities() {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler = NestGateRpcHandler::new();
    let v = handler
        .handle_method("identity.capabilities", serde_json::json!({}))
        .await
        .expect("identity.capabilities");
    assert_eq!(v["zfs"], true);
    assert_eq!(v["storage"], false);
}

#[tokio::test]
async fn dispatch_system_info() {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler = NestGateRpcHandler::new();
    let v = handler
        .handle_method("system.info", serde_json::json!({}))
        .await
        .expect("system.info");
    assert_eq!(v["protocol"], "jsonrpc-2.0");
    assert!(v.get("rust_version").is_some());
}

#[tokio::test]
async fn dispatch_storage_retrieve_param_validation() {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
    let err = handler
        .handle_method("storage.retrieve", serde_json::json!({"unexpected": true}))
        .await
        .expect_err("missing key field");
    assert!(
        err.to_string().contains("Invalid params") || err.to_string().contains("invalid"),
        "{err}"
    );
}

#[tokio::test]
async fn dispatch_storage_store_via_handle_method() {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler = NestGateRpcHandler::with_storage(Arc::new(MockStorage));
    let v = handler
        .handle_method(
            "storage.store",
            serde_json::json!({"key": "k", "value": [1]}),
        )
        .await
        .expect("storage.store");
    assert_eq!(v["success"], true);
}

// ── Content pipeline transport dispatch tests ──────────────────

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serial_test::serial;

async fn cleanup_test_family(fam: &str) {
    let _ = tokio::fs::remove_dir_all(
        nestgate_core::config::storage_paths::get_storage_base_path()
            .join("datasets")
            .join(fam),
    )
    .await;
}

async fn content_dispatch(method: &str, params: serde_json::Value) -> Result<Value> {
    use crate::transport::jsonrpc::RpcMethodHandler;
    let handler: NestGateRpcHandler<NoopStorage> = NestGateRpcHandler::new();
    handler.handle_method(method, params).await
}

#[tokio::test]
#[serial]
async fn transport_content_put_get_roundtrip() {
    let fam = format!("t-content-rt-{}", uuid::Uuid::new_v4());
    let data = STANDARD.encode(b"transport content test");

    let put = content_dispatch("content.put", serde_json::json!({"data": data, "family_id": &fam}))
        .await
        .expect("content.put");
    let hash = put["hash"].as_str().unwrap().to_owned();
    assert!(put["stored"].as_bool().unwrap());

    let get = content_dispatch("content.get", serde_json::json!({"hash": &hash, "family_id": &fam}))
        .await
        .expect("content.get");
    assert_eq!(get["hash"], hash);

    cleanup_test_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn transport_content_exists_dispatch() {
    let fam = format!("t-content-ex-{}", uuid::Uuid::new_v4());
    let data = STANDARD.encode(b"exists via transport");

    let put = content_dispatch("content.put", serde_json::json!({"data": data, "family_id": &fam}))
        .await
        .unwrap();
    let hash = put["hash"].as_str().unwrap();

    let exists = content_dispatch("content.exists", serde_json::json!({"hash": hash, "family_id": &fam}))
        .await
        .unwrap();
    assert_eq!(exists["exists"], true);

    cleanup_test_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn transport_content_list_dispatch() {
    let fam = format!("t-content-ls-{}", uuid::Uuid::new_v4());
    let v = content_dispatch("content.list", serde_json::json!({"family_id": &fam}))
        .await
        .unwrap();
    assert!(v["hashes"].is_array());

    cleanup_test_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn transport_content_collections_dispatch() {
    let fam = format!("t-content-col-{}", uuid::Uuid::new_v4());
    let v = content_dispatch("content.collections", serde_json::json!({"family_id": &fam}))
        .await
        .unwrap();
    assert!(v["collections"].is_array());

    cleanup_test_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn transport_content_publish_resolve_dispatch() {
    let fam = format!("t-content-pub-{}", uuid::Uuid::new_v4());
    let data = STANDARD.encode(b"publish via transport");

    let put = content_dispatch("content.put", serde_json::json!({"data": data, "family_id": &fam}))
        .await
        .unwrap();
    let hash = put["hash"].as_str().unwrap().to_owned();

    let pub_res = content_dispatch(
        "content.publish",
        serde_json::json!({"collection": "t-site", "manifest": {"/page": hash}, "family_id": &fam}),
    )
    .await
    .unwrap();
    assert!(pub_res["stored"].as_bool().unwrap());

    let resolve = content_dispatch(
        "content.resolve",
        serde_json::json!({"collection": "t-site", "path": "/page", "family_id": &fam}),
    )
    .await
    .unwrap();
    assert_eq!(resolve["hash"], hash);

    cleanup_test_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn transport_content_store_stream_dispatch() {
    let fam = format!("t-content-ss-{}", uuid::Uuid::new_v4());
    let v = content_dispatch(
        "content.store_stream",
        serde_json::json!({"total_size": 128, "family_id": &fam}),
    )
    .await
    .unwrap();
    assert!(v["stream_id"].is_string());

    cleanup_test_family(&fam).await;
}

#[tokio::test]
#[serial]
async fn transport_content_replicate_pull_dispatch() {
    let fam = format!("t-content-rp-{}", uuid::Uuid::new_v4());
    let data = STANDARD.encode(b"replicate pull via transport");

    let put = content_dispatch("content.put", serde_json::json!({"data": data, "family_id": &fam}))
        .await
        .unwrap();
    let cid = put["hash"].as_str().unwrap();

    let pull = content_dispatch(
        "content.replicate.pull",
        serde_json::json!({"cids": [cid], "source": "/nope.sock", "family_id": &fam}),
    )
    .await
    .unwrap();
    assert_eq!(pull["skipped_count"], 1);

    cleanup_test_family(&fam).await;
}

#[tokio::test]
async fn transport_lifecycle_status_dispatch() {
    let v = content_dispatch("lifecycle.status", serde_json::json!({}))
        .await
        .unwrap();
    assert_eq!(v["status"], "running");
}
