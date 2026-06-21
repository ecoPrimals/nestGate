// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use nestgate_config::config::storage_paths::get_storage_base_path;
use serde_json::json;

async fn cleanup_family(family_id: &str) {
    let base = get_storage_base_path().join("datasets").join(family_id);
    let _ = tokio::fs::remove_dir_all(&base).await;
}

#[tokio::test]
async fn content_stream_store_and_retrieve_roundtrip() {
    let family_id = format!("content-stream-rt-{}", Uuid::new_v4());
    let payload = vec![0xAB_u8; 6000];

    let begin = content_store_stream_begin(
        json!({
            "family_id": family_id,
            "total_size": payload.len(),
            "content_type": "application/octet-stream"
        }),
        Some("default"),
    )
    .await
    .expect("content store stream begin");

    let sid = begin["stream_id"].as_str().unwrap();
    let chunk_size = begin["chunk_size"].as_u64().unwrap() as usize;
    assert!(chunk_size > 0);

    let mut offset = 0_u64;
    while offset < payload.len() as u64 {
        let end = (offset as usize + chunk_size).min(payload.len());
        let is_last = end == payload.len();
        let chunk = &payload[offset as usize..end];
        let result = content_store_stream_chunk(json!({
            "stream_id": sid,
            "offset": offset,
            "data": STANDARD.encode(chunk),
            "is_last": is_last
        }))
        .await
        .expect("content store stream chunk");

        if is_last {
            assert!(result["hash"].is_string());
            assert_eq!(result["stored"], true);
            let hash = result["hash"].as_str().unwrap();
            assert_eq!(hash.len(), 64);
            assert_eq!(content_hash_hex(&payload), hash, "BLAKE3 mismatch");

            let r_begin = content_retrieve_stream_begin(
                json!({
                    "family_id": family_id,
                    "hash": hash
                }),
                Some("default"),
            )
            .await
            .expect("content retrieve stream begin");

            let rsid = r_begin["stream_id"].as_str().unwrap();
            let total = r_begin["total_size"].as_u64().unwrap();
            assert_eq!(total, payload.len() as u64);

            let mut ro = 0_u64;
            let mut out = Vec::new();
            loop {
                let part = crate::rpc::storage_stream::storage_retrieve_stream_chunk(json!({
                    "stream_id": rsid,
                    "offset": ro
                }))
                .await
                .expect("content retrieve chunk");
                let bytes = STANDARD.decode(part["data"].as_str().unwrap()).unwrap();
                out.extend_from_slice(&bytes);
                ro += bytes.len() as u64;
                if part["is_last"].as_bool() == Some(true) {
                    break;
                }
            }

            assert_eq!(out, payload);
        }

        offset = end as u64;
    }

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn content_stream_dedup_on_second_upload() {
    let family_id = format!("content-stream-dedup-{}", Uuid::new_v4());
    let payload = b"dedup stream test";

    let store_once = |fam: &str, data: &[u8]| {
        let fam = fam.to_owned();
        let data = data.to_vec();
        async move {
            let begin = content_store_stream_begin(
                json!({"family_id": fam, "total_size": data.len()}),
                Some("default"),
            )
            .await?;
            let sid = begin["stream_id"].as_str().unwrap().to_owned();
            content_store_stream_chunk(json!({
                "stream_id": sid,
                "offset": 0,
                "data": STANDARD.encode(&data),
                "is_last": true
            }))
            .await
        }
    };

    let first = store_once(&family_id, payload).await.unwrap();
    assert_eq!(first["deduplicated"], false);

    let second = store_once(&family_id, payload).await.unwrap();
    assert_eq!(second["deduplicated"], true);
    assert_eq!(first["hash"], second["hash"]);

    cleanup_family(&family_id).await;
}

#[tokio::test]
async fn content_retrieve_stream_not_found() {
    let family_id = format!("content-stream-404-{}", Uuid::new_v4());
    let fake_hash = "b".repeat(64);
    let err = content_retrieve_stream_begin(
        json!({"family_id": family_id, "hash": fake_hash}),
        Some("default"),
    )
    .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn begin_rejects_missing_total_size() {
    let fam = format!("cs-no-size-{}", Uuid::new_v4());
    let err = content_store_stream_begin(json!({"family_id": fam}), None).await;
    assert!(err.is_err(), "missing total_size should be rejected");
    cleanup_family(&fam).await;
}

#[tokio::test]
async fn begin_rejects_oversized_total() {
    let fam = format!("cs-oversize-{}", Uuid::new_v4());
    let err = content_store_stream_begin(
        json!({"family_id": fam, "total_size": MAX_STREAM_TOTAL + 1}),
        None,
    )
    .await;
    assert!(err.is_err());
    cleanup_family(&fam).await;
}

#[tokio::test]
async fn chunk_rejects_unknown_stream_id() {
    let err = content_store_stream_chunk(json!({
        "stream_id": "nonexistent-uuid",
        "offset": 0,
        "data": STANDARD.encode(b"x"),
        "is_last": true
    }))
    .await;
    assert!(err.is_err());
    let msg = format!("{}", err.unwrap_err());
    assert!(msg.contains("unknown") || msg.contains("expired"));
}

#[tokio::test]
async fn chunk_rejects_bad_offset() {
    let fam = format!("cs-offset-{}", Uuid::new_v4());
    let begin = content_store_stream_begin(json!({"family_id": fam, "total_size": 100}), None)
        .await
        .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();

    let err = content_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 50,
        "data": STANDARD.encode(b"data"),
        "is_last": false
    }))
    .await;
    assert!(err.is_err());
    let msg = format!("{}", err.unwrap_err());
    assert!(msg.contains("expected 0"));
    cleanup_family(&fam).await;
}

