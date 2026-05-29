// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Public stateless content-addressed storage API.
//!
//! Wraps [`crate::rpc::unix_socket_server::content_handlers`] behind a
//! `StorageState`-free interface so that **any crate** (including `nestgate-api`)
//! can call content operations without constructing internal RPC state.
//!
//! Every function accepts a [`serde_json::Value`] parameter object and returns
//! a JSON-RPC–shaped [`serde_json::Value`] result.
//!
//! # Errors
//!
//! All functions return [`nestgate_types::error::NestGateError`] on invalid
//! parameters, missing content, or I/O failure.

use crate::rpc::unix_socket_server::{StorageState, content_federation_handlers, content_handlers};
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
        StorageState::new().expect("StorageState initialization must not fail for content ops")
    })
}

/// `content.put` — store content-addressed data (BLAKE3 hash-as-key, automatic dedup).
///
/// # Errors
///
/// Returns error on invalid parameters, I/O failure, or encryption failure.
pub async fn put(params: &Value) -> Result<Value> {
    content_handlers::content_put(Some(params), shared_state()).await
}

/// `content.get` — retrieve content by BLAKE3 hash.
///
/// # Errors
///
/// Returns error if the hash is invalid, not found, or I/O fails.
pub async fn get(params: &Value) -> Result<Value> {
    content_handlers::content_get(Some(params), shared_state()).await
}

/// `content.exists` — check if a BLAKE3 hash exists in the content store.
///
/// # Errors
///
/// Returns error on invalid hash or I/O failure.
pub async fn exists(params: &Value) -> Result<Value> {
    content_handlers::content_exists(Some(params), shared_state()).await
}

/// `content.list` — enumerate content-addressed objects.
///
/// # Errors
///
/// Returns error on I/O failure reading the content directory.
pub async fn list(params: &Value) -> Result<Value> {
    content_handlers::content_list(Some(params), shared_state()).await
}

/// `content.publish` — store a manifest mapping URL paths to content hashes.
///
/// # Errors
///
/// Returns error on invalid parameters, missing referenced hashes, or I/O failure.
pub async fn publish(params: &Value) -> Result<Value> {
    content_handlers::content_publish(Some(params), shared_state()).await
}

/// `content.resolve` — look up a content hash by path within a collection.
///
/// # Errors
///
/// Returns error if the collection or manifest is missing, corrupt, or I/O fails.
pub async fn resolve(params: &Value) -> Result<Value> {
    content_handlers::content_resolve(Some(params), shared_state()).await
}

/// `content.promote` — alias one collection name to another (atomic deploy).
///
/// # Errors
///
/// Returns error if the target collection doesn't exist or I/O fails.
pub async fn promote(params: &Value) -> Result<Value> {
    content_handlers::content_promote(Some(params), shared_state()).await
}

/// `content.collections` — list all manifests/aliases within a family.
///
/// # Errors
///
/// Returns error on I/O failure reading the manifests directory.
pub async fn collections(params: &Value) -> Result<Value> {
    content_handlers::content_collections(Some(params), shared_state()).await
}

/// `content.fetch_heads` — read-only freshness check against remote repos.
///
/// # Errors
///
/// Returns error on missing params or if git is unavailable.
pub async fn fetch_heads(params: &Value) -> Result<Value> {
    content_federation_handlers::content_fetch_heads(Some(params), shared_state()).await
}

/// `content.push` — push local content to a remote.
///
/// # Errors
///
/// Returns error on missing params or if git is unavailable.
pub async fn push(params: &Value) -> Result<Value> {
    content_federation_handlers::content_push(Some(params), shared_state()).await
}

/// `content.replicate` — transfer content blobs to a remote `NestGate`.
///
/// # Errors
///
/// Returns error on missing params, invalid CIDs, or remote connection failure.
pub async fn replicate(params: &Value) -> Result<Value> {
    content_federation_handlers::content_replicate(Some(params), shared_state()).await
}

/// `content.sync` — cascade-pull from remote sources.
///
/// # Errors
///
/// Returns error on missing params or if git is unavailable.
pub async fn sync(params: &Value) -> Result<Value> {
    content_federation_handlers::content_sync(Some(params), shared_state()).await
}
