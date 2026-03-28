// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unit tests for discovery mechanism

use super::*;
use std::time::Duration;

#[tokio::test]
async fn test_mdns_discovery_creation() {
    let discovery = DiscoveryBuilder::new()
        .with_timeout(Duration::from_secs(10))
        .build_mdns()
        .await;

    assert!(discovery.is_ok());
    let discovery = discovery.unwrap();
    assert_eq!(discovery.mechanism_name(), "mdns");
}

#[tokio::test]
async fn test_auto_detect_defaults_to_mdns() {
    // When no discovery env vars set, should default to mDNS
    let discovery = DiscoveryBuilder::new().detect().await;

    assert!(discovery.is_ok());
    let discovery = discovery.unwrap();
    assert_eq!(discovery.mechanism_name(), "mdns");
}

#[tokio::test]
async fn test_mdns_announce_and_find() {
    let discovery = DiscoveryBuilder::new().build_mdns().await.unwrap();

    // Create and announce a service
    let self_knowledge = SelfKnowledge::builder()
        .with_id("test-storage")
        .with_name("Test Storage")
        .with_version("1.0.0")
        .with_capability("storage")
        .with_capability("zfs")
        .with_endpoint("api", "0.0.0.0:8080".parse().unwrap())
        .build()
        .unwrap();

    discovery.announce(&self_knowledge).await.unwrap();

    // Should be able to find by capability
    let storage_services = discovery
        .find_by_capability("storage".to_string())
        .await
        .unwrap();

    assert_eq!(storage_services.len(), 1);
    assert_eq!(storage_services[0].id, "test-storage");
    assert_eq!(storage_services[0].name, "Test Storage");
    assert!(storage_services[0]
        .capabilities
        .contains(&"storage".to_string()));
}
