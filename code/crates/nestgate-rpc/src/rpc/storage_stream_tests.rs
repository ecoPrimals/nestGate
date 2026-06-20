// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for [`super::storage_stream`] chunked upload/download flows.

use super::*;
use serde_json::json;

async fn cleanup_family(family_id: &str) {
    let base = get_storage_base_path().join("datasets").join(family_id);
    let _ = tokio::fs::remove_dir_all(&base).await;
}

#[tokio::test]
async fn store_and_retrieve_stream_roundtrip_multichunk() {
    let family_id = format!("stream-test-{}", Uuid::new_v4());
    let dataset = "tensors";
    let key = "weights.bin";
    let payload: Vec<u8> = (0_u8..=211).cycle().take(9000).collect();

    let begin = storage_store_stream_begin(
        json!({
            "family_id": family_id,
            "dataset": dataset,
            "key": key,
            "total_size": payload.len(),
            "content_type": "application/octet-stream"
        }),
        Some("default"),
    )
    .await
    .expect("begin");

    let stream_id = begin["stream_id"].as_str().unwrap().to_string();

    let chunk_size = 4000usize;
    let mut offset = 0_u64;
    while offset < payload.len() as u64 {
        let end = usize::min(offset as usize + chunk_size, payload.len());
        let piece = &payload[offset as usize..end];
        let is_last = end == payload.len();
        let chunk = storage_store_stream_chunk(json!({
            "stream_id": stream_id,
            "offset": offset,
            "data": STANDARD.encode(piece),
            "is_last": is_last
        }))
        .await
        .expect("chunk");
        if !is_last {
            assert_eq!(chunk["ack"], true);
        } else {
            assert_eq!(chunk["status"], "stored");
            assert_eq!(chunk["size"], payload.len());
        }
        offset = end as u64;
    }

    let r_begin = storage_retrieve_stream_begin(
        json!({
            "family_id": family_id,
            "dataset": dataset,
            "key": key,
            "chunk_size": 3000
        }),
        Some("default"),
    )
    .await
    .expect("retrieve begin");

    let rsid = r_begin["stream_id"].as_str().unwrap();
    let total_size = r_begin["total_size"].as_u64().unwrap();
    assert_eq!(total_size, payload.len() as u64);

    let mut ro = 0_u64;
    let mut out = Vec::new();
    loop {
        let part = storage_retrieve_stream_chunk(json!({
            "stream_id": rsid,
            "offset": ro
        }))
        .await
        .expect("retrieve chunk");
        let b64 = part["data"].as_str().expect("data");
        let bytes = STANDARD.decode(b64).expect("b64");
        out.extend_from_slice(&bytes);
        ro = ro.saturating_add(bytes.len() as u64);
        if part["is_last"].as_bool() == Some(true) {
            break;
        }
    }

    assert_eq!(out, payload);

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn store_stream_rejects_out_of_order_chunk() {
    let family_id = format!("stream-oo-{}", Uuid::new_v4());
    let begin = storage_store_stream_begin(
        json!({
            "family_id": family_id,
            "dataset": "d",
            "key": "k",
            "total_size": 10
        }),
        Some("default"),
    )
    .await
    .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();
    let err = storage_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 5,
        "data": STANDARD.encode([0_u8; 5]),
        "is_last": false
    }))
    .await
    .expect_err("out of order");
    assert!(!err.to_string().is_empty());

    let mut guard = maps().lock().await;
    let _ = guard.uploads.remove(sid);
    drop(guard);
    cleanup_family(&family_id).await;
}

#[test]
fn validate_segment_rejects_empty() {
    assert!(validate_segment("", "f").is_err());
}

#[test]
fn validate_segment_rejects_path_separators() {
    assert!(validate_segment("a/b", "f").is_err());
    assert!(validate_segment("a\\b", "f").is_err());
}

#[test]
fn validate_segment_rejects_dot_dot() {
    assert!(validate_segment("..", "f").is_err());
    assert!(validate_segment("a..b", "f").is_err());
}

#[test]
fn validate_segment_rejects_leading_dot() {
    assert!(validate_segment(".hidden", "f").is_err());
}

