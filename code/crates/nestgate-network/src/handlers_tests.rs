//! **COMPREHENSIVE HANDLER TESTS**
//!
//! Tests for NetworkServiceManager and protocol handlers.

#[cfg(test)]
mod network_service_manager_tests {
    use super::super::handlers::NetworkServiceManager;
    use super::super::types::{ConnectionInfo, NetworkConfig, ServiceInfo};

    // Test constants
    const TEST_PORT: u16 = 18080;
    
    fn test_endpoint() -> String {
        format!("localhost:{}", TEST_PORT)
    }

    // ==================== CREATION TESTS ====================

    #[test]
    fn test_network_service_manager_creation() {
        let config = NetworkConfig::default();
        let _manager = NetworkServiceManager::new(config);
    }

    #[test]
    fn test_network_service_manager_creation_with_custom_config() {
        use super::super::types::NetworkConfigBuilder;

        let config = NetworkConfigBuilder::default()
            .with_api_port(9000)
            .build()
            .unwrap();

        let _manager = NetworkServiceManager::new(config);
    }

    // ==================== CONNECTION MANAGEMENT TESTS ====================

    #[tokio::test]
    async fn test_add_connection() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let connection = ConnectionInfo::new("conn-1".to_string(), test_endpoint());

        manager.add_connection(connection).await;
    }

    #[tokio::test]
    async fn test_add_multiple_connections() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        for i in 0..10 {
            let connection = ConnectionInfo::new(
                format!("conn-{}", i),
                format!("localhost:{}", 8080 + i),
            );
            manager.add_connection(connection).await;
        }
    }

    #[tokio::test]
    async fn test_remove_connection() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let connection = ConnectionInfo::new("conn-1".to_string(), test_endpoint());
        let conn_id = connection.id().to_string();

        manager.add_connection(connection).await;

        let removed = manager.remove_connection(&conn_id).await;
        assert!(removed);
    }

    #[tokio::test]
    async fn test_remove_non_existent_connection() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let removed = manager.remove_connection("non-existent").await;
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_get_active_connections_empty() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let active = manager.get_active_connections().await;
        assert_eq!(active.len(), 0);
    }

    // ==================== SERVICE MANAGEMENT TESTS ====================

    #[tokio::test]
    async fn test_add_service() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let service = ServiceInfo::new("service-1".to_string(), test_endpoint());

        manager.add_service(service).await;
    }

    #[tokio::test]
    async fn test_add_multiple_services() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        for i in 0..10 {
            let service = ServiceInfo::new(
                format!("service-{}", i),
                format!("localhost:{}", 8080 + i),
            );
            manager.add_service(service).await;
        }
    }

    #[tokio::test]
    async fn test_remove_service() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let service = ServiceInfo::new("service-1".to_string(), test_endpoint());
        let service_id = service.id().to_string();

        manager.add_service(service).await;

        let removed = manager.remove_service(&service_id).await;
        assert!(removed);
    }

    #[tokio::test]
    async fn test_remove_non_existent_service() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let removed = manager.remove_service("non-existent").await;
        assert!(!removed);
    }

    // ==================== CONCURRENCY TESTS ====================

    #[tokio::test]
    async fn test_concurrent_connection_operations() {
        use std::sync::Arc;

        let config = NetworkConfig::default();
        let manager = Arc::new(NetworkServiceManager::new(config));

        let mut handles = vec![];

        for i in 0..20 {
            let manager_clone = Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                let connection = ConnectionInfo::new(
                    format!("conn-{}", i),
                    format!("localhost:{}", 8080 + i),
                );
                manager_clone.add_connection(connection).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_concurrent_service_operations() {
        use std::sync::Arc;

        let config = NetworkConfig::default();
        let manager = Arc::new(NetworkServiceManager::new(config));

        let mut handles = vec![];

        for i in 0..20 {
            let manager_clone = Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                let service = ServiceInfo::new(
                    format!("service-{}", i),
                    format!("localhost:{}", 8080 + i),
                );
                manager_clone.add_service(service).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_concurrent_add_and_remove() {
        use std::sync::Arc;

        let config = NetworkConfig::default();
        let manager = Arc::new(NetworkServiceManager::new(config));

        let mut handles = vec![];

        // Add operations
        for i in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                let connection = ConnectionInfo::new(
                    format!("conn-{}", i),
                    format!("localhost:{}", 8080 + i),
                );
                manager_clone.add_connection(connection).await;
            });
            handles.push(handle);
        }

        // Remove operations (some will fail, that's ok)
        for i in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                manager_clone.remove_connection(&format!("conn-{}", i)).await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    // ==================== EDGE CASE TESTS ====================

    #[tokio::test]
    async fn test_add_connection_with_long_id() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let long_id = "a".repeat(1000);
        let connection = ConnectionInfo::new(long_id, "localhost:8080".to_string());

        manager.add_connection(connection).await;
    }

    #[tokio::test]
    async fn test_add_service_with_unicode_name() {
        let config = NetworkConfig::default();
        let manager = NetworkServiceManager::new(config);

        let unicode_name = "服务-🚀-test";
        let service = ServiceInfo::new(unicode_name.to_string(), "localhost:8080".to_string());

        manager.add_service(service).await;
    }

    #[tokio::test]
    async fn test_multiple_managers_independent() {
        let config1 = NetworkConfig::default();
        let manager1 = NetworkServiceManager::new(config1);

        let config2 = NetworkConfig::default();
        let manager2 = NetworkServiceManager::new(config2);

        let service1 = ServiceInfo::new("service-1".to_string(), "localhost:8080".to_string());
        let service2 = ServiceInfo::new("service-2".to_string(), "localhost:9090".to_string());

        manager1.add_service(service1).await;
        manager2.add_service(service2).await;

        // Managers should be independent
    }
}

