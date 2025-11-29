//! **COMPREHENSIVE NETWORK SERVICE TESTS**
//!
//! Tests for RealNetworkService and related functionality.

#[cfg(test)]
mod real_network_service_tests {
    use super::super::service::RealNetworkService;
    use super::super::types::{
        NetworkConfig, NetworkConfigBuilder, ServiceInfo, ServiceStatus,
    };

    // Test constants
    const TEST_PORT: u16 = 18080;
    
    fn test_endpoint() -> String {
        format!("localhost:{}", TEST_PORT)
    }

    // ==================== SERVICE CREATION TESTS ====================

    #[test]
    fn test_real_network_service_creation() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        assert!(format!("{:?}", service).contains("RealNetworkService"));
    }

    #[test]
    fn test_real_network_service_creation_with_custom_config() {
        let config = NetworkConfigBuilder::default()
            .with_api_port(9000)
            .with_bind_address("0.0.0.0".to_string())
            .build()
            .unwrap();

        let service = RealNetworkService::new(config);
        assert!(format!("{:?}", service).contains("RealNetworkService"));
    }

    #[test]
    fn test_real_network_service_creation_multiple_instances() {
        let config = NetworkConfig::default();

        for _ in 0..5 {
            let _service = RealNetworkService::new(config.clone());
        }
    }

    // ==================== STATISTICS TESTS ====================

    #[tokio::test]
    async fn test_get_network_statistics_initial() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let stats = service.get_network_statistics().await.unwrap();

        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.registered_services, 0);
        assert_eq!(stats.allocated_ports, 0);
        assert_eq!(stats.total_bytes_sent, 0);
        assert_eq!(stats.total_bytes_received, 0);
    }

    #[tokio::test]
    async fn test_get_network_statistics_after_service_registration() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        // Register a service
        let service_info = ServiceInfo::new(
            "test-service".to_string(),
            test_endpoint(),
        );

        service.register_service(service_info).await.unwrap();

        let stats = service.get_network_statistics().await.unwrap();
        assert_eq!(stats.registered_services, 1);
    }

    #[tokio::test]
    async fn test_get_network_statistics_after_port_allocation() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        // Allocate a port
        let _port = service.allocate_port_for_service("test-service").await.unwrap();

        let stats = service.get_network_statistics().await.unwrap();
        assert_eq!(stats.allocated_ports, 1);
    }

    // ==================== PORT ALLOCATION TESTS ====================

    #[tokio::test]
    async fn test_allocate_port_for_service() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let port = service.allocate_port_for_service("test-service").await.unwrap();
        assert!(port >= 1024 && port <= 65535);
    }

    #[tokio::test]
    async fn test_allocate_port_for_multiple_services() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let mut ports = vec![];

        for i in 0..5 {
            let port = service
                .allocate_port_for_service(&format!("service-{}", i))
                .await
                .unwrap();
            ports.push(port);
        }

        // All ports should be unique
        let unique_ports: std::collections::HashSet<_> = ports.iter().collect();
        assert_eq!(unique_ports.len(), 5);
    }

    #[tokio::test]
    async fn test_allocate_port_sequential() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let port1 = service.allocate_port_for_service("service-1").await.unwrap();
        let port2 = service.allocate_port_for_service("service-2").await.unwrap();

        assert_ne!(port1, port2);
    }

    #[tokio::test]
    async fn test_release_service_port() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let port = service.allocate_port_for_service("test-service").await.unwrap();

        let result = service.release_service_port(port).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_release_unallocated_port() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        // Try to release a port that was never allocated
        let result = service.release_service_port(9999).await;
        assert!(result.is_ok()); // Should not fail
    }

    #[tokio::test]
    async fn test_allocate_and_release_port_cycle() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let port = service.allocate_port_for_service("test-service").await.unwrap();
        service.release_service_port(port).await.unwrap();

        // Allocate again - port might be reused
        let port2 = service.allocate_port_for_service("test-service-2").await.unwrap();
        assert!(port2 >= 1024);
    }

    // ==================== SERVICE REGISTRATION TESTS ====================

    #[tokio::test]
    async fn test_register_service() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let service_info = ServiceInfo::new(
            "test-service".to_string(),
            test_endpoint(),
        );

        let result = service.register_service(service_info).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_multiple_services() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        for i in 0..10 {
            let service_info = ServiceInfo::new(
                format!("service-{}", i),
                format!("localhost:{}", 8080 + i),
            );

            service.register_service(service_info).await.unwrap();
        }

        let stats = service.get_network_statistics().await.unwrap();
        assert_eq!(stats.registered_services, 10);
    }

    #[tokio::test]
    async fn test_unregister_service() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let service_info = ServiceInfo::new(
            "test-service".to_string(),
            test_endpoint(),
        );

        let service_id = service_info.id().to_string();

        service.register_service(service_info).await.unwrap();
        let result = service.unregister_service(&service_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unregister_non_existent_service() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let result = service.unregister_service("non-existent").await;
        assert!(result.is_ok()); // Should not fail
    }

    #[tokio::test]
    async fn test_register_unregister_cycle() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let service_info = ServiceInfo::new(
            "test-service".to_string(),
            test_endpoint(),
        );

        let service_id = service_info.id().to_string();

        service.register_service(service_info.clone()).await.unwrap();
        service.unregister_service(&service_id).await.unwrap();

        let stats = service.get_network_statistics().await.unwrap();
        assert_eq!(stats.registered_services, 0);
    }

    // ==================== SERVICE STATUS TESTS ====================

    #[tokio::test]
    async fn test_get_service_status() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let status = service.get_service_status().await.unwrap();
        assert_eq!(status, ServiceStatus::Running);
    }

    // ==================== HEALTH CHECK TESTS ====================

    #[tokio::test]
    async fn test_health_check() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let healthy = service.health_check().await.unwrap();
        assert!(healthy);
    }

    #[tokio::test]
    async fn test_health_check_with_services() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        // Register some services
        for i in 0..3 {
            let service_info = ServiceInfo::new(
                format!("service-{}", i),
                format!("localhost:{}", 8080 + i),
            );
            service.register_service(service_info).await.unwrap();
        }

        let healthy = service.health_check().await.unwrap();
        assert!(healthy);
    }

    #[tokio::test]
    async fn test_health_check_with_ports() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        // Allocate some ports
        for i in 0..3 {
            service.allocate_port_for_service(&format!("service-{}", i))
                .await
                .unwrap();
        }

        let healthy = service.health_check().await.unwrap();
        assert!(healthy);
    }

    // ==================== STOP SERVICE TESTS ====================

    #[tokio::test]
    async fn test_stop_service() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let result = service.stop().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_stop_service_clears_connections() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        // Allocate some ports
        for i in 0..3 {
            service.allocate_port_for_service(&format!("service-{}", i))
                .await
                .unwrap();
        }

        service.stop().await.unwrap();

        let stats = service.get_network_statistics().await.unwrap();
        assert_eq!(stats.allocated_ports, 0);
    }

    // ==================== CONNECTION DETAILS TESTS ====================

    #[tokio::test]
    async fn test_get_connection_details_not_found() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let details = service.get_connection_details("non-existent").await;
        assert!(details.is_none());
    }

    // ==================== SERVICE DETAILS TESTS ====================

    #[tokio::test]
    async fn test_get_service_details_not_found() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let details = service.get_service_details("non-existent").await;
        assert!(details.is_none());
    }

    #[tokio::test]
    async fn test_get_service_details_found() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let service_info = ServiceInfo::new(
            "test-service".to_string(),
            test_endpoint(),
        );

        let service_id = service_info.id().to_string();

        service.register_service(service_info).await.unwrap();

        let details = service.get_service_details(&service_id).await;
        assert!(details.is_some());

        let details = details.unwrap();
        assert_eq!(details.id, service_id);
    }

    // ==================== CONCURRENCY TESTS ====================

    #[tokio::test]
    async fn test_concurrent_port_allocation() {
        use std::sync::Arc;

        let config = NetworkConfig::default();
        let service = Arc::new(RealNetworkService::new(config));

        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = tokio::spawn(async move {
                service_clone
                    .allocate_port_for_service(&format!("service-{}", i))
                    .await
            });
            handles.push(handle);
        }

        let mut ports = vec![];
        for handle in handles {
            let port = handle.await.unwrap().unwrap();
            ports.push(port);
        }

        // All ports should be unique
        let unique_ports: std::collections::HashSet<_> = ports.iter().collect();
        assert_eq!(unique_ports.len(), 10);
    }

    #[tokio::test]
    async fn test_concurrent_service_registration() {
        use std::sync::Arc;

        let config = NetworkConfig::default();
        let service = Arc::new(RealNetworkService::new(config));

        let mut handles = vec![];

        for i in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = tokio::spawn(async move {
                let service_info = ServiceInfo::new(
                    format!("service-{}", i),
                    format!("localhost:{}", 8080 + i),
                );
                service_clone.register_service(service_info).await
            });
            handles.push(handle);
        }

        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }

        let stats = service.get_network_statistics().await.unwrap();
        assert_eq!(stats.registered_services, 10);
    }

    #[tokio::test]
    async fn test_concurrent_mixed_operations() {
        use std::sync::Arc;

        let config = NetworkConfig::default();
        let service = Arc::new(RealNetworkService::new(config));

        let mut handles = vec![];

        // Mix of different operations
        for i in 0..20 {
            let service_clone = Arc::clone(&service);
            let handle = if i % 3 == 0 {
                // Allocate port
                tokio::spawn(async move {
                    service_clone
                        .allocate_port_for_service(&format!("service-{}", i))
                        .await
                        .map(|_| ())
                })
            } else if i % 3 == 1 {
                // Register service
                tokio::spawn(async move {
                    let service_info = ServiceInfo::new(
                        format!("service-{}", i),
                        format!("localhost:{}", 8080 + i),
                    );
                    service_clone.register_service(service_info).await
                })
            } else {
                // Get statistics
                tokio::spawn(async move { service_clone.get_network_statistics().await.map(|_| ()) })
            };
            handles.push(handle);
        }

        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }
    }

    // ==================== EDGE CASE TESTS ====================

    #[tokio::test]
    async fn test_allocate_port_with_long_service_name() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let long_name = "a".repeat(1000);
        let port = service.allocate_port_for_service(&long_name).await.unwrap();
        assert!(port >= 1024);
    }

    #[tokio::test]
    async fn test_allocate_port_with_unicode_service_name() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let unicode_name = "服务-🚀-test";
        let port = service.allocate_port_for_service(unicode_name).await.unwrap();
        assert!(port >= 1024);
    }

    #[tokio::test]
    async fn test_register_service_with_special_characters() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        let service_info = ServiceInfo::new(
            "test-service-@#$%".to_string(),
            test_endpoint(),
        );

        let result = service.register_service(service_info).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_stop_calls() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        service.stop().await.unwrap();
        let result = service.stop().await;
        assert!(result.is_ok()); // Should handle multiple stops gracefully
    }

    // ==================== STRESS TESTS ====================

    #[tokio::test]
    async fn test_many_service_registrations() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        for i in 0..100 {
            let service_info = ServiceInfo::new(
                format!("service-{}", i),
                format!("localhost:{}", 8080 + i),
            );
            service.register_service(service_info).await.unwrap();
        }

        let stats = service.get_network_statistics().await.unwrap();
        assert_eq!(stats.registered_services, 100);
    }

    #[tokio::test]
    async fn test_many_port_allocations() {
        let config = NetworkConfig::default();
        let service = RealNetworkService::new(config);

        for i in 0..50 {
            service.allocate_port_for_service(&format!("service-{}", i))
                .await
                .unwrap();
        }

        let stats = service.get_network_statistics().await.unwrap();
        assert_eq!(stats.allocated_ports, 50);
    }
}

