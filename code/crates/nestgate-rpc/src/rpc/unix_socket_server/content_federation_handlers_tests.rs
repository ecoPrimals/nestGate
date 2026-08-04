// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use crate::rpc::unix_socket_server::federation_blob_transfer::FEDERATION_STREAM_THRESHOLD;
use serde_json::json;

fn mock_state() -> StorageState {
    StorageState::new()
        .unwrap_or_else(|_| panic!("StorageState::new failed in test — check env"))
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
    assert!(
        result["replicated"][0]["error"]
            .as_str()
            .unwrap()
            .contains("invalid CID")
    );
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
    assert!(content_replicate_pull(None, &state).await.is_err());
}

#[tokio::test]
async fn replicate_pull_rejects_missing_cids() {
    let state = mock_state();
    let params = json!({"source": "/tmp/test.sock"});
    let err = content_replicate_pull(Some(&params), &state).await;
    assert!(err.is_err());
}

#[tokio::test]
async fn replicate_pull_rejects_missing_source() {
    let state = mock_state();
    let params = json!({"cids": ["abc"]});
    let err = content_replicate_pull(Some(&params), &state).await;
    assert!(err.is_err());
}

#[tokio::test]
async fn replicate_pull_rejects_invalid_cid_length() {
    let state = mock_state();
    let params = json!({
        "cids": ["tooshort"],
        "source": "/tmp/test.sock",
        "family_id": "test"
    });
    let result = content_replicate_pull(Some(&params), &state).await.unwrap();
    assert!(
        result["pulled"][0]["error"]
            .as_str()
            .unwrap()
            .contains("invalid CID")
    );
}

#[tokio::test]
async fn replicate_pull_skips_existing_local_blob() {
    let state = mock_state();
    let family_id = "test_pull_skip";
    let cid = "a".repeat(64);
    let blob_path = content_key_path(family_id, &cid);
    tokio::fs::create_dir_all(blob_path.parent().unwrap())
        .await
        .unwrap();
    tokio::fs::write(&blob_path, b"test data").await.unwrap();

    let params = json!({
        "cids": [cid],
        "source": "/tmp/nonexistent.sock",
        "family_id": family_id
    });
    let result = content_replicate_pull(Some(&params), &state).await.unwrap();
    assert_eq!(result["skipped_count"], 1);
    assert!(result["pulled"][0]["skipped"].as_bool().unwrap());

    let _ = tokio::fs::remove_dir_all(
        nestgate_config::config::storage_paths::get_storage_base_path()
            .join("datasets")
            .join(family_id),
    )
    .await;
}

#[test]
fn streaming_threshold_below_btsp_frame_max() {
    assert!(
        FEDERATION_STREAM_THRESHOLD <= 16 * 1024 * 1024,
        "streaming threshold must not exceed BTSP frame limit"
    );
}
