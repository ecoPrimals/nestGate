// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use axum::extract::Path;
use nestgate_zfs::native::{is_zfs_available, is_zpool_available};

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

#[test]
fn infer_tier_from_properties_warm_uncompressed_band() {
    const GIB: u64 = 1024 * 1024 * 1024;
    let props = HashMap::from([
        ("used".to_owned(), format!("{}", GIB + 1)),
        ("compressratio".to_owned(), "1.2x".to_owned()),
    ]);
    assert_eq!(infer_tier_from_properties(&props), "warm");
}

#[test]
fn infer_tier_from_properties_defaults_used_and_ratio() {
    let props = HashMap::new();
    assert_eq!(infer_tier_from_properties(&props), "hot");
}

#[test]
fn infer_tier_from_properties_cold_above_tib() {
    const TIB: u64 = 1024 * 1024 * 1024 * 1024;
    let props = HashMap::from([("used".to_owned(), format!("{}", TIB + 1))]);
    assert_eq!(infer_tier_from_properties(&props), "cold");
}

#[test]
fn infer_tier_from_properties_warm_compress_ratio_at_two_is_not_compressed_band() {
    const GIB: u64 = 1024 * 1024 * 1024;
    let props = HashMap::from([
        ("used".to_owned(), format!("{}", GIB + 1)),
        ("compressratio".to_owned(), "2.0x".to_owned()),
    ]);
    assert_eq!(infer_tier_from_properties(&props), "warm");
}

#[test]
fn infer_tier_from_properties_invalid_used_parses_as_zero_hot() {
    let props = HashMap::from([("used".to_owned(), "not-a-number".to_owned())]);
    assert_eq!(infer_tier_from_properties(&props), "hot");
}

#[test]
fn infer_tier_from_properties_compressratio_without_suffix_uses_default_ratio() {
    const GIB: u64 = 1024 * 1024 * 1024;
    let props = HashMap::from([
        ("used".to_owned(), format!("{}", GIB + 1)),
        ("compressratio".to_owned(), "3.0".to_owned()),
    ]);
    assert_eq!(infer_tier_from_properties(&props), "warm");
}

#[test]
fn zfs_handler_impl_default_matches_new() {
    assert_eq!(
        format!("{:?}", ZfsHandlerImpl::default()),
        format!("{:?}", ZfsHandlerImpl::new())
    );
}

#[tokio::test]
async fn create_pool_rejects_missing_name() {
    let mut body = HashMap::new();
    body.insert("devices".to_owned(), json!(["/dev/null"]));
    let (code, Json(v)) = create_pool(Json(body)).await;
    assert_eq!(code, StatusCode::BAD_REQUEST);
    assert_eq!(v["error"], "bad_request");
}

