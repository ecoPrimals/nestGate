// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;

async fn create_test_service() -> Result<NestGateRpcService> {
    NestGateRpcService::new()
}

#[tokio::test]
async fn test_service_creation() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");
    let datasets = service.backend.list_datasets().await.unwrap();
    assert!(
        datasets.is_empty(),
        "New service should start with empty storage"
    );
}

#[tokio::test]
async fn test_health() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");
    let health = service.health(Context::current()).await;
    assert_eq!(health.status, "healthy");
    assert_eq!(health.version, env!("CARGO_PKG_VERSION"));
}

#[tokio::test]
async fn test_version() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");
    let version = service.version(Context::current()).await;
    assert_eq!(version.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(version.api_version, "1.0");
}

#[tokio::test]
async fn test_protocols() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");
    let protocols = service.protocols(Context::current()).await;
    assert_eq!(protocols.len(), 3);
    assert_eq!(protocols[0].protocol, "tarpc");
    assert_eq!(protocols[0].priority, 1);
    assert!(protocols[0].enabled);
}

#[tokio::test]
async fn test_create_dataset() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");
    let result = service
        .create_dataset(
            Context::current(),
            Arc::from("test-dataset"),
            DatasetParams::default(),
        )
        .await;

    assert!(result.is_ok());
    let dataset = result.unwrap();
    assert_eq!(dataset.name, "test-dataset");
    assert_eq!(dataset.object_count, 0);
}

#[tokio::test]
async fn test_list_datasets() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");

    service
        .clone()
        .create_dataset(
            Context::current(),
            Arc::from("test-dataset"),
            DatasetParams::default(),
        )
        .await
        .unwrap();

    let datasets = service
        .clone()
        .list_datasets(Context::current())
        .await
        .unwrap();
    assert_eq!(datasets.len(), 1);
    assert_eq!(datasets[0].name, "test-dataset");
}

#[tokio::test]
async fn test_store_retrieve_object() {
    let service = create_test_service()
        .await
        .expect("Failed to create service");

    service
        .clone()
        .create_dataset(
            Context::current(),
            Arc::from("test-dataset"),
            DatasetParams::default(),
        )
        .await
        .unwrap();

    let data = vec![1, 2, 3, 4, 5];
    service
        .clone()
        .store_object(
            Context::current(),
            Arc::from("test-dataset"),
            Arc::from("test-key"),
            Bytes::from(data.clone()),
            None,
        )
        .await
        .unwrap();

    let retrieved = service
        .clone()
        .retrieve_object(
            Context::current(),
            Arc::from("test-dataset"),
            Arc::from("test-key"),
        )
        .await
        .unwrap();

    assert_eq!(retrieved.as_ref(), data.as_slice());
}
