// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NAT Traversal JSON-RPC Handlers
//!
//! Persistence previously used `StorageManagerService` + `nat_traversal` from nestgate-core.
//! These methods return `not_implemented` until cross-crate wiring; `beacon.list` uses local paths.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

use super::StorageState;

/// **Integration:** Align with `nestgate_core::nat_traversal::BEACON_DATASET` when this crate links to core.
const BEACON_DATASET: &str = "_known_beacons";

/// `nat.store_traversal_info` — stub until nestgate-core + storage wiring.
pub(super) fn nat_store_traversal_info(
    _params: Option<&Value>,
    _state: &StorageState,
) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core nat_traversal + storage",
    ))
}

/// `nat.retrieve_traversal_info` — stub.
pub(super) fn nat_retrieve_traversal_info(
    _params: Option<&Value>,
    _state: &StorageState,
) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core nat_traversal + storage",
    ))
}

/// `beacon.store` — stub.
pub(super) fn beacon_store(_params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core nat_traversal + storage",
    ))
}

/// `beacon.retrieve` — stub.
pub(super) fn beacon_retrieve(_params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core nat_traversal + storage",
    ))
}

/// `beacon.list` — lists beacon dataset directory keys (filesystem only).
pub(super) async fn beacon_list(_params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    debug!("feature pending: NAT/beacon persistence via nestgate-core nat_traversal");
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
                peer_ids.push(name.to_string());
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

/// `beacon.delete` — stub.
pub(super) fn beacon_delete(_params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core nat_traversal + storage",
    ))
}
