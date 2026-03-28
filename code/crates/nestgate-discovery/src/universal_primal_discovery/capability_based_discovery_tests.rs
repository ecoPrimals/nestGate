// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **Comprehensive Tests for Capability-Based Discovery**
//!
//! Sprint 1: Core discovery system coverage
//! Target: 85% coverage of capability_based_discovery.rs

use super::capability_based_discovery::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, SystemTime};

// ============================================================================
// PRIMAL ID TESTS
// ============================================================================

#[cfg(test)]
mod primal_id_tests {
    use super::*;

    #[test]
    fn test_primal_id_from_environment_creates_unique_id() {
        // ✅ MODERN: Test uniqueness without sleep
        // IDs should be unique due to timestamps and/or random components
        let id1 = PrimalId::from_environment().expect("Should create ID");
        let id2 = PrimalId::from_environment().expect("Should create ID");

        // ✅ CONCURRENT: Time progresses naturally during execution
        // IDs are unique due to timestamp precision and implementation details
        assert_ne!(id1.as_str(), id2.as_str());
    }

    #[test]
    fn test_primal_id_as_str_returns_string() {
        let id = PrimalId::from_environment().expect("Should create ID");
        let s = id.as_str();

        assert!(!s.is_empty());
        assert!(s.contains('-')); // Format: hostname-pid-timestamp
    }

    #[test]
    fn test_primal_id_contains_process_info() {
        let id = PrimalId::from_environment().expect("Should create ID");
        let s = id.as_str();

        // Should contain PID
        let pid = std::process::id();
        assert!(s.contains(&pid.to_string()));
    }

    #[test]
    fn test_primal_id_hash_equality() {
        use std::collections::HashSet;

        let id1 = PrimalId::from_environment().expect("Should create ID");
        let id2 = PrimalId::from_environment().expect("Should create ID");

        let mut set = HashSet::new();
        set.insert(id1.clone());
        set.insert(id2.clone());

        assert!(set.contains(&id1));
        assert!(set.contains(&id2));
    }
}

// ============================================================================
// PRIMAL CAPABILITY TESTS
// ============================================================================

#[cfg(test)]
mod capability_tests {
    use super::*;

    #[test]
    fn test_capability_zfs_storage_creation() {
        let cap = PrimalCapability::ZfsStorage;
        assert!(matches!(cap, PrimalCapability::ZfsStorage));
    }

    #[test]
    fn test_capability_api_gateway_creation() {
        let cap = PrimalCapability::ApiGateway;
        assert!(matches!(cap, PrimalCapability::ApiGateway));
    }

    #[test]
    fn test_capability_service_discovery_creation() {
        let cap = PrimalCapability::ServiceDiscovery;
        assert!(matches!(cap, PrimalCapability::ServiceDiscovery));
    }

    #[test]
    fn test_capability_nfs_v3_creation() {
        let cap = PrimalCapability::NetworkFileSystem(NfsVersion::V3);
        assert!(matches!(
            cap,
            PrimalCapability::NetworkFileSystem(NfsVersion::V3)
        ));
    }

    #[test]
    fn test_capability_nfs_v4_creation() {
        let cap = PrimalCapability::NetworkFileSystem(NfsVersion::V4);
        assert!(matches!(
            cap,
            PrimalCapability::NetworkFileSystem(NfsVersion::V4)
        ));
    }

    #[test]
    fn test_capability_custom_creation() {
        let cap = PrimalCapability::Custom("custom-service".to_string());
        assert!(matches!(cap, PrimalCapability::Custom(_)));
    }

    #[test]
    fn test_capability_equality() {
        let cap1 = PrimalCapability::ZfsStorage;
        let cap2 = PrimalCapability::ZfsStorage;
        assert_eq!(cap1, cap2);
    }

    #[test]
    fn test_capability_inequality() {
        let cap1 = PrimalCapability::ZfsStorage;
        let cap2 = PrimalCapability::ApiGateway;
        assert_ne!(cap1, cap2);
    }

    #[test]
    fn test_capability_hash_in_hashmap() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(PrimalCapability::ZfsStorage, "zfs-service");
        map.insert(PrimalCapability::ApiGateway, "api-service");

