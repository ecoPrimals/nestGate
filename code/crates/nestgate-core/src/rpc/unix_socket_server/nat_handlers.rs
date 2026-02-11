//! NAT Traversal JSON-RPC Handlers
//!
//! Persists and retrieves NAT traversal info and known beacon records
//! using the existing `StorageManagerService` with reserved datasets.
//!
//! ## Methods
//!
//! | Method | Description |
//! |--------|-------------|
//! | `nat.store_traversal_info` | Store this device's NAT traversal info |
//! | `nat.retrieve_traversal_info` | Retrieve persisted NAT traversal info |
//! | `beacon.store` | Store a known beacon record |
//! | `beacon.retrieve` | Retrieve a beacon by peer_id |
//! | `beacon.list` | List all known beacon peer_ids |
//! | `beacon.delete` | Delete a beacon by peer_id |

use crate::error::{NestGateError, Result};
use crate::nat_traversal::{
    KnownBeacon, NatTraversalInfo, BEACON_DATASET, NAT_DATASET, NAT_SELF_KEY,
};
use serde_json::{json, Value};
use tracing::{debug, info};

use super::StorageState;

// ─── NAT Traversal Info ───────────────────────────────────────────────

/// `nat.store_traversal_info` — Persist this device's NAT traversal info.
///
/// Called by Songbird after STUN probing to cache results for future connections.
///
/// ## Params
///
/// The entire params object is deserialized as `NatTraversalInfo`:
///
/// ```json
/// {
///   "our_nat_type": "symmetric",
///   "port_pattern": { "type": "sequential", "step": 1, "last_port": 41204, "predicted_next": 41205, "confidence": 0.85 },
///   "last_probed": "2026-02-11T20:00:00Z",
///   "family_relay": { "tower": { "relay_addr": "192.168.1.144:3479", "stun_addr": "192.168.1.144:3478" } }
/// }
/// ```
pub(super) async fn nat_store_traversal_info(
    params: &Option<Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    // Validate structure by deserializing
    let info: NatTraversalInfo = serde_json::from_value(params.clone()).map_err(|e| {
        NestGateError::invalid_input_with_field("params", format!("invalid NatTraversalInfo: {e}"))
    })?;

    let data_bytes = serde_json::to_vec(&info)
        .map_err(|e| NestGateError::storage_error(&format!("Failed to serialize NAT info: {e}")))?;

    debug!(
        "nat.store_traversal_info: nat_type={}, pattern_type={}",
        info.our_nat_type,
        match &info.port_pattern {
            crate::nat_traversal::PortPattern::Sequential { .. } => "sequential",
            crate::nat_traversal::PortPattern::Random { .. } => "random",
            crate::nat_traversal::PortPattern::Unknown => "unknown",
        }
    );

    state
        .storage_manager
        .store_object(NAT_DATASET, NAT_SELF_KEY, data_bytes)
        .await?;

    info!(
        "nat.store_traversal_info: persisted (nat_type={}, last_probed={})",
        info.our_nat_type, info.last_probed
    );

    Ok(json!({
        "success": true,
        "nat_type": info.our_nat_type,
        "last_probed": info.last_probed,
    }))
}

/// `nat.retrieve_traversal_info` — Retrieve persisted NAT traversal info.
///
/// Returns the full `NatTraversalInfo` object, or `null` if not yet probed.
/// No params required.
pub(super) async fn nat_retrieve_traversal_info(
    _params: &Option<Value>,
    state: &StorageState,
) -> Result<Value> {
    debug!("nat.retrieve_traversal_info: retrieving");

    match state
        .storage_manager
        .retrieve_object(NAT_DATASET, NAT_SELF_KEY)
        .await
    {
        Ok((data, _object_info)) => {
            let nat_info: NatTraversalInfo = serde_json::from_slice(&data).map_err(|e| {
                NestGateError::storage_error(&format!("Failed to deserialize NAT info: {e}"))
            })?;

            info!(
                "nat.retrieve_traversal_info: found (nat_type={})",
                nat_info.our_nat_type
            );
            serde_json::to_value(&nat_info).map_err(|e| {
                NestGateError::storage_error(&format!("Failed to serialize response: {e}"))
            })
        }
        Err(_) => {
            debug!("nat.retrieve_traversal_info: no persisted NAT info");
            Ok(Value::Null)
        }
    }
}