#[tokio::test]
async fn create_pool_rejects_empty_name_string() {
    let mut body = HashMap::new();
    body.insert("name".to_owned(), json!(""));
    body.insert("devices".to_owned(), json!(["/dev/null"]));
    let (code, Json(v)) = create_pool(Json(body)).await;
    assert_eq!(code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_pool_rejects_missing_devices_array() {
    let mut body = HashMap::new();
    body.insert("name".to_owned(), json!("p"));
    body.insert("devices".to_owned(), json!("not-an-array"));
    let (code, Json(v)) = create_pool(Json(body)).await;
    assert_eq!(code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_dataset_accepts_properties_object() {
    let mut body = HashMap::new();
    body.insert("name".to_owned(), json!("tank/fs1"));
    body.insert(
        "properties".to_owned(),
        json!({"atime": "off", "compression": "lz4"}),
    );
    let (code, Json(v)) = create_dataset(Json(body)).await;
    if is_zfs_available().await {
        if code == StatusCode::CREATED {
            assert_eq!(v["status"], "success");
        } else {
            assert_eq!(v["error"], "zfs_operation_failed");
        }
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn create_snapshot_validates_empty_dataset() {
    let mut body = HashMap::new();
    body.insert("dataset".to_owned(), json!(""));
    body.insert("name".to_owned(), json!("snap1"));
    let (code, Json(v)) = create_snapshot(Json(body)).await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::BAD_REQUEST);
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn create_snapshot_validates_empty_snapshot_name() {
    let mut body = HashMap::new();
    body.insert("dataset".to_owned(), json!("tank/fs"));
    body.insert("name".to_owned(), json!(""));
    let (code, Json(v)) = create_snapshot(Json(body)).await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::BAD_REQUEST);
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn get_pool_status_matches_get_universal_pool() {
    let name = "any_pool".to_string();
    let a = get_universal_pool(Path(name.clone())).await;
    let b = get_pool_status(Path(name)).await;
    assert_eq!(a.0, b.0);
    assert_eq!(a.1.0, b.1.0);
}

#[tokio::test]
async fn list_snapshots_delegates_or_unavailable() {
    let (code, Json(v)) = list_snapshots().await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::OK);
        assert_eq!(v["status"], "success");
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn get_universal_storage_health_delegates_or_unavailable() {
    let (code, Json(v)) = get_universal_storage_health().await;
    if is_zpool_available().await {
        assert_eq!(code, StatusCode::OK);
        assert_eq!(v["status"], "success");
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn get_performance_analytics_delegates_or_unavailable() {
    let (code, Json(v)) = get_performance_analytics().await;
    if is_zpool_available().await {
        assert_ne!(code, StatusCode::SERVICE_UNAVAILABLE);
        if code == StatusCode::OK {
            assert_eq!(v["status"], "success");
        } else {
            assert_eq!(v["error"], "zfs_operation_failed");
        }
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn get_zfs_health_reports_version_or_unavailable() {
    let (code, Json(v)) = get_zfs_health().await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::OK);
        assert_eq!(v["status"], "success");
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn delete_pool_trigger_and_snapshot_follow_zfs_availability() {
    let (c1, _) = delete_pool(Path("no_such_pool".into())).await;
    let (c2, _) = trigger_optimization(Path("no_such_pool".into())).await;
    let (c3, _) = delete_snapshot(Path("tank/fs@snap".into())).await;
    if is_zpool_available().await {
        assert_ne!(c1, StatusCode::SERVICE_UNAVAILABLE);
        assert_ne!(c2, StatusCode::SERVICE_UNAVAILABLE);
    } else {
        assert_eq!(c1, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(c2, StatusCode::SERVICE_UNAVAILABLE);
    }
    if is_zfs_available().await {
        assert_ne!(c3, StatusCode::SERVICE_UNAVAILABLE);
    } else {
        assert_eq!(c3, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn get_dataset_returns_not_found_when_missing() {
    let (code, Json(v)) = get_dataset(Path("no_such_dataset_zzz".into())).await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::NOT_FOUND);
        assert_eq!(v["error"], "dataset_not_found");
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn list_datasets_delegates_or_unavailable() {
    let (code, Json(v)) = list_datasets().await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::OK);
        assert_eq!(v["status"], "success");
        assert!(v.get("datasets").is_some());
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(v["error"], "zfs_unavailable");
    }
}

#[tokio::test]
async fn delete_dataset_reports_operation_failed_when_missing() {
    let (code, Json(v)) = delete_dataset(Path("no_such_dataset_delete_zzz".into())).await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(v["error"], "zfs_operation_failed");
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn get_dataset_properties_reports_failure_for_missing_dataset() {
    let (code, Json(v)) = get_dataset_properties(Path("no_such_props_zzz".into())).await;
    if is_zfs_available().await {
        assert_eq!(code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(v["error"], "zfs_operation_failed");
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn set_dataset_properties_accepts_non_empty_body_shape() {
    let mut body = HashMap::new();
    body.insert("atime".to_owned(), json!("off"));
    body.insert("compression".to_owned(), json!("lz4"));
    let (code, Json(v)) =
        set_dataset_properties(Path("no_such_for_props_zzz".into()), Json(body)).await;
    if is_zfs_available().await {
        assert_ne!(code, StatusCode::SERVICE_UNAVAILABLE);
        assert!(matches!(
            code,
            StatusCode::OK | StatusCode::MULTI_STATUS | StatusCode::INTERNAL_SERVER_ERROR
        ));
        if code == StatusCode::OK {
            assert_eq!(v["status"], "success");
            assert!(v.get("properties_set").is_some());
        } else if code == StatusCode::MULTI_STATUS {
            assert_eq!(v["status"], "partial");
            assert!(v.get("errors").is_some());
        } else {
            assert_eq!(v["error"], "zfs_operation_failed");
        }
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn create_dataset_name_only_body_shape() {
    let mut body = HashMap::new();
    body.insert(
        "name".to_owned(),
        json!("tank/nestgate_test_missing_dataset"),
    );
    let (code, Json(v)) = create_dataset(Json(body)).await;
    if is_zfs_available().await {
        assert_ne!(code, StatusCode::SERVICE_UNAVAILABLE);
        if code == StatusCode::CREATED {
            assert_eq!(v["status"], "success");
            assert_eq!(v["dataset"], "tank/nestgate_test_missing_dataset");
        } else {
            assert_eq!(v["error"], "zfs_operation_failed");
        }
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn predict_tier_valid_dataset_field_exercises_handler() {
    let mut body = HashMap::new();
    body.insert("dataset".to_owned(), json!("tank"));
    let (code, Json(v)) = predict_tier(Json(body)).await;
    if is_zfs_available().await {
        assert_ne!(code, StatusCode::SERVICE_UNAVAILABLE);
        if code == StatusCode::OK {
            assert_eq!(v["status"], "success");
            assert_eq!(v["dataset"], "tank");
            assert!(v.get("predicted_tier").is_some());
            assert!(v.get("properties").is_some());
        } else {
            assert_eq!(v["error"], "zfs_operation_failed");
        }
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn get_universal_storage_health_json_includes_pool_counts() {
    let (code, Json(v)) = get_universal_storage_health().await;
    if is_zpool_available().await {
        assert_eq!(code, StatusCode::OK);
        assert_eq!(v["status"], "success");
        assert!(v.get("pool_count").is_some());
        assert!(v.get("pools_unhealthy").is_some());
        assert!(v.get("pools").is_some());
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn create_snapshot_valid_pair_calls_handler() {
    let mut body = HashMap::new();
    body.insert("dataset".to_owned(), json!("tank/nestgate_snap_test_ds"));
    body.insert("name".to_owned(), json!("snap1"));
    let (code, Json(v)) = create_snapshot(Json(body)).await;
    if is_zfs_available().await {
        assert_ne!(code, StatusCode::SERVICE_UNAVAILABLE);
        if code == StatusCode::CREATED {
            assert_eq!(v["status"], "success");
            assert!(v.get("snapshot").is_some());
        } else {
            assert_eq!(v["error"], "zfs_operation_failed");
        }
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[tokio::test]
async fn create_pool_valid_json_shape_when_zpool_present() {
    let mut body = HashMap::new();
    body.insert("name".to_owned(), json!("nestgate_test_pool_should_fail"));
    body.insert("devices".to_owned(), json!(["/dev/nonexistent_device_zzz"]));
    let (code, Json(v)) = create_pool(Json(body)).await;
    if is_zpool_available().await {
        assert_ne!(code, StatusCode::SERVICE_UNAVAILABLE);
        assert!(matches!(
            code,
            StatusCode::CREATED | StatusCode::INTERNAL_SERVER_ERROR
        ));
        if code == StatusCode::CREATED {
            assert_eq!(v["status"], "success");
            assert!(v.get("pool").is_some());
        } else {
            assert_eq!(v["error"], "zfs_operation_failed");
        }
    } else {
        assert_eq!(code, StatusCode::SERVICE_UNAVAILABLE);
    }
}

#[test]
fn zero_cost_zfs_operations_implements_default() {
    assert_eq!(
        format!("{:?}", ZeroCostZfsOperations::default()),
        format!("{:?}", ZeroCostZfsOperations::new())
    );
}
