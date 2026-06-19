// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared storage path helpers, namespace validation, and family resolution.
//!
//! These utilities are used by [`super::storage_handlers`],
//! [`super::blob_handlers`], [`super::external_handlers`],
//! [`super::bonding_handlers`], and [`super::session_handlers`].

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tracing::debug;

use super::StorageState;

/// Compute the BLAKE3 content hash of `data` and return it as a lowercase hex string.
///
/// This is the single canonical hashing entry point for content-addressed storage.
/// Every CAS write/verify path must use this to avoid drift if the hashing
/// algorithm or encoding changes.
#[must_use]
pub fn content_hash_hex(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}

/// Build the filesystem path for a content-addressed object (BLAKE3 hash as key).
///
/// Layout: `{base}/datasets/{family}/_content/{hex[..2]}/{hex}`
/// The 2-char prefix directory prevents flat-directory blowup at scale.
pub fn content_cas_path(family_id: &str, blake3_hex: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_content")
        .join(&blake3_hex[..2])
        .join(blake3_hex)
}

/// Build the filesystem path for a key in a family's dataset.
///
/// When `namespace` is `Some`, uses the isomorphic layout:
///   `{base}/datasets/{family}/{namespace}/{key}`
/// When `None`, uses the flat legacy layout:
///   `{base}/datasets/{family}/{key}`
pub(in crate::rpc::unix_socket_server) fn dataset_key_path(
    family_id: &str,
    namespace: Option<&str>,
    key: &str,
) -> PathBuf {
    let base = get_storage_base_path().join("datasets").join(family_id);
    namespace.map_or_else(|| base.join(key), |ns| base.join(ns).join(key))
}

/// Build the filesystem path for a binary blob.
///
/// When `namespace` is `Some`, uses the isomorphic layout:
///   `{base}/datasets/{family}/{namespace}/_blobs/{key}`
/// When `None`, uses the flat legacy layout:
///   `{base}/datasets/{family}/_blobs/{key}`
pub(in crate::rpc::unix_socket_server) fn blob_key_path(
    family_id: &str,
    namespace: Option<&str>,
    key: &str,
) -> PathBuf {
    let base = get_storage_base_path().join("datasets").join(family_id);
    namespace.map_or_else(
        || base.join("_blobs").join(key),
        |ns| base.join(ns).join("_blobs").join(key),
    )
}

/// Extract and validate the optional `namespace` parameter.
///
/// Returns `Ok(None)` when omitted (backward-compatible flat layout).
/// Returns `Ok(Some(ns))` when present and valid (namespaced layout).
/// Returns `Err` when present but contains path separators, `..`, or
/// starts with `.` or `_` (reserved for internal directories like `_blobs`).
pub(in crate::rpc::unix_socket_server) fn extract_namespace(
    params: &Value,
) -> Result<Option<&str>> {
    let Some(ns) = params.get("namespace").and_then(Value::as_str) else {
        return Ok(None);
    };
    if ns.is_empty()
        || ns.contains('/')
        || ns.contains('\\')
        || ns.contains("..")
        || ns.starts_with('.')
        || ns.starts_with('_')
    {
        return Err(NestGateError::invalid_input_with_field(
            "namespace",
            "must be a non-empty simple name without path separators; \
             cannot start with '.' or '_'",
        ));
    }
    Ok(Some(ns))
}

/// Ensure all parent directories of `path` exist.
pub(in crate::rpc::unix_socket_server) async fn ensure_parent_dirs(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            NestGateError::io_error(format!(
                "Failed to create directories {}: {e}",
                parent.display()
            ))
        })?;
    }
    Ok(())
}

/// Legacy alias — delegates to [`content_cas_path`].
pub(in crate::rpc::unix_socket_server) fn content_key_path(
    family_id: &str,
    blake3_hex: &str,
) -> PathBuf {
    content_cas_path(family_id, blake3_hex)
}

