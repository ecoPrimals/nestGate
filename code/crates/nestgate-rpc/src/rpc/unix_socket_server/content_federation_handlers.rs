// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content federation handlers — remote repo sync, push, and blob replication.
//!
//! These methods form the Neural API surface that replaces `cascade-pull.sh`
//! and enables the cascade / provenance signal graphs to operate against
//! live `NestGate` instances.
//!
//! ## Methods
//!
//! | Method               | Signal Graph         | Purpose                              |
//! |----------------------|----------------------|--------------------------------------|
//! | `content.fetch_heads`| `ecosystem.check`    | Read-only drift detection            |
//! | `content.push`       | `ecosystem.push`     | Push to Forgejo (periplasm)          |
//! | `content.replicate`  | `rootpulse.federate` | Cross-gate content blob transfer     |
//! | `content.sync`       | `ecosystem.pull`     | Cascade-pull from remote sources     |
//!
//! ## External tools
//!
//! Repo operations delegate to the system `git` binary via `tokio::process::Command`.
//! `NestGate` does **not** link a C git library — the ecosystem standard mandates
//! a pure Rust toolchain with no C build dependencies. Git is a runtime peer tool,
//! similar to how the installer uses system `curl`.

use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};

use super::StorageState;
use super::federation_blob_transfer::{pull_blob_from_remote, replicate_blob_to_remote};
use super::federation_ops;
use super::storage_paths::{content_key_path, resolve_family_id};

/// `content.fetch_heads` — read-only freshness check against remote repos.
///
/// For each repo in `repos`, runs `git ls-remote` to fetch remote HEAD refs
/// and compares against the local HEAD. Returns drift status without pulling.
///
/// Serves `ecosystem.check` signal graph.
pub async fn content_fetch_heads(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let repos = params["repos"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "repos",
            "repos array required: [{path, remote?, branch?}]",
        )
    })?;

    let mut heads = Vec::with_capacity(repos.len());
    let mut error_count: u64 = 0;

    for repo in repos {
        let path = repo["path"].as_str().unwrap_or("");
        let remote = repo["remote"].as_str().unwrap_or("origin");
        let branch = repo["branch"].as_str().unwrap_or("main");

        if path.is_empty() {
            heads.push(json!({
                "path": path,
                "error": "path required"
            }));
            error_count += 1;
            continue;
        }

        let result = federation_ops::fetch_head_refs(path, remote, branch).await;
        match result {
            Ok(head_info) => heads.push(head_info),
            Err(e) => {
                heads.push(json!({
                    "path": path,
                    "remote": remote,
                    "branch": branch,
                    "error": e.to_string()
                }));
                error_count += 1;
            }
        }
    }

    let checked_count = heads.len() as u64;
    Ok(json!({
        "heads": heads,
        "checked_count": checked_count,
        "error_count": error_count,
        "checked_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// `content.push` — push local content to a remote (Forgejo periplasm).
///
/// For each repo in `repos`, runs `git push` to the specified remote.
///
/// Serves `ecosystem.push` signal graph.
pub async fn content_push(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let repos = params["repos"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "repos",
            "repos array required: [{path, remote?, branch?}]",
        )
    })?;

    let mut results = Vec::with_capacity(repos.len());
    let mut pushed_count: u64 = 0;

    for repo in repos {
        let path = repo["path"].as_str().unwrap_or("");
        let default_remote =
            std::env::var("NESTGATE_PREFERRED_REMOTE").unwrap_or_else(|_| "origin".into());
        let remote = repo["remote"]
            .as_str()
            .map_or(default_remote.as_str(), |r| r);
        let branch = repo["branch"].as_str().unwrap_or("main");

        if path.is_empty() {
            results.push(json!({
                "path": path,
                "pushed": false,
                "error": "path required"
            }));
            continue;
        }

        let result = federation_ops::push_to_remote(path, remote, branch).await;
        match result {
            Ok(info) => {
                if info["pushed"].as_bool().unwrap_or(false) {
                    pushed_count += 1;
                }
                results.push(info);
            }
            Err(e) => {
                results.push(json!({
                    "path": path,
                    "remote": remote,
                    "branch": branch,
                    "pushed": false,
                    "error": e.to_string()
                }));
            }
        }
    }

    Ok(json!({
        "results": results,
        "pushed_count": pushed_count,
        "total_count": results.len() as u64,
        "pushed_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// `content.replicate` — transfer content blobs (by CID) to a remote `NestGate`.
///
/// Accepts a list of BLAKE3 content identifiers and transfers them to the
/// target `NestGate` instance. Used by `rootpulse.federate` for cross-gate
/// content synchronization.
///
/// Transfer is diff-based: only blobs the remote lacks are sent.
pub async fn content_replicate(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let cids = params["cids"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "cids",
            "cids array required: [\"<blake3_hex>\", ...]",
        )
    })?;

    let target = params["target"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "target",
            "target required: socket path or tcp://host:port of remote nestgate",
        )
    })?;

    let family_id = resolve_family_id(params, state)?;

    let mut replicated = Vec::with_capacity(cids.len());
    let mut total_bytes: u64 = 0;
    let mut transferred_count: u64 = 0;
    let mut skipped_count: u64 = 0;

    for cid_val in cids {
        let cid = match cid_val.as_str() {
            Some(c) if c.len() == 64 => c,
            Some(c) => {
                replicated.push(json!({
                    "cid": c,
                    "transferred": false,
                    "error": "invalid CID: expected 64-char BLAKE3 hex"
                }));
                continue;
            }
            None => continue,
        };

        let blob_path = content_key_path(family_id, cid);
        if !blob_path.exists() {
            replicated.push(json!({
                "cid": cid,
                "transferred": false,
                "error": "blob not found locally"
            }));
            continue;
        }

        match replicate_blob_to_remote(&blob_path, cid, target, family_id).await {
            Ok(size) => {
                if size > 0 {
                    transferred_count += 1;
                    total_bytes += size;
                    replicated.push(json!({
                        "cid": cid,
                        "transferred": true,
                        "size": size
                    }));
                } else {
                    skipped_count += 1;
                    replicated.push(json!({
                        "cid": cid,
                        "transferred": false,
                        "skipped": true,
                        "reason": "already exists on remote"
                    }));
                }
            }
            Err(e) => {
                replicated.push(json!({
                    "cid": cid,
                    "transferred": false,
                    "error": e.to_string()
                }));
            }
        }
    }

    Ok(json!({
        "replicated": replicated,
        "transferred_count": transferred_count,
        "skipped_count": skipped_count,
        "total_bytes": total_bytes,
        "target": target,
        "family_id": family_id,
        "replicated_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// `content.sync` — cascade-pull from remote sources.
///
/// Neural API equivalent of `cascade-pull.sh`. For each repo, resolves the
/// remote (forgejo-first, origin fallback) and pulls with `--ff-only`.
///
/// Serves `ecosystem.pull` signal graph.
pub async fn content_sync(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let repos = params["repos"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "repos",
            "repos array required: [{path, remote?, branch?}]",
        )
    })?;

    let parallel = params["parallel"].as_u64().unwrap_or(4).min(16);
    let clone_missing = params["clone_missing"].as_bool().unwrap_or(false);
    let source = params["source"].as_str().unwrap_or("auto");

    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(parallel as usize));
    let mut handles = Vec::with_capacity(repos.len());

    for repo in repos {
        let path = repo["path"].as_str().unwrap_or("").to_owned();
        let remote = repo["remote"]
            .as_str()
            .map_or_else(|| source.to_owned(), String::from);
        let branch = repo["branch"].as_str().unwrap_or("main").to_owned();
        let clone_url = repo["clone_url"].as_str().map(String::from);
        let do_clone = clone_missing;
        let sem = semaphore.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await;
            federation_ops::sync_repo(&path, &remote, &branch, clone_url.as_deref(), do_clone).await
        }));
    }

    let mut results = Vec::with_capacity(handles.len());
    let mut synced_count: u64 = 0;

    for handle in handles {
        match handle.await {
            Ok(Ok(info)) => {
                if info["synced"].as_bool().unwrap_or(false) {
                    synced_count += 1;
                }
                results.push(info);
            }
            Ok(Err(e)) => {
                results.push(json!({
                    "synced": false,
                    "error": e.to_string()
                }));
            }
            Err(e) => {
                results.push(json!({
                    "synced": false,
                    "error": format!("task panic: {e}")
                }));
            }
        }
    }

    Ok(json!({
        "results": results,
        "synced_count": synced_count,
        "total_count": results.len() as u64,
        "parallel": parallel,
        "source": source,
        "synced_at": chrono::Utc::now().to_rfc3339()
    }))
}

