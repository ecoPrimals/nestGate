// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use axum::extract::Path;
use nestgate_zfs::native::is_zpool_available;

#[tokio::test]
async fn list_universal_pools_delegates_or_reports_unavailable() {
    let (code, Json(v)) = list_universal_pools().await;
    if is_zpool_available().await {
        assert_eq!(code, StatusCode::OK);
        assert_eq!(v["status"], "success");
        assert!(v.get("pools").is_some());
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(v["error"], "zfs_unavailable");
    }
}

#[tokio::test]
async fn create_pool_validates_body() {
    let (code, Json(v)) = create_pool(Json(HashMap::new())).await;
    assert_eq!(code, StatusCode::BAD_REQUEST);
    assert_eq!(v["error"], "bad_request");
}

#[tokio::test]
async fn create_pool_rejects_empty_devices() {
    let mut body = HashMap::new();
    body.insert("name".to_owned(), json!("testpool"));
    body.insert("devices".to_owned(), json!([]));
    let (code, Json(v)) = create_pool(Json(body)).await;
    assert_eq!(code, StatusCode::BAD_REQUEST);
    assert_eq!(v["error"], "bad_request");
}

#[tokio::test]
async fn get_universal_pool_delegates_or_unavailable() {
    let (code, Json(v)) = get_universal_pool(Path("nonexistent_pool_xyz".to_string())).await;
    if is_zpool_available().await {
        assert_ne!(code, StatusCode::SERVICE_UNAVAILABLE);
        if code == StatusCode::OK {
            assert_eq!(v["status"], "success");
        } else {
            assert_eq!(v["error"], "zfs_operation_failed");
        }
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(v["error"], "zfs_unavailable");
    }
}

#[tokio::test]
async fn create_dataset_validates_body() {
    let (code, Json(v)) = create_dataset(Json(HashMap::new())).await;
    assert_eq!(code, StatusCode::BAD_REQUEST);
    assert_eq!(v["error"], "bad_request");
}

#[tokio::test]
async fn create_snapshot_validates_body() {
    let (code, Json(v)) = create_snapshot(Json(HashMap::new())).await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::BAD_REQUEST);
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn predict_tier_validates_body() {
    let (code, Json(v)) = predict_tier(Json(HashMap::new())).await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::BAD_REQUEST);
        assert_eq!(v["error"], "bad_request");
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn set_dataset_properties_rejects_empty_body() {
    let (code, Json(v)) =
        set_dataset_properties(Path("test".to_owned()), Json(HashMap::new())).await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::BAD_REQUEST);
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(v["error"], "zfs_unavailable");
    }
}

#[tokio::test]
async fn placeholder_types_construct() {
    let _ = ZfsConfig;
    let _ = ProductionZfsManager::new(ZfsConfig);
    let _ = ZeroCostZfsOperations::new();
    let _ = ZfsHandlerImpl;
}

#[test]
fn infer_tier_from_properties_hot() {
    let props = HashMap::from([("used".to_owned(), "500000000".to_owned())]);
    assert_eq!(infer_tier_from_properties(&props), "hot");
}

#[test]
fn infer_tier_from_properties_cold() {
    let props = HashMap::from([("used".to_owned(), "2000000000000".to_owned())]);
    assert_eq!(infer_tier_from_properties(&props), "cold");
}

#[test]
fn infer_tier_from_properties_warm_compressed() {
    let props = HashMap::from([
        ("used".to_owned(), "5000000000".to_owned()),
        ("compressratio".to_owned(), "3.5x".to_owned()),
    ]);
    assert_eq!(infer_tier_from_properties(&props), "warm-compressed");
}
