// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content-addressed storage handlers — BLAKE3 hash-as-key with automatic dedup.
//!
//! Objects are immutable: the BLAKE3 hash of the content *is* the key. Storing
//! the same bytes twice is a no-op that returns `deduplicated: true`.
//!
//! Filesystem layout:
//!   `{base}/datasets/{family}/_content/{hex[..2]}/{hex}`
//!   `{base}/datasets/{family}/_content/{hex[..2]}/{hex}.meta.json`
//!
//! ## Provenance
//!
//! `content.put` accepts optional provenance fields (`source`, `pipeline`,
//! `stored_by`) which are persisted in the `.meta.json` sidecar alongside
//! `content_type` and `stored_at`. Both `content.get` and `content.exists`
//! return all available provenance metadata, making `content.get` the
//! canonical artifact provenance query (no separate method needed).

mod cas;
mod manifest;
mod raw;

pub use cas::{content_exists, content_get, content_list, content_put};
pub use manifest::{content_collections, content_promote, content_publish, content_resolve};
pub use raw::{RawContent, content_get_raw};

use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

/// Provenance fields carried in `.meta.json` sidecars.
///
/// Includes lineage fields (`parent_hash`, `derivation_depth`) for provenance
/// chain support (Nest provenance depth 5+).
pub const SIDECAR_PROVENANCE_KEYS: &[&str] = &[
    "content_type",
    "stored_at",
    "source",
    "pipeline",
    "stored_by",
    "parent_hash",
    "derivation_depth",
];

/// Merge non-null provenance fields from a sidecar JSON object into `resp`.
pub fn merge_sidecar_fields(resp: &mut Value, sidecar: &Value) {
    for key in SIDECAR_PROVENANCE_KEYS {
        if let Some(v) = sidecar.get(*key)
            && !v.is_null()
        {
            resp[*key] = v.clone();
        }
    }
}

pub fn validate_blake3_hex(hash: &str) -> Result<()> {
    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(NestGateError::invalid_input_with_field(
            "hash",
            "must be a 64-character lowercase hex BLAKE3 digest",
        ));
    }
    Ok(())
}

pub fn validate_collection_name(name: &str) -> Result<()> {
    if name.is_empty()
        || name.contains('/')
        || name.contains('\\')
        || name.contains("..")
        || name.starts_with('.')
    {
        return Err(NestGateError::invalid_input_with_field(
            "collection",
            "must be a non-empty simple name without path separators",
        ));
    }
    Ok(())
}

/// Decrypt content bytes if an encrypted envelope is detected.
pub fn maybe_decrypt(raw_data: Vec<u8>, state: &super::StorageState) -> Result<Vec<u8>> {
    if crate::rpc::storage_encryption::StorageEncryption::is_encrypted_envelope(&raw_data)
        && let Some(ref enc) = state.encryption
    {
        return enc.decrypt(&raw_data);
    }
    Ok(raw_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn validate_blake3_hex_accepts_valid() {
        let valid = "a".repeat(64);
        assert!(validate_blake3_hex(&valid).is_ok());
    }

    #[test]
    fn validate_blake3_hex_rejects_short() {
        assert!(validate_blake3_hex("abcdef").is_err());
    }

    #[test]
    fn validate_blake3_hex_rejects_non_hex() {
        let bad = "g".repeat(64);
        assert!(validate_blake3_hex(&bad).is_err());
    }

    #[test]
    fn validate_blake3_hex_rejects_empty() {
        assert!(validate_blake3_hex("").is_err());
    }

    #[test]
    fn validate_collection_name_accepts_normal() {
        assert!(validate_collection_name("my-collection").is_ok());
        assert!(validate_collection_name("v1").is_ok());
    }

    #[test]
    fn validate_collection_name_rejects_empty() {
        assert!(validate_collection_name("").is_err());
    }

    #[test]
    fn validate_collection_name_rejects_slash() {
        assert!(validate_collection_name("a/b").is_err());
        assert!(validate_collection_name("a\\b").is_err());
    }

    #[test]
    fn validate_collection_name_rejects_dot_dot() {
        assert!(validate_collection_name("..").is_err());
        assert!(validate_collection_name("a..b").is_err());
    }

    #[test]
    fn validate_collection_name_rejects_leading_dot() {
        assert!(validate_collection_name(".hidden").is_err());
    }

    #[test]
    fn merge_sidecar_fields_copies_known_keys() {
        let mut resp = json!({"hash": "abc"});
        let sidecar = json!({
            "content_type": "text/plain",
            "stored_at": "2026-06-20T00:00:00Z",
            "source": "pipeline-x",
            "pipeline": "ci",
            "stored_by": "agent"
        });
        merge_sidecar_fields(&mut resp, &sidecar);
        assert_eq!(resp["content_type"], "text/plain");
        assert_eq!(resp["source"], "pipeline-x");
        assert_eq!(resp["stored_by"], "agent");
    }

    #[test]
    fn merge_sidecar_fields_skips_null() {
        let mut resp = json!({"hash": "abc"});
        let sidecar = json!({"content_type": null, "source": "src"});
        merge_sidecar_fields(&mut resp, &sidecar);
        assert!(resp.get("content_type").is_none());
        assert_eq!(resp["source"], "src");
    }

    #[test]
    fn merge_sidecar_fields_ignores_unknown_keys() {
        let mut resp = json!({});
        let sidecar = json!({"unknown_key": "value"});
        merge_sidecar_fields(&mut resp, &sidecar);
        assert!(resp.get("unknown_key").is_none());
    }
}
