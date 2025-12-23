//! Deep edge case tests for network client
//! Modern idiomatic implementation - Updated Dec 12, 2025
//!
//! Focus: Connection edge cases, timeout scenarios, retry logic,
//! connection pool exhaustion, error recovery
//!
//! MODERNIZED: Uses actual production API, not deprecated builder pattern

#[cfg(test)]
mod network_client_edge_cases {
    use crate::network::client::config::ClientConfig;
    use crate::network::client::http::HttpClient;
    use crate::network::client::types::{Endpoint, Port};
    use std::time::Duration;

    // ==================== TIMEOUT EDGE CASES ====================

    #[tokio::test]
    async fn test_connection_timeout_zero() {
        // Test behavior with minimal timeout
        let config = ClientConfig::<1>::default(); // 1ms timeout
        let client = HttpClient::new(config);

        let endpoint = Endpoint::http(
            "localhost".to_string(),
            Port::new(9999).expect("Valid port"),
        );

        // Should timeout quickly with minimal timeout
        let result = client.get(&endpoint, "/test").await;
        assert!(result.is_err(), "Should timeout with 1ms timeout");
    }

    #[tokio::test]
    async fn test_connection_timeout_extreme() {
        // Test with extremely long timeout
        let config = ClientConfig::<86400000>::default(); // 24 hours in ms
        let client = HttpClient::new(config);

        // Should accept extreme timeout without panic
        assert_eq!(config.timeout, Duration::from_secs(86400));
    }

    // ==================== CONNECTION POOL EDGE CASES ====================

