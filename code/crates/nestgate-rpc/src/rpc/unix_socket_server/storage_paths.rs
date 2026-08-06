// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared storage path helpers, namespace validation, and family resolution.
//!
//! These utilities are used by [`super::storage_handlers`],
//! [`super::blob_handlers`], [`super::external_handlers`],
//! [`super::bonding_handlers`], and [`super::session_handlers`].

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_config::config::substrate_tiers::SubstrateTiers;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
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
///
/// Uses the hot (warm) tier when `NESTGATE_WARM_PATHS` is configured,
/// otherwise falls back to `get_storage_base_path()`.
pub fn content_cas_path(family_id: &str, blake3_hex: &str) -> PathBuf {
    cas_content_layout(&cas_hot_root(), family_id, blake3_hex)
}

/// Resolve a CAS object path for **reads**, checking hot tier first, then cold
/// tiers, then the legacy single-path root.
///
/// Returns `Some(path)` if the object exists on any tier, `None` otherwise.
/// When `NESTGATE_WARM_PATHS`/`NESTGATE_COLD_PATHS` are not configured,
/// only the legacy `get_storage_base_path()` is checked — zero overhead.
#[must_use]
pub fn resolve_cas_object(family_id: &str, blake3_hex: &str) -> Option<PathBuf> {
    let tiers = cached_tiers();
    let hot = cas_content_layout(&cas_hot_root(), family_id, blake3_hex);
    if hot.exists() {
        return Some(hot);
    }
    for cold_mount in &tiers.cold {
        let cold_path = cas_content_layout(&cold_mount.path, family_id, blake3_hex);
        if cold_path.exists() {
            return Some(cold_path);
        }
    }
    let legacy = cas_content_layout(&get_storage_base_path(), family_id, blake3_hex);
    if legacy != hot && legacy.exists() {
        return Some(legacy);
    }
    None
}

/// Build CAS content path under a given root.
fn cas_content_layout(root: &Path, family_id: &str, blake3_hex: &str) -> PathBuf {
    root.join("datasets")
        .join(family_id)
        .join("_content")
        .join(&blake3_hex[..2])
        .join(blake3_hex)
}

/// Hot tier root for CAS writes.
///
/// When `NESTGATE_WARM_PATHS` is explicitly set, uses the first warm path.
/// Otherwise defers to `get_storage_base_path()` which reads env per call,
/// ensuring tests that modify `NESTGATE_STORAGE_PATH` work correctly.
fn cas_hot_root() -> PathBuf {
    if let Ok(warm) = std::env::var("NESTGATE_WARM_PATHS")
        && let Some(first) = warm.split(':').find(|s| !s.is_empty())
    {
        let p = PathBuf::from(first);
        if p.exists() {
            return p;
        }
    }
    get_storage_base_path()
}

/// Cached `SubstrateTiers` (discovered once from environment).
fn cached_tiers() -> &'static SubstrateTiers {
    static TIERS: OnceLock<SubstrateTiers> = OnceLock::new();
    TIERS.get_or_init(SubstrateTiers::from_environment)
}

/// Check whether dual-path CAS is active (i.e. cold tiers are configured).
#[cfg(test)]
fn is_dual_path_active() -> bool {
    !cached_tiers().cold.is_empty()
}

/// Hot tier root path — exposed for diagnostics.
#[cfg(test)]
fn cas_hot_root_path() -> PathBuf {
    cas_hot_root()
}

/// Return all `_content/` directories for a family across hot, cold, and legacy tiers.
///
/// Used by `content.list` and `content.query` to enumerate objects across
/// all physical storage tiers. Deduplicates paths that resolve to the same root.
#[must_use]
pub fn cas_content_dirs(family_id: &str) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let hot = cas_hot_root()
        .join("datasets")
        .join(family_id)
        .join("_content");
    // Note: hot is computed from cas_hot_root() which re-evaluates per call
    seen.insert(hot.clone());
    dirs.push(hot);

    for cold_mount in &cached_tiers().cold {
        let cold = cold_mount
            .path
            .join("datasets")
            .join(family_id)
            .join("_content");
        if seen.insert(cold.clone()) {
            dirs.push(cold);
        }
    }

    let legacy = get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_content");
    if seen.insert(legacy.clone()) {
        dirs.push(legacy);
    }

    dirs
}

