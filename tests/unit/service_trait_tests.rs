/// Unit tests for UniversalService trait
///
/// Tests the core service trait functionality including lifecycle management,
/// health checks, request handling, and metrics collection.

use std::time::Duration;
use tokio::time::timeout;

mod common;
use common::*;

#[tokio::test]
async fn test_service_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = MockService::new("test-service");
    let config = MockConfig::default();

    // Test initialization
    assert!(service.initialize(config.clone()).await.is_ok());
    assert!(!service.is_started().await);

    // Test start
    assert!(service.start().await.is_ok());
    assert!(service.is_started().await);

    // Test stop
    assert!(service.stop().await.is_ok());
    assert!(!service.is_started().await);
    Ok(())
}

#[tokio::test]
async fn test_service_health_check() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = MockService::new("health-test");
    let config = MockConfig::default();

    assert!(service.initialize(config).await.is_ok());
    assert!(service.start().await.is_ok());

    let health = service.health_check().await?;
    assert_eq!(health.status, "healthy");
    assert_eq!(health.requests_handled, 0);
    Ok(())
}

#[tokio::test]
async fn test_service_request_handling() -> Result<(), Box<dyn std::error::Error>> {
    use nestgate_core::traits::{UniversalServiceRequest, UniversalServiceResponse};
    use chrono::Utc;
    use std::collections::HashMap;

    let mut service = MockService::new("request-test");
    let config = MockConfig::default();

    assert!(service.initialize(config).await.is_ok());
    assert!(service.start().await.is_ok());

    // Create a test request
    let request = ServiceRequest {
        id: "test-request-1".to_string(),
        method: "GET".to_string(),
        path: "/test".to_string(),
        headers: HashMap::new(),
        body: None,
        query_params: HashMap::new(),
        client: ClientInfo {
            id: "test-client".to_string(),
            ip_address: "127.0.0.1".to_string(),
            user_agent: "test-agent".to_string(),
        },
        auth: AuthInfo {
            authenticated: true,
            user_id: Some("test-user".to_string()),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string()],
        },
        timestamp: Utc::now(),
        timeout: nestgate_core::constants::test_defaults::TEST_MEDIUM_TIMEOUT,
        priority: 1,
        tags: HashMap::new(),
        correlation_id: Some("test-correlation".to_string()),
        trace_id: Some("test-trace".to_string()),
    };

    // Handle the request
    let response = service.handle_request(request).await?;
    assert_eq!(response.request_id, "test-request-1");

    // Verify request count increased
    assert_eq!(service.get_request_count().await, 1);
    Ok(())
}

#[tokio::test]
async fn test_service_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = MockService::new("metrics-test");
    let config = MockConfig::default();

    assert!(service.initialize(config).await.is_ok());
    assert!(service.start().await.is_ok());

    let metrics = service.get_metrics().await?;
    assert_eq!(metrics.request_count, 0);
    assert_eq!(metrics.error_rate, 0.0);
    assert!(metrics.avg_response_time_ms > 0.0);
    Ok(())
}

#[tokio::test]
async fn test_service_info() -> Result<(), Box<dyn std::error::Error>> {
    let service = MockService::new("info-test");
    let info = service.service_info();

    assert_eq!(info.id, "info-test");
    assert_eq!(info.service_type, "mock");
    assert!(info.capabilities.contains(&"http".to_string()));
    assert!(info.capabilities.contains(&"testing".to_string()));
    Ok(())
}

#[tokio::test]
async fn test_service_load_handling() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = MockService::new("load-test");
    let config = MockConfig::default();

    assert!(service.initialize(config).await.is_ok());

    // Service not started should not handle load
    assert!(!service.can_handle_load().await?);

    assert!(service.start().await.is_ok());

    // Started service should handle load
    assert!(service.can_handle_load().await?);

    // Test load factor
    let load_factor = service.get_load_factor().await?;
    assert!(load_factor >= 0.0 && load_factor <= 1.0);
    Ok(())
}

#[tokio::test]
async fn test_service_error_simulation() -> Result<(), Box<dyn std::error::Error>> {
    use nestgate_core::traits::{UniversalServiceRequest, UniversalServiceResponse};
    use chrono::Utc;
    use std::collections::HashMap;

    let mut service = MockService::new("error-test");
    let config = MockConfig::default();

    assert!(service.initialize(config).await.is_ok());
    assert!(service.start().await.is_ok());

    // Set 100% error rate
    service.set_error_rate(1.0).await;

    let request = ServiceRequest {
        id: "error-request-1".to_string(),
        method: "GET".to_string(),
        path: "/error".to_string(),
        headers: HashMap::new(),
        body: None,
        query_params: HashMap::new(),
        client: ClientInfo {
            id: "test-client".to_string(),
            ip_address: "127.0.0.1".to_string(),
            user_agent: "test-agent".to_string(),
        },
        auth: AuthInfo {
            authenticated: true,
            user_id: Some("test-user".to_string()),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string()],
        },
        timestamp: Utc::now(),
        timeout: nestgate_core::constants::test_defaults::TEST_MEDIUM_TIMEOUT,
        priority: 1,
        tags: HashMap::new(),
        correlation_id: Some("test-correlation".to_string()),
        trace_id: Some("test-trace".to_string()),
    };

    let response = service.handle_request(request).await?;

    // Should have error status
    match response.status {
        ResponseStatus::Error { code, .. } => assert_eq!(code, 500),
        _ => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Test assertion failed"))),
    Ok(())
}
}

