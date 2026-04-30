// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unix Socket RPC Handler Adapter for isomorphic IPC.
//!
//! `storage.*` methods are backed by the filesystem under `get_storage_base_path()/datasets/`.
//! Each key is a file; values are JSON bytes on disk. This is `NestGate`'s actual storage backend.

mod unix_adapter_handlers;

#[cfg(test)]
mod tests;

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

use crate::rpc::protocol::normalize_method;

use super::tcp_fallback::RpcHandler;

#[derive(Debug, Deserialize)]
pub(super) struct JsonRpcRequest {
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

/// Default namespace for cross-spring shared storage.
const DEFAULT_NAMESPACE: &str = "shared";

/// Filesystem-backed storage state with family-scoped namespace isolation.
///
/// Directory layout:
/// ```text
/// {base}/datasets/{family_id}/{namespace}/{key}.json   (JSON values)
/// {base}/datasets/{family_id}/{namespace}/_blobs/{key}  (binary blobs)
/// ```
///
/// - `family_id` is resolved from env (`NESTGATE_FAMILY_ID` / `FAMILY_ID` / `BIOMEOS_FAMILY_ID`),
///   defaulting to `"default"`.
/// - `namespace` isolates each caller (spring). The `"shared"` namespace is the cross-spring
///   meeting point, readable/writable by all springs in the family. Omitting `namespace` from
///   request params defaults to `"shared"` for backward compatibility.
#[derive(Clone)]
pub(super) struct StorageState {
    /// Root directory for this family: `{base}/datasets/{family_id}`.
    family_dir: PathBuf,
    /// The resolved family identifier.
    family_id: String,
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

/// Resolve the family identifier from environment, with cascading fallback.
fn resolve_family_id() -> String {
    std::env::var("NESTGATE_FAMILY_ID")
        .or_else(|_| std::env::var("FAMILY_ID"))
        .or_else(|_| std::env::var("BIOMEOS_FAMILY_ID"))
        .unwrap_or_else(|_| "default".to_string())
}

impl StorageState {
    fn new() -> Result<Self> {
        let family_id = resolve_family_id();
        let family_dir = get_storage_base_path().join("datasets").join(&family_id);
        let shared_dir = family_dir.join(DEFAULT_NAMESPACE);
        std::fs::create_dir_all(&shared_dir)?;
        Ok(Self {
            family_dir,
            family_id,
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
        })
    }

    /// Validate a name segment (key or namespace) — reject path traversal.
    fn validate_segment(
        name: &str,
        field: &'static str,
    ) -> std::result::Result<(), (i32, Cow<'static, str>)> {
        if name.is_empty()
            || name.contains('/')
            || name.contains('\\')
            || name.contains("..")
            || name.starts_with('.')
        {
            return Err((
                -32602,
                Cow::Owned(format!("Invalid {field}: must be a simple name")),
            ));
        }
        Ok(())
    }

    /// Resolve a namespace directory, creating it on first access.
    fn namespace_dir(&self, namespace: &str) -> PathBuf {
        self.family_dir.join(namespace)
    }

    /// Sanitize a key to a safe filename within a namespace.
    fn key_path(
        &self,
        namespace: &str,
        key: &str,
    ) -> std::result::Result<PathBuf, (i32, Cow<'static, str>)> {
        Self::validate_segment(namespace, "namespace")?;
        Self::validate_segment(key, "key")?;
        Ok(self.namespace_dir(namespace).join(format!("{key}.json")))
    }

    /// Blob storage path within a namespace.
    fn blob_path(
        &self,
        namespace: &str,
        key: &str,
    ) -> std::result::Result<PathBuf, (i32, Cow<'static, str>)> {
        Self::validate_segment(namespace, "namespace")?;
        Self::validate_segment(key, "key")?;
        Ok(self.namespace_dir(namespace).join("_blobs").join(key))
    }

    /// NAT traversal info is stored under the `_nat` sub-directory.
    fn nat_dir(&self) -> PathBuf {
        self.family_dir.join("_nat")
    }

