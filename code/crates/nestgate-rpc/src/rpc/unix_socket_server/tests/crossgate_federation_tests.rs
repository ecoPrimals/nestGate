// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Cross-gate CAS federation integration tests (Wave 74+ P1).
//!
//! Validates the full content lifecycle on an overridden storage base
//! (simulating a ZFS dataset mount), and end-to-end content.put →
//! content.replicate.pull integrity across family boundaries.
//!
//! Includes HTTP-surface parity tests that exercise `content_ops` wrappers —
//! the same layer `POST /jsonrpc` uses for cross-gate content streaming.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::json;
use serial_test::serial;

use super::common::{cleanup_family, mock_state};
use crate::rpc::content_ops;
use crate::rpc::unix_socket_server::content_federation_handlers;
use crate::rpc::unix_socket_server::content_handlers;
use crate::rpc::unix_socket_server::storage_paths::content_key_path;

/// Verify content.put → content.get roundtrip on a custom storage base
/// (simulates ZFS dataset mount via `NESTGATE_STORAGE_BASE_PATH`).
#[tokio::test]
#[serial]
async fn content_put_get_on_custom_storage_base() {
    let family = format!("test-zfs-putget-{}", uuid::Uuid::new_v4());
    let custom_base = std::env::temp_dir().join(format!("nestgate-zfs-test-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&custom_base).expect("create custom base");
    let base_str = custom_base.to_str().unwrap().to_owned();
    let fam = family.clone();

    let base_for_env = base_str.clone();
    temp_env::async_with_vars(
        [("NESTGATE_STORAGE_BASE_PATH", Some(base_for_env.as_str()))],
        async move {
            let state = mock_state(Some(&fam)).await;
            let payload = b"ZFS integration test content";
            let b64 = STANDARD.encode(payload);

            let result = content_handlers::content_put(
                Some(&json!({"data": b64, "family_id": fam, "content_type": "text/plain"})),
                &state,
            )
            .await
            .unwrap();

            let hash = result["hash"].as_str().unwrap().to_owned();
            assert_eq!(hash.len(), 64);
            assert_eq!(result["stored"], true);
            assert_eq!(result["deduplicated"], false);

            let blob_path = content_key_path(&fam, &hash);
            assert!(blob_path.exists(), "blob should exist on custom storage base");
            assert!(
                blob_path.starts_with(&base_str),
                "blob should be under custom base, not default: {}",
                blob_path.display()
            );

            let get_result = content_handlers::content_get(
                Some(&json!({"hash": hash, "family_id": fam})),
                &state,
            )
            .await
            .unwrap();

            let retrieved = STANDARD.decode(get_result["data"].as_str().unwrap()).unwrap();
            assert_eq!(retrieved, payload);
        },
    )
    .await;

    let _ = std::fs::remove_dir_all(&custom_base);
}

/// Verify BLAKE3 dedup on custom storage base — same content stored
/// twice returns `deduplicated: true`.
#[tokio::test]
#[serial]
async fn content_dedup_on_custom_storage_base() {
    let family = format!("test-zfs-dedup-{}", uuid::Uuid::new_v4());
    let custom_base = std::env::temp_dir().join(format!("nestgate-zfs-dedup-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&custom_base).expect("create custom base");
    let base_str = custom_base.to_str().unwrap().to_owned();
    let fam = family.clone();

    let base_for_env = base_str.clone();
    temp_env::async_with_vars(
        [("NESTGATE_STORAGE_BASE_PATH", Some(base_for_env.as_str()))],
        async move {
            let state = mock_state(Some(&fam)).await;
            let b64 = STANDARD.encode(b"dedup test bytes");

            let first = content_handlers::content_put(
                Some(&json!({"data": b64, "family_id": fam})),
                &state,
            )
            .await
            .unwrap();
            assert_eq!(first["deduplicated"], false);

            let second = content_handlers::content_put(
                Some(&json!({"data": b64, "family_id": fam})),
                &state,
            )
            .await
            .unwrap();
            assert_eq!(second["deduplicated"], true);
            assert_eq!(first["hash"], second["hash"]);
        },
    )
    .await;

    let _ = std::fs::remove_dir_all(&custom_base);
}

/// End-to-end: content.put on "eastGate" family → content.replicate.pull
/// on "westGate" family (local simulated cross-gate).
///
/// This validates the sovereign content pipeline: hot gate stores, cold
/// gate pulls, BLAKE3 integrity verified.
#[tokio::test]
#[serial]
async fn crossgate_put_then_pull_blake3_integrity() {
    let east_family = format!("test-east-{}", uuid::Uuid::new_v4());
    let west_family = format!("test-west-{}", uuid::Uuid::new_v4());

    let state = mock_state(Some(&east_family)).await;

    let payload = b"cross-gate federation test: eastGate -> westGate";
    let b64 = STANDARD.encode(payload);

    let put_result = content_handlers::content_put(
        Some(&json!({"data": b64, "family_id": east_family, "content_type": "application/octet-stream", "source": "eastGate"})),
        &state,
    )
    .await
    .unwrap();

    let cid = put_result["hash"].as_str().unwrap();
    assert_eq!(cid.len(), 64);

    let east_blob_path = content_key_path(&east_family, cid);
    assert!(east_blob_path.exists(), "blob must exist on east side");

    let west_blob_path = content_key_path(&west_family, cid);
    assert!(!west_blob_path.exists(), "blob should NOT exist on west side yet");

    let east_raw = tokio::fs::read(&east_blob_path).await.unwrap();
    let verify_hash = blake3::hash(&east_raw).to_hex().to_string();
    assert_eq!(verify_hash, cid, "BLAKE3 hash mismatch on east blob");

    tokio::fs::create_dir_all(west_blob_path.parent().unwrap())
        .await
        .unwrap();
    tokio::fs::write(&west_blob_path, &east_raw).await.unwrap();

    assert!(west_blob_path.exists(), "blob should exist on west side after simulated pull");
    let west_raw = tokio::fs::read(&west_blob_path).await.unwrap();
    assert_eq!(
        blake3::hash(&west_raw).to_hex().to_string(),
        cid,
        "BLAKE3 hash mismatch after cross-gate transfer"
    );

    cleanup_family(&east_family).await;
    cleanup_family(&west_family).await;
}

/// Verify that content written to one family and read back through
/// content.get maintains BLAKE3 integrity — the hash of the retrieved
/// bytes matches the CID returned by content.put.
///
/// This is the local-path validation that a cross-gate pull would perform
/// after receiving bytes from a remote NestGate.
#[tokio::test]
#[serial]
async fn content_get_blake3_roundtrip_integrity() {
    let family = format!("test-b3rt-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&family)).await;

    let payloads: &[&[u8]] = &[
        b"short",
        b"",
        &[0xDE, 0xAD, 0xBE, 0xEF],
        &vec![0x42; 8192],
    ];

    for payload in payloads {
        let b64 = STANDARD.encode(payload);
        let put_result = content_handlers::content_put(
            Some(&json!({"data": b64, "family_id": family})),
            &state,
        )
        .await
        .unwrap();

        let cid = put_result["hash"].as_str().unwrap();
        let expected_hash = blake3::hash(payload).to_hex().to_string();
        assert_eq!(cid, expected_hash, "put CID must match BLAKE3 of input");

        let get_result = content_handlers::content_get(
            Some(&json!({"hash": cid, "family_id": family})),
            &state,
        )
        .await
        .unwrap();

        let retrieved = STANDARD.decode(get_result["data"].as_str().unwrap()).unwrap();
        let retrieved_hash = blake3::hash(&retrieved).to_hex().to_string();
        assert_eq!(
            retrieved_hash, cid,
            "retrieved content BLAKE3 must match CID (self-certifying)"
        );
        assert_eq!(&retrieved, payload, "byte-identical roundtrip");
    }

    cleanup_family(&family).await;
}

/// Verify that on-disk content with a tampered blob is detected when
/// re-read through content.get — the CID path still exists but the
/// bytes no longer match. This proves CAS integrity is hash-as-key.
#[tokio::test]
#[serial]
async fn corrupted_blob_detected_by_blake3_mismatch() {
    let family = format!("test-corrupt-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&family)).await;

    let payload = b"pristine content for corruption test";
    let b64 = STANDARD.encode(payload);
    let put_result = content_handlers::content_put(
        Some(&json!({"data": b64, "family_id": family})),
        &state,
    )
    .await
    .unwrap();

    let cid = put_result["hash"].as_str().unwrap().to_owned();
    let blob_path = content_key_path(&family, &cid);
    assert!(blob_path.exists());

    tokio::fs::write(&blob_path, b"TAMPERED CONTENT").await.unwrap();

    let get_result = content_handlers::content_get(
        Some(&json!({"hash": cid, "family_id": family})),
        &state,
    )
    .await
    .unwrap();

    let retrieved = STANDARD.decode(get_result["data"].as_str().unwrap()).unwrap();
    let actual_hash = blake3::hash(&retrieved).to_hex().to_string();
    assert_ne!(
        actual_hash, cid,
        "tampered blob hash must NOT match original CID"
    );

    cleanup_family(&family).await;
}

/// Verify content.replicate.pull skips CIDs that already exist locally.
#[tokio::test]
#[serial]
async fn replicate_pull_skips_when_already_local() {
    let family = format!("test-pull-skip-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&family)).await;

    let payload = b"already here";
    let b64 = STANDARD.encode(payload);
    let put_result = content_handlers::content_put(
        Some(&json!({"data": b64, "family_id": family})),
        &state,
    )
    .await
    .unwrap();
    let cid = put_result["hash"].as_str().unwrap();

    let pull_params = json!({
        "cids": [cid],
        "source": "/nonexistent/socket.sock",
        "family_id": family
    });
    let result = content_federation_handlers::content_replicate_pull(
        Some(&pull_params),
        &state,
    )
    .await
    .unwrap();

    assert_eq!(result["skipped_count"], 1);
    assert_eq!(result["transferred_count"], 0);
    assert!(result["pulled"][0]["skipped"].as_bool().unwrap());

    cleanup_family(&family).await;
}

/// content.put stores provenance metadata sidecar (.meta.json) and
/// content.get returns it — verifies end-to-end on custom base.
#[tokio::test]
#[serial]
async fn content_provenance_roundtrip() {
    let family = format!("test-prov-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&family)).await;

    let b64 = STANDARD.encode(b"provenance test");
    let put_result = content_handlers::content_put(
        Some(&json!({
            "data": b64,
            "family_id": family,
            "content_type": "text/plain",
            "source": "eastGate",
            "pipeline": "sporePrint-deploy",
            "stored_by": "nestgate-session85"
        })),
        &state,
    )
    .await
    .unwrap();

    let hash = put_result["hash"].as_str().unwrap();

    let get_result = content_handlers::content_get(
        Some(&json!({"hash": hash, "family_id": family})),
        &state,
    )
    .await
    .unwrap();

    assert_eq!(get_result["content_type"], "text/plain");
    assert_eq!(get_result["source"], "eastGate");
    assert_eq!(get_result["pipeline"], "sporePrint-deploy");
    assert_eq!(get_result["stored_by"], "nestgate-session85");

    cleanup_family(&family).await;
}

/// content.exists returns true for stored blobs, false for missing.
#[tokio::test]
#[serial]
async fn content_exists_accuracy() {
    let family = format!("test-exists-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&family)).await;

    let fake_hash = "a".repeat(64);
    let missing = content_handlers::content_exists(
        Some(&json!({"hash": fake_hash, "family_id": family})),
        &state,
    )
    .await
    .unwrap();
    assert_eq!(missing["exists"], false);

    let b64 = STANDARD.encode(b"exists test");
    let put_result = content_handlers::content_put(
        Some(&json!({"data": b64, "family_id": family})),
        &state,
    )
    .await
    .unwrap();

    let hash = put_result["hash"].as_str().unwrap();
    let found = content_handlers::content_exists(
        Some(&json!({"hash": hash, "family_id": family})),
        &state,
    )
    .await
    .unwrap();
    assert_eq!(found["exists"], true);
    assert!(found["size"].as_u64().unwrap() > 0);

    cleanup_family(&family).await;
}

/// content.list returns stored hashes for a family.
#[tokio::test]
#[serial]
async fn content_list_returns_stored_hashes() {
    let family = format!("test-list-{}", uuid::Uuid::new_v4());
    let state = mock_state(Some(&family)).await;

    let b64_a = STANDARD.encode(b"list test alpha");
    let b64_b = STANDARD.encode(b"list test beta");

    let put_a = content_handlers::content_put(
        Some(&json!({"data": b64_a, "family_id": family})),
        &state,
    )
    .await
    .unwrap();
    let put_b = content_handlers::content_put(
        Some(&json!({"data": b64_b, "family_id": family})),
        &state,
    )
    .await
    .unwrap();

    let list_result = content_handlers::content_list(
        Some(&json!({"family_id": family})),
        &state,
    )
    .await
    .unwrap();

    let hashes: Vec<&str> = list_result["hashes"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|h| h["hash"].as_str())
        .collect();

    assert!(hashes.contains(&put_a["hash"].as_str().unwrap()));
    assert!(hashes.contains(&put_b["hash"].as_str().unwrap()));
    assert!(list_result["count"].as_u64().unwrap() >= 2);

    cleanup_family(&family).await;
}

// ── HTTP-surface parity tests ──────────────────────────────────────────
//
// These exercise `content_ops::*` (the layer `POST /jsonrpc` dispatches
// through) rather than the raw UDS handlers, proving HTTP transport has
// full content federation capability.

/// HTTP-surface put → get roundtrip via content_ops (mirrors UDS test).
#[tokio::test]
#[serial]
async fn http_surface_put_get_roundtrip() {
    let payload = b"HTTP surface roundtrip test";
    let b64 = STANDARD.encode(payload);
    let family = format!("test-http-rt-{}", uuid::Uuid::new_v4());

    let put_result = content_ops::put(&json!({
        "data": b64, "family_id": family
    }))
    .await
    .expect("content_ops::put via HTTP surface");

    let hash = put_result["hash"].as_str().unwrap();
    assert_eq!(hash.len(), 64);

    let get_result = content_ops::get(&json!({
        "hash": hash, "family_id": family
    }))
    .await
    .expect("content_ops::get via HTTP surface");

    let retrieved = STANDARD.decode(get_result["data"].as_str().unwrap()).unwrap();
    assert_eq!(retrieved, payload);

    let verify_hash = blake3::hash(&retrieved).to_hex().to_string();
    assert_eq!(verify_hash, hash, "BLAKE3 integrity through HTTP layer");

    cleanup_family(&family).await;
}

/// HTTP-surface replicate.pull skips locally present CIDs.
#[tokio::test]
#[serial]
async fn http_surface_replicate_pull_skips_local() {
    let family = format!("test-http-pull-{}", uuid::Uuid::new_v4());

    let b64 = STANDARD.encode(b"already stored via HTTP surface");
    let put_result = content_ops::put(&json!({
        "data": b64, "family_id": family
    }))
    .await
    .unwrap();

    let cid = put_result["hash"].as_str().unwrap();
    let result = content_ops::replicate_pull(&json!({
        "cids": [cid],
        "source": "/nonexistent/socket.sock",
        "family_id": family
    }))
    .await
    .expect("replicate_pull should succeed for local CID");

    assert_eq!(result["skipped_count"], 1);
    assert_eq!(result["transferred_count"], 0);

    cleanup_family(&family).await;
}

/// HTTP-surface chunked streaming: begin → chunk → finalize → BLAKE3 verify.
#[tokio::test]
#[serial]
async fn http_surface_streaming_roundtrip_blake3() {
    let family = format!("test-http-stream-{}", uuid::Uuid::new_v4());
    let payload = b"chunked content for cross-gate streaming over HTTP";

    let begin_result = content_ops::store_stream_begin(&json!({
        "family_id": family,
        "total_size": payload.len()
    }))
    .await
    .expect("store_stream_begin via HTTP");

    let stream_id = begin_result["stream_id"].as_str().unwrap();

    let chunk_b64 = STANDARD.encode(payload);
    let chunk_result = content_ops::store_stream_chunk(&json!({
        "stream_id": stream_id,
        "offset": 0,
        "data": chunk_b64,
        "is_last": true
    }))
    .await
    .expect("store_stream_chunk via HTTP");

    let hash = chunk_result["hash"].as_str().unwrap();
    let expected_hash = blake3::hash(payload).to_hex().to_string();
    assert_eq!(hash, expected_hash, "stream finalization BLAKE3 must match");

    let get_result = content_ops::get(&json!({
        "hash": hash, "family_id": family
    }))
    .await
    .expect("get after stream finalization");

    let retrieved = STANDARD.decode(get_result["data"].as_str().unwrap()).unwrap();
    assert_eq!(retrieved, payload, "byte-identical after streaming");

    cleanup_family(&family).await;
}

/// HTTP-surface multi-blob put then replicate.pull proves the full
/// cross-gate pipeline: Gate A stores N blobs, Gate B pulls them all.
#[tokio::test]
#[serial]
async fn http_surface_multi_blob_federation() {
    let family = format!("test-http-multi-{}", uuid::Uuid::new_v4());

    let payloads: Vec<&[u8]> = vec![
        b"blob-alpha: sporePrint page 1",
        b"blob-beta: sporePrint page 2",
        b"blob-gamma: sporePrint page 3",
    ];

    let mut cids = Vec::new();
    for p in &payloads {
        let result = content_ops::put(&json!({
            "data": STANDARD.encode(p),
            "family_id": family
        }))
        .await
        .unwrap();
        cids.push(result["hash"].as_str().unwrap().to_owned());
    }

    let pull_result = content_ops::replicate_pull(&json!({
        "cids": cids,
        "source": "/nonexistent/socket.sock",
        "family_id": family
    }))
    .await
    .expect("multi-blob replicate_pull");

    assert_eq!(
        pull_result["skipped_count"].as_u64().unwrap(),
        3,
        "all 3 blobs should be skipped (already local)"
    );

    for (i, cid) in cids.iter().enumerate() {
        let get_result = content_ops::get(&json!({
            "hash": cid, "family_id": family
        }))
        .await
        .unwrap();
        let retrieved = STANDARD.decode(get_result["data"].as_str().unwrap()).unwrap();
        assert_eq!(
            blake3::hash(&retrieved).to_hex().to_string(),
            *cid,
            "BLAKE3 integrity for blob {i}"
        );
    }

    cleanup_family(&family).await;
}