#[tokio::test]
async fn chunk_rejects_exceeding_total_size() {
    let fam = format!("cs-exceed-{}", Uuid::new_v4());
    let begin = content_store_stream_begin(json!({"family_id": fam, "total_size": 4}), None)
        .await
        .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();

    let err = content_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0,
        "data": STANDARD.encode(b"too much data for 4 bytes"),
        "is_last": true
    }))
    .await;
    assert!(err.is_err());
    let msg = format!("{}", err.unwrap_err());
    assert!(msg.contains("exceed"));
    cleanup_family(&fam).await;
}

#[tokio::test]
async fn chunk_rejects_invalid_base64() {
    let fam = format!("cs-b64-{}", Uuid::new_v4());
    let begin = content_store_stream_begin(json!({"family_id": fam, "total_size": 100}), None)
        .await
        .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();

    let err = content_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0,
        "data": "not!valid!base64!!!",
        "is_last": false
    }))
    .await;
    assert!(err.is_err());
    let msg = format!("{}", err.unwrap_err());
    assert!(msg.contains("base64"));
    cleanup_family(&fam).await;
}

#[tokio::test]
async fn begin_missing_family_id_errors() {
    let err = content_store_stream_begin(json!({"total_size": 10}), None).await;
    assert!(err.is_err());
    let msg = format!("{}", err.unwrap_err());
    assert!(msg.contains("family_id") || msg.contains("family"));
}

#[tokio::test]
async fn retrieve_begin_invalid_hash_format() {
    let fam = format!("cs-badhash-{}", Uuid::new_v4());
    let err =
        content_retrieve_stream_begin(json!({"family_id": fam, "hash": "tooshort"}), None).await;
    assert!(err.is_err());
    cleanup_family(&fam).await;
}

#[tokio::test]
async fn begin_uses_family_fallback() {
    let fam = format!("cs-fallback-{}", Uuid::new_v4());
    let result = content_store_stream_begin(json!({"total_size": 16}), Some(&fam)).await;
    assert!(result.is_ok(), "should succeed with fallback family");
    cleanup_family(&fam).await;
}

