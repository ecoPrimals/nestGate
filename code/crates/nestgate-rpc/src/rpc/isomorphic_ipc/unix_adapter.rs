// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! Unix Socket RPC Handler Adapter for isomorphic IPC.
//!
//! Uses an in-memory KV for `storage.*` JSON-RPC methods until nestgate-core storage is wired.

use anyhow::Result;
use nestgate_config::config::storage_paths::get_storage_base_path;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::debug;

use super::tcp_fallback::RpcHandler;

#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    #[expect(
        dead_code,
        reason = "jsonrpc field required by JSON-RPC wire format for deserialization"
    )]
    jsonrpc: Arc<str>,
    method: Arc<str>,
    params: Option<Value>,
    id: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: Arc<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// **Integration:** Match `nestgate_core::nat_traversal::BEACON_DATASET` when core is linked.
const BEACON_DATASET: &str = "_known_beacons";

#[derive(Clone)]
struct StorageState {
    /// `default` dataset: key → JSON bytes
    kv: Arc<tokio::sync::RwLock<HashMap<String, Vec<u8>>>>,
    #[expect(
        dead_code,
        reason = "Template storage wired when template RPC handlers are enabled"
    )]
    templates: crate::rpc::template_storage::TemplateStorage,
    #[expect(
        dead_code,
        reason = "Audit storage wired when audit RPC handlers are enabled"
    )]
    audits: crate::rpc::audit_storage::AuditStorage,
}

impl StorageState {
    fn new() -> Result<Self> {
        Ok(Self {
            kv: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
        })
    }
}

/// JSON-RPC handler that serves the isomorphic IPC Unix path with in-memory stub storage.
pub struct UnixSocketRpcHandler {
    state: Arc<StorageState>,
}

impl UnixSocketRpcHandler {
    /// Builds a handler with empty in-memory datasets and stub template/audit backends.
    pub fn new() -> Result<Self> {
        let state = Arc::new(StorageState::new()?);
        Ok(Self { state })
    }

    async fn handle_rpc_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!("📥 Processing JSON-RPC request: method={}", request.method);

        let id = request.id.clone();
        let result = match &*request.method {
            "storage.store" => self.handle_storage_store(&request).await,
            "storage.retrieve" => self.handle_storage_retrieve(&request).await,
            "storage.list" => self.handle_storage_list(&request),
            "storage.delete" => self.handle_storage_delete(&request).await,
            "storage.exists" => self.handle_storage_exists(&request).await,

            "health" | "health.check" => Ok(json!({
                "status": "healthy",
                "service": "nestgate",
                "version": env!("CARGO_PKG_VERSION"),
                "isomorphic": true
            })),
            "version" => Ok(json!({
                "version": env!("CARGO_PKG_VERSION"),
                "ipc": "isomorphic"
            })),
            "discover_capabilities" => Ok(json!({
                "primal": "nestgate",
                "version": env!("CARGO_PKG_VERSION"),
                "capabilities": [
                    "storage.store", "storage.retrieve", "storage.list",
                    "storage.delete", "storage.exists",
                    "nat.store_traversal_info", "nat.retrieve_traversal_info",
                    "beacon.store", "beacon.retrieve", "beacon.list", "beacon.delete",
                    "health", "health.check", "version", "discover_capabilities"
                ]
            })),

            "nat.store_traversal_info"
            | "nat.retrieve_traversal_info"
            | "beacon.store"
            | "beacon.retrieve"
            | "beacon.delete" => Err((
                -32603,
                Cow::Borrowed("wire cross-crate dep: nestgate-core nat_traversal + storage"),
            )),

            "beacon.list" => self.handle_beacon_list().await,

            _ => Err((
                -32601,
                Cow::Owned(format!("Method not found: {}", request.method)),
            )),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: Arc::from("2.0"),
                result: Some(value),
                error: None,
                id,
            },
            Err((code, message)) => JsonRpcResponse {
                jsonrpc: Arc::from("2.0"),
                result: None,
                error: Some(JsonRpcError {
                    code,
                    message,
                    data: None,
                }),
                id,
            },
        }
    }

    async fn handle_storage_store(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'key' parameter")))?;
        let value = params
            .get("value")
            .ok_or((-32602, Cow::Borrowed("Missing 'value' parameter")))?;

        let data = serde_json::to_vec(value)
            .map_err(|e| (-32603, Cow::Owned(format!("Serialization error: {e}"))))?;

        let mut g = self.state.kv.write().await;
        g.insert(key.to_string(), data);

        Ok(json!({"status": "stored", "key": key}))
    }

    #[allow(clippy::option_if_let_else)] // Match mirrors JSON success vs empty KV branch clearly.
    async fn handle_storage_retrieve(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'key' parameter")))?;

        let g = self.state.kv.read().await;
        match g.get(key) {
            Some(data) => {
                let value: Value = serde_json::from_slice(data)
                    .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(data).to_string()));
                Ok(json!({"value": value, "data": value, "key": key}))
            }
            None => Ok(json!({"value": null, "data": null, "key": key})),
        }
    }

    fn handle_storage_list(
        &self,
        _request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let _ = self;
        Ok(json!({"datasets": []}))
    }

    async fn handle_storage_delete(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'key' parameter")))?;

        let mut g = self.state.kv.write().await;
        g.remove(key);
        Ok(json!({"status": "deleted", "key": key}))
    }

    async fn handle_storage_exists(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let key = params
            .get("key")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'key' parameter")))?;

        let g = self.state.kv.read().await;
        let exists = g.contains_key(key);
        Ok(json!({"exists": exists, "key": key}))
    }

    async fn handle_beacon_list(&self) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let _ = self;
        debug!("feature pending: beacon dataset name alignment with nestgate-core nat_traversal");
        let dataset_path = get_storage_base_path()
            .join("datasets")
            .join(BEACON_DATASET);

        let mut peer_ids: Vec<String> = Vec::new();
        if dataset_path.exists() {
            let mut entries = tokio::fs::read_dir(&dataset_path).await.map_err(|e| {
                (
                    -32603,
                    Cow::Owned(format!("Failed to read beacon dataset: {e}")),
                )
            })?;
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(name) = entry.file_name().to_str()
                    && !name.starts_with('.')
                {
                    peer_ids.push(name.to_string());
                }
            }
        }
        peer_ids.sort();
        let count = peer_ids.len();
        Ok(json!({ "peer_ids": peer_ids, "count": count }))
    }
}

