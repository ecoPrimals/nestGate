// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Content federation handlers — remote repo sync, push, and blob replication.
//!
//! These methods form the Neural API surface that replaces `cascade-pull.sh`
//! and enables the waterFall / rootPulse signal graphs to operate against
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

use base64::{Engine as _, engine::general_purpose::STANDARD};
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::Path;
use tokio::process::Command;
use tracing::debug;

use super::StorageState;
use super::storage_paths::{content_key_path, ensure_parent_dirs, resolve_family_id};

/// `content.fetch_heads` — read-only freshness check against remote repos.
///
/// For each repo in `repos`, runs `git ls-remote` to fetch remote HEAD refs
/// and compares against the local HEAD. Returns drift status without pulling.
///
/// Serves `ecosystem.check` signal graph.
pub async fn content_fetch_heads(params: Option<&Value>, _state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let repos = params["repos"]
        .as_array()
        .ok_or_else(|| {
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

        let result = fetch_head_refs(path, remote, branch).await;
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

    let repos = params["repos"]
        .as_array()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "repos",
                "repos array required: [{path, remote?, branch?}]",
            )
        })?;

    let mut results = Vec::with_capacity(repos.len());
    let mut pushed_count: u64 = 0;

    for repo in repos {
        let path = repo["path"].as_str().unwrap_or("");
        let remote = repo["remote"].as_str().unwrap_or("forgejo");
        let branch = repo["branch"].as_str().unwrap_or("main");

        if path.is_empty() {
            results.push(json!({
                "path": path,
                "pushed": false,
                "error": "path required"
            }));
            continue;
        }

        let result = push_to_remote(path, remote, branch).await;
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

    let cids = params["cids"]
        .as_array()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "cids",
                "cids array required: [\"<blake3_hex>\", ...]",
            )
        })?;

    let target = params["target"]
        .as_str()
        .ok_or_else(|| {
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

        match replicate_blob(&blob_path, cid, target, family_id).await {
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

    let repos = params["repos"]
        .as_array()
        .ok_or_else(|| {
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
            sync_repo(&path, &remote, &branch, clone_url.as_deref(), do_clone).await
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

// ── Internal helpers ──────────────────────────────────────────────────

/// Run `git ls-remote` and compare with the local HEAD.
async fn fetch_head_refs(repo_path: &str, remote: &str, branch: &str) -> Result<Value> {
    verify_git_available().await?;

    let local_head = git_rev_parse(repo_path, branch).await?;

    let remote_output = Command::new("git")
        .args(["ls-remote", "--heads", remote, &format!("refs/heads/{branch}")])
        .current_dir(repo_path)
        .output()
        .await
        .map_err(|e| NestGateError::internal(format!("git ls-remote failed: {e}")))?;

    if !remote_output.status.success() {
        let stderr = String::from_utf8_lossy(&remote_output.stderr);
        return Ok(json!({
            "path": repo_path,
            "remote": remote,
            "branch": branch,
            "local_head": local_head,
            "remote_head": null,
            "drift": "unknown",
            "error": stderr.trim()
        }));
    }

    let stdout = String::from_utf8_lossy(&remote_output.stdout);
    let remote_head = stdout
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_owned();

    let drift = if remote_head.is_empty() {
        "no_remote_ref"
    } else if remote_head == local_head {
        "in_sync"
    } else {
        "diverged"
    };

    let (behind, ahead) = if drift == "diverged" {
        count_divergence(repo_path, &local_head, &remote_head).await
    } else {
        (0u64, 0u64)
    };

    Ok(json!({
        "path": repo_path,
        "remote": remote,
        "branch": branch,
        "local_head": local_head,
        "remote_head": if remote_head.is_empty() { Value::Null } else { Value::String(remote_head) },
        "drift": drift,
        "behind": behind,
        "ahead": ahead
    }))
}

/// Run `git push` to the specified remote.
async fn push_to_remote(repo_path: &str, remote: &str, branch: &str) -> Result<Value> {
    verify_git_available().await?;

    debug!(path = repo_path, remote, branch, "content.push: pushing");

    let output = Command::new("git")
        .args(["push", remote, branch])
        .current_dir(repo_path)
        .output()
        .await
        .map_err(|e| NestGateError::internal(format!("git push failed: {e}")))?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() || stderr.contains("Everything up-to-date") {
        let already_up_to_date = stderr.contains("Everything up-to-date");
        Ok(json!({
            "path": repo_path,
            "remote": remote,
            "branch": branch,
            "pushed": true,
            "already_up_to_date": already_up_to_date,
            "output": stderr.trim()
        }))
    } else {
        Ok(json!({
            "path": repo_path,
            "remote": remote,
            "branch": branch,
            "pushed": false,
            "error": stderr.trim()
        }))
    }
}

/// Transfer a single content blob to a remote `NestGate` via JSON-RPC `content.put`.
///
/// Uses `socat` for UDS or direct TCP connection to send the blob as a
/// `content.put` JSON-RPC call. Returns the blob size if transferred,
/// 0 if already present on the remote.
async fn replicate_blob(
    blob_path: &Path,
    cid: &str,
    target: &str,
    family_id: &str,
) -> Result<u64> {
    let raw = tokio::fs::read(blob_path)
        .await
        .map_err(|e| NestGateError::internal(format!("read blob {cid}: {e}")))?;
    let size = raw.len() as u64;
    let encoded = STANDARD.encode(&raw);

    let exists_request = json!({
        "jsonrpc": "2.0",
        "method": "content.exists",
        "params": {"hash": cid, "family_id": family_id},
        "id": 1
    });

    let exists_response = send_jsonrpc(target, &exists_request).await?;
    if exists_response["result"]["exists"].as_bool() == Some(true) {
        return Ok(0);
    }

    let meta_path = blob_path.with_extension("meta.json");
    let metadata: Option<Value> = if meta_path.exists() {
        tokio::fs::read_to_string(&meta_path)
            .await
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
    } else {
        None
    };

    let mut put_params = json!({
        "data": encoded,
        "family_id": family_id
    });
    if let Some(meta) = metadata {
        if let Some(ct) = meta.get("content_type") {
            put_params["content_type"] = ct.clone();
        }
        if let Some(src) = meta.get("source") {
            put_params["source"] = src.clone();
        }
    }

    let put_request = json!({
        "jsonrpc": "2.0",
        "method": "content.put",
        "params": put_params,
        "id": 2
    });

    let put_response = send_jsonrpc(target, &put_request).await?;
    if put_response.get("error").is_some() {
        return Err(NestGateError::internal(format!(
            "remote content.put failed: {}",
            put_response["error"]
        )));
    }

    Ok(size)
}

/// Sync a single repo: pull with `--ff-only`, optionally clone if missing.
async fn sync_repo(
    path: &str,
    remote: &str,
    branch: &str,
    clone_url: Option<&str>,
    clone_missing: bool,
) -> Result<Value> {
    if path.is_empty() {
        return Ok(json!({"path": path, "synced": false, "error": "path required"}));
    }

    let repo_dir = Path::new(path);

    if !repo_dir.exists() {
        if clone_missing {
            if let Some(url) = clone_url {
                return clone_repo(url, path, branch).await;
            }
            return Ok(json!({
                "path": path,
                "synced": false,
                "error": "repo not found and no clone_url provided"
            }));
        }
        return Ok(json!({
            "path": path,
            "synced": false,
            "error": "repo not found (set clone_missing=true to auto-clone)"
        }));
    }

    verify_git_available().await?;

    let resolved_remote = if remote == "auto" {
        resolve_best_remote(path).await
    } else {
        remote.to_owned()
    };

    debug!(path, remote = %resolved_remote, branch, "content.sync: pulling");

    let fetch = Command::new("git")
        .args(["fetch", &resolved_remote, branch])
        .current_dir(path)
        .output()
        .await
        .map_err(|e| NestGateError::internal(format!("git fetch failed: {e}")))?;

    if !fetch.status.success() {
        let stderr = String::from_utf8_lossy(&fetch.stderr);
        return Ok(json!({
            "path": path,
            "remote": resolved_remote,
            "branch": branch,
            "synced": false,
            "action": "fetch_failed",
            "error": stderr.trim()
        }));
    }

    let before_head = git_rev_parse(path, "HEAD").await.unwrap_or_default();

    let merge = Command::new("git")
        .args(["merge", "--ff-only", &format!("{resolved_remote}/{branch}")])
        .current_dir(path)
        .output()
        .await
        .map_err(|e| NestGateError::internal(format!("git merge --ff-only failed: {e}")))?;

    let after_head = git_rev_parse(path, "HEAD").await.unwrap_or_default();

    let stderr = String::from_utf8_lossy(&merge.stderr);
    let stdout = String::from_utf8_lossy(&merge.stdout);
    let already_up_to_date = stdout.contains("Already up to date");

    let action = if !merge.status.success() {
        "merge_failed"
    } else if already_up_to_date {
        "already_up_to_date"
    } else {
        "fast_forward"
    };

    let synced = merge.status.success();

    let commits_pulled = if synced && !already_up_to_date && !before_head.is_empty() {
        count_commits(path, &before_head, &after_head).await
    } else {
        0u64
    };

    let mut result = json!({
        "path": path,
        "remote": resolved_remote,
        "branch": branch,
        "synced": synced,
        "action": action,
        "commits_pulled": commits_pulled
    });

    if !synced {
        result["error"] = Value::String(stderr.trim().to_owned());
    }

    Ok(result)
}

/// Clone a repo to the given path.
async fn clone_repo(url: &str, path: &str, branch: &str) -> Result<Value> {
    verify_git_available().await?;

    debug!(url, path, branch, "content.sync: cloning missing repo");

    if let Some(parent) = Path::new(path).parent() {
        ensure_parent_dirs(parent).await?;
    }

    let output = Command::new("git")
        .args(["clone", "--branch", branch, "--single-branch", url, path])
        .output()
        .await
        .map_err(|e| NestGateError::internal(format!("git clone failed: {e}")))?;

    if output.status.success() {
        Ok(json!({
            "path": path,
            "synced": true,
            "action": "cloned",
            "clone_url": url,
            "branch": branch
        }))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(json!({
            "path": path,
            "synced": false,
            "action": "clone_failed",
            "error": stderr.trim()
        }))
    }
}

/// Check that `git` is available on the system.
async fn verify_git_available() -> Result<()> {
    static CHECKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if CHECKED.load(std::sync::atomic::Ordering::Relaxed) {
        return Ok(());
    }

    Command::new("git")
        .arg("--version")
        .output()
        .await
        .map_err(|_| {
            NestGateError::internal(
                "git not found — content federation requires system git",
            )
        })?;

    CHECKED.store(true, std::sync::atomic::Ordering::Relaxed);
    Ok(())
}

/// Get the current HEAD commit hash for a ref.
async fn git_rev_parse(repo_path: &str, refspec: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", refspec])
        .current_dir(repo_path)
        .output()
        .await
        .map_err(|e| NestGateError::internal(format!("git rev-parse failed: {e}")))?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Count commits between two refs: how many behind and ahead.
async fn count_divergence(repo_path: &str, local: &str, remote: &str) -> (u64, u64) {
    let output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", &format!("{local}...{remote}")])
        .current_dir(repo_path)
        .output()
        .await;

    match output {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            let parts: Vec<&str> = text.trim().split('\t').collect();
            if parts.len() == 2 {
                let ahead = parts[0].parse().unwrap_or(0);
                let behind = parts[1].parse().unwrap_or(0);
                (behind, ahead)
            } else {
                (0, 0)
            }
        }
        _ => (0, 0),
    }
}

/// Count commits between two points.
async fn count_commits(repo_path: &str, from: &str, to: &str) -> u64 {
    let output = Command::new("git")
        .args(["rev-list", "--count", &format!("{from}..{to}")])
        .current_dir(repo_path)
        .output()
        .await;

    match output {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).trim().parse().unwrap_or(0)
        }
        _ => 0,
    }
}

/// Resolve the best remote for a repo: prefer "forgejo", fall back to "origin".
async fn resolve_best_remote(repo_path: &str) -> String {
    let output = Command::new("git")
        .args(["remote"])
        .current_dir(repo_path)
        .output()
        .await;

    match output {
        Ok(o) if o.status.success() => {
            let remotes = String::from_utf8_lossy(&o.stdout);
            if remotes.lines().any(|r| r.trim() == "forgejo") {
                String::from("forgejo")
            } else {
                String::from("origin")
            }
        }
        _ => String::from("origin"),
    }
}

/// Send a JSON-RPC request to a remote `NestGate` (UDS socket or TCP).
async fn send_jsonrpc(target: &str, request: &Value) -> Result<Value> {
    let payload = serde_json::to_string(request)
        .map_err(|e| NestGateError::internal(format!("serialize request: {e}")))?;

    if target.starts_with("tcp://") {
        send_jsonrpc_tcp(target, &payload).await
    } else {
        send_jsonrpc_uds(target, &payload).await
    }
}

/// Send JSON-RPC over a Unix domain socket using `socat`.
async fn send_jsonrpc_uds(socket_path: &str, payload: &str) -> Result<Value> {
    use tokio::io::AsyncWriteExt;

    let mut child = Command::new("socat")
        .args(["-", &format!("UNIX-CONNECT:{socket_path}")])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            NestGateError::internal(format!(
                "socat not available for UDS replication — install socat or use tcp:// target: {e}"
            ))
        })?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(payload.as_bytes()).await.map_err(|e| {
            NestGateError::internal(format!("write to socat: {e}"))
        })?;
        stdin.write_all(b"\n").await.ok();
        drop(stdin);
    }

    let out = child.wait_with_output().await.map_err(|e| {
        NestGateError::internal(format!("socat wait: {e}"))
    })?;

    let response_text = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(response_text.trim()).map_err(|e| {
        NestGateError::internal(format!("parse remote response: {e}"))
    })
}