        assert_eq!(map.get(&PrimalCapability::ZfsStorage), Some(&"zfs-service"));
        assert_eq!(map.get(&PrimalCapability::ApiGateway), Some(&"api-service"));
    }

    #[test]
    fn test_capability_clone() {
        let cap1 = PrimalCapability::Custom("test".to_string());
        let cap2 = cap1.clone();
        assert_eq!(cap1, cap2);
    }
}

// ============================================================================
// BINDING INFO TESTS
// ============================================================================

#[cfg(test)]
mod binding_info_tests {
    use super::*;

    #[test]
    fn test_binding_info_tcp_creation() {
        let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let binding = BindingInfo {
            address: addr,
            port: 8080,
            protocol: Protocol::Tcp,
        };

        assert_eq!(binding.address, addr);
        assert_eq!(binding.port, 8080);
        assert!(matches!(binding.protocol, Protocol::Tcp));
    }

    #[test]
    fn test_binding_info_http_creation() {
        let addr = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let binding = BindingInfo {
            address: addr,
            port: 80,
            protocol: Protocol::Http,
        };

        assert_eq!(binding.port, 80);
        assert!(matches!(binding.protocol, Protocol::Http));
    }

    #[test]
    fn test_binding_info_https_creation() {
        let addr = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let binding = BindingInfo {
            address: addr,
            port: 443,
            protocol: Protocol::Https,
        };

        assert_eq!(binding.port, 443);
        assert!(matches!(binding.protocol, Protocol::Https));
    }

    #[test]
    fn test_binding_info_clone() {
        let binding = BindingInfo {
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 8080,
            protocol: Protocol::Tcp,
        };

        let binding2 = binding.clone();
        assert_eq!(binding.port, binding2.port);
    }
}

// ============================================================================
// HEALTH STATUS TESTS
// ============================================================================

#[cfg(test)]
mod health_status_tests {
    use super::*;

    #[test]
    fn test_health_status_healthy() {
        let status = HealthStatus::Healthy;
        assert!(matches!(status, HealthStatus::Healthy));
    }

    #[test]
    fn test_health_status_degraded() {
        let status = HealthStatus::Degraded {
            reason: "High latency".to_string(),
        };
        assert!(matches!(status, HealthStatus::Degraded { .. }));
    }

    #[test]
    fn test_health_status_unhealthy() {
        let status = HealthStatus::Unhealthy {
            reason: "Service down".to_string(),
        };
        assert!(matches!(status, HealthStatus::Unhealthy { .. }));
    }

    #[test]
    fn test_health_status_clone() {
        let status = HealthStatus::Degraded {
            reason: "test".to_string(),
        };
        let status2 = status.clone();

        match status2 {
            HealthStatus::Degraded { reason } => assert_eq!(reason, "test"),
            _ => panic!("Expected degraded status"),
        }
    }
}

// ============================================================================
// SERVICE ENDPOINT TESTS
// ============================================================================

#[cfg(test)]
mod service_endpoint_tests {
    use super::*;

    #[test]
    fn test_service_endpoint_tcp_creation() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let endpoint = ServiceEndpoint::tcp(addr);

