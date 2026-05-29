// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Non-storage JSON-RPC method handlers for the isomorphic IPC Unix adapter.
//!
//! `storage.*` handlers live in the sibling [`super::storage_handlers`] module,
//! extracted to keep each module under the 800-line ecosystem threshold.

use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use serde_json::{Value, json};
use std::borrow::Cow;

use super::{DEFAULT_NAMESPACE, JsonRpcRequest, StorageState};

pub(super) type HandlerResult = std::result::Result<Value, (i32, Cow<'static, str>)>;

/// Extract the optional `namespace` parameter, defaulting to `"shared"`.
pub(super) fn extract_namespace(params: &Value) -> &str {
    params
        .get("namespace")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_NAMESPACE)
}

/// Extract required `key` from params.
pub(super) fn extract_key(params: &Value) -> std::result::Result<&str, (i32, Cow<'static, str>)> {
    params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'key' parameter")))
}

pub(super) fn require_params(
    request: &JsonRpcRequest,
) -> std::result::Result<&Value, (i32, Cow<'static, str>)> {
    request
        .params
        .as_ref()
        .ok_or((-32602, Cow::Borrowed("Missing params")))
}

/// Ensure parent directories exist before writing.
pub(super) async fn ensure_parent(
    path: &std::path::Path,
) -> std::result::Result<(), (i32, Cow<'static, str>)> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("mkdir: {e}"))))?;
    }
    Ok(())
}

pub(super) async fn handle_health_readiness(state: &StorageState) -> HandlerResult {
    let shared_dir = state.namespace_dir(DEFAULT_NAMESPACE);
    let dir_ok = shared_dir.exists()
        && tokio::fs::metadata(&shared_dir)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false);
    Ok(json!({
        "ready": dir_ok,
        "storage_path": shared_dir.display().to_string(),
        "family_id": state.family_id,
    }))
}

/// Resolve the session directory under the family root.
fn resolve_session_dir(state: &StorageState) -> std::path::PathBuf {
    state.family_dir.join("_sessions")
}

pub(super) async fn handle_session_save(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let session_id = params
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'session_id' parameter")))?;
    let data = params
        .get("data")
        .or_else(|| params.get("state"))
        .ok_or((-32602, Cow::Borrowed("Missing 'data' or 'state' parameter")))?;

    let dir = resolve_session_dir(state);
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("mkdir: {e}"))))?;
    let path = dir.join(format!("{session_id}.json"));
    let bytes =
        serde_json::to_vec_pretty(data).map_err(|e| (-32603, Cow::Owned(format!("json: {e}"))))?;
    tokio::fs::write(&path, &bytes)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("write: {e}"))))?;
    Ok(
        json!({"status": "saved", "session_id": session_id, "family_id": state.family_id, "size": bytes.len()}),
    )
}

pub(super) async fn handle_session_load(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let session_id = params
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'session_id' parameter")))?;

    let dir = resolve_session_dir(state);
    let path = dir.join(format!("{session_id}.json"));
    if !path.exists() {
        return Ok(json!({"data": null, "session_id": session_id, "found": false}));
    }
    let bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("read: {e}"))))?;
    let data: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));
    Ok(
        json!({"data": data, "session_id": session_id, "family_id": state.family_id, "found": true, "size": bytes.len()}),
    )
}

pub(super) async fn handle_session_list(
    state: &StorageState,
    _request: &JsonRpcRequest,
) -> HandlerResult {
    let dir = resolve_session_dir(state);
    if !dir.exists() {
        return Ok(json!({"sessions": []}));
    }
    let mut entries = tokio::fs::read_dir(&dir)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("readdir: {e}"))))?;
    let mut ids = Vec::new();
    while let Ok(Some(entry)) = entries.next_entry().await {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if let Some(id) = name.strip_suffix(".json") {
            ids.push(id.to_string());
        }
    }
    Ok(json!({"sessions": ids}))
}

pub(super) async fn handle_session_delete(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let session_id = params
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'session_id' parameter")))?;
    let dir = resolve_session_dir(state);
    let path = dir.join(format!("{session_id}.json"));
    if !path.exists() {
        return Ok(json!({"deleted": false, "session_id": session_id, "reason": "not found"}));
    }
    tokio::fs::remove_file(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("rm: {e}"))))?;
    Ok(json!({"deleted": true, "session_id": session_id}))
}

pub(super) async fn handle_nat_store(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let peer_id = params
        .get("peer_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;
    let info = params
        .get("info")
        .ok_or((-32602, Cow::Borrowed("Missing 'info'")))?;

    let dir = state.nat_dir();
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("mkdir: {e}"))))?;
    let path = dir.join(format!("{peer_id}.json"));
    let data =
        serde_json::to_vec_pretty(info).map_err(|e| (-32603, Cow::Owned(format!("json: {e}"))))?;
    tokio::fs::write(&path, &data)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("write: {e}"))))?;
    Ok(json!({"status": "stored", "peer_id": peer_id}))
}

pub(super) async fn handle_nat_retrieve(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let peer_id = params
        .get("peer_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;

    let path = state.nat_dir().join(format!("{peer_id}.json"));
    if !path.exists() {
        return Ok(json!({"info": null, "peer_id": peer_id}));
    }
    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("read: {e}"))))?;
    let info: Value = serde_json::from_slice(&data).unwrap_or(Value::Null);
    Ok(json!({"info": info, "peer_id": peer_id}))
}