impl RpcHandler for UnixSocketRpcHandler {
    fn handle_request(&self, request: Value) -> Pin<Box<dyn Future<Output = Value> + Send + '_>> {
        Box::pin(async move {
            match serde_json::from_value::<JsonRpcRequest>(request) {
                Ok(rpc_request) => {
                    let response = self.handle_rpc_request(rpc_request).await;
                    serde_json::to_value(response).unwrap_or_else(|e| {
                        json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": -32603,
                                "message": format!("Internal error: {}", e)
                            },
                            "id": null
                        })
                    })
                }
                Err(e) => {
                    json!({
                        "jsonrpc": "2.0",
                        "error": {
                            "code": -32700,
                            "message": format!("Parse error: {}", e)
                        },
                        "id": null
                    })
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_handler_creation() {
        let handler = UnixSocketRpcHandler::new();
        assert!(handler.is_ok());
    }

    #[tokio::test]
    async fn test_health_request() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "health",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["result"]["status"] == "healthy");
        assert!(response["result"]["isomorphic"] == true);
    }

    #[tokio::test]
    async fn test_version_request() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "version",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["result"]["version"].is_string());
        assert!(response["result"]["ipc"] == "isomorphic");
    }

    #[tokio::test]
    async fn test_unknown_method() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "unknown.method",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["error"].is_object());
        assert!(response["error"]["code"] == -32601);
    }

    #[tokio::test]
    async fn test_invalid_request() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "not": "valid"
        });

        let response = handler.handle_request(request).await;

        assert!(response["error"].is_object());
        assert!(response["error"]["code"] == -32700);
    }

    #[tokio::test]
    async fn test_discover_capabilities_request() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "discover_capabilities",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["result"]["primal"] == "nestgate");
        assert!(response["result"]["capabilities"].is_array());
    }

    #[tokio::test]
    async fn test_storage_store_request() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "storage.store",
            "params": {"key": "test-key-store", "value": {"data": "value"}},
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["result"]["status"] == "stored");
        assert!(response["result"]["key"] == "test-key-store");
    }

    #[tokio::test]
    async fn test_storage_store_missing_params() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "storage.store",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["error"].is_object());
        assert!(response["error"]["code"] == -32602);
    }

    #[tokio::test]
    async fn test_storage_retrieve_missing_key() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "storage.retrieve",
            "params": {},
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["error"].is_object());
    }

    #[tokio::test]
    async fn test_storage_list_request() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "storage.list",
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["result"]["datasets"].is_array());
    }

    #[tokio::test]
    async fn test_storage_exists_request() {
        let handler = UnixSocketRpcHandler::new().unwrap();

        let request = json!({
            "jsonrpc": "2.0",
            "method": "storage.exists",
            "params": {"key": "nonexistent-key-12345"},
            "id": 1
        });

        let response = handler.handle_request(request).await;

        assert!(response["result"]["exists"] == false);
    }

    #[tokio::test]
    async fn test_storage_store_retrieve_roundtrip() {
        let handler = UnixSocketRpcHandler::new().unwrap();
        let key = format!("roundtrip-{}", uuid::Uuid::new_v4());

        let store_request = json!({
            "jsonrpc": "2.0",
            "method": "storage.store",
            "params": {"key": key, "value": {"test": "data"}},
            "id": 1
        });
        let store_response = handler.handle_request(store_request).await;
        assert!(store_response["result"]["status"] == "stored");

        let retrieve_request = json!({
            "jsonrpc": "2.0",
            "method": "storage.retrieve",
            "params": {"key": key},
            "id": 2
        });
        let retrieve_response = handler.handle_request(retrieve_request).await;
        assert!(retrieve_response["result"]["value"]["test"] == "data");
    }
}