#[test]
fn validate_segment_accepts_normal_names() {
    assert!(validate_segment("my-dataset", "f").is_ok());
    assert!(validate_segment("key_123", "f").is_ok());
}

#[test]
fn resolve_family_id_from_params() {
    let params = json!({"family_id": "abc"});
    assert_eq!(resolve_family_id(&params, None).unwrap(), "abc");
}

#[test]
fn resolve_family_id_uses_fallback() {
    let params = json!({});
    assert_eq!(resolve_family_id(&params, Some("fb")).unwrap(), "fb");
}

#[test]
fn resolve_family_id_requires_at_least_fallback() {
    let params = json!({});
    assert!(resolve_family_id(&params, None).is_err());
}

#[test]
fn extract_dataset_defaults_to_shared() {
    let params = json!({});
    assert_eq!(extract_dataset(&params), "shared");
}

#[test]
fn extract_dataset_prefers_dataset_over_namespace() {
    let params = json!({"dataset": "ds1", "namespace": "ns2"});
    assert_eq!(extract_dataset(&params), "ds1");
}

#[test]
fn extract_dataset_uses_namespace_alias() {
    let params = json!({"namespace": "ns"});
    assert_eq!(extract_dataset(&params), "ns");
}

#[test]
fn ttl_expired_returns_false_for_now() {
    assert!(!ttl_expired(Instant::now()));
}

