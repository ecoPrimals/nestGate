// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Git and transport operations for content federation.
//!
//! Extracted from `content_federation_handlers` to keep file sizes under the
//! 800-line workspace limit. Contains:
//!
//! - **Git operations**: `verify_git_available`, `git_rev_parse`, `count_divergence`,
//!   `count_commits`, `resolve_best_remote`
//! - **Repo sync**: `fetch_head_refs`, `push_to_remote`, `sync_repo`, `clone_repo`
//! - **JSON-RPC transport**: `send_jsonrpc` (UDS via socat, TCP direct)

use nestgate_types::error::{ErrorContextExt, NestGateError, Result};
use serde_json::{Value, json};
use std::path::Path;
use tokio::process::Command;
use tracing::debug;

use super::storage_paths::ensure_parent_dirs;

/// Run `git ls-remote` and compare with the local HEAD.
pub(super) async fn fetch_head_refs(repo_path: &str, remote: &str, branch: &str) -> Result<Value> {
    verify_git_available().await?;

    let local_head = git_rev_parse(repo_path, branch).await?;

    let remote_output = Command::new("git")
        .args([
            "ls-remote",
            "--heads",
            remote,
            &format!("refs/heads/{branch}"),
        ])
        .current_dir(repo_path)
        .output()
        .await
        .internal_ctx("git ls-remote failed")?;

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
    let remote_head = stdout.split_whitespace().next().unwrap_or("").to_owned();

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
pub(super) async fn push_to_remote(repo_path: &str, remote: &str, branch: &str) -> Result<Value> {
    verify_git_available().await?;

    debug!(path = repo_path, remote, branch, "content.push: pushing");

    let output = Command::new("git")
        .args(["push", remote, branch])
        .current_dir(repo_path)
        .output()
        .await
        .internal_ctx("git push failed")?;

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

/// Sync a single repo: pull with `--ff-only`, optionally clone if missing.
pub(super) async fn sync_repo(
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
        .internal_ctx("git fetch failed")?;

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
        .internal_ctx("git merge --ff-only failed")?;

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

/// Send a JSON-RPC request to a remote `NestGate` (UDS socket or TCP).
pub(super) async fn send_jsonrpc(target: &str, request: &Value) -> Result<Value> {
    let payload = serde_json::to_string(request).internal_ctx("serialize request")?;

    if target.starts_with("tcp://") {
        send_jsonrpc_tcp(target, &payload).await
    } else {
        send_jsonrpc_uds(target, &payload).await
    }
}

// ── Private helpers ───────────────────────────────────────────────────

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
        .internal_ctx("git clone failed")?;

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
            NestGateError::internal("git not found — content federation requires system git")
        })?;

    CHECKED.store(true, std::sync::atomic::Ordering::Relaxed);
    Ok(())
}

async fn git_rev_parse(repo_path: &str, refspec: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", refspec])
        .current_dir(repo_path)
        .output()
        .await
        .internal_ctx("git rev-parse failed")?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

async fn count_divergence(repo_path: &str, local: &str, remote: &str) -> (u64, u64) {
    let output = Command::new("git")
        .args([
            "rev-list",
            "--left-right",
            "--count",
            &format!("{local}...{remote}"),
        ])
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

async fn count_commits(repo_path: &str, from: &str, to: &str) -> u64 {
    let output = Command::new("git")
        .args(["rev-list", "--count", &format!("{from}..{to}")])
        .current_dir(repo_path)
        .output()
        .await;

    match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout)
            .trim()
            .parse()
            .unwrap_or(0),
        _ => 0,
    }
}

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
                "forgejo".into()
            } else {
                "origin".into()
            }
        }
        _ => "origin".into(),
    }
}

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
        stdin
            .write_all(payload.as_bytes())
            .await
            .internal_ctx("write to socat")?;
        stdin.write_all(b"\n").await.ok();
        drop(stdin);
    }

    let out = child.wait_with_output().await.internal_ctx("socat wait")?;

    let response_text = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(response_text.trim()).internal_ctx("parse remote response")
}

async fn send_jsonrpc_tcp(target: &str, payload: &str) -> Result<Value> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpStream;

    let addr = target
        .strip_prefix("tcp://")
        .ok_or_else(|| NestGateError::internal("invalid tcp:// target"))?;

    let stream = TcpStream::connect(addr)
        .await
        .map_err(|e| NestGateError::internal(format!("tcp connect to {addr}: {e}")))?;

    let (reader, mut writer) = stream.into_split();
    writer
        .write_all(payload.as_bytes())
        .await
        .internal_ctx("tcp write")?;
    writer.write_all(b"\n").await.ok();
    writer.shutdown().await.ok();

    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();
    buf_reader
        .read_line(&mut line)
        .await
        .internal_ctx("tcp read response")?;

    serde_json::from_str(line.trim()).internal_ctx("parse tcp response")
}

#[cfg(test)]
#[path = "federation_ops_tests.rs"]
mod tests;