        assert_eq!(endpoint.address, addr);
        assert!(matches!(endpoint.protocol, Protocol::Tcp));
        assert!(endpoint.path.is_none());
    }

    #[test]
    fn test_service_endpoint_http_creation() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let endpoint = ServiceEndpoint::http(addr, "/api/v1");

        assert_eq!(endpoint.address, addr);
        assert!(matches!(endpoint.protocol, Protocol::Http));
        assert_eq!(endpoint.path, Some("/api/v1".to_string()));
    }

    #[test]
    fn test_service_endpoint_tcp_url() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let endpoint = ServiceEndpoint::tcp(addr);
        let url = endpoint.url();

        assert_eq!(url, "tcp://127.0.0.1:8080");
    }

    #[test]
    fn test_service_endpoint_http_url_with_path() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let endpoint = ServiceEndpoint::http(addr, "/api/v1");
        let url = endpoint.url();

        assert_eq!(url, "http://127.0.0.1:8080/api/v1");
    }

    #[test]
    fn test_service_endpoint_http_url_default_path() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let mut endpoint = ServiceEndpoint::tcp(addr);
        endpoint.protocol = Protocol::Http;
        let url = endpoint.url();

        assert_eq!(url, "http://127.0.0.1:8080/");
    }

    #[test]
    fn test_service_endpoint_https_url() {
        let addr = SocketAddr::from(([192, 168, 1, 100], 443));
        let mut endpoint = ServiceEndpoint::tcp(addr);
        endpoint.protocol = Protocol::Https;
        endpoint.path = Some("/secure".to_string());
        let url = endpoint.url();

        assert_eq!(url, "https://192.168.1.100:443/secure");
    }

    #[test]
    fn test_service_endpoint_udp_url() {
        let addr = SocketAddr::from(([10, 0, 0, 1], 5353));
        let mut endpoint = ServiceEndpoint::tcp(addr);
        endpoint.protocol = Protocol::Udp;
        let url = endpoint.url();

        assert_eq!(url, "udp://10.0.0.1:5353");
    }

    #[test]
    fn test_service_endpoint_clone() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let endpoint = ServiceEndpoint::http(addr, "/test");
        let endpoint2 = endpoint.clone();

        assert_eq!(endpoint.url(), endpoint2.url());
    }
}

// ============================================================================
// DISCOVERY QUERY TESTS
// ============================================================================

#[cfg(test)]
mod discovery_query_tests {
    use super::*;

    #[test]
    fn test_discovery_query_for_capability() {
        let query = DiscoveryQuery::for_capability(PrimalCapability::ZfsStorage);

        assert_eq!(query.required_capabilities.len(), 1);
        assert_eq!(query.required_capabilities[0], PrimalCapability::ZfsStorage);
        assert!(query.optional_capabilities.is_empty());
        assert!(query.max_latency.is_none());
    }

    #[test]
    fn test_discovery_query_custom_creation() {
        let query = DiscoveryQuery {
            required_capabilities: vec![
                PrimalCapability::ApiGateway,
                PrimalCapability::Authentication,
            ],
            optional_capabilities: vec![PrimalCapability::Observability],
            max_latency: Some(Duration::from_millis(100)),
            min_health: HealthStatus::Healthy,
        };

        assert_eq!(query.required_capabilities.len(), 2);
        assert_eq!(query.optional_capabilities.len(), 1);
        assert_eq!(query.max_latency, Some(Duration::from_millis(100)));
    }

    #[test]
    fn test_discovery_query_clone() {
        let query = DiscoveryQuery::for_capability(PrimalCapability::ServiceDiscovery);
        let query2 = query.clone();

        assert_eq!(
            query.required_capabilities.len(),
            query2.required_capabilities.len()
        );
    }
}

// ============================================================================
// PEER DESCRIPTOR TESTS
// ============================================================================

#[cfg(test)]
mod peer_descriptor_tests {
    use super::*;

    fn create_test_peer() -> PeerDescriptor {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        PeerDescriptor {
            id: PrimalId::from_environment().expect("Should create ID"),
            capabilities: vec![PrimalCapability::ApiGateway],
            endpoint: ServiceEndpoint::tcp(addr),
            last_seen: SystemTime::now(),
            health: HealthStatus::Healthy,
            latency: Some(Duration::from_millis(10)),
        }
    }

    #[test]
    fn test_peer_descriptor_creation() {
        let peer = create_test_peer();

        assert_eq!(peer.capabilities.len(), 1);
        assert!(matches!(peer.health, HealthStatus::Healthy));
        assert!(peer.latency.is_some());
    }

    #[test]
    fn test_peer_descriptor_with_multiple_capabilities() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let peer = PeerDescriptor {
            id: PrimalId::from_environment().expect("Should create ID"),
            capabilities: vec![
                PrimalCapability::ApiGateway,
                PrimalCapability::Authentication,
                PrimalCapability::Observability,
            ],
            endpoint: ServiceEndpoint::tcp(addr),
            last_seen: SystemTime::now(),
            health: HealthStatus::Healthy,
            latency: None,
        };

