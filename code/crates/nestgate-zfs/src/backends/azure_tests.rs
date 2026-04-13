// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for the Azure Blob backend (`azure.rs`).

use super::{AzureAccessTier, AzureBackend};
use crate::zero_cost_zfs_operations::ZeroCostZfsOperations;
use nestgate_core::canonical_types::StorageTier;

#[tokio::test]
async fn test_azure_backend_creation() {
    let backend = AzureBackend::test_with_environment_config("teststorage", "test-nestgate");

    assert_eq!(backend.container_prefix, "test-nestgate");
    assert_eq!(backend.container_name("mypool"), "test-nestgate-mypool");
}

#[tokio::test]
#[ignore = "Requires Azure storage account configuration"]
async fn test_container_name_generation() {
    let backend = AzureBackend::new().unwrap();
    let container = backend.container_name("MyPool_Test");

    // Azure containers must be lowercase and no underscores
    assert!(container.chars().all(|c| c.is_lowercase() || c == '-'));
    assert!(!container.contains('_'));
}

#[tokio::test]
async fn test_tier_mapping() {
    assert!(matches!(
        AzureBackend::map_tier(&StorageTier::Hot),
        AzureAccessTier::Premium
    ));
    assert!(matches!(
        AzureBackend::map_tier(&StorageTier::Warm),
        AzureAccessTier::Cool
    ));
    assert!(matches!(
        AzureBackend::map_tier(&StorageTier::Cold),
        AzureAccessTier::Archive
    ));
    assert!(matches!(
        AzureBackend::map_tier(&StorageTier::Cache),
        AzureAccessTier::Premium
    ));
    assert!(matches!(
        AzureBackend::map_tier(&StorageTier::Archive),
        AzureAccessTier::Archive
    ));
}

#[tokio::test]
#[ignore = "Requires Azure storage account configuration"]
async fn test_create_pool() {
    let backend = AzureBackend::new().unwrap();
    let pool = backend.create_pool("test-pool", &[]).await;

    assert!(pool.is_ok(), "Pool creation should succeed");
    let pool = pool.unwrap();
    assert_eq!(pool.name, "test-pool");
    assert!(pool.container.contains("test-pool"));
}

#[tokio::test]
async fn test_create_dataset() {
    let backend = AzureBackend::test_with_environment_config("teststorage", "nestgate");
    let pool = backend.create_pool("test-pool", &[]).await.unwrap();
    let dataset = backend
        .create_dataset(&pool, "data", StorageTier::Warm)
        .await;

    assert!(dataset.is_ok(), "Dataset creation should succeed");
    let dataset = dataset.unwrap();
    assert_eq!(dataset.name, "data");
    assert_eq!(dataset.pool, "test-pool");
    assert!(matches!(dataset.tier, StorageTier::Warm));
    assert!(matches!(dataset.azure_tier, AzureAccessTier::Cool));
}

#[tokio::test]
#[ignore = "Requires Azure storage account configuration"]
async fn test_create_snapshot() {
    let backend = AzureBackend::new().unwrap();
    let pool = backend.create_pool("test-pool", &[]).await.unwrap();
    let dataset = backend
        .create_dataset(&pool, "data", StorageTier::Hot)
        .await
        .unwrap();

    let snapshot = backend.create_snapshot(&dataset, "snap1").await;

    assert!(snapshot.is_ok(), "Snapshot creation should succeed");
    let snapshot = snapshot.unwrap();
    assert_eq!(snapshot.name, "snap1");
    assert_eq!(snapshot.dataset, "data");
}

#[tokio::test]
#[ignore = "Requires Azure storage account configuration"]
async fn test_list_pools() {
    let backend = AzureBackend::new().unwrap();
    backend.create_pool("pool1", &[]).await.unwrap();
    backend.create_pool("pool2", &[]).await.unwrap();

    let pools = backend.list_pools().await.unwrap();
    assert_eq!(pools.len(), 2);
}

#[tokio::test]
async fn test_get_pool_properties() {
    let backend = AzureBackend::test_with_environment_config("teststorage", "nestgate");
    let pool = backend.create_pool("test-pool", &[]).await.unwrap();

    let props = backend.get_pool_properties(&pool).await;

    assert!(props.is_ok(), "Should get pool properties");
    let props = props.unwrap();
    assert!(!props.account.is_empty());
    assert!(props.encryption);
}

#[tokio::test]
async fn test_all_storage_tiers() {
    let backend = AzureBackend::test_with_environment_config("teststorage", "nestgate");
    let pool = backend.create_pool("test-pool", &[]).await.unwrap();
    for tier in [
        StorageTier::Hot,
        StorageTier::Warm,
        StorageTier::Cold,
        StorageTier::Cache,
        StorageTier::Archive,
    ] {
        let dataset = backend
            .create_dataset(&pool, &format!("data-{tier:?}"), tier.clone())
            .await
            .unwrap();

        // Verify Azure tier mapping
        match tier {
            StorageTier::Hot | StorageTier::Cache => {
                assert!(matches!(dataset.azure_tier, AzureAccessTier::Premium));
            }
            StorageTier::Warm => {
                assert!(matches!(dataset.azure_tier, AzureAccessTier::Cool));
            }
            StorageTier::Cold | StorageTier::Archive => {
                assert!(matches!(dataset.azure_tier, AzureAccessTier::Archive));
            }
        }
    }
}