/// Send JSON-RPC over TCP.
async fn send_jsonrpc_tcp(target: &str, payload: &str) -> Result<Value> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpStream;

    let addr = target
        .strip_prefix("tcp://")
        .ok_or_else(|| NestGateError::internal("invalid tcp:// target"))?;

    let stream = TcpStream::connect(addr).await.map_err(|e| {
        NestGateError::internal(format!("tcp connect to {addr}: {e}"))
    })?;

    let (reader, mut writer) = stream.into_split();
    writer.write_all(payload.as_bytes()).await.map_err(|e| {
        NestGateError::internal(format!("tcp write: {e}"))
    })?;
    writer.write_all(b"\n").await.ok();
    writer.shutdown().await.ok();

    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();
    buf_reader.read_line(&mut line).await.map_err(|e| {
        NestGateError::internal(format!("tcp read response: {e}"))
    })?;

    serde_json::from_str(line.trim()).map_err(|e| {
        NestGateError::internal(format!("parse tcp response: {e}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn mock_state() -> StorageState {
        StorageState::new().unwrap_or_else(|_| {
            panic!("StorageState::new failed in test — check env")
        })
    }

    #[tokio::test]
    async fn fetch_heads_rejects_missing_repos() {
        let state = mock_state();
        let params = json!({});
        let err = content_fetch_heads(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn fetch_heads_empty_path_returns_error_entry() {
        let state = mock_state();
        let params = json!({"repos": [{"path": ""}]});
        let result = content_fetch_heads(Some(&params), &state).await.unwrap();
        assert_eq!(result["error_count"], 1);
        assert!(result["heads"][0]["error"].as_str().is_some());
    }

    #[tokio::test]
    async fn push_rejects_missing_repos() {
        let state = mock_state();
        let params = json!({});
        let err = content_push(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn push_empty_path_returns_error_entry() {
        let state = mock_state();
        let params = json!({"repos": [{"path": ""}]});
        let result = content_push(Some(&params), &state).await.unwrap();
        assert_eq!(result["pushed_count"], 0);
        assert!(result["results"][0]["error"].as_str().is_some());
    }

    #[tokio::test]
    async fn replicate_rejects_missing_cids() {
        let state = mock_state();
        let params = json!({"target": "/tmp/test.sock"});
        let err = content_replicate(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_rejects_missing_target() {
        let state = mock_state();
        let params = json!({"cids": ["abc"]});
        let err = content_replicate(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn replicate_rejects_invalid_cid_length() {
        let state = mock_state();
        let params = json!({
            "cids": ["tooshort"],
            "target": "/tmp/test.sock",
            "family_id": "test"
        });
        let result = content_replicate(Some(&params), &state).await.unwrap();
        assert!(result["replicated"][0]["error"].as_str().unwrap().contains("invalid CID"));
    }

    #[tokio::test]
    async fn sync_rejects_missing_repos() {
        let state = mock_state();
        let params = json!({});
        let err = content_sync(Some(&params), &state).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn sync_empty_path_returns_error() {
        let state = mock_state();
        let params = json!({"repos": [{"path": ""}]});
        let result = content_sync(Some(&params), &state).await.unwrap();
        assert_eq!(result["synced_count"], 0);
    }

    #[tokio::test]
    async fn sync_missing_repo_without_clone() {
        let state = mock_state();
        let params = json!({"repos": [{"path": "/nonexistent/repo/path"}]});
        let result = content_sync(Some(&params), &state).await.unwrap();
        let repo_result = &result["results"][0];
        assert_eq!(repo_result["synced"], false);
        assert!(repo_result["error"].as_str().unwrap().contains("not found"));
    }

    #[tokio::test]
    async fn sync_respects_parallel_limit() {
        let state = mock_state();
        let params = json!({
            "repos": [{"path": "/nonexistent1"}, {"path": "/nonexistent2"}],
            "parallel": 1
        });
        let result = content_sync(Some(&params), &state).await.unwrap();
        assert_eq!(result["parallel"], 1);
        assert_eq!(result["total_count"], 2);
    }

    #[tokio::test]
    async fn all_handlers_reject_none_params() {
        let state = mock_state();
        assert!(content_fetch_heads(None, &state).await.is_err());
        assert!(content_push(None, &state).await.is_err());
        assert!(content_replicate(None, &state).await.is_err());
        assert!(content_sync(None, &state).await.is_err());
    }
}
