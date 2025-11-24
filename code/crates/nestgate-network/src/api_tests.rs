//! **COMPREHENSIVE NETWORK API TESTS**
//!
//! Tests for NetworkApi, OrchestrationCapability, and service management.

#[cfg(test)]
mod network_api_tests {
    use super::super::api::{NetworkApi, OrchestrationCapability, ServiceInstance, ServiceStatus};
    use chrono::Utc;

    // ==================== ORCHESTRATION CAPABILITY TESTS ====================

    #[test]
    fn test_orchestration_capability_creation() {
        let base_url = "http://localhost:8080".to_string();
        let client = OrchestrationCapability::new(base_url.clone());

        assert_eq!(client.base_url, base_url);
        assert!(format!("{:?}", client).contains("OrchestrationCapability"));
    }

    #[test]
    fn test_orchestration_capability_with_empty_url() {
        let client = OrchestrationCapability::new(String::new());
        assert_eq!(client.base_url, "");
    }

    #[test]
    fn test_orchestration_capability_with_custom_port() {
        let base_url = "http://example.com:9999".to_string();
        let client = OrchestrationCapability::new(base_url.clone());
        assert_eq!(client.base_url, base_url);
    }

    #[test]
    fn test_orchestration_capability_https() {
        let base_url = "https://secure.example.com".to_string();
        let client = OrchestrationCapability::new(base_url.clone());
        assert_eq!(client.base_url, base_url);
    }

    #[test]
    fn test_orchestration_capability_clone() {
        let client1 = OrchestrationCapability::new("http://test.com".to_string());
        let client2 = client1.clone();
        assert_eq!(client1.base_url, client2.base_url);
    }

    // ==================== SERVICE INSTANCE TESTS ====================

    #[test]
    fn test_service_instance_creation() {
        let now = Utc::now();
        let service = ServiceInstance {
            id: "test-service-1".to_string(),
            name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 8080,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        assert_eq!(service.id, "test-service-1");
        assert_eq!(service.name, "test-service");
        assert_eq!(service.host, "localhost");
        assert_eq!(service.port, 8080);
        assert_eq!(service.status, ServiceStatus::Running);
    }

    #[test]
    fn test_service_instance_clone() {
        let now = Utc::now();
        let service1 = ServiceInstance {
            id: "test-1".to_string(),
            name: "service-1".to_string(),
            host: "host1".to_string(),
            port: 3000,
            status: ServiceStatus::Starting,
            created_at: now,
            updated_at: now,
        };

        let service2 = service1.clone();
        assert_eq!(service1.id, service2.id);
        assert_eq!(service1.name, service2.name);
        assert_eq!(service1.status, service2.status);
    }

    #[test]
    fn test_service_instance_with_different_ports() {
        let now = Utc::now();
        let ports = vec![80, 443, 3000, 8080, 9000];

        for port in ports {
            let service = ServiceInstance {
                id: format!("service-{}", port),
                name: "test-service".to_string(),
                host: "localhost".to_string(),
                port,
                status: ServiceStatus::Running,
                created_at: now,
                updated_at: now,
            };
            assert_eq!(service.port, port);
        }
    }

    // ==================== SERVICE STATUS TESTS ====================

    #[test]
    fn test_service_status_variants() {
        let statuses = vec![
            ServiceStatus::Starting,
            ServiceStatus::Running,
            ServiceStatus::Stopping,
            ServiceStatus::Stopped,
            ServiceStatus::Failed,
        ];

        for status in statuses {
            assert!(format!("{:?}", status).len() > 0);
        }
    }

    #[test]
    fn test_service_status_equality() {
        assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
        assert_eq!(ServiceStatus::Stopped, ServiceStatus::Stopped);
        assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
    }

    #[test]
    fn test_service_status_clone() {
        let status1 = ServiceStatus::Running;
        let status2 = status1.clone();
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_service_status_serialization_round_trip() {
        let status = ServiceStatus::Running;
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: ServiceStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }

    // ==================== NETWORK API TESTS ====================

    #[test]
    fn test_network_api_creation() {
        let api = NetworkApi::new();
        assert!(format!("{:?}", api).contains("NetworkApi"));
    }

    #[test]
    fn test_network_api_default() {
        let api = NetworkApi::default();
        assert!(format!("{:?}", api).contains("NetworkApi"));
    }

    #[tokio::test]
    async fn test_network_api_initialize_with_orchestration() {
        let mut api = NetworkApi::new();
        let endpoint = "http://orchestrator:8080".to_string();

        let result = api.initialize_with_orchestration(endpoint);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_network_api_register_service_without_orchestration() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let service = ServiceInstance {
            id: "test-1".to_string(),
            name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 3000,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        let result = api.register_service(service).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_network_api_list_services_empty() {
        let api = NetworkApi::new();
        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 0);
    }

    #[tokio::test]
    async fn test_network_api_list_services_after_registration() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let service = ServiceInstance {
            id: "test-1".to_string(),
            name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 3000,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        api.register_service(service).await.unwrap();

        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].name, "test-service");
    }

    #[tokio::test]
    async fn test_network_api_register_multiple_services() {
        let api = NetworkApi::new();
        let now = Utc::now();

        for i in 0..5 {
            let service = ServiceInstance {
                id: format!("service-{}", i),
                name: format!("service-{}", i),
                host: "localhost".to_string(),
                port: 3000 + i as u16,
                status: ServiceStatus::Running,
                created_at: now,
                updated_at: now,
            };
            api.register_service(service).await.unwrap();
        }

        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 5);
    }