        assert_eq!(peer.capabilities.len(), 3);
    }

    #[test]
    fn test_peer_descriptor_degraded_health() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let peer = PeerDescriptor {
            id: PrimalId::from_environment().expect("Should create ID"),
            capabilities: vec![PrimalCapability::ZfsStorage],
            endpoint: ServiceEndpoint::tcp(addr),
            last_seen: SystemTime::now(),
            health: HealthStatus::Degraded {
                reason: "High load".to_string(),
            },
            latency: Some(Duration::from_millis(500)),
        };

        assert!(matches!(peer.health, HealthStatus::Degraded { .. }));
        assert!(peer.latency.unwrap() > Duration::from_millis(100));
    }

    #[test]
    fn test_peer_descriptor_clone() {
        let peer = create_test_peer();
        let peer2 = peer.clone();

        assert_eq!(peer.capabilities.len(), peer2.capabilities.len());
        assert_eq!(peer.endpoint.url(), peer2.endpoint.url());
    }
}

// ============================================================================
// DISCOVERY CONFIG TESTS
// ============================================================================

#[cfg(test)]
mod discovery_config_tests {
    use super::*;

    #[test]
    fn test_discovery_config_default() {
        let config = DiscoveryConfig::default();

        assert_eq!(config.announce_interval, Duration::from_secs(30));
        assert_eq!(config.refresh_interval, Duration::from_secs(60));
        assert_eq!(config.peer_ttl, Duration::from_secs(300));
    }

    #[test]
    fn test_discovery_config_custom() {
        let config = DiscoveryConfig {
            announce_interval: Duration::from_secs(10),
            refresh_interval: Duration::from_secs(20),
            peer_ttl: Duration::from_secs(180),
        };

        assert_eq!(config.announce_interval, Duration::from_secs(10));
        assert_eq!(config.refresh_interval, Duration::from_secs(20));
        assert_eq!(config.peer_ttl, Duration::from_secs(180));
    }

    #[test]
    fn test_discovery_config_clone() {
        let config = DiscoveryConfig::default();
        let config2 = config.clone();

        assert_eq!(config.announce_interval, config2.announce_interval);
        assert_eq!(config.refresh_interval, config2.refresh_interval);
    }
}

// ============================================================================
// PRIMAL SELF-KNOWLEDGE TESTS
// ============================================================================

#[cfg(test)]
mod self_knowledge_tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_self_knowledge() -> PrimalSelfKnowledge {
        PrimalSelfKnowledge {
            id: PrimalId::from_environment().expect("Should create ID"),
            capabilities: vec![
                PrimalCapability::ApiGateway,
                PrimalCapability::Authentication,
            ],
            binding: BindingInfo {
                address: IpAddr::V4(Ipv4Addr::LOCALHOST),
                port: 8080,
                protocol: Protocol::Http,
            },
            health: HealthStatus::Healthy,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_self_knowledge_creation() {
        let knowledge = create_test_self_knowledge();

        assert_eq!(knowledge.capabilities.len(), 2);
        assert_eq!(knowledge.binding.port, 8080);
        assert!(matches!(knowledge.health, HealthStatus::Healthy));
    }

    #[test]
    fn test_self_knowledge_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("region".to_string(), "us-west-2".to_string());

        let knowledge = PrimalSelfKnowledge {
            id: PrimalId::from_environment().expect("Should create ID"),
            capabilities: vec![PrimalCapability::ZfsStorage],
            binding: BindingInfo {
                address: IpAddr::V4(Ipv4Addr::LOCALHOST),
                port: 8080,
                protocol: Protocol::Tcp,
            },
            health: HealthStatus::Healthy,
            metadata,
        };

        assert_eq!(knowledge.metadata.len(), 2);
        assert_eq!(
            knowledge.metadata.get("version"),
            Some(&"1.0.0".to_string())
        );
    }

    #[test]
    fn test_self_knowledge_clone() {
        let knowledge = create_test_self_knowledge();
        let knowledge2 = knowledge.clone();

        assert_eq!(knowledge.capabilities.len(), knowledge2.capabilities.len());
        assert_eq!(knowledge.binding.port, knowledge2.binding.port);
    }
}

// ============================================================================
// CAPABILITY DISCOVERY MANAGER ASYNC TESTS
// ============================================================================

#[cfg(test)]
mod capability_manager_async_tests {
    use super::*;