pub(super) async fn handle_beacon_store(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let peer_id = params
        .get("peer_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;
    let beacon = params
        .get("beacon")
        .ok_or((-32602, Cow::Borrowed("Missing 'beacon'")))?;

    let dir = state.beacon_dir();
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

pub(super) async fn handle_beacon_retrieve(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let peer_id = params
        .get("peer_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;

    let path = state.beacon_dir().join(format!("{peer_id}.json"));
    if !path.exists() {
        return Ok(json!({"beacon": null, "peer_id": peer_id}));
    }
    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("read: {e}"))))?;
    let beacon: Value = serde_json::from_slice(&data).unwrap_or(Value::Null);
    Ok(json!({"beacon": beacon, "peer_id": peer_id}))
}

pub(super) async fn handle_beacon_delete(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let peer_id = params
        .get("peer_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'peer_id'")))?;

    let path = state.beacon_dir().join(format!("{peer_id}.json"));
    if path.exists() {
        tokio::fs::remove_file(&path)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("delete: {e}"))))?;
    }
    Ok(json!({"status": "deleted", "peer_id": peer_id}))
}

pub(super) async fn handle_beacon_list(state: &StorageState) -> HandlerResult {
    let dataset_path = state.beacon_dir();

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

// ── content.* delegation ───────────────────────────────────────────────
//
// Content-addressed storage uses the canonical handlers from
// `unix_socket_server::content_handlers`, bridged through a shared
// `unix_socket_server::StorageState`.

use crate::rpc::unix_socket_server::{StorageState as ContentState, content_handlers};
use std::sync::OnceLock;

fn content_state() -> &'static ContentState {
    static STATE: OnceLock<ContentState> = OnceLock::new();
    STATE.get_or_init(|| {
        #[expect(
            clippy::expect_used,
            reason = "ContentState::new only fails on unrecoverable I/O — crash is correct"
        )]
        ContentState::new().expect("ContentState initialization must not fail for content routing")
    })
}

fn content_err(e: &nestgate_types::error::NestGateError) -> (i32, Cow<'static, str>) {
    (-32603, Cow::Owned(e.to_string()))
}

pub(super) async fn handle_content_put(request: &JsonRpcRequest) -> HandlerResult {
    content_handlers::content_put(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_get(request: &JsonRpcRequest) -> HandlerResult {
    content_handlers::content_get(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_exists(request: &JsonRpcRequest) -> HandlerResult {
    content_handlers::content_exists(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_list(request: &JsonRpcRequest) -> HandlerResult {
    content_handlers::content_list(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_publish(request: &JsonRpcRequest) -> HandlerResult {
    content_handlers::content_publish(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_resolve(request: &JsonRpcRequest) -> HandlerResult {
    content_handlers::content_resolve(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_promote(request: &JsonRpcRequest) -> HandlerResult {
    content_handlers::content_promote(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_collections(request: &JsonRpcRequest) -> HandlerResult {
    content_handlers::content_collections(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

use crate::rpc::unix_socket_server::content_federation_handlers;

pub(super) async fn handle_content_fetch_heads(request: &JsonRpcRequest) -> HandlerResult {
    content_federation_handlers::content_fetch_heads(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_push(request: &JsonRpcRequest) -> HandlerResult {
    content_federation_handlers::content_push(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_replicate(request: &JsonRpcRequest) -> HandlerResult {
    content_federation_handlers::content_replicate(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

pub(super) async fn handle_content_sync(request: &JsonRpcRequest) -> HandlerResult {
    content_federation_handlers::content_sync(request.params.as_ref(), content_state())
        .await
        .map_err(|e| content_err(&e))
}

/// All methods advertised by the isomorphic IPC adapter.
const ISOMORPHIC_IPC_METHODS: &[&str] = &[
    "storage.store",
    "storage.retrieve",
    "storage.list",
    "storage.delete",
    "storage.exists",
    "storage.fetch_external",
    "storage.store_blob",
    "storage.retrieve_blob",
    "storage.retrieve_range",
    "storage.object.size",
    "storage.store_stream",
    "storage.store_stream_chunk",
    "storage.retrieve_stream",
    "storage.retrieve_stream_chunk",
    "storage.namespaces.list",
    "content.put",
    "content.get",
    "content.exists",
    "content.list",
    "content.publish",
    "content.resolve",
    "content.promote",
    "content.collections",
    "content.fetch_heads",
    "content.push",
    "content.replicate",
    "content.sync",
    "session.save",
    "session.load",
    "nat.store_traversal_info",
    "nat.retrieve_traversal_info",
    "beacon.store",
    "beacon.retrieve",
    "beacon.list",
    "beacon.delete",
    "zfs.pool.list",
    "zfs.pool.get",
    "zfs.pool.health",
    "zfs.dataset.list",
    "zfs.dataset.get",
    "zfs.snapshot.list",
    "zfs.health",
    "health.check",
    "health.liveness",
    "health.readiness",
    "capabilities.list",
    "identity.get",
    "lifecycle.status",
    "auth.check",
    "auth.mode",
    "auth.peer_info",
    "btsp.capabilities",
];

/// Wire Standard L3 envelope for `capabilities.list` / `discover_capabilities`.
pub(super) fn capabilities_response() -> Value {
    json!({
        "primal": DEFAULT_SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION"),
        "capabilities": ISOMORPHIC_IPC_METHODS,
        "count": ISOMORPHIC_IPC_METHODS.len(),
        "protocol": "jsonrpc-2.0",
        "transport": ["uds", "tcp", "http"]
    })
}