#[tokio::test]
async fn test_service_config_update() -> Result<(), Box<dyn std::error::Error>> {
    let mut service = MockService::new("config-test");
    let mut config = MockConfig::default();
    config.response_delay_ms = 50;

    assert!(service.initialize(config.clone()).await.is_ok());
    assert!(service.start().await.is_ok());

    // Update config
    config.response_delay_ms = 100;
    assert!(service.update_config(config).await.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_service_response_timing() -> Result<(), Box<dyn std::error::Error>> {
    use nestgate_core::traits::{UniversalServiceRequest, UniversalServiceResponse};
    use chrono::Utc;
    use std::collections::HashMap;
    use std::time::Instant;

    let mut service = MockService::new("timing-test");
    let mut config = MockConfig::default();
    config.response_delay_ms = 100; // 100ms delay

    assert!(service.initialize(config).await.is_ok());
    assert!(service.start().await.is_ok());

    let request = ServiceRequest {
        id: "timing-request-1".to_string(),
        method: "GET".to_string(),
        path: "/timing".to_string(),
        headers: HashMap::new(),
        body: None,
        query_params: HashMap::new(),
        client: ClientInfo {
            id: "test-client".to_string(),
            ip_address: "127.0.0.1".to_string(),
            user_agent: "test-agent".to_string(),
        },
        auth: AuthInfo {
            authenticated: true,
            user_id: Some("test-user".to_string()),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string()],
        },
        timestamp: Utc::now(),
        timeout: nestgate_core::constants::test_defaults::TEST_MEDIUM_TIMEOUT,
        priority: 1,
        tags: HashMap::new(),
        correlation_id: Some("test-correlation".to_string()),
        trace_id: Some("test-trace".to_string()),
    };

    let start = Instant::now();
    let _response = service.handle_request(request).await?;
    let elapsed = start.elapsed();

    // Should take at least 100ms due to configured delay
    assert!(elapsed >= Duration::from_millis(95)); // Allow some tolerance
    Ok(())
}

#[tokio::test]
async fn test_concurrent_requests() -> Result<(), Box<dyn std::error::Error>> {
    use nestgate_core::traits::{UniversalServiceRequest, UniversalServiceResponse};
    use chrono::Utc;
    use std::collections::HashMap;
    use tokio::time::Instant;

    let mut service = MockService::new("concurrent-test");
    let config = MockConfig::default();

    assert!(service.initialize(config).await.is_ok());
    assert!(service.start().await.is_ok());

    let service = std::sync::Arc::new(service);
    let mut tasks = vec![];

    // Send 10 concurrent requests
    for i in 0..10 {
        let service_clone = service.clone();
        let request = ServiceRequest {
            id: format!("concurrent-request-{}", i),
            method: "GET".to_string(),
            path: "/concurrent".to_string(),
            headers: HashMap::new(),
            body: None,
            query_params: HashMap::new(),
            client: ClientInfo {
                id: "test-client".to_string(),
                ip_address: "127.0.0.1".to_string(),
                user_agent: "test-agent".to_string(),
            },
            auth: AuthInfo {
                authenticated: true,
                user_id: Some("test-user".to_string()),
                roles: vec!["user".to_string()],
                permissions: vec!["read".to_string()],
            },
            timestamp: Utc::now(),
            timeout: nestgate_core::constants::test_defaults::TEST_MEDIUM_TIMEOUT,
            priority: 1,
            tags: HashMap::new(),
            correlation_id: Some("test-correlation".to_string()),
            trace_id: Some("test-trace".to_string()),
        };

        let task = tokio::spawn(async move {
            service_clone.handle_request(request).await
        });
        tasks.push(task);
    Ok(())
    }

    // Wait for all requests to complete
    let start = Instant::now();
    let results = futures::future::join_all(tasks).await;
    let elapsed = start.elapsed();

    // All requests should succeed
    for result in results {
        assert!(result?.is_ok());
    }

    // Total request count should be 10
    assert_eq!(service.get_request_count().await, 10);

    // Should handle concurrently (not take 10x the time)
    assert!(elapsed < Duration::from_millis(200));
}