    fn beacon_dir(&self) -> PathBuf {
        self.family_dir.join(BEACON_DATASET)
    }
}

/// JSON-RPC handler that serves the isomorphic IPC Unix path with filesystem-backed storage.
pub struct UnixSocketRpcHandler {
    state: Arc<StorageState>,
}

impl UnixSocketRpcHandler {
    /// Builds a handler with an on-disk default dataset and production template/audit storage handles.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the default dataset directory under the configured storage base
    /// cannot be created (for example missing permissions or I/O failure from
    /// [`std::fs::create_dir_all`]).
    pub fn new() -> Result<Self> {
        let state = Arc::new(StorageState::new()?);
        Ok(Self { state })
    }

    async fn handle_rpc_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Processing JSON-RPC request: method={}", request.method);

        let id = request.id.clone();
        let state = self.state.as_ref();
        let method = normalize_method(&request.method);
        let result = match method.as_ref() {
            "storage.store" => unix_adapter_handlers::handle_storage_store(state, &request).await,
            "storage.retrieve" => {
                unix_adapter_handlers::handle_storage_retrieve(state, &request).await
            }
            "storage.list" => unix_adapter_handlers::handle_storage_list(state, &request).await,
            "storage.delete" => unix_adapter_handlers::handle_storage_delete(state, &request).await,
            "storage.exists" => unix_adapter_handlers::handle_storage_exists(state, &request),
            "storage.store_blob" => {
                unix_adapter_handlers::handle_storage_store_blob(state, &request).await
            }
            "storage.retrieve_blob" => {
                unix_adapter_handlers::handle_storage_retrieve_blob(state, &request).await
            }
            "storage.retrieve_range" => {
                unix_adapter_handlers::handle_storage_retrieve_range(state, &request).await
            }
            "storage.store_stream" => {
                unix_adapter_handlers::handle_storage_store_stream(state, &request).await
            }
            "storage.store_stream_chunk" => {
                unix_adapter_handlers::handle_storage_store_stream_chunk(&request).await
            }
            "storage.retrieve_stream" => {
                unix_adapter_handlers::handle_storage_retrieve_stream(state, &request).await
            }
            "storage.retrieve_stream_chunk" => {
                unix_adapter_handlers::handle_storage_retrieve_stream_chunk(&request).await
            }
            "storage.object.size" => {
                unix_adapter_handlers::handle_storage_object_size(state, &request).await
            }
            "storage.namespaces.list" => {
                unix_adapter_handlers::handle_storage_namespaces_list(state).await
            }
            "storage.fetch_external" => {
                unix_adapter_handlers::handle_storage_fetch_external(state, &request).await
            }
            "session.save" => unix_adapter_handlers::handle_session_save(state, &request).await,
            "session.load" => unix_adapter_handlers::handle_session_load(state, &request).await,
            "session.list" => unix_adapter_handlers::handle_session_list(state, &request).await,
            "session.delete" => unix_adapter_handlers::handle_session_delete(state, &request).await,

            "health" | "health.check" => Ok(json!({
                "status": "healthy",
                "service": DEFAULT_SERVICE_NAME,
                "version": env!("CARGO_PKG_VERSION"),
                "isomorphic": true
            })),
            "health.liveness" => Ok(json!({"alive": true})),
            "health.readiness" => unix_adapter_handlers::handle_health_readiness(state).await,

            "version" => Ok(json!({
                "version": env!("CARGO_PKG_VERSION"),
                "ipc": "isomorphic"
            })),
            "capabilities.list" | "discover_capabilities" => {
                Ok(unix_adapter_handlers::capabilities_response())
            }
            "identity.get" => Ok(json!({
                "primal": nestgate_config::constants::system::DEFAULT_SERVICE_NAME,
                "version": env!("CARGO_PKG_VERSION"),
                "domain": "storage",
                "license": "AGPL-3.0-or-later",
                "family_id": state.family_id
            })),

            // nat.* — NAT traversal info uses its own sub-directory
            "nat.store_traversal_info" => {
                unix_adapter_handlers::handle_nat_store(state, &request).await
            }
            "nat.retrieve_traversal_info" => {
                unix_adapter_handlers::handle_nat_retrieve(state, &request).await
            }

            "beacon.store" => unix_adapter_handlers::handle_beacon_store(state, &request).await,
            "beacon.retrieve" => {
                unix_adapter_handlers::handle_beacon_retrieve(state, &request).await
            }
            "beacon.delete" => unix_adapter_handlers::handle_beacon_delete(state, &request).await,
            "beacon.list" => unix_adapter_handlers::handle_beacon_list(state).await,

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
