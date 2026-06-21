// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use serde_json::json;
use tempfile::TempDir;
use tokio::process::Command;

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

async fn init_temp_git_repo_with_bare_remote() -> (TempDir, String, TempDir, String) {
    let bare_dir = TempDir::new().expect("bare dir");
    let bare_path = bare_dir.path().to_str().unwrap().to_owned();
    Command::new("git")
        .args(["init", "--bare", "--initial-branch=main"])
        .current_dir(&bare_path)
        .output()
        .await
        .expect("git init --bare");

    let (work_dir, work_path) = init_temp_git_repo().await;

    Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(&work_path)
        .output()
        .await
        .expect("git branch -M main");

    Command::new("git")
        .args(["remote", "add", "origin", &bare_path])
        .current_dir(&work_path)
        .output()
        .await
        .expect("git remote add");

    Command::new("git")
        .args(["push", "-u", "origin", "main"])
        .current_dir(&work_path)
        .output()
        .await
        .expect("git push");

    (work_dir, work_path, bare_dir, bare_path)
}

#[tokio::test]
async fn count_divergence_detects_ahead() {
    let (_work_dir, work_path, _bare_dir, _) = init_temp_git_repo_with_bare_remote().await;

    let base = git_rev_parse(&work_path, "HEAD").await.unwrap();

    tokio::fs::write(std::path::Path::new(&work_path).join("new.txt"), "content")
        .await
        .unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(&work_path)
        .output()
        .await
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "second"])
        .current_dir(&work_path)
        .output()
        .await
        .unwrap();

    let head = git_rev_parse(&work_path, "HEAD").await.unwrap();
    let (behind, ahead) = count_divergence(&work_path, &head, &base).await;
    assert_eq!(behind, 0);
    assert_eq!(ahead, 1);
}

#[tokio::test]
async fn count_commits_reports_one() {
    let (_dir, path) = init_temp_git_repo().await;
    let head = git_rev_parse(&path, "HEAD").await.unwrap();
    let count = count_commits(&path, &head, &head).await;
    assert_eq!(count, 0);

    tokio::fs::write(std::path::Path::new(&path).join("a.txt"), "a")
        .await
        .unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(&path)
        .output()
        .await
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "second"])
        .current_dir(&path)
        .output()
        .await
        .unwrap();

    let head2 = git_rev_parse(&path, "HEAD").await.unwrap();
    let count = count_commits(&path, &head, &head2).await;
    assert_eq!(count, 1);
}

#[tokio::test]
async fn resolve_best_remote_prefers_forgejo() {
    let (_dir, path) = init_temp_git_repo().await;
    Command::new("git")
        .args(["remote", "add", "forgejo", "https://example.com/repo.git"])
        .current_dir(&path)
        .output()
        .await
        .unwrap();
    let remote = resolve_best_remote(&path).await;
    assert_eq!(remote, "forgejo");
}

#[tokio::test]
async fn fetch_head_refs_with_remote_shows_in_sync() {
    let (_work_dir, work_path, _bare_dir, _) = init_temp_git_repo_with_bare_remote().await;
    let result = fetch_head_refs(&work_path, "origin", "main").await.unwrap();
    assert_eq!(result["drift"].as_str(), Some("in_sync"));
}

#[tokio::test]
async fn push_to_remote_succeeds_when_up_to_date() {
    let (_work_dir, work_path, _bare_dir, _) = init_temp_git_repo_with_bare_remote().await;
    let result = push_to_remote(&work_path, "origin", "main").await.unwrap();
    assert_eq!(result["pushed"].as_bool(), Some(true));
}

#[tokio::test]
async fn sync_repo_succeeds_on_clean_repo() {
    let (_work_dir, work_path, _bare_dir, _) = init_temp_git_repo_with_bare_remote().await;
    let result = sync_repo(&work_path, "origin", "main", None, false)
        .await
        .unwrap();
    assert_eq!(result["synced"].as_bool(), Some(true));
}
