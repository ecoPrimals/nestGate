// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Public stateless content-addressed storage API.
//!
//! Wraps the `unix_socket_server::content_handlers` module behind a
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
use crate::rpc::{content_stream, storage_stream};
pub use content_handlers::RawContent;
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

/// Retrieve raw content bytes for direct HTTP serving (no base64 encoding).
///
/// Returns `Ok(None)` when the hash is not found (caller should return 404).
/// The [`RawContent`] struct carries decrypted bytes, the MIME type from the
/// `.meta.json` sidecar, and the BLAKE3 hash (for `ETag`).
///
/// # Errors
///
/// Returns error on invalid hash format, I/O failure, or decryption failure.
pub async fn get_raw(hash: &str, family_id: &str) -> Result<Option<RawContent>> {
    content_handlers::content_get_raw(hash, family_id, shared_state()).await
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

/// `content.replicate.pull` — pull content from a remote `NestGate` by CID list.
///
/// Verifies BLAKE3 integrity on every blob received. The hash IS the authority.
///
/// # Errors
///
/// Returns error on missing params, invalid CIDs, remote connection failure,
/// or BLAKE3 integrity mismatch.
pub async fn replicate_pull(params: &Value) -> Result<Value> {
    content_federation_handlers::content_replicate_pull(Some(params), shared_state()).await
}

/// `content.store_stream` — begin a chunked content upload session.
///
/// Caller supplies `family_id` in params (no per-connection state on HTTP).
///
/// # Errors
///
/// Returns error on invalid parameters or session creation failure.
pub async fn store_stream_begin(params: &Value) -> Result<Value> {
    content_stream::content_store_stream_begin(params.clone(), None).await
}

/// `content.store_stream_chunk` — append a chunk to an active upload session.
///
/// # Errors
///
/// Returns error if `stream_id` is invalid, expired, or data exceeds limits.
pub async fn store_stream_chunk(params: &Value) -> Result<Value> {
    content_stream::content_store_stream_chunk(params.clone()).await
}

/// `content.retrieve_stream` — begin a chunked content download session.
///
/// Caller supplies `family_id` in params (no per-connection state on HTTP).
///
/// # Errors
///
/// Returns error if the hash is not found or session creation fails.
pub async fn retrieve_stream_begin(params: &Value) -> Result<Value> {
    content_stream::content_retrieve_stream_begin(params.clone(), None).await
}

/// `content.retrieve_stream_chunk` / `storage.retrieve_stream_chunk` — read the
/// next range of bytes for an active download session.
///
/// # Errors
///
/// Returns error if `stream_id` is invalid, expired, or I/O fails.
pub async fn retrieve_stream_chunk(params: &Value) -> Result<Value> {
    storage_stream::storage_retrieve_stream_chunk(params.clone()).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn put_then_get_roundtrip() {
        let params = json!({"data": "aGVsbG8gd29ybGQ=", "family_id": "test-roundtrip"});
        let put_result = put(&params).await.expect("put should succeed");
        let hash = put_result["hash"].as_str().expect("hash field present");

        let get_params = json!({"hash": hash, "family_id": "test-roundtrip"});
        let get_result = get(&get_params).await.expect("get should succeed");
        assert_eq!(get_result["data"].as_str().unwrap(), "aGVsbG8gd29ybGQ=");
    }

    #[tokio::test]
    async fn exists_returns_false_for_unknown() {
        let params = json!({"hash": "0000000000000000000000000000000000000000000000000000000000000000", "family_id": "test-exists"});
        let result = exists(&params).await.expect("exists should succeed");
        assert_eq!(result["exists"].as_bool(), Some(false));
    }

    #[tokio::test]
    async fn list_returns_hashes_array() {
        let params = json!({"family_id": "test-list-empty"});
        let result = list(&params).await.expect("list should succeed");
        assert!(result["hashes"].is_array());
    }

    #[tokio::test]
    async fn fetch_heads_missing_params_errors() {
        let params = json!({});
        let err = fetch_heads(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn push_missing_path_errors() {
        let params = json!({});
        let err = push(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_missing_cids_errors() {
        let params = json!({"target": "/tmp/nonexistent.sock"});
        let err = replicate(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn sync_missing_repos_errors() {
        let params = json!({});
        let err = sync(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_pull_missing_cids_errors() {
        let params = json!({"source": "tcp://localhost:9999"});
        let err = replicate_pull(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_pull_missing_source_errors() {
        let params = json!({"cids": ["abc123"]});
        let err = replicate_pull(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn store_stream_begin_creates_session() {
        let params = json!({"family_id": "test-stream-http", "total_size": 1024});
        let result = store_stream_begin(&params).await;
        assert!(result.is_ok());
        let val = result.unwrap();
        assert!(val["stream_id"].is_string());
    }

    #[tokio::test]
    async fn store_stream_chunk_invalid_session_errors() {
        let params = json!({"stream_id": "nonexistent-session", "data": "aGVsbG8="});
        let err = store_stream_chunk(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn retrieve_stream_begin_missing_hash_errors() {
        let params = json!({"family_id": "test-retrieve-http"});
        let err = retrieve_stream_begin(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn retrieve_stream_chunk_invalid_session_errors() {
        let params = json!({"stream_id": "nonexistent-session"});
        let err = retrieve_stream_chunk(&params).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn get_raw_returns_bytes_and_content_type() {
        use base64::Engine as _;
        let family = format!("test-raw-{}", uuid::Uuid::new_v4());
        let payload = b"raw content serving test payload";
        let b64 = base64::engine::general_purpose::STANDARD.encode(payload);

        let put_result = put(&json!({
            "data": b64, "family_id": family, "content_type": "text/plain"
        }))
        .await
        .unwrap();
        let hash = put_result["hash"].as_str().unwrap();

        let raw = get_raw(hash, &family).await.unwrap().unwrap();
        assert_eq!(raw.data, payload);
        assert_eq!(raw.content_type.as_deref(), Some("text/plain"));
        assert_eq!(raw.hash, hash);
    }

    #[tokio::test]
    async fn get_raw_returns_none_for_missing_hash() {
        let family = format!("test-raw-missing-{}", uuid::Uuid::new_v4());
        let fake = "b".repeat(64);
        let result = get_raw(&fake, &family).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn get_raw_invalid_hash_errors() {
        let err = get_raw("short", "family").await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn publish_resolve_facade_roundtrip() {
        use base64::Engine as _;
        let family = format!("test-facade-pub-{}", uuid::Uuid::new_v4());
        let data = base64::engine::general_purpose::STANDARD.encode(b"facade publish");
        let put_r = put(&json!({"data": data, "family_id": &family}))
            .await
            .unwrap();
        let hash = put_r["hash"].as_str().unwrap();

        let pub_r = publish(&json!({
            "family_id": &family,
            "collection": "test-facade",
            "manifest": { "/index.html": hash }
        }))
        .await
        .unwrap();
        assert!(pub_r.get("collection").is_some() || pub_r.get("published").is_some());

        let res_r = resolve(&json!({
            "family_id": &family,
            "collection": "test-facade",
            "path": "/index.html"
        }))
        .await
        .unwrap();
        assert!(res_r.get("hash").is_some() || res_r.get("data").is_some());
    }

    #[tokio::test]
    async fn promote_facade_roundtrip() {
        use base64::Engine as _;
        let family = format!("test-facade-promote-{}", uuid::Uuid::new_v4());
        let data = base64::engine::general_purpose::STANDARD.encode(b"promote test");
        let put_r = put(&json!({"data": data, "family_id": &family}))
            .await
            .unwrap();
        let hash = put_r["hash"].as_str().unwrap();

        publish(&json!({
            "family_id": &family,
            "collection": "staging",
            "manifest": { "/app.js": hash }
        }))
        .await
        .unwrap();

        let promo_r = promote(&json!({
            "family_id": &family,
            "collection": "staging",
            "alias": "production"
        }))
        .await
        .unwrap();
        assert!(promo_r.get("alias").is_some() || promo_r.get("promoted").is_some(),);
    }

    #[tokio::test]
    async fn collections_facade_returns_array() {
        let family = format!("test-facade-coll-{}", uuid::Uuid::new_v4());
        let r = collections(&json!({"family_id": &family})).await.unwrap();
        assert!(r.get("collections").is_some());
    }
}