    #[tokio::test]
    async fn test_concurrent_connection_limit() {
        // Test connection pool behavior at limit
        let config = ClientConfig::<30000>::default().with_max_connections(2);
        let client = HttpClient::new(config);

        let endpoint = Endpoint::http(
            "localhost".to_string(),
            Port::new(8080).expect("Valid port"),
        );

        // Spawn 5 concurrent requests (more than max connections)
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let client_clone = client.clone();
                let endpoint_clone = endpoint.clone();
                tokio::spawn(async move {
                    // Will queue if over limit
                    client_clone.get(&endpoint_clone, &format!("/test{}", i)).await
                })
            })
            .collect();

        // All should complete (queued if over limit)
        for handle in handles {
            let _ = handle.await;
        }
    }

    #[tokio::test]
    async fn test_retry_with_zero_attempts() {
        // Test retry logic with 0 max attempts
        let config = ClientConfig::<30000>::default().with_max_retries(0);
        let client = HttpClient::new(config);

        assert_eq!(config.max_retries, 0);
    }

    #[tokio::test]
    async fn test_retry_with_extreme_attempts() {
        // Test retry logic with very high max attempts
        let config = ClientConfig::<30000>::default().with_max_retries(1000);
        let client = HttpClient::new(config);

        // Should handle high retry count without overflow
        assert_eq!(config.max_retries, 1000);
    }

    // ==================== ENDPOINT EDGE CASES ====================

    #[tokio::test]
    async fn test_endpoint_with_max_port() {
        let port = Port::new(65535).expect("Max valid port");
        let endpoint = Endpoint::http("localhost".to_string(), port);

        assert_eq!(endpoint.port.get(), 65535);
        assert!(endpoint.base_url().contains("65535"));
    }

    #[tokio::test]
    async fn test_endpoint_with_min_port() {
        let port = Port::new(1).expect("Min valid port");
        let endpoint = Endpoint::http("localhost".to_string(), port);

        assert_eq!(endpoint.port.get(), 1);
        assert!(endpoint.base_url().contains(":1"));
    }

    #[tokio::test]
    async fn test_endpoint_with_empty_host() {
        // Empty host should be handled (though may not be valid)
        let port = Port::new(8080).expect("Valid port");
        let endpoint = Endpoint::http(String::new(), port);

        let url = endpoint.base_url();
        assert!(url.contains("http://"));
    }

    #[tokio::test]
    async fn test_endpoint_with_ipv4_address() {
        let port = Port::new(8080).expect("Valid port");
        let endpoint = Endpoint::http("192.168.1.1".to_string(), port);

        let url = endpoint.base_url();
        assert!(url.contains("192.168.1.1"));
        assert!(url.contains("8080"));
    }

    // ==================== CLIENT CONFIGURATION EDGE CASES ====================

    #[test]
    fn test_config_with_all_custom_values() {
        let config = ClientConfig::<30000>::default()
            .with_timeout(Duration::from_secs(60))
            .with_max_connections(50)
            .with_max_retries(10);

        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_connections_per_host, 50);
        assert_eq!(config.max_retries, 10);
    }

    #[test]
    fn test_config_extreme_values() {
        let config = ClientConfig::<1>::default()
            .with_max_connections(1)
            .with_max_retries(0);

        assert_eq!(config.timeout, Duration::from_millis(1));
        assert_eq!(config.max_connections_per_host, 1);
        assert_eq!(config.max_retries, 0);
    }

    #[test]
    fn test_config_high_values() {
        let mut config = ClientConfig::<86400000>::default();
        config.max_connections_per_host = 1000;
        config.max_idle_connections = 10000;
        config.max_retries = 100;

        assert_eq!(config.max_connections_per_host, 1000);
        assert_eq!(config.max_idle_connections, 10000);
        assert_eq!(config.max_retries, 100);
    }

    // ==================== CLIENT STATS EDGE CASES ====================

    #[tokio::test]
    async fn test_stats_with_no_activity() {
        let client = HttpClient::default();
        let stats = client.stats().await;

        // Fresh client should have zero activity
        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.active_requests, 0);
        assert_eq!(stats.idle_connections, 0);
        assert_eq!(stats.endpoints, 0);
    }

    #[tokio::test]
    async fn test_multiple_clients_isolated() {
        // Test that multiple clients maintain separate state
        let client1 = HttpClient::default();
        let client2 = HttpClient::default();

        let stats1 = client1.stats().await;
        let stats2 = client2.stats().await;

        // Each client should have independent stats
        assert_eq!(stats1.total_connections, 0);
        assert_eq!(stats2.total_connections, 0);
    }

    // ==================== CONCURRENT OPERATION EDGE CASES ====================

    #[tokio::test]
    async fn test_concurrent_stats_access() {
        let client = HttpClient::default();

        // Spawn multiple concurrent stats requests
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let client_clone = client.clone();
                tokio::spawn(async move { client_clone.stats().await })
            })
            .collect();

        // All should complete successfully
        for handle in handles {
            let stats = handle.await.expect("Task should complete");
            assert_eq!(stats.total_connections, 0);
        }
    }

    #[tokio::test]
    async fn test_concurrent_client_creation() {
        // Test creating multiple clients concurrently
        let handles: Vec<_> = (0..10)
            .map(|_| {
                tokio::spawn(async {
                    let client = HttpClient::default();
                    client.stats().await
                })
            })
            .collect();

        // All should create successfully
        for handle in handles {
            let stats = handle.await.expect("Task should complete");
            assert_eq!(stats.total_connections, 0);
        }
    }

    // ==================== ERROR PATH EDGE CASES ====================

    #[tokio::test]
    async fn test_invalid_port_handling() {
        // Test port validation
        let result = Port::new(0);
        assert!(result.is_err(), "Port 0 should be invalid");

        let result = Port::new(65536);
        assert!(result.is_err(), "Port 65536 should be invalid");
    }

    #[tokio::test]
    async fn test_connection_to_invalid_endpoint() {
        let client = HttpClient::default();
        
        let endpoint = Endpoint::http(
            "definitely-not-a-real-host-12345".to_string(),
            Port::new(8080).expect("Valid port"),
        );

        let result = client.get(&endpoint, "/test").await;
        assert!(result.is_err(), "Should fail to connect to invalid host");
    }

    // ==================== CLEANUP EDGE CASES ====================

    #[tokio::test]
    async fn test_cleanup_with_no_connections() {
        let client = HttpClient::default();
        
        // Should handle cleanup with no connections gracefully
        client.cleanup().await;
        
        let stats = client.stats().await;
        assert_eq!(stats.total_connections, 0);
    }

    #[tokio::test]
    async fn test_multiple_cleanups() {
        let client = HttpClient::default();
        
        // Multiple cleanups should be safe
        client.cleanup().await;
        client.cleanup().await;
        client.cleanup().await;
        
        let stats = client.stats().await;
        assert_eq!(stats.total_connections, 0);
    }
}