    #[tokio::test]
    async fn test_capability_manager_initialization() {
        let capabilities = vec![PrimalCapability::ApiGateway];

        // May succeed or fail depending on environment, but shouldn't panic
        let _ = CapabilityDiscoveryManager::initialize(capabilities).await;
    }

    #[tokio::test]
    async fn test_capability_manager_with_multiple_capabilities() {
        let capabilities = vec![
            PrimalCapability::ApiGateway,
            PrimalCapability::Authentication,
            PrimalCapability::Observability,
        ];

        let _ = CapabilityDiscoveryManager::initialize(capabilities).await;
    }

    #[tokio::test]
    async fn test_capability_manager_with_zfs_capability() {
        let capabilities = vec![PrimalCapability::ZfsStorage];
        let _ = CapabilityDiscoveryManager::initialize(capabilities).await;
    }

    #[tokio::test]
    async fn test_capability_manager_with_custom_capability() {
        let capabilities = vec![PrimalCapability::Custom("test-service".to_string())];
        let _ = CapabilityDiscoveryManager::initialize(capabilities).await;
    }
}

// ============================================================================
// PROTOCOL TESTS
// ============================================================================

#[cfg(test)]
mod protocol_tests {
    use super::*;

    #[test]
    fn test_protocol_tcp() {
        let proto = Protocol::Tcp;
        assert!(matches!(proto, Protocol::Tcp));
    }

    #[test]
    fn test_protocol_udp() {
        let proto = Protocol::Udp;
        assert!(matches!(proto, Protocol::Udp));
    }

    #[test]
    fn test_protocol_http() {
        let proto = Protocol::Http;
        assert!(matches!(proto, Protocol::Http));
    }

    #[test]
    fn test_protocol_https() {
        let proto = Protocol::Https;
        assert!(matches!(proto, Protocol::Https));
    }

    #[test]
    fn test_protocol_clone() {
        let proto = Protocol::Http;
        let proto2 = proto;
        assert!(matches!(proto2, Protocol::Http));
    }
}

// ============================================================================
// NFS VERSION TESTS
// ============================================================================

#[cfg(test)]
mod nfs_version_tests {
    use super::*;

    #[test]
    fn test_nfs_version_v3() {
        let version = NfsVersion::V3;
        assert!(matches!(version, NfsVersion::V3));
    }

    #[test]
    fn test_nfs_version_v4() {
        let version = NfsVersion::V4;
        assert!(matches!(version, NfsVersion::V4));
    }

    #[test]
    fn test_nfs_version_equality() {
        let v3_1 = NfsVersion::V3;
        let v3_2 = NfsVersion::V3;
        assert_eq!(v3_1, v3_2);
    }

    #[test]
    fn test_nfs_version_inequality() {
        let v3 = NfsVersion::V3;
        let v4 = NfsVersion::V4;
        assert_ne!(v3, v4);
    }

    #[test]
    fn test_nfs_version_clone() {
        let v3 = NfsVersion::V3;
        let v3_clone = v3.clone();
        assert_eq!(v3, v3_clone);
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_discovery_workflow() {
        // Create capabilities
        let capabilities = vec![
            PrimalCapability::ApiGateway,
            PrimalCapability::Authentication,
        ];

        // Initialize manager (may fail in test environment)
        if let Ok(_manager) = CapabilityDiscoveryManager::initialize(capabilities).await {
            // Success - manager created
        }
    }

    #[test]
    fn test_query_and_endpoint_construction() {
        // Create query
        let query = DiscoveryQuery::for_capability(PrimalCapability::ZfsStorage);
        assert_eq!(query.required_capabilities.len(), 1);

        // Create endpoint
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        let endpoint = ServiceEndpoint::http(addr, "/api");
        assert!(endpoint.url().contains("http://"));

        // Create peer descriptor
        let peer = PeerDescriptor {
            id: PrimalId::from_environment().expect("Should create ID"),
            capabilities: query.required_capabilities.clone(),
            endpoint,
            last_seen: SystemTime::now(),
            health: HealthStatus::Healthy,
            latency: Some(Duration::from_millis(15)),
        };

        assert!(peer.latency.unwrap() < Duration::from_millis(100));
    }
}
