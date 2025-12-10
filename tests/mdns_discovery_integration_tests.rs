//! Integration tests for mDNS discovery backend
//!
//! Tests the complete mDNS discovery flow with self-knowledge pattern.

use nestgate_core::universal_primal_discovery::{
    backends::mdns::{MdnsConfig, MdnsDiscoveryBackend},
    capability_based_discovery::{
        BindingInfo, DiscoveryBackend, DiscoveryQuery, HealthStatus, PeerDescriptor,
        PrimalCapability, PrimalId, PrimalSelfKnowledge, Protocol,
    },
};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

#[tokio::test]
async fn test_mdns_backend_creation() {
    // Test basic creation
    let _backend = MdnsDiscoveryBackend::new();

    // If we got here without panic, test passes
}

#[tokio::test]
async fn test_mdns_backend_with_custom_config() {
    // Test creation with custom configuration
    let config = MdnsConfig {
        service_type: "_test._tcp".to_string(),
        domain: "local".to_string(),
        query_timeout: Duration::from_secs(2),
        cache_ttl: Duration::from_secs(60),
    };

    let _backend = MdnsDiscoveryBackend::with_config(config);
}

#[tokio::test]
async fn test_mdns_announce_self_knowledge() {
    // Test announcing our self-knowledge
    let backend = MdnsDiscoveryBackend::new();

    let self_knowledge = PrimalSelfKnowledge {
        id: PrimalId::new("test-primal"),
        capabilities: vec![
            PrimalCapability::new("storage"),
            PrimalCapability::new("zfs_management"),
        ],
        binding: BindingInfo {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            protocol: Protocol::Http,
        },
        metadata: vec![
            ("version".to_string(), "1.0.0".to_string()),
            ("region".to_string(), "local".to_string()),
        ]
        .into_iter()
        .collect(),
        health: HealthStatus::Healthy,
    };

    let result = backend.announce(&self_knowledge).await;
    assert!(
        result.is_ok(),
        "Announce should succeed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_mdns_query_capabilities() {
    // Test querying for capabilities
    let backend = MdnsDiscoveryBackend::new();

    // First announce ourselves
    let self_knowledge = PrimalSelfKnowledge {
        id: PrimalId::new("storage-primal"),
        capabilities: vec![PrimalCapability::new("storage")],
        binding: BindingInfo {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8080,
            protocol: Protocol::Http,
        },
        metadata: std::collections::HashMap::new(),
        health: HealthStatus::Healthy,
    };

    backend
        .announce(&self_knowledge)
        .await
        .expect("Announce should succeed");

    // Now query for storage capability
    let query = DiscoveryQuery {
        required_capabilities: vec![PrimalCapability::new("storage")],
        optional_capabilities: vec![],
        max_results: Some(10),
        timeout: Some(Duration::from_secs(1)),
    };

    let result = backend.query(&query).await;
    assert!(result.is_ok(), "Query should succeed");

    let peers = result.unwrap();
    assert!(peers.len() > 0, "Should find at least ourselves");

    // Verify we found ourselves
    let found_self = peers.iter().any(|p| p.id.as_str() == "storage-primal");
    assert!(found_self, "Should discover ourselves via mDNS cache");
}

#[tokio::test]
async fn test_mdns_query_multiple_capabilities() {
    // Test querying for multiple capabilities
    let backend = MdnsDiscoveryBackend::new();

    // Announce a primal with multiple capabilities
    let self_knowledge = PrimalSelfKnowledge {
        id: PrimalId::new("multi-cap-primal"),
        capabilities: vec![
            PrimalCapability::new("storage"),
            PrimalCapability::new("security"),
        ],
        binding: BindingInfo {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 9090,
            protocol: Protocol::Http,
        },
        metadata: std::collections::HashMap::new(),
        health: HealthStatus::Healthy,
    };

    backend
        .announce(&self_knowledge)
        .await
        .expect("Announce should succeed");

    // Query requiring BOTH capabilities
    let query = DiscoveryQuery {
        required_capabilities: vec![
            PrimalCapability::new("storage"),
            PrimalCapability::new("security"),
        ],
        optional_capabilities: vec![],
        max_results: Some(10),
        timeout: Some(Duration::from_secs(1)),
    };

    let result = backend.query(&query).await;
    assert!(result.is_ok());

    let peers = result.unwrap();

    // Should find our multi-capability primal
    let found = peers.iter().any(|p| {
        p.id.as_str() == "multi-cap-primal"
            && p.capabilities.contains(&PrimalCapability::new("storage"))
            && p.capabilities.contains(&PrimalCapability::new("security"))
    });

    assert!(found, "Should find primal with both capabilities");
}

#[tokio::test]
async fn test_mdns_unannounce() {
    // Test unannouncing
    let backend = MdnsDiscoveryBackend::new();

    let primal_id = PrimalId::new("temporary-primal");

    // Announce
    let self_knowledge = PrimalSelfKnowledge {
        id: primal_id.clone(),
        capabilities: vec![PrimalCapability::new("test")],
        binding: BindingInfo {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 7070,
            protocol: Protocol::Http,
        },
        metadata: std::collections::HashMap::new(),
        health: HealthStatus::Healthy,
    };

    backend
        .announce(&self_knowledge)
        .await
        .expect("Announce should succeed");

    // Unannounce
    let result = backend.unannounce(&primal_id).await;
    assert!(result.is_ok(), "Unannounce should succeed");

    // Query to verify it's gone
    let query = DiscoveryQuery {
        required_capabilities: vec![PrimalCapability::new("test")],
        optional_capabilities: vec![],
        max_results: Some(10),
        timeout: Some(Duration::from_secs(1)),
    };

    let peers = backend.query(&query).await.expect("Query should succeed");
    let still_there = peers.iter().any(|p| p.id.as_str() == "temporary-primal");

    assert!(!still_there, "Primal should be gone after unannounce");
}

#[tokio::test]
async fn test_mdns_zero_hardcoding_principle() {
    // This test verifies the "zero hardcoding" principle
    let backend = MdnsDiscoveryBackend::new();

    // We announce ONLY our self-knowledge
    // No hardcoded peer names, no hardcoded addresses
    let self_knowledge = PrimalSelfKnowledge {
        id: PrimalId::new("autonomous-primal"),
        capabilities: vec![PrimalCapability::new("autonomous")],
        binding: BindingInfo {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 6060,
            protocol: Protocol::Http,
        },
        metadata: [("principle".to_string(), "self-knowledge-only".to_string())]
            .into_iter()
            .collect(),
        health: HealthStatus::Healthy,
    };

    backend
        .announce(&self_knowledge)
        .await
        .expect("Announce should succeed");

    // We discover by CAPABILITY, not by name
    let query = DiscoveryQuery {
        required_capabilities: vec![PrimalCapability::new("autonomous")],
        optional_capabilities: vec![],
        max_results: None,
        timeout: None,
    };

    let peers = backend.query(&query).await.expect("Query should succeed");

    // The system works WITHOUT any hardcoded names
    assert!(peers.len() > 0, "Discovery works without hardcoding");

    // Verify we found by capability, not by predetermined name
    let found = peers.iter().any(|p| {
        p.capabilities
            .contains(&PrimalCapability::new("autonomous"))
    });

    assert!(found, "Capability-based discovery successful");
}

#[tokio::test]
async fn test_mdns_self_discovery() {
    // Test that a primal can discover itself (useful for verification)
    let backend = MdnsDiscoveryBackend::new();

    let my_id = PrimalId::new("self-aware-primal");
    let my_capability = PrimalCapability::new("self-awareness");

    let self_knowledge = PrimalSelfKnowledge {
        id: my_id.clone(),
        capabilities: vec![my_capability.clone()],
        binding: BindingInfo {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 5050,
            protocol: Protocol::Http,
        },
        metadata: std::collections::HashMap::new(),
        health: HealthStatus::Healthy,
    };

    backend
        .announce(&self_knowledge)
        .await
        .expect("Announce should succeed");

    // Discover ourselves
    let query = DiscoveryQuery {
        required_capabilities: vec![my_capability],
        optional_capabilities: vec![],
        max_results: Some(1),
        timeout: Some(Duration::from_secs(1)),
    };

    let peers = backend.query(&query).await.expect("Query should succeed");

    // Should find ourselves
    assert!(peers.len() > 0, "Should discover self");
    assert_eq!(
        peers[0].id.as_str(),
        "self-aware-primal",
        "Should be ourselves"
    );
}

#[tokio::test]
async fn test_mdns_capability_filtering() {
    // Test that discovery correctly filters by capabilities
    let backend = MdnsDiscoveryBackend::new();

    // Announce storage primal
    let storage = PrimalSelfKnowledge {
        id: PrimalId::new("storage-only"),
        capabilities: vec![PrimalCapability::new("storage")],
        binding: BindingInfo {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8001,
            protocol: Protocol::Http,
        },
        metadata: std::collections::HashMap::new(),
        health: HealthStatus::Healthy,
    };

    // Announce security primal
    let security = PrimalSelfKnowledge {
        id: PrimalId::new("security-only"),
        capabilities: vec![PrimalCapability::new("security")],
        binding: BindingInfo {
            address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 8002,
            protocol: Protocol::Http,
        },
        metadata: std::collections::HashMap::new(),
        health: HealthStatus::Healthy,
    };

    backend
        .announce(&storage)
        .await
        .expect("Storage announce should succeed");
    backend
        .announce(&security)
        .await
        .expect("Security announce should succeed");

    // Query for storage only
    let storage_query = DiscoveryQuery {
        required_capabilities: vec![PrimalCapability::new("storage")],
        optional_capabilities: vec![],
        max_results: None,
        timeout: Some(Duration::from_secs(1)),
    };

    let storage_peers = backend
        .query(&storage_query)
        .await
        .expect("Query should succeed");

    // Should find only storage primal
    assert_eq!(
        storage_peers.len(),
        1,
        "Should find exactly one storage primal"
    );
    assert_eq!(storage_peers[0].id.as_str(), "storage-only");

    // Query for security only
    let security_query = DiscoveryQuery {
        required_capabilities: vec![PrimalCapability::new("security")],
        optional_capabilities: vec![],
        max_results: None,
        timeout: Some(Duration::from_secs(1)),
    };

    let security_peers = backend
        .query(&security_query)
        .await
        .expect("Query should succeed");

    // Should find only security primal
    assert_eq!(
        security_peers.len(),
        1,
        "Should find exactly one security primal"
    );
    assert_eq!(security_peers[0].id.as_str(), "security-only");
}

#[tokio::test]
async fn test_mdns_empty_query() {
    // Test querying with no required capabilities
    let backend = MdnsDiscoveryBackend::new();

    let query = DiscoveryQuery {
        required_capabilities: vec![],
        optional_capabilities: vec![],
        max_results: None,
        timeout: Some(Duration::from_secs(1)),
    };

    let result = backend.query(&query).await;
    assert!(result.is_ok(), "Empty query should succeed");
}

#[tokio::test]
async fn test_mdns_concurrent_announcements() {
    // Test multiple concurrent announcements
    use std::sync::Arc;

    let backend = Arc::new(MdnsDiscoveryBackend::new());

    let mut handles = vec![];

    for i in 0..5 {
        let backend_clone = Arc::clone(&backend);

        let handle = tokio::spawn(async move {
            let self_knowledge = PrimalSelfKnowledge {
                id: PrimalId::from_string(format!("concurrent-{}", i)),
                capabilities: vec![PrimalCapability::Custom("test".to_string())],
                binding: BindingInfo {
                    address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    port: 9000 + i as u16,
                    protocol: Protocol::Http,
                },
                metadata: std::collections::HashMap::new(),
                health: HealthStatus::Healthy,
            };

            backend_clone.announce(&self_knowledge).await
        });

        handles.push(handle);
    }

    // Wait for all announcements
    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert!(result.is_ok(), "Concurrent announcement should succeed");
    }
}
