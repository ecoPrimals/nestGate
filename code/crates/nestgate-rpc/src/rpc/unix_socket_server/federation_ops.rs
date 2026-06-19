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

use nestgate_types::error::{NestGateError, Result};
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

/// Send a JSON-RPC request to a remote `NestGate` (UDS socket or TCP).
pub(super) async fn send_jsonrpc(target: &str, request: &Value) -> Result<Value> {
    let payload = serde_json::to_string(request)
        .map_err(|e| NestGateError::internal(format!("serialize request: {e}")))?;

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
        .map_err(|e| NestGateError::internal(format!("git rev-parse failed: {e}")))?;

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
                String::from("forgejo")
            } else {
                String::from("origin")
            }
        }
        _ => String::from("origin"),
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
            .map_err(|e| NestGateError::internal(format!("write to socat: {e}")))?;
        stdin.write_all(b"\n").await.ok();
        drop(stdin);
    }

    let out = child
        .wait_with_output()
        .await
        .map_err(|e| NestGateError::internal(format!("socat wait: {e}")))?;

    let response_text = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(response_text.trim())
        .map_err(|e| NestGateError::internal(format!("parse remote response: {e}")))
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
        .map_err(|e| NestGateError::internal(format!("tcp write: {e}")))?;
    writer.write_all(b"\n").await.ok();
    writer.shutdown().await.ok();

    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();
    buf_reader
        .read_line(&mut line)
        .await
        .map_err(|e| NestGateError::internal(format!("tcp read response: {e}")))?;

    serde_json::from_str(line.trim())
        .map_err(|e| NestGateError::internal(format!("parse tcp response: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn init_temp_git_repo() -> (TempDir, String) {
        let dir = TempDir::new().expect("create tempdir");
        let path = dir.path().to_str().unwrap().to_owned();

        Command::new("git")
            .args(["init"])
            .current_dir(&path)
            .output()
            .await
            .expect("git init");

        Command::new("git")
            .args(["config", "user.email", "test@test.local"])
            .current_dir(&path)
            .output()
            .await
            .expect("git config email");

        Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(&path)
            .output()
            .await
            .expect("git config name");

        tokio::fs::write(dir.path().join("README.md"), "# test\n")
            .await
            .expect("write file");

        Command::new("git")
            .args(["add", "."])
            .current_dir(&path)
            .output()
            .await
            .expect("git add");

        Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(&path)
            .output()
            .await
            .expect("git commit");

        (dir, path)
    }

    #[tokio::test]
    async fn verify_git_available_succeeds() {
        verify_git_available()
            .await
            .expect("git should be available");
    }

    #[tokio::test]
    async fn git_rev_parse_returns_hash() {
        let (_dir, path) = init_temp_git_repo().await;
        let hash = git_rev_parse(&path, "HEAD").await.expect("rev-parse");
        assert_eq!(hash.len(), 40);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[tokio::test]
    async fn resolve_best_remote_defaults_to_origin() {
        let (_dir, path) = init_temp_git_repo().await;
        let remote = resolve_best_remote(&path).await;
        assert_eq!(remote, "origin");
    }

    #[tokio::test]
    async fn count_divergence_same_ref_is_zero() {
        let (_dir, path) = init_temp_git_repo().await;
        let hash = git_rev_parse(&path, "HEAD").await.unwrap();
        let (behind, ahead) = count_divergence(&path, &hash, &hash).await;
        assert_eq!(behind, 0);
        assert_eq!(ahead, 0);
    }

    #[tokio::test]
    async fn count_commits_same_ref_is_zero() {
        let (_dir, path) = init_temp_git_repo().await;
        let hash = git_rev_parse(&path, "HEAD").await.unwrap();
        let count = count_commits(&path, &hash, &hash).await;
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn sync_repo_empty_path_returns_error() {
        let result = sync_repo("", "origin", "main", None, false)
            .await
            .expect("should return json, not error");
        assert_eq!(result["synced"].as_bool(), Some(false));
        assert!(result["error"].as_str().unwrap().contains("path required"));
    }

    #[tokio::test]
    async fn sync_repo_nonexistent_no_clone_returns_error() {
        let result = sync_repo(
            "/tmp/nonexistent-repo-xyz-abc",
            "origin",
            "main",
            None,
            false,
        )
        .await
        .expect("should return json");
        assert_eq!(result["synced"].as_bool(), Some(false));
        assert!(result["error"].as_str().unwrap().contains("not found"));
    }

    #[tokio::test]
    async fn sync_repo_nonexistent_clone_missing_no_url_returns_error() {
        let result = sync_repo(
            "/tmp/nonexistent-repo-xyz-abc",
            "origin",
            "main",
            None,
            true,
        )
        .await
        .expect("should return json");
        assert_eq!(result["synced"].as_bool(), Some(false));
        assert!(result["error"].as_str().unwrap().contains("no clone_url"));
    }

    #[tokio::test]
    async fn fetch_head_refs_on_local_repo_reports_no_remote() {
        let (_dir, path) = init_temp_git_repo().await;
        let result = fetch_head_refs(&path, "origin", "main").await;
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["drift"].as_str(), Some("unknown"));
    }

    #[tokio::test]
    async fn push_to_remote_without_remote_fails_gracefully() {
        let (_dir, path) = init_temp_git_repo().await;
        let result = push_to_remote(&path, "origin", "main").await;
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["pushed"].as_bool(), Some(false));
    }

    #[tokio::test]
    async fn send_jsonrpc_uds_nonexistent_socket_errors() {
        let result = send_jsonrpc("/tmp/nonexistent-socket-xyz.sock", &json!({"test": true})).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn send_jsonrpc_tcp_nonexistent_host_errors() {
        let result = send_jsonrpc("tcp://127.0.0.1:1", &json!({"test": true})).await;
        assert!(result.is_err());
    }
}
