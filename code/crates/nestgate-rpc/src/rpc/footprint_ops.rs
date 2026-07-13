// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Public stateless footPrint API.
//!
//! Wraps `unix_socket_server::footprint_handlers` behind a `StorageState`-free
//! interface so that **any crate** (including `nestgate-api`) can call
//! footPrint operations without constructing internal RPC state.
//!
//! Every function accepts a [`serde_json::Value`] parameter object and returns
//! a JSON-RPC–shaped [`serde_json::Value`] result.

use crate::rpc::unix_socket_server::{StorageState, footprint_handlers};
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
        StorageState::new().expect("StorageState initialization must not fail for footprint ops")
    })
}

/// `footprint.save` — save or update a project with a new CAS revision.
///
/// # Errors
///
/// Returns error on invalid params, base64 decode failure, or I/O failure.
pub async fn save(params: &Value) -> Result<Value> {
    footprint_handlers::footprint_save(Some(params), shared_state()).await
}

/// `footprint.get` — retrieve a project and optionally its current content.
///
/// # Errors
///
/// Returns error on missing params, not found, or I/O failure.
pub async fn get(params: &Value) -> Result<Value> {
    footprint_handlers::footprint_get(Some(params), shared_state()).await
}

/// `footprint.list` — list all projects (paginated).
///
/// # Errors
///
/// Returns error on I/O or manifest failure.
pub async fn list(params: &Value) -> Result<Value> {
    footprint_handlers::footprint_list(Some(params), shared_state()).await
}

/// `footprint.delete` — soft-delete a project from the manifest.
///
/// # Errors
///
/// Returns error on missing params, not found, or I/O failure.
pub async fn delete(params: &Value) -> Result<Value> {
    footprint_handlers::footprint_delete(Some(params), shared_state()).await
}

/// `footprint.history` — list revision history for a project.
///
/// # Errors
///
/// Returns error on missing params, not found, or I/O failure.
pub async fn history(params: &Value) -> Result<Value> {
    footprint_handlers::footprint_history(Some(params), shared_state()).await
}
