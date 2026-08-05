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

use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::io::{Read, Write};
use tracing::debug;

use super::super::StorageState;
use super::super::storage_paths::{
    build_http_agent, content_key_path, http_user_agent, resolve_cas_object, resolve_family_id,
    validate_fetch_url,
};

/// Chunk size for rate-limited reads (64 KB).
const CHUNK_SIZE: usize = 64 * 1024;

/// Megabits-per-second → bytes-per-second conversion factor.
const MBPS_TO_BYTES: f64 = 125_000.0;

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

    tokio::task::spawn_blocking(move || {
        do_fetch_to_cas(
            &url_owned,
            &family_owned,
            rate_limit_mbps,
            timeout_secs,
            expected_hash.as_deref(),
        )
    })
    .await
    .map_err(|e| NestGateError::io_error(format!("Fetch task panicked: {e}")))?
}

fn do_fetch_to_cas(
    url: &str,
    family_id: &str,
    rate_limit_mbps: Option<f64>,
    timeout_secs: u64,
    expected_hash: Option<&str>,
) -> Result<Value> {
    debug!("content.fetch: GET {url} (family={family_id})");

    let timeout = std::time::Duration::from_secs(timeout_secs);
    let allow_http = url.starts_with("http://");
    let agent = build_http_agent(timeout, allow_http);
    let user_agent = http_user_agent();

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

    let rate_bytes_per_sec = rate_limit_mbps.map(|mbps| {
        #[expect(
            clippy::cast_sign_loss,
            clippy::cast_possible_truncation,
            reason = "mbps is user-supplied and validated non-negative; f64→u64 intentional"
        )]
        let bytes = mbps.max(0.0).mul_add(MBPS_TO_BYTES, 0.0) as u64;
        bytes
    });

    let cas_temp_dir = content_key_path(family_id, "00");
    let cas_temp_dir = cas_temp_dir.parent().and_then(std::path::Path::parent).ok_or_else(|| {
        NestGateError::io_error("content.fetch: cannot determine CAS base directory")
    })?;
    std::fs::create_dir_all(cas_temp_dir).map_err(|e| {
        NestGateError::io_error(format!("content.fetch: mkdir failed: {e}"))
    })?;

    let part_path = cas_temp_dir.join(format!("_fetch_{}.part", std::process::id()));
    let mut part_file = std::fs::File::create(&part_path).map_err(|e| {
        NestGateError::io_error(format!("content.fetch: create .part failed: {e}"))
    })?;

    let mut reader = response.body_mut().as_reader();
    let mut hasher = blake3::Hasher::new();
    let mut chunk = vec![0u8; CHUNK_SIZE];
    let mut total_bytes: u64 = 0;
    let mut bytes_this_second: u64 = 0;
    let mut second_start = std::time::Instant::now();

    loop {
        let n = reader
            .read(&mut chunk)
            .map_err(|e| NestGateError::io_error(format!("content.fetch read error: {e}")))?;
        if n == 0 {
            break;
        }
        hasher.update(&chunk[..n]);
        part_file.write_all(&chunk[..n]).map_err(|e| {
            NestGateError::io_error(format!("content.fetch: write error: {e}"))
        })?;
        total_bytes += n as u64;

        if let Some(limit) = rate_bytes_per_sec {
            bytes_this_second += n as u64;
            if bytes_this_second >= limit {
                let elapsed = second_start.elapsed();
                if let Some(remaining) =
                    std::time::Duration::from_secs(1).checked_sub(elapsed)
                {
                    std::thread::sleep(remaining);
                }
                bytes_this_second = 0;
                second_start = std::time::Instant::now();
            }
        }
    }
    drop(part_file);

    let hash_hex = hasher.finalize().to_hex().to_string();

    debug!("content.fetch: {total_bytes} bytes, blake3={hash_hex}");

    if let Some(expected) = expected_hash
        && hash_hex != expected
    {
        let _ = std::fs::remove_file(&part_path);
        return Err(NestGateError::invalid_input_with_field(
            "expected_hash",
            format!("BLAKE3 mismatch: expected {expected}, got {hash_hex}"),
        ));
    }

    let cas_path = content_key_path(family_id, &hash_hex);

    if resolve_cas_object(family_id, &hash_hex).is_some() {
        let _ = std::fs::remove_file(&part_path);
        debug!("content.fetch: deduplicated — {hash_hex} already in CAS");
        return Ok(json!({
            "hash": hash_hex,
            "size": total_bytes,
            "url": url,
            "content_type": content_type,
            "stored": true,
            "deduplicated": true,
            "family_id": family_id,
            "fetched_at": chrono::Utc::now().to_rfc3339(),
        }));
    }

    if let Some(parent) = cas_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            NestGateError::io_error(format!("content.fetch: mkdir failed: {e}"))
        })?;
    }

    std::fs::rename(&part_path, &cas_path).map_err(|e| {
        NestGateError::io_error(format!("content.fetch: rename to CAS failed: {e}"))
    })?;

    let meta_path = cas_path.with_file_name(format!("{hash_hex}.meta.json"));
    let meta = json!({
        "content_type": content_type,
        "stored_at": chrono::Utc::now().to_rfc3339(),
        "source": url,
        "pipeline": "content.fetch",
        "stored_by": "nestgate",
    });
    std::fs::write(
        &meta_path,
        serde_json::to_vec_pretty(&meta).unwrap_or_default(),
    )
    .map_err(|e| {
        NestGateError::io_error(format!("content.fetch: meta write failed: {e}"))
    })?;

    Ok(json!({
        "hash": hash_hex,
        "size": total_bytes,
        "url": url,
        "content_type": content_type,
        "stored": true,
        "deduplicated": false,
        "family_id": family_id,
        "fetched_at": chrono::Utc::now().to_rfc3339(),
    }))
}
