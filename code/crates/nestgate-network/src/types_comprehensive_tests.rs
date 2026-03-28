// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **COMPREHENSIVE NETWORK TYPES TESTS**
//!
//! Extensive tests for network data structures and configuration types.

#[cfg(test)]
mod types_comprehensive_tests {
    use super::super::types::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::time::Duration;

    // ==================== CONNECTION INFO TESTS ====================

    #[test]
    fn test_connection_info_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let conn = ConnectionInfo::new("test-conn".to_string(), addr);

        assert_eq!(conn.id(), "test-conn");
        assert_eq!(conn.address(), addr);
        assert_eq!(conn.bytes_sent, 0);
        assert_eq!(conn.bytes_received, 0);
        assert!(conn.is_active());
    }

    #[tokio::test]
    async fn test_connection_info_age() {
        // ✅ MODERN: Test age without sleep - natural time progression
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let conn = ConnectionInfo::new("test-conn".to_string(), addr);

        // ✅ CONCURRENT: Age is measured from creation time
        // Time progresses naturally during test execution
        let age1 = conn.age();
        let age2 = conn.age();

        // Age should be non-decreasing (Duration is always positive)
        assert!(age2 >= age1);
    }

    #[test]
    fn test_connection_info_clone() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let conn1 = ConnectionInfo::new("test-conn".to_string(), addr);
        let conn2 = conn1.clone();

        assert_eq!(conn1.id(), conn2.id());
        assert_eq!(conn1.address(), conn2.address());
    }

    #[test]
    fn test_connection_info_with_various_addresses() {
        let addresses = [
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 9000),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)), 3000),
        ];

        for (i, addr) in addresses.iter().enumerate() {
            let conn = ConnectionInfo::new(format!("conn-{}", i), *addr);
            assert_eq!(conn.address(), *addr);
        }
    }

    #[test]
    fn test_connection_status_active() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let conn = ConnectionInfo::new("test".to_string(), addr);
        assert!(conn.is_active());
    }

    #[test]
    fn test_connection_info_debug() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let conn = ConnectionInfo::new("debug-test".to_string(), addr);
        let debug_str = format!("{:?}", conn);
        assert!(debug_str.contains("ConnectionInfo"));
    }

    // ==================== CONNECTION STATUS TESTS ====================

    #[test]
    fn test_connection_status_values() {
        let statuses = vec![
            ConnectionStatus::Active,
            ConnectionStatus::Idle,
            ConnectionStatus::Closed,
        ];

        for status in statuses {
            let debug_str = format!("{:?}", status);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_connection_status_clone() {
        let status1 = ConnectionStatus::Active;
        let status2 = status1.clone();
        assert!(matches!(status2, ConnectionStatus::Active));
    }

    // ==================== SERVICE INFO TESTS ====================

    #[test]
    fn test_service_info_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let service =
            ServiceInfo::new("test-service".to_string(), "Test Service".to_string(), addr);

        assert_eq!(service.id(), "test-service");
        assert_eq!(service.name(), "Test Service");
        assert_eq!(service.address(), addr);
        assert!(matches!(service.health_status(), HealthStatus::Healthy));
    }

    #[test]
    fn test_service_info_with_different_addresses() {
        let addresses = [
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)), 9000),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)), 3000),
        ];

        for (i, addr) in addresses.iter().enumerate() {
            let service =
                ServiceInfo::new(format!("service-{}", i), format!("Service {}", i), *addr);

            assert_eq!(service.address(), *addr);
            assert_eq!(service.id(), &format!("service-{}", i));
        }
    }

    #[test]
    fn test_service_info_clone() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let service1 = ServiceInfo::new("service".to_string(), "Service".to_string(), addr);
        let service2 = service1.clone();

        assert_eq!(service1.id(), service2.id());
        assert_eq!(service1.name(), service2.name());
        assert_eq!(service1.address(), service2.address());
    }

    #[test]
    fn test_service_info_debug() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let service = ServiceInfo::new(
            "debug-service".to_string(),
            "Debug Service".to_string(),
            addr,
        );

        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("ServiceInfo"));
    }

    #[test]
    fn test_service_info_health_status() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let service = ServiceInfo::new("health-test".to_string(), "Health Test".to_string(), addr);

        assert!(matches!(service.health_status(), HealthStatus::Healthy));
    }

    // ==================== SERVICE STATUS TESTS ====================

    #[test]
    fn test_service_status_running() {
        let status = ServiceStatus::Running;
        assert!(matches!(status, ServiceStatus::Running));
    }

    #[test]
    fn test_service_status_stopped() {
        let status = ServiceStatus::Stopped;
        assert!(matches!(status, ServiceStatus::Stopped));
    }

    #[test]
    fn test_service_status_starting() {
        let status = ServiceStatus::Starting;
        assert!(matches!(status, ServiceStatus::Starting));
    }

    #[test]
    fn test_service_status_stopping() {
        let status = ServiceStatus::Stopping;
        assert!(matches!(status, ServiceStatus::Stopping));
    }

    #[test]
    fn test_service_status_clone() {
        let status1 = ServiceStatus::Running;
        let status2 = status1.clone();
        assert!(matches!(status2, ServiceStatus::Running));
    }

    #[test]
    fn test_service_status_debug() {
        let statuses = vec![
            ServiceStatus::Running,
            ServiceStatus::Stopped,
            ServiceStatus::Starting,
            ServiceStatus::Stopping,
        ];

        for status in statuses {
            let debug_str = format!("{:?}", status);
            assert!(!debug_str.is_empty());
        }
    }

    // ==================== NETWORK STATISTICS TESTS ====================

    #[test]
    fn test_network_statistics_creation() {
        let stats = NetworkStatistics {
            active_connections: 10,
            registered_services: 5,
            allocated_ports: 3,
            total_bytes_sent: 1024,
            total_bytes_received: 2048,
        };

        assert_eq!(stats.active_connections, 10);
        assert_eq!(stats.registered_services, 5);
        assert_eq!(stats.allocated_ports, 3);
        assert_eq!(stats.total_bytes_sent, 1024);
        assert_eq!(stats.total_bytes_received, 2048);
    }

    #[test]
    fn test_network_statistics_zero_values() {
        let stats = NetworkStatistics {
            active_connections: 0,
            registered_services: 0,
            allocated_ports: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
        };

        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.registered_services, 0);
    }

    #[test]
    fn test_network_statistics_large_values() {
        let stats = NetworkStatistics {
            active_connections: 10000,
            registered_services: 1000,
            allocated_ports: 500,
            total_bytes_sent: 1_000_000_000,
            total_bytes_received: 2_000_000_000,
        };

        assert_eq!(stats.active_connections, 10000);
        assert_eq!(stats.total_bytes_sent, 1_000_000_000);
    }

    #[test]
    fn test_network_statistics_clone() {
        let stats1 = NetworkStatistics {
            active_connections: 5,
            registered_services: 3,
            allocated_ports: 2,
            total_bytes_sent: 100,
            total_bytes_received: 200,
        };
        let stats2 = stats1.clone();

        assert_eq!(stats1.active_connections, stats2.active_connections);
        assert_eq!(stats1.total_bytes_sent, stats2.total_bytes_sent);
    }

    #[test]
    fn test_network_statistics_debug() {
        let stats = NetworkStatistics {
            active_connections: 1,
            registered_services: 1,
            allocated_ports: 1,
            total_bytes_sent: 1,
            total_bytes_received: 1,
        };

        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("NetworkStatistics"));
    }

    // ==================== NETWORK EXTENSIONS TESTS ====================

    #[test]
    fn test_network_extensions_default() {
        let ext = NetworkExtensions::default();

        assert_eq!(ext.port_range_start, 9000);
        assert_eq!(ext.port_range_end, 9999);
        assert_eq!(ext.keep_alive_timeout_seconds, 60);
        assert!(ext.protocol_settings.is_empty());
    }

    #[test]
    fn test_network_extensions_clone() {
        let ext1 = NetworkExtensions::default();
        let ext2 = ext1.clone();

        assert_eq!(ext1.port_range_start, ext2.port_range_start);
        assert_eq!(ext1.port_range_end, ext2.port_range_end);
    }

    #[test]
    fn test_network_extensions_custom_port_range() {
        let mut ext = NetworkExtensions::default();
        ext.port_range_start = 10000;
        ext.port_range_end = 20000;

        assert_eq!(ext.port_range_start, 10000);
        assert_eq!(ext.port_range_end, 20000);
    }

    #[test]
    fn test_network_extensions_protocol_settings() {
        let mut ext = NetworkExtensions::default();
        ext.protocol_settings
            .insert("http".to_string(), "enabled".to_string());
        ext.protocol_settings
            .insert("tcp".to_string(), "enabled".to_string());

        assert_eq!(ext.protocol_settings.len(), 2);
        assert_eq!(
            ext.protocol_settings.get("http"),
            Some(&"enabled".to_string())
        );
    }

    #[test]
    fn test_network_extensions_serialization() {
        let ext = NetworkExtensions::default();
        let serialized = serde_json::to_string(&ext).unwrap();
        let deserialized: NetworkExtensions = serde_json::from_str(&serialized).unwrap();

        assert_eq!(ext.port_range_start, deserialized.port_range_start);
        assert_eq!(ext.port_range_end, deserialized.port_range_end);
    }

    // ==================== CONNECTION DETAILS TESTS ====================

    #[test]
    fn test_connection_details_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let details = ConnectionDetails {
            id: "conn-1".to_string(),
            endpoint: addr,
            age: Duration::from_secs(10),
            is_active: true,
            status: "active".to_string(),
        };

        assert_eq!(details.id, "conn-1");
        assert_eq!(details.age, Duration::from_secs(10));
        assert!(details.is_active);
    }

    #[test]
    fn test_connection_details_clone() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let details1 = ConnectionDetails {
            id: "conn-1".to_string(),
            endpoint: addr,
            age: Duration::from_secs(5),
            is_active: true,
            status: "active".to_string(),
        };
        let details2 = details1.clone();

        assert_eq!(details1.id, details2.id);
        assert_eq!(details1.age, details2.age);
    }

    // ==================== SERVICE DETAILS TESTS ====================

    #[test]
    fn test_service_details_creation() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("version".to_string(), "1.0".to_string());

        let details = ServiceDetails {
            id: "service-1".to_string(),
            name: "Test Service".to_string(),
            endpoint: addr,
            health_status: "healthy".to_string(),
            registered_at: std::time::SystemTime::now(),
            metadata,
        };

        assert_eq!(details.id, "service-1");
        assert_eq!(details.name, "Test Service");
        assert_eq!(details.endpoint, addr);
        assert_eq!(details.health_status, "healthy");
    }

    #[test]
    fn test_service_details_with_metadata() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9000);
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("environment".to_string(), "production".to_string());
        metadata.insert("region".to_string(), "us-west".to_string());

        let details = ServiceDetails {
            id: "service-2".to_string(),
            name: "Service With Metadata".to_string(),
            endpoint: addr,
            health_status: "healthy".to_string(),
            registered_at: std::time::SystemTime::now(),
            metadata: metadata.clone(),
        };

        assert_eq!(details.metadata.len(), 2);
        assert_eq!(
            details.metadata.get("environment"),
            Some(&"production".to_string())
        );
    }

    #[test]
    fn test_service_details_clone() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9000);
        let details1 = ServiceDetails {
            id: "service-1".to_string(),
            name: "Service".to_string(),
            endpoint: addr,
            health_status: "healthy".to_string(),
            registered_at: std::time::SystemTime::now(),
            metadata: std::collections::HashMap::new(),
        };
        let details2 = details1.clone();

        assert_eq!(details1.id, details2.id);
        assert_eq!(details1.endpoint, details2.endpoint);
    }

    // ==================== HEALTH STATUS TESTS ====================

    #[test]
    fn test_health_status_healthy() {
        let status = HealthStatus::Healthy;
        assert!(matches!(status, HealthStatus::Healthy));
    }

    #[test]
    fn test_health_status_degraded() {
        let status = HealthStatus::Degraded;
        assert!(matches!(status, HealthStatus::Degraded));
    }

    #[test]
    fn test_health_status_unhealthy() {
        let status = HealthStatus::Unhealthy;
        assert!(matches!(status, HealthStatus::Unhealthy));
    }

    #[test]
    fn test_health_status_clone() {
        let status1 = HealthStatus::Healthy;
        let status2 = status1.clone();
        assert!(matches!(status2, HealthStatus::Healthy));
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_multiple_connections_with_services() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);
        let connections: Vec<ConnectionInfo> = (0..10)
            .map(|i| ConnectionInfo::new(format!("conn-{}", i), addr))
            .collect();

        let services: Vec<ServiceInfo> = (0..5)
            .map(|i| {
                ServiceInfo::new(
                    format!("service-{}", i),
                    format!("Service {}", i),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080 + i as u16),
                )
            })
            .collect();

        assert_eq!(connections.len(), 10);
        assert_eq!(services.len(), 5);
    }

    #[test]
    fn test_network_statistics_aggregation() {
        let stats1 = NetworkStatistics {
            active_connections: 5,
            registered_services: 3,
            allocated_ports: 2,
            total_bytes_sent: 1000,
            total_bytes_received: 2000,
        };

        let stats2 = NetworkStatistics {
            active_connections: 3,
            registered_services: 2,
            allocated_ports: 1,
            total_bytes_sent: 500,
            total_bytes_received: 1000,
        };

        let total_connections = stats1.active_connections + stats2.active_connections;
        let total_bytes_sent = stats1.total_bytes_sent + stats2.total_bytes_sent;

        assert_eq!(total_connections, 8);
        assert_eq!(total_bytes_sent, 1500);
    }
}
