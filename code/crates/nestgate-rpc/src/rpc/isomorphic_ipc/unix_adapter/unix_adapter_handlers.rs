// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC method handlers for [`super::UnixSocketRpcHandler`](crate::rpc::isomorphic_ipc::unix_adapter::UnixSocketRpcHandler).

use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use serde_json::{Value, json};
use std::borrow::Cow;

use super::{JsonRpcRequest, StorageState};

pub(super) async fn handle_storage_store(
    state: &StorageState,
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

    let path = state.key_path(key)?;
    let data = serde_json::to_vec_pretty(value)
        .map_err(|e| (-32603, Cow::Owned(format!("Serialization error: {e}"))))?;

    tokio::fs::write(&path, &data)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Storage write error: {e}"))))?;

    Ok(json!({"status": "stored", "key": key}))
}

pub(super) async fn handle_storage_retrieve(
    state: &StorageState,
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

    let path = state.key_path(key)?;
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

pub(super) async fn handle_storage_list(
    state: &StorageState,
    _request: &JsonRpcRequest,
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let dir = &state.dataset_dir;
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

pub(super) async fn handle_storage_delete(
    state: &StorageState,
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

    let path = state.key_path(key)?;
    if path.exists() {
        tokio::fs::remove_file(&path)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("Storage delete error: {e}"))))?;
    }
    Ok(json!({"status": "deleted", "key": key}))
}

pub(super) fn handle_storage_exists(
    state: &StorageState,
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

    let path = state.key_path(key)?;
    Ok(json!({"exists": path.exists(), "key": key}))
}

pub(super) async fn handle_health_readiness(
    state: &StorageState,
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let dir_ok = state.dataset_dir.exists()
        && tokio::fs::metadata(&state.dataset_dir)
            .await
            .map(|m| m.is_dir())
            .unwrap_or(false);
    Ok(json!({
        "ready": dir_ok,
        "storage_path": state.dataset_dir.display().to_string(),
    }))
}

/// Resolve the session directory, using `family_id` from `params` for path consistency
/// with the `unix_socket_server` session handlers.
fn resolve_session_dir(state: &StorageState, params: Option<&Value>) -> std::path::PathBuf {
    let params = params.unwrap_or(&Value::Null);
    let family = params
        .get("family_id")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    // Layout: {base}/datasets/{family}/_sessions/
    state
        .dataset_dir
        .parent()
        .unwrap_or(&state.dataset_dir)
        .join(family)
        .join("_sessions")
}

pub(super) async fn handle_session_save(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let params = request
        .params
        .as_ref()
        .ok_or((-32602, Cow::Borrowed("Missing params")))?;
    let session_id = params
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'session_id' parameter")))?;
    let data = params
        .get("data")
        .or_else(|| params.get("state"))
        .ok_or((-32602, Cow::Borrowed("Missing 'data' or 'state' parameter")))?;

    let dir = resolve_session_dir(state, request.params.as_ref());
    tokio::fs::create_dir_all(&dir)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("mkdir: {e}"))))?;
    let path = dir.join(format!("{session_id}.json"));
    let bytes =
        serde_json::to_vec_pretty(data).map_err(|e| (-32603, Cow::Owned(format!("json: {e}"))))?;
    tokio::fs::write(&path, &bytes)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("write: {e}"))))?;
    let family = params
        .get("family_id")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    Ok(
        json!({"status": "saved", "session_id": session_id, "family_id": family, "size": bytes.len()}),
    )
}

pub(super) async fn handle_session_load(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let params = request
        .params
        .as_ref()
        .ok_or((-32602, Cow::Borrowed("Missing params")))?;
    let session_id = params
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'session_id' parameter")))?;

    let dir = resolve_session_dir(state, request.params.as_ref());
    let path = dir.join(format!("{session_id}.json"));
    if !path.exists() {
        return Ok(json!({"data": null, "session_id": session_id, "found": false}));
    }
    let bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("read: {e}"))))?;
    let data: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));
    let family = params
        .get("family_id")
        .and_then(|v| v.as_str())
        .unwrap_or("default");
    Ok(
        json!({"data": data, "session_id": session_id, "family_id": family, "found": true, "size": bytes.len()}),
    )
}

pub(super) async fn handle_session_list(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let dir = resolve_session_dir(state, request.params.as_ref());
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
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let params = request
        .params
        .as_ref()
        .ok_or((-32602, Cow::Borrowed("Missing params")))?;
    let session_id = params
        .get("session_id")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'session_id' parameter")))?;
    let dir = resolve_session_dir(state, request.params.as_ref());
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
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let params = request
        .params
        .as_ref()
        .ok_or((-32602, Cow::Borrowed("Missing params")))?;
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
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let params = request
        .params
        .as_ref()
        .ok_or((-32602, Cow::Borrowed("Missing params")))?;
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
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
    let params = request
        .params
        .as_ref()
        .ok_or((-32602, Cow::Borrowed("Missing params")))?;
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

pub(super) async fn handle_beacon_list(
    state: &StorageState,
) -> std::result::Result<Value, (i32, Cow<'static, str>)> {
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

/// Static JSON for `capabilities.list` / `discover_capabilities`.
pub(super) fn capabilities_response() -> Value {
    json!({
        "primal": DEFAULT_SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION"),
        "domain": "storage",
        "capabilities": [
            "storage.store", "storage.retrieve", "storage.list",
            "storage.delete", "storage.exists",
            "session.save", "session.load",
            "nat.store_traversal_info", "nat.retrieve_traversal_info",
            "beacon.store", "beacon.retrieve", "beacon.list", "beacon.delete",
            "health", "health.check", "health.liveness", "health.readiness",
            "capabilities.list", "version"
        ]
    })
}
