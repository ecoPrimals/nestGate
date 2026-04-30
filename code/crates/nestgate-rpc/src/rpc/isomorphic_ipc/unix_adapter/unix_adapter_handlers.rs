// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC method handlers for [`super::UnixSocketRpcHandler`](crate::rpc::isomorphic_ipc::unix_adapter::UnixSocketRpcHandler).

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use serde_json::{Value, json};
use std::borrow::Cow;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use super::{DEFAULT_NAMESPACE, JsonRpcRequest, StorageState};

type HandlerResult = std::result::Result<Value, (i32, Cow<'static, str>)>;

/// Extract the optional `namespace` parameter, defaulting to `"shared"`.
fn extract_namespace(params: &Value) -> &str {
    params
        .get("namespace")
        .and_then(|v| v.as_str())
        .unwrap_or(DEFAULT_NAMESPACE)
}

/// Extract required `key` from params.
fn extract_key(params: &Value) -> std::result::Result<&str, (i32, Cow<'static, str>)> {
    params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or((-32602, Cow::Borrowed("Missing 'key' parameter")))
}

fn require_params(
    request: &JsonRpcRequest,
) -> std::result::Result<&Value, (i32, Cow<'static, str>)> {
    request
        .params
        .as_ref()
        .ok_or((-32602, Cow::Borrowed("Missing params")))
}

/// Ensure parent directories exist before writing.
async fn ensure_parent(
    path: &std::path::Path,
) -> std::result::Result<(), (i32, Cow<'static, str>)> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("mkdir: {e}"))))?;
    }
    Ok(())
}

// ── storage.store ──────────────────────────────────────────────────────

pub(super) async fn handle_storage_store(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let key = extract_key(params)?;
    let namespace = extract_namespace(params);
    let value = params
        .get("value")
        .ok_or((-32602, Cow::Borrowed("Missing 'value' parameter")))?;

    let path = state.key_path(namespace, key)?;
    ensure_parent(&path).await?;
    let data = serde_json::to_vec_pretty(value)
        .map_err(|e| (-32603, Cow::Owned(format!("Serialization error: {e}"))))?;

    tokio::fs::write(&path, &data)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Storage write error: {e}"))))?;

    Ok(
        json!({"status": "stored", "key": key, "namespace": namespace, "family_id": state.family_id}),
    )
}

// ── storage.retrieve ───────────────────────────────────────────────────

pub(super) async fn handle_storage_retrieve(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let key = extract_key(params)?;
    let namespace = extract_namespace(params);

    let path = state.key_path(namespace, key)?;
    if !path.exists() {
        return Ok(json!({"value": null, "data": null, "key": key, "namespace": namespace}));
    }

    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Storage read error: {e}"))))?;
    let value: Value = serde_json::from_slice(&data)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&data).to_string()));
    Ok(json!({"value": value, "data": value, "key": key, "namespace": namespace}))
}

// ── storage.list ───────────────────────────────────────────────────────

pub(super) async fn handle_storage_list(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let null = Value::Null;
    let params = request.params.as_ref().unwrap_or(&null);
    let namespace = extract_namespace(params);
    StorageState::validate_segment(namespace, "namespace")?;

    let dir = state.namespace_dir(namespace);
    let mut keys = Vec::new();
    if dir.exists() {
        let mut entries = tokio::fs::read_dir(&dir)
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
    Ok(
        json!({"keys": keys, "count": keys.len(), "namespace": namespace, "family_id": state.family_id}),
    )
}

// ── storage.delete ─────────────────────────────────────────────────────

pub(super) async fn handle_storage_delete(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let key = extract_key(params)?;
    let namespace = extract_namespace(params);

    let path = state.key_path(namespace, key)?;
    if path.exists() {
        tokio::fs::remove_file(&path)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("Storage delete error: {e}"))))?;
    }
    // Also remove blob if present.
    let blob = state.blob_path(namespace, key)?;
    if blob.exists() {
        let _ = tokio::fs::remove_file(&blob).await;
    }
    Ok(json!({"status": "deleted", "key": key, "namespace": namespace}))
}

