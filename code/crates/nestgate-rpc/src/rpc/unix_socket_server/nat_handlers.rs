// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NAT Traversal & Beacon Persistence JSON-RPC Handlers
//!
//! Filesystem-backed persistence for NAT traversal info and peer beacons.
//! NAT records are stored under `{storage_base}/datasets/_nat_traversal/{peer_id}.json`.
//! Beacon records are stored under `{storage_base}/datasets/_known_beacons/{peer_id}.json`.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::PathBuf;
use tracing::debug;

use super::StorageState;

const NAT_DATASET: &str = "_nat_traversal";
const BEACON_DATASET: &str = "_known_beacons";

fn nat_path(peer_id: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(NAT_DATASET)
        .join(format!("{peer_id}.json"))
}

fn beacon_path(peer_id: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(BEACON_DATASET)
        .join(format!("{peer_id}.json"))
}

async fn ensure_dir(path: &std::path::Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            NestGateError::io_error(format!(
                "Failed to create directory {}: {e}",
                parent.display()
            ))
        })?;
    }
    Ok(())
}

fn require_peer_id(params: Option<&Value>) -> Result<&str> {
    params.and_then(|p| p["peer_id"].as_str()).ok_or_else(|| {
        NestGateError::invalid_input_with_field("peer_id", "peer_id (string) required")
    })
}

/// `nat.store_traversal_info` — persist NAT traversal info for a peer.
pub(super) async fn nat_store_traversal_info(
    params: Option<&Value>,
    _state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let peer_id = params["peer_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("peer_id", "peer_id (string) required")
    })?;

    let path = nat_path(peer_id);
    ensure_dir(&path).await?;

    let empty = json!({});
    let record = json!({
        "peer_id": peer_id,
        "traversal_info": params.get("traversal_info").unwrap_or(&empty),
        "stored_at": chrono::Utc::now().to_rfc3339(),
    });

    let data = serde_json::to_vec_pretty(&record)
        .map_err(|e| NestGateError::io_error(format!("Failed to serialize NAT record: {e}")))?;
    tokio::fs::write(&path, &data)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to write NAT traversal file: {e}")))?;

    debug!(peer_id, "nat.store_traversal_info: persisted");
    Ok(json!({ "peer_id": peer_id, "stored": true }))
}

/// `nat.retrieve_traversal_info` — read NAT traversal info for a peer.
pub(super) async fn nat_retrieve_traversal_info(
    params: Option<&Value>,
    _state: &StorageState,
) -> Result<Value> {
    let peer_id = require_peer_id(params)?;
    let path = nat_path(peer_id);

    let data = tokio::fs::read(&path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NestGateError::not_found(format!("No NAT traversal info for peer '{peer_id}'"))
        } else {
            NestGateError::io_error(format!("Failed to read NAT traversal file: {e}"))
        }
    })?;

    let record: Value = serde_json::from_slice(&data)
        .map_err(|e| NestGateError::io_error(format!("Corrupted NAT record: {e}")))?;

    debug!(peer_id, "nat.retrieve_traversal_info: loaded");
    Ok(record)
}

/// `beacon.store` — persist a peer beacon record.
pub(super) async fn beacon_store(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;
    let peer_id = params["peer_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("peer_id", "peer_id (string) required")
    })?;

    let path = beacon_path(peer_id);
    ensure_dir(&path).await?;

    let empty = json!({});
    let record = json!({
        "peer_id": peer_id,
        "beacon_data": params.get("beacon_data").unwrap_or(&empty),
        "endpoint": params.get("endpoint"),
        "stored_at": chrono::Utc::now().to_rfc3339(),
    });

    let data = serde_json::to_vec_pretty(&record)
        .map_err(|e| NestGateError::io_error(format!("Failed to serialize beacon record: {e}")))?;
    tokio::fs::write(&path, &data)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to write beacon file: {e}")))?;

    debug!(peer_id, "beacon.store: persisted");
    Ok(json!({ "peer_id": peer_id, "stored": true }))
}

/// `beacon.retrieve` — read a peer beacon record.
pub(super) async fn beacon_retrieve(
    params: Option<&Value>,
    _state: &StorageState,
) -> Result<Value> {
    let peer_id = require_peer_id(params)?;
    let path = beacon_path(peer_id);

    let data = tokio::fs::read(&path).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            NestGateError::not_found(format!("No beacon for peer '{peer_id}'"))
        } else {
            NestGateError::io_error(format!("Failed to read beacon file: {e}"))
        }
    })?;

    let record: Value = serde_json::from_slice(&data)
        .map_err(|e| NestGateError::io_error(format!("Corrupted beacon record: {e}")))?;

    debug!(peer_id, "beacon.retrieve: loaded");
    Ok(record)
}

/// `beacon.list` — list all known peer beacons from the beacon dataset directory.
pub(super) async fn beacon_list(_params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    debug!("beacon.list: listing known beacons");

    let dataset_path = get_storage_base_path()
        .join("datasets")
        .join(BEACON_DATASET);

    let mut peer_ids: Vec<String> = Vec::new();

    if dataset_path.exists() {
        let mut entries = tokio::fs::read_dir(&dataset_path).await.map_err(|e| {
            NestGateError::storage_error(format!("Failed to read beacon dataset: {e}"))
        })?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Some(name) = entry.file_name().to_str()
                && !name.starts_with('.')
            {
                let peer_id = name.strip_suffix(".json").unwrap_or(name);
                peer_ids.push(peer_id.to_string());
            }
        }
    }

    peer_ids.sort();
    let count = peer_ids.len();

    Ok(json!({
        "peer_ids": peer_ids,
        "count": count,
    }))
}

/// `beacon.delete` — remove a peer beacon record.
pub(super) async fn beacon_delete(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let peer_id = require_peer_id(params)?;
    let path = beacon_path(peer_id);

    if !path.exists() {
        return Err(NestGateError::not_found(format!(
            "No beacon for peer '{peer_id}'"
        )));
    }

    tokio::fs::remove_file(&path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to delete beacon file: {e}")))?;

    debug!(peer_id, "beacon.delete: removed");
    Ok(json!({ "peer_id": peer_id, "deleted": true }))
}
