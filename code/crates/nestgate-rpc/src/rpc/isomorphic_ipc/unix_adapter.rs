// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unix Socket RPC Handler Adapter for isomorphic IPC.
//!
//! `storage.*` methods are backed by the filesystem under `get_storage_base_path()/datasets/`.
//! Each key is a file; values are JSON bytes on disk. This is `NestGate`'s actual storage backend.

use anyhow::Result;
use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::borrow::Cow;
use std::future::Future;
use std::path::PathBuf;
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

/// Match `nestgate_core::nat_traversal::BEACON_DATASET` when core is linked.
const BEACON_DATASET: &str = "_known_beacons";

/// Filesystem-backed storage state.
///
/// Keys are stored as individual JSON files under `{base}/datasets/default/{key}.json`.
/// This replaces the former in-memory `HashMap` so IPC storage survives restarts.
#[derive(Clone)]
struct StorageState {
    /// Root directory for the default dataset.
    dataset_dir: PathBuf,
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
        let dataset_dir = get_storage_base_path().join("datasets").join("default");
        std::fs::create_dir_all(&dataset_dir)?;
        Ok(Self {
            dataset_dir,
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
        })
    }

    /// Sanitize a key to a safe filename (reject path traversal).
    fn key_path(&self, key: &str) -> std::result::Result<PathBuf, (i32, Cow<'static, str>)> {
        if key.is_empty()
            || key.contains('/')
            || key.contains('\\')
            || key.contains("..")
            || key.starts_with('.')
        {
            return Err((-32602, Cow::Borrowed("Invalid key: must be a simple name")));
        }
        Ok(self.dataset_dir.join(format!("{key}.json")))
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
            "storage.store" | "data.store" => self.handle_storage_store(&request).await,
            "storage.retrieve" | "data.retrieve" => self.handle_storage_retrieve(&request).await,
            "storage.list" => self.handle_storage_list(&request).await,
            "storage.delete" => self.handle_storage_delete(&request).await,
            "storage.exists" => self.handle_storage_exists(&request),

            "health" | "health.check" => Ok(json!({
                "status": "healthy",
                "service": DEFAULT_SERVICE_NAME,
                "version": env!("CARGO_PKG_VERSION"),
                "isomorphic": true
            })),
            "health.liveness" => Ok(json!({"alive": true})),
            "health.readiness" => self.handle_health_readiness().await,

            "version" => Ok(json!({
                "version": env!("CARGO_PKG_VERSION"),
                "ipc": "isomorphic"
            })),
            "capabilities.list" | "discover_capabilities" => Ok(json!({
                "primal": DEFAULT_SERVICE_NAME,
                "version": env!("CARGO_PKG_VERSION"),
                "domain": "storage",
                "capabilities": [
                    "storage.store", "storage.retrieve", "storage.list",
                    "storage.delete", "storage.exists",
                    "nat.store_traversal_info", "nat.retrieve_traversal_info",
                    "beacon.store", "beacon.retrieve", "beacon.list", "beacon.delete",
                    "data.store", "data.retrieve",
                    "health", "health.check", "health.liveness", "health.readiness",
                    "capabilities.list", "version"
                ]
            })),

            // nat.* — NAT traversal info uses its own sub-directory
            "nat.store_traversal_info" => self.handle_nat_store(&request).await,
            "nat.retrieve_traversal_info" => self.handle_nat_retrieve(&request).await,

            "beacon.store" => self.handle_beacon_store(&request).await,
            "beacon.retrieve" => self.handle_beacon_retrieve(&request).await,
            "beacon.delete" => self.handle_beacon_delete(&request).await,
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

        let path = self.state.key_path(key)?;
        let data = serde_json::to_vec_pretty(value)
            .map_err(|e| (-32603, Cow::Owned(format!("Serialization error: {e}"))))?;

        tokio::fs::write(&path, &data)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("Storage write error: {e}"))))?;

        Ok(json!({"status": "stored", "key": key}))
    }

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

        let path = self.state.key_path(key)?;
        if !path.exists() {
            return Ok(json!({"value": null, "data": null, "key": key}));
        }

        let data = tokio::fs::read(&path)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("Storage read error: {e}"))))?;
        let value: Value = serde_json::from_slice(&data)
            .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&data).to_string()));
        Ok(json!({"value": value, "data": value, "key": key}))
    }

    async fn handle_storage_list(
        &self,
        _request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let dir = &self.state.dataset_dir;
        let mut keys = Vec::new();
        if dir.exists() {
            let mut entries = tokio::fs::read_dir(dir)
                .await
                .map_err(|e| (-32603, Cow::Owned(format!("Storage list error: {e}"))))?;
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Some(key) = entry
                    .file_name()
                    .to_str()
                    .and_then(|n| n.strip_suffix(".json"))
                {
                    keys.push(key.to_string());
                }
            }
        }
        keys.sort();
        Ok(json!({"datasets": ["default"], "keys": keys, "count": keys.len()}))
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

        let path = self.state.key_path(key)?;
        if path.exists() {
            tokio::fs::remove_file(&path)
                .await
                .map_err(|e| (-32603, Cow::Owned(format!("Storage delete error: {e}"))))?;
        }
        Ok(json!({"status": "deleted", "key": key}))
    }

    fn handle_storage_exists(
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

        let path = self.state.key_path(key)?;
        Ok(json!({"exists": path.exists(), "key": key}))
    }

    async fn handle_health_readiness(
        &self,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let dir_ok = self.state.dataset_dir.exists()
            && tokio::fs::metadata(&self.state.dataset_dir)
                .await
                .map(|m| m.is_dir())
                .unwrap_or(false);
        Ok(json!({
            "ready": dir_ok,
            "storage_path": self.state.dataset_dir.display().to_string(),
        }))
    }

    /// NAT traversal info is stored under the `_nat` sub-directory.
    fn nat_dir(&self) -> PathBuf {
        self.state
            .dataset_dir
            .parent()
            .unwrap_or(&self.state.dataset_dir)
            .join("_nat")
    }

    async fn handle_nat_store(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let peer_id = params
            .get("peer_id")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;
        let info = params
            .get("info")
            .ok_or((-32602, Cow::Borrowed("Missing 'info'")))?;

        let dir = self.nat_dir();
        tokio::fs::create_dir_all(&dir)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("mkdir: {e}"))))?;
        let path = dir.join(format!("{peer_id}.json"));
        let data = serde_json::to_vec_pretty(info)
            .map_err(|e| (-32603, Cow::Owned(format!("json: {e}"))))?;
        tokio::fs::write(&path, &data)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("write: {e}"))))?;
        Ok(json!({"status": "stored", "peer_id": peer_id}))
    }

    async fn handle_nat_retrieve(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let peer_id = params
            .get("peer_id")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;

        let path = self.nat_dir().join(format!("{peer_id}.json"));
        if !path.exists() {
            return Ok(json!({"info": null, "peer_id": peer_id}));
        }
        let data = tokio::fs::read(&path)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("read: {e}"))))?;
        let info: Value = serde_json::from_slice(&data).unwrap_or(Value::Null);
        Ok(json!({"info": info, "peer_id": peer_id}))
    }

    fn beacon_dir(&self) -> PathBuf {
        self.state
            .dataset_dir
            .parent()
            .unwrap_or(&self.state.dataset_dir)
            .join(BEACON_DATASET)
    }

    async fn handle_beacon_store(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let peer_id = params
            .get("peer_id")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;
        let beacon = params
            .get("beacon")
            .ok_or((-32602, Cow::Borrowed("Missing 'beacon'")))?;

        let dir = self.beacon_dir();
        tokio::fs::create_dir_all(&dir)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("mkdir: {e}"))))?;
        let path = dir.join(format!("{peer_id}.json"));
        let data = serde_json::to_vec_pretty(beacon)
            .map_err(|e| (-32603, Cow::Owned(format!("json: {e}"))))?;
        tokio::fs::write(&path, &data)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("write: {e}"))))?;
        Ok(json!({"status": "stored", "peer_id": peer_id}))
    }

    async fn handle_beacon_retrieve(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let peer_id = params
            .get("peer_id")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;

        let path = self.beacon_dir().join(format!("{peer_id}.json"));
        if !path.exists() {
            return Ok(json!({"beacon": null, "peer_id": peer_id}));
        }
        let data = tokio::fs::read(&path)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("read: {e}"))))?;
        let beacon: Value = serde_json::from_slice(&data).unwrap_or(Value::Null);
        Ok(json!({"beacon": beacon, "peer_id": peer_id}))
    }

    async fn handle_beacon_delete(
        &self,
        request: &JsonRpcRequest,
    ) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let params = request
            .params
            .as_ref()
            .ok_or((-32602, Cow::Borrowed("Missing params")))?;
        let peer_id = params
            .get("peer_id")
            .and_then(|v| v.as_str())
            .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;

        let path = self.beacon_dir().join(format!("{peer_id}.json"));
        if path.exists() {
            tokio::fs::remove_file(&path)
                .await
                .map_err(|e| (-32603, Cow::Owned(format!("delete: {e}"))))?;
        }
        Ok(json!({"status": "deleted", "peer_id": peer_id}))
    }

    async fn handle_beacon_list(&self) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
        let dataset_path = self.beacon_dir();

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

    fn test_handler() -> UnixSocketRpcHandler {
        UnixSocketRpcHandler::new().expect("handler creation should succeed")
    }

    #[tokio::test]
    async fn test_handler_creation() {
        assert!(UnixSocketRpcHandler::new().is_ok());
    }

    #[tokio::test]
    async fn test_health_request() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"health","id":1}))
            .await;
        assert_eq!(r["result"]["status"], "healthy");
        assert_eq!(r["result"]["isomorphic"], true);
    }

    #[tokio::test]
    async fn test_health_liveness() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"health.liveness","id":1}))
            .await;
        assert_eq!(r["result"]["alive"], true);
    }

    #[tokio::test]
    async fn test_health_readiness() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"health.readiness","id":1}))
            .await;
        assert_eq!(r["result"]["ready"], true);
        assert!(r["result"]["storage_path"].is_string());
    }

    #[tokio::test]
    async fn test_capabilities_list() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"capabilities.list","id":1}))
            .await;
        assert_eq!(r["result"]["primal"], "nestgate");
        assert_eq!(r["result"]["domain"], "storage");
        let caps = r["result"]["capabilities"].as_array().unwrap();
        assert!(caps.iter().any(|c| c == "health.liveness"));
        assert!(caps.iter().any(|c| c == "health.readiness"));
        assert!(caps.iter().any(|c| c == "data.store"));
    }

    #[tokio::test]
    async fn test_version_request() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"version","id":1}))
            .await;
        assert!(r["result"]["version"].is_string());
        assert_eq!(r["result"]["ipc"], "isomorphic");
    }

    #[tokio::test]
    async fn test_unknown_method() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"unknown.method","id":1}))
            .await;
        assert_eq!(r["error"]["code"], -32601);
    }

    #[tokio::test]
    async fn test_invalid_request() {
        let h = test_handler();
        let r = h.handle_request(json!({"not": "valid"})).await;
        assert_eq!(r["error"]["code"], -32700);
    }

    #[tokio::test]
    async fn test_storage_store_retrieve_roundtrip() {
        let h = test_handler();
        let key = format!("rt-{}", uuid::Uuid::new_v4());

        let store = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"storage.store",
                "params":{"key": key, "value":{"test":"data"}}, "id":1
            }))
            .await;
        assert_eq!(store["result"]["status"], "stored");

        let retrieve = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"storage.retrieve",
                "params":{"key": key}, "id":2
            }))
            .await;
        assert_eq!(retrieve["result"]["value"]["test"], "data");

        let exists = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"storage.exists",
                "params":{"key": key}, "id":3
            }))
            .await;
        assert_eq!(exists["result"]["exists"], true);

        let delete = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"storage.delete",
                "params":{"key": key}, "id":4
            }))
            .await;
        assert_eq!(delete["result"]["status"], "deleted");

        let gone = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"storage.exists",
                "params":{"key": key}, "id":5
            }))
            .await;
        assert_eq!(gone["result"]["exists"], false);
    }

    #[tokio::test]
    async fn test_data_store_retrieve_alias() {
        let h = test_handler();
        let key = format!("data-{}", uuid::Uuid::new_v4());

        let store = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"data.store",
                "params":{"key": key, "value":"hello"}, "id":1
            }))
            .await;
        assert_eq!(store["result"]["status"], "stored");

        let retrieve = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"data.retrieve",
                "params":{"key": key}, "id":2
            }))
            .await;
        assert_eq!(retrieve["result"]["value"], "hello");
    }

    #[tokio::test]
    async fn test_storage_list_returns_keys() {
        let h = test_handler();
        let key = format!("list-{}", uuid::Uuid::new_v4());

        h.handle_request(json!({
            "jsonrpc":"2.0","method":"storage.store",
            "params":{"key": key, "value": 42}, "id":1
        }))
        .await;

        let list = h
            .handle_request(json!({"jsonrpc":"2.0","method":"storage.list","id":2}))
            .await;
        assert!(list["result"]["datasets"].is_array());
        let keys = list["result"]["keys"].as_array().unwrap();
        assert!(keys.iter().any(|k| k.as_str() == Some(&key)));
    }

    #[tokio::test]
    async fn test_storage_store_missing_params() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"storage.store","id":1}))
            .await;
        assert_eq!(r["error"]["code"], -32602);
    }

    #[tokio::test]
    async fn test_storage_retrieve_missing_key() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"storage.retrieve","params":{},"id":1}))
            .await;
        assert!(r["error"].is_object());
    }

    #[tokio::test]
    async fn test_nat_store_retrieve_roundtrip() {
        let h = test_handler();
        let peer = format!("peer-{}", uuid::Uuid::new_v4());

        let store = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"nat.store_traversal_info",
                "params":{"peer_id": peer, "info":{"endpoint":"1.2.3.4:9000"}}, "id":1
            }))
            .await;
        assert_eq!(store["result"]["status"], "stored");

        let retrieve = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"nat.retrieve_traversal_info",
                "params":{"peer_id": peer}, "id":2
            }))
            .await;
        assert_eq!(retrieve["result"]["info"]["endpoint"], "1.2.3.4:9000");
    }

    #[tokio::test]
    async fn test_beacon_crud() {
        let h = test_handler();
        let peer = format!("beacon-{}", uuid::Uuid::new_v4());

        let store = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"beacon.store",
                "params":{"peer_id": peer, "beacon":{"ts":12345}}, "id":1
            }))
            .await;
        assert_eq!(store["result"]["status"], "stored");

        let retrieve = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"beacon.retrieve",
                "params":{"peer_id": peer}, "id":2
            }))
            .await;
        assert_eq!(retrieve["result"]["beacon"]["ts"], 12345);

        let delete = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"beacon.delete",
                "params":{"peer_id": peer}, "id":3
            }))
            .await;
        assert_eq!(delete["result"]["status"], "deleted");

        let list = h
            .handle_request(json!({"jsonrpc":"2.0","method":"beacon.list","id":4}))
            .await;
        let ids = list["result"]["peer_ids"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>();
        assert!(!ids.iter().any(|id| id.contains(&peer)));
    }

    #[tokio::test]
    async fn test_key_path_traversal_rejected() {
        let h = test_handler();
        let r = h
            .handle_request(json!({
                "jsonrpc":"2.0","method":"storage.store",
                "params":{"key": "../etc/passwd", "value": "bad"}, "id":1
            }))
            .await;
        assert_eq!(r["error"]["code"], -32602);
    }

    #[tokio::test]
    async fn test_discover_capabilities_legacy_method() {
        let h = test_handler();
        let r = h
            .handle_request(json!({"jsonrpc":"2.0","method":"discover_capabilities","id":1}))
            .await;
        assert_eq!(r["result"]["primal"], "nestgate");
        assert!(r["result"]["capabilities"].is_array());
    }
}
