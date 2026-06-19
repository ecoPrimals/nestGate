// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Direct HTTP content serving — `GET /content/:hash`.
//!
//! Serves raw content-addressed blobs with correct MIME types and immutable
//! cache headers. Designed for reverse proxy (Caddy `nestgate.io/<hash>` →
//! NestGate `GET /content/<hash>`).
//!
//! BLAKE3 hashes are immutable keys — content never changes for a given hash,
//! so aggressive caching is safe.

use axum::{
    extract::{Path, Query},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
};
use serde::Deserialize;
use tracing::debug;

/// Optional query parameters for content serving.
#[derive(Debug, Deserialize)]
pub struct ContentServeQuery {
    /// Override family context (defaults to `NESTGATE_FAMILY_ID` on the server).
    pub family_id: Option<String>,
}

/// `GET /content/:hash` — serve raw content-addressed blob by BLAKE3 hash.
///
/// Returns the raw bytes with `Content-Type` from the `.meta.json` sidecar
/// (falls back to `application/octet-stream`). Immutable cache headers are
/// set since BLAKE3 hashes are content-derived and never change.
///
/// # Responses
///
/// - `200 OK` — blob found, raw bytes in body
/// - `400 Bad Request` — invalid hash format
/// - `404 Not Found` — hash not in content store
/// - `500 Internal Server Error` — I/O or decryption failure
pub async fn serve_content_by_hash(
    Path(hash): Path<String>,
    Query(query): Query<ContentServeQuery>,
) -> impl IntoResponse {
    let family_id = query
        .family_id
        .or_else(|| std::env::var("NESTGATE_FAMILY_ID").ok())
        .unwrap_or_else(|| String::from("default"));

    debug!(hash = %hash, family_id = %family_id, "content serve request");

    match nestgate_core::rpc::content_ops::get_raw(&hash, &family_id).await {
        Ok(Some(content)) => {
            let mime = content
                .content_type
                .unwrap_or_else(|| String::from("application/octet-stream"));

            let mut headers = HeaderMap::new();

            if let Ok(val) = HeaderValue::from_str(&mime) {
                headers.insert(header::CONTENT_TYPE, val);
            }

            if let Ok(val) = HeaderValue::from_str(&format!("\"{}\"", content.hash)) {
                headers.insert(header::ETAG, val);
            }

            headers.insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=31536000, immutable"),
            );

            if let Ok(len) = HeaderValue::from_str(&content.data.len().to_string()) {
                headers.insert(header::CONTENT_LENGTH, len);
            }

            if let Ok(name) = "x-content-hash".parse::<axum::http::HeaderName>() {
                headers.insert(name, HeaderValue::from_static("blake3"));
            }

            (StatusCode::OK, headers, content.data).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, format!("content not found: {hash}")).into_response(),
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("invalid") || msg.contains("must be a 64-character") {
                (StatusCode::BAD_REQUEST, msg).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, http::Request, routing::get};
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use serde_json::json;
    use serial_test::serial;
    use tower::ServiceExt;

    async fn test_router() -> Router {
        Router::new().route("/content/:hash", get(serve_content_by_hash))
    }

    #[tokio::test]
    #[serial]
    async fn serve_stored_content_returns_raw_bytes() {
        let family = format!("test-serve-{}", uuid::Uuid::new_v4());
        let payload = b"Hello from nestgate.io content serving";
        let b64 = STANDARD.encode(payload);

        let put_result = nestgate_core::rpc::content_ops::put(&json!({
            "data": b64,
            "family_id": family,
            "content_type": "text/plain"
        }))
        .await
        .expect("put should succeed");

        let hash = put_result["hash"].as_str().unwrap();

        let app = test_router().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/content/{hash}?family_id={family}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/plain"
        );
        assert_eq!(
            response.headers().get("cache-control").unwrap(),
            "public, max-age=31536000, immutable"
        );
        assert!(response.headers().get("etag").is_some());

        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();
        assert_eq!(&body[..], payload);
    }

    #[tokio::test]
    #[serial]
    async fn serve_missing_hash_returns_404() {
        let fake_hash = "a".repeat(64);
        let family = format!("test-serve-404-{}", uuid::Uuid::new_v4());

        let app = test_router().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/content/{fake_hash}?family_id={family}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn serve_invalid_hash_returns_400() {
        let app = test_router().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/content/not-a-valid-hash?family_id=test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn serve_content_blake3_integrity() {
        let family = format!("test-serve-b3-{}", uuid::Uuid::new_v4());
        let payload = b"BLAKE3 integrity verification through HTTP content serving";
        let b64 = STANDARD.encode(payload);

        let put_result = nestgate_core::rpc::content_ops::put(&json!({
            "data": b64, "family_id": family
        }))
        .await
        .unwrap();

        let hash = put_result["hash"].as_str().unwrap();

        let app = test_router().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/content/{hash}?family_id={family}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
            .await
            .unwrap();

        let actual_hash = blake3::hash(&body).to_hex().to_string();
        assert_eq!(actual_hash, hash, "served bytes must match BLAKE3 CID");
    }

    #[tokio::test]
    #[serial]
    async fn serve_content_default_mime_when_no_content_type() {
        let family = format!("test-serve-mime-{}", uuid::Uuid::new_v4());
        let payload = b"\x00\x01\x02binary blob with no content type";
        let b64 = STANDARD.encode(payload);

        let put_result = nestgate_core::rpc::content_ops::put(&json!({
            "data": b64, "family_id": family
        }))
        .await
        .unwrap();

        let hash = put_result["hash"].as_str().unwrap();

        let app = test_router().await;
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/content/{hash}?family_id={family}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/octet-stream"
        );
    }
}