    #[tokio::test]
    async fn test_network_api_get_service_status_not_found() {
        let api = NetworkApi::new();
        let result = api.get_service_status("non-existent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_network_api_get_service_status_found() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let service = ServiceInstance {
            id: "test-1".to_string(),
            name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 3000,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        api.register_service(service).await.unwrap();

        let status = api.get_service_status("test-service").await.unwrap();
        assert_eq!(status, ServiceStatus::Running);
    }

    #[tokio::test]
    async fn test_network_api_allocate_port_without_orchestration() {
        let api = NetworkApi::new();
        let result = api.allocate_port("test-service", "http").await;
        // Should fail because orchestration is required
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_network_api_release_port_without_orchestration() {
        let api = NetworkApi::new();
        let result = api.release_port("test-service").await;
        // Should fail because orchestration is required
        assert!(result.is_err());
    }

    // ==================== INTEGRATION TESTS ====================

    #[tokio::test]
    async fn test_network_api_full_service_lifecycle() {
        let api = NetworkApi::new();
        let now = Utc::now();

        // Register service
        let service = ServiceInstance {
            id: "lifecycle-test".to_string(),
            name: "lifecycle-service".to_string(),
            host: "localhost".to_string(),
            port: 4000,
            status: ServiceStatus::Starting,
            created_at: now,
            updated_at: now,
        };

        api.register_service(service.clone()).await.unwrap();

        // Verify registration
        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 1);

        // Check status
        let status = api.get_service_status("lifecycle-service").await.unwrap();
        assert_eq!(status, ServiceStatus::Starting);
    }

    #[tokio::test]
    async fn test_network_api_concurrent_service_registration() {
        use std::sync::Arc;

        let api = Arc::new(NetworkApi::new());
        let now = Utc::now();

        let mut handles = vec![];

        for i in 0..10 {
            let api_clone = Arc::clone(&api);
            let handle = tokio::spawn(async move {
                let service = ServiceInstance {
                    id: format!("concurrent-{}", i),
                    name: format!("service-{}", i),
                    host: "localhost".to_string(),
                    port: 5000 + i as u16,
                    status: ServiceStatus::Running,
                    created_at: now,
                    updated_at: now,
                };
                api_clone.register_service(service).await
            });
            handles.push(handle);
        }

        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }

        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 10);
    }

    #[tokio::test]
    async fn test_network_api_service_update() {
        let api = NetworkApi::new();
        let now = Utc::now();

        // Register initial service
        let service1 = ServiceInstance {
            id: "update-test".to_string(),
            name: "update-service".to_string(),
            host: "localhost".to_string(),
            port: 6000,
            status: ServiceStatus::Starting,
            created_at: now,
            updated_at: now,
        };

        api.register_service(service1).await.unwrap();

        // Update service (re-register with new status)
        let service2 = ServiceInstance {
            id: "update-test-2".to_string(),
            name: "update-service".to_string(),
            host: "localhost".to_string(),
            port: 6000,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: Utc::now(),
        };

        api.register_service(service2).await.unwrap();

        // Should have updated, not added
        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 1);

        let status = api.get_service_status("update-service").await.unwrap();
        assert_eq!(status, ServiceStatus::Running);
    }

    #[test]
    fn test_service_instance_debug_format() {
        let now = Utc::now();
        let service = ServiceInstance {
            id: "debug-test".to_string(),
            name: "debug-service".to_string(),
            host: "localhost".to_string(),
            port: 7000,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("debug-test"));
        assert!(debug_str.contains("debug-service"));
    }

    #[test]
    fn test_service_status_all_variants() {
        let statuses = vec![
            (ServiceStatus::Starting, "Starting"),
            (ServiceStatus::Running, "Running"),
            (ServiceStatus::Stopping, "Stopping"),
            (ServiceStatus::Stopped, "Stopped"),
            (ServiceStatus::Failed, "Failed"),
        ];

        for (status, name) in statuses {
            let debug_str = format!("{:?}", status);
            assert_eq!(debug_str, name);
        }
    }

    // ==================== ERROR HANDLING TESTS ====================

    #[tokio::test]
    async fn test_network_api_get_status_invalid_service() {
        let api = NetworkApi::new();
        let result = api.get_service_status("").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_network_api_get_status_special_characters() {
        let api = NetworkApi::new();
        let result = api.get_service_status("service@#$%").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_network_api_register_service_with_different_statuses() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let statuses = vec![
            ServiceStatus::Starting,
            ServiceStatus::Running,
            ServiceStatus::Stopping,
            ServiceStatus::Stopped,
            ServiceStatus::Failed,
        ];

        for (i, status) in statuses.iter().enumerate() {
            let service = ServiceInstance {
                id: format!("status-test-{}", i),
                name: format!("service-{}", i),
                host: "localhost".to_string(),
                port: 8000 + i as u16,
                status: status.clone(),
                created_at: now,
                updated_at: now,
            };
            api.register_service(service).await.unwrap();
        }

        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 5);
    }

    // ==================== ROUTER TESTS ====================

    #[test]
    fn test_network_api_create_router() {
        let api = NetworkApi::new();
        let _router = api.create_router();
        // Router creation should succeed
    }

    // ==================== EDGE CASE TESTS ====================

    #[tokio::test]
    async fn test_network_api_list_services_ordering() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let names = vec!["zebra", "alpha", "beta", "gamma"];

        for name in &names {
            let service = ServiceInstance {
                id: format!("{}-id", name),
                name: name.to_string(),
                host: "localhost".to_string(),
                port: 9000,
                status: ServiceStatus::Running,
                created_at: now,
                updated_at: now,
            };
            api.register_service(service).await.unwrap();
        }

        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 4);
    }

    #[tokio::test]
    async fn test_network_api_service_with_long_name() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let long_name = "a".repeat(1000);

        let service = ServiceInstance {
            id: "long-name-test".to_string(),
            name: long_name.clone(),
            host: "localhost".to_string(),
            port: 9001,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        api.register_service(service).await.unwrap();

        let status = api.get_service_status(&long_name).await.unwrap();
        assert_eq!(status, ServiceStatus::Running);
    }

    #[tokio::test]
    async fn test_network_api_service_with_unicode_name() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let unicode_name = "服务-🚀-test";

        let service = ServiceInstance {
            id: "unicode-test".to_string(),
            name: unicode_name.to_string(),
            host: "localhost".to_string(),
            port: 9002,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        api.register_service(service).await.unwrap();

        let status = api.get_service_status(unicode_name).await.unwrap();
        assert_eq!(status, ServiceStatus::Running);
    }

    #[tokio::test]
    async fn test_network_api_high_port_numbers() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let high_port = 65535;

        let service = ServiceInstance {
            id: "high-port-test".to_string(),
            name: "high-port-service".to_string(),
            host: "localhost".to_string(),
            port: high_port,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        api.register_service(service).await.unwrap();

        let services = api.list_services().await.unwrap();
        assert_eq!(services[0].port, high_port);
    }

    #[tokio::test]
    async fn test_network_api_low_port_numbers() {
        let api = NetworkApi::new();
        let now = Utc::now();

        let low_port = 1;

        let service = ServiceInstance {
            id: "low-port-test".to_string(),
            name: "low-port-service".to_string(),
            host: "localhost".to_string(),
            port: low_port,
            status: ServiceStatus::Running,
            created_at: now,
            updated_at: now,
        };

        api.register_service(service).await.unwrap();

        let services = api.list_services().await.unwrap();
        assert_eq!(services[0].port, low_port);
    }

    // ==================== CONCURRENCY TESTS ====================

    #[tokio::test]
    async fn test_network_api_concurrent_reads() {
        use std::sync::Arc;

        let api = Arc::new(NetworkApi::new());
        let now = Utc::now();

        // Register some services
        for i in 0..5 {
            let service = ServiceInstance {
                id: format!("read-test-{}", i),
                name: format!("service-{}", i),
                host: "localhost".to_string(),
                port: 10000 + i as u16,
                status: ServiceStatus::Running,
                created_at: now,
                updated_at: now,
            };
            api.register_service(service).await.unwrap();
        }

        // Concurrent reads
        let mut handles = vec![];

        for _ in 0..20 {
            let api_clone = Arc::clone(&api);
            let handle = tokio::spawn(async move { api_clone.list_services().await });
            handles.push(handle);
        }

        for handle in handles {
            let services = handle.await.unwrap().unwrap();
            assert_eq!(services.len(), 5);
        }
    }

    #[tokio::test]
    async fn test_network_api_mixed_concurrent_operations() {
        use std::sync::Arc;

        let api = Arc::new(NetworkApi::new());
        let now = Utc::now();

        let mut handles = vec![];

        // Mix of writes and reads
        for i in 0..10 {
            let api_clone = Arc::clone(&api);
            let handle = if i % 2 == 0 {
                // Write
                tokio::spawn(async move {
                    let service = ServiceInstance {
                        id: format!("mixed-{}", i),
                        name: format!("service-{}", i),
                        host: "localhost".to_string(),
                        port: 11000 + i as u16,
                        status: ServiceStatus::Running,
                        created_at: now,
                        updated_at: now,
                    };
                    api_clone.register_service(service).await
                })
            } else {
                // Read
                tokio::spawn(async move {
                    api_clone.list_services().await.map(|_| ())
                })
            };
            handles.push(handle);
        }

        for handle in handles {
            assert!(handle.await.unwrap().is_ok());
        }

        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 5); // Only 5 writes (even numbers)
    }
}