// ─── Known Beacons ────────────────────────────────────────────────────

/// `beacon.store` — Store a known beacon record.
///
/// Called after a peer is discovered at rendezvous. The `peer_id` field
/// is used as the storage key.
///
/// ## Params
///
/// The entire params object is deserialized as `KnownBeacon`.
pub(super) async fn beacon_store(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let beacon: KnownBeacon = serde_json::from_value(params.clone()).map_err(|e| {
        NestGateError::invalid_input_with_field("params", format!("invalid KnownBeacon: {e}"))
    })?;

    let peer_id = beacon.peer_id.clone();
    let family_id = beacon.family_id.clone();

    let data_bytes = serde_json::to_vec(&beacon)
        .map_err(|e| NestGateError::storage_error(&format!("Failed to serialize beacon: {e}")))?;

    debug!(
        "beacon.store: peer_id='{}', family_id='{}'",
        peer_id, family_id
    );

    state
        .storage_manager
        .store_object(BEACON_DATASET, &peer_id, data_bytes)
        .await?;

    info!("beacon.store: persisted peer_id='{}'", peer_id);

    Ok(json!({
        "success": true,
        "peer_id": peer_id,
        "family_id": family_id,
    }))
}

/// `beacon.retrieve` — Retrieve a known beacon by peer_id.
///
/// ## Params
///
/// ```json
/// { "peer_id": "pixel-abc123" }
/// ```
///
/// Returns the full `KnownBeacon` object, or `null` if not found.
pub(super) async fn beacon_retrieve(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let peer_id = params["peer_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("peer_id", "peer_id (string) required")
    })?;

    debug!("beacon.retrieve: peer_id='{}'", peer_id);

    match state
        .storage_manager
        .retrieve_object(BEACON_DATASET, peer_id)
        .await
    {
        Ok((data, _object_info)) => {
            let beacon: KnownBeacon = serde_json::from_slice(&data).map_err(|e| {
                NestGateError::storage_error(&format!("Failed to deserialize beacon: {e}"))
            })?;

            info!("beacon.retrieve: found peer_id='{}'", peer_id);
            serde_json::to_value(&beacon).map_err(|e| {
                NestGateError::storage_error(&format!("Failed to serialize response: {e}"))
            })
        }
        Err(_) => {
            debug!("beacon.retrieve: peer_id='{}' not found", peer_id);
            Ok(Value::Null)
        }
    }
}

/// `beacon.list` — List all known beacon peer_ids.
///
/// No params required. Returns an object with `peer_ids` array and `count`.
///
/// Reads the `_known_beacons` dataset directory directly since
/// `StorageManagerService` doesn't expose an object-listing API.
pub(super) async fn beacon_list(_params: &Option<Value>, _state: &StorageState) -> Result<Value> {
    debug!("beacon.list: listing known beacons");

    let dataset_path = crate::config::storage_paths::get_storage_base_path()
        .join("datasets")
        .join(BEACON_DATASET);

    let mut peer_ids: Vec<String> = Vec::new();

    if dataset_path.exists() {
        let mut entries = tokio::fs::read_dir(&dataset_path).await.map_err(|e| {
            NestGateError::storage_error(&format!("Failed to read beacon dataset: {e}"))
        })?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Some(name) = entry.file_name().to_str() {
                // Skip hidden files and metadata
                if !name.starts_with('.') {
                    peer_ids.push(name.to_string());
                }
            }
        }
    }

    peer_ids.sort();
    let count = peer_ids.len();

    info!("beacon.list: {} beacons found", count);

    Ok(json!({
        "peer_ids": peer_ids,
        "count": count,
    }))
}

/// `beacon.delete` — Delete a known beacon by peer_id.
///
/// ## Params
///
/// ```json
/// { "peer_id": "pixel-abc123" }
/// ```
pub(super) async fn beacon_delete(params: &Option<Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let peer_id = params["peer_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("peer_id", "peer_id (string) required")
    })?;

    debug!("beacon.delete: peer_id='{}'", peer_id);

    state
        .storage_manager
        .delete_object(BEACON_DATASET, peer_id)
        .await?;

    info!("beacon.delete: removed peer_id='{}'", peer_id);

    Ok(json!({
        "success": true,
        "peer_id": peer_id,
    }))
}