// ── storage.exists ─────────────────────────────────────────────────────

pub(super) fn handle_storage_exists(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let key = extract_key(params)?;
    let namespace = extract_namespace(params);

    let json_exists = state
        .key_path(namespace, key)
        .map(|p| p.exists())
        .unwrap_or(false);
    let blob_exists = state
        .blob_path(namespace, key)
        .map(|p| p.exists())
        .unwrap_or(false);
    Ok(json!({"exists": json_exists || blob_exists, "key": key, "namespace": namespace}))
}

// ── storage.store_blob ─────────────────────────────────────────────────

const MAX_CHUNK: u64 = 4 * 1024 * 1024; // 4 MiB per chunk

pub(super) async fn handle_storage_store_blob(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let key = extract_key(params)?;
    let namespace = extract_namespace(params);
    let blob_b64 = params.get("blob").and_then(|v| v.as_str()).ok_or((
        -32602,
        Cow::Borrowed("Missing 'blob' (base64 string) parameter"),
    ))?;

    let blob_data = STANDARD
        .decode(blob_b64)
        .map_err(|e| (-32602, Cow::Owned(format!("Invalid base64: {e}"))))?;

    let path = state.blob_path(namespace, key)?;
    ensure_parent(&path).await?;
    tokio::fs::write(&path, &blob_data)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Blob write error: {e}"))))?;

    Ok(json!({
        "status": "stored", "key": key, "namespace": namespace,
        "family_id": state.family_id, "size": blob_data.len()
    }))
}

// ── storage.retrieve_blob ──────────────────────────────────────────────

pub(super) async fn handle_storage_retrieve_blob(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let key = extract_key(params)?;
    let namespace = extract_namespace(params);

    let path = state.blob_path(namespace, key)?;
    if !path.exists() {
        return Ok(json!({"data": null, "key": key, "namespace": namespace}));
    }

    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Blob read error: {e}"))))?;
    Ok(json!({
        "data": STANDARD.encode(&data), "key": key,
        "namespace": namespace, "encoding": "base64", "size": data.len()
    }))
}

// ── storage.retrieve_range ─────────────────────────────────────────────