#[tokio::test]
async fn empty_upload_fast_path_returns_hash() {
    let fam = format!("cs-empty-{}", Uuid::new_v4());
    let result = content_store_stream_begin(
        json!({"family_id": &fam, "total_size": 0, "content_type": "text/plain"}),
        None,
    )
    .await
    .expect("empty upload should succeed");

    let expected_hash = content_hash_hex(&[]);
    assert_eq!(result["hash"].as_str().unwrap(), expected_hash);
    assert_eq!(result["size"], 0);
    assert_eq!(result["stored"], true);
    assert_eq!(result["family_id"].as_str().unwrap(), fam);
    assert!(result.get("stream_id").is_some());

    let cas_path = content_cas_path(&fam, &expected_hash);
    assert!(cas_path.exists(), "CAS file should exist for empty content");
    let data = tokio::fs::read(&cas_path).await.unwrap();
    assert!(
        data.is_empty(),
        "empty content should be zero bytes on disk"
    );

    cleanup_family(&fam).await;
}

#[tokio::test]
async fn empty_upload_dedup_on_second_call() {
    let fam = format!("cs-empty-dedup-{}", Uuid::new_v4());
    let params = json!({"family_id": &fam, "total_size": 0});

    let first = content_store_stream_begin(params.clone(), None)
        .await
        .unwrap();
    let second = content_store_stream_begin(params, None).await.unwrap();

    assert_eq!(first["hash"], second["hash"]);
    assert_eq!(second["deduplicated"], true);

    cleanup_family(&fam).await;
}

#[tokio::test]
async fn chunk_rejects_oversized_decoded_data() {
    let fam = format!("cs-chunkmax-{}", Uuid::new_v4());
    let total = MAX_STREAM_CHUNK + 1024;
    let begin = content_store_stream_begin(json!({"family_id": &fam, "total_size": total}), None)
        .await
        .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();

    let oversized = vec![0xCC_u8; (MAX_STREAM_CHUNK + 1) as usize];
    let err = content_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0,
        "data": STANDARD.encode(&oversized),
        "is_last": false
    }))
    .await;
    assert!(err.is_err());
    let msg = format!("{}", err.unwrap_err());
    assert!(msg.contains("chunk exceeds") || msg.contains("MAX_STREAM_CHUNK"));

    cleanup_family(&fam).await;
}

#[tokio::test]
async fn retrieve_stream_begin_valid_hash_returns_stream() {
    let fam = format!("cs-retrieve-ok-{}", Uuid::new_v4());
    let payload = b"retrieve me via stream";

    let begin = content_store_stream_begin(
        json!({"family_id": &fam, "total_size": payload.len()}),
        None,
    )
    .await
    .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();

    let fin = content_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0,
        "data": STANDARD.encode(payload),
        "is_last": true
    }))
    .await
    .unwrap();
    let hash = fin["hash"].as_str().unwrap();

    let r = content_retrieve_stream_begin(json!({"family_id": &fam, "hash": hash}), None)
        .await
        .expect("should open retrieve stream");
    assert!(r.get("stream_id").is_some());
    assert_eq!(r["total_size"].as_u64().unwrap(), payload.len() as u64);

    cleanup_family(&fam).await;
}

#[tokio::test]
async fn chunk_missing_stream_id_param_errors() {
    let err = content_store_stream_chunk(json!({
        "offset": 0,
        "data": STANDARD.encode(b"x"),
        "is_last": true
    }))
    .await;
    assert!(err.is_err(), "missing stream_id should be rejected");
}

#[tokio::test]
async fn chunk_missing_offset_param_errors() {
    let fam = format!("cs-no-offset-{}", Uuid::new_v4());
    let begin = content_store_stream_begin(json!({"family_id": &fam, "total_size": 100}), None)
        .await
        .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();

    let err = content_store_stream_chunk(json!({
        "stream_id": sid,
        "data": STANDARD.encode(b"x"),
        "is_last": false
    }))
    .await;
    assert!(err.is_err(), "missing offset should be rejected");
    cleanup_family(&fam).await;
}

#[tokio::test]
async fn chunk_missing_data_param_errors() {
    let fam = format!("cs-no-data-{}", Uuid::new_v4());
    let begin = content_store_stream_begin(json!({"family_id": &fam, "total_size": 100}), None)
        .await
        .unwrap();
    let sid = begin["stream_id"].as_str().unwrap();

    let err = content_store_stream_chunk(json!({
        "stream_id": sid,
        "offset": 0,
        "is_last": false
    }))
    .await;
    assert!(err.is_err(), "missing data should be rejected");
    cleanup_family(&fam).await;
}
