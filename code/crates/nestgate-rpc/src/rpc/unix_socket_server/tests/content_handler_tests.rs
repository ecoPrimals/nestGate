// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for `content_handlers` — content-addressed storage (NG-1) and
//! versioned manifests (NG-2). Extracted from inline `#[cfg(test)]` module.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::json;
use serial_test::serial;

use super::super::StorageState;
use super::super::content_handlers::*;

use super::common::{cleanup_family, mock_state};

// ── NG-1: Content-addressed storage tests ────────────────────────────

#[tokio::test]
#[serial]
async fn content_put_and_get_round_trip() {
    let state = mock_state(Some("test-content")).await;
    let family_id = format!("test-cas-{}", uuid::Uuid::new_v4());
    let raw = b"hello content-addressed world";
    let encoded = STANDARD.encode(raw);
    let expected_hash = blake3::hash(raw).to_hex().to_string();

    let put_params = json!({
        "family_id": &family_id,
        "data": encoded,
        "content_type": "text/plain"
    });
    let put_result = content_put(Some(&put_params), &state).await;
    assert!(put_result.is_ok(), "put failed: {put_result:?}");
    let put_val = put_result.unwrap();
    assert_eq!(put_val["hash"], expected_hash);
    assert_eq!(put_val["deduplicated"], false);
    assert_eq!(put_val["stored"], true);
    assert_eq!(put_val["size"], raw.len());

    let get_params = json!({"family_id": &family_id, "hash": &expected_hash});
    let get_result = content_get(Some(&get_params), &state).await;
    assert!(get_result.is_ok(), "get failed: {get_result:?}");
    let get_val = get_result.unwrap();
    let decoded = STANDARD.decode(get_val["data"].as_str().unwrap()).unwrap();
    assert_eq!(decoded, raw);
    assert_eq!(get_val["content_type"], "text/plain");

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn content_put_deduplicates() {
    let state = mock_state(Some("test-dedup")).await;
    let family_id = format!("test-dedup-{}", uuid::Uuid::new_v4());
    let raw = b"duplicate content";
    let encoded = STANDARD.encode(raw);

    let params = json!({"family_id": &family_id, "data": &encoded});
    let first = content_put(Some(&params), &state).await.unwrap();
    assert_eq!(first["deduplicated"], false);

    let second = content_put(Some(&params), &state).await.unwrap();
    assert_eq!(second["deduplicated"], true);
    assert_eq!(second["hash"], first["hash"]);

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn content_exists_returns_correct_state() {
    let state = mock_state(Some("test-exists")).await;
    let family_id = format!("test-exists-{}", uuid::Uuid::new_v4());
    let raw = b"existence check";
    let encoded = STANDARD.encode(raw);
    let hash = blake3::hash(raw).to_hex().to_string();

    let missing = json!({"family_id": &family_id, "hash": &hash});
    let r = content_exists(Some(&missing), &state).await.unwrap();
    assert_eq!(r["exists"], false);

    let put_p = json!({"family_id": &family_id, "data": &encoded});
    content_put(Some(&put_p), &state).await.unwrap();

    let found = json!({"family_id": &family_id, "hash": &hash});
    let r = content_exists(Some(&found), &state).await.unwrap();
    assert_eq!(r["exists"], true);
    assert!(r["size"].as_u64().unwrap() > 0);

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn content_list_returns_stored_hashes() {
    let state = mock_state(Some("test-list")).await;
    let family_id = format!("test-list-{}", uuid::Uuid::new_v4());

    for i in 0..3 {
        let data = format!("item-{i}");
        let encoded = STANDARD.encode(data.as_bytes());
        let p = json!({"family_id": &family_id, "data": &encoded});
        content_put(Some(&p), &state).await.unwrap();
    }

    let list_p = json!({"family_id": &family_id});
    let result = content_list(Some(&list_p), &state).await.unwrap();
    assert_eq!(result["count"], 3);
    assert_eq!(result["hashes"].as_array().unwrap().len(), 3);

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn content_get_returns_null_for_missing() {
    let state = mock_state(Some("test-miss")).await;
    let hash = "a".repeat(64);
    let params = json!({"family_id": "test-miss", "hash": &hash});
    let result = content_get(Some(&params), &state).await.unwrap();
    assert!(result["data"].is_null());
}

#[tokio::test]
#[serial]
async fn content_put_requires_data() {
    let state = mock_state(Some("test")).await;
    let params = json!({"family_id": "test"});
    assert!(content_put(Some(&params), &state).await.is_err());
}

#[tokio::test]
#[serial]
async fn content_get_rejects_invalid_hash() {
    let state = mock_state(Some("test")).await;
    let params = json!({"family_id": "test", "hash": "tooshort"});
    assert!(content_get(Some(&params), &state).await.is_err());
}

// ── NG-2: Manifest / collection tests ────────────────────────────────

async fn put_test_content(state: &StorageState, family_id: &str, data: &[u8]) -> String {
    let encoded = STANDARD.encode(data);
    let p = json!({"family_id": family_id, "data": &encoded, "content_type": "text/html"});
    let r = content_put(Some(&p), state).await.unwrap();
    r["hash"].as_str().unwrap().to_string()
}

#[tokio::test]
#[serial]
async fn publish_and_resolve_round_trip() {
    let state = mock_state(Some("test-pub")).await;
    let family_id = format!("test-pub-{}", uuid::Uuid::new_v4());

    let h1 = put_test_content(&state, &family_id, b"<html>index</html>").await;
    let h2 = put_test_content(&state, &family_id, b"body{color:red}").await;

    let pub_p = json!({
        "family_id": &family_id,
        "collection": "site-v1",
        "manifest": {"/": &h1, "/css/main.css": &h2}
    });
    let r = content_publish(Some(&pub_p), &state).await.unwrap();
    assert_eq!(r["stored"], true);
    assert_eq!(r["entry_count"], 2);

    let resolve_p = json!({
        "family_id": &family_id,
        "collection": "site-v1",
        "path": "/css/main.css"
    });
    let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
    assert_eq!(r["hash"], h2);

    let resolve_missing = json!({
        "family_id": &family_id,
        "collection": "site-v1",
        "path": "/not-here"
    });
    let r = content_resolve(Some(&resolve_missing), &state)
        .await
        .unwrap();
    assert!(r["hash"].is_null());

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn promote_alias_resolves_correctly() {
    let state = mock_state(Some("test-promo")).await;
    let family_id = format!("test-promo-{}", uuid::Uuid::new_v4());

    let h1 = put_test_content(&state, &family_id, b"hello").await;

    let pub_p = json!({
        "family_id": &family_id,
        "collection": "release-v1",
        "manifest": {"/": &h1}
    });
    content_publish(Some(&pub_p), &state).await.unwrap();

    let promote_p = json!({
        "family_id": &family_id,
        "collection": "release-v1",
        "alias": "latest"
    });
    let r = content_promote(Some(&promote_p), &state).await.unwrap();
    assert_eq!(r["promoted"], true);
    assert_eq!(r["target"], "release-v1");

    let resolve_p = json!({
        "family_id": &family_id,
        "collection": "latest",
        "path": "/"
    });
    let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
    assert_eq!(r["hash"], h1);

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn collections_lists_manifests() {
    let state = mock_state(Some("test-coll")).await;
    let family_id = format!("test-coll-{}", uuid::Uuid::new_v4());

    let h = put_test_content(&state, &family_id, b"x").await;

    for name in &["alpha", "beta", "gamma"] {
        let p = json!({
            "family_id": &family_id,
            "collection": name,
            "manifest": {"/": &h}
        });
        content_publish(Some(&p), &state).await.unwrap();
    }

    let list_p = json!({"family_id": &family_id});
    let r = content_collections(Some(&list_p), &state).await.unwrap();
    assert_eq!(r["count"], 3);

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn publish_rejects_missing_content_hash() {
    let state = mock_state(Some("test-reject")).await;
    let family_id = format!("test-reject-{}", uuid::Uuid::new_v4());
    let fake_hash = "b".repeat(64);

    let p = json!({
        "family_id": &family_id,
        "collection": "bad",
        "manifest": {"/": &fake_hash}
    });
    assert!(content_publish(Some(&p), &state).await.is_err());
}

#[tokio::test]
#[serial]
async fn resolve_inline_returns_content() {
    let state = mock_state(Some("test-inline")).await;
    let family_id = format!("test-inline-{}", uuid::Uuid::new_v4());

    let content = b"<html>inline</html>";
    let h = put_test_content(&state, &family_id, content).await;

    let pub_p = json!({
        "family_id": &family_id,
        "collection": "inline-test",
        "manifest": {"/": &h}
    });
    content_publish(Some(&pub_p), &state).await.unwrap();

    let resolve_p = json!({
        "family_id": &family_id,
        "collection": "inline-test",
        "path": "/",
        "inline": true
    });
    let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
    assert_eq!(r["hash"], h);
    let decoded = STANDARD.decode(r["data"].as_str().unwrap()).unwrap();
    assert_eq!(decoded, content);
    assert_eq!(r["content_type"], "text/html");

    cleanup_family(&family_id).await;
}

// ── Path normalization (index.html fallback for static sites) ──────────

#[tokio::test]
#[serial]
async fn resolve_trailing_slash_falls_back_to_index_html() {
    let state = mock_state(Some("test-idx-slash")).await;
    let family_id = format!("test-idx-slash-{}", uuid::Uuid::new_v4());

    let h = put_test_content(&state, &family_id, b"<html>home</html>").await;

    let pub_p = json!({
        "family_id": &family_id,
        "collection": "static-site",
        "manifest": {"/index.html": &h}
    });
    content_publish(Some(&pub_p), &state).await.unwrap();

    let resolve_p = json!({
        "family_id": &family_id,
        "collection": "static-site",
        "path": "/"
    });
    let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
    assert_eq!(r["hash"], h, "/ should resolve to /index.html");
    assert_eq!(r["resolved_path"], "/index.html");

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn resolve_bare_path_falls_back_to_dir_index_html() {
    let state = mock_state(Some("test-idx-bare")).await;
    let family_id = format!("test-idx-bare-{}", uuid::Uuid::new_v4());

    let h = put_test_content(&state, &family_id, b"<html>about</html>").await;

    let pub_p = json!({
        "family_id": &family_id,
        "collection": "static-site",
        "manifest": {"/about/index.html": &h}
    });
    content_publish(Some(&pub_p), &state).await.unwrap();

    let resolve_p = json!({
        "family_id": &family_id,
        "collection": "static-site",
        "path": "/about"
    });
    let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
    assert_eq!(r["hash"], h, "/about should resolve to /about/index.html");
    assert_eq!(r["resolved_path"], "/about/index.html");

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn resolve_exact_match_has_no_resolved_path() {
    let state = mock_state(Some("test-idx-exact")).await;
    let family_id = format!("test-idx-exact-{}", uuid::Uuid::new_v4());

    let h = put_test_content(&state, &family_id, b"body{color:red}").await;

    let pub_p = json!({
        "family_id": &family_id,
        "collection": "static-site",
        "manifest": {"/style.css": &h}
    });
    content_publish(Some(&pub_p), &state).await.unwrap();

    let resolve_p = json!({
        "family_id": &family_id,
        "collection": "static-site",
        "path": "/style.css"
    });
    let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
    assert_eq!(r["hash"], h);
    assert!(
        r.get("resolved_path").is_none(),
        "exact match should not have resolved_path"
    );

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn resolve_includes_timing_metadata() {
    let state = mock_state(Some("test-timing")).await;
    let family_id = format!("test-timing-{}", uuid::Uuid::new_v4());

    let h = put_test_content(&state, &family_id, b"content").await;

    let pub_p = json!({
        "family_id": &family_id,
        "collection": "timed",
        "manifest": {"/x": &h}
    });
    content_publish(Some(&pub_p), &state).await.unwrap();

    let resolve_p = json!({
        "family_id": &family_id,
        "collection": "timed",
        "path": "/x"
    });
    let r = content_resolve(Some(&resolve_p), &state).await.unwrap();
    assert!(
        r["resolved_in_ms"].is_number(),
        "should include resolved_in_ms"
    );

    let get_p = json!({"hash": &h, "family_id": &family_id});
    let r = content_get(Some(&get_p), &state).await.unwrap();
    assert!(
        r["retrieved_in_ms"].is_number(),
        "should include retrieved_in_ms"
    );

    cleanup_family(&family_id).await;
}

// ── SP-4 compatibility: content_base64 alias + nested metadata ─────────

#[tokio::test]
#[serial]
async fn content_put_accepts_content_base64_alias() {
    let state = mock_state(Some("test-sp4-alias")).await;
    let family_id = format!("test-sp4-alias-{}", uuid::Uuid::new_v4());

    let content = b"<html>SP-4 test</html>";
    let encoded = STANDARD.encode(content);

    let p = json!({
        "family_id": &family_id,
        "content_base64": &encoded,
        "content_type": "text/html"
    });
    let r = content_put(Some(&p), &state).await.unwrap();
    assert_eq!(r["stored"], true);
    assert!(!r["hash"].as_str().unwrap().is_empty());

    let hash = r["hash"].as_str().unwrap();
    let get_p = json!({"hash": hash, "family_id": &family_id});
    let r = content_get(Some(&get_p), &state).await.unwrap();
    let decoded = STANDARD.decode(r["data"].as_str().unwrap()).unwrap();
    assert_eq!(decoded, content);

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn content_put_extracts_nested_metadata() {
    let state = mock_state(Some("test-sp4-meta")).await;
    let family_id = format!("test-sp4-meta-{}", uuid::Uuid::new_v4());

    let encoded = STANDARD.encode(b"metadata-test");
    let p = json!({
        "family_id": &family_id,
        "data": &encoded,
        "content_type": "text/plain",
        "metadata": {
            "source": "primalSpring",
            "pipeline": "SP-4"
        }
    });
    let r = content_put(Some(&p), &state).await.unwrap();
    assert_eq!(r["stored"], true);

    let hash = r["hash"].as_str().unwrap();
    let get_p = json!({"hash": hash, "family_id": &family_id});
    let r = content_get(Some(&get_p), &state).await.unwrap();
    assert_eq!(r["source"], "primalSpring");
    assert_eq!(r["pipeline"], "SP-4");

    cleanup_family(&family_id).await;
}

#[tokio::test]
#[serial]
async fn content_put_top_level_provenance_overrides_nested() {
    let state = mock_state(Some("test-sp4-override")).await;
    let family_id = format!("test-sp4-override-{}", uuid::Uuid::new_v4());

    let encoded = STANDARD.encode(b"override-test");
    let p = json!({
        "family_id": &family_id,
        "data": &encoded,
        "source": "direct",
        "metadata": {
            "source": "nested-should-lose"
        }
    });
    let r = content_put(Some(&p), &state).await.unwrap();
    let hash = r["hash"].as_str().unwrap();
    let get_p = json!({"hash": hash, "family_id": &family_id});
    let r = content_get(Some(&get_p), &state).await.unwrap();
    assert_eq!(
        r["source"], "direct",
        "top-level source should take precedence"
    );

    cleanup_family(&family_id).await;
}