pub(super) async fn handle_storage_retrieve_range(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let key = extract_key(params)?;
    let namespace = extract_namespace(params);
    let offset = params.get("offset").and_then(Value::as_u64).unwrap_or(0);
    let raw_length = params
        .get("length")
        .and_then(Value::as_u64)
        .ok_or((-32602, Cow::Borrowed("Missing 'length' (u64) parameter")))?;
    let length = usize::try_from(raw_length.min(MAX_CHUNK)).unwrap_or(usize::MAX);

    let blob = state.blob_path(namespace, key)?;
    let json_path = state.key_path(namespace, key)?;
    let path = if blob.exists() {
        blob
    } else if json_path.exists() {
        json_path
    } else {
        return Ok(json!({"data": null, "key": key, "error": "not_found"}));
    };

    let mut file = tokio::fs::File::open(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Open error: {e}"))))?;
    let total_size = file
        .metadata()
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Stat error: {e}"))))?
        .len();

    file.seek(std::io::SeekFrom::Start(offset))
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Seek error: {e}"))))?;

    let mut buf = vec![0u8; length];
    let bytes_read = file
        .read(&mut buf)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Read error: {e}"))))?;
    buf.truncate(bytes_read);

    Ok(json!({
        "data": STANDARD.encode(&buf),
        "offset": offset, "length": bytes_read, "total_size": total_size,
        "key": key, "namespace": namespace, "encoding": "base64"
    }))
}

// ── storage.store_stream / storage.retrieve_stream (chunked tensor payloads) ──

pub(super) async fn handle_storage_store_stream(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?.clone();
    crate::rpc::storage_stream::storage_store_stream_begin(params, Some(state.family_id.as_str()))
        .await
        .map_err(|e| (-32603, Cow::Owned(e.to_string())))
}

pub(super) async fn handle_storage_store_stream_chunk(request: &JsonRpcRequest) -> HandlerResult {
    let params = require_params(request)?.clone();
    crate::rpc::storage_stream::storage_store_stream_chunk(params)
        .await
        .map_err(|e| (-32603, Cow::Owned(e.to_string())))
}

pub(super) async fn handle_storage_retrieve_stream(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?.clone();
    crate::rpc::storage_stream::storage_retrieve_stream_begin(
        params,
        Some(state.family_id.as_str()),
    )
    .await
    .map_err(|e| (-32603, Cow::Owned(e.to_string())))
}

pub(super) async fn handle_storage_retrieve_stream_chunk(
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?.clone();
    crate::rpc::storage_stream::storage_retrieve_stream_chunk(params)
        .await
        .map_err(|e| (-32603, Cow::Owned(e.to_string())))
}

// ── storage.object.size ────────────────────────────────────────────────

pub(super) async fn handle_storage_object_size(
    state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let params = require_params(request)?;
    let key = extract_key(params)?;
    let namespace = extract_namespace(params);

    let blob = state.blob_path(namespace, key)?;
    let json_path = state.key_path(namespace, key)?;

    let (path, storage_type) = if blob.exists() {
        (blob, "blob")
    } else if json_path.exists() {
        (json_path, "object")
    } else {
        return Ok(
            json!({"exists": false, "key": key, "namespace": namespace, "storage_type": "none"}),
        );
    };

    let meta = tokio::fs::metadata(&path)
        .await
        .map_err(|e| (-32603, Cow::Owned(format!("Stat error: {e}"))))?;

    Ok(json!({
        "exists": true, "key": key, "namespace": namespace,
        "size": meta.len(), "storage_type": storage_type
    }))
}

// ── storage.namespaces.list ────────────────────────────────────────────

pub(super) async fn handle_storage_namespaces_list(state: &StorageState) -> HandlerResult {
    let mut namespaces = Vec::new();
    if state.family_dir.exists() {
        let mut entries = tokio::fs::read_dir(&state.family_dir)
            .await
            .map_err(|e| (-32603, Cow::Owned(format!("readdir: {e}"))))?;
        while let Ok(Some(entry)) = entries.next_entry().await {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with('_')
                && entry
                    .file_type()
                    .await
                    .map(|ft| ft.is_dir())
                    .unwrap_or(false)
            {
                namespaces.push(name.to_string());
            }
        }
    }
    namespaces.sort();
    Ok(json!({"namespaces": namespaces, "family_id": state.family_id, "count": namespaces.len()}))
}

pub(super) async fn handle_storage_fetch_external(
    _state: &StorageState,
    request: &JsonRpcRequest,
) -> HandlerResult {
    let legacy_state = crate::rpc::unix_socket_server::StorageState::new()
        .map_err(|e| (-32603, Cow::Owned(format!("state init: {e}"))))?;
    crate::rpc::unix_socket_server::external_handlers::storage_fetch_external(
        request.params.as_ref(),
        &legacy_state,
    )
    .await
    .map_err(|e| (-32603, Cow::Owned(e.to_string())))
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

/// Wire Standard L2 envelope for `capabilities.list` / `discover_capabilities`.
pub(super) fn capabilities_response() -> Value {
    json!({
        "primal": DEFAULT_SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION"),
        "methods": [
            "storage.store", "storage.retrieve", "storage.list",
            "storage.delete", "storage.exists", "storage.fetch_external",
            "storage.store_blob", "storage.retrieve_blob",
            "storage.retrieve_range", "storage.object.size",
            "storage.store_stream", "storage.store_stream_chunk",
            "storage.retrieve_stream", "storage.retrieve_stream_chunk",
            "storage.namespaces.list",
            "session.save", "session.load",
            "nat.store_traversal_info", "nat.retrieve_traversal_info",
            "beacon.store", "beacon.retrieve", "beacon.list", "beacon.delete",
            "zfs.pool.list", "zfs.pool.get", "zfs.pool.health",
            "zfs.dataset.list", "zfs.dataset.get",
            "zfs.snapshot.list", "zfs.health",
            "health.check", "health.liveness", "health.readiness",
            "capabilities.list", "identity.get"
        ]
    })
}
