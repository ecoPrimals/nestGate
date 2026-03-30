// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use crate::handlers::NetworkServiceManager;

use super::{create_test_config, create_test_connection, create_test_service};

#[tokio::test]
async fn test_network_service_manager_creation() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    // Verify manager is created with empty state
    let active_connections = manager.get_active_connections().await;
    assert!(active_connections.is_empty());

    let healthy_services = manager.get_healthy_services().await;
    assert!(healthy_services.is_empty());
}

#[tokio::test]
async fn test_add_and_remove_connection() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    let conn = create_test_connection("conn1", true);
    manager.add_connection(conn).await;

    let active = manager.get_active_connections().await;
    assert_eq!(active.len(), 1);
    assert_eq!(active[0], "conn1");

    let removed = manager.remove_connection("conn1").await;
    assert!(removed);

    let active = manager.get_active_connections().await;
    assert!(active.is_empty());
}

#[tokio::test]
async fn test_remove_nonexistent_connection() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    let removed = manager.remove_connection("nonexistent").await;
    assert!(!removed);
}

#[tokio::test]
async fn test_add_and_remove_service() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    let service = create_test_service("svc1", "test-service", true);
    manager.add_service(service).await;

    let healthy = manager.get_healthy_services().await;
    assert_eq!(healthy.len(), 1);
    assert_eq!(healthy[0], "test-service");

    let removed = manager.remove_service("svc1").await;
    assert!(removed);

    let healthy = manager.get_healthy_services().await;
    assert!(healthy.is_empty());
}

#[tokio::test]
async fn test_get_active_connections_filtering() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    // All connections start as active by default
    manager
        .add_connection(create_test_connection("conn1", true))
        .await;
    manager
        .add_connection(create_test_connection("conn2", true))
        .await;
    manager
        .add_connection(create_test_connection("conn3", true))
        .await;

    let active = manager.get_active_connections().await;
    assert_eq!(active.len(), 3);
    assert!(active.contains(&"conn1".to_string()));
    assert!(active.contains(&"conn3".to_string()));
}

#[tokio::test]
async fn test_get_healthy_services_filtering() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    // All services start as healthy by default
    manager
        .add_service(create_test_service("svc1", "service1", true))
        .await;
    manager
        .add_service(create_test_service("svc2", "service2", true))
        .await;
    manager
        .add_service(create_test_service("svc3", "service3", true))
        .await;

    let healthy = manager.get_healthy_services().await;
    assert_eq!(healthy.len(), 3);
    assert!(healthy.contains(&"service1".to_string()));
    assert!(healthy.contains(&"service3".to_string()));
}

#[tokio::test]
async fn test_get_connection_details() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    let conn = create_test_connection("conn1", true);
    manager.add_connection(conn).await;

    let details = manager.get_connection_details("conn1").await;
    assert!(details.is_some());

    let details = details.expect("Connection details should be present");
    assert_eq!(details.id, "conn1");
    assert!(details.is_active);
}

#[tokio::test]
async fn test_get_connection_details_nonexistent() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    let details = manager.get_connection_details("nonexistent").await;
    assert!(details.is_none());
}

#[tokio::test]
async fn test_get_service_details() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    let service = create_test_service("svc1", "test-service", true);
    manager.add_service(service).await;

    let details = manager.get_service_details("svc1").await;
    assert!(details.is_some());

    let details = details.expect("Service details should be present");
    assert_eq!(details.id, "svc1");
    assert_eq!(details.name, "test-service");
}

#[tokio::test]
async fn test_health_check_services() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    // All services start as healthy by default
    manager
        .add_service(create_test_service("svc1", "service1", true))
        .await;
    manager
        .add_service(create_test_service("svc2", "service2", true))
        .await;

    let health_results = manager.health_check_services().await;
    assert!(health_results.is_ok());

    let results = health_results.expect("Health check should succeed");
    assert_eq!(results.len(), 2);
    assert_eq!(results.get("svc1"), Some(&true));
    assert_eq!(results.get("svc2"), Some(&true));
}

#[tokio::test]
async fn test_get_statistics() {
    let config = create_test_config();
    let manager = NetworkServiceManager::new(config);

    manager
        .add_connection(create_test_connection("conn1", true))
        .await;
    manager
        .add_connection(create_test_connection("conn2", true))
        .await;
    manager
        .add_service(create_test_service("svc1", "service1", true))
        .await;

    let stats = manager.get_statistics().await;
    assert!(stats.is_ok());

    let stats = stats.expect("Statistics should be available");
    assert_eq!(stats.active_connections, 2);
    assert_eq!(stats.registered_services, 1);
}
