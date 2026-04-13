// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for the GCS backend (`gcs.rs`).

use super::{GcsBackend, GcsStorageClass};
use crate::zero_cost_zfs_operations::ZeroCostZfsOperations;
use nestgate_core::canonical_types::StorageTier;

#[test]
fn tier_mapping_and_storage_class_names() {
    assert_eq!(
        GcsBackend::map_tier(&StorageTier::Hot),
        GcsStorageClass::Standard
    );
    assert_eq!(
        GcsBackend::map_tier(&StorageTier::Warm),
        GcsStorageClass::Nearline
    );
    assert_eq!(
        GcsBackend::map_tier(&StorageTier::Cold),
        GcsStorageClass::Coldline
    );
    assert_eq!(
        GcsBackend::map_tier(&StorageTier::Cache),
        GcsStorageClass::Standard
    );
    assert_eq!(
        GcsBackend::map_tier(&StorageTier::Archive),
        GcsStorageClass::Archive
    );
    assert_eq!(
        GcsBackend::storage_class_name(&GcsStorageClass::Standard),
        "STANDARD"
    );
    assert_eq!(
        GcsBackend::storage_class_name(&GcsStorageClass::Nearline),
        "NEARLINE"
    );
    assert_eq!(
        GcsBackend::storage_class_name(&GcsStorageClass::Coldline),
        "COLDLINE"
    );
    assert_eq!(
        GcsBackend::storage_class_name(&GcsStorageClass::Archive),
        "ARCHIVE"
    );
}

#[test]
fn dataset_prefix_format() {
    assert_eq!(GcsBackend::dataset_prefix("tank", "data"), "tank/data");
}

#[tokio::test]
async fn gcs_backend_from_config_no_external_apis() {
    let backend = GcsBackend::from_discovered_config_for_test(
        "env-test",
        "nestgate-gcs-test",
        None,
        "test-nestgate",
        "US-WEST1",
    )
    .await
    .expect("config-injected backend");
    assert_eq!(backend.bucket_prefix, "test-nestgate");
    assert_eq!(backend.location, "US-WEST1");
}

#[tokio::test]
async fn gcs_backend_accepts_project_id_via_config() {
    let backend = GcsBackend::from_discovered_config_for_test(
        "alias-test",
        "alias-proj",
        None,
        "nestgate",
        "US",
    )
    .await
    .expect("project id via config injection");
    assert_eq!(backend.bucket_prefix, "nestgate");
}

#[tokio::test]
async fn gcs_operations_in_memory_round_trip() {
    let backend = GcsBackend::from_discovered_config_for_test(
        "inmem-test",
        "inmem-proj",
        None,
        "nestgate",
        "US",
    )
    .await
    .expect("backend");

    let pool = backend
        .create_pool("test-pool", &[])
        .await
        .expect("create_pool");
    let bucket = backend.bucket_name("test-pool");
    assert!(bucket.contains("test-pool"));
    assert!(!bucket.contains('_'));

    let dataset = backend
        .create_dataset(&pool, "data", StorageTier::Warm)
        .await
        .expect("dataset");
    assert_eq!(dataset.storage_class, GcsStorageClass::Nearline);

    let snapshot = backend
        .create_snapshot(&dataset, "snap1")
        .await
        .expect("snapshot");
    assert_eq!(snapshot.dataset, "data");

    let props = backend.get_pool_properties(&pool).await.expect("props");
    assert_eq!(props.project_id, "inmem-proj");
    assert!(
        props
            .custom
            .get("config_source")
            .is_some_and(|s| s.contains("capability"))
    );
}

#[tokio::test]
async fn gcs_backend_from_discovered_capability_path() {
    let backend = GcsBackend::from_discovered_config_for_test(
        "svc-1",
        "discovered-project",
        Some("/tmp/creds.json".to_string()),
        "ng-prefix",
        "EU",
    )
    .await
    .expect("discovered backend");

    let pool = backend.create_pool("p1", &[]).await.expect("pool");
    let props = backend
        .get_pool_properties(&pool)
        .await
        .expect("properties");
    assert_eq!(props.project_id, "discovered-project");
    let src = props.custom.get("config_source").expect("config_source");
    assert!(src.contains("capability:svc-1"));
}

#[tokio::test]
async fn gcs_pools_list_and_empty_datasets() {
    let backend = GcsBackend::from_discovered_config_for_test(
        "list-test",
        "list-proj",
        None,
        "nestgate",
        "US",
    )
    .await
    .expect("backend");

    backend.create_pool("pool1", &[]).await.unwrap();
    backend.create_pool("pool2", &[]).await.unwrap();
    let pools = backend.list_pools().await.expect("list pools");
    assert_eq!(pools.len(), 2);

    let p = &pools[0];
    let datasets = backend.list_datasets(p).await.expect("list datasets");
    assert!(datasets.is_empty());
}