#[tokio::test]
async fn store_stream_zero_byte_upload() {
    let family_id = format!("stream-zero-{}", Uuid::new_v4());
    let result = storage_store_stream_begin(
        json!({
            "family_id": family_id,
            "dataset": "d",
            "key": "empty.bin",
            "total_size": 0
        }),
        Some("default"),
    )
    .await
    .expect("zero-byte upload");
    assert_eq!(result["status"], "stored");
    assert_eq!(result["size"], 0);
    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn store_stream_rejects_oversized_total() {
    let err = storage_store_stream_begin(
        json!({
            "family_id": "oversized-test",
            "dataset": "d",
            "key": "k",
            "total_size": MAX_STREAM_TOTAL + 1
        }),
        Some("default"),
    )
    .await
    .expect_err("should reject");
    assert!(err.to_string().contains("exceeds maximum"));
}

#[tokio::test]
async fn store_stream_chunk_rejects_unknown_stream() {
    let err = storage_store_stream_chunk(json!({
        "stream_id": "nonexistent-id",
        "offset": 0,
        "data": STANDARD.encode([1_u8; 1]),
        "is_last": true
    }))
    .await
    .expect_err("unknown stream");
    assert!(err.to_string().contains("unknown or expired"));
}

#[tokio::test]
async fn store_stream_chunk_rejects_size_overflow() {
    let family_id = format!("stream-overflow-{}", Uuid::new_v4());
    let begin = storage_store_stream_begin(
        json!({
            "family_id": family_id,
            "dataset": "d",
            "key": "k",
            "total_size": 2
        }),
        Some("default"),
    )
    .await
    .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();
    let err = storage_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0,
        "data": STANDARD.encode([0_u8; 10]),
        "is_last": true
    }))
    .await
    .expect_err("overflow");
    assert!(err.to_string().contains("exceed total_size"));
    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn store_stream_chunk_rejects_size_mismatch_on_last() {
    let family_id = format!("stream-mismatch-{}", Uuid::new_v4());
    let begin = storage_store_stream_begin(
        json!({
            "family_id": family_id,
            "dataset": "d",
            "key": "k",
            "total_size": 10
        }),
        Some("default"),
    )
    .await
    .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();
    let err = storage_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0,
        "data": STANDARD.encode([0_u8; 5]),
        "is_last": true
    }))
    .await
    .expect_err("mismatch");
    assert!(err.to_string().contains("do not match"));
    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn store_stream_chunk_rejects_invalid_base64() {
    let family_id = format!("stream-b64-{}", Uuid::new_v4());
    let begin = storage_store_stream_begin(
        json!({
            "family_id": family_id,
            "dataset": "d",
            "key": "k",
            "total_size": 10
        }),
        Some("default"),
    )
    .await
    .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();
    let err = storage_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0,
        "data": "!!!not-base64!!!",
        "is_last": false
    }))
    .await
    .expect_err("bad base64");
    assert!(err.to_string().contains("base64") || err.to_string().contains("Invalid"));

    let mut guard = maps().lock().await;
    let _ = guard.uploads.remove(sid);
    drop(guard);
    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn retrieve_stream_not_found() {
    let err = storage_retrieve_stream_begin(
        json!({
            "family_id": "nonexistent-fam",
            "dataset": "d",
            "key": "k"
        }),
        None,
    )
    .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn retrieve_stream_chunk_unknown_stream() {
    let err = storage_retrieve_stream_chunk(json!({
        "stream_id": "bogus",
        "offset": 0
    }))
    .await
    .expect_err("unknown");
    assert!(err.to_string().contains("unknown or expired"));
}

#[tokio::test]
async fn retrieve_stream_chunk_offset_past_end() {
    let family_id = format!("stream-past-{}", Uuid::new_v4());
    let key = "small.bin";
    let payload = vec![1_u8; 4];

    let ns = namespaced_blob_path(&family_id, "d", key);
    if let Some(p) = ns.parent() {
        tokio::fs::create_dir_all(p).await.unwrap();
    }
    tokio::fs::write(&ns, &payload).await.unwrap();

    let begin = storage_retrieve_stream_begin(
        json!({"family_id": family_id, "dataset": "d", "key": key}),
        None,
    )
    .await
    .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();

    let err = storage_retrieve_stream_chunk(json!({
        "stream_id": sid,
        "offset": 999
    }))
    .await
    .expect_err("past end");
    assert!(err.to_string().contains("past end"));

    let mut guard = maps().lock().await;
    let _ = guard.retrieves.remove(sid);
    drop(guard);
    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn retrieve_stream_chunk_size_zero_normalized() {
    let family_id = format!("stream-csz-{}", Uuid::new_v4());
    let key = "tiny.bin";
    let payload = vec![42_u8; 16];

    let ns = namespaced_blob_path(&family_id, "d", key);
    if let Some(p) = ns.parent() {
        tokio::fs::create_dir_all(p).await.unwrap();
    }
    tokio::fs::write(&ns, &payload).await.unwrap();

    let begin = storage_retrieve_stream_begin(
        json!({
            "family_id": family_id,
            "dataset": "d",
            "key": key,
            "chunk_size": 0
        }),
        None,
    )
    .await
    .unwrap();
    assert_eq!(begin["chunk_size"], MAX_STREAM_CHUNK);

    let sid = begin["stream_id"].as_str().unwrap();
    let chunk = storage_retrieve_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0
    }))
    .await
    .unwrap();
    assert_eq!(chunk["is_last"], true);
    assert_eq!(chunk["length"], 16);

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn retrieve_stream_falls_back_to_flat_blob_path() {
    let family_id = format!("stream-flat-{}", Uuid::new_v4());
    let key = "legacy.bin";
    let payload = vec![0xCA_u8; 256];

    let flat = flat_blob_path(&family_id, key);
    if let Some(parent) = flat.parent() {
        tokio::fs::create_dir_all(parent).await.unwrap();
    }
    tokio::fs::write(&flat, &payload).await.unwrap();

    let begin = storage_retrieve_stream_begin(
        json!({
            "family_id": family_id,
            "key": key,
            "dataset": "shared"
        }),
        Some("default"),
    )
    .await
    .expect("should find via flat fallback");

    assert_eq!(begin["total_size"], 256);
    let stream_id = begin["stream_id"].as_str().unwrap();

    let chunk = storage_retrieve_stream_chunk(json!({
        "stream_id": stream_id,
        "offset": 0
    }))
    .await
    .expect("chunk");
    let bytes = STANDARD.decode(chunk["data"].as_str().unwrap()).unwrap();
    assert_eq!(bytes, payload);
    assert_eq!(chunk["is_last"], true);

    cleanup_family(&family_id).await;
}