/// `content.replicate.pull` — pull CIDs from a remote `NestGate` to local storage.
///
/// The inverse of `content.replicate` (which pushes). Used for cold-from-hot
/// federation: a cold-storage gate schedules pulls from the hot gate.
///
/// Diff-based: only fetches blobs the local store lacks.
pub async fn content_replicate_pull(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let cids = params["cids"].as_array().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "cids",
            "cids array required: [\"<blake3_hex>\", ...]",
        )
    })?;

    let source = params["source"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field(
            "source",
            "source required: socket path or tcp://host:port of remote nestgate",
        )
    })?;

    let family_id = resolve_family_id(params, state)?;

    let mut pulled = Vec::with_capacity(cids.len());
    let mut total_bytes: u64 = 0;
    let mut transferred_count: u64 = 0;
    let mut skipped_count: u64 = 0;

    for cid_val in cids {
        let cid = match cid_val.as_str() {
            Some(c) if c.len() == 64 => c,
            Some(c) => {
                pulled.push(json!({
                    "cid": c,
                    "pulled": false,
                    "error": "invalid CID: expected 64-char BLAKE3 hex"
                }));
                continue;
            }
            None => continue,
        };

        let local_path = content_key_path(family_id, cid);
        if local_path.exists() {
            skipped_count += 1;
            pulled.push(json!({
                "cid": cid,
                "pulled": false,
                "skipped": true,
                "reason": "already exists locally"
            }));
            continue;
        }

        match pull_blob_from_remote(cid, source, family_id, &local_path).await {
            Ok(size) => {
                transferred_count += 1;
                total_bytes += size;
                pulled.push(json!({
                    "cid": cid,
                    "pulled": true,
                    "size": size
                }));
            }
            Err(e) => {
                pulled.push(json!({
                    "cid": cid,
                    "pulled": false,
                    "error": e.to_string()
                }));
            }
        }
    }

    Ok(json!({
        "pulled": pulled,
        "transferred_count": transferred_count,
        "skipped_count": skipped_count,
        "total_bytes": total_bytes,
        "source": source,
        "family_id": family_id,
        "pulled_at": chrono::Utc::now().to_rfc3339()
    }))
}

#[cfg(test)]
#[path = "content_federation_handlers_tests.rs"]
mod tests;