/// Check available capacity on the warm (hot) tier. Returns `(available_bytes, total_bytes)`.
///
/// Used for high-water-mark backpressure: reject writes when the warm tier
/// is nearly full rather than filling the filesystem to 100%.
pub fn warm_tier_capacity() -> (u64, u64) {
    let hot = cas_hot_root();
    match rustix::fs::statvfs(&hot) {
        Ok(st) => {
            let avail = st.f_bavail * st.f_frsize;
            let total = st.f_blocks * st.f_frsize;
            (avail, total)
        }
        Err(_) => (u64::MAX, u64::MAX),
    }
}

/// Minimum free bytes on the warm tier before `content.put` starts rejecting writes.
/// Default 10 GB; override with `NESTGATE_WARM_MIN_FREE_BYTES`.
pub fn warm_tier_min_free() -> u64 {
    std::env::var("NESTGATE_WARM_MIN_FREE_BYTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10 * 1024 * 1024 * 1024) // 10 GB
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

/// Characters reserved on NTFS that are invalid in path components.
///
/// Includes `:` (ADS separator), `<`, `>`, `"`, `|`, `?`, `*`.
/// Forward- and back-slash are checked separately as path separators.
const NTFS_RESERVED_CHARS: &[char] = &[':', '<', '>', '"', '|', '?', '*'];

/// Maximum byte length for a single path component.
///
/// NTFS and most Unix filesystems cap individual directory/file names at 255
/// bytes. We enforce 200 to leave margin for suffixes (`.meta.json`, `.part`).
const MAX_SEGMENT_BYTES: usize = 200;

/// Validate a path segment for cross-platform safety.
///
/// Rejects:
/// - empty names
/// - path separators (`/`, `\`) and traversal (`..`)
/// - leading `.` or `_` (reserved for internal dirs like `_content`, `_blobs`)
/// - NTFS-reserved characters (`:`, `<`, `>`, `"`, `|`, `?`, `*`)
/// - trailing `.` or space (silently stripped by NTFS, causing misrouting)
/// - segments exceeding [`MAX_SEGMENT_BYTES`] (200)
pub fn validate_path_segment(name: &str, field: &'static str) -> Result<()> {
    if name.is_empty() {
        return Err(NestGateError::invalid_input_with_field(
            field,
            "must not be empty",
        ));
    }
    if name.len() > MAX_SEGMENT_BYTES {
        return Err(NestGateError::invalid_input_with_field(
            field,
            "exceeds maximum length (200 bytes)",
        ));
    }
    if name.contains('/')
        || name.contains('\\')
        || name.contains("..")
        || name.starts_with('.')
        || name.starts_with('_')
    {
        return Err(NestGateError::invalid_input_with_field(
            field,
            "must be a non-empty simple name without path separators; \
             cannot start with '.' or '_'",
        ));
    }
    if name.ends_with('.') || name.ends_with(' ') {
        return Err(NestGateError::invalid_input_with_field(
            field,
            "must not end with '.' or space (invalid on NTFS)",
        ));
    }
    if name.contains(NTFS_RESERVED_CHARS) {
        return Err(NestGateError::invalid_input_with_field(
            field,
            "contains characters reserved on Windows (: < > \" | ? *)",
        ));
    }
    Ok(())
}

/// Extract and validate the optional `namespace` parameter.
///
/// Returns `Ok(None)` when omitted (backward-compatible flat layout).
/// Returns `Ok(Some(ns))` when present and valid (namespaced layout).
/// Returns `Err` when present but fails [`validate_path_segment`].
pub(in crate::rpc::unix_socket_server) fn extract_namespace(
    params: &Value,
) -> Result<Option<&str>> {
    let Some(ns) = params.get("namespace").and_then(Value::as_str) else {
        return Ok(None);
    };
    validate_path_segment(ns, "namespace")?;
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
        validate_path_segment(fid, "family_id")?;
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

/// Validate that a URL has an allowed scheme (HTTPS by default, HTTP if permitted).
///
/// Shared between `content.fetch` and `storage.fetch_external` to enforce
/// consistent URL-scheme policy.
pub(in crate::rpc::unix_socket_server) fn validate_fetch_url(
    url: &str,
    allow_http: bool,
) -> Result<()> {
    let parsed = url::Url::parse(url)
        .map_err(|e| NestGateError::invalid_input_with_field("url", format!("invalid URL: {e}")))?;
    match parsed.scheme() {
        "https" => Ok(()),
        "http" if allow_http => {
            tracing::warn!("HTTP (insecure) requested for {url}");
            Ok(())
        }
        scheme => Err(NestGateError::invalid_input_with_field(
            "url",
            format!("scheme '{scheme}' not allowed — use https"),
        )),
    }
}

/// Build a `ureq::Agent` with the standard `NestGate` configuration.
///
/// Uses `rustls-rustcrypto` (pure Rust TLS) and the standard `NestGate/<version>`
/// user agent. Shared between `content.fetch` and `storage.fetch_external`.
pub(in crate::rpc::unix_socket_server) fn build_http_agent(
    timeout: std::time::Duration,
    allow_http: bool,
) -> ureq::Agent {
    let _ = rustls_rustcrypto::provider().install_default();
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(timeout))
        .https_only(!allow_http)
        .build();
    ureq::Agent::new_with_config(config)
}

/// Standard `NestGate` HTTP User-Agent header value.
#[must_use]
pub(in crate::rpc::unix_socket_server) fn http_user_agent() -> String {
    format!("NestGate/{}", env!("CARGO_PKG_VERSION"))
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

    #[test]
    fn validate_path_segment_accepts_normal() {
        assert!(validate_path_segment("my-family", "test").is_ok());
        assert!(validate_path_segment("v1", "test").is_ok());
        assert!(validate_path_segment("data-2026", "test").is_ok());
    }

    #[test]
    fn validate_path_segment_rejects_ntfs_reserved() {
        assert!(validate_path_segment("a:b", "test").is_err());
        assert!(validate_path_segment("a<b", "test").is_err());
        assert!(validate_path_segment("a>b", "test").is_err());
        assert!(validate_path_segment("a\"b", "test").is_err());
        assert!(validate_path_segment("a|b", "test").is_err());
        assert!(validate_path_segment("a?b", "test").is_err());
        assert!(validate_path_segment("a*b", "test").is_err());
    }

    #[test]
    fn validate_path_segment_rejects_trailing_dot_or_space() {
        assert!(validate_path_segment("name.", "test").is_err());
        assert!(validate_path_segment("name ", "test").is_err());
    }

    #[test]
    fn validate_path_segment_rejects_too_long() {
        let long = "a".repeat(201);
        assert!(validate_path_segment(&long, "test").is_err());
        let ok = "a".repeat(200);
        assert!(validate_path_segment(&ok, "test").is_ok());
    }

    #[test]
    fn validate_path_segment_rejects_traversal() {
        assert!(validate_path_segment("../etc", "test").is_err());
        assert!(validate_path_segment("a/b", "test").is_err());
        assert!(validate_path_segment("a\\b", "test").is_err());
    }

    #[test]
    fn validate_path_segment_rejects_reserved_prefixes() {
        assert!(validate_path_segment(".hidden", "test").is_err());
        assert!(validate_path_segment("_internal", "test").is_err());
    }

    #[test]
    fn validate_path_segment_rejects_empty() {
        assert!(validate_path_segment("", "test").is_err());
    }

    #[test]
    fn dual_path_helpers_do_not_panic() {
        let _active = is_dual_path_active();
        let _hot = cas_hot_root_path();
        let dirs = cas_content_dirs("test-family");
        assert!(!dirs.is_empty(), "should return at least one content dir");
    }

    #[test]
    fn resolve_cas_object_returns_none_for_missing() {
        let hash = "a".repeat(64);
        assert!(resolve_cas_object("nonexistent-family-xyzzy", &hash).is_none());
    }
}
