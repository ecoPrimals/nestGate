// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content domain semantic methods (`content.*`)
//!
//! Delegates to the canonical content-addressed storage handlers in
//! [`crate::rpc::unix_socket_server::content_handlers`] through a shared
//! [`crate::rpc::unix_socket_server::StorageState`].

use super::{MetadataBackend, SemanticRouter};
use crate::rpc::unix_socket_server::StorageState;
use nestgate_types::error::Result;
use serde_json::Value;
use std::sync::OnceLock;

use crate::rpc::unix_socket_server::content_federation_handlers;

/// Cached [`StorageState`] for content handler delegation.
///
/// Content operations are stateless filesystem ops keyed by `family_id` and
/// BLAKE3 hashes. A single shared `StorageState` avoids re-initializing
/// template/audit backends on every call while remaining correct because
/// content handlers only read `family_id` and `encryption` from it.
fn shared_state() -> &'static StorageState {
    static STATE: OnceLock<StorageState> = OnceLock::new();
    STATE.get_or_init(|| {
        #[expect(
            clippy::expect_used,
            reason = "StorageState::new only fails on unrecoverable I/O — crash is correct"
        )]
        StorageState::new().expect("StorageState initialization must not fail for content routing")
    })
}

use crate::rpc::unix_socket_server::content_handlers;

/// Route `content.put` → content-addressed store (BLAKE3 hash-as-key).
pub(super) async fn content_put(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_handlers::content_put(Some(&params), shared_state()).await
}

/// Route `content.get` → content-addressed retrieval by BLAKE3 hash.
pub(super) async fn content_get(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_handlers::content_get(Some(&params), shared_state()).await
}

/// Route `content.exists` → check if a BLAKE3 hash exists in the store.
pub(super) async fn content_exists(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_handlers::content_exists(Some(&params), shared_state()).await
}

/// Route `content.list` → enumerate content-addressed objects.
pub(super) async fn content_list(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_handlers::content_list(Some(&params), shared_state()).await
}

/// Route `content.publish` → store a manifest mapping paths to content hashes.
pub(super) async fn content_publish(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_handlers::content_publish(Some(&params), shared_state()).await
}

/// Route `content.resolve` → look up a content hash by path in a collection.
pub(super) async fn content_resolve(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_handlers::content_resolve(Some(&params), shared_state()).await
}

/// Route `content.promote` → alias one collection to another (atomic deploy).
pub(super) async fn content_promote(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_handlers::content_promote(Some(&params), shared_state()).await
}

/// Route `content.collections` → list all manifests/aliases within a family.
pub(super) async fn content_collections(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_handlers::content_collections(Some(&params), shared_state()).await
}

/// Route `content.fetch_heads` → read-only freshness check against remote repos.
pub(super) async fn content_fetch_heads(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_federation_handlers::content_fetch_heads(Some(&params), shared_state()).await
}

/// Route `content.push` → push local content to a remote.
pub(super) async fn content_push(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_federation_handlers::content_push(Some(&params), shared_state()).await
}

/// Route `content.replicate` → transfer content blobs to a remote `NestGate`.
pub(super) async fn content_replicate(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_federation_handlers::content_replicate(Some(&params), shared_state()).await
}

/// Route `content.sync` → cascade-pull from remote sources.
pub(super) async fn content_sync(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    content_federation_handlers::content_sync(Some(&params), shared_state()).await
}
