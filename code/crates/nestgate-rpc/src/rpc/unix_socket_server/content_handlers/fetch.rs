// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `content.fetch` — HTTP(S) GET → BLAKE3 hash → CAS store in one atomic step.
//!
//! Unlike `storage.fetch_external` (which caches to `_external/`), this handler
//! writes directly to the content-addressed store at `_content/{hash[..2]}/{hash}`.
//! The result is identical to fetching a file externally and then calling
//! `content.put`, but without the intermediate copy.
//!
//! Rate limiting is enforced via chunked reads with a configurable bytes-per-second
//! cap. This is the primal-level bandwidth governance mechanism — the signal graph
//! calls `topology.bandwidth.request` before dispatching this handler.
//!
//! # Parameters
//!
//! | Field | Type | Required | Description |
//! |-------|------|----------|-------------|
//! | `url` | string | yes | URL to fetch (HTTPS required unless `allow_http` set) |
//! | `rate_limit_mbps` | number | no | Max download speed in Mbps (default: unlimited) |
//! | `timeout_secs` | number | no | Request timeout in seconds (default: 3600) |
//! | `allow_http` | bool | no | Allow plain HTTP (default: false) |
//! | `expected_hash` | string | no | Expected BLAKE3 hash — fails if mismatch |
//!
//! # Returns
//!
//! ```json
//! {
//!   "hash": "abcdef...",
//!   "size": 12345,
//!   "url": "https://...",
//!   "content_type": "application/gzip",
//!   "stored": true,
//!   "deduplicated": false,
//!   "fetched_at": "2026-08-03T12:00:00Z"
//! }
//! ```

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::io::Read;
use tracing::debug;

use super::super::StorageState;
use super::super::storage_paths::{content_key_path, ensure_parent_dirs, resolve_family_id};

/// Chunk size for rate-limited reads (64 KB).
const CHUNK_SIZE: usize = 64 * 1024;

/// `content.fetch` — fetch URL, BLAKE3 hash, store to CAS atomically.
pub async fn content_fetch(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let url = params["url"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("url", "url (string) required"))?;

    let allow_http = params["allow_http"].as_bool().unwrap_or(false);
    validate_fetch_url(url, allow_http)?;

    let rate_limit_mbps = params["rate_limit_mbps"].as_f64();
    let timeout_secs = params["timeout_secs"].as_u64().unwrap_or(3600);
    let expected_hash = params["expected_hash"].as_str().map(String::from);
    let family_id = resolve_family_id(params, state)?;

    let url_owned = url.to_owned();
    let family_owned = family_id.to_owned();

    let result = tokio::task::spawn_blocking(move || {
        do_fetch_to_cas(
            &url_owned,
            &family_owned,
            rate_limit_mbps,
            timeout_secs,
            expected_hash.as_deref(),
        )
    })
    .await
    .map_err(|e| NestGateError::io_error(format!("Fetch task panicked: {e}")))?;

    result
}

fn validate_fetch_url(url: &str, allow_http: bool) -> Result<()> {
    let parsed = url::Url::parse(url)
        .map_err(|e| NestGateError::invalid_input_with_field("url", format!("invalid URL: {e}")))?;
    match parsed.scheme() {
        "https" => Ok(()),
        "http" if allow_http => {
            tracing::warn!("content.fetch: HTTP (insecure) for {url}");
            Ok(())
        }
        scheme => Err(NestGateError::invalid_input_with_field(
            "url",
            format!("scheme '{scheme}' not allowed — use https"),
        )),
    }
}