/// Build the filesystem path for a content manifest (versioned collection).
///
/// Layout: `{base}/datasets/{family}/_manifests/{collection}.json`
pub(in crate::rpc::unix_socket_server) fn manifest_path(
    family_id: &str,
    collection: &str,
) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_manifests")
        .join(format!("{collection}.json"))
}

/// Resolve `family_id`: explicit param wins, then server's socket-scoped default.
///
/// In NUCLEUS compositions, callers connect via a family-scoped socket
/// (`nestgate-{family}.sock`) and can omit `family_id` entirely — the server
/// already knows the family context from `NESTGATE_FAMILY_ID` / `FAMILY_ID`.
///
/// # Errors
///
/// Returns [`NestGateError`] only when both the request params and the server
/// state lack a `family_id` (standalone mode with no env var set).
pub(in crate::rpc::unix_socket_server) fn resolve_family_id<'a>(
    params: &'a Value,
    state: &'a StorageState,
) -> Result<&'a str> {
    if let Some(fid) = params["family_id"].as_str() {
        return Ok(fid);
    }
    if let Some(ref fid) = state.family_id {
        debug!(
            family_id = fid.as_str(),
            "family_id omitted in request, using server default"
        );
        return Ok(fid.as_str());
    }
    Err(NestGateError::invalid_input_with_field(
        "family_id",
        "family_id required — set NESTGATE_FAMILY_ID or pass family_id in params",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn content_hash_hex_deterministic() {
        let h1 = content_hash_hex(b"hello world");
        let h2 = content_hash_hex(b"hello world");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64, "BLAKE3 hex is always 64 chars");
    }

    #[test]
    fn content_hash_hex_differs_for_different_input() {
        let h1 = content_hash_hex(b"aaa");
        let h2 = content_hash_hex(b"bbb");
        assert_ne!(h1, h2);
    }

    #[test]
    fn content_hash_hex_empty_input() {
        let h = content_hash_hex(&[]);
        assert_eq!(h.len(), 64);
        assert_ne!(h, content_hash_hex(b"notempty"));
    }

    #[test]
    fn content_cas_path_layout() {
        let hash = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
        let path = content_cas_path("my-family", hash);
        let s = path.to_string_lossy();
        assert!(s.contains("datasets/my-family/_content/ab/"));
        assert!(s.ends_with(hash));
    }

    #[test]
    fn content_key_path_delegates_to_cas_path() {
        let hash = "ff00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";
        assert_eq!(content_key_path("fam", hash), content_cas_path("fam", hash),);
    }

    #[test]
    fn extract_namespace_valid() {
        let params = json!({"namespace": "myns"});
        assert_eq!(extract_namespace(&params).unwrap(), Some("myns"));
    }

    #[test]
    fn extract_namespace_absent() {
        let params = json!({});
        assert_eq!(extract_namespace(&params).unwrap(), None);
    }

    #[test]
    fn extract_namespace_rejects_path_traversal() {
        let params = json!({"namespace": "../etc"});
        assert!(extract_namespace(&params).is_err());
    }

    #[test]
    fn extract_namespace_rejects_slash() {
        let params = json!({"namespace": "a/b"});
        assert!(extract_namespace(&params).is_err());
    }

    #[test]
    fn extract_namespace_rejects_dot_prefix() {
        let params = json!({"namespace": ".hidden"});
        assert!(extract_namespace(&params).is_err());
    }

    #[test]
    fn extract_namespace_rejects_underscore_prefix() {
        let params = json!({"namespace": "_internal"});
        assert!(extract_namespace(&params).is_err());
    }

    #[test]
    fn extract_namespace_rejects_empty() {
        let params = json!({"namespace": ""});
        assert!(extract_namespace(&params).is_err());
    }

    #[test]
    fn manifest_path_layout() {
        let path = manifest_path("fam", "staging");
        let s = path.to_string_lossy();
        assert!(s.contains("datasets/fam/_manifests/staging.json"));
    }
}
