// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! HTTP raw content adapter: `RawContent` and `content_get_raw`.

use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

use super::super::StorageState;
use super::super::storage_paths::content_key_path;
use super::{maybe_decrypt, validate_blake3_hex};

/// Raw content retrieval result for direct HTTP serving (no base64 encoding).
#[derive(Debug)]
pub struct RawContent {
    /// Decrypted content bytes.
    pub data: Vec<u8>,
    /// MIME type from `.meta.json` sidecar (if stored).
    pub content_type: Option<String>,
    /// BLAKE3 hash (for `ETag`).
    pub hash: String,
}

/// Retrieve raw content bytes by BLAKE3 hash — for direct HTTP serving.
///
/// Unlike [`super::cas::content_get`] which returns base64-encoded JSON, this
/// returns raw decrypted bytes and the content-type from the `.meta.json`
/// sidecar, suitable for `GET /content/:hash` with proper HTTP headers.
///
/// Returns `Ok(None)` when the hash is not found (caller maps to 404).
///
/// # Errors
///
/// Returns error on invalid hash format, I/O failure, or decryption failure.
pub async fn content_get_raw(
    hash: &str,
    family_id: &str,
    state: &StorageState,
) -> Result<Option<RawContent>> {
    validate_blake3_hex(hash)?;

    let object_path = content_key_path(family_id, hash);
    if !object_path.exists() {
        return Ok(None);
    }

    let raw_data = tokio::fs::read(&object_path)
        .await
        .map_err(|e| NestGateError::io_error(format!("Failed to read content {hash}: {e}")))?;

    let plain = maybe_decrypt(raw_data, state)?;

    let meta_path = object_path.with_extension("meta.json");
    let content_type: Option<String> = if meta_path.exists() {
        tokio::fs::read(&meta_path)
            .await
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            .and_then(|v| v["content_type"].as_str().map(String::from))
    } else {
        None
    };

    Ok(Some(RawContent {
        data: plain,
        content_type,
        hash: hash.to_owned(),
    }))
}