fn do_fetch_to_cas(
    url: &str,
    family_id: &str,
    rate_limit_mbps: Option<f64>,
    timeout_secs: u64,
    expected_hash: Option<&str>,
) -> Result<Value> {
    debug!("content.fetch: GET {url} (family={family_id})");

    let _ = rustls_rustcrypto::provider().install_default();

    let timeout = std::time::Duration::from_secs(timeout_secs);
    let user_agent = format!("NestGate/{}", env!("CARGO_PKG_VERSION"));

    let config = ureq::Agent::config_builder()
        .timeout_global(Some(timeout))
        .https_only(false)
        .build();
    let agent = ureq::Agent::new_with_config(config);

    let mut response = agent
        .get(url)
        .header("User-Agent", &user_agent)
        .call()
        .map_err(|e| NestGateError::io_error(format!("content.fetch failed for {url}: {e}")))?;

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_owned();

    let rate_bytes_per_sec = rate_limit_mbps.map(|mbps| (mbps * 125_000.0) as u64);

    let body = response.body_mut();
    let mut hasher = blake3::Hasher::new();
    let mut all_bytes: Vec<u8> = Vec::new();
    let mut chunk = vec![0u8; CHUNK_SIZE];
    let mut bytes_this_second: u64 = 0;
    let mut second_start = std::time::Instant::now();

    loop {
        let n = body
            .read(&mut chunk)
            .map_err(|e| NestGateError::io_error(format!("content.fetch read error: {e}")))?;
        if n == 0 {
            break;
        }
        hasher.update(&chunk[..n]);
        all_bytes.extend_from_slice(&chunk[..n]);

        if let Some(limit) = rate_bytes_per_sec {
            bytes_this_second += n as u64;
            if bytes_this_second >= limit {
                let elapsed = second_start.elapsed();
                if elapsed < std::time::Duration::from_secs(1) {
                    std::thread::sleep(std::time::Duration::from_secs(1) - elapsed);
                }
                bytes_this_second = 0;
                second_start = std::time::Instant::now();
            }
        }
    }

    let hash = hasher.finalize();
    let hash_hex = hash.to_hex().to_string();
    let size = all_bytes.len() as u64;

    debug!("content.fetch: {size} bytes, blake3={hash_hex}");

    if let Some(expected) = expected_hash {
        if hash_hex != expected {
            return Err(NestGateError::invalid_input_with_field(
                "expected_hash",
                format!("BLAKE3 mismatch: expected {expected}, got {hash_hex}"),
            ));
        }
    }

    let cas_path = content_key_path(family_id, &hash_hex);
    let meta_path = {
        let mut p = cas_path.clone();
        let fname = p.file_name().unwrap().to_string_lossy().to_string();
        p.set_file_name(format!("{fname}.meta.json"));
        p
    };

    if cas_path.exists() {
        debug!("content.fetch: deduplicated — {hash_hex} already in CAS");
        return Ok(json!({
            "hash": hash_hex,
            "size": size,
            "url": url,
            "content_type": content_type,
            "stored": true,
            "deduplicated": true,
            "family_id": family_id,
            "fetched_at": chrono::Utc::now().to_rfc3339(),
        }));
    }

    let parent = cas_path.parent().ok_or_else(|| {
        NestGateError::io_error("content.fetch: cannot determine CAS parent directory")
    })?;
    std::fs::create_dir_all(parent).map_err(|e| {
        NestGateError::io_error(format!("content.fetch: mkdir failed: {e}"))
    })?;

    let part_path = cas_path.with_extension("part");
    std::fs::write(&part_path, &all_bytes).map_err(|e| {
        NestGateError::io_error(format!("content.fetch: write .part failed: {e}"))
    })?;
    std::fs::rename(&part_path, &cas_path).map_err(|e| {
        NestGateError::io_error(format!("content.fetch: rename to CAS failed: {e}"))
    })?;

    let meta = json!({
        "content_type": content_type,
        "stored_at": chrono::Utc::now().to_rfc3339(),
        "source": url,
        "pipeline": "content.fetch",
        "stored_by": "nestgate",
    });
    let _ = std::fs::write(&meta_path, serde_json::to_vec_pretty(&meta).unwrap_or_default());

    Ok(json!({
        "hash": hash_hex,
        "size": size,
        "url": url,
        "content_type": content_type,
        "stored": true,
        "deduplicated": false,
        "family_id": family_id,
        "fetched_at": chrono::Utc::now().to_rfc3339(),
    }))
}
