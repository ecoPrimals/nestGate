// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Public stateless coordination API.
//!
//! Wraps `unix_socket_server::coord_handlers` behind a `StorageState`-free
//! interface so that **any crate** (including `nestgate-api`) can call
//! coordination operations without constructing internal RPC state.
//!
//! Every function accepts a [`serde_json::Value`] parameter object and returns
//! a JSON-RPC–shaped [`serde_json::Value`] result.

use crate::rpc::unix_socket_server::{StorageState, coord_handlers};
use nestgate_types::error::Result;
use serde_json::Value;
use std::sync::OnceLock;

fn shared_state() -> &'static StorageState {
    static STATE: OnceLock<StorageState> = OnceLock::new();
    STATE.get_or_init(|| {
        #[expect(
            clippy::expect_used,
            reason = "StorageState::new only fails on unrecoverable I/O — crash is correct"
        )]
        StorageState::new().expect("StorageState initialization must not fail for coord ops")
    })
}

/// `coord.blurbs.current` — return the current wave blurb.
///
/// # Errors
///
/// Returns error on I/O or manifest failure.
pub async fn blurbs_current(params: &Value) -> Result<Value> {
    coord_handlers::coord_blurbs_current(Some(params), shared_state()).await
}

/// `coord.blurbs.list` — list all ingested blurbs (newest first).
///
/// # Errors
///
/// Returns error on I/O or manifest failure.
pub async fn blurbs_list(params: &Value) -> Result<Value> {
    coord_handlers::coord_blurbs_list(Some(params), shared_state()).await
}

/// `coord.blurbs.get` — retrieve a specific blurb by hash or wave number.
///
/// # Errors
///
/// Returns error on missing params, not found, or I/O failure.
pub async fn blurbs_get(params: &Value) -> Result<Value> {
    coord_handlers::coord_blurbs_get(Some(params), shared_state()).await
}

/// `coord.fragos.list` — list all FRAGOs and AARs (newest first).
///
/// # Errors
///
/// Returns error on I/O or manifest failure.
pub async fn fragos_list(params: &Value) -> Result<Value> {
    coord_handlers::coord_fragos_list(Some(params), shared_state()).await
}

/// `coord.fragos.get` — retrieve a specific FRAGO/AAR by hash.
///
/// # Errors
///
/// Returns error on missing params, not found, or I/O failure.
pub async fn fragos_get(params: &Value) -> Result<Value> {
    coord_handlers::coord_fragos_get(Some(params), shared_state()).await
}

/// `coord.waves.current` — return the current wave state.
///
/// # Errors
///
/// Returns error on I/O or manifest failure.
pub async fn waves_current(params: &Value) -> Result<Value> {
    coord_handlers::coord_waves_current(Some(params), shared_state()).await
}

/// `coord.waves.history` — return wave history from the coordination manifest.
///
/// # Errors
///
/// Returns error on I/O or manifest failure.
pub async fn waves_history(params: &Value) -> Result<Value> {
    coord_handlers::coord_waves_history(Some(params), shared_state()).await
}

/// `coord.heads.get` — return a specific gate's HEAD state.
///
/// # Errors
///
/// Returns error on missing params or I/O failure.
pub async fn heads_get(params: &Value) -> Result<Value> {
    coord_handlers::coord_heads_get(Some(params), shared_state()).await
}

/// `coord.heads.all` — return all gate HEAD states.
///
/// # Errors
///
/// Returns error on I/O or manifest failure.
pub async fn heads_all(params: &Value) -> Result<Value> {
    coord_handlers::coord_heads_all(Some(params), shared_state()).await
}

/// `coord.topology` — return mesh topology from gate heads.
///
/// # Errors
///
/// Returns error on I/O or manifest failure.
pub async fn topology(params: &Value) -> Result<Value> {
    coord_handlers::coord_topology(Some(params), shared_state()).await
}

/// `coord.depot.status` — return depot staleness and binary info.
///
/// # Errors
///
/// Returns error on I/O failure.
pub async fn depot_status(params: &Value) -> Result<Value> {
    coord_handlers::coord_depot_status(Some(params), shared_state()).await
}

/// `coord.provenance` — return provenance trail for an artifact.
///
/// # Errors
///
/// Returns error on missing params or I/O failure.
pub async fn provenance(params: &Value) -> Result<Value> {
    coord_handlers::coord_provenance(Some(params), shared_state()).await
}

/// `coord.ingest` — ingest wateringHole artifacts into the coordination CAS.
///
/// # Errors
///
/// Returns error on invalid params, base64 decode failure, or I/O failure.
pub async fn ingest(params: &Value) -> Result<Value> {
    coord_handlers::coord_ingest(Some(params), shared_state()).await
}
